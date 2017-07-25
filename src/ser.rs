use std::fmt::Display;
use std::marker::PhantomData;

use serde;
use serde::ser::{SerializeSeq, SerializeTuple, SerializeTupleStruct,
                 SerializeTupleVariant, SerializeMap, SerializeStruct,
                 SerializeStructVariant};

use any::Any;
use error::Error;

// TRAITS //////////////////////////////////////////////////////////////////////

pub trait Serialize {
    fn erased_serialize(&self, &mut Serializer) -> Result<Ok, Error>;
}

pub trait Serializer {
    fn erased_serialize_bool(&mut self, bool) -> Result<Ok, Error>;
    fn erased_serialize_i8(&mut self, i8) -> Result<Ok, Error>;
    fn erased_serialize_i16(&mut self, i16) -> Result<Ok, Error>;
    fn erased_serialize_i32(&mut self, i32) -> Result<Ok, Error>;
    fn erased_serialize_i64(&mut self, i64) -> Result<Ok, Error>;
    fn erased_serialize_u8(&mut self, u8) -> Result<Ok, Error>;
    fn erased_serialize_u16(&mut self, u16) -> Result<Ok, Error>;
    fn erased_serialize_u32(&mut self, u32) -> Result<Ok, Error>;
    fn erased_serialize_u64(&mut self, u64) -> Result<Ok, Error>;
    fn erased_serialize_f32(&mut self, f32) -> Result<Ok, Error>;
    fn erased_serialize_f64(&mut self, f64) -> Result<Ok, Error>;
    fn erased_serialize_char(&mut self, char) -> Result<Ok, Error>;
    fn erased_serialize_str(&mut self, &str) -> Result<Ok, Error>;
    fn erased_serialize_bytes(&mut self, &[u8]) -> Result<Ok, Error>;
    fn erased_serialize_none(&mut self) -> Result<Ok, Error>;
    fn erased_serialize_some(&mut self, v: &Serialize) -> Result<Ok, Error>;
    fn erased_serialize_unit(&mut self) -> Result<Ok, Error>;
    fn erased_serialize_unit_struct(&mut self, name: &'static str) -> Result<Ok, Error>;
    fn erased_serialize_unit_variant(&mut self, name: &'static str, variant_index: usize, variant: &'static str) -> Result<Ok, Error>;
    fn erased_serialize_newtype_struct(&mut self, name: &'static str, v: &Serialize) -> Result<Ok, Error>;
    fn erased_serialize_newtype_variant(&mut self, name: &'static str, variant_index: usize, variant: &'static str, v: &Serialize) -> Result<Ok, Error>;
    fn erased_serialize_seq(&mut self, len: Option<usize>) -> Result<Seq, Error>;
    fn erased_serialize_seq_fixed_size(&mut self, size: usize) -> Result<Seq, Error>;
    fn erased_serialize_tuple(&mut self, len: usize) -> Result<Tuple, Error>;
    fn erased_serialize_tuple_struct(&mut self, name: &'static str, len: usize) -> Result<TupleStruct, Error>;
    fn erased_serialize_tuple_variant(&mut self, name: &'static str, variant_index: usize, variant: &'static str, len: usize) -> Result<TupleVariant, Error>;
    fn erased_serialize_map(&mut self, len: Option<usize>) -> Result<Map, Error>;
    fn erased_serialize_struct(&mut self, name: &'static str, len: usize) -> Result<Struct, Error>;
    fn erased_serialize_struct_variant(&mut self, name: &'static str, variant_index: usize, variant: &'static str, len: usize) -> Result<StructVariant, Error>;
}

impl Serializer {
    pub fn erase<S>(serializer: S) -> erase::Serializer<S>
        where S: serde::Serializer,
              S::Ok: 'static,
    {
        erase::Serializer {
            state: Some(serializer),
        }
    }
}

// OK //////////////////////////////////////////////////////////////////////////

// Corresponds to the Serializer::Ok associated type.
//
// This struct is exposed to users by invoking methods on the Serialize or
// Serializer trait objects, so we need to make sure they do not hold on to the
// Ok beyond the lifetime of the data in the Any.
//
// We do this by enforcing S::Ok is 'static for every Serializer trait object
// created by the user.
pub struct Ok {
    data: Any,
}

impl Ok {
    fn new<T>(t: T) -> Self {
        Ok {
            data: Any::new(t),
        }
    }

    fn take<T>(self) -> T {
        self.data.take()
    }
}

// IMPL ERASED SERDE FOR SERDE /////////////////////////////////////////////////

impl<T: ?Sized> Serialize for T
    where T: serde::Serialize
{
    fn erased_serialize(&self, serializer: &mut Serializer) -> Result<Ok, Error> {
        self.serialize(serializer)
    }
}

mod erase {
    pub struct Serializer<S> {
        pub(crate) state: Option<S>,
    }

    impl<S> Serializer<S> {
        pub(crate) fn take(&mut self) -> S {
            self.state.take().unwrap()
        }
    }
}

impl<T: ?Sized> Serializer for erase::Serializer<T>
    where T: serde::Serializer
{
    fn erased_serialize_bool(&mut self, v: bool) -> Result<Ok, Error> {
        self.take().serialize_bool(v).map(Ok::new).map_err(erase)
    }
    fn erased_serialize_i8(&mut self, v: i8) -> Result<Ok, Error> {
        self.take().serialize_i8(v).map(Ok::new).map_err(erase)
    }
    fn erased_serialize_i16(&mut self, v: i16) -> Result<Ok, Error> {
        self.take().serialize_i16(v).map(Ok::new).map_err(erase)
    }
    fn erased_serialize_i32(&mut self, v: i32) -> Result<Ok, Error> {
        self.take().serialize_i32(v).map(Ok::new).map_err(erase)
    }
    fn erased_serialize_i64(&mut self, v: i64) -> Result<Ok, Error> {
        self.take().serialize_i64(v).map(Ok::new).map_err(erase)
    }
    fn erased_serialize_u8(&mut self, v: u8) -> Result<Ok, Error> {
        self.take().serialize_u8(v).map(Ok::new).map_err(erase)
    }
    fn erased_serialize_u16(&mut self, v: u16) -> Result<Ok, Error> {
        self.take().serialize_u16(v).map(Ok::new).map_err(erase)
    }
    fn erased_serialize_u32(&mut self, v: u32) -> Result<Ok, Error> {
        self.take().serialize_u32(v).map(Ok::new).map_err(erase)
    }
    fn erased_serialize_u64(&mut self, v: u64) -> Result<Ok, Error> {
        self.take().serialize_u64(v).map(Ok::new).map_err(erase)
    }
    fn erased_serialize_f32(&mut self, v: f32) -> Result<Ok, Error> {
        self.take().serialize_f32(v).map(Ok::new).map_err(erase)
    }
    fn erased_serialize_f64(&mut self, v: f64) -> Result<Ok, Error> {
        self.take().serialize_f64(v).map(Ok::new).map_err(erase)
    }
    fn erased_serialize_char(&mut self, v: char) -> Result<Ok, Error> {
        self.take().serialize_char(v).map(Ok::new).map_err(erase)
    }
    fn erased_serialize_str(&mut self, v: &str) -> Result<Ok, Error> {
        self.take().serialize_str(v).map(Ok::new).map_err(erase)
    }
    fn erased_serialize_bytes(&mut self, v: &[u8]) -> Result<Ok, Error> {
        self.take().serialize_bytes(v).map(Ok::new).map_err(erase)
    }
    fn erased_serialize_none(&mut self) -> Result<Ok, Error> {
        self.take().serialize_none().map(Ok::new).map_err(erase)
    }
    fn erased_serialize_some(&mut self, v: &Serialize) -> Result<Ok, Error> {
        self.take().serialize_some(v).map(Ok::new).map_err(erase)
    }
    fn erased_serialize_unit(&mut self) -> Result<Ok, Error> {
        self.take().serialize_unit().map(Ok::new).map_err(erase)
    }
    fn erased_serialize_unit_struct(&mut self, name: &'static str) -> Result<Ok, Error> {
        self.take().serialize_unit_struct(name).map(Ok::new).map_err(erase)
    }
    fn erased_serialize_unit_variant(&mut self, name: &'static str, variant_index: usize, variant: &'static str) -> Result<Ok, Error> {
        self.take().serialize_unit_variant(name, variant_index, variant).map(Ok::new).map_err(erase)
    }
    fn erased_serialize_newtype_struct(&mut self, name: &'static str, v: &Serialize) -> Result<Ok, Error> {
        self.take().serialize_newtype_struct(name, v).map(Ok::new).map_err(erase)
    }
    fn erased_serialize_newtype_variant(&mut self, name: &'static str, variant_index: usize, variant: &'static str, v: &Serialize) -> Result<Ok, Error> {
        self.take().serialize_newtype_variant(name, variant_index, variant, v).map(Ok::new).map_err(erase)
    }
    fn erased_serialize_seq(&mut self, len: Option<usize>) -> Result<Seq, Error> {
        self.take().serialize_seq(len).map(Seq::new).map_err(erase)
    }
    fn erased_serialize_seq_fixed_size(&mut self, size: usize) -> Result<Seq, Error> {
        self.take().serialize_seq_fixed_size(size).map(Seq::new).map_err(erase)
    }
    fn erased_serialize_tuple(&mut self, len: usize) -> Result<Tuple, Error> {
        self.take().serialize_tuple(len).map(Tuple::new).map_err(erase)
    }
    fn erased_serialize_tuple_struct(&mut self, name: &'static str, len: usize) -> Result<TupleStruct, Error> {
        self.take().serialize_tuple_struct(name, len).map(TupleStruct::new).map_err(erase)
    }
    fn erased_serialize_tuple_variant(&mut self, name: &'static str, variant_index: usize, variant: &'static str, len: usize) -> Result<TupleVariant, Error> {
        self.take().serialize_tuple_variant(name, variant_index, variant, len).map(TupleVariant::new).map_err(erase)
    }
    fn erased_serialize_map(&mut self, len: Option<usize>) -> Result<Map, Error> {
        self.take().serialize_map(len).map(Map::new).map_err(erase)
    }
    fn erased_serialize_struct(&mut self, name: &'static str, len: usize) -> Result<Struct, Error> {
        self.take().serialize_struct(name, len).map(Struct::new).map_err(erase)
    }
    fn erased_serialize_struct_variant(&mut self, name: &'static str, variant_index: usize, variant: &'static str, len: usize) -> Result<StructVariant, Error> {
        self.take().serialize_struct_variant(name, variant_index, variant, len).map(StructVariant::new).map_err(erase)
    }
}

// IMPL SERDE FOR ERASED SERDE /////////////////////////////////////////////////

macro_rules! impl_serialize_for_trait_object {
    (Serialize $(+ $traits:ident)*) => {
        impl<'a> serde::Serialize for Serialize + 'a $(+ $traits)* {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: serde::Serializer
            {
                let mut erased = erase::Serializer {
                    state: Some(serializer),
                };
                self.erased_serialize(&mut erased).map(Ok::take).map_err(unerase)
            }
        }
    };
}

impl_serialize_for_trait_object!(Serialize);
impl_serialize_for_trait_object!(Serialize + Send);
impl_serialize_for_trait_object!(Serialize + Sync);
impl_serialize_for_trait_object!(Serialize + Send + Sync);

macro_rules! impl_serializer_for_trait_object {
    ($ty:ty) => {
        impl<'a> serde::Serializer for $ty {
            type Ok = Ok;
            type Error = Error;
            type SerializeSeq = Seq<'a>;
            type SerializeTuple = Tuple<'a>;
            type SerializeTupleStruct = TupleStruct<'a>;
            type SerializeTupleVariant = TupleVariant<'a>;
            type SerializeMap = Map<'a>;
            type SerializeStruct = Struct<'a>;
            type SerializeStructVariant = StructVariant<'a>;
            fn serialize_bool(mut self, v: bool) -> Result<Ok, Error> {
                self.erased_serialize_bool(v)
            }
            fn serialize_i8(mut self, v: i8) -> Result<Ok, Error> {
                self.erased_serialize_i8(v)
            }
            fn serialize_i16(mut self, v: i16) -> Result<Ok, Error> {
                self.erased_serialize_i16(v)
            }
            fn serialize_i32(mut self, v: i32) -> Result<Ok, Error> {
                self.erased_serialize_i32(v)
            }
            fn serialize_i64(mut self, v: i64) -> Result<Ok, Error> {
                self.erased_serialize_i64(v)
            }
            fn serialize_u8(mut self, v: u8) -> Result<Ok, Error> {
                self.erased_serialize_u8(v)
            }
            fn serialize_u16(mut self, v: u16) -> Result<Ok, Error> {
                self.erased_serialize_u16(v)
            }
            fn serialize_u32(mut self, v: u32) -> Result<Ok, Error> {
                self.erased_serialize_u32(v)
            }
            fn serialize_u64(mut self, v: u64) -> Result<Ok, Error> {
                self.erased_serialize_u64(v)
            }
            fn serialize_f32(mut self, v: f32) -> Result<Ok, Error> {
                self.erased_serialize_f32(v)
            }
            fn serialize_f64(mut self, v: f64) -> Result<Ok, Error> {
                self.erased_serialize_f64(v)
            }
            fn serialize_char(mut self, v: char) -> Result<Ok, Error> {
                self.erased_serialize_char(v)
            }
            fn serialize_str(mut self, v: &str) -> Result<Ok, Error> {
                self.erased_serialize_str(v)
            }
            fn serialize_bytes(mut self, v: &[u8]) -> Result<Ok, Error> {
                self.erased_serialize_bytes(v)
            }
            fn serialize_none(mut self) -> Result<Ok, Error> {
                self.erased_serialize_none()
            }
            fn serialize_some<T: ?Sized + serde::Serialize>(mut self, v: &T) -> Result<Ok, Error> {
                self.erased_serialize_some(&v)
            }
            fn serialize_unit(mut self) -> Result<Ok, Error> {
                self.erased_serialize_unit()
            }
            fn serialize_unit_struct(mut self, name: &'static str) -> Result<Ok, Error> {
                self.erased_serialize_unit_struct(name)
            }
            fn serialize_unit_variant(mut self, name: &'static str, variant_index: usize, variant: &'static str) -> Result<Ok, Error> {
                self.erased_serialize_unit_variant(name, variant_index, variant)
            }
            fn serialize_newtype_struct<T: ?Sized + serde::Serialize>(mut self, name: &'static str, v: &T) -> Result<Ok, Error> {
                self.erased_serialize_newtype_struct(name, &v)
            }
            fn serialize_newtype_variant<T: ?Sized + serde::Serialize>(mut self, name: &'static str, variant_index: usize, variant: &'static str, v: &T) -> Result<Ok, Error> {
                self.erased_serialize_newtype_variant(name, variant_index, variant, &v)
            }
            fn serialize_seq(mut self, len: Option<usize>) -> Result<Seq<'a>, Error> {
                self.erased_serialize_seq(len)
            }
            fn serialize_seq_fixed_size(mut self, size: usize) -> Result<Seq<'a>, Error> {
                self.erased_serialize_seq_fixed_size(size)
            }
            fn serialize_tuple(mut self, len: usize) -> Result<Tuple<'a>, Error> {
                self.erased_serialize_tuple(len)
            }
            fn serialize_tuple_struct(mut self, name: &'static str, len: usize) -> Result<TupleStruct<'a>, Error> {
                self.erased_serialize_tuple_struct(name, len)
            }
            fn serialize_tuple_variant(mut self, name: &'static str, variant_index: usize, variant: &'static str, len: usize) -> Result<TupleVariant<'a>, Error> {
                self.erased_serialize_tuple_variant(name, variant_index, variant, len)
            }
            fn serialize_map(mut self, len: Option<usize>) -> Result<Map<'a>, Error> {
                self.erased_serialize_map(len)
            }
            fn serialize_struct(mut self, name: &'static str, len: usize) -> Result<Struct<'a>, Error> {
                self.erased_serialize_struct(name, len)
            }
            fn serialize_struct_variant(mut self, name: &'static str, variant_index: usize, variant: &'static str, len: usize) -> Result<StructVariant<'a>, Error> {
                self.erased_serialize_struct_variant(name, variant_index, variant, len)
            }
        }
    };
}

