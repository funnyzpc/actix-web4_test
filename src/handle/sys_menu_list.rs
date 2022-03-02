use actix_web::{HttpResponse, web};
use deadpool_postgres::{Client, Pool};
use crate::{ResultBuild, SysMenu};

pub async fn echo() -> HttpResponse{
    return HttpResponse::Ok().json(ResultBuild::<&str>::success());
}

// #[get("/echo")]
pub async fn sys_menu_list(/*req_body: String,*/db: web::Data<Pool>) -> HttpResponse{
// async fn echo(/*req_body: String*/db: web::Data<Pool>) ->impl Responder {
    // let mut conn=db.get().await.unwrap();
    // let rows=conn.query("select * from sys_menu ",&[]).await.unwrap();
    // // get参数可以是str,也可以是i32，获取第几个。但是必须要指明获取的类型
    // let sys_menus = menu_list(&db).await.expect("---error---");
    let sys_menu_list = menu_list(&db).await;
    // HttpResponse::Ok().json(sys_menus)
    return HttpResponse::Ok().json(ResultBuild::success_with_data(sys_menu_list));

}

async fn menu_list(pool: &Pool) -> Vec<SysMenu> {
    let client: Client = pool.get().await.expect("---error---");
    let stmt = client.prepare_cached("SELECT id,name,show_flag,create_date,code,parent_code from sys_menu limit 2").await.expect("--error2--");
    let rows = client.query(&stmt, &[]).await.expect("--error3");
    rows
        .into_iter()
        .map(|row| SysMenu {
            id: row.get(0),
            name: row.get(1),
            show_flag: row.get(2),
            create_date: row.get(3),
            code: row.get(4),
            parent_code: row.get(5),
        })
        .collect()
}
