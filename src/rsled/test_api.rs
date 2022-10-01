use super::*;
use crate::g::date;

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

    println!("-----------begin_show_all-----------",);
    let _ = self::flush().await;
    let _ = self::show_all().await;

    Ok(())
}
#[tokio::test]
async fn test_read_1() -> anyhow::Result<()> {
    let cfg = Config::default();
    super::init(cfg)?;
    println!("-----------after init-----------");
    let _ = self::show_all().await;

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
async fn test_4() -> anyhow::Result<()> {
    let cfg = Config::default();
    super::init(cfg)?;
    println!("-----------after init-----------");
    //
    let key = "json";
    let mut s = "".to_string();

    let i = date::now().timestamp_millis();
    for i in 0..10_000 {
        s = format!("{s}{i}_value");
    }
    let i = date::now().timestamp_millis() - i;
    //
    let _ = self::set_str(key, s.as_str()).await;
    let r = self::get_str(key).await?;

    // println!("-----------{:?}-----------", r);
    println!("-----------{}-----------", r.len());
    println!("-----------millis: {i}-----------",);

    Ok(())
}

#[tokio::test]
async fn test_write_41() -> anyhow::Result<()> {
    let cfg = Config::default();
    super::init(cfg)?;
    println!("-----------after init-----------");
    //

    let i0 = date::now().timestamp_millis();
    for i in 0..1_000_000 {
        let key = format!("json_{i}",);
        let s = format!("json_value_{i}",);
        let _ = self::set_str(key.as_str(), s.as_str()).await;
        // let r = self::get_str(key.as_str()).await;

        if i % 10000 == 0 {
            println!(
                "----sled: {i}----ms: {}-----------",
                date::now().timestamp_millis() - i0
            );
        }
    }
    self::flush().await;

    println!(
        "--------sled---ms: {}-----------",
        date::now().timestamp_millis() - i0
    );

    Ok(())
}
#[tokio::test]
async fn test_read_41() -> anyhow::Result<()> {
    let cfg = Config::default();
    super::init(cfg)?;
    println!("-----------after init-----------");
    //

    let i0 = date::now().timestamp_millis();
    for i in 0..100_000 {
        let key = format!("json_{i}",);
        let r = self::get_str(key.as_str()).await;
        // let r = self::get_str(key.as_str()).await;

        if i % 10000 == 0 {
            println!("-----------{r:#?}-----------",);
        }
    }
    // self::flush().await;

    println!(
        "--------sled---ms: {}-----------",
        date::now().timestamp_millis() - i0
    );

    Ok(())
}
