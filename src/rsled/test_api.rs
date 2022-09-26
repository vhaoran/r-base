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
    self::flush().await;
    self::show_all().await;

    Ok(())
}
#[tokio::test]
async fn test_read_1() -> anyhow::Result<()> {
    let cfg = Config::default();
    super::init(cfg)?;
    println!("-----------after init-----------");
    self::show_all().await;

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
    self::set_str(key, s.as_str()).await;
    let r = self::get_str(key).await?;

    // println!("-----------{:?}-----------", r);
    println!("-----------{}-----------", r.len());
    println!("-----------millis: {i}-----------",);

    Ok(())
}