impl_serializer_for_trait_object!(&'a mut Serializer);
impl_serializer_for_trait_object!(&'a mut (Serializer + Send));
impl_serializer_for_trait_object!(&'a mut (Serializer + Sync));
impl_serializer_for_trait_object!(&'a mut (Serializer + Send + Sync));

pub struct Seq<'a> {
    data: Any,
    serialize_element: fn(&mut Any, &Serialize) -> Result<(), Error>,
    end: fn(Any) -> Result<Ok, Error>,
    lifetime: PhantomData<&'a Serializer>,
}

impl<'a> Seq<'a> {
    fn new<T: serde::ser::SerializeSeq>(data: T) -> Self {
        Seq {
            data: Any::new(data),
            serialize_element: |data, v| {
                data.view::<T>().serialize_element(v).map_err(erase)
            },
            end: |data| {
                data.take::<T>().end().map(Ok::new).map_err(erase)
            },
            lifetime: PhantomData,
        }
    }
}

impl<'a> SerializeSeq for Seq<'a> {
    type Ok = Ok;
    type Error = Error;
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error> where T: serde::Serialize {
        (self.serialize_element)(&mut self.data, &value)
    }
    fn end(self) -> Result<Ok, Error> {
        (self.end)(self.data)
    }
}

pub struct Tuple<'a> {
    data: Any,
    serialize_element: fn(&mut Any, &Serialize) -> Result<(), Error>,
    end: fn(Any) -> Result<Ok, Error>,
    lifetime: PhantomData<&'a Serializer>,
}

impl<'a> Tuple<'a> {
    fn new<T: serde::ser::SerializeTuple>(data: T) -> Self {
        Tuple {
            data: Any::new(data),
            serialize_element: |data, v| {
                data.view::<T>().serialize_element(v).map_err(erase)
            },
            end: |data| {
                data.take::<T>().end().map(Ok::new).map_err(erase)
            },
            lifetime: PhantomData,
        }
    }
}

impl<'a> SerializeTuple for Tuple<'a> {
    type Ok = Ok;
    type Error = Error;
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error> where T: serde::Serialize {
        (self.serialize_element)(&mut self.data, &value)
    }
    fn end(self) -> Result<Ok, Error> {
        (self.end)(self.data)
    }
}

pub struct TupleStruct<'a> {
    data: Any,
    serialize_field: fn(&mut Any, &Serialize) -> Result<(), Error>,
    end: fn(Any) -> Result<Ok, Error>,
    lifetime: PhantomData<&'a Serializer>,
}

impl<'a> TupleStruct<'a> {
    fn new<T: serde::ser::SerializeTupleStruct>(data: T) -> Self {
        TupleStruct {
            data: Any::new(data),
            serialize_field: |data, v| {
                data.view::<T>().serialize_field(v).map_err(erase)
            },
            end: |data| {
                data.take::<T>().end().map(Ok::new).map_err(erase)
            },
            lifetime: PhantomData,
        }
    }
}

impl<'a> SerializeTupleStruct for TupleStruct<'a> {
    type Ok = Ok;
    type Error = Error;
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error> where T: serde::Serialize {
        (self.serialize_field)(&mut self.data, &value)
    }
    fn end(self) -> Result<Ok, Error> {
        (self.end)(self.data)
    }
}

pub struct TupleVariant<'a> {
    data: Any,
    serialize_field: fn(&mut Any, &Serialize) -> Result<(), Error>,
    end: fn(Any) -> Result<Ok, Error>,
    lifetime: PhantomData<&'a Serializer>,
}

