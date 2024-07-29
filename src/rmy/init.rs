use std::sync::Arc;

use once_cell::sync::OnceCell;
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use sqlx::{MySql, MySqlPool, Pool};

use crate::rmy::Config;

//-----------instance--------------------------
type IType = Pool<MySql>;

static INSTANCE: OnceCell<Arc<IType>> = OnceCell::new();
// const SECS: i64 = 60 * 2;

//-----------install action--------------------------
pub async fn init(cfg: &Config) -> anyhow::Result<()> {
    let host = cfg.host.clone().unwrap_or("127.0.0.1".to_string());
    let port = cfg.port.clone().unwrap_or(3306);
    let user_name = cfg.user_name.clone().unwrap_or("root".to_string());
    let pwd = cfg.pwd.clone().unwrap_or("password".to_string());
    let db_name = cfg.db_name.clone().unwrap_or("test".to_string());
    let max_conn = cfg.max_conn.clone().unwrap_or(10);
    let min_conn = cfg.min_conn.clone().unwrap_or(5);
    let opt = MySqlConnectOptions::new()
        .host(host.as_str())
        .port(port)
        .username(user_name.as_str())
        .password(pwd.as_str())
        .database(db_name.as_str())
        .charset("utf8mb4")
        .timezone(Some("+08:00".to_string()));
    let pool = MySqlPoolOptions::new()
        .min_connections(min_conn)
        .max_connections(max_conn)
        .connect_with(opt)
        .await?;

    let a = Arc::new(pool);
    let _ = INSTANCE.set(a);
    Ok(())
}

pub async fn get_my_cnt() -> Arc<IType> {
    self::INSTANCE.get().unwrap().clone()
}

//-----------------------------------
mod t {
    use crate::rmy::init;
    use crate::rmy::init::get_my_cnt;
    use crate::{g, rmy};

    #[tokio::test]
    async fn test_v1() -> anyhow::Result<()> {
        //
        let cfg = rmy::Config {
            host: Some("w5".to_string()),
            port: Some(3306),
            user_name: Some("root".to_string()),
            pwd: Some("password".to_string()),
            db_name: Some("test".to_string()),
            max_conn: Some(10),
            min_conn: Some(2),
        };
        let _ = init(&cfg).await?;
        //
        println!("-----------start-----------",);

        let cnt = get_my_cnt().await;
        println!("-----------ok-----------",);

        let pool = get_my_cnt().await;

        let r: i64 = sqlx::query_scalar(
            r#"  select count(*) from t
                        "#,
        )
        .fetch_one(pool.as_ref())
        .await?;

        let start = g::unix_sec();
        let start_str = g::timestamp_str(start);
        loop {
            // tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            let i = g::unix_millis() % 123456789;

            let r = sqlx::query(
                r#"  insert into t(a,b)values(?,?)
                        "#,
            )
            .bind(g::random())
            .bind(i)
            .execute(pool.as_ref())
            .await?;
            println!("-----------count: {r:#?}-----------",);
        }
    }
}
