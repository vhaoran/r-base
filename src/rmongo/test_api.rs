use super::*;
// use futures_util::stream::stream::StreamExt;
use crate::init_modules;
use tracing::*;
use mongodb::bson::{doc, Bson, Document};
use mongodb::options::{Acknowledgment, ReadConcern, TransactionOptions, WriteConcern};
use mongodb::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::default::Default;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct User {
    #[serde(rename = "_id")]
    id: Option<i32>,
    title: Option<String>,
    author: Option<String>,
}
impl User {
    pub fn new() -> Self {
        User {
            id: None,
            title: None,
            author: None,
        }
    }
}

impl Into<Document> for User {
    fn into(self) -> Document {
        let bson = mongodb::bson::to_bson(&self).unwrap();
        let mut d: Document = mongodb::bson::from_bson(bson).unwrap();

        println!("-----from_bson----{:?}--", d);

        let _r = d.remove("id");
        d.insert("_id", self.id.clone().unwrap());

        d
        //
        // doc! {
        //     "_id":self.id.unwrap(),
        //     "title": self.title.unwrap(),
        //     "author":self.author.unwrap(),
        // }
    }
}
impl From<Document> for User {
    fn from(d: Document) -> User {
        let bson = mongodb::bson::to_bson(&d).unwrap();
        match mongodb::bson::from_bson::<User>(bson) {
            Ok(mut x) => {
                x.id = Some(d.get_i32("_id").unwrap_or(0));
                x
            }
            _ => User::new(),
        }

        // User {
        //     id: Some(d.get_i32("_id").unwrap_or(0)),
        //     title: d.get("title").map(|x| x.as_str().unwrap_or("").to_string()),
        //     author: d
        //         .get("author")
        //         .map(|x| x.as_str().unwrap_or("").to_string()),
        // }
    }
}

// impl From<Document> for Option<User> {
//     fn from(d: Document) -> Option<User> {
//         let bson = mongodb::bson::to_bson(d).unwrap();
//         //
//         mongodb::bson::from_bson::<User>(bson).map_or(None, |x| Some(x))
//     }
// }

async fn test_init() -> anyhow::Result<()> {
    let cfg: Config = Default::default();
    init(&cfg).await
}

#[tokio::test]
async fn mc_1() -> anyhow::Result<()> {
    let cfg: Config = Default::default();
    init(&cfg).await?;

    // Spawn tasks
    let _futures = (0..10)
        .map(|id| {
            // println!("----id {}---", id);
            let c = cnt();
            tokio::spawn(async move {
                println!("---inner-id {}---", id);
                let _ = c.list_database_names(None, None).await.map(|l| {
                    for str in l.iter() {
                        println!("-{}---db---{}--", id, str);
                    }
                });

                tokio::time::sleep(Duration::from_secs(5u64)).await;
                println!("--{}  after sleep-----", id);
            })
        })
        .collect::<Vec<_>>();
    // for future in futures {
    //     future.await.unwrap();
    // }

    let _ = tokio::spawn(async move {
        println!("----after sleep-----");
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(20)).await;
            println!("----after sleep-----");
        }
    })
    .await;

    Ok(())
}

#[tokio::test]
async fn mc_2() -> anyhow::Result<()> {
    test_init().await?;

    let r: anyhow::Result<Option<Document>> = raw_find_one(
        "test",
        "a",
        doc! {
            "_id":3,
        },
        None,
    )
    .await;
    println!("----test_cnt.rs---{:?}----", r);

    Ok(())
}

#[tokio::test]
async fn mc_3() -> anyhow::Result<()> {
    test_init().await?;

    let r: anyhow::Result<Option<Document>> = raw_find_one(
        "test",
        "a",
        doc! {
            "_id":3,
        },
        None,
    )
    .await;
    println!("----test_cnt.rs---{:?}----", r);

    let r = r?.unwrap();
    let u = User::from(r);

    println!("----test_cnt.rs---{:?}----", u);
    Ok(())
}

#[tokio::test]
async fn mc_4() -> anyhow::Result<()> {
    test_init().await?;

    let r = raw_insert_one(
        "test",
        "a",
        doc! {
            "_id":4,
            "title":"4_title".to_string(),
            "author":"4_author".to_string(),
        },
        None,
    )
    .await;
    println!("----test_cnt.rs---{:?}----", r);
    Ok(())
}

