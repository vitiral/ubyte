//! The ubin project is very similar to the bincode library. The intent is to create an ultra
//! simple and lightweight serialization library for microcontrollers serialization library for
//! microcontrollers and other memory constrained devices.
//!
//! Similar to bincode, it has the following caveots
//! -   isize/usize are encoded as i64/u64, for portability.    enums variants are encoded as a u32
//! -   instead of a usize. u32 is enough for all practical uses.    str is encoded as (u64,
//! -   &[u8]), where the u64 is the number of bytes contained in the encoded string.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;

extern crate serde;
extern crate byteorder;

#[cfg(test)]
#[macro_use]
extern crate serde_derive;

mod dev_prefix;
mod error;
pub mod de;
pub mod ser;

pub use ser::{to_bytes, Serializer};
pub use de::{from_bytes, Deserializer};
