//! serialization module

use dev_prefix::*;
use core::u8;

use byteorder::{ByteOrder, BigEndian};
use serde::ser::{self, Serialize};

pub struct Serializer<'buffer> {
    bytes: &'buffer mut [u8],
}

/// serialize the value in the buffer and return the length
/// of the buffer used.
pub fn to_bytes<T>(bytes: &mut [u8], value: &T) -> SerResult<usize>
    where T: Serialize
{
    let initial = bytes.as_ptr() as usize;
    let mut serializer = Serializer { bytes: bytes };
    value.serialize(&mut serializer)?;
    Ok(serializer.bytes.as_ptr() as usize - initial)
}


macro_rules! impl_value {
    ($ty:ty, $ser_method:ident, $bo_method:ident) => {
        #[inline(always)]
        fn $ser_method(self, value: $ty) -> SerResult<()> {
            self.assert_enough::<$ty>()?;
            BigEndian::$bo_method(&mut self.bytes, value);
            self.consume::<$ty>();
            Ok(())
        }
    }
}

macro_rules! impl_not_supported {
    ($ty:ty, $ser_method:ident) => {
        #[inline(always)]
        fn $ser_method(self, _value: $ty) -> SerResult<()> {
            unimplemented!()
        }
    }
}

impl<'buffer> Serializer<'buffer> {
    /// Consume some of the buffer.
    /// this should NEVER fail (the buffer should always be checked first)
    #[inline(always)]
    fn consume<T>(&mut self) {
        let num = mem::size_of::<T>();
        // FIXME: WHY CAN'T I DO THIS???
        //self.bytes = &mut self.bytes[num..];
        let mut ptr = self.bytes.as_mut_ptr();
        unsafe {
            ptr = ptr.offset(num as isize);
            self.bytes = slice::from_raw_parts_mut(ptr, self.bytes.len() - num);
        }
    }

    /// make sure there is enough buffer left
    #[inline(always)]
    fn assert_enough<T>(&self) -> SerResult<()> {
        if mem::size_of::<T>() > self.bytes.len() {
            Err(SerError::Overflow)
        } else {
            Ok(())
        }
    }

    #[inline(always)]
    fn write_variant(&mut self, index: u32) -> SerResult<()> {
        self.assert_enough::<u8>()?;
        if index > u8::MAX as u32 {
            panic!(MSG_ENUM_LARGE);
        }
        self.bytes[0] = index as u8;
        self.consume::<u8>();
        Ok(())
    }
}

impl<'a, 'buffer: 'a> ser::Serializer for &'a mut Serializer<'buffer> {
    type Ok = (); // outputs data into buffer
    type Error = SerError;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    // bool, i8 and u8 are special case

    #[inline(always)]
    fn serialize_bool(self, v: bool) -> SerResult<()> {
        self.assert_enough::<u8>()?;
        self.bytes[0] = v as u8;
        self.consume::<u8>();
        Ok(())
    }

    #[inline(always)]
    fn serialize_u8(self, v: u8) -> SerResult<()> {
        self.assert_enough::<u8>()?;
        self.bytes[0] = v;
        self.consume::<u8>();
        Ok(())
    }

    #[inline(always)]
    fn serialize_i8(self, v: i8) -> SerResult<()> {
        self.assert_enough::<i8>()?;
        self.bytes[0] = v as u8;
        self.consume::<i8>();
        Ok(())
    }

    // numbers
    impl_value!(u16, serialize_u16, write_u16);
    impl_value!(i16, serialize_i16, write_i16);
    impl_value!(u32, serialize_u32, write_u32);
    impl_value!(i32, serialize_i32, write_i32);
    impl_value!(u64, serialize_u64, write_u64);
    impl_value!(i64, serialize_i64, write_i64);

    impl_value!(f32, serialize_f32, write_f32);
    impl_value!(f64, serialize_f64, write_f64);

    // not supported
    impl_not_supported!(char, serialize_char);
    impl_not_supported!(&str, serialize_str);
    impl_not_supported!(&[u8], serialize_bytes);

    #[inline(always)]
    fn collect_str<T: ?Sized>(self, _value: &T) -> SerResult<()> {
        unimplemented!()
    }

    // enums with values

    #[inline(always)]
    fn serialize_none(self) -> SerResult<()> {
        self.write_variant(0)
    }

    #[inline(always)]
    fn serialize_some<T>(self, value: &T) -> SerResult<()>
        where T: ?Sized + Serialize
    {
        self.write_variant(1)?;
        value.serialize(self)
    }

    #[inline(always)]
    fn serialize_unit(self) -> SerResult<()> {
        Ok(())
    }

    #[inline(always)]
    fn serialize_unit_struct(self, _name: &'static str) -> SerResult<()> {
        Ok(())
    }

    #[inline(always)]
    fn serialize_unit_variant(self,
                              _name: &'static str,
                              variant_index: u32,
                              _variant: &'static str)
                              -> SerResult<()> {
        self.write_variant(variant_index)
    }

    // nested struct
    #[inline(always)]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> SerResult<()>
        where T: ?Sized + Serialize
    {
        value.serialize(self)
    }

    #[inline(always)]
    fn serialize_newtype_variant<T>(self,
                                    _name: &'static str,
                                    variant_index: u32,
                                    _variant: &'static str,
                                    value: &T)
                                    -> SerResult<()>
        where T: ?Sized + Serialize
    {
        // enum with value
        self.write_variant(variant_index)?;
        value.serialize(self)
    }

    // Compound Types: only some supported
    #[inline(always)]
    fn serialize_seq(self, _len: Option<usize>) -> SerResult<Self::SerializeSeq> {
        unimplemented!()
    }

    #[inline(always)]
    fn serialize_tuple(self, _len: usize) -> SerResult<Self::SerializeTuple> {
        Ok(self)
    }

    #[inline(always)]
    fn serialize_tuple_struct(self,
                              _name: &'static str,
                              _len: usize)
                              -> SerResult<Self::SerializeTupleStruct> {
        Ok(self)
    }

    #[inline(always)]
    fn serialize_tuple_variant(self,
                               _name: &'static str,
                               variant_index: u32,
                               _variant: &'static str,
                               _len: usize)
                               -> SerResult<Self::SerializeTupleStruct> {
        self.write_variant(variant_index)?;
        Ok(self)
    }

    #[inline(always)]
    fn serialize_map(self, _len: Option<usize>) -> SerResult<Self::SerializeMap> {
        unimplemented!()
    }

    #[inline(always)]
    fn serialize_struct(self, _name: &'static str, _len: usize) -> SerResult<Self::SerializeStruct> {
        Ok(self)
    }

    #[inline(always)]
    fn serialize_struct_variant(self,
                                _name: &'static str,
                                variant_index: u32,
                                _variant: &'static str,
                                _len: usize)
                                -> SerResult<Self::SerializeStruct> {
        self.write_variant(variant_index)?;
        Ok(self)
    }
}

