//! error and result module
use core::result::Result as StdResult;

pub use core::fmt;
pub use core::fmt::Write;

pub const MSG_ENUM_LARGE: &str = "enum > u8::MAX";

pub type SerResult<T> = StdResult<T, SerError>;
pub type DeResult<T> = StdResult<T, DeError>;

#[derive(Clone, Debug, PartialEq)]
pub enum SerError {
    /// the write buffer is an invalid size
    Overflow,
    /// enum has too many values
    EnumLarge,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DeError {
    /// the data in the buffer is smaller than the type
    /// (the type is larger than the data available)
    BufferSmall,
    /// the data in the buffer is larger than the type
    BufferLarge,
    /// expected 0 or 1
    ExpectedBoolean,
    /// expected specific value in an Enum
    InvalidVariant,
}

// impl SerError

#[cfg(feature = "std")]
impl ::std::error::Error for SerError {
    fn description(&self) -> &str {
        "TODO: no description yet"
    }
}

impl fmt::Display for SerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ::serde::ser::Error for SerError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        panic!("{}", msg)
    }
}

// impl DeError

#[cfg(feature = "std")]
impl ::std::error::Error for DeError {
    fn description(&self) -> &str {
        "TODO: no description yet"
    }
}


impl fmt::Display for DeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ::serde::de::Error for DeError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        panic!("{}", msg)
    }
}
