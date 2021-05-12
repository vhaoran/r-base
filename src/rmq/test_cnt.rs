use super::*;

use lapin::{
    message::Delivery, message::DeliveryResult, options::*, publisher_confirm::Confirmation,
    types::FieldTable, BasicProperties, Connection, ConnectionProperties,
};
use log::*;

#[tokio::test]
async fn mq_cnt_1() -> Result<(), Box<dyn std::error::Error>> {
    let cfg: Config = std::default::Default::default();
    init(&cfg).await?;
    let conn = conn().await;
    //
    let channel_a = conn.create_channel().await?;
    let channel_b = conn.create_channel().await?;

    let queue = channel_a
        .queue_declare(
            "hello",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let _queue2 = channel_b
        .queue_declare(
            "hello",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    info!("Declared queue {:?}", queue);

    let consumer = channel_b
        .basic_consume(
            "hello",
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    consumer.set_delegate(move |delivery: DeliveryResult| async move {
        let delivery = delivery.expect("error caught in in consumer");
        if let Some(delivery) = delivery {
            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("failed to ack");
            let s = std::str::from_utf8(delivery.data.as_slice());
            println!("----test_sqlx------{:?}--", s);
        }
    });

    let payload = b"Hello world!";

    for i in 0..100u16 {
        println!(" {:?}", i);
        let confirm = channel_a
            .basic_publish(
                "",
                "hello",
                BasicPublishOptions::default(),
                format!("hello,time: {:?}", std::time::SystemTime::now()).as_bytes(),
                BasicProperties::default(),
            )
            .await?
            .await?;
        assert_eq!(confirm, Confirmation::NotRequested);
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    }

    Ok(())
}
