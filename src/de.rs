use std::fmt::{self, Display};

use serde;

use any::Any;
use error::Error;

pub fn deserialize<T>(deserializer: &mut Deserializer) -> Result<T, Error>
    where T: serde::Deserialize
{
    serde::Deserialize::deserialize(deserializer)
}

// TRAITS //////////////////////////////////////////////////////////////////////

pub trait Deserializer {
    fn erased_deserialize(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_bool(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_u8(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_u16(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_u32(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_u64(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_i8(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_i16(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_i32(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_i64(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_f32(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_f64(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_char(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_str(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_string(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_bytes(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_byte_buf(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_option(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_unit(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_unit_struct(&mut self, name: &'static str, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_newtype_struct(&mut self, name: &'static str, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_seq(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_seq_fixed_size(&mut self, len: usize, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_tuple(&mut self, len: usize, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_tuple_struct(&mut self, name: &'static str, len: usize, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_map(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_struct(&mut self, name: &'static str, fields: &'static [&'static str], &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_struct_field(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_enum(&mut self, name: &'static str, variants: &'static [&'static str], &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_ignored_any(&mut self, &mut Visitor) -> Result<Out, Error>;
}

pub trait Visitor {
    fn erased_expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result;
    fn erased_visit_bool(&mut self, v: bool) -> Result<Out, Error>;
    fn erased_visit_i8(&mut self, v: i8) -> Result<Out, Error>;
    fn erased_visit_i16(&mut self, v: i16) -> Result<Out, Error>;
    fn erased_visit_i32(&mut self, v: i32) -> Result<Out, Error>;
    fn erased_visit_i64(&mut self, v: i64) -> Result<Out, Error>;
    fn erased_visit_u8(&mut self, v: u8) -> Result<Out, Error>;
    fn erased_visit_u16(&mut self, v: u16) -> Result<Out, Error>;
    fn erased_visit_u32(&mut self, v: u32) -> Result<Out, Error>;
    fn erased_visit_u64(&mut self, v: u64) -> Result<Out, Error>;
    fn erased_visit_f32(&mut self, v: f32) -> Result<Out, Error>;
    fn erased_visit_f64(&mut self, v: f64) -> Result<Out, Error>;
    fn erased_visit_char(&mut self, v: char) -> Result<Out, Error>;
    fn erased_visit_str(&mut self, v: &str) -> Result<Out, Error>;
    fn erased_visit_string(&mut self, v: String) -> Result<Out, Error>;
    fn erased_visit_unit(&mut self) -> Result<Out, Error>;
    fn erased_visit_none(&mut self) -> Result<Out, Error>;
    fn erased_visit_some(&mut self, deserializer: &mut Deserializer) -> Result<Out, Error>;
    fn erased_visit_newtype_struct(&mut self, deserializer: &mut Deserializer) -> Result<Out, Error>;
    fn erased_visit_seq(&mut self, visitor: &mut SeqVisitor) -> Result<Out, Error>;
    fn erased_visit_map(&mut self, visitor: &mut MapVisitor) -> Result<Out, Error>;
    fn erased_visit_bytes(&mut self, v: &[u8]) -> Result<Out, Error>;
    fn erased_visit_byte_buf(&mut self, v: Vec<u8>) -> Result<Out, Error>;
}

pub trait SeqVisitor {
    fn erased_visit(&mut self, seed: &mut DeserializeSeed) -> Result<Option<Out>, Error>;
    fn erased_size_hint(&self) -> (usize, Option<usize>);
}

pub trait MapVisitor {
    fn erased_visit_key(&mut self, seed: &mut DeserializeSeed) -> Result<Option<Out>, Error>;
    fn erased_visit_value(&mut self, seed: &mut DeserializeSeed) -> Result<Out, Error>;
    fn erased_visit(&mut self, k: &mut DeserializeSeed, v: &mut DeserializeSeed) -> Result<Option<(Out, Out)>, Error>;
    fn erased_size_hint(&self) -> (usize, Option<usize>);
}

pub trait EnumVisitor {
    fn erased_visit_variant(&mut self, seed: &mut DeserializeSeed) -> Result<(Out, Variant), Error>;
}

pub struct Variant {
    data: Any,
    visit_unit: fn(Any) -> Result<(), Error>,
    visit_newtype: fn(Any, seed: &mut DeserializeSeed) -> Result<Out, Error>,
    visit_tuple: fn(Any, len: usize, visitor: &mut Visitor) -> Result<Out, Error>,
    visit_struct: fn(Any, fields: &'static [&'static str], visitor: &mut Visitor) -> Result<Out, Error>,
}

pub trait DeserializeSeed {
    fn erased_deserialize_seed(&mut self, deserializer: &mut Deserializer) -> Result<Out, Error>;
}

impl<T> DeserializeSeed for erase::DeserializeSeed<T> where T: serde::de::DeserializeSeed {
    fn erased_deserialize_seed(&mut self, deserializer: &mut Deserializer) -> Result<Out, Error> {
        self.take().deserialize(deserializer).map(Out::new)
    }
}

impl<'a> serde::de::DeserializeSeed for &'a mut DeserializeSeed {
    type Value = Out;
    fn deserialize<D>(self, deserializer: D) -> Result<Out, D::Error>
        where D: serde::Deserializer
    {
        let mut erased = erase::Deserializer { state: Some(deserializer) };
        self.erased_deserialize_seed(&mut erased).map_err(unerase)
    }
}

impl serde::de::VariantVisitor for Variant {
    type Error = Error;
    fn visit_unit(self) -> Result<(), Error> {
        (self.visit_unit)(self.data)
    }
    fn visit_newtype_seed<T>(self, seed: T) -> Result<T::Value, Error>
        where T: serde::de::DeserializeSeed
    {
        let mut erased = erase::DeserializeSeed { state: Some(seed) };
        (self.visit_newtype)(self.data, &mut erased).map(Out::take)
    }
    fn visit_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Error>
        where V: serde::de::Visitor
    {
        let mut erased = erase::Visitor { state: Some(visitor) };
        (self.visit_tuple)(self.data, len, &mut erased).map(Out::take)
    }
    fn visit_struct<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value, Error>
        where V: serde::de::Visitor
    {
        let mut erased = erase::Visitor { state: Some(visitor) };
        (self.visit_struct)(self.data, fields, &mut erased).map(Out::take)
    }
}

// OUT /////////////////////////////////////////////////////////////////////////

pub struct Out(Any);

impl Out {
    fn new<T>(t: T) -> Self {
        Out(Any::new(t))
    }

    fn take<T>(self) -> T {
        self.0.take()
    }
}

// Required because Out is used as the Value type of the Visitor.
impl serde::Deserialize for Out {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer
    {
        // I don't think this is called...?
        // Please file a ticket if it gets called.
        unimplemented!()
    }
}

// IMPL SERDE FOR ERASED SERDE /////////////////////////////////////////////////

macro_rules! impl_deserializer_for_trait_object {
    ({$($generics:tt)*} $ty:ty) => {
        impl <$($generics)*> serde::Deserializer for $ty {
            type Error = Error;
            fn deserialize<V>(mut self, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize(&mut erased).map(Out::take)
            }
            fn deserialize_bool<V>(mut self, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_bool(&mut erased).map(Out::take)
            }
            fn deserialize_u8<V>(mut self, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_u8(&mut erased).map(Out::take)
            }
            fn deserialize_u16<V>(mut self, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_u16(&mut erased).map(Out::take)
            }
            fn deserialize_u32<V>(mut self, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_u32(&mut erased).map(Out::take)
            }
            fn deserialize_u64<V>(mut self, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_u64(&mut erased).map(Out::take)
            }
            fn deserialize_i8<V>(mut self, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_i8(&mut erased).map(Out::take)
            }
            fn deserialize_i16<V>(mut self, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_i16(&mut erased).map(Out::take)
            }
            fn deserialize_i32<V>(mut self, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_i32(&mut erased).map(Out::take)
            }
            fn deserialize_i64<V>(mut self, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_i64(&mut erased).map(Out::take)
            }
            fn deserialize_f32<V>(mut self, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_f32(&mut erased).map(Out::take)
            }
            fn deserialize_f64<V>(mut self, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_f64(&mut erased).map(Out::take)
            }
            fn deserialize_char<V>(mut self, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_char(&mut erased).map(Out::take)
            }
            fn deserialize_str<V>(mut self, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_str(&mut erased).map(Out::take)
            }
            fn deserialize_string<V>(mut self, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_string(&mut erased).map(Out::take)
            }
            fn deserialize_bytes<V>(mut self, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_bytes(&mut erased).map(Out::take)
            }
            fn deserialize_byte_buf<V>(mut self, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_byte_buf(&mut erased).map(Out::take)
            }
            fn deserialize_option<V>(mut self, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_option(&mut erased).map(Out::take)
            }
            fn deserialize_unit<V>(mut self, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_unit(&mut erased).map(Out::take)
            }
            fn deserialize_unit_struct<V>(mut self, name: &'static str, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_unit_struct(name, &mut erased).map(Out::take)
            }
            fn deserialize_newtype_struct<V>(mut self, name: &'static str, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_newtype_struct(name, &mut erased).map(Out::take)
            }
            fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_seq(&mut erased).map(Out::take)
            }
            fn deserialize_seq_fixed_size<V>(mut self, len: usize, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_seq_fixed_size(len, &mut erased).map(Out::take)
            }
            fn deserialize_tuple<V>(mut self, len: usize, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_tuple(len, &mut erased).map(Out::take)
            }
            fn deserialize_tuple_struct<V>(mut self, name: &'static str, len: usize, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_tuple_struct(name, len, &mut erased).map(Out::take)
            }
            fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_map(&mut erased).map(Out::take)
            }
            fn deserialize_struct<V>(mut self, name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_struct(name, fields, &mut erased).map(Out::take)
            }
            fn deserialize_struct_field<V>(mut self, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_struct_field(&mut erased).map(Out::take)
            }
            fn deserialize_enum<V>(mut self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_enum(name, variants, &mut erased).map(Out::take)
            }
            fn deserialize_ignored_any<V>(mut self, visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                let mut erased = erase::Visitor { state: Some(visitor) };
                self.erased_deserialize_ignored_any(&mut erased).map(Out::take)
            }
        }
    };
}

impl_deserializer_for_trait_object!({'a} &'a mut Deserializer);
impl_deserializer_for_trait_object!({'a} &'a mut (Deserializer + Send));
impl_deserializer_for_trait_object!({'a} &'a mut (Deserializer + Sync));
impl_deserializer_for_trait_object!({'a} &'a mut (Deserializer + Send + Sync));
impl_deserializer_for_trait_object!({} Box<Deserializer>);
impl_deserializer_for_trait_object!({} Box<Deserializer + Send>);
impl_deserializer_for_trait_object!({} Box<Deserializer + Sync>);
impl_deserializer_for_trait_object!({} Box<Deserializer + Send + Sync>);

impl<'a> serde::de::Visitor for &'a mut Visitor {
    type Value = Out;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        (**self).erased_expecting(formatter)
    }
    fn visit_bool<E>(self, v: bool) -> Result<Out, E> where E: serde::de::Error {
        self.erased_visit_bool(v).map_err(unerase)
    }
    fn visit_i8<E>(self, v: i8) -> Result<Out, E> where E: serde::de::Error {
        self.erased_visit_i8(v).map_err(unerase)
    }
    fn visit_i16<E>(self, v: i16) -> Result<Out, E> where E: serde::de::Error {
        self.erased_visit_i16(v).map_err(unerase)
    }
    fn visit_i32<E>(self, v: i32) -> Result<Out, E> where E: serde::de::Error {
        self.erased_visit_i32(v).map_err(unerase)
    }
    fn visit_i64<E>(self, v: i64) -> Result<Out, E> where E: serde::de::Error {
        self.erased_visit_i64(v).map_err(unerase)
    }
    fn visit_u8<E>(self, v: u8) -> Result<Out, E> where E: serde::de::Error {
        self.erased_visit_u8(v).map_err(unerase)
    }
    fn visit_u16<E>(self, v: u16) -> Result<Out, E> where E: serde::de::Error {
        self.erased_visit_u16(v).map_err(unerase)
    }
    fn visit_u32<E>(self, v: u32) -> Result<Out, E> where E: serde::de::Error {
        self.erased_visit_u32(v).map_err(unerase)
    }
    fn visit_u64<E>(self, v: u64) -> Result<Out, E> where E: serde::de::Error {
        self.erased_visit_u64(v).map_err(unerase)
    }
    fn visit_f32<E>(self, v: f32) -> Result<Out, E> where E: serde::de::Error {
        self.erased_visit_f32(v).map_err(unerase)
    }
    fn visit_f64<E>(self, v: f64) -> Result<Out, E> where E: serde::de::Error {
        self.erased_visit_f64(v).map_err(unerase)
    }
    fn visit_char<E>(self, v: char) -> Result<Out, E> where E: serde::de::Error {
        self.erased_visit_char(v).map_err(unerase)
    }
    fn visit_str<E>(self, v: &str) -> Result<Out, E> where E: serde::de::Error {
        self.erased_visit_str(v).map_err(unerase)
    }
    fn visit_string<E>(self, v: String) -> Result<Out, E> where E: serde::de::Error {
        self.erased_visit_string(v).map_err(unerase)
    }
    fn visit_unit<E>(self) -> Result<Out, E> where E: serde::de::Error {
        self.erased_visit_unit().map_err(unerase)
    }
    fn visit_none<E>(self) -> Result<Out, E> where E: serde::de::Error {
        self.erased_visit_none().map_err(unerase)
    }
    fn visit_some<D>(self, deserializer: D) -> Result<Out, D::Error> where D: serde::Deserializer {
        let mut erased = erase::Deserializer {
            state: Some(deserializer),
        };
        self.erased_visit_some(&mut erased).map_err(unerase)
    }
    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Out, D::Error> where D: serde::Deserializer {
        let mut erased = erase::Deserializer {
            state: Some(deserializer),
        };
        self.erased_visit_newtype_struct(&mut erased).map_err(unerase)
    }
    fn visit_seq<V>(self, mut visitor: V) -> Result<Out, V::Error> where V: serde::de::SeqVisitor {
        self.erased_visit_seq(&mut visitor).map_err(unerase)
    }
    fn visit_map<V>(self, mut visitor: V) -> Result<Out, V::Error> where V: serde::de::MapVisitor {
        self.erased_visit_map(&mut visitor).map_err(unerase)
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Out, E> where E: serde::de::Error {
        self.erased_visit_bytes(v).map_err(unerase)
    }
    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Out, E> where E: serde::de::Error {
        self.erased_visit_byte_buf(v).map_err(unerase)
    }
}

impl<'a> serde::de::SeqVisitor for &'a mut SeqVisitor {
    type Error = Error;
    fn visit_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error> where T: serde::de::DeserializeSeed {
        let mut seed = erase::DeserializeSeed { state: Some(seed) };
        (**self).erased_visit(&mut seed).map(|opt| opt.map(Out::take))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (**self).erased_size_hint()
    }
}

impl<'a> serde::de::MapVisitor for &'a mut MapVisitor {
    type Error = Error;
    fn visit_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Error> where K: serde::de::DeserializeSeed {
        let mut erased = erase::DeserializeSeed { state: Some(seed) };
        (**self).erased_visit_key(&mut erased).map(|opt| opt.map(Out::take))
    }
    fn visit_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Error> where V: serde::de::DeserializeSeed {
        let mut erased = erase::DeserializeSeed { state: Some(seed) };
        (**self).erased_visit_value(&mut erased).map(Out::take)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (**self).erased_size_hint()
    }
}

impl<'a> serde::de::EnumVisitor for &'a mut EnumVisitor {
    type Error = Error;
    type Variant = Variant;
    fn visit_variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error> where V: serde::de::DeserializeSeed {
        let mut erased = erase::DeserializeSeed { state: Some(seed) };
        self.erased_visit_variant(&mut erased).map(|(out, variant)| (out.take(), variant))
    }
}

// IMPL ERASED SERDE FOR SERDE /////////////////////////////////////////////////

mod erase {
    pub struct Deserializer<D> {
        pub(super) state: Option<D>,
    }

    impl<D> Deserializer<D> {
        pub(super) fn take(&mut self) -> D {
            self.state.take().unwrap()
        }
    }

    pub struct DeserializeSeed<D> {
        pub(super) state: Option<D>,
    }

    impl<D> DeserializeSeed<D> {
        pub(super) fn take(&mut self) -> D {
            self.state.take().unwrap()
        }
    }

    pub struct Visitor<D> {
        pub(super) state: Option<D>,
    }

    impl<D> Visitor<D> {
        pub(super) fn take(&mut self) -> D {
            self.state.take().unwrap()
        }

        pub(super) fn as_ref(&self) -> &D {
            self.state.as_ref().unwrap()
        }
    }

    pub struct EnumVisitor<D> {
        pub(super) state: Option<D>,
    }

    impl<D> EnumVisitor<D> {
        pub(super) fn take(&mut self) -> D {
            self.state.take().unwrap()
        }
    }
}

impl Deserializer {
    pub fn erase<D>(deserializer: D) -> erase::Deserializer<D>
        where D: serde::Deserializer
    {
        erase::Deserializer {
            state: Some(deserializer),
        }
    }
}

impl<T: ?Sized> Deserializer for erase::Deserializer<T> where T: serde::Deserializer {
    fn erased_deserialize(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize(visitor).map_err(erase)
    }
    fn erased_deserialize_bool(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_bool(visitor).map_err(erase)
    }
    fn erased_deserialize_u8(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_u8(visitor).map_err(erase)
    }
    fn erased_deserialize_u16(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_u16(visitor).map_err(erase)
    }
    fn erased_deserialize_u32(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_u32(visitor).map_err(erase)
    }
    fn erased_deserialize_u64(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_u64(visitor).map_err(erase)
    }
    fn erased_deserialize_i8(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_i8(visitor).map_err(erase)
    }
    fn erased_deserialize_i16(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_u16(visitor).map_err(erase)
    }
    fn erased_deserialize_i32(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_i32(visitor).map_err(erase)
    }
    fn erased_deserialize_i64(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_i64(visitor).map_err(erase)
    }
    fn erased_deserialize_f32(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_f32(visitor).map_err(erase)
    }
    fn erased_deserialize_f64(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_f64(visitor).map_err(erase)
    }
    fn erased_deserialize_char(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_char(visitor).map_err(erase)
    }
    fn erased_deserialize_str(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_str(visitor).map_err(erase)
    }
    fn erased_deserialize_string(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_string(visitor).map_err(erase)
    }
    fn erased_deserialize_bytes(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_bytes(visitor).map_err(erase)
    }
    fn erased_deserialize_byte_buf(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_byte_buf(visitor).map_err(erase)
    }
    fn erased_deserialize_option(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_option(visitor).map_err(erase)
    }
    fn erased_deserialize_unit(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_unit(visitor).map_err(erase)
    }
    fn erased_deserialize_unit_struct(&mut self, name: &'static str, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_unit_struct(name, visitor).map_err(erase)
    }
    fn erased_deserialize_newtype_struct(&mut self, name: &'static str, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_newtype_struct(name, visitor).map_err(erase)
    }
    fn erased_deserialize_seq(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_seq(visitor).map_err(erase)
    }
    fn erased_deserialize_seq_fixed_size(&mut self, len: usize, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_seq_fixed_size(len, visitor).map_err(erase)
    }
    fn erased_deserialize_tuple(&mut self, len: usize, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_tuple(len, visitor).map_err(erase)
    }
    fn erased_deserialize_tuple_struct(&mut self, name: &'static str, len: usize, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_tuple_struct(name, len, visitor).map_err(erase)
    }
    fn erased_deserialize_map(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_map(visitor).map_err(erase)
    }
    fn erased_deserialize_struct(&mut self, name: &'static str, fields: &'static [&'static str], visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_struct(name, fields, visitor).map_err(erase)
    }
    fn erased_deserialize_struct_field(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_struct_field(visitor).map_err(erase)
    }
    fn erased_deserialize_enum(&mut self, name: &'static str, variants: &'static [&'static str], visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_enum(name, variants, visitor).map_err(erase)
    }
    fn erased_deserialize_ignored_any(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.take().deserialize_ignored_any(visitor).map_err(erase)
    }
}

impl<T: ?Sized> Visitor for erase::Visitor<T> where T: serde::de::Visitor {
    fn erased_expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.as_ref().expecting(formatter)
    }
    fn erased_visit_bool(&mut self, v: bool) -> Result<Out, Error> {
        self.take().visit_bool(v).map(Out::new)
    }
    fn erased_visit_i8(&mut self, v: i8) -> Result<Out, Error> {
        self.take().visit_i8(v).map(Out::new)
    }
    fn erased_visit_i16(&mut self, v: i16) -> Result<Out, Error> {
        self.take().visit_i16(v).map(Out::new)
    }
    fn erased_visit_i32(&mut self, v: i32) -> Result<Out, Error> {
        self.take().visit_i32(v).map(Out::new)
    }
    fn erased_visit_i64(&mut self, v: i64) -> Result<Out, Error> {
        self.take().visit_i64(v).map(Out::new)
    }
    fn erased_visit_u8(&mut self, v: u8) -> Result<Out, Error> {
        self.take().visit_u8(v).map(Out::new)
    }
    fn erased_visit_u16(&mut self, v: u16) -> Result<Out, Error> {
        self.take().visit_u16(v).map(Out::new)
    }
    fn erased_visit_u32(&mut self, v: u32) -> Result<Out, Error> {
        self.take().visit_u32(v).map(Out::new)
    }
    fn erased_visit_u64(&mut self, v: u64) -> Result<Out, Error> {
        self.take().visit_u64(v).map(Out::new)
    }
    fn erased_visit_f32(&mut self, v: f32) -> Result<Out, Error> {
        self.take().visit_f32(v).map(Out::new)
    }
    fn erased_visit_f64(&mut self, v: f64) -> Result<Out, Error> {
        self.take().visit_f64(v).map(Out::new)
    }
    fn erased_visit_char(&mut self, v: char) -> Result<Out, Error> {
        self.take().visit_char(v).map(Out::new)
    }
    fn erased_visit_str(&mut self, v: &str) -> Result<Out, Error> {
        self.take().visit_str(v).map(Out::new)
    }
    fn erased_visit_string(&mut self, v: String) -> Result<Out, Error> {
        self.take().visit_string(v).map(Out::new)
    }
    fn erased_visit_unit(&mut self) -> Result<Out, Error> {
        self.take().visit_unit().map(Out::new)
    }
    fn erased_visit_none(&mut self) -> Result<Out, Error> {
        self.take().visit_none().map(Out::new)
    }
    fn erased_visit_some(&mut self, deserializer: &mut Deserializer) -> Result<Out, Error> {
        self.take().visit_some(deserializer).map(Out::new)
    }
    fn erased_visit_newtype_struct(&mut self, deserializer: &mut Deserializer) -> Result<Out, Error> {
        self.take().visit_newtype_struct(deserializer).map(Out::new)
    }
    fn erased_visit_seq(&mut self, visitor: &mut SeqVisitor) -> Result<Out, Error> {
        self.take().visit_seq(visitor).map(Out::new)
    }
    fn erased_visit_map(&mut self, visitor: &mut MapVisitor) -> Result<Out, Error> {
        self.take().visit_map(visitor).map(Out::new)
    }
    fn erased_visit_bytes(&mut self, v: &[u8]) -> Result<Out, Error> {
        self.take().visit_bytes(v).map(Out::new)
    }
    fn erased_visit_byte_buf(&mut self, v: Vec<u8>) -> Result<Out, Error> {
        self.take().visit_byte_buf(v).map(Out::new)
    }
}

impl<T: ?Sized> SeqVisitor for T where T: serde::de::SeqVisitor {
    fn erased_visit(&mut self, seed: &mut DeserializeSeed) -> Result<Option<Out>, Error> {
        self.visit_seed(seed).map_err(erase)
    }
    fn erased_size_hint(&self) -> (usize, Option<usize>) {
        self.size_hint()
    }
}

impl<T: ?Sized> MapVisitor for T where T: serde::de::MapVisitor {
    fn erased_visit_key(&mut self, seed: &mut DeserializeSeed) -> Result<Option<Out>, Error> {
        self.visit_key_seed(seed).map_err(erase)
    }
    fn erased_visit_value(&mut self, seed: &mut DeserializeSeed) -> Result<Out, Error> {
        self.visit_value_seed(seed).map_err(erase)
    }
    fn erased_visit(&mut self, k: &mut DeserializeSeed, v: &mut DeserializeSeed) -> Result<Option<(Out, Out)>, Error> {
        self.visit_seed(k, v).map_err(erase)
    }
    fn erased_size_hint(&self) -> (usize, Option<usize>) {
        self.size_hint()
    }
}

impl<T: ?Sized> EnumVisitor for erase::EnumVisitor<T> where T: serde::de::EnumVisitor {
    fn erased_visit_variant(&mut self, seed: &mut DeserializeSeed) -> Result<(Out, Variant), Error> {
        self.take().visit_variant_seed(seed).map(|(out, variant)| {
            use serde::de::VariantVisitor;
            let erased = Variant {
                data: Any::new(variant),
                visit_unit: |a| {
                    a.take::<T::Variant>().visit_unit().map_err(erase)
                },
                visit_newtype: |a, seed| {
                    a.take::<T::Variant>().visit_newtype_seed(seed).map_err(erase)
                },
                visit_tuple: |a, len, visitor| {
                    a.take::<T::Variant>().visit_tuple(len, visitor).map_err(erase)
                },
                visit_struct: |a, fields, visitor| {
                    a.take::<T::Variant>().visit_struct(fields, visitor).map_err(erase)
                },
            };
            (Out::new(out), erased)
        }).map_err(erase)
    }
}

// ERROR ///////////////////////////////////////////////////////////////////////

fn erase<E>(e: E) -> Error
    where E: Display
{
    serde::de::Error::custom(e)
}

fn unerase<E>(e: Error) -> E
    where E: serde::de::Error
{
    use std::error::Error;
    E::custom(e.description())
}

// TEST ////////////////////////////////////////////////////////////////////////

#[test]
fn trait_object() {
    extern crate serde_json;

    let json = br#"["a", 1, [true], {"a": 1}]"#;
    let expected: serde_json::Value = serde_json::from_slice(json).unwrap();

    let mut de = serde_json::Deserializer::from_slice(json);
    let de: &mut Deserializer = &mut Deserializer::erase(&mut de);
    assert_eq!(expected, deserialize::<serde_json::Value>(de).unwrap());
}

#[test]
fn assert_deserializer() {
    fn assert<T: serde::Deserializer>() {}

    assert::<&mut Deserializer>();
    assert::<&mut (Deserializer + Send)>();
    assert::<&mut (Deserializer + Sync)>();
    assert::<&mut (Deserializer + Send + Sync)>();
    assert::<&mut (Deserializer + Sync + Send)>();

    assert::<Box<Deserializer>>();
    assert::<Box<Deserializer + Send>>();
    assert::<Box<Deserializer + Sync>>();
    assert::<Box<Deserializer + Send + Sync>>();
    assert::<Box<Deserializer + Sync + Send>>();
}
