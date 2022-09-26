use log::*;
use mongodb::bson::{doc, Document};
use mongodb::options::{
    CountOptions, FindOneOptions, FindOptions, InsertManyOptions, InsertOneOptions,
};
use mongodb::results::{InsertManyResult, InsertOneResult};
use std::borrow::Borrow;
use std::str::FromStr;

use anyhow::anyhow;
use futures::stream::{StreamExt, TryStreamExt};
use mongodb::options::{AggregateOptions, DeleteOptions, UpdateModifications, UpdateOptions};
use mongodb::results::{DeleteResult, UpdateResult};
use mongodb::{options::ClientOptions, Client, ClientSession};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::any::Any;
use std::default::Default;
use std::fmt::Debug;

use super::*;

pub async fn find_one<T>(
    db: &str,
    tb: &str,
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<FindOneOptions>>,
) -> anyhow::Result<T>
where
    T: Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
{
    let ret: Option<T> = raw_find_one(db, tb, filter, options).await?;
    if let Some(data) = ret {
        return Ok(data);
    }

    Err(anyhow!("no data"))
}
pub async fn tx_find_one<T>(
    db: &str,
    tb: &str,
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<FindOneOptions>>,
    tx: &mut ClientSession,
) -> anyhow::Result<T>
where
    T: Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
{
    let ret: Option<T> = tx_raw_find_one(db, tb, filter, options, tx).await?;
    if let Some(data) = ret {
        return Ok(data);
    }

    Err(anyhow!("no data"))
}

pub async fn exist(
    db: &str,
    tb: &str,
    filter: impl Into<Option<Document>>,
) -> anyhow::Result<bool> {
    let r = raw_exist(db, tb, filter).await?;
    Ok(r)
}
pub async fn tx_exist(
    db: &str,
    tb: &str,
    filter: impl Into<Option<Document>>,
    tx: &mut ClientSession,
) -> anyhow::Result<bool> {
    let r = tx_raw_exist(db, tb, filter, tx).await?;
    Ok(r)
}

pub async fn count(
    db: &str,
    tb: &str,
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<CountOptions>>,
) -> anyhow::Result<u64> {
    let c = raw_count(db, tb, filter, options).await?;
    Ok(c)
}
pub async fn tx_count(
    db: &str,
    tb: &str,
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<CountOptions>>,
    tx: &mut ClientSession,
) -> anyhow::Result<u64> {
    let c = tx_raw_count(db, tb, filter, options, tx).await?;
    Ok(c)
}

pub async fn insert_one<T>(
    db: &str,
    tb: &str,
    doc: T,
    options: impl Into<Option<InsertOneOptions>>,
) -> anyhow::Result<InsertOneResult>
where
    T: Serialize + DeserializeOwned + Unpin + Debug,
{
    // let d: Document = doc.into();
    let r = raw_insert_one(db, tb, doc, options).await?;
    Ok(r)
}

pub async fn tx_insert_one<T>(
    db: &str,
    tb: &str,
    doc: T,
    options: impl Into<Option<InsertOneOptions>>,
    tx: &mut ClientSession,
) -> anyhow::Result<InsertOneResult>
where
    T: Serialize + DeserializeOwned + Unpin + Debug,
{
    // let d: Document = doc.into();
    let r = tx_raw_insert_one(db, tb, doc, options, tx).await?;
    Ok(r)
}

pub async fn insert_many<T>(
    db: &str,
    tb: &str,
    doc: Vec<T>,
    options: impl Into<Option<InsertManyOptions>>,
) -> anyhow::Result<InsertManyResult>
where
    T: Serialize + DeserializeOwned + Unpin + Debug,
{
    // let d: Document = doc.into();
    let r = raw_insert_many(db, tb, doc, options).await?;
    Ok(r)
}

pub async fn tx_insert_many<T>(
    db: &str,
    tb: &str,
    doc: Vec<T>,
    options: impl Into<Option<InsertManyOptions>>,
    tx: &mut ClientSession,
) -> anyhow::Result<InsertManyResult>
where
    T: Serialize + DeserializeOwned + Unpin + Debug,
{
    // let d: Document = doc.into();
    let r = tx_raw_insert_many(db, tb, doc, options, tx).await?;
    Ok(r)
}

pub async fn find_many_fields<T>(
    db: &str,
    tb: &str,
    filter: impl Into<Option<Document>>,
    cols: Option<Document>,
    limit: Option<i64>,
) -> anyhow::Result<Vec<T>>
where
    T: Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
{
    let mut opt: FindOptions = Default::default();
    opt.projection = cols;
    opt.limit = limit;

    let r = self::find_many(db, tb, filter, Some(opt)).await?;
    Ok(r)
}

