use super::*;
use crate::{g, init_module_n};
use log::*;
use polodb_core::bson::oid::ObjectId;
use polodb_core::bson::{doc, Document};
use polodb_core::results::{DeleteResult, InsertOneResult, UpdateResult};
use polodb_core::{bson, ClientCursor, IndexModel, IndexOptions};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub async fn raw_insert_one<T>(tb: &str, doc: T) -> anyhow::Result<InsertOneResult>
where
    T: Serialize + DeserializeOwned + Unpin + Debug,
{
    let a = cnt();
    let id = g::random();
    debug!("-raw_insert_one-before_lock_polo: {id}-------");
    let c = a.lock().await;
    debug!("-raw_insert_one-after_lock_polo: {id}-------");

    let tb = c.collection::<T>(tb);
    let r = tb.insert_one(doc)?;

    Ok(r)
}

pub async fn raw_del_one(tb: &str, filter: Document) -> anyhow::Result<DeleteResult> {
    let a = cnt();
    let id = g::random();
    debug!("-raw_del_one-before_lock_polo: {id}-------");
    let c = a.lock().await;
    debug!("-raw_del_one-after_lock_polo: {id}-------");

    let tb = c.collection::<Document>(tb);
    let r = tb.delete_one(filter)?;
    Ok(r)
}
pub async fn raw_del_many(tb: &str, filter: Document) -> anyhow::Result<DeleteResult> {
    let a = cnt();
    let id = g::random();
    debug!("-before_lock_polo: raw_del_many- {id}-------");
    let c = a.lock().await;
    debug!("-after_lock_polo:  raw_del_many-: {id}-------");
    debug!("--talbe: {tb} filter: {filter:#?}-------");

    let tb = c.collection::<Document>(tb);
    debug!("--after_get_tb delete_many-------");

    let r = tb.delete_many(filter).map_err(|e| {
        error!("---delete_many_error---{}-", e.to_string());
        e
    })?;
    debug!("--after_delete_many_raw_del_many-------");
    Ok(r)
}

pub async fn raw_update_one(tb: &str, doc: Document, up: Document) -> anyhow::Result<UpdateResult> {
    let a = cnt();
    let id = g::random();
    debug!("-raw_update_one-before_lock_polo: {id}-------");
    let c = a.lock().await;
    debug!("-raw_update_one-after_lock_polo: {id}-------");

    let tb = c.collection::<Document>(tb);
    let r = tb.update_one(doc, up)?;
    Ok(r)
}

pub async fn raw_update_many(
    tb: &str,
    doc: Document,
    up: Document,
) -> anyhow::Result<UpdateResult> {
    let a = cnt();
    let id = g::random();
    debug!("-raw_update_many-before_lock_polo: {id}-------");
    let c = a.lock().await;
    debug!("-raw_update_many-after_lock_polo: {id}-------");

    let tb = c.collection::<Document>(tb);
    let r = tb.update_many(doc, up)?;
    Ok(r)
}

async fn _raw_find_many<T>(
    tb: &str,
    filter: impl Into<Option<Document>>,
) -> anyhow::Result<ClientCursor<T>>
where
    T: Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
{
    let a = cnt();
    let id = g::random();
    debug!("-_raw_find_many-before_lock_polo: {id}-------");
    let c = a.lock().await;
    debug!("-_raw_find_many-after_lock_polo: {id}-------");

    let tb = c.collection::<T>(tb);
    let r = tb.find(filter)?;

    Ok(r)
}

pub async fn raw_find_many<T>(
    tb: &str,
    filter: impl Into<Option<Document>>,
) -> anyhow::Result<Vec<T>>
where
    T: Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
{
    let r = _raw_find_many(tb, filter).await?;

    let mut l = Vec::<T>::new();
    for v in r {
        match v {
            Ok(v) => {
                l.push(v);
            }
            _ => {}
        }
    }

    Ok(l)
}

async fn _raw_find_one<T>(
    tb: &str,
    filter: impl Into<Option<Document>>,
) -> anyhow::Result<Option<T>>
where
    T: Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
{
    let a = cnt();
    let id = g::random();
    debug!("-before_lock_polo_raw_find_one-: {id}-------");
    let c = a.lock().await;
    debug!("-after_lock_polo:_raw_find_one- {id}-------");
    let doc: Option<Document> = filter.into();
    debug!("--tb: {tb}--filter: {doc:#?}-----");

    let tb = c.collection::<T>(tb);
    let r = tb.find_one(doc)?;
    Ok(r)
}

