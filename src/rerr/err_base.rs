use std::error::Error;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct RErr {
    pub info: Arc<String>,
}

pub fn err<T>(s: T) -> Box<RErr>
where
    T: AsRef<str> + std::fmt::Display,
{
    Box::new(RErr::new(s))
}

impl RErr {
    pub fn new<T>(s: T) -> Self
    where
        T: AsRef<str> + std::fmt::Display,
    {
        RErr {
            info: Arc::new(s.to_string()),
        }
    }

    pub fn tran<T>(e: T) -> Self
    where
        T: std::error::Error,
    {
        RErr {
            info: Arc::new(e.to_string()),
        }
    }
}

impl std::fmt::Display for RErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.info)
    }
}

impl Error for RErr {}

unsafe impl Send for RErr {}
unsafe impl Sync for RErr {}