impl<'a> TupleVariant<'a> {
    fn new<T: serde::ser::SerializeTupleVariant>(data: T) -> Self {
        TupleVariant {
            data: Any::new(data),
            serialize_field: |data, v| {
                data.view::<T>().serialize_field(v).map_err(erase)
            },
            end: |data| {
                data.take::<T>().end().map(Ok::new).map_err(erase)
            },
            lifetime: PhantomData,
        }
    }
}

impl<'a> SerializeTupleVariant for TupleVariant<'a> {
    type Ok = Ok;
    type Error = Error;
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error> where T: serde::Serialize {
        (self.serialize_field)(&mut self.data, &value)
    }
    fn end(self) -> Result<Ok, Error> {
        (self.end)(self.data)
    }
}

pub struct Map<'a> {
    data: Any,
    serialize_key: fn(&mut Any, &Serialize) -> Result<(), Error>,
    serialize_value: fn(&mut Any, &Serialize) -> Result<(), Error>,
    serialize_entry: fn(&mut Any, &Serialize, &Serialize) -> Result<(), Error>,
    end: fn(Any) -> Result<Ok, Error>,
    lifetime: PhantomData<&'a Serializer>,
}

impl<'a> Map<'a> {
    fn new<T: serde::ser::SerializeMap>(data: T) -> Self {
        Map {
            data: Any::new(data),
            serialize_key: |data, v| {
                data.view::<T>().serialize_key(v).map_err(erase)
            },
            serialize_value: |data, v| {
                data.view::<T>().serialize_value(v).map_err(erase)
            },
            serialize_entry: |data, k, v| {
                data.view::<T>().serialize_entry(k, v).map_err(erase)
            },
            end: |data| {
                data.take::<T>().end().map(Ok::new).map_err(erase)
            },
            lifetime: PhantomData,
        }
    }
}

