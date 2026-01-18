#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate cached;
#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;
#[allow(unused_imports)]
#[macro_use]
extern crate serde;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;


pub use module_cfg::*;
pub use module_init::*;

//--------macro--------------------------
pub mod audios;
#[macro_use]
pub mod verify;
#[cfg(feature = "rmongo")]
#[macro_use]
pub mod m_dao;

#[cfg(feature = "rmongo")]
#[macro_use]
mod m_dyn_dao;

#[macro_use]
pub mod cache_wrapper;

// #[cfg(feature = "rmy")]
// #[macro_use]
// extern crate rbatis;

//--------pub--module------------------------
pub mod g;
#[cfg(feature = "res")]
pub mod res;
#[cfg(feature = "rlite")]
pub mod rlite;
#[cfg(feature = "rmy")]
pub mod rmy;


pub mod rlog;

#[cfg(feature = "rmongo")]
pub mod rmongo;

#[cfg(feature = "rmq")]
pub mod rmq;
// #[cfg(feature = "rpolo")]
// pub mod rpolo;
// pub mod rr;

// #[cfg(feature = "rmy")]
// pub mod rmy;
// #[cfg(feature = "rlevel")]
// pub mod rlevel;
#[cfg(feature = "rnats")]
pub mod rnats;
#[cfg(feature = "rnatsx")]
pub mod rnatsx;
#[cfg(feature = "rred")]
pub mod rred;
// #[cfg(feature = "rsled")]
// pub mod rsled;
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

// #[cfg(feature = "rpolo")]
// #[macro_use]
// pub mod polo_dao;
pub mod storages;
// mod test_polo_dao;
#[macro_use]
pub mod store_kv_utils;
mod test_store_kv;
mod polo_dao;
