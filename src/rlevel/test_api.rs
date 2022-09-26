use super::*;

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
async fn test_4_1() -> anyhow::Result<()> {
    let cfg = Config::default();
    super::init(cfg)?;
    //

    for i in 0..1000 {
        let key = format!("json_{i}");
        let v = format!("json_{i}_value_{i}");
        let r = set_str(key.as_str(), v.as_str()).await;
        self::flush().await;
        println!("-----------{key} has set-----------");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    Ok(())
}

#[tokio::test]
async fn test_4_2() -> anyhow::Result<()> {
    let cfg = Config::default();
    super::init(cfg)?;
    //
    let key = "json";

    for i in 0..1000 {
        let key = format!("json_{i}");
        let v = get_str(key.as_str()).await.unwrap_or("".to_string());
        println!("---{key}: {v}-----------");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    Ok(())
}

#[tokio::test]
async fn test_4_3() -> anyhow::Result<()> {
    let cfg = Config::default();
    super::init(cfg)?;
    //
    self::show_all().await;
    Ok(())
}
