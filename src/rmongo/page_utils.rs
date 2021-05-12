use std::default::Default;
use std::fmt::Debug;

use mongodb::bson::{doc, Document};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

// #[derive(Clone, Debug)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Page<T: Debug + Clone> {
    #[serde(default)]
    pub page_no: u64,
    #[serde(default)]
    pub rows_per_page: u64,
    #[serde(default)]
    pub all_count: u64,
    #[serde(default)]
    pub all_page: u64,
    #[serde(default)]
    pub sort: Option<Document>,
    #[serde(default)]
    pub data: Vec<T>,
    #[serde(default)]
    pub filter: Option<Document>,
    #[serde(default)]
    pub fields: Option<Document>,
}

// impl Copy<T> for Page<T> where T: Clone + Debug + Copy {}

impl<T> Page<T>
    where
        T: Debug + Clone,
{
    pub fn builder() -> PageBuilder<T> {
        PageBuilder::<T>::new()
    }

    pub fn new() -> Self {
        Page {
            page_no: 1,
            rows_per_page: 10,
            all_count: 0,
            all_page: 0,
            sort: None,
            filter: None,
            data: Vec::new(),
            fields: None,
        }
    }

    pub fn adjust(&mut self) {
        if self.page_no < 1 {
            self.page_no = 1;
        }
        if self.rows_per_page < 1 {
            self.rows_per_page = 10;
        }
    }
}

impl<T> Default for Page<T>
    where
        T: Debug + Clone,
{
    fn default() -> Self {
        Page::new()
    }
}

//-------------------------------------
pub struct PageBuilder<T>
    where
        T: Debug + Clone,
{
    inner: Page<T>,
}

impl<T> PageBuilder<T>
    where
        T: Debug + Clone,
{
    pub fn new() -> Self {
        PageBuilder { inner: Page::new() }
    }
    // pub page_no: u64,
    pub fn page_no(&mut self, page_no: u64) -> &mut Self {
        self.inner.page_no = page_no;
        self
    }
    // pub rows_per_page: u64,
    pub fn rows_per_page(&mut self, rows_per_page: u64) -> &mut Self {
        self.inner.rows_per_page = rows_per_page;
        self
    }

    // pub all_count: u64,
    // pub all_page: u64,
    // pub sort: Option<Document>,
    pub fn sort(&mut self, d: Document) -> &mut Self {
        self.inner.sort = Some(d);
        self
    }

    // pub data: Vec<T>,
    // pub filter: Option<Document>,
    pub fn filter(&mut self, filter: Document) -> &mut Self {
        self.inner.filter = Some(filter);
        self
    }

    // pub fields: Option<Document>,
    pub fn fields(&mut self, fields: Document) -> &mut Self {
        self.inner.fields = Some(fields);
        self
    }

    pub fn build(&self) -> Page<T> {
        self.inner.clone()
    }
}
