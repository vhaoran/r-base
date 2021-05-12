#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

// #[macro_use]
// extern crate ormlite;
// #[macro_use]
// extern crate sqlx;

//----pub fn/struct--------------------------
pub use self::rerr::*;

pub use module_cfg::*;
pub use module_init::*;

//--------macro--------------------------
pub mod audios;
#[macro_use]
pub mod verify;
#[macro_use]
pub mod m_dao;
#[macro_use]
pub mod cache_wrapper;
#[macro_use]
extern crate cached;

#[macro_use]
extern crate serde_json;

// #[cfg(feature = "rmy")]
// #[macro_use]
// extern crate rbatis;

//--------pub--module------------------------
pub mod g;
pub mod rerr;
pub mod res;
pub mod rlog;
pub mod rmongo;
pub mod rmq;

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
