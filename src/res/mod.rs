mod api;
mod config;
mod init;

mod test_api;
mod test_init;
mod test_raw;

pub use api::*;
pub use config::*;
pub use init::*;

pub trait SetESID {
    fn set_id(&mut self, id: String);
}