// Implement Compound Types

macro_rules! impl_field {
    ($trait:path) => {
        impl<'a, 'buffer: 'a> $trait for &'a mut Serializer<'buffer> {
            type Ok = ();
            type Error = SerError;

            fn serialize_field<T>(&mut self, value: &T) -> SerResult<()>
                where T: ?Sized + Serialize
            {
                value.serialize(&mut **self)
            }

            fn end(self) -> SerResult<()> {
                Ok(())
            }
        }
    }
}

macro_rules! impl_key_field {
    ($trait:path) => {
        impl<'a, 'buffer: 'a> $trait for &'a mut Serializer<'buffer> {
            type Ok = ();
            type Error = SerError;

            fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> SerResult<()>
                where T: ?Sized + Serialize
            {
                value.serialize(&mut **self)
            }

            fn end(self) -> SerResult<()> {
                Ok(())
            }
        }
    }
}

impl_field!(ser::SerializeTupleStruct);
impl_field!(ser::SerializeTupleVariant);
impl_key_field!(ser::SerializeStruct);
impl_key_field!(ser::SerializeStructVariant);

impl<'a, 'buffer: 'a> ser::SerializeTuple for &'a mut Serializer<'buffer> {
    type Ok = ();
    type Error = SerError;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> SerResult<()>
        where T: Serialize
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> SerResult<()> {
        Ok(())
    }
}

// seq with unknown length not supported
impl<'a, 'buffer: 'a> ser::SerializeSeq for &'a mut Serializer<'buffer> {
    type Ok = ();
    type Error = SerError;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> SerResult<()>
        where T: Serialize
    {
        unreachable!()
    }

    #[inline]
    fn end(self) -> SerResult<()> {
        unreachable!()
    }
}

// map not supported
impl<'a, 'buffer: 'a> ser::SerializeMap for &'a mut Serializer<'buffer> {
    type Ok = ();
    type Error = SerError;

    fn serialize_key<T>(&mut self, _key: &T) -> SerResult<()>
        where T: ?Sized + Serialize
    {
        unreachable!()
    }

    // It doesn't make a difference whether the colon is printed at the end of
    // `serialize_key` or at the beginning of `serialize_value`. In this case
    // the code is a bit simpler having it here.
    fn serialize_value<T>(&mut self, _value: &T) -> SerResult<()>
        where T: ?Sized + Serialize
    {
        unreachable!()
    }

    fn end(self) -> SerResult<()> {
        unreachable!()
    }
}

#[test]
fn test_ser_enum() {
    #[derive(Serialize)]
    enum E {
        Unit,
        Newtype(u32),
        Tuple(u32, u32),
        Struct { a: u32 },
    }
    let mut buffer: [u8; 100] = [0; 100];

    // Unit
    let expected = [0];
    let len = to_bytes(&mut buffer, &E::Unit).unwrap();
    assert_eq!(len, 1);
    assert_eq!(&expected, &buffer[..len]);

    // Newtype
	let expected = [
        1,                  // variant
        0, 0, 0, 1,         // data
    ];
    let len = to_bytes(&mut buffer, &E::Newtype(1)).unwrap();
    assert_eq!(len, 5);
    assert_eq!(&expected, &buffer[..len]);

    // Tuple
	let expected = [
        2,                  // variant
        0, 0, 0, 0x01,      // index 0
        0, 0, 0, 0x02,      // index 1
    ];
    let len = to_bytes(&mut buffer, &E::Tuple(1, 2)).unwrap();
    assert_eq!(len, 9);
    assert_eq!(&expected, &buffer[..len]);

    // Struct
	let expected = [
        3,                  // variant
        0, 0, 0, 0x01,      // field `a`
    ];
    let len = to_bytes(&mut buffer, &E::Struct{a: 1}).unwrap();
    assert_eq!(len, 5);
    assert_eq!(&expected, &buffer[..len]);
}
