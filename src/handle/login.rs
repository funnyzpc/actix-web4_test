use std::collections::HashMap;
use actix_web::{HttpResponse, web};
// use actix_web::cookie::{Cookie, SameSite};
use crate::{ResultBuild};
use crate::util::token_util::TokenUtil;


// 登录处理
pub async fn login(params: web::Form<HashMap<String,String>>) -> HttpResponse {
    if params.is_empty() {
        return HttpResponse::Ok().json(ResultBuild::<&str>::fail_with_msg("参数为空(&,&)"));
    }
    println!("===>login");
    let username = params.get("username");
    let password = params.get("password");
    // let context_path2 = std::env::var("CONTEXT_PATH").unwrap();

    if username.is_none() || password.is_none(){
        println!("用户或密码为空:{:?}->{:?}",username,password);
        return HttpResponse::Ok().json(ResultBuild::<&str>::fail_with_msg("用户或密码为空"));
    }
    // 从配置获取用户并检查
    let password_cfg =  std::env::var(format!("U.{}",&username.unwrap()));

    match password_cfg {
        Err(error)=>{
            println!("用户不存在或密码错误:{:?}->{:?},{}",username,password,error);
            return HttpResponse::Ok().json(ResultBuild::<&str>::fail_with_msg("用户不存在或密码错误"));
        },
        Ok(result)=>{
            if !result.eq(password.unwrap()){
                println!("用户密码不匹配:{:?}->{:?}",username,password);
                return HttpResponse::Ok().json(ResultBuild::<&str>::fail_with_msg("用户密码不匹配"));
            }
        }
    }
    let token = TokenUtil::gen_token(username.unwrap());
    return HttpResponse::Ok().json(ResultBuild::<&str>::success_with_data_str(&format!("bearer {}",token)))
    // let r = chrono::offset::Local::now().timestamp();
    // let one_hour = Duration::minutes(60);
    // let mut cookie = Cookie::new("Authorization",format!("bearer {}",token));
    // //cookie.set_path("/");
    // cookie.set_path(&context_path);
    // cookie.set_http_only(true);
    // //cookie.set_expires(None);
    // cookie.set_max_age(one_hour);
    // cookie.set_same_site(SameSite::Strict);
    // // return HttpResponse::MovedPermanently().cookie(cookie).header("Location",format!("{}?r={}",context_path,r)).finish();
    // return HttpResponse::MovedPermanently().cookie(cookie).header("Location",format!("{}?r={}",context_path,r)).finish();
}
