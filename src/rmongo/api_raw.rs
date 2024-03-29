use tracing::*;
use mongodb::bson::{doc, Document};
use mongodb::options::{
    AggregateOptions, CountOptions, DeleteOptions, FindOneOptions, FindOptions, InsertManyOptions,
    InsertOneOptions, UpdateModifications, UpdateOptions,
};
use mongodb::results::{DeleteResult, InsertManyResult, InsertOneResult, UpdateResult};
use std::borrow::Borrow;

use futures::stream::{StreamExt, TryStreamExt};
use mongodb::{error::Result, options::ClientOptions, Client, ClientSession};
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
) -> anyhow::Result<Option<T>>
where
    T: Serialize + DeserializeOwned + Unpin + Debug + std::marker::Send + Sync,
{
    let c = cnt();
    let tb = c.database(db).collection::<T>(tb);
    let r = tb.find_one(filter, options).await?;
    Ok(r)
}

pub async fn tx_raw_find_one<T>(
    db: &str,
    tb: &str,
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<FindOneOptions>>,
    tx: &mut ClientSession,
) -> anyhow::Result<Option<T>>
where
    T: Serialize + DeserializeOwned + Unpin + Debug + std::marker::Send + Sync,
{
    let c = cnt();
    let tb = c.database(db).collection::<T>(tb);
    let r = tb.find_one_with_session(filter, options, tx).await?;
    Ok(r)
}

pub async fn raw_exist(
    db: &str,
    tb: &str,
    filter: impl Into<Option<Document>>,
) -> anyhow::Result<bool> {
    let r: Option<Document> = raw_find_one(db, tb, filter, None).await?;
    Ok(r.is_some())
}

pub async fn tx_raw_exist(
    db: &str,
    tb: &str,
    filter: impl Into<Option<Document>>,
    tx: &mut ClientSession,
) -> anyhow::Result<bool> {
    let r: Option<Document> = tx_raw_find_one(db, tb, filter, None, tx).await?;
    Ok(r.is_some())
}

pub async fn raw_count(
    db: &str,
    tb: &str,
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<CountOptions>>,
) -> anyhow::Result<u64> {
    let c = cnt();
    let tb = c.database(db).collection::<Document>(tb);
    let c = tb.count_documents(filter, options).await?;
    Ok(c)
}

pub async fn tx_raw_count(
    db: &str,
    tb: &str,
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<CountOptions>>,
    tx: &mut ClientSession,
) -> anyhow::Result<u64> {
    let c = cnt();
    let tb = c.database(db).collection::<Document>(tb);
    let c = tb.count_documents_with_session(filter, options, tx).await?;
    Ok(c)
}

pub async fn raw_find_many<T>(
    db: &str,
    tb: &str,
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<FindOptions>>,
) -> anyhow::Result<mongodb::Cursor<T>>
where
    T: Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
{
    let c = cnt();
    let tb = c.database(db).collection::<T>(tb);
    let r = tb.find(filter, options).await?;
    Ok(r)
}

pub async fn tx_raw_find_many<T>(
    db: &str,
    tb: &str,
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<FindOptions>>,
    tx: &mut ClientSession,
) -> anyhow::Result<mongodb::SessionCursor<T>>
where
    T: Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
{
    let c = cnt();
    let tb = c.database(db).collection::<T>(tb);
    let r = tb.find_with_session(filter, options, tx).await?;
    Ok(r)
}

