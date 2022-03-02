
use actix_web::dev::{self, Service, Transform};
use actix_web::{dev::{ ServiceRequest, ServiceResponse},Error, HttpResponse};

use actix_web::body::EitherBody;
use futures_util::future::{ok, FutureExt,LocalBoxFuture, Ready, ready};

use crate::ResultBuild;
use crate::util::token_util::TokenUtil;

#[derive(Clone)]
pub struct Authentication;

// 由于actix-web4的middleware尚在dev阶段,目前写法请参考下方地址
// https://github.com/dsferruzza/actix-web-middleware-keycloak-auth
impl<S, B> Transform<S,ServiceRequest> for Authentication
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
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
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    // 以下两者似乎是等同的,后者为actix-web3时代的写法~
    dev::forward_ready!(service);
    // fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    //     self.service.poll_ready(cx)
    // }
    fn call(&self, req: ServiceRequest) -> Self::Future {

        let auth_header = req.headers().get("Authorization");

        match auth_header {
            Some(auth_header_value) => {
                let auth_header_str = auth_header_value.to_str();
                match auth_header_str {
                    Ok(raw_token) => {
                        let token = raw_token.trim_start_matches("Bearer ");
                        println!("Bearer token was extracted from request headers {}",token);
                        Box::pin(self.service.call(req).map(map_body_left))
                    }
                    Err(_) => {
                        // 验证失败 ~
                        Box::pin(ready(Ok(req.into_response(
                            // HttpResponse::build(self.status_code()).body(self.status_code().to_string())
                            HttpResponse::Ok().json(ResultBuild::<&str>::fail_with_msg("验证失败01")).map_into_right_body()
                        ))))
                    }
                }
            }
            None => {
                Box::pin(ready(Ok(req.into_response(
                    HttpResponse::Ok().json(ResultBuild::<&str>::fail_with_msg("验证失败02")).map_into_right_body()
                ))))
            }
        }
    }
}

fn map_body_left<B, E>(res: Result<ServiceResponse<B>, E>,) -> Result<ServiceResponse<EitherBody<B>>, E> {
    res.map(|res| res.map_into_left_body())
}
