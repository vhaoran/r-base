use futures::StreamExt;

///
/// when test then change cargo.toml
/// add fetchre to default: rnatsx
///
#[tokio::test]
async fn test_natsx() -> anyhow::Result<()> {
    let cfg = super::Config {
        hosts: vec![
            "nats://w5:4222".to_string(),
            "nats://w5:4223".to_string(),
            "nats://w5:4224".to_string(),
        ],
        user_name: "root".to_string(),
        pwd: "password".to_string(),
    };
    let _ = super::init(&cfg).await.map_err(|e| {
        println!("---init_error---{}-", e.to_string());
        e
    })?;
    const TOPIC: &str = "test-topic";
    println!("-----------start-----------",);

    tokio::spawn(async move {
        for i in 0..100000 {
            let body = format!(" {i} of msg");
            let _ = super::publish(TOPIC, body.as_str()).await;
            println!("-----------wait send-----------",);
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    });
    //
    let _ = tokio::spawn(async move {
        let cnt = super::cnt();
        //
        let mut subscriber = cnt.subscribe(TOPIC).await.unwrap();
        while let Some(message) = subscriber.next().await {
            let s = String::from_utf8(message.payload.as_ref().to_vec()).unwrap();
            println!("###### {s}");
        }
    });
    //-------------------------------------
    let _ = tokio::spawn(async move {
        let cnt = super::cnt();
        //
        let mut subscriber = cnt.subscribe(TOPIC).await.unwrap();
        while let Some(message) = subscriber.next().await {
            let s = String::from_utf8(message.payload.as_ref().to_vec()).unwrap();
            println!("******* {s}");
        }
    })
    .await;

    Ok(())
}