pub async fn tx_find_many_fields<T>(
    db: &str,
    tb: &str,
    filter: impl Into<Option<Document>>,
    cols: Option<Document>,
    limit: Option<i64>,
    tx: &mut ClientSession,
) -> anyhow::Result<Vec<T>>
where
    T: Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
{
    let mut opt: FindOptions = Default::default();
    opt.projection = cols;
    opt.limit = limit;

    let r = self::tx_find_many(db, tb, filter, Some(opt), tx).await?;
    Ok(r)
}

pub async fn find_many<T>(
    db: &str,
    tb: &str,
    filter: impl Into<Option<Document>>,
    options: Option<FindOptions>,
) -> anyhow::Result<Vec<T>>
where
    T: Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
{
    let mut opt: FindOptions = Default::default();
    // limit result
    const LIMIT: i64 = 10000 * 5;
    if options.is_some() {
        opt = options.unwrap().clone();
        let n = opt.limit.unwrap_or(0);
        if n < 1 || n > LIMIT {
            opt.limit = Some(LIMIT);
        }
    } else {
        opt.limit = Some(LIMIT);
    }

    // debug!("-----before --raw_find_many--");
    let mut c: mongodb::Cursor<T> = raw_find_many(db, tb, filter, opt).await?;
    // debug!("-----after --raw_find_many--");

    // println!("-----------{:?}-----------",c);

    let mut l: Vec<T> = Vec::new();
    while let Some(row) = c.next().await {
        // println!("---row--------{:?}-----------", row);

        if let Ok(row) = row {
            // println!(" {:?}", row);
            l.push(row);
        }
    }

    // debug!("-----after cursor.next()--");
    Ok(l)
}

pub async fn tx_find_many<T>(
    db: &str,
    tb: &str,
    filter: impl Into<Option<Document>>,
    options: Option<FindOptions>,
    tx: &mut ClientSession,
) -> anyhow::Result<Vec<T>>
where
    T: Serialize + DeserializeOwned + Unpin + Debug + Send + Sync,
{
    let mut opt: FindOptions = Default::default();
    // limit result
    const LIMIT: i64 = 10000 * 5;
    if options.is_some() {
        opt = options.unwrap().clone();
        let n = opt.limit.unwrap_or(0);
        if n < 1 || n > LIMIT {
            opt.limit = Some(LIMIT);
        }
    } else {
        opt.limit = Some(LIMIT);
    }

    // debug!("-----before --raw_find_many--");
    let mut c: mongodb::SessionCursor<T> = tx_raw_find_many(db, tb, filter, opt, tx).await?;
    // debug!("-----after --raw_find_many--");

    // println!("-----------{:?}-----------",c);

    let mut l: Vec<T> = Vec::new();
    while let Some(row) = c.next(tx).await {
        // println!("---row--------{:?}-----------", row);

        if let Ok(row) = row {
            // println!(" {:?}", row);
            l.push(row);
        }
    }

    // debug!("-----after cursor.next()--");
    Ok(l)
}

pub async fn delete_one(
    db: &str,
    tb: &str,
    doc: Document,
    options: impl Into<Option<DeleteOptions>>,
) -> anyhow::Result<DeleteResult> {
    let r = raw_delete_one(db, tb, doc, options).await?;
    Ok(r)
}

pub async fn tx_delete_one(
    db: &str,
    tb: &str,
    doc: Document,
    options: impl Into<Option<DeleteOptions>>,
    tx: &mut ClientSession,
) -> anyhow::Result<DeleteResult> {
    let r = tx_raw_delete_one(db, tb, doc, options, tx).await?;
    Ok(r)
}

pub async fn delete_many(
    db: &str,
    tb: &str,
    doc: Document,
    options: impl Into<Option<DeleteOptions>>,
) -> anyhow::Result<DeleteResult> {
    let r = raw_delete_many(db, tb, doc, options).await?;
    Ok(r)
}

pub async fn tx_delete_many(
    db: &str,
    tb: &str,
    doc: Document,
    options: impl Into<Option<DeleteOptions>>,
    tx: &mut ClientSession,
) -> anyhow::Result<DeleteResult> {
    let r = tx_raw_delete_many(db, tb, doc, options, tx).await?;
    Ok(r)
}

