use futures::{AsyncWriteExt, StreamExt};
use polodb_core::bson::{doc, Document};
use polodb_core::{bson, Database, IndexModel, IndexOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    uuid: Option<bson::Uuid>,
    title: String,
    author: String,
}

#[tokio::test]
async fn test_init() -> anyhow::Result<()> {
    let p = "./polo.db";
    let db = Database::open_file(p)?;
    //-------------------------------------

    //-------------------------------------
    let mut collection = db.collection("u");
    collection
        .insert_one(Book {
            uuid: Some(bson::Uuid::new()),
            title: "The Three-Body Problem".to_string(),
            author: "Liu Cixin".to_string(),
        })
        .unwrap();
    //-------------------------------------
    let book = collection.find_one(None).unwrap();
    println!("-----------{book:?}-----------",);

    let r = collection.update_one(
        doc! {},
        doc! {
            "$set":doc!{
                "author":"good-after my modify"
            }
        },
    );
    println!("-----update result:------{r:#?}-----------",);
    let r = collection.update_many(
        doc! {},
        doc! {
            "$set":doc!{
                "author":"many-update-good-after my modify"
            }
        },
    );
    println!("-----update many result:------{r:#?}-----------",);

    //-------------------------------------
    //-------------------------------------
    let books = collection.find(None).unwrap();
    for book in books {
        println!("many: {:?}", book);
    }

    // let _ = collection.flush();
    //-------------------------------------
    let r = collection.create_index(IndexModel {
        keys: doc! {
            "_id": 1,
        },
        options: Some(IndexOptions {
            unique: Some(true),
            ..Default::default()
        }),
    });

    //-------------------------------------
    Ok(())
}

#[test]
fn test_aggregate_count() {
    let db = Database::open_memory().unwrap();
    let fruits = db.collection::<Document>("fruits");
    fruits
        .insert_many(vec![
            doc! {
                "name": "apple",
                "color": "red",
                "shape": "round",
            },
            doc! {
                "name": "banana",
                "color": "yellow",
                "shape": "long",
            },
            doc! {
                "name": "orange",
                "color": "orange",
                "shape": "round",
            },
            doc! {
                "name": "pear",
                "color": "yellow",
                "shape": "round",
            },
            doc! {
                "name": "peach",
                "color": "orange",
                "shape": "round",
            },
        ])
        .unwrap();

    let result = fruits
        .aggregate(vec![
            doc! {
                "$match": {
                    "color": "yellow",
                },
            },
            doc! {
                "$count": "count",
            },
        ])
        .unwrap()
        .collect::<polodb_core::Result<Vec<Document>>>()
        .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].get("count").unwrap().as_i64().unwrap(), 2);
    println!(
        "-----------{:?}-----------",
        result[0].get("count").unwrap().as_i64().unwrap()
    );

    let result = fruits
        .aggregate(vec![doc! {
            "$count": "count",
        }])
        .unwrap()
        .collect::<polodb_core::Result<Vec<Document>>>()
        .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].get("count").unwrap().as_i64().unwrap(), 5);
}
