use std::fmt::Display;

use tobj::LoadError;

#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl Error {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.msg.as_str())
    }
}

impl std::error::Error for Error {}

macro_rules! implement_error {
    ($t:ty, $message:expr) => {
        impl From<$t> for Error {
            fn from(e: $t) -> Self {
                Self {
                    msg: format!(concat!($message, ": {}"), e),
                }
            }
        }
    };
}

implement_error!(std::io::Error, "IO error");
implement_error!(LoadError, "OBJ load error");