#[tokio::test]
async fn mg_insert_1() -> anyhow::Result<()> {
    test_init().await?;

    let doc: Document = User {
        id: Some(555),
        title: Some("5 title".to_string()),
        author: Some("5 author".to_string()),
    }
    .into();

    let r = raw_insert_one("test", "a", doc, None).await;
    println!("----test_cnt.rs---{:?}----", r);

    Ok(())
}

#[tokio::test]
async fn mc_6() -> anyhow::Result<()> {
    use futures::stream::{StreamExt, TryStreamExt};

    test_init().await?;

    let mut c: mongodb::Cursor<Document> = raw_find_many("test", "a", doc! {"_id":3}, None).await?;
    println!("----test_cnt.rs---{:?}----", c);
    println!("----");
    println!("----");
    println!("----");
    println!("----");
    while let Some(row) = c.next().await {
        println!("--{:?}----", row);
    }

    Ok(())
}

#[tokio::test]
async fn mc_7() -> anyhow::Result<()> {
    use futures::stream::{StreamExt, TryStreamExt};
    test_init().await?;
    let r: User = find_one(
        "test",
        "a",
        doc! {
            "_id":4,
        },
        None,
    )
    .await?;
    println!("----test_api.rs---{:?}-----", r);

    Ok(())
}

#[tokio::test]
async fn mc_8() -> anyhow::Result<()> {
    use futures::stream::{StreamExt, TryStreamExt};

    test_init().await?;

    let r = raw_insert_one(
        "test",
        "aaa",
        User {
            id: Some(16),
            title: Some("16 title".to_string()),
            author: Some("16 autoor".to_string()),
        },
        None,
    )
    .await?;
    println!("-----id---{:?}---", r.inserted_id.to_string());
    println!("----r---{:?}-----", r);

    Ok(())
}

//find_many
#[tokio::test]
async fn mg_find_1() -> anyhow::Result<()> {
    use futures::stream::{StreamExt, TryStreamExt};

    test_init().await?;

    let r: User = raw_find_one("test", "aaa", None, None).await?.unwrap();
    println!("----r---{:?}-----", r);

    Ok(())
}

#[tokio::test]
async fn mc_exist_1() -> anyhow::Result<()> {
    use futures::stream::{StreamExt, TryStreamExt};

    test_init().await?;

    let r: bool = exist("test", "whr5555a", None).await?;
    println!("----r---{:?}-----", r);

    Ok(())
}

#[tokio::test]
async fn mg_count_1() -> anyhow::Result<()> {
    use futures::stream::{StreamExt, TryStreamExt};

    test_init().await?;

    let r = count("test", "a", None, None).await?;
    println!("----count---{:?}-----", r);

    Ok(())
}

#[tokio::test]
async fn mg_update_one_1() -> anyhow::Result<()> {
    use futures::stream::{StreamExt, TryStreamExt};

    test_init().await?;

    let up = doc! {
        "$set":{
            "title":"ttttttt".to_string()
            }
    };

    let r = update_one("test", "a", doc! {"_id":16}, up, None).await?;
    println!("----count---{:?}-----", r);

    Ok(())
}

#[tokio::test]
async fn mg_update_many_1() -> anyhow::Result<()> {
    use futures::stream::{StreamExt, TryStreamExt};

    test_init().await?;

    let up = doc! {
        "$set":{
            "title":"so good".to_string()
            }
    };

    let r = update_many(
        "test",
        "a",
        doc! {"_id":
        {"$gte":16}},
        up,
        None,
    )
    .await?;
    println!("----count---{:?}-----", r);

    Ok(())
}

//find_many
#[tokio::test]
async fn mg_page_1() -> anyhow::Result<()> {
    use mongodb::options::FindOptions;

    test_init().await?;

    let mut opt: FindOptions = Default::default();

    opt.limit = Some(10);
    opt.skip = Some(1);
    opt.sort = Some(doc! {
       "_id":1
    });

    const DB: &str = "test";
    const TB: &str = "a";

    let r: Vec<User> = find_many(DB, TB, None, Some(opt)).await?;
    println!("----r---{:?}-----", r);

    Ok(())
}

