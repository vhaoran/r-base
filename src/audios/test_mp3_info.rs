#[tokio::test]
async fn mp3_1() -> anyhow::Result<()> {
    let p = "./a.mp3";
    let r = mp3_metadata::read_from_file(p)?;

    println!("-----------{:#?}-----------", r.duration);
    println!("-----------{:#?}-----------", r.tag);
    println!("-----------{:#?}-----------", r.optional_info);
    Ok(())
}
