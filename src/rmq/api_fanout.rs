use super::*;
use tracing::*;

use lapin::{
    message::Delivery, message::DeliveryResult, options::*, publisher_confirm::Confirmation,
    types::FieldTable, BasicProperties, Connection, ConnectionProperties, ExchangeKind,
};
use serde::{Deserialize, Serialize};
// use serde_json::*;

type Callback = fn(delivery: &Delivery, body: &str) -> anyhow::Result<()>;

// const ROUTING_KEY: &str = "tel-fan-out";
pub async fn publish_json_fan_out<T>(exchanger_name: &str, body: T) -> anyhow::Result<()>
where
    T: Serialize,
{
    let s = serde_json::to_string(&body)?;

    debug!("shall publish msg: {}", s);
    publish_fan_out(exchanger_name, s.as_str()).await?;
    debug!("after publish msg: {}", s);

    Ok(())
}

async fn publish_fan_out(exchanger_name: &str, body: &str) -> anyhow::Result<()> {
    let conn = conn().await;
    //
    let ch = conn.create_channel().await?;
    let p_opt = BasicPublishOptions {
        mandatory: false,
        immediate: false,
    };

    ch.exchange_declare(
        exchanger_name,
        ExchangeKind::Fanout,
        ExchangeDeclareOptions::default(),
        FieldTable::default(),
    )
    .await?;

    let _confirm = ch
        .basic_publish(
            exchanger_name,
            "",
            p_opt,
            body.as_bytes(),
            BasicProperties::default(),
        )
        .await?
        .await?;

    Ok(())
}

// pub async fn consume_fan_out(
//     queue_name: &str,
//     exchanger_name: &str,
//     callback: Callback,
// ) anyhow::Result<()> {
//     let conn = cnt();
//     //
//     let ch = conn.create_channel().await?;
//     let opt = QueueDeclareOptions {
//         passive: false,
//         durable: false,
//         exclusive: false,
//         auto_delete: true,
//         nowait: false,
//     };
//
//     let _q = ch
//         .queue_declare(queue_name, opt, FieldTable::default())
//         .await?;
//     //
//     ch.exchange_declare(
//         exchanger_name,
//         ExchangeKind::Fanout,
//         ExchangeDeclareOptions::default(),
//         FieldTable::default(),
//     )
//     .await?;
//     ch.queue_bind(
//         queue_name,
//         exchanger_name,
//         "",
//         QueueBindOptions::default(),
//         FieldTable::default(),
//     )
//     .await?;
//
//     //
//     let consumer = ch
//         .basic_consume(
//             queue_name,
//             "",
//             BasicConsumeOptions::default(),
//             FieldTable::default(),
//         )
//         .await?;
//
//     consumer.set_delegate(move |delivery: DeliveryResult| async move {
//         if delivery.is_err() {
//             return;
//         }
//         let delivery = delivery.unwrap();
//         if let Some((_, delivery)) = delivery {
//             match std::str::from_utf8(delivery.data.as_slice()) {
//                 Ok(body) => {
//                     let r = callback(&delivery, body);
//                     if r.is_err() {
//                         error!("消息回调失败： {}", r.err().unwrap().to_string())
//                     }
//                 }
//                 Err(e) => {
//                     error!("消息体解析失败： {}", e.to_string())
//                 }
//             }
//         }
//     })?;
//
//     Ok(())
// }

pub async fn queue_of_fan_out(
    exchanger_name: &str,
    queue_name: &str,
) -> anyhow::Result<lapin::Consumer> {
    let conn = conn().await;
    //
    let ch = conn.create_channel().await?;
    let opt = QueueDeclareOptions {
        passive: false,
        durable: false,
        exclusive: false,
        auto_delete: true,
        nowait: false,
    };

    let _q = ch
        .queue_declare(queue_name, opt, FieldTable::default())
        .await?;
    ch.exchange_declare(
        exchanger_name,
        ExchangeKind::Fanout,
        ExchangeDeclareOptions::default(),
        FieldTable::default(),
    )
    .await?;

    ch.queue_bind(
        queue_name,
        exchanger_name,
        "",
        QueueBindOptions::default(),
        FieldTable::default(),
    )
    .await?;

    let mut c_opt = BasicConsumeOptions::default();
    c_opt.no_ack = true;
    c_opt.exclusive = false;
    c_opt.no_local = false;
    c_opt.nowait = false;

    //
    let c = ch
        .basic_consume(queue_name, "", c_opt, FieldTable::default())
        .await?;
    Ok(c)
}

// pub async fn consumer_of_queue_fan_out_ack(
//     queue_name: &str,
//     exchanger_name: &str,
// ) -> anyhow::<lapin::Consumer, Box<dyn std::error::Error>> {
//     let conn = cnt();
//     //
//     let ch = conn.create_channel().await?;
//     let opt = QueueDeclareOptions {
//         passive: false,
//         durable: false,
//         exclusive: false,
//         auto_delete: true,
//         nowait: false,
//     };
//
//     let _q = ch
//         .queue_declare(queue_name, opt, FieldTable::default())
//         .await?;
//     ch.exchange_declare(
//         exchanger_name,
//         ExchangeKind::Fanout,
//         ExchangeDeclareOptions::default(),
//         FieldTable::default(),
//     )
//     .await?;
//     ch.queue_bind(
//         queue_name,
//         exchanger_name,
//         "",
//         QueueBindOptions::default(),
//         FieldTable::default(),
//     )
//     .await?;
//
//     let mut c_opt = BasicConsumeOptions::default();
//     c_opt.no_ack = false;
//     c_opt.exclusive = false;
//     c_opt.no_local = false;
//     c_opt.nowait = false;
//
//     //
//     let c = ch
//         .basic_consume(
//             queue_name,
//             "my_consumer",
//             // BasicConsumeOptions::default(),
//             c_opt,
//             FieldTable::default(),
//         )
//         .await?;
//     Ok(c)
// }
