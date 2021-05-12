use mongodb::bson::{doc, Document};
use mongodb::options::{
    AggregateOptions, CountOptions, DeleteOptions, FindOneOptions, FindOptions, InsertManyOptions,
    InsertOneOptions, UpdateModifications, UpdateOptions,
};
use mongodb::results::{DeleteResult, InsertManyResult, InsertOneResult, UpdateResult};
use std::borrow::Borrow;

use futures::stream::{StreamExt, TryStreamExt};
use mongodb::{error::Result, options::ClientOptions, Client};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::any::Any;
use std::default::Default;
use std::fmt::Debug;

use super::*;

pub async fn raw_find_one<T>(
    db: &str,
    tb: &str,
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<FindOneOptions>>,
) -> Result<Option<T>>
where
    T: Serialize + DeserializeOwned + Unpin + Debug + std::marker::Send + Sync,
{
    let c = cnt();
    let tb = c.database(db).collection::<T>(tb);
    tb.find_one(filter, options).await
}

pub async fn raw_exist(db: &str, tb: &str, filter: impl Into<Option<Document>>) -> Result<bool> {
    let r: Option<Document> = raw_find_one(db, tb, filter, None).await?;
    Ok(r.is_some())
}

pub async fn raw_count(
    db: &str,
    tb: &str,
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<CountOptions>>,
) -> Result<u64> {
    let c = cnt();
    let tb = c.database(db).collection::<Document>(tb);
    let c = tb.count_documents(filter, options).await?;
    Ok(c)
}

pub async fn raw_find_many<T>(
    db: &str,
    tb: &str,
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<FindOptions>>,
) -> Result<mongodb::Cursor<T>>
where
    T: Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
{
    let c = cnt();
    let tb = c.database(db).collection::<T>(tb);
    tb.find(filter, options).await
}

pub async fn raw_find_many_doc(
    db: &str,
    tb: &str,
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<FindOptions>>,
) -> Result<Vec<Document>> {
    use futures::stream::{StreamExt, TryStreamExt};

    let mut c = raw_find_many(db, tb, filter, options).await?;

    let mut l: Vec<Document> = Vec::new();
    while let Some(row) = c.next().await {
        if row.is_ok() {
            l.push(row.unwrap());
        }
    }
    Ok(l)
}

pub async fn raw_insert_one<T>(
    db: &str,
    tb: &str,
    doc: T,
    options: impl Into<Option<InsertOneOptions>>,
) -> Result<InsertOneResult>
where
    T: Serialize + DeserializeOwned + Unpin + Debug,
{
    let c = cnt();
    let tb = c.database(db).collection::<T>(tb);
    tb.insert_one(doc, options).await
}

pub async fn raw_insert_many<T>(
    db: &str,
    tb: &str,
    doc: Vec<T>,
    options: impl Into<Option<InsertManyOptions>>,
) -> Result<InsertManyResult>
where
    T: Serialize + DeserializeOwned + Unpin + Debug,
{
    let c = cnt();
    let tb = c.database(db).collection::<T>(tb);
    tb.insert_many(doc, options).await
}

pub async fn raw_delete_one(
    db: &str,
    tb: &str,
    doc: Document,
    options: impl Into<Option<DeleteOptions>>,
) -> Result<DeleteResult> {
    let c = cnt();
    let tb = c.database(db).collection::<Document>(tb);
    tb.delete_one(doc, options).await
}

pub async fn raw_delete_many(
    db: &str,
    tb: &str,
    doc: Document,
    options: impl Into<Option<DeleteOptions>>,
) -> Result<DeleteResult> {
    let c = cnt();
    let tb = c.database(db).collection::<Document>(tb);
    tb.delete_many(doc, options).await
}

pub async fn raw_update_one(
    db: &str,
    tb: &str,
    doc: Document,
    update: impl Into<UpdateModifications>,
    options: impl Into<Option<UpdateOptions>>,
) -> Result<UpdateResult> {
    let c = cnt();
    let tb = c.database(db).collection::<Document>(tb);
    tb.update_one(doc, update, options).await
}

pub async fn raw_update_many(
    db: &str,
    tb: &str,
    doc: Document,
    update: impl Into<UpdateModifications>,
    options: impl Into<Option<UpdateOptions>>,
) -> Result<UpdateResult> {
    let c = cnt();
    let tb = c.database(db).collection::<Document>(tb);
    tb.update_many(doc, update, options).await
}

pub async fn raw_aggregate(
    db: &str,
    tb: &str,
    pipeline: impl IntoIterator<Item = Document>,
    options: impl Into<Option<AggregateOptions>>,
) -> Result<Vec<Document>> {
    let c = cnt();
    let tb = c.database(db).collection::<Document>(tb);
    let mut cursor = tb.aggregate(pipeline, options).await?;

    let mut l: Vec<Document> = Vec::new();
    while let Some(row) = cursor.next().await {
        if let Ok(row) = row {
            l.push(row.clone());
        }
    }

    Ok(l)
}