impl<'a> SerializeMap for Map<'a> {
    type Ok = Ok;
    type Error = Error;
    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Error> where T: serde::Serialize {
        (self.serialize_key)(&mut self.data, &key)
    }
    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Error> where T: serde::Serialize {
        (self.serialize_value)(&mut self.data, &value)
    }
    fn serialize_entry<K: ?Sized, V: ?Sized>(&mut self, key: &K, value: &V) -> Result<(), Error> where K: serde::Serialize, V: serde::Serialize {
        (self.serialize_entry)(&mut self.data, &key, &value)
    }
    fn end(self) -> Result<Ok, Error> {
        (self.end)(self.data)
    }
}

pub struct Struct<'a> {
    data: Any,
    serialize_field: fn(&mut Any, &'static str, &Serialize) -> Result<(), Error>,
    end: fn(Any) -> Result<Ok, Error>,
    lifetime: PhantomData<&'a Serializer>,
}

impl<'a> Struct<'a> {
    fn new<T: serde::ser::SerializeStruct>(data: T) -> Self {
        Struct {
            data: Any::new(data),
            serialize_field: |data, k, v| {
                data.view::<T>().serialize_field(k, v).map_err(erase)
            },
            end: |data| {
                data.take::<T>().end().map(Ok::new).map_err(erase)
            },
            lifetime: PhantomData,
        }
    }
}

