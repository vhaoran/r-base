#[tokio::test]
async fn test_aaa() -> anyhow::Result<()> {
    super::store_kv!(i64, String, "./x.yml", 5000);

    let _ = set(1, "good".to_string(), 10).await;
    let _ = flush_to_disk().await;
    println!("-----------ok-----------",);

    Ok(())
}
