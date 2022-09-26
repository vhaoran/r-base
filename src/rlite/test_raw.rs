// use ormlite::model::*;
// use ormlite::Connection;
//
// #[derive(Model, FromRow, Debug)]
// pub struct Person {
//     pub id: Option<i32>,
//     pub name: Option<String>,
//     pub age: Option<i32>,
// }
//
// pub static CREATE_TABLE_SQL: &str =
//     "CREATE TABLE person (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)";
//
// #[tokio::test]
// async fn test_1() anyhow::Result<()> {
//     /// Start by making a database connection.
//     let mut conn = ormlite::SqliteConnection::connect(":memory:").await?;
//
//     ormlite::query(CREATE_TABLE_SQL).execute(&mut conn).await?;
//
//     /// You can insert the model directly.
//     let mut john = Person {
//         id: Some(1),
//         name: Some("John".to_string()),
//         age: Some(99),
//     }
//     .insert(&mut conn)
//     .await?;
//     println!("{:?}", john);
//
//     john.age = Some(1 + john.age.unwrap_or(0));
//     john.update_all_fields(&mut conn).await?;
//
//     let people = Person::select()
//         .filter("age > ?")
//         .bind(50)
//         .fetch_all(&mut conn)
//         .await?;
//     println!("{:?}", people);
//
//     Ok(())
// }
