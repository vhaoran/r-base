#[macro_export]
macro_rules! mongo_base {
    ($db:expr,$tb:expr,$T:ident) => {
        use mongodb::bson::{doc, Document};
        use mongodb::options::{
            AggregateOptions, CountOptions, FindOneOptions, FindOptions, InsertManyOptions,
            InsertOneOptions,
        };
        use mongodb::results::{InsertManyResult, InsertOneResult};
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
        ) -> Result<$T, Box<dyn std::error::Error>>
        where
            $T: Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
        {
            let r = $crate::rmongo::find_one($db, $tb, filter, options).await?;
            Ok(r)
        }

        pub async fn exist(
            filter: impl Into<Option<Document>>,
        ) -> Result<bool, Box<dyn std::error::Error>> {
            let r = $crate::rmongo::exist($db, $tb, filter).await?;
            Ok(r)
        }

        pub async fn count(
            filter: impl Into<Option<Document>>,
            options: impl Into<Option<CountOptions>>,
        ) -> Result<u64, Box<dyn std::error::Error>> {
            let r = $crate::rmongo::count($db, $tb, filter, options).await?;
            Ok(r)
        }

        pub async fn insert_one<$T>(
            doc: $T,
            options: impl Into<Option<InsertOneOptions>>,
        ) -> Result<InsertOneResult, Box<dyn std::error::Error>>
        where
            $T: Serialize + DeserializeOwned + Unpin + Debug,
        {
            let r = $crate::rmongo::insert_one($db, $tb, doc, options).await?;
            Ok(r)
        }

        pub async fn insert_many<$T>(
            doc: Vec<$T>,
            options: impl Into<Option<InsertManyOptions>>,
        ) -> Result<InsertManyResult, Box<dyn std::error::Error>>
        where
            $T: Serialize + DeserializeOwned + Unpin + Debug,
        {
            let r = $crate::rmongo::insert_many($db, $tb, doc, options).await?;
            Ok(r)
        }

        pub async fn find_many<$T>(
            filter: impl Into<Option<Document>>,
            options: Option<FindOptions>,
        ) -> Result<Vec<$T>, Box<dyn std::error::Error>>
        where
            $T: Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
        {
            let r = $crate::rmongo::find_many($db, $tb, filter, options).await?;
            Ok(r)
        }

        pub async fn find_many_fields<$T>(
            filter: impl Into<Option<Document>>,
            fields: Option<Document>,
            limit: Option<i64>,
        ) -> Result<Vec<$T>, Box<dyn std::error::Error>>
        where
            $T: Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
        {
            let r = $crate::rmongo::find_many_fields($db, $tb, filter, fields, limit).await?;
            Ok(r)
        }

        pub async fn delete_one(
            filter: Document,
            options: impl Into<Option<DeleteOptions>>,
        ) -> Result<DeleteResult, Box<dyn std::error::Error>> {
            let r = $crate::rmongo::delete_one($db, $tb, filter, options).await?;
            Ok(r)
        }

        pub async fn delete_many(
            filter: Document,
            options: impl Into<Option<DeleteOptions>>,
        ) -> Result<DeleteResult, Box<dyn std::error::Error>> {
            let r = $crate::rmongo::delete_many($db, $tb, filter, options).await?;
            Ok(r)
        }

        pub async fn update_one(
            filter: Document,
            update: impl Into<UpdateModifications>,
            options: impl Into<Option<UpdateOptions>>,
        ) -> Result<UpdateResult, Box<dyn std::error::Error>> {
            let r = $crate::rmongo::update_one($db, $tb, filter, update, options).await?;
            Ok(r)
        }

        pub async fn update_many(
            filter: Document,
            update: impl Into<UpdateModifications>,
            options: impl Into<Option<UpdateOptions>>,
        ) -> Result<UpdateResult, Box<dyn std::error::Error>> {
            let r = $crate::rmongo::update_many($db, $tb, filter, update, options).await?;
            Ok(r)
        }

        pub async fn page(filter: Page<$T>) -> Result<Page<$T>, Box<dyn std::error::Error>>
        where
            $T: Debug + Clone + Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
        {
            let r: Page<$T> = $crate::rmongo::page($db, $tb, filter).await?;
            Ok(r)
        }

        pub async fn aggregate<T>(
            pipeline: impl IntoIterator<Item = Document>,
            options: impl Into<Option<AggregateOptions>>,
        ) -> Result<Vec<T>, Box<dyn std::error::Error>>
        where
            T: Debug + Clone + Serialize + DeserializeOwned + Unpin + Send + Sync,
        {
            let r: Vec<T> = $crate::rmongo::aggregate($db, $tb, pipeline, options).await?;
            Ok(r)
        }

        pub async fn raw_aggregate(
            pipeline: impl IntoIterator<Item = Document>,
            options: impl Into<Option<AggregateOptions>>,
        ) -> Result<Vec<Document>, Box<dyn std::error::Error>> {
            let r: Vec<Document> =
                $crate::rmongo::raw_aggregate($db, $tb, pipeline, options).await?;
            Ok(r)
        }

        pub async fn min<T>(
            doc: Document,
            field_name: &str,
        ) -> std::result::Result<T, Box<dyn std::error::Error>>
        where
            T: FromStr,
        {
            let r: T = $crate::rmongo::min($db, $tb, doc, field_name).await?;
            Ok(r)
        }

        pub async fn max<T>(
            doc: Document,
            field_name: &str,
        ) -> std::result::Result<T, Box<dyn std::error::Error>>
        where
            T: FromStr,
        {
            let r: T = $crate::rmongo::max($db, $tb, doc, field_name).await?;
            Ok(r)
        }

        pub async fn avg<T>(
            doc: Document,
            field_name: &str,
        ) -> std::result::Result<T, Box<dyn std::error::Error>>
        where
            T: FromStr,
        {
            let r: T = $crate::rmongo::avg($db, $tb, doc, field_name).await?;
            Ok(r)
        }

        pub async fn sum<T>(
            doc: Document,
            field_name: &str,
        ) -> std::result::Result<T, Box<dyn std::error::Error>>
        where
            T: FromStr,
        {
            let r: T = $crate::rmongo::sum($db, $tb, doc, field_name).await?;
            Ok(r)
        }
    };
}