pub async fn raw_find_many_doc(
    db: &str,
    tb: &str,
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<FindOptions>>,
) -> anyhow::Result<Vec<Document>> {
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

pub async fn tx_raw_find_many_doc(
    db: &str,
    tb: &str,
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<FindOptions>>,
    tx: &mut ClientSession,
) -> anyhow::Result<Vec<Document>> {
    use futures::stream::{StreamExt, TryStreamExt};

    let mut c = tx_raw_find_many(db, tb, filter, options, tx).await?;

    let mut l: Vec<Document> = Vec::new();
    while let Some(row) = c.next(tx).await {
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
) -> anyhow::Result<InsertOneResult>
where
    T: Serialize + DeserializeOwned + Unpin + Debug,
{
    let c = cnt();
    let tb = c.database(db).collection::<T>(tb);
    let r = tb.insert_one(doc, options).await?;
    Ok(r)
}
pub async fn tx_raw_insert_one<T>(
    db: &str,
    tb: &str,
    doc: T,
    options: impl Into<Option<InsertOneOptions>>,
    tx: &mut ClientSession,
) -> anyhow::Result<InsertOneResult>
where
    T: Serialize + DeserializeOwned + Unpin + Debug,
{
    let c = cnt();
    let tb = c.database(db).collection::<T>(tb);
    let r = tb.insert_one_with_session(doc, options, tx).await?;
    Ok(r)
}

pub async fn raw_insert_many<T>(
    db: &str,
    tb: &str,
    doc: Vec<T>,
    options: impl Into<Option<InsertManyOptions>>,
) -> anyhow::Result<InsertManyResult>
where
    T: Serialize + DeserializeOwned + Unpin + Debug,
{
    let c = cnt();
    let tb = c.database(db).collection::<T>(tb);
    let r = tb.insert_many(doc, options).await?;
    Ok(r)
}

pub async fn tx_raw_insert_many<T>(
    db: &str,
    tb: &str,
    doc: Vec<T>,
    options: impl Into<Option<InsertManyOptions>>,
    tx: &mut ClientSession,
) -> anyhow::Result<InsertManyResult>
where
    T: Serialize + DeserializeOwned + Unpin + Debug,
{
    let c = cnt();
    let tb = c.database(db).collection::<T>(tb);
    let r = tb.insert_many_with_session(doc, options, tx).await?;
    Ok(r)
}

pub async fn raw_delete_one(
    db: &str,
    tb: &str,
    doc: Document,
    options: impl Into<Option<DeleteOptions>>,
) -> anyhow::Result<DeleteResult> {
    let c = cnt();
    let tb = c.database(db).collection::<Document>(tb);
    let r = tb.delete_one(doc, options).await?;
    Ok(r)
}

pub async fn tx_raw_delete_one(
    db: &str,
    tb: &str,
    doc: Document,
    options: impl Into<Option<DeleteOptions>>,
    tx: &mut ClientSession,
) -> anyhow::Result<DeleteResult> {
    let c = cnt();
    let tb = c.database(db).collection::<Document>(tb);
    let r = tb.delete_one_with_session(doc, options, tx).await?;
    Ok(r)
}

pub async fn raw_delete_many(
    db: &str,
    tb: &str,
    doc: Document,
    options: impl Into<Option<DeleteOptions>>,
) -> anyhow::Result<DeleteResult> {
    let c = cnt();
    let tb = c.database(db).collection::<Document>(tb);
    let r = tb.delete_many(doc, options).await?;
    Ok(r)
}

pub async fn tx_raw_delete_many(
    db: &str,
    tb: &str,
    doc: Document,
    options: impl Into<Option<DeleteOptions>>,
    tx: &mut ClientSession,
) -> anyhow::Result<DeleteResult> {
    let c = cnt();
    let tb = c.database(db).collection::<Document>(tb);
    let r = tb.delete_many_with_session(doc, options, tx).await?;
    Ok(r)
}

pub async fn raw_update_one(
    db: &str,
    tb: &str,
    doc: Document,
    update: impl Into<UpdateModifications>,
    options: impl Into<Option<UpdateOptions>>,
) -> anyhow::Result<UpdateResult> {
    let c = cnt();
    let tb = c.database(db).collection::<Document>(tb);
    let r = tb.update_one(doc, update, options).await?;
    Ok(r)
}

pub async fn tx_raw_update_one(
    db: &str,
    tb: &str,
    doc: Document,
    update: impl Into<UpdateModifications>,
    options: impl Into<Option<UpdateOptions>>,
    tx: &mut ClientSession,
) -> anyhow::Result<UpdateResult> {
    let c = cnt();
    let tb = c.database(db).collection::<Document>(tb);
    let r = tb.update_one_with_session(doc, update, options, tx).await?;
    Ok(r)
}

pub async fn raw_update_many(
    db: &str,
    tb: &str,
    doc: Document,
    update: impl Into<UpdateModifications>,
    options: impl Into<Option<UpdateOptions>>,
) -> anyhow::Result<UpdateResult> {
    let c = cnt();
    let tb = c.database(db).collection::<Document>(tb);
    let r = tb.update_many(doc, update, options).await?;
    Ok(r)
}
pub async fn tx_raw_update_many(
    db: &str,
    tb: &str,
    doc: Document,
    update: impl Into<UpdateModifications>,
    options: impl Into<Option<UpdateOptions>>,
    tx: &mut ClientSession,
) -> anyhow::Result<UpdateResult> {
    let c = cnt();
    let tb = c.database(db).collection::<Document>(tb);
    let r = tb
        .update_many_with_session(doc, update, options, tx)
        .await?;
    Ok(r)
}

pub async fn raw_aggregate(
    db: &str,
    tb: &str,
    pipeline: impl IntoIterator<Item = Document>,
    options: impl Into<Option<AggregateOptions>>,
) -> anyhow::Result<Vec<Document>> {
    let c = cnt();
    let tb = c.database(db).collection::<Document>(tb);
    let mut cursor = tb.aggregate(pipeline, options).await?;

    let mut l: Vec<Document> = Vec::new();
    while let Some(row) = cursor.next().await {
        if let Ok(row) = row {
            l.push(row.clone());
        }
    }

    debug!("--raw_aggregate: {l:#?}-------");

    Ok(l)
}

pub async fn tx_raw_aggregate(
    db: &str,
    tb: &str,
    pipeline: impl IntoIterator<Item = Document>,
    options: impl Into<Option<AggregateOptions>>,
    tx: &mut ClientSession,
) -> anyhow::Result<Vec<Document>> {
    let c = cnt();
    let tb = c.database(db).collection::<Document>(tb);
    let mut cursor = tb.aggregate_with_session(pipeline, options, tx).await?;

    let mut l: Vec<Document> = Vec::new();
    while let Some(row) = cursor.next(tx).await {
        if let Ok(row) = row {
            l.push(row.clone());
        }
    }

    Ok(l)
}
