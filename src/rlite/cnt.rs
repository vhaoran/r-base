use crate::g;
use futures::executor::block_on;
use once_cell::sync::OnceCell;
use sqlx::sqlite::{SqliteConnectOptions, SqliteSynchronous};
use sqlx::{Pool, Sqlite, SqlitePool};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

//
type IType = Pool<Sqlite>;
const SQLITE_DB_NAME: &str = "sqlite.db";

fn instance() -> &'static Arc<IType> {
    static INSTANCE: OnceCell<Arc<IType>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let pwd = g::pwd();
        let p = Path::new(pwd.as_str()).join(SQLITE_DB_NAME);
        let path = format!("{}", p.display());

        // let url = format!("sqlite:{SQLITE_DB_NAME}");
        // let pool = block_on(SqlitePool::connect(url.as_str())).unwrap();
        let opt = SqliteConnectOptions::default()
            .create_if_missing(true)
            .synchronous(SqliteSynchronous::Off)
            .filename(path.as_str());
        let pool = block_on(SqlitePool::connect_with(opt)).unwrap();
        Arc::new(pool)
    })
}

pub async fn get_lite_cnt() -> Arc<IType> {
    self::instance().clone()
}

pub async fn get_fast_lite_cnt() -> IType {
    let pwd = g::pwd();
    let p = Path::new(pwd.as_str()).join(SQLITE_DB_NAME);
    let path = format!("{}", p.display());

    // let url = format!("sqlite:{SQLITE_DB_NAME}");
    let opt = SqliteConnectOptions::default()
        .create_if_missing(true)
        .synchronous(SqliteSynchronous::Off)
        .statement_cache_capacity(10_000)
        .filename(path.as_str());
    SqlitePool::connect_with(opt).await.unwrap()
}
async fn get_mem_lite_cnt() -> IType {
    // let url = format!("sqlite:{SQLITE_DB_NAME}");
    SqlitePool::connect("sqlite::memory:").await.unwrap()
}
