use sqlx::sqlite::{SqlitePool, SqliteQueryResult};
use sqlx::{query, Executor, FromRow};

//use mongodb::bson::oid;
use crate::g;
use crate::rlite::cnt::get_lite_cnt;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[macro_use]
mod x {
    macro_rules! ttt {
        () => {
            println!("-----------hellop worlkd-----------",);
        };
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, FromRow)]
pub struct T {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub done: Option<bool>,
}

#[tokio::test]
async fn test_gen_table() -> anyhow::Result<()> {
    let pool = get_lite_cnt().await;
    //
    let r = pool
        .execute(sqlx::query(
            r#"        
        CREATE TABLE IF NOT EXISTS t
(
    id          INTEGER PRIMARY KEY NOT NULL,
    name TEXT                NOT NULL,
    done        BOOLEAN             NOT NULL DEFAULT 0
);
   "#,
        ))
        .await;
    //
    println!("-----------gen table ok- {r:?}----------",);
    let name = "good";
    let r: SqliteQueryResult = pool
        .execute(
            sqlx::query(
                r#"
      insert into t(name)values(?);
    "#,
            )
            .bind(name),
        )
        .await?;
    println!("-----------insert ok----{r:#?}-------",);
    Ok(())
}

#[tokio::test]
async fn test_count() -> anyhow::Result<()> {
    let pool = get_lite_cnt().await;

    let r: i64 = sqlx::query_scalar(
        r#"
    select count(*) from t;
    "#,
    )
    .fetch_one(pool.as_ref())
    .await?;
    println!("-----------{r:?}-----------",);

    Ok(())
}

#[tokio::test]
async fn test_find_many() -> anyhow::Result<()> {
    let pool = get_lite_cnt().await;

    let r: Vec<T> = sqlx::query_as(
        r#"
    select * from t where id > ?;
    "#,
    )
    .bind(2)
    .fetch_all(pool.as_ref())
    .await?;
    println!("-----------{r:#?}-----------",);
    ttt!();
    Ok(())
}
#[tokio::test]
async fn test_find_one() -> anyhow::Result<()> {
    let pool = get_lite_cnt().await;

    let r: T = sqlx::query_as(
        r#"
    select * from t where id = ?;
    "#,
    )
    .bind(5)
    .fetch_one(pool.as_ref())
    .await?;
    println!("-----------{r:#?}-----------",);
    ttt!();
    Ok(())
}

#[tokio::test]
async fn test_bench_insert() -> anyhow::Result<()> {
    let pool = get_lite_cnt().await;

    let start = g::unix_sec();
    for i in 0..1_00_000_000 {
        let name = format!("name_{i}");
        match sqlx::query(
            r#"
    insert into t (name)values(?);
    "#,
        )
        .bind(name.as_str())
        .execute(pool.as_ref())
        .await
        {
            Ok(v) => {
                if i % 10000 == 0 {
                    println!("-----------ok: {i}--{} s---------", g::unix_sec() - start);
                }
            }
            Err(e) => {
                println!("----------insert error-{e:?}-----------",);
            }
        }
    }
    println!("-----------secs: {}-----------", g::unix_sec() - start);
    Ok(())
}
