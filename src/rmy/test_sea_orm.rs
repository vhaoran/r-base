// // use sea_orm::database;
// #[macro_use]
// use sea_orm::entity::prelude::*;
// use sea_orm::ConnectOptions;
// use sea_orm::Database;
// use sea_orm::DatabaseConnection;
//
// #[derive(Clone, Debug, PartialEq)]
// #[sea_orm(table_name = "cake")]
// pub struct Model {
//     #[sea_orm(primary_key)]
//     pub id: i32,
//     pub name: String,
// }
// impl ActiveModelBehavior for ActiveModel {}
//
// pub async fn cnt() -> anyhow::<DatabaseConnection, Box<dyn std::error::Error>> {
//     let url = "mysql://root:password@192.168.0.99/test";
//     let mut opt = ConnectOptions::new(url.to_string());
//     let db = Database::connect(opt).await?;
//
//     Ok(db)
// }
//
// #[tokio::test]
// async fn my_crate_1() anyhow::Result<()> {
//     let db = cnt().await?;
//     println!("----db-cnt-------{:#?}-----------", db);
//     //
//
//     //
//     Ok(())
// }
