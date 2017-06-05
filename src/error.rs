//! error and result module


use core::result::Result as StdResult;
use core::error::Error as StdError;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// the write buffer is an invalid size
    Overflow,
    /// the data in the buffer is smaller than the type
    TypeLarge,
    /// the data in the buffer is larger than the type
    TypeSmall,
    /// expected 0 or 1
    ExpectedBoolean,
    /// expected specific values in an Enum
    ExpectedEnum,
}



impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Overflow => "buffer too small",
            Error::TypeLarge => "type too large",
            Error::TypeSmall => "type too small",
            Error::ExpectedBoolean => "expected boolean",
            Error::ExpectedEnum => "expected enum value",
        }
    }
}
