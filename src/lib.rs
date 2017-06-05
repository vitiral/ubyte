//! The ubin project is very similar to the bincode library. The intent is to create an ultra
//! simple and lightweight serialization library for microcontrollers serialization library for 
//! microcontrollers and other memory constrained devices.
//! 
//! Similar to bincode, it has the following caveots
//! -   isize/usize are encoded as i64/u64, for portability.    enums variants are encoded as a u32
//! -   instead of a usize. u32 is enough for all practical uses.    str is encoded as (u64,
//! -   &[u8]), where the u64 is the number of bytes contained in the encoded string.


extern crate serde;
extern crate byteorder;

mod error;
mod de;
mod ser;


