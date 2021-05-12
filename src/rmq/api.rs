use super::*;
use log::*;

use lapin::{
    message::Delivery, message::DeliveryResult, options::*, publisher_confirm::Confirmation,
    types::FieldTable, BasicProperties, Connection, ConnectionProperties,
};
use serde::{Deserialize, Serialize};
// use serde_json::*;

type Callback = fn(delivery: &Delivery, body: &str) -> Result<(), Box<dyn std::error::Error>>;

pub async fn publish_basic_json<T>(
    queue_name: &str,
    body: T,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize,
{
    let s = serde_json::to_string(&body)?;

    debug!("publish msg: {}", s);

    publish_basic(queue_name, s.as_str()).await
    // Ok(())
}

pub async fn publish_basic(queue_name: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
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

    let p_opt = BasicPublishOptions {
        mandatory: false,
        immediate: false,
    };

    let _q = ch
        .queue_declare(queue_name, opt, FieldTable::default())
        .await?;
    let _confirm = ch
        .basic_publish(
            "",
            queue_name,
            p_opt,
            body.as_bytes(),
            BasicProperties::default(),
        )
        .await?
        .await?;

    Ok(())
}

// pub async fn consume_basic(
//     queue_name: &str,
//     callback: Callback,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let conn = cnt().await;
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
//     let consumer = ch
//         .basic_consume(
//             queue_name,
//             "my_consumer",
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
//         if let Some(delivery) = delivery {
//             // delivery
//             //     .ack(BasicAckOptions::default())
//             //     .await
//             //     .expect("failed to ack");
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
//     });
//
//     Ok(())
// }

pub async fn consumer_of_queue(
    queue_name: &str,
) -> Result<lapin::Consumer, Box<dyn std::error::Error>> {
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

    let mut c_opt = BasicConsumeOptions::default();
    c_opt.no_ack = true;
    c_opt.exclusive = false;
    c_opt.no_local = false;
    c_opt.nowait = false;

    //
    let c = ch
        .basic_consume(
            queue_name,
            "my_consumer",
            // BasicConsumeOptions::default(),
            c_opt,
            FieldTable::default(),
        )
        .await?;
    Ok(c)
}
pub async fn consumer_of_queue_ack(
    queue_name: &str,
) -> Result<lapin::Consumer, Box<dyn std::error::Error>> {
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

    let mut c_opt = BasicConsumeOptions::default();
    c_opt.no_ack = false;
    c_opt.exclusive = false;
    c_opt.no_local = false;
    c_opt.nowait = false;

    //
    let c = ch
        .basic_consume(
            queue_name,
            "my_consumer",
            // BasicConsumeOptions::default(),
            c_opt,
            FieldTable::default(),
        )
        .await?;
    Ok(c)
}
