#[macro_export]
macro_rules! mongo_base_dyn {
    ($db:expr,$tb:expr,$T:ident) => {
        use mongodb::bson::{doc, Document};
        use mongodb::options::{
            AggregateOptions, CountOptions, FindOneOptions, FindOptions, InsertManyOptions,
            InsertOneOptions,
        };
        //for tx
        // use anyhow::anyhow;
        use mongodb::results::{InsertManyResult, InsertOneResult};
        use mongodb::{options::ClientOptions, Client, ClientSession};
        use serde::{de::DeserializeOwned, Deserialize, Serialize};
        use std::borrow::Borrow;
        use $crate::rmongo::Page;
        // use serde::{Deserialize, Serialize};
        use mongodb::options::{DeleteOptions, UpdateModifications, UpdateOptions};
        use mongodb::results::{DeleteResult, UpdateResult};
        use std::any::Any;
        use std::default::Default;
        use std::fmt::Debug;
        use std::str::FromStr;

        pub fn tb_name() -> String {
            let s = format!("{}", $tb);
            s
        }

        pub async fn find_one<$T>(
            filter: impl Into<Option<Document>>,
            options: impl Into<Option<FindOneOptions>>,
        ) -> anyhow::Result<$T>
        where
            $T: Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
        {
            let db = $db.await;
            let r = $crate::rmongo::find_one(db.as_str(), $tb, filter, options).await?;
            Ok(r)
        }

        /// tx
        pub async fn tx_find_one<$T>(
            filter: impl Into<Option<Document>>,
            options: impl Into<Option<FindOneOptions>>,
            tx: &mut ClientSession,
        ) -> anyhow::Result<$T>
        where
            $T: Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
        {
            let db = $db.await;
            let r = $crate::rmongo::tx_find_one(db.as_str(), $tb, filter, options, tx).await?;
            Ok(r)
        }

        pub async fn exist(filter: impl Into<Option<Document>>) -> anyhow::Result<bool> {
            let db = $db.await;
            let r = $crate::rmongo::exist(db.as_str(), $tb, filter).await?;
            Ok(r)
        }

        pub async fn tx_exist(
            filter: impl Into<Option<Document>>,
            tx: &mut ClientSession,
        ) -> anyhow::Result<bool> {
            let db = $db.await;
            let r = $crate::rmongo::tx_exist(db.as_str(), $tb, filter, tx).await?;
            Ok(r)
        }

        pub async fn count(
            filter: impl Into<Option<Document>>,
            options: impl Into<Option<CountOptions>>,
        ) -> anyhow::Result<u64> {
            let db = $db.await;
            let r = $crate::rmongo::count(db.as_str(), $tb, filter, options).await?;
            Ok(r)
        }

        pub async fn tx_count(
            filter: impl Into<Option<Document>>,
            options: impl Into<Option<CountOptions>>,
            tx: &mut ClientSession,
        ) -> anyhow::Result<u64> {
            let db = $db.await;
            let r = $crate::rmongo::tx_count(db.as_str(), $tb, filter, options, tx).await?;
            Ok(r)
        }

        pub async fn insert_one<$T>(
            doc: $T,
            options: impl Into<Option<InsertOneOptions>>,
        ) -> anyhow::Result<InsertOneResult>
        where
            $T: Serialize + DeserializeOwned + Unpin + Debug,
        {
            let db = $db.await;
            let r = $crate::rmongo::insert_one(db.as_str(), $tb, doc, options).await?;
            Ok(r)
        }

        pub async fn tx_insert_one<$T>(
            doc: $T,
            options: impl Into<Option<InsertOneOptions>>,
            tx: &mut ClientSession,
        ) -> anyhow::Result<InsertOneResult>
        where
            $T: Serialize + DeserializeOwned + Unpin + Debug,
        {
            let db = $db.await;
            let r = $crate::rmongo::tx_insert_one(db.as_str(), $tb, doc, options, tx).await?;
            Ok(r)
        }

        pub async fn insert_many<$T>(
            doc: Vec<$T>,
            options: impl Into<Option<InsertManyOptions>>,
        ) -> anyhow::Result<InsertManyResult>
        where
            $T: Serialize + DeserializeOwned + Unpin + Debug,
        {
            let db = $db.await;
            let r = $crate::rmongo::insert_many(db.as_str(), $tb, doc, options).await?;
            Ok(r)
        }

        pub async fn tx_insert_many<$T>(
            doc: Vec<$T>,
            options: impl Into<Option<InsertManyOptions>>,
            tx: &mut ClientSession,
        ) -> anyhow::Result<InsertManyResult>
        where
            $T: Serialize + DeserializeOwned + Unpin + Debug,
        {
            let db = $db.await;
            let r = $crate::rmongo::tx_insert_many(db.as_str(), $tb, doc, options, tx).await?;
            Ok(r)
        }

        pub async fn find_many<$T>(
            filter: impl Into<Option<Document>>,
            options: Option<FindOptions>,
        ) -> anyhow::Result<Vec<$T>>
        where
            $T: Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
        {
            let db = $db.await;
            let r = $crate::rmongo::find_many::<$T>(db.as_str(), $tb, filter, options).await?;
            Ok(r)
        }

        pub async fn tx_find_many<$T>(
            filter: impl Into<Option<Document>>,
            options: Option<FindOptions>,
            tx: &mut ClientSession,
        ) -> anyhow::Result<Vec<$T>>
        where
            $T: Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
        {
            let db = $db.await;
            let r = $crate::rmongo::tx_find_many(db.as_str(), $tb, filter, options, tx).await?;
            Ok(r)
        }

        pub async fn find_many_fields<$T>(
            filter: impl Into<Option<Document>>,
            fields: Option<Document>,
            limit: Option<i64>,
        ) -> anyhow::Result<Vec<$T>>
        where
            $T: Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
        {
            let db = $db.await;
            let r =
                $crate::rmongo::find_many_fields(db.as_str(), $tb, filter, fields, limit).await?;
            Ok(r)
        }

        pub async fn tx_find_many_fields<$T>(
            filter: impl Into<Option<Document>>,
            fields: Option<Document>,
            limit: Option<i64>,
            tx: &mut ClientSession,
        ) -> anyhow::Result<Vec<$T>>
        where
            $T: Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
        {
            let db = $db.await;
            let r =
                $crate::rmongo::tx_find_many_fields(db.as_str(), $tb, filter, fields, limit, tx)
                    .await?;
            Ok(r)
        }

        pub async fn delete_one(
            filter: Document,
            options: impl Into<Option<DeleteOptions>>,
        ) -> anyhow::Result<DeleteResult> {
            let db = $db.await;
            let r = $crate::rmongo::delete_one(db.as_str(), $tb, filter, options).await?;
            Ok(r)
        }

        pub async fn tx_delete_one(
            filter: Document,
            options: impl Into<Option<DeleteOptions>>,
            tx: &mut ClientSession,
        ) -> anyhow::Result<DeleteResult> {
            let db = $db.await;
            let r = $crate::rmongo::tx_delete_one(db.as_str(), $tb, filter, options, tx).await?;
            Ok(r)
        }

        pub async fn delete_many(
            filter: Document,
            options: impl Into<Option<DeleteOptions>>,
        ) -> anyhow::Result<DeleteResult> {
            let db = $db.await;
            let r = $crate::rmongo::delete_many(db.as_str(), $tb, filter, options).await?;
            Ok(r)
        }

        pub async fn tx_delete_many(
            filter: Document,
            options: impl Into<Option<DeleteOptions>>,
            tx: &mut ClientSession,
        ) -> anyhow::Result<DeleteResult> {
            let db = $db.await;
            let r = $crate::rmongo::tx_delete_many(db.as_str(), $tb, filter, options, tx).await?;
            Ok(r)
        }

        pub async fn update_one(
            filter: Document,
            update: impl Into<UpdateModifications>,
            options: impl Into<Option<UpdateOptions>>,
        ) -> anyhow::Result<UpdateResult> {
            let db = $db.await;
            let r = $crate::rmongo::update_one(db.as_str(), $tb, filter, update, options).await?;
            Ok(r)
        }

        pub async fn tx_update_one(
            filter: Document,
            update: impl Into<UpdateModifications>,
            options: impl Into<Option<UpdateOptions>>,
            tx: &mut ClientSession,
        ) -> anyhow::Result<UpdateResult> {
            let db = $db.await;
            let r = $crate::rmongo::tx_update_one(db.as_str(), $tb, filter, update, options, tx)
                .await?;
            Ok(r)
        }

        pub async fn update_many(
            filter: Document,
            update: impl Into<UpdateModifications>,
            options: impl Into<Option<UpdateOptions>>,
        ) -> anyhow::Result<UpdateResult> {
            let db = $db.await;
            let r = $crate::rmongo::update_many(db.as_str(), $tb, filter, update, options).await?;
            Ok(r)
        }

        pub async fn tx_update_many(
            filter: Document,
            update: impl Into<UpdateModifications>,
            options: impl Into<Option<UpdateOptions>>,
            tx: &mut ClientSession,
        ) -> anyhow::Result<UpdateResult> {
            let db = $db.await;
            let r = $crate::rmongo::tx_update_many(db.as_str(), $tb, filter, update, options, tx)
                .await?;
            Ok(r)
        }

        pub async fn page(filter: Page<$T>) -> anyhow::Result<Page<$T>>
        where
            $T: Debug + Clone + Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
        {
            let db = $db.await;
            let r: Page<$T> = $crate::rmongo::page(db.as_str(), $tb, filter).await?;
            Ok(r)
        }

        pub async fn tx_page(filter: Page<$T>, tx: &mut ClientSession) -> anyhow::Result<Page<$T>>
        where
            $T: Debug + Clone + Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
        {
            let db = $db.await;
            let r: Page<$T> = $crate::rmongo::tx_page(db.as_str(), $tb, filter, tx).await?;
            Ok(r)
        }

        pub async fn aggregate<T>(
            pipeline: impl IntoIterator<Item = Document>,
            options: impl Into<Option<AggregateOptions>>,
        ) -> anyhow::Result<Vec<T>>
        where
            T: Debug + Clone + Serialize + DeserializeOwned + Unpin + Send + Sync,
        {
            let db = $db.await;
            let r: Vec<T> = $crate::rmongo::aggregate(db.as_str(), $tb, pipeline, options).await?;
            Ok(r)
        }

        pub async fn tx_aggregate<T>(
            pipeline: impl IntoIterator<Item = Document>,
            options: impl Into<Option<AggregateOptions>>,
            tx: &mut ClientSession,
        ) -> anyhow::Result<Vec<T>>
        where
            T: Debug + Clone + Serialize + DeserializeOwned + Unpin + Send + Sync,
        {
            let db = $db.await;
            let r: Vec<T> =
                $crate::rmongo::tx_aggregate(db.as_str(), $tb, pipeline, options, tx).await?;
            Ok(r)
        }

        pub async fn raw_aggregate(
            pipeline: impl IntoIterator<Item = Document>,
            options: impl Into<Option<AggregateOptions>>,
        ) -> anyhow::Result<Vec<Document>> {
            let db = $db.await;
            let r: Vec<Document> =
                $crate::rmongo::raw_aggregate(db.as_str(), $tb, pipeline, options).await?;
            Ok(r)
        }

        pub async fn tx_raw_aggregate(
            pipeline: impl IntoIterator<Item = Document>,
            options: impl Into<Option<AggregateOptions>>,
            tx: &mut ClientSession,
        ) -> anyhow::Result<Vec<Document>> {
            let db = $db.await;
            let r: Vec<Document> =
                $crate::rmongo::tx_raw_aggregate(db.as_str(), $tb, pipeline, options, tx).await?;
            Ok(r)
        }

        pub async fn min<T>(doc: Document, field_name: &str) -> anyhow::Result<T>
        where
            T: FromStr,
        {
            let db = $db.await;
            let r: T = $crate::rmongo::min(db.as_str(), $tb, doc, field_name).await?;
            Ok(r)
        }

        pub async fn tx_min<T>(
            doc: Document,
            field_name: &str,
            tx: &mut ClientSession,
        ) -> anyhow::Result<T>
        where
            T: FromStr,
        {
            let db = $db.await;
            let r: T = $crate::rmongo::tx_min(db.as_str(), $tb, doc, field_name, tx).await?;
            Ok(r)
        }

        pub async fn max<T>(doc: Document, field_name: &str) -> anyhow::Result<T>
        where
            T: FromStr,
        {
            let db = $db.await;
            let r: T = $crate::rmongo::max(db.as_str(), $tb, doc, field_name).await?;
            Ok(r)
        }

        pub async fn tx_max<T>(
            doc: Document,
            field_name: &str,
            tx: &mut ClientSession,
        ) -> anyhow::Result<T>
        where
            T: FromStr,
        {
            let db = $db.await;
            let r: T = $crate::rmongo::tx_max(db.as_str(), $tb, doc, field_name, tx).await?;
            Ok(r)
        }

        pub async fn avg<T>(doc: Document, field_name: &str) -> anyhow::Result<T>
        where
            T: FromStr,
        {
            let db = $db.await;
            let r: T = $crate::rmongo::avg(db.as_str(), $tb, doc, field_name).await?;
            Ok(r)
        }

        pub async fn tx_avg<T>(
            doc: Document,
            field_name: &str,
            tx: &mut ClientSession,
        ) -> anyhow::Result<T>
        where
            T: FromStr,
        {
            let db = $db.await;
            let r: T = $crate::rmongo::tx_avg(db.as_str(), $tb, doc, field_name, tx).await?;
            Ok(r)
        }

        pub async fn sum<T>(doc: Document, field_name: &str) -> anyhow::Result<T>
        where
            T: FromStr,
        {
            let db = $db.await;
            let r: T = $crate::rmongo::sum(db.as_str(), $tb, doc, field_name).await?;
            Ok(r)
        }

        pub async fn tx_sum<T>(
            doc: Document,
            field_name: &str,
            tx: &mut ClientSession,
        ) -> anyhow::Result<T>
        where
            T: FromStr,
        {
            let db = $db.await;
            let r: T = $crate::rmongo::tx_sum(db.as_str(), $tb, doc, field_name, tx).await?;
            Ok(r)
        }
    };
}
