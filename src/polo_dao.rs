#[macro_export]
macro_rules! polo_base {
    ($tb:expr,$T:ident) => {
        use polodb_core::bson::oid::ObjectId;
        use polodb_core::bson::{doc, Document};
        use polodb_core::results::{DeleteResult, InsertOneResult, UpdateResult};
        use polodb_core::{bson, ClientCursor};
        use serde::{de::DeserializeOwned, Deserialize, Serialize};
        use std::borrow::Borrow;
        // use $crate::rpolo::Page;
        use std::any::Any;
        use std::default::Default;
        use std::fmt::Debug;
        use std::str::FromStr;

        pub fn tb_name() -> String {
            let s = format!("{}", $tb);
            s
        }

        pub async fn find_one<$T>(filter: impl Into<Option<Document>>) -> anyhow::Result<$T>
        where
            $T: Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
        {
            let r = $crate::rpolo::find_one($tb, filter).await?;
            Ok(r)
        }

        pub async fn exist(filter: impl Into<Option<Document>>) -> bool {
            $crate::rpolo::exist($tb, filter).await
        }

        pub async fn count(filter: Document) -> anyhow::Result<i64> {
            let r = $crate::rpolo::count($tb, filter).await?;
            Ok(r)
        }

        pub async fn insert_one<$T>(doc: $T) -> anyhow::Result<InsertOneResult>
        where
            $T: Serialize + DeserializeOwned + Unpin + Debug,
        {
            let r = $crate::rpolo::insert_one($tb, doc).await?;
            Ok(r)
        }

        // pub  fn insert_many<$T>(
        //     doc: Vec<$T>,
        // ) -> anyhow::Result<InsertManyResult>
        // where
        //     $T: Serialize + DeserializeOwned + Unpin + Debug,
        // {
        //     let r = $crate::rpolo::insert_many($db, $tb, doc, options).await?;
        //     Ok(r)
        // }

        pub async fn find_many<$T>(filter: impl Into<Option<Document>>) -> anyhow::Result<Vec<$T>>
        where
            $T: Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
        {
            let r = $crate::rpolo::find_many::<$T>($tb, filter).await?;
            Ok(r)
        }

        pub async fn delete_one(filter: Document) -> anyhow::Result<DeleteResult> {
            let r = $crate::rpolo::delete_one($tb, filter).await?;
            Ok(r)
        }

        pub async fn delete_many(filter: Document) -> anyhow::Result<DeleteResult> {
            let r = $crate::rpolo::delete_many($tb, filter).await?;
            Ok(r)
        }

        pub async fn update_one(
            filter: Document,
            update: Document,
        ) -> anyhow::Result<UpdateResult> {
            let r = $crate::rpolo::update_one($tb, filter, update).await?;
            Ok(r)
        }

        pub async fn update_many(
            filter: Document,
            update: Document,
        ) -> anyhow::Result<UpdateResult> {
            let r = $crate::rpolo::update_many($tb, filter, update).await?;
            Ok(r)
        }
    };
}