impl<'a> SerializeStruct for Struct<'a> {
    type Ok = Ok;
    type Error = Error;
    fn serialize_field<T: ?Sized>(&mut self, name: &'static str, field: &T) -> Result<(), Error> where T: serde::Serialize {
        (self.serialize_field)(&mut self.data, name, &field)
    }
    fn end(self) -> Result<Ok, Error> {
        (self.end)(self.data)
    }
}

pub struct StructVariant<'a> {
    data: Any,
    serialize_field: fn(&mut Any, &'static str, &Serialize) -> Result<(), Error>,
    end: fn(Any) -> Result<Ok, Error>,
    lifetime: PhantomData<&'a Serializer>,
}

impl<'a> StructVariant<'a> {
    fn new<T: serde::ser::SerializeStructVariant>(data: T) -> Self {
        StructVariant {
            data: Any::new(data),
            serialize_field: |data, k, v| {
                data.view::<T>().serialize_field(k, v).map_err(erase)
            },
            end: |data| {
                data.take::<T>().end().map(Ok::new).map_err(erase)
            },
            lifetime: PhantomData,
        }
    }
}

impl<'a> SerializeStructVariant for StructVariant<'a> {
    type Ok = Ok;
    type Error = Error;
    fn serialize_field<T: ?Sized>(&mut self, name: &'static str, field: &T) -> Result<(), Error> where T: serde::Serialize {
        (self.serialize_field)(&mut self.data, name, &field)
    }
    fn end(self) -> Result<Ok, Error> {
        (self.end)(self.data)
    }
}

