use super::*;
use crate::g;

#[tokio::test]
async fn test_1() -> anyhow::Result<()> {
    let cfg = Config::default();
    super::init(cfg)?;
    println!("-----------after init-----------");

    //
    let r = set_str("abc", "test_value").await;
    println!("-----------{:?}-----------", r);

    let r: String = get_str("abc").await?;
    println!("-----------{:?}-----------", r);

    Ok(())
}

#[tokio::test]
async fn test_2() -> anyhow::Result<()> {
    let cfg = Config::default();
    super::init(cfg)?;
    println!("-----------after init-----------");

    //
    let key = "json";
    let r = set_json(key, vec![1, 2, 3, 4]).await;
    println!("-----------{:?}-----------", r);

    let r: Vec<i64> = get_json(key).await?;
    println!("-----------{:?}-----------", r);

    Ok(())
}

#[tokio::test]
async fn test_3() -> anyhow::Result<()> {
    let cfg = Config::default();
    super::init(cfg)?;
    //
    let key = "json";
    let r = set_json(key, vec![1, 2, 3, 4]).await;
    println!("-----------{:?}-----------", r);

    let r: Vec<i64> = get_json(key).await?;
    println!("-----------{:?}-----------", r);

    Ok(())
}

#[tokio::test]
async fn test_l_1() -> anyhow::Result<()> {
    let cfg = Config::default();
    println!("--before init-------");

    super::init(cfg)?;
    //println!("--after init-------");

    let i0 = g::now().timestamp_millis();
    for i in 0..1_000_000 {
        let key = format!("json_key_{i}");
        let v = format!("json_{i}_va{i}");
        let r = set_str(key.as_str(), v.as_str()).await;
        if i % 10000 == 0 {
            println!(
                "----level---{i}----ms: {}-----------",
                g::now().timestamp_millis() - i0
            );
        }
    }
    let _ = self::flush().await;
    println!(
        "-----------lenvel end: ms: {}----------",
        g::now().timestamp_millis() - i0
    );

    Ok(())
}

#[tokio::test]
async fn test_l_2() -> anyhow::Result<()> {
    let cfg = Config::default();
    super::init(cfg)?;
    //
    // let key = "json";

    for i in 0..1000 {
        let key = format!("json_{i}");
        let v = get_str(key.as_str()).await.unwrap_or("".to_string());
        println!("---{key}: {v}-----------");
        // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    Ok(())
}

#[tokio::test]
async fn test_4_3() -> anyhow::Result<()> {
    let cfg = Config::default();
    super::init(cfg)?;
    //
    let _ = self::show_all().await;
    Ok(())
}
