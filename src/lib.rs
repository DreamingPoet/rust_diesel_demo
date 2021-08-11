// this is a macro which I don't know 
#[macro_use]

// 引用外部的库 diesel
extern crate diesel;
// 外部库 dotenv ，配置开发环境的
extern crate dotenv;


use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

// 定义个函数，建立连接，返回一个 PgConnection
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    // 从配置文件中获取数据库连接地址
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}