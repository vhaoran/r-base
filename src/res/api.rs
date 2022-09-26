use super::*;
use log::*;

#[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
use elasticsearch::cert::CertificateValidation;
use elasticsearch::{
    auth::Credentials,
    cat::CatIndicesParts,
    http::transport::{SingleNodeConnectionPool, Transport, TransportBuilder},
    http::StatusCode,
    Count, CountParts, DeleteParts, Elasticsearch, Error, IndexParts, SearchParts, DEFAULT_ADDRESS,
};

use crate::rmongo::Page;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::fmt::Debug;
use std::vec::Vec;

pub async fn get<T>(db: &str, id_str: &str) -> anyhow::Result<T>
where
    T: Serialize + DeserializeOwned,
{
    let c = cnt();
    let r = c
        .get(elasticsearch::GetParts::IndexId(db, id_str))
        .pretty(true)
        .send()
        .await?;
    let r = r.error_for_status_code()?;
    let r = r.json::<Value>().await?;

    println!("----api.rs---get res-{:?}----", r);
    let str = serde_json::to_string(&r["_source"])?;
    let bean: T = serde_json::from_str(str.as_str())?;

    Ok(bean)
}

pub async fn add<T>(db: &str, id: &str, src: T) -> anyhow::Result<bool>
where
    T: Serialize + DeserializeOwned,
{
    let c = cnt();
    let r = c
        .index(IndexParts::IndexId(db, id))
        .body(src)
        .send()
        .await?;

    Ok(r.status_code() == StatusCode::OK)
}

pub async fn del(db: &str, id: &str) -> anyhow::Result<bool> {
    let c = cnt();
    let r = c
        .delete(DeleteParts::IndexId(db, id))
        .pretty(true)
        .send()
        .await?;
    Ok(r.status_code() == StatusCode::OK)
}

pub async fn count(db: &str) -> anyhow::Result<usize> {
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct ResultCount {
        pub count: Option<usize>,
    }

    let c = cnt();
    let r = c.count(CountParts::Index(&[db])).human(true).send().await?;

    let r: ResultCount = r.json().await?;
    Ok(r.count.unwrap_or(0_usize))
}

pub async fn exist(db: &str, id_str: &str) -> bool {
    let c = cnt();
    let r = c
        .get(elasticsearch::GetParts::IndexId(db, id_str))
        .pretty(true)
        .send()
        .await;
    println!("----api.rs------exist {:?}--", r);

    match r {
        Ok(res) => res.status_code() == StatusCode::OK,
        _ => false,
    }
}

pub async fn page<T>(db: &str, pb: Page<T>) -> anyhow::Result<Page<T>>
where
    T: SetESID + Serialize + DeserializeOwned + Clone + Debug,
{
    let c = cnt();
    //-----------a--------------------------
    let from = (pb.page_no - 1) * pb.rows_per_page;
    let size = pb.rows_per_page;
    //-----------a--------------------------
    let r = c
        .search(SearchParts::Index(&[db]))
        .from(from as i64)
        .size(size as i64)
        .body(pb.filter.clone().unwrap())
        .pretty(true)
        .send()
        .await?;

    debug!("--------r:{:?}-", r);

    let body = r.json::<Value>().await?;
    debug!("-es_body---{:#?}-----", body);

    let _took = body["took"].as_i64().unwrap();
    let mut l: Vec<T> = Vec::new();

    let all = body["hits"]["total"]["value"].as_i64().unwrap_or(0) as u64;
    for hit in body["hits"]["hits"].as_array().unwrap() {
        let each = &hit["_source"];
        let id_obj = &hit["_id"];

        let id = id_obj.as_str().unwrap_or("");
        debug!("id_obj: {:?} id {}", id_obj, id);

        let str = serde_json::to_string(each)?;
        //

        //
        let mut bean: T = serde_json::from_str(str.as_str())?;
        bean.set_id(id.to_string());
        l.push(bean);
    }

    println!("----api.rs---get res-{:?}----", body);

    //-------------------------------------
    let mut ret = pb.clone();
    ret.all_count = all;
    ret.all_page = ret.all_count / ret.rows_per_page;
    if ret.all_count % ret.rows_per_page > 0 {
        ret.all_page += 1;
    };
    ret.data = l;

    Ok(ret)
}
