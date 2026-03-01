use lcdhat_sys as sys;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidConfig(&'static str),
    InvalidString,
    AlreadyInitialized,
    UnsupportedPlatform,
    UnknownKey(u32),
    Status {
        status: sys::lcdhat_status_t,
        message: String,
    },
}

pub type Result<T> = core::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidConfig(msg) => write!(f, "invalid config: {msg}"),
            Error::InvalidString => write!(f, "string contains NUL byte"),
            Error::AlreadyInitialized => {
                write!(f, "only one active lcdhat context is supported")
            }
            Error::UnsupportedPlatform => {
                write!(
                    f,
                    "unsupported platform: this crate requires Linux for hardware calls"
                )
            }
            Error::UnknownKey(code) => write!(f, "unknown key code: {code}"),
            Error::Status { status, message } => {
                write!(f, "lcdhat status {status:?}: {message}")
            }
        }
    }
}

impl std::error::Error for Error {}