pub async fn update_one(
    db: &str,
    tb: &str,
    doc: Document,
    update: impl Into<UpdateModifications>,
    options: impl Into<Option<UpdateOptions>>,
) -> anyhow::Result<UpdateResult> {
    let r = raw_update_one(db, tb, doc, update, options).await?;
    Ok(r)
}
pub async fn tx_update_one(
    db: &str,
    tb: &str,
    doc: Document,
    update: impl Into<UpdateModifications>,
    options: impl Into<Option<UpdateOptions>>,
    tx: &mut ClientSession,
) -> anyhow::Result<UpdateResult> {
    let r = tx_raw_update_one(db, tb, doc, update, options, tx).await?;
    Ok(r)
}

pub async fn update_many(
    db: &str,
    tb: &str,
    doc: Document,
    update: impl Into<UpdateModifications>,
    options: impl Into<Option<UpdateOptions>>,
) -> anyhow::Result<UpdateResult> {
    let r = raw_update_many(db, tb, doc, update, options).await?;
    Ok(r)
}

pub async fn tx_update_many(
    db: &str,
    tb: &str,
    doc: Document,
    update: impl Into<UpdateModifications>,
    options: impl Into<Option<UpdateOptions>>,
    tx: &mut ClientSession,
) -> anyhow::Result<UpdateResult> {
    let r = tx_raw_update_many(db, tb, doc, update, options, tx).await?;
    Ok(r)
}

pub async fn page<T>(db: &str, tb: &str, filter: Page<T>) -> anyhow::Result<Page<T>>
where
    T: Debug + Clone + Serialize + DeserializeOwned + Unpin + Send + Sync,
{
    use mongodb::options::FindOptions;
    // sort
    let mut opt: FindOptions = Default::default();
    opt.limit = Some(filter.rows_per_page as i64);
    opt.skip = Some(((filter.page_no - 1) * filter.rows_per_page) as u64);
    opt.sort = filter.sort.clone();
    opt.projection = filter.fields.clone();

    //where
    let mut pg = filter.clone();
    pg.adjust();

    //------------------all count---------------------
    let all_count = self::count(db, tb, pg.filter.clone(), None).await?;
    //
    let mut all_page = all_count / pg.rows_per_page;
    if all_count % pg.rows_per_page > 0 {
        all_page += 1;
    }
    pg.all_count = all_count;
    pg.all_page = all_page;

    let l: Vec<T> = self::find_many(db, tb, pg.filter.clone(), Some(opt)).await?;
    //
    pg.data = l;
    Ok(pg)
}

pub async fn tx_page<T>(
    db: &str,
    tb: &str,
    filter: Page<T>,
    tx: &mut ClientSession,
) -> anyhow::Result<Page<T>>
where
    T: Debug + Clone + Serialize + DeserializeOwned + Unpin + Send + Sync,
{
    use mongodb::options::FindOptions;
    // sort
    let mut opt: FindOptions = Default::default();
    opt.limit = Some(filter.rows_per_page as i64);
    opt.skip = Some(((filter.page_no - 1) * filter.rows_per_page) as u64);
    opt.sort = filter.sort.clone();
    opt.projection = filter.fields.clone();

    //where
    let mut pg = filter.clone();
    pg.adjust();

    //------------------all count---------------------
    let all_count = self::tx_count(db, tb, pg.filter.clone(), None, tx).await?;
    //
    let mut all_page = all_count / pg.rows_per_page;
    if all_count % pg.rows_per_page > 0 {
        all_page += 1;
    }
    pg.all_count = all_count;
    pg.all_page = all_page;

    let l: Vec<T> = self::tx_find_many(db, tb, pg.filter.clone(), Some(opt), tx).await?;
    //
    pg.data = l;
    Ok(pg)
}

/*
sometimes int maybe parsed to float,
so best choose T as Document or choose raw_aggregate
*/
pub async fn aggregate<T>(
    db: &str,
    tb: &str,
    pipeline: impl IntoIterator<Item = Document>,
    options: impl Into<Option<AggregateOptions>>,
) -> anyhow::Result<Vec<T>>
where
    T: Debug + Clone + Serialize + DeserializeOwned + Unpin,
{
    let src = raw_aggregate(db, tb, pipeline, options).await?;
    // println!("-----------src {:?}-----------", src);

    //
    let s = serde_json::to_string(&src)?;
    let l: Vec<T> = serde_json::from_str(s.as_str())?;

    Ok(l)
}

pub async fn tx_aggregate<T>(
    db: &str,
    tb: &str,
    pipeline: impl IntoIterator<Item = Document>,
    options: impl Into<Option<AggregateOptions>>,
    tx: &mut ClientSession,
) -> anyhow::Result<Vec<T>>
where
    T: Debug + Clone + Serialize + DeserializeOwned + Unpin,
{
    let src = tx_raw_aggregate(db, tb, pipeline, options, tx).await?;
    // println!("-----------src {:?}-----------", src);

    //
    let s = serde_json::to_string(&src)?;
    let l: Vec<T> = serde_json::from_str(s.as_str())?;

    Ok(l)
}

