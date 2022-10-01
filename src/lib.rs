#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate cached;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_json;

// #[macro_use]
// extern crate ormlite;
// #[macro_use]
// extern crate sqlx;

//----pub fn/struct--------------------------

use std::error::Error;

pub use module_cfg::*;
pub use module_init::*;

//--------macro--------------------------
pub mod audios;
#[macro_use]
pub mod verify;
#[macro_use]
pub mod m_dao;

#[macro_use]
mod m_dyn_dao;

#[macro_use]
pub mod cache_wrapper;

// #[cfg(feature = "rmy")]
// #[macro_use]
// extern crate rbatis;

//--------pub--module------------------------
pub mod g;
pub mod res;
pub mod rlog;
pub mod rmongo;
pub mod rmq;
pub mod rpolo;
// pub mod rr;

// #[cfg(feature = "rmy")]
// pub mod rmy;

pub mod rlevel;
// pub mod rlite;
pub mod rnats;
pub mod rred;
pub mod rsled;
//------common module------------------
mod module_cfg;
mod module_init;
//------test module------------------
mod cache_test;
mod cached_test_2;
//-------------------------------------
mod test_m_dao;
mod test_module;
mod test_verify;
#[macro_use]
pub mod polo_dao;
pub mod storages;
mod test_polo_dao;
#[macro_use]
pub mod store_kv_utils;
mod test_store_kv;
