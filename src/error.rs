pub mod io;

use crate::consts::*;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error {
    #[error("{}: {:?}: ", msg::ERR_IO, 0)]
    IoError(#[from] io::Error),
}


impl From<std::ffi::OsString> for Error {
    fn from(oss: std::ffi::OsString) -> Self {
        arg::Error::from(oss).into()
    }
}