pub async fn min<T>(db: &str, tb: &str, doc: Document, field_name: &str) -> anyhow::Result<T>
where
    T: FromStr,
{
    let r: T = self::agg_mult(db, tb, doc, "$min", field_name).await?;
    Ok(r)
}

pub async fn tx_min<T>(
    db: &str,
    tb: &str,
    doc: Document,
    field_name: &str,
    tx: &mut ClientSession,
) -> anyhow::Result<T>
where
    T: FromStr,
{
    let r: T = self::tx_agg_mult(db, tb, doc, "$min", field_name, tx).await?;
    Ok(r)
}

pub async fn max<T>(db: &str, tb: &str, doc: Document, field_name: &str) -> anyhow::Result<T>
where
    T: FromStr,
{
    let r: T = self::agg_mult(db, tb, doc, "$max", field_name).await?;
    Ok(r)
}

pub async fn tx_max<T>(
    db: &str,
    tb: &str,
    doc: Document,
    field_name: &str,
    tx: &mut ClientSession,
) -> anyhow::Result<T>
where
    T: FromStr,
{
    let r: T = self::tx_agg_mult(db, tb, doc, "$max", field_name, tx).await?;
    Ok(r)
}

pub async fn avg<T>(db: &str, tb: &str, doc: Document, field_name: &str) -> anyhow::Result<T>
where
    T: FromStr,
{
    let r: T = self::agg_mult(db, tb, doc, "$avg", field_name).await?;
    Ok(r)
}

pub async fn tx_avg<T>(
    db: &str,
    tb: &str,
    doc: Document,
    field_name: &str,
    tx: &mut ClientSession,
) -> anyhow::Result<T>
where
    T: FromStr,
{
    let r: T = self::tx_agg_mult(db, tb, doc, "$avg", field_name, tx).await?;
    Ok(r)
}

pub async fn sum<T>(db: &str, tb: &str, doc: Document, field_name: &str) -> anyhow::Result<T>
where
    T: FromStr,
{
    let r: T = self::agg_mult(db, tb, doc, "$sum", field_name).await?;
    Ok(r)
}

pub async fn tx_sum<T>(
    db: &str,
    tb: &str,
    doc: Document,
    field_name: &str,
    tx: &mut ClientSession,
) -> anyhow::Result<T>
where
    T: FromStr,
{
    let r: T = self::tx_agg_mult(db, tb, doc, "$sum", field_name, tx).await?;
    Ok(r)
}

async fn agg_mult<T>(
    db: &str,
    tb: &str,
    doc: Document,
    fn_name: &str,
    field_name: &str,
) -> anyhow::Result<T>
where
    T: FromStr,
{
    use gjson::Kind;
    let fd = format!("${}", field_name);
    let src = raw_aggregate(
        db,
        tb,
        vec![
            doc! {
                    "$match":doc
            },
            doc! {
                "$group":{
                "_id":null,
                "col":{fn_name:fd}
            }},
        ],
        None,
    )
    .await?;

    //
    // println!("-----agg_mult------src {:?}-----------", src);

    match src.get(0) {
        Some(v) => {
            let s = serde_json::to_string(&v)?;
            let r = gjson::get(s.as_str(), "col");
            let s = format!("{}", r);
            let i = s.parse::<T>().map_err(|e| anyhow!(""))?;
            Ok(i)
        }
        _ => Err(anyhow!("无法解析到数据")),
    }
}

async fn tx_agg_mult<T>(
    db: &str,
    tb: &str,
    doc: Document,
    fn_name: &str,
    field_name: &str,
    tx: &mut ClientSession,
) -> anyhow::Result<T>
where
    T: FromStr,
{
    use gjson::Kind;
    let fd = format!("${}", field_name);
    let src = tx_raw_aggregate(
        db,
        tb,
        vec![
            doc! {
                    "$match":doc
            },
            doc! {
                "$group":{
                "_id":null,
                "col":{fn_name:fd}
            }},
        ],
        None,
        tx,
    )
    .await?;

    //
    // println!("-----agg_mult------src {:?}-----------", src);

    match src.get(0) {
        Some(v) => {
            let s = serde_json::to_string(&v)?;
            let r = gjson::get(s.as_str(), "col");
            let s = format!("{}", r);
            let i = s.parse::<T>().map_err(|e| anyhow!("error"))?;
            Ok(i)
        }
        _ => Err(anyhow!("无法解析到数据")),
    }
}
