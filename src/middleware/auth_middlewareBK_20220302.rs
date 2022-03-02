
use actix_service::{Service, Transform};
use actix_web::{body::MessageBody, cookie::{Cookie, CookieJar, SameSite}, dev::{Payload, ServiceRequest, ServiceResponse}, error::{ErrorInternalServerError, ErrorUnauthorized}, http::header::{HeaderValue, SET_COOKIE}, Error, FromRequest, HttpMessage, HttpRequest, HttpResponse};

use std::{
    pin::Pin,
    task::{Context, Poll},
};
use std::fmt::{Display, Formatter};
// use futures_util::{future::LocalBoxFuture, stream::StreamExt};
use std::rc::Rc;
use futures_util::future::{err, ok, LocalBoxFuture, Ready, ready};
use jsonwebtoken::{decode, decode_header, Validation};
use crate::ResultBuild;
use crate::util::token_util::TokenUtil;

pub struct Authentication;

impl<S, B> Transform<S,ServiceRequest> for Authentication
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>>,
        S::Future: 'static,
        S::Error: 'static,
        B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware {
            service
        })
    }
}


pub struct AuthenticationMiddleware<S> {
    service: S,
    // service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>>,
        S::Future: 'static,
        S::Error: 'static,
        B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {

        let auth_header = req.headers().get("Authorization");

        match auth_header {
            Some(auth_header_value) => {
                let auth_header_str = auth_header_value.to_str();
                match auth_header_str {
                    Ok(raw_token) => {
                        let token = raw_token.trim_start_matches("Bearer ");
                        println!("Bearer token was extracted from request headers");

                        match decode_header(token) {
                            Ok(jwt_header) => {
                                println!("JWT header was decoded");
                                println!("JWT is using {:?} algorithm", &jwt_header.alg);

                                match decode::<Value>(
                                    token,
                                    &self.keycloak_oid_public_key,
                                    &Validation::new(jwt_header.alg),
                                ) {
                                    Ok(raw_token) => {
                                        println!("JWT was decoded");

                                        match from_value::<RoleClaims>(raw_token.claims.clone()) {
                                            Ok(role_claims) => {
                                                let roles = role_claims.roles();

                                                match check_roles(&roles, &self.required_roles) {
                                                    Ok(_) => {
                                                        println!("JWT is valid");

                                                        {
                                                            let mut extensions =
                                                                req.extensions_mut();
                                                            extensions.insert(
                                                                KeycloakAuthStatus::Success,
                                                            );
                                                            extensions.insert(RawClaims(
                                                                raw_token.claims,
                                                            ));
                                                            extensions.insert(roles);
                                                        }

                                                        Box::pin(
                                                            self.service
                                                                .call(req)
                                                                .map(map_body_left),
                                                        )
                                                    }
                                                    Err(e) => {
                                                        println!("{}", &e);
                                                        match self.passthrough_policy.policy(&e) {
                                                            PassthroughAction::Pass => {
                                                                {
                                                                    let mut extensions =
                                                                        req.extensions_mut();
                                                                    extensions.insert(
                                                                        KeycloakAuthStatus::Failure(
                                                                            e.clone(),
                                                                        ),
                                                                    );
                                                                }
                                                                Box::pin(
                                                                    self.service
                                                                        .call(req)
                                                                        .map(map_body_left),
                                                                )
                                                            }
                                                            PassthroughAction::Return => {
                                                                Box::pin(ready(Ok(req
                                                                    .into_response(e.to_response(
                                                                        self.detailed_responses,
                                                                    ))
                                                                    .map_into_right_body())))
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                let e = AuthError::RoleParsingError(e.to_string());
                                                println!("{}", &e);
                                                match self.passthrough_policy.policy(&e) {
                                                    PassthroughAction::Pass => {
                                                        {
                                                            let mut extensions =
                                                                req.extensions_mut();
                                                            extensions.insert(
                                                                KeycloakAuthStatus::Failure(
                                                                    e.clone(),
                                                                ),
                                                            );
                                                        }
                                                        Box::pin(
                                                            self.service
                                                                .call(req)
                                                                .map(map_body_left),
                                                        )
                                                    }
                                                    PassthroughAction::Return => {
                                                        Box::pin(ready(Ok(req.into_response(
                                                            e.to_response(self.detailed_responses)
                                                                .map_into_right_body(),
                                                        ))))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        let e = AuthError::DecodeError(e.to_string());
                                        println!("{}", &e);
                                        match self.passthrough_policy.policy(&e) {
                                            PassthroughAction::Pass => {
                                                {
                                                    let mut extensions = req.extensions_mut();
                                                    extensions.insert(KeycloakAuthStatus::Failure(
                                                        e.clone(),
                                                    ));
                                                }
                                                Box::pin(self.service.call(req).map(map_body_left))
                                            }
                                            PassthroughAction::Return => Box::pin(ready(Ok(req
                                                .into_response(
                                                    e.to_response(self.detailed_responses)
                                                        .map_into_right_body(),
                                                )))),
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                let e = AuthError::InvalidJwt(e.to_string());
                                println!("{}", &e);
                                match self.passthrough_policy.policy(&e) {
                                    PassthroughAction::Pass => {
                                        {
                                            let mut extensions = req.extensions_mut();
                                            extensions
                                                .insert(KeycloakAuthStatus::Failure(e.clone()));
                                        }
                                        Box::pin(self.service.call(req).map(map_body_left))
                                    }
                                    PassthroughAction::Return => Box::pin(ready(Ok(req
                                        .into_response(
                                            e.to_response(self.detailed_responses)
                                                .map_into_right_body(),
                                        )))),
                                }
                            }
                        }
                    }
                    Err(_) => {
                        let e = AuthError::InvalidAuthorizationHeader;
                        println!("{}", &e);
                        match self.passthrough_policy.policy(&e) {
                            PassthroughAction::Pass => {
                                {
                                    let mut extensions = req.extensions_mut();
                                    extensions.insert(KeycloakAuthStatus::Failure(e.clone()));
                                }
                                Box::pin(self.service.call(req).map(map_body_left))
                            }
                            PassthroughAction::Return => Box::pin(ready(Ok(req.into_response(
                                e.to_response(self.detailed_responses).map_into_right_body(),
                            )))),
                        }
                    }
                }
            }
            None => {
                let e = AuthError::NoAuthorizationHeader;
                println!("{}", &e);
                match self.passthrough_policy.policy(&e) {
                    PassthroughAction::Pass => {
                        {
                            let mut extensions = req.extensions_mut();
                            extensions.insert(KeycloakAuthStatus::Failure(e.clone()));
                        }
                        Box::pin(self.service.call(req).map(map_body_left))
                    }
                    PassthroughAction::Return => Box::pin(ready(Ok(req.into_response(
                        e.to_response(self.detailed_responses).map_into_right_body(),
                    )))),
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeycloakAuthStatus {
    /// Authentication is successful
    Success,
    /// Authentication failed
    Failure(AuthError),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassthroughAction {
    /// Continue to the handler as if authentication was not mandatory
    Pass,
    /// Return a HTTP error immediately
    Return,
}
