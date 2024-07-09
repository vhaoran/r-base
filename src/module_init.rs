use crate::*;
use anyhow::anyhow;
use tracing::*;

use super::module_cfg::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Module;

impl Module {
    #[cfg(feature = "rred")]
    pub fn init_redis(&self, cfg: &rred::Config) -> anyhow::Result<Self> {
        rred::init(cfg)?;
        Ok(self.clone())
    }

    #[cfg(feature = "rmongo")]
    pub async fn init_mongo(&self, cfg: &rmongo::Config) -> anyhow::Result<Self> {
        rmongo::init(cfg).await?;

        Ok(self.clone())
    }

    #[cfg(feature = "rpolo")]
    pub async fn init_polo(&self, cfg: &rpolo::Config) -> anyhow::Result<Self> {
        rpolo::init(cfg)?;
        Ok(self.clone())
    }

    #[cfg(feature = "rmq")]
    pub async fn init_mq(&self, cfg: &rmq::Config) -> anyhow::Result<Self> {
        rmq::init(cfg).await?;
        Ok(self.clone())
    }

    #[cfg(feature = "res")]
    pub async fn init_es(&self, cfg: &res::Config) -> anyhow::Result<Self> {
        res::init(cfg).await?;
        Ok(self.clone())
    }

    #[cfg(feature = "rnats")]
    pub async fn init_nats(&self, cfg: &rnats::Config) -> anyhow::Result<Self> {
        rnats::init(cfg).await?;
        Ok(self.clone())
    }

    #[cfg(feature = "rmy")]
    pub async fn init_mysql(&self) -> anyhow::Result<Self> {
        debug!("--init_my_sql_enter-------");
        Ok(self.clone())
    }

    pub fn init_log(&self, cfg: &rlog::Config) -> anyhow::Result<Self> {
        rlog::init(cfg)?;
        Ok(self.clone())
    }

    #[cfg(feature = "rsled")]
    pub fn init_sled(&self, cfg: &rsled::Config) -> anyhow::Result<Self> {
        rsled::init(cfg.clone())?;
        Ok(self.clone())
    }

    #[cfg(feature = "rlevel")]
    pub fn init_level(&self, cfg: &rlevel::Config) -> anyhow::Result<Self> {
        rlevel::init(cfg.clone())?;
        debug!("--after init_level_db-------");
        Ok(self.clone())
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct ModuleConfig {
    #[cfg(feature = "rred")]
    pub redis: Option<rred::Config>,
    #[cfg(feature = "rmongo")]
    pub mongo: Option<rmongo::Config>,
    #[cfg(feature = "rpolo")]
    pub polo: Option<rpolo::Config>,
    #[cfg(feature = "res")]
    pub es: Option<res::Config>,
    #[cfg(feature = "rmq")]
    pub mq: Option<rmq::Config>,
    #[cfg(feature = "rnats")]
    pub nats: Option<rnats::Config>,

    pub log: Option<rlog::Config>,
    #[cfg(feature = "rsled")]
    pub sled: Option<rsled::Config>,
    #[cfg(feature = "rlevel")]
    pub level: Option<rlevel::Config>,
}

impl ModuleConfig {
    pub fn is_log_debug(&self) -> bool {
        match &self.log {
            Some(v) => {
                let up = v.level.to_uppercase().trim().to_string();
                up.contains("DEBUG") || up.contains("TRACE")
            }
            _ => true,
        }
    }
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
            print!("##### log init starting ####");
            Module.init_log(&cfg)?;
            print!("----------- log init 完成 -----------");
            if !load_other {
                return Ok(config);
            }
        }
    }

    #[cfg(feature = "rmy")]
    Module.init_mysql().await?;

    #[cfg(feature = "rred")]
    if let Some(cfg) = cfg.redis {
        info!("##### redis init starting ####");
        Module.init_redis(&cfg)?;
        print!("-----------  init 完成 -----------");
    }
    #[cfg(feature = "rmongo")]
    if let Some(cfg) = cfg.mongo {
        info!("##### mongo init starting ####");

        Module.init_mongo(&cfg).await?;
        debug!("-----------  init 完成 -----------");
    }
    #[cfg(feature = "rpolo")]
    if let Some(cfg) = cfg.polo {
        info!("##### mongo init starting ####");
        let _ = Module.init_polo(&cfg).await?;
        debug!("-----------  init 完成 -----------");
    }

    #[cfg(feature = "res")]
    if let Some(cfg) = cfg.es {
        info!("##### es init starting ####");
        Module.init_es(&cfg).await?;
        print!("-----------  init 完成 -----------");
    }

    #[cfg(feature = "rmq")]
    if let Some(cfg) = cfg.mq {
        info!("##### rabbitMQ init starting ####");
        Module.init_mq(&cfg).await?;
        print!("-----------rabbitMQ  init 完成 -----------");
    }
    debug!("--after init mongo-------");

    #[cfg(feature = "rnats")]
    if let Some(cfg) = cfg.nats {
        info!("##### mq-nats init starting ####");
        Module.init_nats(&cfg).await?;
        info!("##### mq-nats init ok ####");
    }
    debug!("--after init nats-------");

    //-----------sled--------------------------
    #[cfg(feature = "rsled")]
    if let Some(cfg) = cfg.sled {
        info!("##### sled init starting ####");
        Module.init_sled(&cfg)?;
        info!("##### sled init ok ####");
    }
    debug!("--after init sled-------");

    #[cfg(feature = "rlevel")]
    if let Some(cfg) = cfg.level {
        info!("##### level-db init starting ####");
        Module.init_level(&cfg)?;
        info!("##### level-db init ok ####");
    }

    debug!("--after init level-------");

    debug!("--before leave init-------");
    if load_other {
        Ok(config)
    } else {
        error!("--init error-------");
        Err(anyhow!("not load options"))
    }
}

pub async fn init_modules(file_name: Option<&str>) -> anyhow::Result<ModuleConfig> {
    let r = self::init_module_n(file_name, true, true).await?;
    let _ = module_cfg::set(r.clone()).await;
    Ok(r)
}
