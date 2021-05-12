#[tokio::test]
async fn cnt_1() -> Result<(), Box<dyn std::error::Error>> {
    //
    const TOPIC: &str = "topic-test";
    const SECS: u64 = 5;
    const SECS_LOW: u64 = 2;
    const HOST: &str = "192.168.0.99:4222";
    const PWD: &str = "password";

    tokio::spawn(async move {
        let conn = nats::Options::with_user_pass("root", PWD)
            .with_name("My Rust NATS App")
            .max_reconnects(2000)
            .reconnect_buffer_size(60 * 1024)
            .disconnect_callback(|| println!("***********connection has been lost**********"))
            .reconnect_callback(|| println!(".....connecting........"))
            .connect(HOST)
            .expect("no conn**************");

        let c = conn.clone();

        for i in 0..10000000 {
            let s = format!("{}_msg....hello", i);
            let s2 = format!("** 2 cnt publish: {}_msg....hello", i);
            let r = conn.publish(TOPIC, s.as_str()).expect("no conn");
            let r = c.publish(TOPIC, s2.as_str()).expect("no conn");

            println!("--------{}---publish ---{:?}--------", i, r);
            tokio::time::sleep(std::time::Duration::from_secs(SECS)).await;
        }
    });

    tokio::spawn(async move {
        println!("----after sleep-----");
        let nc = nats::Options::with_user_pass("root", PWD)
            .with_name("My Rust NATS App")
            .connect(HOST)
            .expect(" ----########no conn.....");
        let sub = nc.subscribe(TOPIC).expect(" don't subscribe");
        println!("----########-------after sbu-----------");

        loop {
            for msg in sub.try_iter() {
                println!("---------------{:?}-------", msg);
                println!(
                    "---------------{:?}-------",
                    std::str::from_utf8(msg.data.as_slice())
                );
            }
            println!("-----########---iter end-----------");
            tokio::time::sleep(std::time::Duration::from_secs(SECS_LOW)).await;
        }
    })
    .await;

    Ok(())
}

#[tokio::test]
async fn cnt_2() -> Result<(), Box<dyn std::error::Error>> {
    const TOPIC: &str = "topic-test-1";
    const SECS: u64 = 5;
    const SECS_LOW: u64 = 2;
    const HOST: &str = "192.168.0.99:4222";
    const PWD: &str = "password";

    tokio::spawn(async move {
        let conn = nats::Options::with_user_pass("root", PWD)
            .with_name("My Rust NATS App")
            .max_reconnects(2000)
            .reconnect_buffer_size(60 * 1024)
            .disconnect_callback(|| println!("***********connection has been lost**********"))
            .reconnect_callback(|| println!(".....connecting........"))
            .connect(HOST)
            .expect("no conn**************");

        for i in 0..300 {
            let s = format!("{}_msg....hello", i);
            let reply = conn.new_inbox();
            let rsub = conn.subscribe(&reply).unwrap();

            let s = format!("msg_{}__", i);
            let r = conn.publish_request(TOPIC, &reply, s.as_str()).unwrap();
            let response = rsub.iter().take(1);
        }
        tokio::time::sleep(std::time::Duration::from_secs(SECS)).await;
    });

    tokio::spawn(async move {
        println!("----enter  subscribe task-----");
        let nc = nats::Options::with_user_pass("root", PWD)
            .with_name("My Rust NATS App")
            .connect(HOST)
            .expect(" ----########no conn.....");
        let sub = nc.subscribe(TOPIC).expect(" don't subscribe");

        loop {
            println!("-----------loop sub-----------");
            for msg in sub.try_iter() {
                println!(
                    "---####---receive: ---------{:?}-------",
                    std::str::from_utf8(msg.data.as_slice())
                );
                msg.ack();
            }
            tokio::time::sleep(std::time::Duration::from_secs(SECS_LOW)).await;
        }
    })
    .await;

    Ok(())
}