// IMPL ERASED SERDE FOR ERASED SERDE //////////////////////////////////////////

macro_rules! deref_erased_serializer {
    ($($imp:tt)+) => {
        impl $($imp)+ {
            fn erased_serialize_bool(&mut self, v: bool) -> Result<Ok, Error> {
                (**self).erased_serialize_bool(v)
            }
            fn erased_serialize_i8(&mut self, v: i8) -> Result<Ok, Error> {
                (**self).erased_serialize_i8(v)
            }
            fn erased_serialize_i16(&mut self, v: i16) -> Result<Ok, Error> {
                (**self).erased_serialize_i16(v)
            }
            fn erased_serialize_i32(&mut self, v: i32) -> Result<Ok, Error> {
                (**self).erased_serialize_i32(v)
            }
            fn erased_serialize_i64(&mut self, v: i64) -> Result<Ok, Error> {
                (**self).erased_serialize_i64(v)
            }
            fn erased_serialize_u8(&mut self, v: u8) -> Result<Ok, Error> {
                (**self).erased_serialize_u8(v)
            }
            fn erased_serialize_u16(&mut self, v: u16) -> Result<Ok, Error> {
                (**self).erased_serialize_u16(v)
            }
            fn erased_serialize_u32(&mut self, v: u32) -> Result<Ok, Error> {
                (**self).erased_serialize_u32(v)
            }
            fn erased_serialize_u64(&mut self, v: u64) -> Result<Ok, Error> {
                (**self).erased_serialize_u64(v)
            }
            fn erased_serialize_f32(&mut self, v: f32) -> Result<Ok, Error> {
                (**self).erased_serialize_f32(v)
            }
            fn erased_serialize_f64(&mut self, v: f64) -> Result<Ok, Error> {
                (**self).erased_serialize_f64(v)
            }
            fn erased_serialize_char(&mut self, v: char) -> Result<Ok, Error> {
                (**self).erased_serialize_char(v)
            }
            fn erased_serialize_str(&mut self, v: &str) -> Result<Ok, Error> {
                (**self).erased_serialize_str(v)
            }
            fn erased_serialize_bytes(&mut self, v: &[u8]) -> Result<Ok, Error> {
                (**self).erased_serialize_bytes(v)
            }
            fn erased_serialize_none(&mut self) -> Result<Ok, Error> {
                (**self).erased_serialize_none()
            }
            fn erased_serialize_some(&mut self, v: &Serialize) -> Result<Ok, Error> {
                (**self).erased_serialize_some(v)
            }
            fn erased_serialize_unit(&mut self) -> Result<Ok, Error> {
                (**self).erased_serialize_unit()
            }
            fn erased_serialize_unit_struct(&mut self, name: &'static str) -> Result<Ok, Error> {
                (**self).erased_serialize_unit_struct(name)
            }
            fn erased_serialize_unit_variant(&mut self, name: &'static str, variant_index: usize, variant: &'static str) -> Result<Ok, Error> {
                (**self).erased_serialize_unit_variant(name, variant_index, variant)
            }
            fn erased_serialize_newtype_struct(&mut self, name: &'static str, v: &Serialize) -> Result<Ok, Error> {
                (**self).erased_serialize_newtype_struct(name, v)
            }
            fn erased_serialize_newtype_variant(&mut self, name: &'static str, variant_index: usize, variant: &'static str, v: &Serialize) -> Result<Ok, Error> {
                (**self).erased_serialize_newtype_variant(name, variant_index, variant, v)
            }
            fn erased_serialize_seq(&mut self, len: Option<usize>) -> Result<Seq, Error> {
                (**self).erased_serialize_seq(len)
            }
            fn erased_serialize_seq_fixed_size(&mut self, size: usize) -> Result<Seq, Error> {
                (**self).erased_serialize_seq_fixed_size(size)
            }
            fn erased_serialize_tuple(&mut self, len: usize) -> Result<Tuple, Error> {
                (**self).erased_serialize_tuple(len)
            }
            fn erased_serialize_tuple_struct(&mut self, name: &'static str, len: usize) -> Result<TupleStruct, Error> {
                (**self).erased_serialize_tuple_struct(name, len)
            }
            fn erased_serialize_tuple_variant(&mut self, name: &'static str, variant_index: usize, variant: &'static str, len: usize) -> Result<TupleVariant, Error> {
                (**self).erased_serialize_tuple_variant(name, variant_index, variant, len)
            }
            fn erased_serialize_map(&mut self, len: Option<usize>) -> Result<Map, Error> {
                (**self).erased_serialize_map(len)
            }
            fn erased_serialize_struct(&mut self, name: &'static str, len: usize) -> Result<Struct, Error> {
                (**self).erased_serialize_struct(name, len)
            }
            fn erased_serialize_struct_variant(&mut self, name: &'static str, variant_index: usize, variant: &'static str, len: usize) -> Result<StructVariant, Error> {
                (**self).erased_serialize_struct_variant(name, variant_index, variant, len)
            }
        }
    };
}

