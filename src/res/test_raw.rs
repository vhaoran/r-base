extern crate elasticsearch;
extern crate serde_json;
extern crate url;

#[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
use elasticsearch::cert::CertificateValidation;
use elasticsearch::{
    auth::Credentials,
    cat::CatIndicesParts,
    http::transport::{SingleNodeConnectionPool, Transport, TransportBuilder},
    Elasticsearch, Error, IndexParts, SearchParts, DEFAULT_ADDRESS,
};

use serde_json::Value;
use std::env;
// use sysinfo::SystemExt;
use self::elasticsearch::GetParts;
use url::Url;
// use stack_overflow::*;
// use textwrap::fill;

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

#[tokio::test]
async fn es_1() -> anyhow::Result<()> {
    println!("---- 1 -----");
    let transport = Transport::single_node("http://192.168.0.99:9200")?;
    println!("---- 2 -----");
    let c = Elasticsearch::new(transport);

    let client = c.clone();

    println!("---- 3 -----");
    //-----------a--------------------------
    let response = client
        .search(SearchParts::Index(&["teladhis"]))
        .from(0)
        .size(100)
        .pretty(true)
        .body(json!({
            "query": {
                "match": {
                        "title":"whr"
                    }
            }
        }))
        .size(5)
        .send()
        .await?;

    let response_body = response.json::<Value>().await?;
    let _took = response_body["took"].as_i64().unwrap();
    for hit in response_body["hits"]["hits"].as_array().unwrap() {
        // print the source document
        println!("{:?}", hit["_source"]);
    }

    Ok(())
}

#[tokio::test]
async fn es_add() -> anyhow::Result<()> {
    println!("---- 1 -----");
    let transport = Transport::single_node("http://192.168.0.99:9200")?;
    println!("---- 2 -----");
    let c = Elasticsearch::new(transport);
    let client = c.clone();

    //-----------a--------------------------
    for i in 0..100u32 {
        let id = format!("{}", i);
        let r = client
            .index(IndexParts::IndexId("teladhis", id.as_str()))
            .body(json!({
                "id": id.as_str(),
                "hash": "",
                "url": "https://url",
                "title": "whr",
                "visited_count": 999,
                "enabled":false
            }))
            .send()
            .await?;
        println!("----ok {:?}-----", r);
    }
    Ok(())
}

#[tokio::test]
async fn es_get() -> anyhow::Result<()> {
    println!("---- 1 -----");
    let transport = Transport::single_node("http://192.168.0.99:9200")?;
    println!("---- 2 -----");
    let c = Elasticsearch::new(transport);
    //-----------a--------------------------
    let r = c
        .get(elasticsearch::GetParts::IndexId("teladhis", "3"))
        .pretty(true)
        .send()
        .await?
        .json::<Value>()
        .await?;

    // let response_body = response.json::<Value>().await?;

    println!("----test_es_1.rs---a---{:?}--", r);
    println!("----test_es_1.rs---found---{:?}--", r["found"]);
    println!("----test_es_1.rs---id---{:?}--", r["_id"]);
    println!("----test_es_1.rs---_source---{:?}--", r["_source"]);

    let str = serde_json::to_string(&r["_source"])?;
    println!("----test_sqlx---str--{}---", str);
    //
    let a: AdHis = serde_json::from_str(str.as_str())?;
    println!("----test_sqlx---unmarshal {:?}-----", a);

    Ok(())
}
