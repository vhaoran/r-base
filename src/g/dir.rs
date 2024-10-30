use std::fs;
use std::path::{Path, PathBuf};
use tracing::*;

// pub fn get_pwd() -> String {
//     let p = std::env::current_dir().unwrap();
//     format!("{}", p.display())
// }

pub fn pwd() -> String {
    let p = std::env::current_dir().unwrap_or(Default::default());
    format!("{}", p.display())
}

pub fn app_path_name() -> String {
    let s = std::env::args().nth(0).unwrap_or("".to_string());
    s
}
pub fn app_dir() -> String {
    let s = self::app_path_name();
    match Path::new(s.as_str()).parent() {
        Some(v) => {
            format!("{}", v.display())
        }
        _ => "".to_string(),
    }
}

pub fn app_base() -> String {
    let s = self::app_path_name();
    match Path::new(s.as_str()).file_name() {
        Some(v) => match v.to_str() {
            Some(v) => v.to_string(),
            _ => "".to_string(),
        },
        _ => "".to_string(),
    }
}

pub fn verify_mkdir_parent_of_pathfile<T>(p: T) -> anyhow::Result<()>
where
    T: AsRef<str> + std::fmt::Display,
{
    let s = format!("{p}");
    let p = Path::new(s.as_str());
    match p.parent() {
        Some(p) => {
            if !p.exists() {
                let _ = fs::create_dir_all(p).map_err(|e| {
                    error!(
                        "---create_dir_error---{}-dir: {}",
                        e.to_string(),
                        p.display()
                    );
                    e
                })?;
            }
        }
        None => {}
    }
    Ok(())
}

pub fn verify_mkdir<T>(path: T) -> anyhow::Result<()>
where
    T: AsRef<str> + std::fmt::Display,
{
    let path = path.to_string();
    let p = Path::new(path.as_str()).to_path_buf();
    if !p.exists() {
        fs::create_dir_all(p)?
    }

    Ok(())
}
