use super::*;
use log::*;

#[tokio::test]
async fn init_module_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("test begin ");
    let r = super::init_modules(None).await;
    match r {
        Ok(_) => println!(" init_modules ok"),
        Err(e) => println!("{}", e.to_string()),
    }

    println!("all 完成");
    Ok(())
}