#[tokio::test]
async fn mc_15() -> anyhow::Result<()> {
    use mongodb::options::FindOptions;

    test_init().await?;

    let mut pg = Page::new();
    pg.rows_per_page = 2;
    pg.page_no = 1;
    pg.filter = Some(doc! {
      "_id":{
            "$gte":3,
        }
    });

    let r: Page<User> = page("test", "a", pg).await?;
    println!("----page bean---{:?}-----", r);

    Ok(())
}

#[tokio::test]
async fn raw_aggre_1() -> anyhow::Result<()> {
    let cfg: Config = Default::default();
    init(&cfg).await?;
    //-------------------------------------

    let r = raw_aggregate(
        "telinfo",
        "tel_chat",
        vec![doc! {
            "$group":{
                "_id":null,
                "max":{"$max":"$_id"}
            }
        }],
        None,
    )
    .await?;
    println!("-----------{:?}-----------", r);

    //-------------------------------------
    let r = raw_aggregate(
        "telinfo",
        "tel_chat",
        vec![doc! {
            "$group":{
                "_id":null,
                "min":{"$min":"$_id"}
            }
        }],
        None,
    )
    .await?;
    println!("-----------{:?}-----------", r);
    //-------------------------------------
    let r = raw_aggregate(
        "telinfo",
        "tel_chat",
        vec![doc! {
            "$group":{
                "_id":null,
                "avg":{"$avg":"$_id"}
            }
        }],
        None,
    )
    .await?;
    println!("-----------{:?}-----------", r);
    //-------------------------------------
    let r = raw_aggregate(
        "telinfo",
        "tel_chat",
        vec![doc! {
            "$group":{
                "_id":null,
                "sum":{"$sum":"$_id"}
            }
        }],
        None,
    )
    .await?;
    println!("-----------{:?}-----------", r);

    Ok(())
}

#[tokio::test]
async fn aggre_1() -> anyhow::Result<()> {
    let cfg: Config = Default::default();
    init(&cfg).await?;
    let r: Vec<User> = aggregate(
        "telinfo",
        "tel_chat",
        vec![doc! {
         "$group":{
         "_id":1,
         "max":{
                 "$max":"$_id"
             }
         }
        }],
        None,
    )
    .await?;

    println!("-----------{:?}-----------", r);
    let r: Vec<Document> = aggregate(
        "telinfo",
        "tel_chat",
        vec![doc! {
         "$project":{
           "_id":5555,
           "title":"title"
         }
        }],
        None,
    )
    .await?;

    println!("-----------{:?}-----------", r);

    Ok(())
}
#[tokio::test]
async fn aggre_min_1() -> anyhow::Result<()> {
    let cfg: Config = Default::default();
    init(&cfg).await?;

    //--------------------------
    let r: f32 = min("telinfo", "tel_chat", doc! {}, "_id").await?;
    println!("--------min---{}-----------", r);
    //--------------------------
    let r: f32 = max("telinfo", "tel_chat", doc! {}, "_id").await?;
    println!("------max-----{}-----------", r);
    //--------------------------
    let r: f64 = avg("telinfo", "tel_chat", doc! {}, "_id").await?;
    println!("-----avg------{}-----------", r);
    //--------------------------
    let r: f64 = sum("telinfo", "tel_chat", doc! {}, "_id").await?;
    println!("---sum--------{}-----------", r);

    Ok(())
}

#[tokio::test]
async fn test_tx_1() -> anyhow::Result<()> {
    init_modules(None).await?;
    let cnt = cnt();
    // let db = cnt.database("test");
    let mut session = cnt.start_session(None).await?;
    let options = TransactionOptions::builder()
        .read_concern(ReadConcern::majority())
        .write_concern(WriteConcern::builder().w(Acknowledgment::Majority).build())
        .build();
    session.start_transaction(options).await?;
    // db.collection("abc").insert_one_with_session();
    Ok(())
}

#[tokio::test]
async fn test_tx_2() -> anyhow::Result<()> {
    init_modules(None).await?;

    for id in 1002..1500 {
        let mut tx = get_tx().await?;
        debug!("--after get_tx-------");
        let doc: Document = User {
            id: Some(id),
            title: Some("5 title".to_string()),
            author: Some("5 author".to_string()),
        }
        .into();

        let r = tx_raw_insert_one("test", "aa", doc, None, &mut tx).await;
        println!("----after insert---{r:#?}----");
        let r = tx.commit_transaction().await?;
        println!("----after commit {r:?}-----------",);
        println!("-----------no: {id}-----------",);
    }

    //
    Ok(())
}
