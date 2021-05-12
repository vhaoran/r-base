#[tokio::test]
async fn mp3_1() -> Result<(), Box<dyn std::error::Error>> {
    let p = "./a.mp3";
    let r = mp3_metadata::read_from_file(p)?;

    println!("-----------{:#?}-----------", r.duration);
    println!("-----------{:#?}-----------", r.tag);
    println!("-----------{:#?}-----------", r.optional_info);
    Ok(())
}
