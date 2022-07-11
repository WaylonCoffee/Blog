use std::sync::Arc;

use axum::{routing::get, Extension, Router};
use dotenv::dotenv;

mod config;
mod handler;

#[derive(Clone)]
pub struct AppState {
    /// PostgreSQL 连接池
    pub pool: deadpool_postgres::Pool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // load config
    let cfg = config::AppConfig::from_env().expect("初始化配置失败");

    // connnect database
    let pool = cfg
        .pg
        .create_pool(tokio_postgres::NoTls)
        .expect("初始化数据库连接池失败");

    let app = Router::new()
        .route("/", get(handler::usage))
        .layer(Extension(Arc::new(AppState { pool })));

    // 绑定到配置文件设置的地址
    axum::Server::bind(&cfg.server.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
