use std::str::FromStr;
use deadpool_postgres::{ Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::{Config, NoTls};

pub  async fn init_pg_pool() -> Pool {
    // base 文件目录(必须在.env文件配置)
    let db_url = std::env::var("DB_URL").expect("配置不存在::DB_URL");

    // 创建连接池
    let  cfg = Config::from_str(&db_url).expect("-- db cfg error break --");
    // let cfg = Config::from_str("postgres://chicmax:chicmax$test$0129@172.18.172.151:5432/chicmax?connect_timeout=60&search_path=public").unwrap();

    // let mut cfg = Config::new();
    // cfg.host("172.18.172.151");//数据库地址
    // cfg.port(5432); // 端口
    // cfg.user("chicmax");//数据库用户名
    // cfg.password("chicmax$test$0129");//数据库密码
    // cfg.dbname("chicmax");//数据库名称

    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast
    };
    let mgr = Manager::from_config(cfg,NoTls,mgr_config);
    // let pool = Pool::builder(mgr).max_size(8).build().unwrap();
    Pool::builder(mgr).max_size(8).build().expect("-- init db error --")

}
