extern crate mongodb;
extern crate serde;
extern crate serde_derive;
extern crate tokio;

use mongodb::bson::{doc, Document};
use mongodb::{options::ClientOptions, Client};

use futures::stream::{StreamExt, TryStreamExt};

async fn cnt() -> Result<Client, Box<dyn std::error::Error>> {
    let client = Client::with_uri_str("mongodb://root:password@192.168.0.99").await?;
    Ok(client.clone())
}

use serde::{Deserialize, Serialize};
use std::any::Any;
use std::default;
use std::default::Default;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: u64,
    pub title: String,
    pub author: String,
}
// impl std::default::Default for User {
//     fn default() -> Self {
//         User {
//             id: 0,
//             title: "".to_string(),
//             author: "".to_string(),
//         }
//     }
// }

impl User {
    pub fn to_doc_with_id(&self, id: &str) -> Document {
        let src = self.clone();
        doc! {
            "_id":id.to_string(),
            "title": src.title,
            "author": src.author,
        }
    }
}

#[tokio::test]
// async fn mongo_1()->mongodb::error::Result<()> {
async fn mongo_1() -> Result<(), Box<dyn std::error::Error>> {
    println!("----aaaa--a----");

    let client = Client::with_uri_str("mongodb://192.168.0.99").await?;
    let c = client.clone();

    let l = c.list_database_names(None, None).await?;
    for each in l.iter() {
        println!("---dbname: {}-----", each);
    }

    println!("----rmongo_test.rs---ok-----");
    Ok(())
}

#[tokio::test]
async fn mongo_2() -> mongodb::error::Result<()> {
    let client = Client::with_uri_str("mongodb://192.168.0.99").await?;
    let c = client.clone();

    let l = c.list_database_names(None, None).await?;
    for db in l.iter() {
        println!("---db {}-----", db);
        let r = c.database(db.as_str()).list_collection_names(None).await;
        if let Ok(l) = r {
            for tb in l.iter() {
                println!("collection: - {} -", tb);
            }
        }
    }

    println!("----rmongo_test.rs---ok-----");
    Ok(())
}

#[tokio::test]
async fn mongo_3() -> mongodb::error::Result<()> {
    use mongodb::bson::{doc, Document};

    let client = Client::with_uri_str("mongodb://192.168.0.99").await?;
    let c = client.clone();
    let db = c.database("test");
    let tb = db.collection::<Document>("books");

    let docs = vec![
        doc! { "title": "1984", "author": "George Orwell" },
        doc! { "title": "Animal Farm", "author": "George Orwell" },
        doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    ];

    let r = tb.insert_many(docs, None).await?;

    println!("----rmongo_test.rs---insert return: {:?}----", r);

    Ok(())
}

#[tokio::test]
async fn mi_one() -> Result<(), Box<dyn std::error::Error>> {
    use mongodb::bson::{doc, Document};
    let cnt = cnt().await?;
    let db = cnt.database("test");
    let tb = db.collection::<Document>("a");
    //
    let ret = tb
        .insert_one(
            doc! {
            "_id":"a" ,
            "title": "1984",
                "author": "George Orwell"
            },
            None,
        )
        .await?;
    println!("----rmongo_test.rs---a--{:?}---", ret);
    Ok(())
}

#[tokio::test]
async fn mi_one_1() -> Result<(), Box<dyn std::error::Error>> {
    use mongodb::bson::{doc, Document};
    let cnt = cnt().await?;
    let db = cnt.database("test");
    let tb = db.collection::<Document>("a");
    //
    let ret = tb
        .insert_one(
            doc! {
                "_id": "aaa",
                "title": "whr".to_string(),
                "author": "authore is whr".to_string(),
            },
            None,
        )
        .await?;
    println!("----rmongo_test.rs---a--{:?}---", ret);
    Ok(())
}

#[tokio::test]
async fn m_find_1() -> Result<(), Box<dyn std::error::Error>> {
    use mongodb::bson::{doc, Document};
    let cnt = cnt().await?;
    let db = cnt.database("test");
    let tb = db.collection::<User>("a");
    let r = tb
        .find_one(
            doc! {
                "_id":3,
            },
            None,
        )
        .await?;
    println!("----rmongo_test.rs---a---{:?}--", r);
    Ok(())
}

#[tokio::test]
async fn mi_one_2() -> Result<(), Box<dyn std::error::Error>> {
    use mongodb::bson::{doc, Document};
    let cnt = cnt().await?;
    let db = cnt.database("test");
    let tb = db.collection::<Document>("a");

    //
    let ret = tb
        .insert_one(
            User {
                id: 88u64,
                title: "whr".to_string(),
                author: "authore is whr".to_string(),
            }
            .to_doc_with_id("123"),
            None,
        )
        .await?;
    println!("----rmongo_test.rs---a--{:?}---", ret);
    Ok(())
}

#[tokio::test]
async fn mi_one_3() -> Result<(), Box<dyn std::error::Error>> {
    use mongodb::bson::{doc, Document};
    let cnt = cnt().await?;
    let db = cnt.database("test");
    let tb = db.collection::<User>("user");

    //
    let ret = tb
        .insert_one(
            User {
                id: 88u64,
                title: "whr-88".to_string(),
                author: "88-authore is whr".to_string(),
            },
            None,
        )
        .await?;
    println!("----rmongo_test.rs---a--{:?}---", ret);
    Ok(())
}

#[tokio::test]
async fn cnt_op_1() -> Result<(), Box<dyn std::error::Error>> {
    use mongodb::{options::ClientOptions, Client};

    let mut opt = ClientOptions::parse("mongodb://root:password@192.168.0.99:27017").await?;

    // Manually set an option.
    opt.app_name = Some("My App".to_string());

    println!("----{:?}-----", opt);

    opt.max_idle_time = Some(std::time::Duration::from_secs(3600));
    opt.max_pool_size = Some(10);
    opt.min_pool_size = Some(5);
    println!("--2--{:?}-----", opt);
    let c = Client::with_options(opt)?;
    //
    let l = c.list_database_names(None, None).await?;
    for str in l.iter() {
        println!("----db---{}--", str);
    }

    //
    println!("----ok-----");

    Ok(())
}

#[tokio::test]
async fn m_agg_1() -> Result<(), Box<dyn std::error::Error>> {
    use futures::stream::{StreamExt, TryStreamExt};
    use mongodb::bson::{doc, Document};
    // use std::pin::Pin;
    let cnt = cnt().await?;
    let db = cnt.database("telinfo");
    let tb = db.collection::<User>("tel_chat");
    let mut c = tb
        .aggregate(
            vec![doc! {
                "$group":{
                    "_id":1,
                    "aa":{
                        "$max":"$_id"
                    }
                }
            }],
            None,
        )
        .await?;

    let mut l: Vec<Document> = Vec::new();
    while let Some(row) = c.next().await {
        if let Ok(row) = row {
            l.push(row.clone());
            println!("------------{:?}----------", row);
        }
    }

    println!("-----------ok-----------");
    Ok(())
}
