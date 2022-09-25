use crate::*;
use anyhow::anyhow;
use log::*;

use super::module_cfg::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Module;

impl Module {
    pub fn init_redis(&self, cfg: &rred::Config) -> anyhow::Result<Self> {
        rred::init(cfg)?;
        Ok(self.clone())
    }
    pub async fn init_mongo(&self, cfg: &rmongo::Config) -> anyhow::Result<Self> {
        rmongo::init(cfg).await?;
        Ok(self.clone())
    }
    pub async fn init_mq(&self, cfg: &rmq::Config) -> anyhow::Result<Self> {
        rmq::init(cfg).await?;
        Ok(self.clone())
    }
    pub async fn init_es(&self, cfg: &res::Config) -> anyhow::Result<Self> {
        res::init(cfg).await?;
        Ok(self.clone())
    }

    pub async fn init_nats(&self, cfg: &rnats::Config) -> anyhow::Result<Self> {
        rnats::init(cfg).await?;
        Ok(self.clone())
    }

    #[cfg(feature = "rmy")]
    pub async fn init_mysql(&self) -> Result<Self, Box<dyn std::error::Error>> {
        debug!("--init_my_sql_enter-------");

        Ok(self.clone())
    }

    pub fn init_log(&self, cfg: &rlog::Config) -> anyhow::Result<Self> {
        rlog::init(cfg)?;
        Ok(self.clone())
    }
    pub fn init_sled(&self, cfg: &rsled::Config) -> anyhow::Result<Self> {
        rsled::init(cfg.clone())?;
        Ok(self.clone())
    }
    pub fn init_level(&self, cfg: &rlevel::Config) -> anyhow::Result<Self> {
        rlevel::init(cfg.clone())?;
        debug!("--after init_level_db-------");
        Ok(self.clone())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModuleConfig {
    pub redis: Option<rred::Config>,
    pub mongo: Option<rmongo::Config>,
    pub es: Option<res::Config>,
    pub mq: Option<rmq::Config>,
    pub nats: Option<rnats::Config>,
    pub log: Option<rlog::Config>,
    pub sled: Option<rsled::Config>,
    pub level: Option<rlevel::Config>,
}

pub fn get_module_config(file_name: Option<&str>) -> anyhow::Result<ModuleConfig> {
    let name = file_name.unwrap_or("config.yml");

    let p = std::env::current_dir()?.join(name);

    println!("read config from path {}", p.display());

    let s = std::fs::read_to_string(p.as_path())?;
    //
    let r: ModuleConfig = serde_yaml::from_str(s.as_str())?;
    Ok(r)
}

pub async fn init_module_n(
    file_name: Option<&str>,
    load_log: bool,
    load_other: bool,
) -> anyhow::Result<ModuleConfig> {
    if !load_log && !load_other {
        return Err(anyhow!("nothing to load...."));
    }

    print!("--开始读取配置文件----");
    let cfg = get_module_config(file_name)?;

    let config = cfg.clone();
    //only for test

    //
    if load_log {
        if let Some(cfg) = cfg.log {
            print!("##### log init start ####");
            Module.init_log(&cfg)?;
            print!("----------- log init 完成 -----------");
            if !load_other {
                return Ok(config);
            }
        }
    }

    #[cfg(feature = "rmy")]
    Module.init_mysql().await?;

    if let Some(cfg) = cfg.redis {
        info!("##### redis init start ####");
        Module.init_redis(&cfg)?;
        print!("-----------  init 完成 -----------");
    }
    if let Some(cfg) = cfg.mongo {
        info!("##### mongo init start ####");
        Module.init_mongo(&cfg).await?;
        print!("-----------  init 完成 -----------");
    }

    if let Some(cfg) = cfg.es {
        info!("##### es init start ####");
        Module.init_es(&cfg).await?;
        print!("-----------  init 完成 -----------");
    }
    if let Some(cfg) = cfg.mq {
        info!("##### rabbitMQ init start ####");
        Module.init_mq(&cfg).await?;
        print!("-----------rabbitMQ  init 完成 -----------");
    }

    if let Some(cfg) = cfg.nats {
        info!("##### mq-nats init start ####");
        Module.init_nats(&cfg).await?;
        info!("##### mq-nats init ok ####");
    }
    //-----------sled--------------------------
    if let Some(cfg) = cfg.sled {
        info!("##### sled init start ####");
        Module.init_sled(&cfg)?;
        info!("##### sled init ok ####");
    }
    if let Some(cfg) = cfg.level {
        info!("##### level-db init start ####");
        Module.init_level(&cfg)?;
        info!("##### level-db init ok ####");
    }

    if load_other {
        Ok(config)
    } else {
        Err(anyhow!("not load options"))
    }
}

pub async fn init_modules(file_name: Option<&str>) -> anyhow::Result<ModuleConfig> {
    let r = self::init_module_n(file_name, true, true).await?;

    Ok(r)
}
