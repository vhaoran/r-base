use super::*;
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

#[tokio::test]
async fn es_cnt_1() -> anyhow::Result<()> {
    // let cfg : Config = std::default::Default();
    let url = "http://192.168.0.99:9200";
    let c = get_client(url)?;

    //-----------a--------------------------
    for i in 0..100u8 {
        let id = format!("{}", i * 10);
        let client = c.clone();
        let _response = client
            .index(IndexParts::IndexId("teladhis", id.as_str()))
            .body(json!({
                "id": id.as_str(),
                "hash": "",
                "url": "https://url",
                "title": "good good good",
                "visited_count": 999,
                "enabled":false
            }))
            .send()
            .await?;
        println!("----{} add ok----", id);
    }

    Ok(())
}
#[tokio::test]
async fn es_cnt_11() -> anyhow::Result<()> {
    let cfg: Config = std::default::Default::default();
    init(&cfg).await?;
    let client = cnt();

    //-----------a--------------------------
    for i in 0..100u32 {
        let id = format!("{}", i * 10);
        let _response = client
            .index(IndexParts::IndexId("teladhis", id.as_str()))
            .body(json!({
                "id": id.as_str(),
                "hash": "",
                "url": "https://url",
                "title": "good good good",
                "visited_count": 999,
                "enabled":false
            }))
            .send()
            .await?;
        println!("----{} add ok----", id);
    }

    Ok(())
}

#[tokio::test]
async fn es_cnt_2() -> anyhow::Result<()> {
    // let cfg : Config = std::default::Default();
    let url = "http://192.168.0.99:9200";
    let c = get_client(url)?;
    let client = c.clone();

    //-----------a--------------------------
    let response = client
        .search(SearchParts::Index(&["teladhis"]))
        .from(0)
        .size(100)
        .body(json!({
            "query": {
                "match": {
                        "title":"good"
                    }
            }
        }))
        .send()
        .await?;

    let response_body = response.json::<Value>().await?;
    let _took = response_body["took"].as_i64().unwrap();
    for hit in response_body["hits"]["hits"].as_array().unwrap() {
        // print the source document
        println!("{:?}", hit);
    }

    Ok(())
}

#[tokio::test]
async fn es_cnt_3() -> anyhow::Result<()> {
    // let cfg : Config = std::default::Default();
    // let url = "http://192.168.0.99:9200";
    let cfg: Config = std::default::Default::default();
    init(&cfg).await?;
    let client = cnt();

    //-----------a--------------------------
    let response = client
        .search(SearchParts::Index(&["teladhis"]))
        .from(0)
        .size(100)
        .body(json!({
            "query": {
                "match": {
                        "title":"good"
                    }
            }
        }))
        .send()
        .await?;

    let response_body = response.json::<Value>().await?;
    let _took = response_body["took"].as_i64().unwrap();
    for hit in response_body["hits"]["hits"].as_array().unwrap() {
        // print the source document
        println!("{:?}", hit);
    }

    Ok(())
}

#[tokio::test]
async fn es_cnt_4() -> anyhow::Result<()> {
    let cfg: Config = std::default::Default::default();
    init(&cfg).await?;

    //-----------a--------------------------
    // Spawn tasks
    let futures = (0..100)
        .map(|id| {
            tokio::spawn(async move {
                let client = cnt();
                //-------------------------------------
                loop {
                    for i in 0..1000u32 {
                        let id2 = format!("{}/{}", id, i * 10);
                        let _response = client
                            .index(IndexParts::IndexId("teladhis", id2.as_str()))
                            .body(json!({
                                "id": id2.as_str(),
                                "hash": "",
                                "url": "https://url",
                                "title": "good good good",
                                "visited_count": 999,
                                "enabled":false
                            }))
                            .send()
                            .await;
                        println!("---{} add ok----", id2);
                    }
                }
                //-----------a--------------------------
                // let _ = tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                // println!("-###########  -{}  after-###########----", id);
            })
        })
        .collect::<Vec<_>>();

    for future in futures {
        future.await.unwrap();
    }

    //-----------a--------------------------
    let _ = tokio::spawn(async move {
        for i in 0..100u32 {
            println!("----after sleep---{}--", i);
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
            println!("----after sleep-----");
        }
    })
    .await;

    Ok(())
}