pub async fn raw_count(tb: &str, filter: Document) -> anyhow::Result<i64> {
    // todo-modify
    // avoid use this function
    // let r = _raw_find_many::<Document>(tb, filter)?;
    let a = cnt();
    let id = g::random();
    debug!("-raw_count-before_lock_polo: {id}-------");
    let c = a.lock().await;
    debug!("-raw_count-after_lock_polo: {id}-------");

    let tb = c.collection::<Document>(tb);
    let r = if filter.len() > 0 {
        tb.aggregate(vec![
            doc! {
                "$match": filter
            },
            doc! {
                "$count": "count",
            },
        ])
        .map_err(|e| {
            error!("---aggregate_error---{}-", e.to_string());
            e
        })?
    } else {
        tb.aggregate(vec![doc! {
            "$count": "count",
        }])
        .map_err(|e| {
            error!("---aggregate_error---{}-", e.to_string());
            e
        })?
    };
    match r.collect::<polodb_core::Result<Vec<Document>>>() {
        Ok(l) if l.len() == 1 => match l[0].get("count") {
            Some(v) => Ok(v.as_i64().unwrap_or(0)),
            _ => Err(anyhow!("not get result",)),
        },
        _ => Err(anyhow!("not get result",)),
    }
}

pub async fn raw_exist(tb: &str, filter: impl Into<Option<Document>>) -> bool {
    match _raw_find_one::<Document>(tb, filter).await {
        Ok(v) => v.is_some(),
        _ => false,
    }
}

pub async fn raw_find_one<T>(tb: &str, filter: impl Into<Option<Document>>) -> anyhow::Result<T>
where
    T: Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
{
    match _raw_find_one(tb, filter).await? {
        Some(v) => Ok(v),
        _ => Err(anyhow!("not found data in {tb}",)),
    }
}

pub async fn raw_create_index(tb: &str, data: IndexModel) -> anyhow::Result<()> {
    let a = cnt();
    let id = g::random();
    debug!("--raw_create_index-----before_lock_polo: {id}-------");
    let c = a.lock().await;
    debug!("-raw_create_index-after_lock_polo: {id}-------");

    let tb = c.collection::<Document>(tb);
    let _ = tb.create_index(data)?;
    Ok(())
}

#[tokio::test]
async fn test_insert_1() -> anyhow::Result<()> {
    // only init-log
    init_module_n(None, true, false).await?;

    #[derive(Debug, Serialize, Deserialize)]
    struct Book {
        #[serde(rename = "_id")]
        id: Option<ObjectId>,
        title: String,
        author: String,
    }

    let tb = "update-test";
    let cfg = Config { path: None };
    super::init(&cfg)?;
    //-------------------------------------
    for i in 0..5 {
        let r = raw_insert_one(
            tb,
            Book {
                id: Some(ObjectId::new()),
                title: "aa".to_string(),
                author: format!("{i}_author"),
            },
        )
        .await;
        println!("-----------{r:#?}-----------",);
    }

    //----find-one---------------------------------
    let r = raw_find_one::<Book>(
        tb,
        doc! {
            "author":"bbb"
        },
    )
    .await?;
    println!("-----find_one------{r:?}-----------",);
    //-------------------------------------
    let r = raw_update_one(
        tb,
        doc! {
            "title":"aa"
        },
        doc! {
            "$set":doc!{
                "title":"818181---update-one"
            }
        },
    )
    .await;
    println!("-----------{r:?}-----------",);
    //-------------------------------------
    let r = raw_update_many(
        tb,
        doc! {
            "title":"aa"
        },
        doc! {
            "$set":doc!{
                "author":"oooo um---update-one"
            }
        },
    )
    .await;
    println!("-----------{r:?}-----------",);

    let r = raw_find_many::<Book>(tb, None).await?;
    println!("-----------{r:#?}-----------",);
    Ok(())
}

