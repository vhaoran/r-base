use anyhow::anyhow;
use futures::stream::StreamExt;
use lapin::{
    message::Delivery, message::DeliveryResult, options::*, publisher_confirm::Confirmation,
    types::FieldTable, BasicProperties, Connection, ConnectionProperties, ConnectionStatus,
};
use serde::{Deserialize, Serialize};
use tracing::*;

use super::*;
use super::*;

const QUEUE_TEST: &str = "q_test";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    id: i32,
    name: String,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: 1,
            name: "whr".to_string(),
        }
    }
}

#[tokio::test]
async fn mq_publish_1() -> anyhow::Result<()> {
    // log::set_max_level(LevelFilter::Debug);

    let cfg: Config = std::default::Default::default();
    init(&cfg).await?;
    let _conn = conn();
    //-----------a--------------------------
    // Spawn tasks
    for i in 0..2u32 {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        tokio::spawn(async move {
            for id in 0..10000u32 {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                let s = format!(" {}/{}  hello!", i, id);
                match publish_basic_json(QUEUE_TEST, s.as_str()).await {
                    Ok(_) => println!("{id} ok {}/{}", i, id),
                    Err(e) => println!("{id} ############ error ### {}", e.to_string()),
                }
            }
        });
    }

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        println!("----after sleep-----");
    }

    // Ok(())
}

#[tokio::test]
async fn mq_consume_queue_1() -> anyhow::Result<()> {
    // log::set_max_level(LevelFilter::Debug);

    let cfg: Config = std::default::Default::default();
    init(&cfg).await?;
    let _conn = conn();
    //
    publish_basic(QUEUE_TEST, "hello world -------").await?;

    println!("---------publish ok-----------");

    //-----------a--------------------------
    let r = consumer_of_queue(QUEUE_TEST).await?.set_delegate(
        move |delivery: lapin::message::DeliveryResult| async move {
            if delivery.is_err() {
                return;
            }
            let delivery = delivery.unwrap();
            if let Some(delivery) = delivery {
                // delivery
                //     .ack(BasicAckOptions::default())
                //     .await
                //     .expect("failed to ack");
                match std::str::from_utf8(delivery.data.as_slice()) {
                    Ok(body) => {
                        println!(" recevie : {}", body);
                    }
                    Err(e) => {
                        error!("消息体解析失败： {}", e.to_string())
                    }
                }
            }
        },
    );

    println!("consume after result: {:?}", r);

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        println!("----after sleep-----");
    }

    // Ok(())
}

#[tokio::test]
async fn mq_consume_queue_2() -> anyhow::Result<()> {
    // log::set_max_level(LevelFilter::Debug);
    let cfg: Config = std::default::Default::default();
    let _ = init(&cfg).await?;
    loop {
        let is_err = loop_single().await.is_err();
        if is_err {
            println!("-----------error -----------");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}

pub async fn loop_single() -> anyhow::Result<()> {
    loop {
        println!("--start doing-------");
        let mut c = consumer_of_queue(QUEUE_TEST).await?;
        while let Some(delivery) = c.next().await {
            if delivery.is_err() {
                return Err(anyhow!(""));
            }

            let delivery = delivery.unwrap();
            // delivery.ack(BasicAckOptions::default()).await.expect("ack");
            match std::str::from_utf8(delivery.data.as_slice()) {
                Ok(body) => {
                    println!("recevie : {}", body);
                }
                Err(e) => {
                    error!("消息体解析失败： {}", e.to_string())
                }
            }
        }

        println!("consume error and then reconnect ");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
    // Ok(())
}

#[tokio::test]
async fn mq_api_publish_1() -> anyhow::Result<()> {
    // log::set_max_level(LevelFilter::Debug);

    let cfg: Config = std::default::Default::default();
    init(&cfg).await?;
    let _conn = conn();
    //-----------a--------------------------
    // Spawn tasks
    for i in 0..500u32 {
        tokio::spawn(async move {
            for id in 0..100i32 {
                let s = format!(" {}_{}", i, id);
                let u = User {
                    id: id,
                    name: s.clone(),
                };
                match publish_basic_json(QUEUE_TEST, u).await {
                    Ok(_) => println!("publish result: ok {}/{}", i, id),
                    Err(e) => println!("publish result: ok {}", e.to_string()),
                }
            }
        });
    }

    let _ = tokio::spawn(async move {
        println!("----after sleep-----");
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            println!("----after sleep-----");
            break;
        }
    })
    .await;

    Ok(())
}

#[test]
fn str_marshal_1() {
    let s = "abc";
    let r = serde_json::to_string(s).unwrap();
    let rs: &str = serde_json::from_str(r.as_str()).unwrap();

    println!("publish result: {} {}", s, s.len());
    println!("publish result: {} {}", r, r.len());
    println!("publish result: {} {}", rs, rs.len());
}

#[tokio::test]
async fn mq_api_fan_out_1() -> anyhow::Result<()> {
    // log::set_max_level(LevelFilter::Debug);

    let cfg: Config = Config::default();
    init(&cfg).await?;
    let _conn = conn();

    const EXCHANGER_NAME: &str = "ex";
    //-----------a--------------------------
    tokio::spawn(async move {
        for i in 0..3 {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            let q = format!("q_{}", i);

            let id = i;
            let _ = queue_of_fan_out(EXCHANGER_NAME, q.as_str())
                .await
                .unwrap()
                .set_delegate(move |delivery: lapin::message::DeliveryResult| async move {
                    println!("-----------enter callback------{}-----", id);

                    if delivery.is_err() {
                        println!("-----------consumer is err-----------");
                        return;
                    }
                    let delivery = delivery.unwrap();

                    if let Some(delivery) = delivery {
                        match std::str::from_utf8(delivery.data.as_slice()) {
                            Ok(body) => {
                                println!("recevie : {}", body);
                            }
                            Err(e) => {
                                error!("消息体解析失败： {}", e.to_string())
                            }
                        }
                    }
                });
        }
    });
    let _ = tokio::spawn(async move {
        println!("----after sleep-----");
        for i in 0..100 {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            let s = format!("msg_{}  hello!", i);
            match publish_json_fan_out(EXCHANGER_NAME, s.as_str()).await {
                Ok(_) => println!("publish result: ok {}", i),
                Err(e) => println!("publish result: ok {}", e.to_string()),
            }
        }
    })
    .await;

    Ok(())
}
