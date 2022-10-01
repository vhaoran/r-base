// use super::*;
// use std::time;
//
// #[tokio::test]
// async fn red_1() -> anyhow::Result<()> {
//     let cfg: super::Config = Default::default();
//     super::init(&cfg)?;
//
//     println!("----init successful-----");
//
//     let a = INSTANCE.get().unwrap().clone();
//     let mut c = a.lock().await;
//
//     println!("----after lock-----");
//
//     let key = "whr";
//     c.simple_set(key, "whr is good")?;
//     println!("----after set-----");
//     let r: String = c.get(key)?;
//     println!("----after get-----");
//
//     println!("----test.rs---{:?}-----", r);
//
//     Ok(())
// }
//
// #[tokio::test]
// async fn red_2() -> anyhow::Result<()> {
//     let cfg: super::Config = Default::default();
//     super::init(&cfg)?;
//     println!("----init successful-----");
//
//     // Spawn tasks
//     let futures = (0..10000)
//         .map(|id| {
//             println!("----id {}---", id);
//             tokio::spawn(async move {
//                 let a = INSTANCE.get().unwrap().clone();
//                 let pool = a.lock().await;
//                 let mut c = pool.get().await.unwrap();
//
//                 println!("----enter {}-----", id);
//
//                 // let mut c = c.lock().await;
//                 let key = format!("whr_{}", id);
//                 let v = format!("whr_{}_value_{}", id, id);
//                 let _ = c.simple_set(key.clone(), v);
//                 let r: String = c.get(key).unwrap_or("".to_string());
//                 println!("----after get-- {}- = {}--", id, r);
//                 tokio::time::sleep(time::Duration::from_millis(5)).await;
//
//                 println!("---inner-id {}---", id);
//             })
//         })
//         .collect::<Vec<_>>();
//
//     for future in futures {
//         future.await.unwrap();
//     }
//
//     println!("---- end-----");
//     Ok(())
// }
//
// #[tokio::test]
// async fn red_3() -> anyhow::Result<()> {
//     let cfg: super::Config = Default::default();
//     super::init(&cfg)?;
//
//     println!("----init successful-----");
//     let l = (0..100)
//         .map(|id| {
//             tokio::spawn(async move {
//                 println!("----enter {}-----", id);
//                 let key = format!("whr_{}", id);
//                 let v = format!("whr_{}_value_{}", id, id);
//                 //
//                 let _ = super::set(key.as_str(), v.as_str()).await;
//                 let r = super::get(key.as_str())
//                     .await
//                     .unwrap_or(" not read data".to_string());
//
//                 println!("----after get-- {}- = {}--", id, r);
//                 tokio::time::sleep(time::Duration::from_secs(5)).await;
//             })
//         })
//         .collect::<Vec<_>>();
//
//     for i in l {
//         i.await?;
//     }
//
//     println!("---- end-----");
//     Ok(())
// }
