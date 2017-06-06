//! deserialization module
 
use dev_prefix::*;
use core::u8;

use byteorder::{ByteOrder, BigEndian};
use serde::de::{self, Deserialize, DeserializeSeed, Visitor, SeqAccess, EnumAccess,
                VariantAccess, IntoDeserializer};

pub struct Deserializer<'de> {
    // Starts with the input data and characters are truncated off
    // the beginning as data is parsed.
    input: &'de [u8],
}


impl<'de> Deserializer<'de> {
	/// Create a deserializer from a byte array
    pub fn from_bytes(input: &'de [u8]) -> Self {
        Deserializer { input: input }
    }
}


pub fn from_bytes<'de, T>(bytes: &'de [u8]) -> DeResult<T>
    where T: Deserialize<'de>
{
    let mut deserializer = Deserializer::from_bytes(bytes);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(DeError::BufferLarge)
    }
}

impl <'de> Deserializer<'de> {
    /// make sure there is enough buffer left
    #[inline(always)]
    fn assert_enough<T>(&self) -> DeResult<()> {
        if mem::size_of::<T>() <= self.input.len() {
            Ok(())
        } else {
            Err(DeError::BufferSmall)
        }
    }

    #[inline(always)]
    fn consume_u8(&mut self) -> DeResult<u8>{
        self.assert_enough::<u8>()?;
        let v = self.input[0];
        self.consume::<u8>();
        Ok(v)
    }

    #[inline(always)]
    fn consume_bool(&mut self) -> DeResult<bool>{
        Ok(match self.consume_u8()? {
            0 => false,
            1 => true,
            _ => return Err(DeError::ExpectedBoolean),
        })
    }

    #[inline(always)]
    fn consume<T>(&mut self) {
        let num = mem::size_of::<T>();
        self.input = &self.input[num..];
    }
}

macro_rules! impl_value {
    ($ty:ty, $de_method:ident, $bo_method:ident, $visitor_method:ident) => {
        #[inline(always)]
        fn $de_method<V>(self, visitor: V) -> DeResult<V::Value>
            where V: Visitor<'de>,
        {
            self.assert_enough::<$ty>()?;
            let v = BigEndian::$bo_method(self.input);
            self.consume::<$ty>();
            visitor.$visitor_method(v)
        }
    }
}

macro_rules! not_impl {
    ($de_method:ident) => {
        #[inline(always)]
        fn $de_method<V>(self, _visitor: V) -> DeResult<V::Value>
            where V: Visitor<'de>,
        {
            unimplemented!();
        }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = DeError;

    #[inline(always)]
    fn deserialize_bool<V>(self, visitor: V) -> DeResult<V::Value>
        where V: Visitor<'de>
    {
        visitor.visit_bool(self.consume_bool()?)
    }

    #[inline(always)]
    fn deserialize_u8<V>(self, visitor: V) -> DeResult<V::Value>
        where V: Visitor<'de>
    {
        visitor.visit_u8(self.consume_u8()?)
    }

    #[inline(always)]
    fn deserialize_i8<V>(self, visitor: V) -> DeResult<V::Value>
        where V: Visitor<'de>
    {
        visitor.visit_i8(self.consume_u8()? as i8)
    }

    impl_value!(u16, deserialize_u16, read_u16, visit_u16);
    impl_value!(i16, deserialize_i16, read_i16, visit_i16);
    impl_value!(u32, deserialize_u32, read_u32, visit_u32);
    impl_value!(i32, deserialize_i32, read_i32, visit_i32);
    impl_value!(u64, deserialize_u64, read_u64, visit_u64);
    impl_value!(i64, deserialize_i64, read_i64, visit_i64);
    impl_value!(f32, deserialize_f32, read_f32, visit_f32);
    impl_value!(f64, deserialize_f64, read_f64, visit_f64);

    #[inline(always)]
    fn deserialize_option<V>(self, visitor: V) -> DeResult<V::Value>
        where V: Visitor<'de>
    {
        if self.consume_bool()? {
            visitor.visit_some(self)
        } else {
            visitor.visit_none()
        }
    }

    #[inline(always)]
    fn deserialize_unit<V>(self, visitor: V) -> DeResult<V::Value>
        where V: Visitor<'de>
    {
        visitor.visit_unit()
    }

    #[inline(always)]
    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V
    ) -> DeResult<V::Value>
        where V: Visitor<'de>
    {
        self.deserialize_unit(visitor)
    }

    #[inline(always)]
    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V
    ) -> DeResult<V::Value>
        where V: Visitor<'de>
    {
        visitor.visit_newtype_struct(self)
    }

    #[inline(always)]
    fn deserialize_tuple<V>(
        self,
        len: usize,
        visitor: V
    ) -> DeResult<V::Value>
        where V: Visitor<'de>
    {
        visitor.visit_seq(Tuple { deserializer: self, len: len })
    }

    #[inline(always)]
    fn deserialize_struct<V>(self,
                       _name: &str,
                       fields: &'static [&'static str],
                       visitor: V) -> DeResult<V::Value>
        where V: Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    #[inline(always)]
    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        variants: &'static [&'static str],
        visitor: V
    ) -> DeResult<V::Value>
        where V: Visitor<'de>
    {
        if variants.len() > u8::MAX as usize {
            panic!(MSG_ENUM_LARGE);
        }
        if self.input[0] as usize >= variants.len() {
            return Err(DeError::InvalidVariant);
        }
        visitor.visit_enum(self)
    }

    #[inline(always)]
	fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V
    ) -> DeResult<V::Value>
        where V: Visitor<'de>
    {
        self.deserialize_tuple(len, visitor)
    }

    // not supported
    not_impl!(deserialize_identifier);
    not_impl!(deserialize_any);
    not_impl!(deserialize_char);
    not_impl!(deserialize_str);
    not_impl!(deserialize_string);
    not_impl!(deserialize_bytes);
    not_impl!(deserialize_byte_buf);
    not_impl!(deserialize_seq);
    not_impl!(deserialize_map);
    not_impl!(deserialize_ignored_any);
}

