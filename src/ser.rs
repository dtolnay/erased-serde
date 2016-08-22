use std::str;
use std::fmt::Display;

use serde;

use any::Any;
use error::Error;

pub trait Serialize {
    fn erased_serialize(&self, &mut Serializer) -> Result<(), Error>;
}

pub trait Serializer {
    fn erased_serialize_bool(&mut self, bool) -> Result<(), Error>;
    fn erased_serialize_isize(&mut self, isize) -> Result<(), Error>;
    fn erased_serialize_i8(&mut self, i8) -> Result<(), Error>;
    fn erased_serialize_i16(&mut self, i16) -> Result<(), Error>;
    fn erased_serialize_i32(&mut self, i32) -> Result<(), Error>;
    fn erased_serialize_i64(&mut self, i64) -> Result<(), Error>;
    fn erased_serialize_usize(&mut self, usize) -> Result<(), Error>;
    fn erased_serialize_u8(&mut self, u8) -> Result<(), Error>;
    fn erased_serialize_u16(&mut self, u16) -> Result<(), Error>;
    fn erased_serialize_u32(&mut self, u32) -> Result<(), Error>;
    fn erased_serialize_u64(&mut self, u64) -> Result<(), Error>;
    fn erased_serialize_f32(&mut self, f32) -> Result<(), Error>;
    fn erased_serialize_f64(&mut self, f64) -> Result<(), Error>;
    fn erased_serialize_char(&mut self, char) -> Result<(), Error>;
    fn erased_serialize_str(&mut self, &str) -> Result<(), Error>;
    fn erased_serialize_bytes(&mut self, &[u8]) -> Result<(), Error>;
    fn erased_serialize_unit(&mut self) -> Result<(), Error>;
    fn erased_serialize_unit_struct(&mut self, name: &'static str) -> Result<(), Error>;
    fn erased_serialize_unit_variant(&mut self, name: &'static str, variant_index: usize, variant: &'static str) -> Result<(), Error>;
    fn erased_serialize_newtype_struct(&mut self, name: &'static str, v: &Serialize) -> Result<(), Error>;
    fn erased_serialize_newtype_variant(&mut self, name: &'static str, variant_index: usize, variant: &'static str, v: &Serialize) -> Result<(), Error>;
    fn erased_serialize_none(&mut self) -> Result<(), Error>;
    fn erased_serialize_some(&mut self, v: &Serialize) -> Result<(), Error>;
    fn erased_serialize_seq(&mut self, len: Option<usize>) -> Result<SeqState, Error>;
    fn erased_serialize_seq_elt(&mut self, state: &mut SeqState, v: &Serialize) -> Result<(), Error>;
    fn erased_serialize_seq_end(&mut self, state: SeqState) -> Result<(), Error>;
    fn erased_serialize_seq_fixed_size(&mut self, size: usize) -> Result<SeqState, Error>;
    fn erased_serialize_tuple(&mut self, len: usize) -> Result<TupleState, Error>;
    fn erased_serialize_tuple_elt(&mut self, state: &mut TupleState, v: &Serialize) -> Result<(), Error>;
    fn erased_serialize_tuple_end(&mut self, state: TupleState) -> Result<(), Error>;
    fn erased_serialize_tuple_struct(&mut self, name: &'static str, len: usize) -> Result<TupleStructState, Error>;
    fn erased_serialize_tuple_struct_elt(&mut self, state: &mut TupleStructState, v: &Serialize) -> Result<(), Error>;
    fn erased_serialize_tuple_struct_end(&mut self, state: TupleStructState) -> Result<(), Error>;
    fn erased_serialize_tuple_variant(&mut self, name: &'static str, variant_index: usize, variant: &'static str, len: usize) -> Result<TupleVariantState, Error>;
    fn erased_serialize_tuple_variant_elt(&mut self, state: &mut TupleVariantState, v: &Serialize) -> Result<(), Error>;
    fn erased_serialize_tuple_variant_end(&mut self, state: TupleVariantState) -> Result<(), Error>;
    fn erased_serialize_map(&mut self, len: Option<usize>) -> Result<MapState, Error>;
    fn erased_serialize_map_key(&mut self, state: &mut MapState, k: &Serialize) -> Result<(), Error>;
    fn erased_serialize_map_value(&mut self, state: &mut MapState, v: &Serialize) -> Result<(), Error>;
    fn erased_serialize_map_end(&mut self, state: MapState) -> Result<(), Error>;
    fn erased_serialize_struct(&mut self, name: &'static str, len: usize) -> Result<StructState, Error>;
    fn erased_serialize_struct_elt(&mut self, state: &mut StructState, k: &'static str, v: &Serialize) -> Result<(), Error>;
    fn erased_serialize_struct_end(&mut self, state: StructState) -> Result<(), Error>;
    fn erased_serialize_struct_variant(&mut self, name: &'static str, variant_index: usize, variant: &'static str, len: usize) -> Result<StructVariantState, Error>;
    fn erased_serialize_struct_variant_elt(&mut self, state: &mut StructVariantState, k: &'static str, v: &Serialize) -> Result<(), Error>;
    fn erased_serialize_struct_variant_end(&mut self, state: StructVariantState) -> Result<(), Error>;
}

impl<T: ?Sized> Serialize for T
    where T: serde::Serialize
{
    fn erased_serialize(&self, mut serializer: &mut Serializer) -> Result<(), Error> {
        // This reference-to-a-reference is because Serialize::serialize<S>
        // requires S: Sized (for no reason).
        self.serialize(&mut serializer).map_err(erase)
    }
}

impl<'a> serde::Serialize for &'a Serialize {
    fn serialize<S>(&self, mut serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        (**self).erased_serialize(serializer).map_err(unerase)
    }
}

impl<T: ?Sized> Serializer for T
    where T: serde::Serializer
{
    fn erased_serialize_bool(&mut self, v: bool) -> Result<(), Error> {
        self.serialize_bool(v).map_err(erase)
    }
    fn erased_serialize_isize(&mut self, v: isize) -> Result<(), Error> {
        self.serialize_isize(v).map_err(erase)
    }
    fn erased_serialize_i8(&mut self, v: i8) -> Result<(), Error> {
        self.serialize_i8(v).map_err(erase)
    }
    fn erased_serialize_i16(&mut self, v: i16) -> Result<(), Error> {
        self.serialize_i16(v).map_err(erase)
    }
    fn erased_serialize_i32(&mut self, v: i32) -> Result<(), Error> {
        self.serialize_i32(v).map_err(erase)
    }
    fn erased_serialize_i64(&mut self, v: i64) -> Result<(), Error> {
        self.serialize_i64(v).map_err(erase)
    }
    fn erased_serialize_usize(&mut self, v: usize) -> Result<(), Error> {
        self.serialize_usize(v).map_err(erase)
    }
    fn erased_serialize_u8(&mut self, v: u8) -> Result<(), Error> {
        self.serialize_u8(v).map_err(erase)
    }
    fn erased_serialize_u16(&mut self, v: u16) -> Result<(), Error> {
        self.serialize_u16(v).map_err(erase)
    }
    fn erased_serialize_u32(&mut self, v: u32) -> Result<(), Error> {
        self.serialize_u32(v).map_err(erase)
    }
    fn erased_serialize_u64(&mut self, v: u64) -> Result<(), Error> {
        self.serialize_u64(v).map_err(erase)
    }
    fn erased_serialize_f32(&mut self, v: f32) -> Result<(), Error> {
        self.serialize_f32(v).map_err(erase)
    }
    fn erased_serialize_f64(&mut self, v: f64) -> Result<(), Error> {
        self.serialize_f64(v).map_err(erase)
    }
    fn erased_serialize_char(&mut self, v: char) -> Result<(), Error> {
        self.serialize_char(v).map_err(erase)
    }
    fn erased_serialize_str(&mut self, v: &str) -> Result<(), Error> {
        self.serialize_str(v).map_err(erase)
    }
    fn erased_serialize_bytes(&mut self, v: &[u8]) -> Result<(), Error> {
        self.serialize_bytes(v).map_err(erase)
    }
    fn erased_serialize_unit(&mut self) -> Result<(), Error> {
        self.serialize_unit().map_err(erase)
    }
    fn erased_serialize_unit_struct(&mut self, name: &'static str) -> Result<(), Error> {
        self.serialize_unit_struct(name).map_err(erase)
    }
    fn erased_serialize_unit_variant(&mut self, name: &'static str, variant_index: usize, variant: &'static str) -> Result<(), Error> {
        self.serialize_unit_variant(name, variant_index, variant).map_err(erase)
    }
    fn erased_serialize_newtype_struct(&mut self, name: &'static str, v: &Serialize) -> Result<(), Error> {
        self.serialize_newtype_struct(name, v).map_err(erase)
    }
    fn erased_serialize_newtype_variant(&mut self, name: &'static str, variant_index: usize, variant: &'static str, v: &Serialize) -> Result<(), Error> {
        self.serialize_newtype_variant(name, variant_index, variant, v).map_err(erase)
    }
    fn erased_serialize_none(&mut self) -> Result<(), Error> {
        self.serialize_none().map_err(erase)
    }
    fn erased_serialize_some(&mut self, v: &Serialize) -> Result<(), Error> {
        self.serialize_some(v).map_err(erase)
    }
    fn erased_serialize_seq(&mut self, len: Option<usize>) -> Result<SeqState, Error> {
        self.serialize_seq(len).map(SeqState::new).map_err(erase)
    }
    fn erased_serialize_seq_elt(&mut self, state: &mut SeqState, v: &Serialize) -> Result<(), Error> {
        self.serialize_seq_elt(state.view(), v).map_err(erase)
    }
    fn erased_serialize_seq_end(&mut self, state: SeqState) -> Result<(), Error> {
        self.serialize_seq_end(state.take()).map_err(erase)
    }
    fn erased_serialize_seq_fixed_size(&mut self, size: usize) -> Result<SeqState, Error> {
        self.serialize_seq_fixed_size(size).map(SeqState::new).map_err(erase)
    }
    fn erased_serialize_tuple(&mut self, len: usize) -> Result<TupleState, Error> {
        self.serialize_tuple(len).map(TupleState::new).map_err(erase)
    }
    fn erased_serialize_tuple_elt(&mut self, state: &mut TupleState, v: &Serialize) -> Result<(), Error> {
        self.serialize_tuple_elt(state.view(), v).map_err(erase)
    }
    fn erased_serialize_tuple_end(&mut self, state: TupleState) -> Result<(), Error> {
        self.serialize_tuple_end(state.take()).map_err(erase)
    }
    fn erased_serialize_tuple_struct(&mut self, name: &'static str, len: usize) -> Result<TupleStructState, Error> {
        self.serialize_tuple_struct(name, len).map(TupleStructState::new).map_err(erase)
    }
    fn erased_serialize_tuple_struct_elt(&mut self, state: &mut TupleStructState, v: &Serialize) -> Result<(), Error> {
        self.serialize_tuple_struct_elt(state.view(), v).map_err(erase)
    }
    fn erased_serialize_tuple_struct_end(&mut self, state: TupleStructState) -> Result<(), Error> {
        self.serialize_tuple_struct_end(state.take()).map_err(erase)
    }
    fn erased_serialize_tuple_variant(&mut self, name: &'static str, variant_index: usize, variant: &'static str, len: usize) -> Result<TupleVariantState, Error> {
        self.serialize_tuple_variant(name, variant_index, variant, len).map(TupleVariantState::new).map_err(erase)
    }
    fn erased_serialize_tuple_variant_elt(&mut self, state: &mut TupleVariantState, v: &Serialize) -> Result<(), Error> {
        self.serialize_tuple_variant_elt(state.view(), v).map_err(erase)
    }
    fn erased_serialize_tuple_variant_end(&mut self, state: TupleVariantState) -> Result<(), Error> {
        self.serialize_tuple_variant_end(state.take()).map_err(erase)
    }
    fn erased_serialize_map(&mut self, len: Option<usize>) -> Result<MapState, Error> {
        self.serialize_map(len).map(MapState::new).map_err(erase)
    }
    fn erased_serialize_map_key(&mut self, state: &mut MapState, k: &Serialize) -> Result<(), Error> {
        self.serialize_map_key(state.view(), k).map_err(erase)
    }
    fn erased_serialize_map_value(&mut self, state: &mut MapState, v: &Serialize) -> Result<(), Error> {
        self.serialize_map_value(state.view(), v).map_err(erase)
    }
    fn erased_serialize_map_end(&mut self, state: MapState) -> Result<(), Error> {
        self.serialize_map_end(state.take()).map_err(erase)
    }
    fn erased_serialize_struct(&mut self, name: &'static str, len: usize) -> Result<StructState, Error> {
        self.serialize_struct(name, len).map(StructState::new).map_err(erase)
    }
    fn erased_serialize_struct_elt(&mut self, state: &mut StructState, k: &'static str, v: &Serialize) -> Result<(), Error> {
        self.serialize_struct_elt(state.view(), k, v).map_err(erase)
    }
    fn erased_serialize_struct_end(&mut self, state: StructState) -> Result<(), Error> {
        self.serialize_struct_end(state.take()).map_err(erase)
    }
    fn erased_serialize_struct_variant(&mut self, name: &'static str, variant_index: usize, variant: &'static str, len: usize) -> Result<StructVariantState, Error> {
        self.serialize_struct_variant(name, variant_index, variant, len).map(StructVariantState::new).map_err(erase)
    }
    fn erased_serialize_struct_variant_elt(&mut self, state: &mut StructVariantState, k: &'static str, v: &Serialize) -> Result<(), Error> {
        self.serialize_struct_variant_elt(state.view(), k, v).map_err(erase)
    }
    fn erased_serialize_struct_variant_end(&mut self, state: StructVariantState) -> Result<(), Error> {
        self.serialize_struct_variant_end(state.take()).map_err(erase)
    }
}

macro_rules! state_any {
    ($name:ident) => {
        pub struct $name(Any);
        impl $name {
            fn new<T>(t: T) -> Self {
                $name(Any::new(t))
            }
            fn view<T>(&mut self) -> &mut T {
                self.0.view()
            }
            fn take<T>(self) -> T {
                self.0.take()
            }
        }
    }
}

// These are different types instead of all just Any in order to prevent
// mistakenly passing the wrong one which would lead to memory corruption.
state_any!(SeqState);
state_any!(TupleState);
state_any!(TupleStructState);
state_any!(TupleVariantState);
state_any!(MapState);
state_any!(StructState);
state_any!(StructVariantState);

impl<'a> serde::Serializer for &'a mut Serializer {
    type Error = Error;
    type SeqState = SeqState;
    type TupleState = TupleState;
    type TupleStructState = TupleStructState;
    type TupleVariantState = TupleVariantState;
    type MapState = MapState;
    type StructState = StructState;
    type StructVariantState = StructVariantState;
    fn serialize_bool(&mut self, v: bool) -> Result<(), Error> {
        (**self).erased_serialize_bool(v).map_err(erase)
    }
    fn serialize_isize(&mut self, v: isize) -> Result<(), Error> {
        (**self).erased_serialize_isize(v).map_err(erase)
    }
    fn serialize_i8(&mut self, v: i8) -> Result<(), Error> {
        (**self).erased_serialize_i8(v).map_err(erase)
    }
    fn serialize_i16(&mut self, v: i16) -> Result<(), Error> {
        (**self).erased_serialize_i16(v).map_err(erase)
    }
    fn serialize_i32(&mut self, v: i32) -> Result<(), Error> {
        (**self).erased_serialize_i32(v).map_err(erase)
    }
    fn serialize_i64(&mut self, v: i64) -> Result<(), Error> {
        (**self).erased_serialize_i64(v).map_err(erase)
    }
    fn serialize_usize(&mut self, v: usize) -> Result<(), Error> {
        (**self).erased_serialize_usize(v).map_err(erase)
    }
    fn serialize_u8(&mut self, v: u8) -> Result<(), Error> {
        (**self).erased_serialize_u8(v).map_err(erase)
    }
    fn serialize_u16(&mut self, v: u16) -> Result<(), Error> {
        (**self).erased_serialize_u16(v).map_err(erase)
    }
    fn serialize_u32(&mut self, v: u32) -> Result<(), Error> {
        (**self).erased_serialize_u32(v).map_err(erase)
    }
    fn serialize_u64(&mut self, v: u64) -> Result<(), Error> {
        (**self).erased_serialize_u64(v).map_err(erase)
    }
    fn serialize_f32(&mut self, v: f32) -> Result<(), Error> {
        (**self).erased_serialize_f32(v).map_err(erase)
    }
    fn serialize_f64(&mut self, v: f64) -> Result<(), Error> {
        (**self).erased_serialize_f64(v).map_err(erase)
    }
    fn serialize_char(&mut self, v: char) -> Result<(), Error> {
        (**self).erased_serialize_char(v).map_err(erase)
    }
    fn serialize_str(&mut self, v: &str) -> Result<(), Error> {
        (**self).erased_serialize_str(v).map_err(erase)
    }
    fn serialize_bytes(&mut self, v: &[u8]) -> Result<(), Error> {
        (**self).erased_serialize_bytes(v).map_err(erase)
    }
    fn serialize_unit(&mut self) -> Result<(), Error> {
        (**self).erased_serialize_unit().map_err(erase)
    }
    fn serialize_unit_struct(&mut self, name: &'static str) -> Result<(), Error> {
        (**self).erased_serialize_unit_struct(name).map_err(erase)
    }
    fn serialize_unit_variant(&mut self, name: &'static str, variant_index: usize, variant: &'static str) -> Result<(), Error> {
        (**self).erased_serialize_unit_variant(name, variant_index, variant).map_err(erase)
    }
    fn serialize_newtype_struct<T: serde::Serialize>(&mut self, name: &'static str, v: T) -> Result<(), Error> {
        (**self).erased_serialize_newtype_struct(name, &v).map_err(erase)
    }
    fn serialize_newtype_variant<T: serde::Serialize>(&mut self, name: &'static str, variant_index: usize, variant: &'static str, v: T) -> Result<(), Error> {
        (**self).erased_serialize_newtype_variant(name, variant_index, variant, &v).map_err(erase)
    }
    fn serialize_none(&mut self) -> Result<(), Error> {
        (**self).erased_serialize_none().map_err(erase)
    }
    fn serialize_some<T: serde::Serialize>(&mut self, v: T) -> Result<(), Error> {
        (**self).erased_serialize_some(&v).map_err(erase)
    }
    fn serialize_seq(&mut self, len: Option<usize>) -> Result<Self::SeqState, Error> {
        (**self).erased_serialize_seq(len).map_err(erase)
    }
    fn serialize_seq_elt<T: serde::Serialize>(&mut self, state: &mut Self::SeqState, v: T) -> Result<(), Error> {
        (**self).erased_serialize_seq_elt(state, &v).map_err(erase)
    }
    fn serialize_seq_end(&mut self, state: Self::SeqState) -> Result<(), Error> {
        (**self).erased_serialize_seq_end(state).map_err(erase)
    }
    fn serialize_seq_fixed_size(&mut self, size: usize) -> Result<Self::SeqState, Error> {
        (**self).erased_serialize_seq_fixed_size(size).map_err(erase)
    }
    fn serialize_tuple(&mut self, len: usize) -> Result<Self::TupleState, Error> {
        (**self).erased_serialize_tuple(len).map_err(erase)
    }
    fn serialize_tuple_elt<T: serde::Serialize>(&mut self, state: &mut Self::TupleState, v: T) -> Result<(), Error> {
        (**self).erased_serialize_tuple_elt(state, &v).map_err(erase)
    }
    fn serialize_tuple_end(&mut self, state: Self::TupleState) -> Result<(), Error> {
        (**self).erased_serialize_tuple_end(state).map_err(erase)
    }
    fn serialize_tuple_struct(&mut self, name: &'static str, len: usize) -> Result<Self::TupleStructState, Error> {
        (**self).erased_serialize_tuple_struct(name, len).map_err(erase)
    }
    fn serialize_tuple_struct_elt<T: serde::Serialize>(&mut self, state: &mut Self::TupleStructState, v: T) -> Result<(), Error> {
        (**self).erased_serialize_tuple_struct_elt(state, &v).map_err(erase)
    }
    fn serialize_tuple_struct_end(&mut self, state: Self::TupleStructState) -> Result<(), Error> {
        (**self).erased_serialize_tuple_struct_end(state).map_err(erase)
    }
    fn serialize_tuple_variant(&mut self, name: &'static str, variant_index: usize, variant: &'static str, len: usize) -> Result<Self::TupleVariantState, Error> {
        (**self).erased_serialize_tuple_variant(name, variant_index, variant, len).map_err(erase)
    }
    fn serialize_tuple_variant_elt<T: serde::Serialize>(&mut self, state: &mut Self::TupleVariantState, v: T) -> Result<(), Error> {
        (**self).erased_serialize_tuple_variant_elt(state, &v).map_err(erase)
    }
    fn serialize_tuple_variant_end(&mut self, state: Self::TupleVariantState) -> Result<(), Error> {
        (**self).erased_serialize_tuple_variant_end(state).map_err(erase)
    }
    fn serialize_map(&mut self, len: Option<usize>) -> Result<Self::MapState, Error> {
        (**self).erased_serialize_map(len).map_err(erase)
    }
    fn serialize_map_key<T: serde::Serialize>(&mut self, state: &mut Self::MapState, k: T) -> Result<(), Error> {
        (**self).erased_serialize_map_key(state, &k).map_err(erase)
    }
    fn serialize_map_value<T: serde::Serialize>(&mut self, state: &mut Self::MapState, v: T) -> Result<(), Error> {
        (**self).erased_serialize_map_value(state, &v).map_err(erase)
    }
    fn serialize_map_end(&mut self, state: Self::MapState) -> Result<(), Error> {
        (**self).erased_serialize_map_end(state).map_err(erase)
    }
    fn serialize_struct(&mut self, name: &'static str, len: usize) -> Result<Self::StructState, Error> {
        (**self).erased_serialize_struct(name, len).map_err(erase)
    }
    fn serialize_struct_elt<V: serde::Serialize>(&mut self, state: &mut Self::StructState, k: &'static str, v: V) -> Result<(), Error> {
        (**self).erased_serialize_struct_elt(state, k, &v).map_err(erase)
    }
    fn serialize_struct_end(&mut self, state: Self::StructState) -> Result<(), Error> {
        (**self).erased_serialize_struct_end(state).map_err(erase)
    }
    fn serialize_struct_variant(&mut self, name: &'static str, variant_index: usize, variant: &'static str, len: usize) -> Result<Self::StructVariantState, Error> {
        (**self).erased_serialize_struct_variant(name, variant_index, variant, len).map_err(erase)
    }
    fn serialize_struct_variant_elt<V: serde::Serialize>(&mut self, state: &mut Self::StructVariantState, k: &'static str, v: V) -> Result<(), Error> {
        (**self).erased_serialize_struct_variant_elt(state, k, &v).map_err(erase)
    }
    fn serialize_struct_variant_end(&mut self, state: Self::StructVariantState) -> Result<(), Error> {
        (**self).erased_serialize_struct_variant_end(state).map_err(erase)
    }
}

fn erase<E>(e: E) -> Error
    where E: Display
{
    Error::new(e.to_string())
}

fn unerase<E>(e: Error) -> E
    where E: serde::ser::Error
{
    use std::error::Error;
    E::custom(e.description())
}

#[test]
fn trait_object() {
    extern crate serde_json;

    let obj: &Serialize = &vec!["a", "b"];

    let mut buf = Vec::new();

    {
        let mut ser = serde_json::Serializer::new(&mut buf);
        let ser: &mut Serializer = &mut ser;

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
        let ser = serde_json::Serializer::new(&mut buf);
        let mut ser: Box<Serializer> = Box::new(ser);

        obj.erased_serialize(&mut *ser).unwrap();
    }

    assert_eq!(&buf, br#"["a","b"]"#);
}
