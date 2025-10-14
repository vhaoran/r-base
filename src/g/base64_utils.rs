use base64::engine::general_purpose;
use base64::Engine;

pub fn str_2_base64(s: &str) -> anyhow::Result<String> {
    let mut dst = "".to_string();
    let _ = general_purpose::STANDARD.encode_string(s, &mut dst);
    if !dst.is_empty() {
        Ok(dst)
    } else {
        Err(anyhow!("transfer to base64 err",))
    }
}
pub fn base64_2_str(src: &str) -> anyhow::Result<String> {
    let mut r = Vec::<u8>::new();
    let _ = general_purpose::STANDARD.decode_vec(src, &mut r);
    let r = String::from_utf8(r)?;
    Ok(r)
}

mod t {
    use base64::engine::general_purpose;
    use base64::Engine;

    #[test]
    fn test_base64_2_str() -> anyhow::Result<()> {
        //---------------------
        let s = "12345长城6abc中国";
        let mut dst = "".to_string();
        let r = general_purpose::STANDARD.encode_string(s, &mut dst);
        println!("---encode test--------srcL {s}-\n r: {dst}----------",);

        let mut r = Vec::<u8>::new();
        let _ = general_purpose::STANDARD.decode_vec(dst, &mut r);
        let r1 = String::from_utf8(r);
        println!("---decode test--------srcL {s}-\n r: {r1:#?}----------",);

        Ok(())
    }

    #[test]
    fn test_base64() -> anyhow::Result<()> {
        //---------------------
        let s = "长城万里长，中国\\//.&^1234\n\r56abcdef";
        let r = super::str_2_base64(s)?;
        println!("---src: {s}-------\nr:-{r}-----------",);
        //
        let rr = super::base64_2_str(r.as_str())?;
        println!("---src: {r}-------\nr:-{rr}-----------",);
        Ok(())
    }
}