struct Tuple<'a, 'de: 'a> {
    deserializer: &'a mut Deserializer<'de>,
    len: usize,
}

impl<'a, 'de> SeqAccess<'de> for Tuple<'a, 'de> { 
    type Error = DeError;

    fn next_element_seed<T>(&mut self, seed: T) -> DeResult<Option<T::Value>>
        where T: DeserializeSeed<'de>
    {
        if self.len > 0 {
            self.len -= 1;
            let value = DeserializeSeed::deserialize(seed, &mut *self.deserializer)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }

    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

impl<'de, 'a> EnumAccess<'de> for &'a mut Deserializer<'de> {
    type Error = DeError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> DeResult<(V::Value, Self::Variant)>
        where V: DeserializeSeed<'de>
    {
        let index = self.consume_u8()?;
        let val = seed.deserialize((index as u32).into_deserializer())?;
        Ok((val, self))
    }
}

impl<'de, 'a> VariantAccess<'de> for &'a mut Deserializer<'de> {
    type Error = DeError;

    fn unit_variant(self) -> DeResult<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> DeResult<T::Value>
        where T: DeserializeSeed<'de>
    {
        DeserializeSeed::deserialize(seed, self)
    }

    fn tuple_variant<V>(self,
                      len: usize,
                      visitor: V) -> DeResult<V::Value>
        where V: Visitor<'de>,
    {
        de::Deserializer::deserialize_tuple(self, len, visitor)
    }

    fn struct_variant<V>(self,
                       fields: &'static [&'static str],
                       visitor: V) -> DeResult<V::Value>
        where V: Visitor<'de>,
    {
        de::Deserializer::deserialize_tuple(self, fields.len(), visitor)
    }
}

#[test]
fn test_de_enum() {
    #[derive(Debug, PartialEq, Eq, Deserialize)]
    enum E {
        Unit,
        Newtype(u32),
        Tuple(u32, u32),
        Struct { a: u32 },
    }

    // Invalid
    let buffer = [42];
    assert_eq!(from_bytes::<E>(&buffer).unwrap_err(), DeError::InvalidVariant);

    // Unit
    let buffer = [0];
    let v: E = from_bytes(&buffer).unwrap();
    assert_eq!(v, E::Unit);

    // Newtype
    let buffer = [
        1,                  // variant
        0, 0, 0, 1,         // data
    ];
    let v: E = from_bytes(&buffer).unwrap();
    assert_eq!(v, E::Newtype(1));

    // Tuple
    let buffer = [
        2,                  // variant
        0, 0, 0, 0x01,      // index 0
        0, 0, 0, 0x02,      // index 1
    ];
    let v: E = from_bytes(&buffer).unwrap();
    assert_eq!(v, E::Tuple(1, 2));

    // Struct
    let buffer = [
        3,                  // variant
        0, 0, 0, 0x01,      // field `a`
    ];
    let v: E = from_bytes(&buffer).unwrap();
    assert_eq!(v, E::Struct{a: 1});
}
