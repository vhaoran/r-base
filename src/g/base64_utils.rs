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
