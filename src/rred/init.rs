// // impl Cnt {}
//
// //
// // extern crate redis_rs;
//
// use async_trait::async_trait;
// use tracing::*;
// use once_cell::sync::OnceCell;
//
// use redisclient::RedisClient;
// use redisclient::RedisError;
// use redisclient::RedisResult;
//
// use std::net::TcpStream;
// use std::sync::Arc;
// use tokio::sync::Mutex as Mu;
//
// use super::Config;
//
// struct Manager {
//     pub cfg: Config,
// }
//
// type RedisPool = deadpool::managed::Pool<RedisClient, Box<dyn std::error::Error>>;
// pub static INSTANCE: OnceCell<Arc<Mu<RedisPool>>> = OnceCell::new();
//
// #[async_trait]
// // impl deadpool::managed::Manager<RedisClient, Box<dyn std::error::Error>> for Manager {
// impl deadpool::managed::Manager for Manager {
//     type Type = RedisClient;
//     type Error = anyhow::Error;
//     async fn create(&self) -> anyhow::<RedisClient, Box<dyn std::error::Error>> {
//         println!("----computer-----created....");
//         let c = create_cnt(self.cfg.clone().to_owned())?;
//         Ok(c)
//     }
//     async fn recycle(
//         &self,
//         conn: &mut RedisClient,
//     ) -> deadpool::managed::RecycleResult<Box<dyn std::error::Error>> {
//         println!("----computer-----recycled....");
//         Ok(())
//     }
// }
//
// pub fn create_cnt(cfg: Config) -> anyhow::<RedisClient, Box<dyn std::error::Error>> {
//     println!(".############# CCC create redis pool..............");
//     debug!(".############# CCC create redis pool..............");
//
//     let host = cfg.host.as_str();
//     let port = cfg.port;
//     let addr = format!("{}:{}", host, port);
//
//     let cfg = redisclient::config::RedisConfig {
//         address: addr,
//         database: 0,
//         username: None,
//         password: None,
//         pool_capacity: 10,
//     };
//
//     let c = RedisClient::with_config(cfg)?;
//
//     Ok(c)
// }
//
// pub fn init(cfg: &Config) -> anyhow::Result<()> {
//     debug!("-----rred(is) init start----");
//
//     let mgr = Manager {
//         cfg: cfg.clone().to_owned(),
//     };
//     let pool = RedisPool::new(mgr, 100);
//
//     let a = Arc::new(Mu::new(pool));
//     if let Err(_) = INSTANCE.set(a) {
//         panic!("redis init error");
//     }
//
//     debug!("-----rred(is) init ok----");
//
//     Ok(())
// }
