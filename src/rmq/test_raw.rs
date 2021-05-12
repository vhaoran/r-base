use std::{env, thread};
#[tokio::test]
async fn mq_raw_1() -> Result<(), Box<dyn std::error::Error>> {
    use lapin::{
        message::DeliveryResult, options::*, publisher_confirm::Confirmation, types::FieldTable,
        BasicProperties, Connection, ConnectionProperties, Result,
    };

    use log::info;
    use tokio_amqp::*;
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    // let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());
    let addr = "amqp://root:password@192.168.0.99:5672/%2f";
    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;

    info!("CONNECTED");

    // let cnt = conn.clone();

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

    let _payload = b"Hello world!";

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

#[test]
fn fn_callback_a() {
    type Callback = fn(a: i32) -> i32;
    fn call(i: i32, f: Callback) {
        let a = f(i);
        println!("----test_raw.rs-callback result--{}-----", a);
    }

    call(16, |i| i + 10);
}
