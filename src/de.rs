use std::cell::RefCell;
use std::fmt::Display;
use std::mem;

use serde;

use any::Any;
use error::Error;

pub fn deserialize<T>(mut deserializer: &mut Deserializer) -> Result<T, Error>
    where T: serde::Deserialize
{
    // This reference-to-a-reference is because Deserialize::deserialize<D>
    // requires D: Sized (for no reason).
    serde::Deserialize::deserialize(&mut deserializer)
}

// TRAITS //////////////////////////////////////////////////////////////////////

pub trait Deserializer {
    fn erased_deserialize(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_bool(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_usize(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_u8(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_u16(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_u32(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_u64(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_isize(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_i8(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_i16(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_i32(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_i64(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_f32(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_f64(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_char(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_str(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_string(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_unit(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_option(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_seq(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_seq_fixed_size(&mut self, len: usize, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_bytes(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_map(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_unit_struct(&mut self, name: &'static str, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_newtype_struct(&mut self, name: &'static str, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_tuple_struct(&mut self, name: &'static str, len: usize, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_struct(&mut self, name: &'static str, fields: &'static [&'static str], &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_struct_field(&mut self, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_tuple(&mut self, len: usize, &mut Visitor) -> Result<Out, Error>;
    fn erased_deserialize_enum(&mut self, name: &'static str, variants: &'static [&'static str], &mut EnumVisitor) -> Result<Out, Error>;
    fn erased_deserialize_ignored_any(&mut self, &mut Visitor) -> Result<Out, Error>;
}

pub trait Visitor {
    fn erased_visit_bool(&mut self, v: bool) -> Result<Out, Error>;
    fn erased_visit_isize(&mut self, v: isize) -> Result<Out, Error>;
    fn erased_visit_i8(&mut self, v: i8) -> Result<Out, Error>;
    fn erased_visit_i16(&mut self, v: i16) -> Result<Out, Error>;
    fn erased_visit_i32(&mut self, v: i32) -> Result<Out, Error>;
    fn erased_visit_i64(&mut self, v: i64) -> Result<Out, Error>;
    fn erased_visit_usize(&mut self, v: usize) -> Result<Out, Error>;
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
    fn erased_visit_unit_struct(&mut self, name: &'static str) -> Result<Out, Error>;
    fn erased_visit_none(&mut self) -> Result<Out, Error>;
    fn erased_visit_some(&mut self, deserializer: &mut Deserializer) -> Result<Out, Error>;
    fn erased_visit_newtype_struct(&mut self, deserializer: &mut Deserializer) -> Result<Out, Error>;
    fn erased_visit_seq(&mut self, visitor: &mut SeqVisitor) -> Result<Out, Error>;
    fn erased_visit_map(&mut self, visitor: &mut MapVisitor) -> Result<Out, Error>;
    fn erased_visit_bytes(&mut self, v: &[u8]) -> Result<Out, Error>;
    fn erased_visit_byte_buf(&mut self, v: Vec<u8>) -> Result<Out, Error>;
}

pub trait SeqVisitor {
    fn erased_visit(&mut self, rei: &mut ReifiedDeserialize) -> Result<Option<()>, Error>;
    fn erased_end(&mut self) -> Result<(), Error>;
    fn erased_size_hint(&self) -> (usize, Option<usize>);
}

pub trait MapVisitor {
    fn erased_visit_key(&mut self, rei: &mut ReifiedDeserialize) -> Result<Option<()>, Error>;
    fn erased_visit_value(&mut self, rei: &mut ReifiedDeserialize) -> Result<(), Error>;
    fn erased_end(&mut self) -> Result<(), Error>;
    fn erased_visit(&mut self, k: &mut ReifiedDeserialize, v: &mut ReifiedDeserialize) -> Result<Option<()>, Error>;
    fn erased_size_hint(&self) -> (usize, Option<usize>);
    fn erased_missing_field(&mut self, field: &'static str, rei: &mut ReifiedDeserialize) -> Result<(), Error>;
}

pub trait EnumVisitor {
    fn erased_visit(&mut self, visitor: &mut VariantVisitor) -> Result<Out, Error>;
}

pub trait VariantVisitor {
    fn erased_visit_variant(&mut self, rei: &mut ReifiedDeserialize) -> Result<(), Error>;
    fn erased_visit_newtype(&mut self, rei: &mut ReifiedDeserialize) -> Result<(), Error>;
    fn erased_visit_tuple(&mut self, len: usize, visitor: &mut Visitor) -> Result<Out, Error>;
    fn erased_visit_struct(&mut self, fields: &'static [&'static str], visitor: &mut Visitor) -> Result<Out, Error>;
    fn erased_visit_unit(&mut self) -> Result<(), Error>;
}

pub trait ReifiedDeserialize {
    fn reified_deserialize(&mut self, deserializer: &mut Deserializer) -> Result<(), Error>;
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

// Required because Out is used as the Value type of the Visitor and EnumVisitor.
impl serde::Deserialize for Out {
    fn deserialize<D>(_deserializer: &mut D) -> Result<Self, D::Error>
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
            fn deserialize<V>(&mut self, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize(&mut visitor).map(Out::take)
            }
            fn deserialize_bool<V>(&mut self, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_bool(&mut visitor).map(Out::take)
            }
            fn deserialize_usize<V>(&mut self, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_usize(&mut visitor).map(Out::take)
            }
            fn deserialize_u8<V>(&mut self, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_u8(&mut visitor).map(Out::take)
            }
            fn deserialize_u16<V>(&mut self, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_u16(&mut visitor).map(Out::take)
            }
            fn deserialize_u32<V>(&mut self, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_u32(&mut visitor).map(Out::take)
            }
            fn deserialize_u64<V>(&mut self, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_u64(&mut visitor).map(Out::take)
            }
            fn deserialize_isize<V>(&mut self, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_isize(&mut visitor).map(Out::take)
            }
            fn deserialize_i8<V>(&mut self, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_i8(&mut visitor).map(Out::take)
            }
            fn deserialize_i16<V>(&mut self, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_i16(&mut visitor).map(Out::take)
            }
            fn deserialize_i32<V>(&mut self, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_i32(&mut visitor).map(Out::take)
            }
            fn deserialize_i64<V>(&mut self, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_i64(&mut visitor).map(Out::take)
            }
            fn deserialize_f32<V>(&mut self, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_f32(&mut visitor).map(Out::take)
            }
            fn deserialize_f64<V>(&mut self, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_f64(&mut visitor).map(Out::take)
            }
            fn deserialize_char<V>(&mut self, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_char(&mut visitor).map(Out::take)
            }
            fn deserialize_str<V>(&mut self, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_str(&mut visitor).map(Out::take)
            }
            fn deserialize_string<V>(&mut self, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_string(&mut visitor).map(Out::take)
            }
            fn deserialize_unit<V>(&mut self, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_unit(&mut visitor).map(Out::take)
            }
            fn deserialize_option<V>(&mut self, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_option(&mut visitor).map(Out::take)
            }
            fn deserialize_seq<V>(&mut self, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_seq(&mut visitor).map(Out::take)
            }
            fn deserialize_seq_fixed_size<V>(&mut self, len: usize, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_seq_fixed_size(len, &mut visitor).map(Out::take)
            }
            fn deserialize_bytes<V>(&mut self, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_bytes(&mut visitor).map(Out::take)
            }
            fn deserialize_map<V>(&mut self, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_map(&mut visitor).map(Out::take)
            }
            fn deserialize_unit_struct<V>(&mut self, name: &'static str, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_unit_struct(name, &mut visitor).map(Out::take)
            }
            fn deserialize_newtype_struct<V>(&mut self, name: &'static str, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_newtype_struct(name, &mut visitor).map(Out::take)
            }
            fn deserialize_tuple_struct<V>(&mut self, name: &'static str, len: usize, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_tuple_struct(name, len, &mut visitor).map(Out::take)
            }
            fn deserialize_struct<V>(&mut self, name: &'static str, fields: &'static [&'static str], mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_struct(name, fields, &mut visitor).map(Out::take)
            }
            fn deserialize_struct_field<V>(&mut self, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_struct_field(&mut visitor).map(Out::take)
            }
            fn deserialize_tuple<V>(&mut self, len: usize, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_tuple(len, &mut visitor).map(Out::take)
            }
            fn deserialize_enum<V>(&mut self, name: &'static str, variants: &'static [&'static str], mut visitor: V) -> Result<V::Value, Error> where V: serde::de::EnumVisitor {
                (**self).erased_deserialize_enum(name, variants, &mut visitor).map(Out::take)
            }
            fn deserialize_ignored_any<V>(&mut self, mut visitor: V) -> Result<V::Value, Error> where V: serde::de::Visitor {
                (**self).erased_deserialize_ignored_any(&mut visitor).map(Out::take)
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
    fn visit_bool<E>(&mut self, v: bool) -> Result<Out, E> where E: serde::de::Error {
        (**self).erased_visit_bool(v).map_err(unerase)
    }
    fn visit_isize<E>(&mut self, v: isize) -> Result<Out, E> where E: serde::de::Error {
        (**self).erased_visit_isize(v).map_err(unerase)
    }
    fn visit_i8<E>(&mut self, v: i8) -> Result<Out, E> where E: serde::de::Error {
        (**self).erased_visit_i8(v).map_err(unerase)
    }
    fn visit_i16<E>(&mut self, v: i16) -> Result<Out, E> where E: serde::de::Error {
        (**self).erased_visit_i16(v).map_err(unerase)
    }
    fn visit_i32<E>(&mut self, v: i32) -> Result<Out, E> where E: serde::de::Error {
        (**self).erased_visit_i32(v).map_err(unerase)
    }
    fn visit_i64<E>(&mut self, v: i64) -> Result<Out, E> where E: serde::de::Error {
        (**self).erased_visit_i64(v).map_err(unerase)
    }
    fn visit_usize<E>(&mut self, v: usize) -> Result<Out, E> where E: serde::de::Error {
        (**self).erased_visit_usize(v).map_err(unerase)
    }
    fn visit_u8<E>(&mut self, v: u8) -> Result<Out, E> where E: serde::de::Error {
        (**self).erased_visit_u8(v).map_err(unerase)
    }
    fn visit_u16<E>(&mut self, v: u16) -> Result<Out, E> where E: serde::de::Error {
        (**self).erased_visit_u16(v).map_err(unerase)
    }
    fn visit_u32<E>(&mut self, v: u32) -> Result<Out, E> where E: serde::de::Error {
        (**self).erased_visit_u32(v).map_err(unerase)
    }
    fn visit_u64<E>(&mut self, v: u64) -> Result<Out, E> where E: serde::de::Error {
        (**self).erased_visit_u64(v).map_err(unerase)
    }
    fn visit_f32<E>(&mut self, v: f32) -> Result<Out, E> where E: serde::de::Error {
        (**self).erased_visit_f32(v).map_err(unerase)
    }
    fn visit_f64<E>(&mut self, v: f64) -> Result<Out, E> where E: serde::de::Error {
        (**self).erased_visit_f64(v).map_err(unerase)
    }
    fn visit_char<E>(&mut self, v: char) -> Result<Out, E> where E: serde::de::Error {
        (**self).erased_visit_char(v).map_err(unerase)
    }
    fn visit_str<E>(&mut self, v: &str) -> Result<Out, E> where E: serde::de::Error {
        (**self).erased_visit_str(v).map_err(unerase)
    }
    fn visit_string<E>(&mut self, v: String) -> Result<Out, E> where E: serde::de::Error {
        (**self).erased_visit_string(v).map_err(unerase)
    }
    fn visit_unit<E>(&mut self) -> Result<Out, E> where E: serde::de::Error {
        (**self).erased_visit_unit().map_err(unerase)
    }
    fn visit_unit_struct<E>(&mut self, name: &'static str) -> Result<Out, E> where E: serde::de::Error {
        (**self).erased_visit_unit_struct(name).map_err(unerase)
    }
    fn visit_none<E>(&mut self) -> Result<Out, E> where E: serde::de::Error {
        (**self).erased_visit_none().map_err(unerase)
    }
    fn visit_some<D>(&mut self, deserializer: &mut D) -> Result<Out, D::Error> where D: serde::Deserializer {
        (**self).erased_visit_some(deserializer).map_err(unerase)
    }
    fn visit_newtype_struct<D>(&mut self, deserializer: &mut D) -> Result<Out, D::Error> where D: serde::Deserializer {
        (**self).erased_visit_newtype_struct(deserializer).map_err(unerase)
    }
    fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Out, V::Error> where V: serde::de::SeqVisitor {
        (**self).erased_visit_seq(&mut visitor).map_err(unerase)
    }
    fn visit_map<V>(&mut self, mut visitor: V) -> Result<Out, V::Error> where V: serde::de::MapVisitor {
        (**self).erased_visit_map(&mut visitor).map_err(unerase)
    }
    fn visit_bytes<E>(&mut self, v: &[u8]) -> Result<Out, E> where E: serde::de::Error {
        (**self).erased_visit_bytes(v).map_err(unerase)
    }
    fn visit_byte_buf<E>(&mut self, v: Vec<u8>) -> Result<Out, E> where E: serde::de::Error {
        (**self).erased_visit_byte_buf(v).map_err(unerase)
    }
}

impl<'a> serde::de::SeqVisitor for &'a mut SeqVisitor {
    type Error = Error;
    fn visit<T>(&mut self) -> Result<Option<T>, Error> where T: serde::Deserialize {
        let mut witness = Witness::new();
        (**self).erased_visit(&mut witness).and_then(|opt| match opt {
            Some(_) => witness.take().map(Some),
            None => Ok(None),
        })
    }
    fn end(&mut self) -> Result<(), Error> {
        (**self).erased_end().map_err(unerase)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (**self).erased_size_hint()
    }
}

impl<'a> serde::de::MapVisitor for &'a mut MapVisitor {
    type Error = Error;
    fn visit_key<K>(&mut self) -> Result<Option<K>, Error> where K: serde::Deserialize {
        let mut witness = Witness::new();
        (**self).erased_visit_key(&mut witness).and_then(|opt| match opt {
            Some(_) => witness.take().map(Some),
            None => Ok(None),
        })
    }
    fn visit_value<V>(&mut self) -> Result<V, Error> where V: serde::Deserialize {
        let mut witness = Witness::new();
        (**self).erased_visit_value(&mut witness).and_then(|_| witness.take())
    }
    fn end(&mut self) -> Result<(), Error> {
        (**self).erased_end()
    }
    fn visit<K, V>(&mut self) -> Result<Option<(K, V)>, Error> where K: serde::Deserialize, V: serde::Deserialize {
        let mut witness_k = Witness::new();
        let mut witness_v = Witness::new();
        (**self).erased_visit(&mut witness_k, &mut witness_v).and_then(|opt| match opt {
            Some(_) => witness_k.take().and_then(|k| witness_v.take().map(|v| Some((k, v)))),
            None => Ok(None),
        })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (**self).erased_size_hint()
    }
    fn missing_field<V>(&mut self, field: &'static str) -> Result<V, Error> where V: serde::Deserialize {
        let mut witness = Witness::new();
        (**self).erased_missing_field(field, &mut witness).and_then(|_| witness.take())
    }
}

impl<'a> serde::de::EnumVisitor for &'a mut EnumVisitor {
    type Value = Out;
    fn visit<V>(&mut self, mut visitor: V) -> Result<Out, V::Error>
        where V: serde::de::VariantVisitor
    {
        (**self).erased_visit(&mut visitor).map_err(unerase)
    }
}

impl<'a> serde::de::VariantVisitor for &'a mut VariantVisitor {
    type Error = Error;
    fn visit_variant<V>(&mut self) -> Result<V, Self::Error> where V: serde::Deserialize {
        let mut witness = Witness::new();
        (**self).erased_visit_variant(&mut witness).and_then(|_| witness.take())
    }
    fn visit_newtype<T>(&mut self) -> Result<T, Self::Error> where T: serde::Deserialize {
        let mut witness = Witness::new();
        (**self).erased_visit_newtype(&mut witness).and_then(|_| witness.take())
    }
    fn visit_tuple<V>(&mut self, len: usize, mut visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        (**self).erased_visit_tuple(len, &mut visitor).map(Out::take)
    }
    fn visit_struct<V>(&mut self, fields: &'static [&'static str], mut visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        (**self).erased_visit_struct(fields, &mut visitor).map(Out::take)
    }
    fn visit_unit(&mut self) -> Result<(), Self::Error> {
        (**self).erased_visit_unit()
    }
}

// WITNESS /////////////////////////////////////////////////////////////////////

struct Witness<T> {
    result: Option<T>,
}

impl<T> Witness<T> {
    fn new() -> Self {
        Witness {
            result: None,
        }
    }

    fn take(mut self) -> Result<T, Error> {
        self.result.take().ok_or_else(|| Error::new("nothing got deserialized"))
    }
}

impl<T> ReifiedDeserialize for Witness<T> where T: serde::Deserialize {
    fn reified_deserialize(&mut self, mut deserializer: &mut Deserializer) -> Result<(), Error> {
        self.result = Some(try!(deserialize(deserializer)));
        Ok(())
    }
}

// STACK ///////////////////////////////////////////////////////////////////////

thread_local! {
    static STACK: RefCell<Vec<&'static mut ReifiedDeserialize>> = RefCell::new(Vec::new())
}

macro_rules! stack_guard {
    ($($rei:ident),* => $body:expr) => {
        unsafe { $( stack_push($rei); )* }
        $body.map_err(erase).and_then(GuardedRet::finalize)
    };
}

unsafe fn stack_push(rei: &mut ReifiedDeserialize) {
    // Cast away the lifetime. Okay as long as it is gone from the stack before
    // the caller returns.
    let rei = mem::transmute(rei);

    STACK.with(|s| s.borrow_mut().push(rei));
}

// If `SeqVisitor::visit` or `MapVisitor::visit_key` returns None, nothing was
// deserialized and the unused entries need to be cleaned up.
fn stack_pop() -> Result<(), Error> {
    STACK.with(|s| match s.borrow_mut().pop() {
        Some(_) => Ok(()),
        None => Err(Error::new("empty deserialize stack")),
    })
}

struct DeserializeFromStack;

impl serde::Deserialize for DeserializeFromStack {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: serde::Deserializer
    {
        let rei = match STACK.with(|s| s.borrow_mut().pop()) {
            Some(rei) => rei,
            None => return Err(serde::de::Error::custom("empty deserialize stack")),
        };

        rei.reified_deserialize(deserializer).map(|_| DeserializeFromStack).map_err(unerase)
    }
}

trait GuardedRet {
    type Ret;
    fn finalize(self) -> Result<Self::Ret, Error>;
}

impl GuardedRet for DeserializeFromStack {
    type Ret = ();
    fn finalize(self) -> Result<Self::Ret, Error> {
        Ok(())
    }
}

impl GuardedRet for Option<DeserializeFromStack> {
    type Ret = Option<()>;
    fn finalize(self) -> Result<Self::Ret, Error> {
        if self.is_some() {
            Ok(Some(()))
        } else {
            try!(stack_pop());
            Ok(None)
        }
    }
}

impl GuardedRet for Option<(DeserializeFromStack, DeserializeFromStack)> {
    type Ret = Option<()>;
    fn finalize(self) -> Result<Self::Ret, Error> {
        if self.is_some() {
            Ok(Some(()))
        } else {
            try!(stack_pop());
            try!(stack_pop());
            Ok(None)
        }
    }
}

// IMPL ERASED SERDE FOR SERDE /////////////////////////////////////////////////

impl<T: ?Sized> Deserializer for T where T: serde::Deserializer {
    fn erased_deserialize(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize(visitor).map_err(erase)
    }
    fn erased_deserialize_bool(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_bool(visitor).map_err(erase)
    }
    fn erased_deserialize_usize(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_usize(visitor).map_err(erase)
    }
    fn erased_deserialize_u8(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_u8(visitor).map_err(erase)
    }
    fn erased_deserialize_u16(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_u16(visitor).map_err(erase)
    }
    fn erased_deserialize_u32(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_u32(visitor).map_err(erase)
    }
    fn erased_deserialize_u64(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_u64(visitor).map_err(erase)
    }
    fn erased_deserialize_isize(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_isize(visitor).map_err(erase)
    }
    fn erased_deserialize_i8(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_i8(visitor).map_err(erase)
    }
    fn erased_deserialize_i16(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_u16(visitor).map_err(erase)
    }
    fn erased_deserialize_i32(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_i32(visitor).map_err(erase)
    }
    fn erased_deserialize_i64(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_i64(visitor).map_err(erase)
    }
    fn erased_deserialize_f32(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_f32(visitor).map_err(erase)
    }
    fn erased_deserialize_f64(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_f64(visitor).map_err(erase)
    }
    fn erased_deserialize_char(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_char(visitor).map_err(erase)
    }
    fn erased_deserialize_str(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_str(visitor).map_err(erase)
    }
    fn erased_deserialize_string(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_string(visitor).map_err(erase)
    }
    fn erased_deserialize_unit(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_unit(visitor).map_err(erase)
    }
    fn erased_deserialize_option(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_option(visitor).map_err(erase)
    }
    fn erased_deserialize_seq(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_seq(visitor).map_err(erase)
    }
    fn erased_deserialize_seq_fixed_size(&mut self, len: usize, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_seq_fixed_size(len, visitor).map_err(erase)
    }
    fn erased_deserialize_bytes(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_bytes(visitor).map_err(erase)
    }
    fn erased_deserialize_map(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_map(visitor).map_err(erase)
    }
    fn erased_deserialize_unit_struct(&mut self, name: &'static str, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_unit_struct(name, visitor).map_err(erase)
    }
    fn erased_deserialize_newtype_struct(&mut self, name: &'static str, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_newtype_struct(name, visitor).map_err(erase)
    }
    fn erased_deserialize_tuple_struct(&mut self, name: &'static str, len: usize, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_tuple_struct(name, len, visitor).map_err(erase)
    }
    fn erased_deserialize_struct(&mut self, name: &'static str, fields: &'static [&'static str], visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_struct(name, fields, visitor).map_err(erase)
    }
    fn erased_deserialize_struct_field(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_struct_field(visitor).map_err(erase)
    }
    fn erased_deserialize_tuple(&mut self, len: usize, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_tuple(len, visitor).map_err(erase)
    }
    fn erased_deserialize_enum(&mut self, name: &'static str, variants: &'static [&'static str], visitor: &mut EnumVisitor) -> Result<Out, Error> {
        self.deserialize_enum(name, variants, visitor).map_err(erase)
    }
    fn erased_deserialize_ignored_any(&mut self, visitor: &mut Visitor) -> Result<Out, Error> {
        self.deserialize_ignored_any(visitor).map_err(erase)
    }
}

impl<T: ?Sized> Visitor for T where T: serde::de::Visitor {
    fn erased_visit_bool(&mut self, v: bool) -> Result<Out, Error> {
        self.visit_bool(v).map(Out::new)
    }
    fn erased_visit_isize(&mut self, v: isize) -> Result<Out, Error> {
        self.visit_isize(v).map(Out::new)
    }
    fn erased_visit_i8(&mut self, v: i8) -> Result<Out, Error> {
        self.visit_i8(v).map(Out::new)
    }
    fn erased_visit_i16(&mut self, v: i16) -> Result<Out, Error> {
        self.visit_i16(v).map(Out::new)
    }
    fn erased_visit_i32(&mut self, v: i32) -> Result<Out, Error> {
        self.visit_i32(v).map(Out::new)
    }
    fn erased_visit_i64(&mut self, v: i64) -> Result<Out, Error> {
        self.visit_i64(v).map(Out::new)
    }
    fn erased_visit_usize(&mut self, v: usize) -> Result<Out, Error> {
        self.visit_usize(v).map(Out::new)
    }
    fn erased_visit_u8(&mut self, v: u8) -> Result<Out, Error> {
        self.visit_u8(v).map(Out::new)
    }
    fn erased_visit_u16(&mut self, v: u16) -> Result<Out, Error> {
        self.visit_u16(v).map(Out::new)
    }
    fn erased_visit_u32(&mut self, v: u32) -> Result<Out, Error> {
        self.visit_u32(v).map(Out::new)
    }
    fn erased_visit_u64(&mut self, v: u64) -> Result<Out, Error> {
        self.visit_u64(v).map(Out::new)
    }
    fn erased_visit_f32(&mut self, v: f32) -> Result<Out, Error> {
        self.visit_f32(v).map(Out::new)
    }
    fn erased_visit_f64(&mut self, v: f64) -> Result<Out, Error> {
        self.visit_f64(v).map(Out::new)
    }
    fn erased_visit_char(&mut self, v: char) -> Result<Out, Error> {
        self.visit_char(v).map(Out::new)
    }
    fn erased_visit_str(&mut self, v: &str) -> Result<Out, Error> {
        self.visit_str(v).map(Out::new)
    }
    fn erased_visit_string(&mut self, v: String) -> Result<Out, Error> {
        self.visit_string(v).map(Out::new)
    }
    fn erased_visit_unit(&mut self) -> Result<Out, Error> {
        self.visit_unit().map(Out::new)
    }
    fn erased_visit_unit_struct(&mut self, name: &'static str) -> Result<Out, Error> {
        self.visit_unit_struct(name).map(Out::new)
    }
    fn erased_visit_none(&mut self) -> Result<Out, Error> {
        self.visit_none().map(Out::new)
    }
    fn erased_visit_some(&mut self, mut deserializer: &mut Deserializer) -> Result<Out, Error> {
        self.visit_some(&mut deserializer).map(Out::new)
    }
    fn erased_visit_newtype_struct(&mut self, mut deserializer: &mut Deserializer) -> Result<Out, Error> {
        self.visit_newtype_struct(&mut deserializer).map(Out::new)
    }
    fn erased_visit_seq(&mut self, visitor: &mut SeqVisitor) -> Result<Out, Error> {
        self.visit_seq(visitor).map(Out::new)
    }
    fn erased_visit_map(&mut self, visitor: &mut MapVisitor) -> Result<Out, Error> {
        self.visit_map(visitor).map(Out::new)
    }
    fn erased_visit_bytes(&mut self, v: &[u8]) -> Result<Out, Error> {
        self.visit_bytes(v).map(Out::new)
    }
    fn erased_visit_byte_buf(&mut self, v: Vec<u8>) -> Result<Out, Error> {
        self.visit_byte_buf(v).map(Out::new)
    }
}

impl<T: ?Sized> SeqVisitor for T where T: serde::de::SeqVisitor {
    fn erased_visit(&mut self, rei: &mut ReifiedDeserialize) -> Result<Option<()>, Error> {
        stack_guard! { rei =>
            self.visit::<DeserializeFromStack>()
        }
    }
    fn erased_end(&mut self) -> Result<(), Error> {
        self.end().map_err(erase)
    }
    fn erased_size_hint(&self) -> (usize, Option<usize>) {
        self.size_hint()
    }
}

impl<T: ?Sized> MapVisitor for T where T: serde::de::MapVisitor {
    fn erased_visit_key(&mut self, rei: &mut ReifiedDeserialize) -> Result<Option<()>, Error> {
        stack_guard! { rei =>
            self.visit_key::<DeserializeFromStack>()
        }
    }
    fn erased_visit_value(&mut self, rei: &mut ReifiedDeserialize) -> Result<(), Error> {
        stack_guard! { rei =>
            self.visit_value::<DeserializeFromStack>()
        }
    }
    fn erased_end(&mut self) -> Result<(), Error> {
        self.end().map_err(erase)
    }
    fn erased_visit(&mut self, k: &mut ReifiedDeserialize, v: &mut ReifiedDeserialize) -> Result<Option<()>, Error> {
        // v, k in reverse order because k will be used first
        stack_guard! { v, k =>
            self.visit::<DeserializeFromStack, DeserializeFromStack>()
        }
    }
    fn erased_size_hint(&self) -> (usize, Option<usize>) {
        self.size_hint()
    }
    fn erased_missing_field(&mut self, field: &'static str, rei: &mut ReifiedDeserialize) -> Result<(), Error> {
        stack_guard! { rei =>
            self.missing_field::<DeserializeFromStack>(field)
        }
    }
}

impl<T: ?Sized> EnumVisitor for T where T: serde::de::EnumVisitor {
    fn erased_visit(&mut self, visitor: &mut VariantVisitor) -> Result<Out, Error> {
        self.visit(visitor).map(Out::new).map_err(erase)
    }
}

impl<T: ?Sized> VariantVisitor for T where T: serde::de::VariantVisitor {
    fn erased_visit_variant(&mut self, rei: &mut ReifiedDeserialize) -> Result<(), Error> {
        stack_guard! { rei =>
            self.visit_variant::<DeserializeFromStack>()
        }
    }
    fn erased_visit_newtype(&mut self, rei: &mut ReifiedDeserialize) -> Result<(), Error> {
        stack_guard! { rei =>
            self.visit_newtype::<DeserializeFromStack>()
        }
    }
    fn erased_visit_tuple(&mut self, len: usize, visitor: &mut Visitor) -> Result<Out, Error> {
        self.visit_tuple(len, visitor).map_err(erase)
    }
    fn erased_visit_struct(&mut self, fields: &'static [&'static str], visitor: &mut Visitor) -> Result<Out, Error> {
        self.visit_struct(fields, visitor).map_err(erase)
    }
    fn erased_visit_unit(&mut self) -> Result<(), Error> {
        self.visit_unit().map_err(erase)
    }
}

// ERROR ///////////////////////////////////////////////////////////////////////

fn erase<E>(e: E) -> Error
    where E: Display
{
    Error::new(e.to_string())
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

    use std::io::Read;

    let json = br#"["a", 1, [true], {"a": 1}]"#;
    let expected: serde_json::Value = serde_json::from_slice(json).unwrap();

    let mut de = serde_json::Deserializer::new(json.bytes());
    let de: &mut Deserializer = &mut de;
    assert_eq!(expected, deserialize(de).unwrap());
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
