// use mongodb::event::cmap::ConnectionPoolOptions;
// use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions, MySqlRow};
// use sqlx::Connection;
// use sqlx::MySqlPool;
//
// pub async fn cnt() -> anyhow::<MySqlPool, Box<dyn std::error::Error>> {
//     let url = "mysql://root:password@192.168.0.99/test";
//     let mut pool = MySqlPool::connect(url).await?;
//     // let pool = MySqlPool::connect_with(MySqlConnectOptions::new()).await?;
//     Ok(pool)
// }
//
// #[tokio::test]
// async fn my_1() anyhow::Result<()> {
//     let conn = self::cnt().await?;
//     // println!("-----------{:#?}-----------", pool);
//     //-------------------------------------
//     let row: (i64,) = sqlx::query_as("SELECT ?")
//         .bind(100_i64)
//         .fetch_one(&conn)
//         .await?;
//     //
//     println!("-----------{:#?}-----------", row);
//
//     Ok(())
// }
//
// #[tokio::test]
// async fn my_2() anyhow::Result<()> {
//     let conn = self::cnt().await?;
//     // println!("-----------{:#?}-----------", pool);
//     //-------------------------------------
//     let r: Vec<MySqlRow> = sqlx::query("SELECT id from t").fetch_all(&conn).await?;
//     println!("-----------{:#?}-----------", r);
//
//     Ok(())
// }
//
// #[tokio::test]
// async fn my_3() anyhow::Result<()> {
//     let conn = self::cnt().await?;
//     // println!("-----------{:#?}-----------", pool);
//     //-------------------------------------
//     let r: Vec<MySqlRow> = sqlx::query("SELECT max(id) as id from t")
//         .fetch_all(&conn)
//         .await?;
//     println!("-----------{:#?}-----------", r);
//
//     Ok(())
// }
