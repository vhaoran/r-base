// use std::collections::HashMap;
// //-------------------------------------
// use log::*;
// use rbatis::core::db::DBPoolOptions;
// use rbatis::crud::CRUD;
// use rbatis::rbatis::Rbatis;
// use rbson::{bson, Bson};
//
// #[crud_table(table_name:"t")]
// #[derive(Clone, Debug)]
// pub struct BizActivity {
//     pub id: Option<i64>,
//     pub name: Option<String>,
// }
// // this macro will create impl BizActivity{ pub fn id()->&str ..... }
// impl_field_name_method!(BizActivity { id, name });
//
// pub async fn cnt() -> anyhow::<Rbatis, Box<dyn std::error::Error>> {
//     fast_log::init(fast_log::config::Config::new().console());
//     // initialize rbatis. May use `lazy_static` crate to define rbatis as a global variable because rbatis is thread safe
//
//     let url = "mysql://root:password@w99:3306/test";
//     let rb = Rbatis::new();
//     // connect to database
//     // rb.link("mysql://root:password@w99:3306/test")
//     //     .await
//     //     .unwrap();
//     let mut opt = DBPoolOptions::new();
//     opt.max_connections = 100;
//     rb.link_opt(url, opt).await.unwrap();
//
//     Ok(rb)
// }
//
// #[tokio::test]
// async fn rb_1() anyhow::Result<()> {
//     //
//     let conn = self::cnt().await?;
//     /// customize connection pool parameters (optional)
//     // let mut opt =PoolOptions::new();
//     // opt.max_size=100;
//     // rb.link_opt("mysql://root:123456@localhost:3306/test",&opt).await.unwrap();
//
//     /// newly constructed wrapper sql logic
//     let wrapper = conn
//         .new_wrapper()
//         .eq("id", 2) //sql:  id = 1
//         .and() //sql:  and
//         .ne(BizActivity::id(), 1) //sql:  id <> 1
//         .in_array("id", &[1, 2, 3]) //sql:  id in (1,2,3)
//         .not_in("id", &[1, 2, 3]) //sql:  id not in (1,2,3)
//         .like("name", 1) //sql:  name like 1
//         .or() //sql:  or
//         .not_like(BizActivity::name(), "asdf") //sql:  name not like 'asdf'
//         // .between("create_time", "2020-01-01 00:00:00", "2020-12-12 00:00:00")//sql:  create_time between '2020-01-01 00:00:00' and '2020-01-01 00:00:00'
//         // .group_by(&["id"])              //sql:  group by id
//         .order_by(true, &["id", "name"]);
//
//     let activity = BizActivity {
//         id: None,
//         name: None,
//     };
//     /// saving
//     let r = conn.save(&activity, &[]).await;
//     println!("--save: {:?}-------", r);
//
//     //Exec ==> INSERT INTO biz_activity (create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version) VALUES ( ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? )
//
//     /// batch saving
//     conn.save_batch(&vec![activity], &[]).await;
//     //Exec ==> INSERT INTO biz_activity (create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version) VALUES ( ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? ),( ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? )
//
//     /// fetch allow None or one result. column you can use BizActivity::id() or "id"
//     let result: Option<BizActivity> = conn.fetch_by_column(BizActivity::id(), "1").await.unwrap();
//     //Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1  AND id =  ?
//     println!("--fetch: {:?}-------", r);
//
//     /// query all
//     let result: Vec<BizActivity> = conn.fetch_list().await.unwrap();
//     //Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1
//     println!("--list: {:?}-------", r);
//
//     ///query by id vec
//     let result: Vec<BizActivity> = conn.fetch_list_by_column("id", &["1"]).await.unwrap();
//     //Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1  AND id IN  (?)
//     println!("--list_by: {:?}-------", r);
//
//     ///query by wrapper
//     let r: Result<Option<BizActivity>, _> = conn
//         .fetch_by_wrapper(conn.new_wrapper().eq("id", "1"))
//         .await;
//     //Query ==> SELECT  create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1  AND id =  ?
//     println!("--fetch_by_wrapper: {:?}-------", r);
//
//     ///delete
//     let r = conn.remove_by_column::<BizActivity, _>("id", &"1").await;
//     //Exec ==> UPDATE biz_activity SET delete_flag = 0 WHERE id = 1
//     println!("--remove_by_column: {:?}-------", r);
//
//     ///delete batch
//     // rb.remove_batch_by_column::<BizActivity, _>("id", &["1", "2"])
//     //     .await;
//     //Exec ==> UPDATE biz_activity SET delete_flag = 0 WHERE id IN (  ?  ,  ?  )
//
//     ///update
//     // let mut activity = activity.clone();
//     // let r = rb.update_by_column("id", &activity).await;
//     // //Exec   ==> update biz_activity set  status = ?, create_time = ?, version = ?, delete_flag = ?  where id = ?
//     // rb.update_by_wrapper(
//     //     &activity,
//     //     rb.new_wrapper().eq("id", "12312"),
//     //     &[Skip::Value(&serde_json::Value::Null), Skip::Column("id")],
//     // )
//     // .await;
//
//     //Exec ==> UPDATE biz_activity SET  create_time =  ? , delete_flag =  ? , status =  ? , version =  ?  WHERE id =  ?
//     Ok(())
// }
//
// #[tokio::test]
// async fn test_add_2() anyhow::Result<()> {
//     let conn = self::cnt().await?;
//     /// customize connection pool parameters (optional)
//     let activity = BizActivity {
//         id: Some(5_i64),
//         name: None,
//     };
//     /// saving
//     let r = conn.save(&activity, &[]).await;
//     println!("--save: {:?}-------", r);
//
//     let result: Vec<BizActivity> = conn.fetch_list().await.unwrap();
//     //Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1
//     println!("--list: {:?}-------", r);
//
//     Ok(())
// }
//
// #[tokio::test]
// async fn test_ch_3() anyhow::Result<()> {
//     let conn = self::cnt().await?;
//     /// customize connection pool parameters (optional)
//     let activity = BizActivity {
//         id: Some(5_i64),
//         name: None,
//     };
//     //
//     // conn.update_by_column()
//     let r = conn.save(&activity, &[]).await;
//     println!("--save: {:?}-------", r);
//
//     let result: Vec<BizActivity> = conn.fetch_list().await.unwrap();
//     //Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1
//     println!("--list: {:?}-------", r);
//
//     Ok(())
// }
// #[tokio::test]
// async fn test_wrapper_4() anyhow::Result<()> {
//     let conn = self::cnt().await?;
//     let w = conn
//         .new_wrapper()
//         .push_sql(" id > ?  ")
//         .push_arg(0)
//         .push_sql(" and name <> ?  ")
//         .push_arg("a");
//
//     println!("--SQL: {:?}-------", w.sql);
//     let r: Vec<BizActivity> = conn.fetch_list_by_wrapper(w).await.unwrap();
//     println!("-----------{:?}-----------", r);
//
//     Ok(())
// }
//
// #[tokio::test]
// async fn test_raw_sql_5() anyhow::Result<()> {
//     let conn = self::cnt().await?;
//     let r: Vec<HashMap<String, Bson>> = conn
//         .fetch(
//             r#"select sum(id) as id,name from t
//             group by name
//         "#,
//             vec![],
//         )
//         .await?;
//     println!("-----------{:?}-----------", r);
//     println!("-----------abc-----------");
//
//     Ok(())
// }
