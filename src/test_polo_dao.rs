use serde::{Deserialize, Serialize};
use std::collections::HashSet;

static TB: &str = "my_table";
#[tokio::test]
async fn aaa() -> anyhow::Result<()> {
    //use mongodb::bson::oid;

    #[derive(Serialize, Deserialize, Clone, Default, Debug)]
    pub struct Book {
        #[serde(rename = "_id")]
        pub id: Option<i64>,
        pub name: Option<String>,
    }

    super::polo_base!(TB, Book);

    let s = tb_name();
    println!("-----------{}-----------", s);
    // let r = exist(doc! {}).await;
    // println!("-----------r {:?}-----------", r);
    Ok(())
}
