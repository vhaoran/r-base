use super::*;
use crate::{g, init_module_n};
use tracing::*;
use polodb_core::bson::oid::ObjectId;
use polodb_core::bson::{doc, Document};
use polodb_core::results::{DeleteResult, InsertOneResult, UpdateResult};
use polodb_core::{bson, ClientCursor};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub fn insert_one<T>(tb: &str, doc: T) -> anyhow::Result<InsertOneResult>
where
    T: Serialize + DeserializeOwned + Unpin + Debug,
{
    raw_insert_one::<T>(tb, doc)
}

pub fn delete_one(tb: &str, filter: Document) -> anyhow::Result<DeleteResult> {
    raw_del_one(tb, filter)
}

pub fn delete_many(tb: &str, filter: Document) -> anyhow::Result<DeleteResult> {
    raw_del_many(tb, filter)
}

pub fn update_one(tb: &str, doc: Document, up: Document) -> anyhow::Result<UpdateResult> {
    raw_update_one(tb, doc, up)
}

pub fn update_many(tb: &str, doc: Document, up: Document) -> anyhow::Result<UpdateResult> {
    raw_update_many(tb, doc, up)
}

pub fn find_many<T>(tb: &str, filter: impl Into<Option<Document>>) -> anyhow::Result<Vec<T>>
where
    T: Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
{
    raw_find_many::<T>(tb, filter)
}

pub fn find_one<T>(tb: &str, filter: impl Into<Option<Document>>) -> anyhow::Result<T>
where
    T: Serialize + DeserializeOwned + Unpin + Debug + std::marker::Send + Sync,
{
    raw_find_one::<T>(tb, filter)
}

pub fn count(tb: &str, filter: Document) -> anyhow::Result<i64> {
    raw_count(tb, filter)
}
pub fn exist(tb: &str, filter: impl Into<Option<Document>>) -> bool {
    raw_exist(tb, filter)
}
