#[cfg(feature = "rmy")]
mod t {
    use sqlx::mysql::MySqlPool;
    #[tokio::test]
    async fn test_pool() -> anyhow::Result<()> {
        println!("-----------start-----------",);

        // mysql:
        // root:password@localhost/db
        let url = "mysql://root:password@w5/test";
        let pool = MySqlPool::connect(url).await?;
        //
        println!("-----------ok-----------",);
        Ok(())
    }
}
