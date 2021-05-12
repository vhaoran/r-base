pub fn get_pwd() -> String {
    let p = std::env::current_dir().unwrap();
    format!("{}", p.display())
}

pub fn app_path_name() -> String {
    let s = std::env::args().nth(0).unwrap();
    s
}

pub fn verify_mk_dir(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let p = std::path::Path::new(path.as_str()).to_path_buf();
    if !p.exists() {
        std::fs::create_dir(p)?
    }

    Ok(())
}