#[tokio::test]
async fn test_del_1() -> anyhow::Result<()> {
    // only init-log
    init_module_n(None, true, false).await?;

    #[derive(Debug, Serialize, Deserialize)]
    struct Book {
        #[serde(rename = "_id")]
        id: Option<ObjectId>,
        title: String,
        author: String,
    }

    let tb = "update-test";
    let cfg = Config { path: None };
    super::init(&cfg)?;
    //-------------------------------------
    for i in 0..5 {
        let r = raw_insert_one(
            tb,
            Book {
                id: Some(ObjectId::new()),
                title: "aa".to_string(),
                author: format!("{i}_author"),
            },
        )
        .await;
        println!("-----------{r:#?}-----------",);
    }

    //-----------delete one--------------------------
    let r = raw_del_one(
        tb,
        doc! {
            "title":"aa"
        },
    )
    .await;
    println!("-----------del result: {r:?}-----------",);
    let r = raw_del_many(
        tb,
        doc! {
            "author":"bbb"
        },
    )
    .await;
    println!("-----------del result: {r:?}-----------",);
    //-------------------------------------
    let r = raw_find_many::<Book>(tb, None).await;
    println!("-----------{r:#?}-----------",);
    //-------------------------------------
    let r = raw_count(
        tb,
        doc! {
            "author":"1_author"
        },
    )
    .await?;
    println!("----------count:-{r:?}-----------",);
    //-------------------------------------
    let r = raw_exist(
        tb,
        doc! {
            "author":"1_1 author"
        },
    )
    .await;
    println!("----------exist:-{r:?}-----------",);

    Ok(())
}

#[tokio::test]
async fn test_bench() -> anyhow::Result<()> {
    // only init-log
    init_module_n(None, true, false).await?;

    #[derive(Debug, Serialize, Deserialize)]
    struct Book {
        #[serde(rename = "_id")]
        id: Option<ObjectId>,
        title: String,
        author: String,
    }

    let tb = "bench_test";
    let cfg = Config { path: None };
    super::init(&cfg)?;
    debug!("--after init of polo-------");

    let _ = raw_create_index(
        tb,
        IndexModel {
            keys: doc! {
                "_id": 1,
            },
            options: Some(IndexOptions {
                unique: Some(true),
                ..Default::default()
            }),
        },
    )
    .await;

    //-------------------------------------
    let h = 1_000_000;
    let mut start = g::unix_sec();
    let start_0 = start;
    for i in 0..h {
        let r = raw_insert_one(
            tb,
            Book {
                id: Some(ObjectId::new()),
                title: "aa".to_string(),
                author: format!("{i}_author"),
            },
        )
        .await;
        if r.is_err() {
            debug!("--{r:?}-------");
        }
        if i % 10000 == 0 {
            debug!("-----------secs: {}-----------", g::unix_sec() - start);
            start = g::unix_sec();
        }
    }

    debug!("-----------{}-----------", g::unix_sec() - start_0);
    Ok(())
}

#[tokio::test]
async fn test_xxx() -> anyhow::Result<()> {
    //
    // only init-log
    init_module_n(None, true, false).await?;
    debug!("--after init model-------");

    #[derive(Debug, Serialize, Deserialize)]
    struct Book {
        #[serde(rename = "_id")]
        id: Option<i64>,
        title: Option<String>,
    }

    let tb = "xxx";
    let cfg = Config::default();

    super::init(&cfg)?;
    debug!("--after init of polo-------");

    let _ = raw_insert_one(
        tb,
        Book {
            id: Some(1),
            title: Some("aa".to_string()),
        },
    )
    .await;
    let r = raw_find_many::<Document>(tb, None).await?;
    println!("-------find-many----{r:#?}-----------",);

    //-------------------------------------
    let a = cnt();
    let id = g::random();
    let c = a.lock().await;

    let tb = c.collection::<Document>(tb);
    let r = tb.aggregate(vec![
        doc! {
            "$match":doc!{
                "_id":doc!{
                    "$eq":1_i64
                }
            }
        },
        doc! {
            "$count": "count",
        },
    ])?;
    debug!("--after aggregate-------");
    let l: polodb_core::Result<Vec<Document>> = r.collect();
    for v in l {
        debug!("---aggregate:--------{v:#?}-----------",);
    }

    //-------------------------------------

    Ok(())
}

#[tokio::test]
async fn test_count() -> anyhow::Result<()> {
    //
    // only init-log
    init_module_n(None, true, false).await?;
    debug!("--after init model-------");

    #[derive(Debug, Serialize, Deserialize)]
    struct Book {
        #[serde(rename = "_id")]
        id: Option<i64>,
        title: Option<String>,
    }

    let tb = "xxx";
    let cfg = Config::default();

    super::init(&cfg)?;
    debug!("--after init of polo-------");

    let _ = raw_insert_one(
        tb,
        Book {
            id: Some(1),
            title: Some("aa".to_string()),
        },
    )
    .await;

    let r = raw_count(
        tb,
        doc! {
            "_id":{
                "$eq":1_i64
            },
        },
    )
    .await;
    println!("-----------{r:?}-----------",);

    Ok(())
}