deref_erased_serializer!(<'a> Serializer for Box<Serializer + 'a>);
deref_erased_serializer!(<'a> Serializer for Box<Serializer + Send + 'a>);
deref_erased_serializer!(<'a> Serializer for Box<Serializer + Sync + 'a>);
deref_erased_serializer!(<'a> Serializer for Box<Serializer + Send + Sync + 'a>);
deref_erased_serializer!(<'a, T: ?Sized + Serializer> Serializer for &'a mut T);

// ERROR ///////////////////////////////////////////////////////////////////////

fn erase<E>(e: E) -> Error
    where E: Display
{
    serde::ser::Error::custom(e)
}

fn unerase<E>(e: Error) -> E
    where E: serde::ser::Error
{
    use std::error::Error;
    E::custom(e.description())
}

// TEST ////////////////////////////////////////////////////////////////////////

#[test]
fn trait_object() {
    extern crate serde_json;

    let obj: &Serialize = &vec!["a", "b"];

    let mut buf = Vec::new();

    {
        let mut ser = serde_json::Serializer::new(&mut buf);
        let ser: &mut Serializer = &mut Serializer::erase(&mut ser);

        obj.erased_serialize(ser).unwrap();
    }

    assert_eq!(&buf, br#"["a","b"]"#);
}

#[test]
fn box_trait() {
    extern crate serde_json;

    let obj: Box<Serialize> = Box::new(vec!["a", "b"]);

    let mut buf = Vec::new();

    {
        let mut ser = serde_json::Serializer::new(&mut buf);
        let mut ser: Box<Serializer> = Box::new(Serializer::erase(&mut ser));

        obj.erased_serialize(&mut ser).unwrap();
    }

    assert_eq!(&buf, br#"["a","b"]"#);
}

#[test]
fn assert_serialize() {
    fn assert<T: serde::Serialize>() {}

    assert::<&Serialize>();
    assert::<&(Serialize + Send)>();
    assert::<&(Serialize + Sync)>();
    assert::<&(Serialize + Send + Sync)>();
    assert::<&(Serialize + Sync + Send)>();
    assert::<Vec<&Serialize>>();
    assert::<Vec<&(Serialize + Send)>>();

    assert::<Box<Serialize>>();
    assert::<Box<Serialize + Send>>();
    assert::<Box<Serialize + Sync>>();
    assert::<Box<Serialize + Send + Sync>>();
    assert::<Box<Serialize + Sync + Send>>();
    assert::<Vec<Box<Serialize>>>();
    assert::<Vec<Box<Serialize + Send>>>();
}

#[test]
fn assert_serializer() {
    fn assert<T: serde::Serializer>() {}

    assert::<&mut Serializer>();
    assert::<&mut (Serializer + Send)>();
    assert::<&mut (Serializer + Sync)>();
    assert::<&mut (Serializer + Send + Sync)>();
    assert::<&mut (Serializer + Sync + Send)>();
}
