use std::convert::TryFrom;
use std::sync::Arc;

use mongodb::bson::Document;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MyStruct {
    pub name: Option<Document>,
}

impl Default for MyStruct {
    fn default() -> Self {
        MyStruct { name: None }
    }
}

// impl Copy for MyStruct {}
