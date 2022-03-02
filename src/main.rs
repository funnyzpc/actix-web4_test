mod date_fmt;
mod structs;
mod handle;
mod cfg;
mod util;
mod middleware;

use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use env_logger::Env;
use dotenv::dotenv;
use crate::cfg::pg_db;
use crate::structs::result_build::ResultBuild;
use crate::structs::sys_menu::SysMenu;
use crate::handle::{login, sys_menu_list};
use crate::middleware::auth_middleware;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 使用日志
    std::env::set_var("RUST_LOG", "RUST_LOG=debug,actix_server=debug,actix_web=debug");
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    // 加载配置
    if dotenv().ok().is_none(){
        println!("--->配置文件加载失败，请检查~<---");
        // panic!("-->配置文件加载失败，请检查~",)
        return Ok(());
    }
    let context_path = std::env::var("CONTEXT_PATH").expect("配置不存在::CONTEXT_PATH");
    let server_port = std::env::var("SERVER_PORT").expect("配置不存在::SERVER_PORT");

    // db
    let pool = pg_db::init_pg_pool().await;
    // router
    println!("visit:\n http://127.0.0.1:8080/");
    HttpServer::new(move|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(pool.clone()))
            // .data(pool.clone())
            // 授权资源 需要注意的是不同的service存在优先级(比如 "/api" 要比 ""优先级高)
            .service(web::scope(&context_path).wrap(auth_middleware::Authentication)
                .route("/sys_menu_list",web::get().to(sys_menu_list::sys_menu_list))
            )
            // 非授权资源
            // .service(echo)
            .service(web::scope("") // context path
                .route("/echo",web::get().to(sys_menu_list::echo))
                .route("/login",web::post().to(login::login))
            )

    })
        .bind(format!("0.0.0.0:{}",server_port))?
        .run()
        .await
}
