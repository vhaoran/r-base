use super::*;
use crate::res::SetESID;

use mongodb::bson::{doc, Document};

#[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
use elasticsearch::cert::CertificateValidation;
use elasticsearch::{
    auth::Credentials,
    cat::CatIndicesParts,
    http::transport::{SingleNodeConnectionPool, Transport, TransportBuilder},
    Count, CountParts, Elasticsearch, Error, IndexParts, SearchParts, DEFAULT_ADDRESS,
};

use serde_json::Value;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResultCount {
    pub count: Option<i64>,
}

const DB: &str = "teladhis";

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AdHis {
    id: Option<String>,
    hash: Option<String>,
    url: Option<String>,
    title: Option<String>,
    visited_count: Option<i64>,
    enabled: Option<bool>,
}

impl SetESID for AdHis {
    fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }
}

const TEST_KEY: &str = "key-test";

#[tokio::test]
async fn es_api_get() -> anyhow::Result<()> {
    let cfg: Config = std::default::Default::default();
    init(&cfg).await?;
    println!("----test_api.rs---0-----");

    let _c = cnt();
    println!("----test_api.rs---1-----");
    // //-----------a--------------------------
    let r = get::<AdHis>(DB, TEST_KEY).await?;
    println!("----test_api.rs---2-----");
    println!("----test_api.rs---{:?}-----", r);
    Ok(())
}

#[tokio::test]
async fn es_api_add() -> anyhow::Result<()> {
    let cfg: Config = std::default::Default::default();
    init(&cfg).await?;
    println!("----test_api.rs---0-----");

    let _c = cnt();
    println!("----test_api.rs---1-----");
    // //-----------a--------------------------
    let src = AdHis {
        id: Some(TEST_KEY.to_string()),
        hash: Some("99a".to_string()),
        url: Some("99a url".to_string()),
        title: Some("99a title".to_string()),
        visited_count: Some(512),
        enabled: Some(true),
    };

    let r = add::<AdHis>(DB, TEST_KEY, src).await?;
    println!("----test_api.rs---2-----");
    println!("----test_api.rs---{:?}-----", r);
    Ok(())
}

#[tokio::test]
async fn es_api_del() -> anyhow::Result<()> {
    let cfg: Config = std::default::Default::default();
    init(&cfg).await?;
    println!("----test_api.rs---del-----");

    let _c = cnt();
    println!("----test_api.rs---1-----");
    let r = del(DB, TEST_KEY).await?;
    println!("----test_api.rs---{:?}-----", r);
    Ok(())
}

#[tokio::test]
async fn es_api_exist() -> anyhow::Result<()> {
    let cfg: Config = std::default::Default::default();
    init(&cfg).await?;
    println!("----test_api.rs---del-----");

    let _c = cnt();
    println!("----test_api.rs---1-----");
    let r = exist(DB, TEST_KEY).await;
    println!("----test_api.rs---{:?}-----", r);
    Ok(())
}

#[tokio::test]
async fn es_api_page() -> anyhow::Result<()> {
    use crate::rmongo::Page;
    use mongodb::bson::doc;

    let cfg: Config = std::default::Default::default();

    init(&cfg).await?;
    println!("----test_api.rs---del-----");

    let mut pb = Page::new();
    pb.rows_per_page = 2;
    pb.filter = Some(doc! {
        "query": {
            "match": {
                    "title":"whr"
                }
        }

    });

    let _c = cnt();
    println!("----test_api.rs---1-----");
    let r: Page<AdHis> = page(DB, pb).await?;
    println!("----test_api.rs---{:?}-----", r);
    Ok(())
}

#[tokio::test]
async fn es_count_1() -> anyhow::Result<()> {
    // crate::init_modules(None).await;
    let cfg: Config = std::default::Default::default();
    println!("-----------{:#?}-----------", cfg);

    init(&cfg).await?;

    println!("-----------after init -----------");

    let c = cnt();
    println!("-----------after get_cnt-----------");
    let r = c
        .count(CountParts::Index(&["teladhis"]))
        .human(true)
        .send()
        .await?;
    println!("-------------------{:#?}---", r);

    // let a = r.text().await;
    // println!("-------------------{:#?}---", a);
    let r: ResultCount = r.json().await?;
    println!("-------------------{:#?}---", r);

    Ok(())
}

#[tokio::test]
async fn es_count_2() -> anyhow::Result<()> {
    //---------------------
    // crate::init_modules(None).await;
    let cfg: Config = std::default::Default::default();
    println!("-----------{:#?}-----------", cfg);

    init(&cfg).await?;

    let r = count("teladhis").await;
    println!("-----------{:#?}-----------", r);
    Ok(())
}
