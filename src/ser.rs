use crate::any::Any;
use crate::error::{erase_ser as erase, unerase_ser as unerase, Error};
use crate::map::ResultExt;
use crate::sealed;
use alloc::boxed::Box;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};

// TRAITS //////////////////////////////////////////////////////////////////////

/// An object-safe equivalent of Serde's `Serialize` trait.
///
/// Any implementation of Serde's `Serialize` converts seamlessly to an
/// `&erased_serde::Serialize` or `Box<erased_serde::Serialize>` trait object.
///
/// ```rust
/// use erased_serde::{Serialize, Serializer};
/// use std::collections::BTreeMap as Map;
/// use std::io;
///
/// fn main() {
///     // Construct some serializers.
///     let json = &mut serde_json::Serializer::new(io::stdout());
///     let cbor = &mut serde_cbor::Serializer::new(serde_cbor::ser::IoWrite::new(io::stdout()));
///
///     // The values in this map are boxed trait objects. Ordinarily this would not
///     // be possible with serde::Serializer because of object safety, but type
///     // erasure makes it possible with erased_serde::Serializer.
///     let mut formats: Map<&str, Box<dyn Serializer>> = Map::new();
///     formats.insert("json", Box::new(<dyn Serializer>::erase(json)));
///     formats.insert("cbor", Box::new(<dyn Serializer>::erase(cbor)));
///
///     // These are boxed trait objects as well. Same thing here - type erasure
///     // makes this possible.
///     let mut values: Map<&str, Box<dyn Serialize>> = Map::new();
///     values.insert("vec", Box::new(vec!["a", "b"]));
///     values.insert("int", Box::new(65536));
///
///     // Pick a Serializer out of the formats map.
///     let format = formats.get_mut("json").unwrap();
///
///     // Pick a Serialize out of the values map.
///     let value = values.get("vec").unwrap();
///
///     // This line prints `["a","b"]` to stdout.
///     value.erased_serialize(format).unwrap();
/// }
/// ```
///
/// This trait is sealed and can only be implemented via a `serde::Serialize`
/// impl.
pub trait Serialize: sealed::serialize::Sealed {
    fn erased_serialize(&self, serializer: &mut dyn Serializer) -> Result<(), Error>;

    #[doc(hidden)]
    fn do_erased_serialize(
        &self,
        serializer: &mut dyn Serializer,
        out: &mut MaybeUninit<Result<Ok, Error>>,
    );
}

/// An object-safe equivalent of Serde's `Serializer` trait.
///
/// Any implementation of Serde's `Serializer` can be converted to an
/// `&erased_serde::Serializer` or `Box<erased_serde::Serializer>` trait object
/// using `erased_serde::Serializer::erase`.
///
/// ```rust
/// use erased_serde::{Serialize, Serializer};
/// use std::collections::BTreeMap as Map;
/// use std::io;
///
/// fn main() {
///     // Construct some serializers.
///     let json = &mut serde_json::Serializer::new(io::stdout());
///     let cbor = &mut serde_cbor::Serializer::new(serde_cbor::ser::IoWrite::new(io::stdout()));
///
///     // The values in this map are boxed trait objects. Ordinarily this would not
///     // be possible with serde::Serializer because of object safety, but type
///     // erasure makes it possible with erased_serde::Serializer.
///     let mut formats: Map<&str, Box<dyn Serializer>> = Map::new();
///     formats.insert("json", Box::new(<dyn Serializer>::erase(json)));
///     formats.insert("cbor", Box::new(<dyn Serializer>::erase(cbor)));
///
///     // These are boxed trait objects as well. Same thing here - type erasure
///     // makes this possible.
///     let mut values: Map<&str, Box<dyn Serialize>> = Map::new();
///     values.insert("vec", Box::new(vec!["a", "b"]));
///     values.insert("int", Box::new(65536));
///
///     // Pick a Serializer out of the formats map.
///     let format = formats.get_mut("json").unwrap();
///
///     // Pick a Serialize out of the values map.
///     let value = values.get("vec").unwrap();
///
///     // This line prints `["a","b"]` to stdout.
///     value.erased_serialize(format).unwrap();
/// }
/// ```
///
/// This trait is sealed and can only be implemented via a `serde::Serializer`
/// impl.
pub trait Serializer: sealed::serializer::Sealed {
    fn erased_serialize_bool(&mut self, v: bool, out: &mut MaybeUninit<Result<Ok, Error>>);
    fn erased_serialize_i8(&mut self, v: i8, out: &mut MaybeUninit<Result<Ok, Error>>);
    fn erased_serialize_i16(&mut self, v: i16, out: &mut MaybeUninit<Result<Ok, Error>>);
    fn erased_serialize_i32(&mut self, v: i32, out: &mut MaybeUninit<Result<Ok, Error>>);
    fn erased_serialize_i64(&mut self, v: i64, out: &mut MaybeUninit<Result<Ok, Error>>);
    fn erased_serialize_i128(&mut self, v: i128, out: &mut MaybeUninit<Result<Ok, Error>>);
    fn erased_serialize_u8(&mut self, v: u8, out: &mut MaybeUninit<Result<Ok, Error>>);
    fn erased_serialize_u16(&mut self, v: u16, out: &mut MaybeUninit<Result<Ok, Error>>);
    fn erased_serialize_u32(&mut self, v: u32, out: &mut MaybeUninit<Result<Ok, Error>>);
    fn erased_serialize_u64(&mut self, v: u64, out: &mut MaybeUninit<Result<Ok, Error>>);
    fn erased_serialize_u128(&mut self, v: u128, out: &mut MaybeUninit<Result<Ok, Error>>);
    fn erased_serialize_f32(&mut self, v: f32, out: &mut MaybeUninit<Result<Ok, Error>>);
    fn erased_serialize_f64(&mut self, v: f64, out: &mut MaybeUninit<Result<Ok, Error>>);
    fn erased_serialize_char(&mut self, v: char, out: &mut MaybeUninit<Result<Ok, Error>>);
    fn erased_serialize_str(&mut self, v: &str, out: &mut MaybeUninit<Result<Ok, Error>>);
    fn erased_serialize_bytes(&mut self, v: &[u8], out: &mut MaybeUninit<Result<Ok, Error>>);
    fn erased_serialize_none(&mut self, out: &mut MaybeUninit<Result<Ok, Error>>);
    fn erased_serialize_some(
        &mut self,
        value: &dyn Serialize,
        out: &mut MaybeUninit<Result<Ok, Error>>,
    );
    fn erased_serialize_unit(&mut self, out: &mut MaybeUninit<Result<Ok, Error>>);
    fn erased_serialize_unit_struct(
        &mut self,
        name: &'static str,
        out: &mut MaybeUninit<Result<Ok, Error>>,
    );
    fn erased_serialize_unit_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        out: &mut MaybeUninit<Result<Ok, Error>>,
    );
    fn erased_serialize_newtype_struct(
        &mut self,
        name: &'static str,
        value: &dyn Serialize,
        out: &mut MaybeUninit<Result<Ok, Error>>,
    );
    fn erased_serialize_newtype_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &dyn Serialize,
        out: &mut MaybeUninit<Result<Ok, Error>>,
    );
    fn erased_serialize_seq(
        &mut self,
        len: Option<usize>,
        out: &mut MaybeUninit<Result<Seq, Error>>,
    );
    fn erased_serialize_tuple(&mut self, len: usize, out: &mut MaybeUninit<Result<Tuple, Error>>);
    fn erased_serialize_tuple_struct(
        &mut self,
        name: &'static str,
        len: usize,
        out: &mut MaybeUninit<Result<TupleStruct, Error>>,
    );
    fn erased_serialize_tuple_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
        out: &mut MaybeUninit<Result<TupleVariant, Error>>,
    );
    fn erased_serialize_map(
        &mut self,
        len: Option<usize>,
        out: &mut MaybeUninit<Result<Map, Error>>,
    );
    fn erased_serialize_struct(
        &mut self,
        name: &'static str,
        len: usize,
        out: &mut MaybeUninit<Result<Struct, Error>>,
    );
    fn erased_serialize_struct_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
        out: &mut MaybeUninit<Result<StructVariant, Error>>,
    );
    fn erased_is_human_readable(&self) -> bool;
}

impl dyn Serializer {
    /// Convert any Serde `Serializer` to a trait object.
    ///
    /// ```rust
    /// use erased_serde::{Serialize, Serializer};
    /// use std::collections::BTreeMap as Map;
    /// use std::io;
    ///
    /// fn main() {
    ///     // Construct some serializers.
    ///     let json = &mut serde_json::Serializer::new(io::stdout());
    ///     let cbor = &mut serde_cbor::Serializer::new(serde_cbor::ser::IoWrite::new(io::stdout()));
    ///
    ///     // The values in this map are boxed trait objects. Ordinarily this would not
    ///     // be possible with serde::Serializer because of object safety, but type
    ///     // erasure makes it possible with erased_serde::Serializer.
    ///     let mut formats: Map<&str, Box<dyn Serializer>> = Map::new();
    ///     formats.insert("json", Box::new(<dyn Serializer>::erase(json)));
    ///     formats.insert("cbor", Box::new(<dyn Serializer>::erase(cbor)));
    ///
    ///     // These are boxed trait objects as well. Same thing here - type erasure
    ///     // makes this possible.
    ///     let mut values: Map<&str, Box<dyn Serialize>> = Map::new();
    ///     values.insert("vec", Box::new(vec!["a", "b"]));
    ///     values.insert("int", Box::new(65536));
    ///
    ///     // Pick a Serializer out of the formats map.
    ///     let format = formats.get_mut("json").unwrap();
    ///
    ///     // Pick a Serialize out of the values map.
    ///     let value = values.get("vec").unwrap();
    ///
    ///     // This line prints `["a","b"]` to stdout.
    ///     value.erased_serialize(format).unwrap();
    /// }
    /// ```
    pub fn erase<S>(serializer: S) -> erase::Serializer<S>
    where
        S: serde::Serializer,
        S::Ok: 'static,
    {
        erase::Serializer::new(serializer)
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
    unsafe fn new<T>(t: T) -> Self {
        Ok {
            data: unsafe { Any::new(t) },
        }
    }

    unsafe fn take<T>(self) -> T {
        unsafe { self.data.take() }
    }
}

// IMPL ERASED SERDE FOR SERDE /////////////////////////////////////////////////

impl<T> Serialize for T
where
    T: ?Sized + serde::Serialize,
{
    fn erased_serialize(&self, serializer: &mut dyn Serializer) -> Result<(), Error> {
        self.serialize(MakeSerializer(serializer)).map(drop)
    }

    fn do_erased_serialize(
        &self,
        serializer: &mut dyn Serializer,
        out: &mut MaybeUninit<Result<Ok, Error>>,
    ) {
        out.write(self.serialize(MakeSerializer(serializer)));
    }
}

impl<T> sealed::serialize::Sealed for T where T: ?Sized + serde::Serialize {}

mod erase {
    use core::mem;

    pub enum Serializer<S> {
        Ready(S),
        Consumed,
    }

    impl<S> Serializer<S> {
        pub(crate) fn new(serializer: S) -> Self {
            Serializer::Ready(serializer)
        }

        pub(crate) fn take(&mut self) -> S {
            match mem::replace(self, Serializer::Consumed) {
                Serializer::Ready(serializer) => serializer,
                Serializer::Consumed => panic!(),
            }
        }

        pub(crate) fn as_ref(&self) -> &S {
            match self {
                Serializer::Ready(serializer) => serializer,
                Serializer::Consumed => panic!(),
            }
        }
    }
}

impl<T> Serializer for erase::Serializer<T>
where
    T: serde::Serializer,
{
    fn erased_serialize_bool(&mut self, v: bool, out: &mut MaybeUninit<Result<Ok, Error>>) {
        out.write(unsafe {
            self.take()
                .serialize_bool(v)
                .unsafe_map(Ok::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_i8(&mut self, v: i8, out: &mut MaybeUninit<Result<Ok, Error>>) {
        out.write(unsafe {
            self.take()
                .serialize_i8(v)
                .unsafe_map(Ok::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_i16(&mut self, v: i16, out: &mut MaybeUninit<Result<Ok, Error>>) {
        out.write(unsafe {
            self.take()
                .serialize_i16(v)
                .unsafe_map(Ok::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_i32(&mut self, v: i32, out: &mut MaybeUninit<Result<Ok, Error>>) {
        out.write(unsafe {
            self.take()
                .serialize_i32(v)
                .unsafe_map(Ok::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_i64(&mut self, v: i64, out: &mut MaybeUninit<Result<Ok, Error>>) {
        out.write(unsafe {
            self.take()
                .serialize_i64(v)
                .unsafe_map(Ok::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_i128(&mut self, v: i128, out: &mut MaybeUninit<Result<Ok, Error>>) {
        out.write(unsafe {
            self.take()
                .serialize_i128(v)
                .unsafe_map(Ok::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_u8(&mut self, v: u8, out: &mut MaybeUninit<Result<Ok, Error>>) {
        out.write(unsafe {
            self.take()
                .serialize_u8(v)
                .unsafe_map(Ok::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_u16(&mut self, v: u16, out: &mut MaybeUninit<Result<Ok, Error>>) {
        out.write(unsafe {
            self.take()
                .serialize_u16(v)
                .unsafe_map(Ok::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_u32(&mut self, v: u32, out: &mut MaybeUninit<Result<Ok, Error>>) {
        out.write(unsafe {
            self.take()
                .serialize_u32(v)
                .unsafe_map(Ok::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_u64(&mut self, v: u64, out: &mut MaybeUninit<Result<Ok, Error>>) {
        out.write(unsafe {
            self.take()
                .serialize_u64(v)
                .unsafe_map(Ok::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_u128(&mut self, v: u128, out: &mut MaybeUninit<Result<Ok, Error>>) {
        out.write(unsafe {
            self.take()
                .serialize_u128(v)
                .unsafe_map(Ok::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_f32(&mut self, v: f32, out: &mut MaybeUninit<Result<Ok, Error>>) {
        out.write(unsafe {
            self.take()
                .serialize_f32(v)
                .unsafe_map(Ok::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_f64(&mut self, v: f64, out: &mut MaybeUninit<Result<Ok, Error>>) {
        out.write(unsafe {
            self.take()
                .serialize_f64(v)
                .unsafe_map(Ok::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_char(&mut self, v: char, out: &mut MaybeUninit<Result<Ok, Error>>) {
        out.write(unsafe {
            self.take()
                .serialize_char(v)
                .unsafe_map(Ok::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_str(&mut self, v: &str, out: &mut MaybeUninit<Result<Ok, Error>>) {
        out.write(unsafe {
            self.take()
                .serialize_str(v)
                .unsafe_map(Ok::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_bytes(&mut self, v: &[u8], out: &mut MaybeUninit<Result<Ok, Error>>) {
        out.write(unsafe {
            self.take()
                .serialize_bytes(v)
                .unsafe_map(Ok::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_none(&mut self, out: &mut MaybeUninit<Result<Ok, Error>>) {
        out.write(unsafe {
            self.take()
                .serialize_none()
                .unsafe_map(Ok::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_some(
        &mut self,
        value: &dyn Serialize,
        out: &mut MaybeUninit<Result<Ok, Error>>,
    ) {
        out.write(unsafe {
            self.take()
                .serialize_some(value)
                .unsafe_map(Ok::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_unit(&mut self, out: &mut MaybeUninit<Result<Ok, Error>>) {
        out.write(unsafe {
            self.take()
                .serialize_unit()
                .unsafe_map(Ok::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_unit_struct(
        &mut self,
        name: &'static str,
        out: &mut MaybeUninit<Result<Ok, Error>>,
    ) {
        out.write(unsafe {
            self.take()
                .serialize_unit_struct(name)
                .unsafe_map(Ok::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_unit_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        out: &mut MaybeUninit<Result<Ok, Error>>,
    ) {
        out.write(unsafe {
            self.take()
                .serialize_unit_variant(name, variant_index, variant)
                .unsafe_map(Ok::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_newtype_struct(
        &mut self,
        name: &'static str,
        value: &dyn Serialize,
        out: &mut MaybeUninit<Result<Ok, Error>>,
    ) {
        out.write(unsafe {
            self.take()
                .serialize_newtype_struct(name, value)
                .unsafe_map(Ok::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_newtype_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &dyn Serialize,
        out: &mut MaybeUninit<Result<Ok, Error>>,
    ) {
        out.write(unsafe {
            self.take()
                .serialize_newtype_variant(name, variant_index, variant, value)
                .unsafe_map(Ok::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_seq(
        &mut self,
        len: Option<usize>,
        out: &mut MaybeUninit<Result<Seq, Error>>,
    ) {
        out.write(unsafe {
            self.take()
                .serialize_seq(len)
                .unsafe_map(Seq::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_tuple(&mut self, len: usize, out: &mut MaybeUninit<Result<Tuple, Error>>) {
        out.write(unsafe {
            self.take()
                .serialize_tuple(len)
                .unsafe_map(Tuple::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_tuple_struct(
        &mut self,
        name: &'static str,
        len: usize,
        out: &mut MaybeUninit<Result<TupleStruct, Error>>,
    ) {
        out.write(unsafe {
            self.take()
                .serialize_tuple_struct(name, len)
                .unsafe_map(TupleStruct::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_tuple_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
        out: &mut MaybeUninit<Result<TupleVariant, Error>>,
    ) {
        out.write(unsafe {
            self.take()
                .serialize_tuple_variant(name, variant_index, variant, len)
                .unsafe_map(TupleVariant::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_map(
        &mut self,
        len: Option<usize>,
        out: &mut MaybeUninit<Result<Map, Error>>,
    ) {
        out.write(unsafe {
            self.take()
                .serialize_map(len)
                .unsafe_map(Map::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_struct(
        &mut self,
        name: &'static str,
        len: usize,
        out: &mut MaybeUninit<Result<Struct, Error>>,
    ) {
        out.write(unsafe {
            self.take()
                .serialize_struct(name, len)
                .unsafe_map(Struct::new)
                .map_err(erase)
        });
    }

    fn erased_serialize_struct_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
        out: &mut MaybeUninit<Result<StructVariant, Error>>,
    ) {
        out.write(unsafe {
            self.take()
                .serialize_struct_variant(name, variant_index, variant, len)
                .unsafe_map(StructVariant::new)
                .map_err(erase)
        });
    }

    fn erased_is_human_readable(&self) -> bool {
        self.as_ref().is_human_readable()
    }
}

impl<T> sealed::serializer::Sealed for erase::Serializer<T> where T: serde::Serializer {}

// IMPL SERDE FOR ERASED SERDE /////////////////////////////////////////////////

/// Serialize the given type-erased serializable value.
///
/// This can be used to implement `serde::Serialize` for trait objects that have
/// `erased_serde::Serialize` as a supertrait.
///
/// ```
/// trait Event: erased_serde::Serialize {
///     /* ... */
/// }
///
/// impl<'a> serde::Serialize for dyn Event + 'a {
///     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
///     where
///         S: serde::Serializer,
///     {
///         erased_serde::serialize(self, serializer)
///     }
/// }
/// ```
///
/// Since this is reasonably common, the `serialize_trait_object!` macro
/// generates such a Serialize impl.
///
/// ```
/// use erased_serde::serialize_trait_object;
/// #
/// # trait Event: erased_serde::Serialize {}
///
/// serialize_trait_object!(Event);
/// ```
pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: ?Sized + Serialize,
    S: serde::Serializer,
{
    let mut erased = erase::Serializer::new(serializer);
    let mut out = MaybeUninit::uninit();
    value.do_erased_serialize(&mut erased, &mut out);
    unsafe { out.assume_init().unsafe_map(Ok::take).map_err(unerase) }
}

serialize_trait_object!(Serialize);

struct MakeSerializer<'a>(&'a mut dyn Serializer);

impl<'a> serde::Serializer for MakeSerializer<'a> {
    type Ok = Ok;
    type Error = Error;
    type SerializeSeq = Seq<'a>;
    type SerializeTuple = Tuple<'a>;
    type SerializeTupleStruct = TupleStruct<'a>;
    type SerializeTupleVariant = TupleVariant<'a>;
    type SerializeMap = Map<'a>;
    type SerializeStruct = Struct<'a>;
    type SerializeStructVariant = StructVariant<'a>;

    fn serialize_bool(self, v: bool) -> Result<Ok, Error> {
        let mut out = MaybeUninit::uninit();
        self.0.erased_serialize_bool(v, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_i8(self, v: i8) -> Result<Ok, Error> {
        let mut out = MaybeUninit::uninit();
        self.0.erased_serialize_i8(v, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_i16(self, v: i16) -> Result<Ok, Error> {
        let mut out = MaybeUninit::uninit();
        self.0.erased_serialize_i16(v, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_i32(self, v: i32) -> Result<Ok, Error> {
        let mut out = MaybeUninit::uninit();
        self.0.erased_serialize_i32(v, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_i64(self, v: i64) -> Result<Ok, Error> {
        let mut out = MaybeUninit::uninit();
        self.0.erased_serialize_i64(v, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_i128(self, v: i128) -> Result<Ok, Error> {
        let mut out = MaybeUninit::uninit();
        self.0.erased_serialize_i128(v, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_u8(self, v: u8) -> Result<Ok, Error> {
        let mut out = MaybeUninit::uninit();
        self.0.erased_serialize_u8(v, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_u16(self, v: u16) -> Result<Ok, Error> {
        let mut out = MaybeUninit::uninit();
        self.0.erased_serialize_u16(v, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_u32(self, v: u32) -> Result<Ok, Error> {
        let mut out = MaybeUninit::uninit();
        self.0.erased_serialize_u32(v, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_u64(self, v: u64) -> Result<Ok, Error> {
        let mut out = MaybeUninit::uninit();
        self.0.erased_serialize_u64(v, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_u128(self, v: u128) -> Result<Ok, Error> {
        let mut out = MaybeUninit::uninit();
        self.0.erased_serialize_u128(v, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_f32(self, v: f32) -> Result<Ok, Error> {
        let mut out = MaybeUninit::uninit();
        self.0.erased_serialize_f32(v, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_f64(self, v: f64) -> Result<Ok, Error> {
        let mut out = MaybeUninit::uninit();
        self.0.erased_serialize_f64(v, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_char(self, v: char) -> Result<Ok, Error> {
        let mut out = MaybeUninit::uninit();
        self.0.erased_serialize_char(v, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_str(self, v: &str) -> Result<Ok, Error> {
        let mut out = MaybeUninit::uninit();
        self.0.erased_serialize_str(v, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Ok, Error> {
        let mut out = MaybeUninit::uninit();
        self.0.erased_serialize_bytes(v, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_none(self) -> Result<Ok, Error> {
        let mut out = MaybeUninit::uninit();
        self.0.erased_serialize_none(&mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_some<T>(self, value: &T) -> Result<Ok, Error>
    where
        T: ?Sized + serde::Serialize,
    {
        let mut out = MaybeUninit::uninit();
        self.0.erased_serialize_some(&value, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_unit(self) -> Result<Ok, Error> {
        let mut out = MaybeUninit::uninit();
        self.0.erased_serialize_unit(&mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Ok, Error> {
        let mut out = MaybeUninit::uninit();
        self.0.erased_serialize_unit_struct(name, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Ok, Error> {
        let mut out = MaybeUninit::uninit();
        self.0
            .erased_serialize_unit_variant(name, variant_index, variant, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Ok, Error>
    where
        T: ?Sized + serde::Serialize,
    {
        let mut out = MaybeUninit::uninit();
        self.0
            .erased_serialize_newtype_struct(name, &value, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Ok, Error>
    where
        T: ?Sized + serde::Serialize,
    {
        let mut out = MaybeUninit::uninit();
        self.0
            .erased_serialize_newtype_variant(name, variant_index, variant, &value, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Seq<'a>, Error> {
        let mut out = MaybeUninit::uninit();
        self.0.erased_serialize_seq(len, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_tuple(self, len: usize) -> Result<Tuple<'a>, Error> {
        let mut out = MaybeUninit::uninit();
        self.0.erased_serialize_tuple(len, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<TupleStruct<'a>, Error> {
        let mut out = MaybeUninit::uninit();
        self.0.erased_serialize_tuple_struct(name, len, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<TupleVariant<'a>, Error> {
        let mut out = MaybeUninit::uninit();
        self.0
            .erased_serialize_tuple_variant(name, variant_index, variant, len, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Map<'a>, Error> {
        let mut out = MaybeUninit::uninit();
        self.0.erased_serialize_map(len, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Struct<'a>, Error> {
        let mut out = MaybeUninit::uninit();
        self.0.erased_serialize_struct(name, len, &mut out);
        unsafe { out.assume_init() }
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<StructVariant<'a>, Error> {
        let mut out = MaybeUninit::uninit();
        self.0
            .erased_serialize_struct_variant(name, variant_index, variant, len, &mut out);
        unsafe { out.assume_init() }
    }

    #[cfg(not(feature = "alloc"))]
    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Display,
    {
        unreachable!()
    }

    fn is_human_readable(&self) -> bool {
        self.0.erased_is_human_readable()
    }
}

pub struct Seq<'a> {
    data: Any,
    serialize_element: unsafe fn(&mut Any, &dyn Serialize) -> Result<(), Error>,
    end: unsafe fn(Any) -> Result<Ok, Error>,
    lifetime: PhantomData<&'a dyn Serializer>,
}

impl<'a> Seq<'a> {
    unsafe fn new<T>(data: T) -> Self
    where
        T: serde::ser::SerializeSeq,
    {
        Seq {
            data: unsafe { Any::new(data) },
            serialize_element: {
                unsafe fn serialize_element<T>(
                    data: &mut Any,
                    v: &dyn Serialize,
                ) -> Result<(), Error>
                where
                    T: serde::ser::SerializeSeq,
                {
                    unsafe { data.as_ref::<T>().serialize_element(v).map_err(erase) }
                }
                serialize_element::<T>
            },
            end: {
                unsafe fn end<T>(data: Any) -> Result<Ok, Error>
                where
                    T: serde::ser::SerializeSeq,
                {
                    unsafe { data.take::<T>().end().unsafe_map(Ok::new).map_err(erase) }
                }
                end::<T>
            },
            lifetime: PhantomData,
        }
    }
}

impl<'a> SerializeSeq for Seq<'a> {
    type Ok = Ok;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + serde::Serialize,
    {
        unsafe { (self.serialize_element)(&mut self.data, &value) }
    }

    fn end(self) -> Result<Ok, Error> {
        unsafe { (self.end)(self.data) }
    }
}

pub struct Tuple<'a> {
    data: Any,
    serialize_element: unsafe fn(&mut Any, &dyn Serialize) -> Result<(), Error>,
    end: unsafe fn(Any) -> Result<Ok, Error>,
    lifetime: PhantomData<&'a dyn Serializer>,
}

impl<'a> Tuple<'a> {
    unsafe fn new<T>(data: T) -> Self
    where
        T: serde::ser::SerializeTuple,
    {
        Tuple {
            data: unsafe { Any::new(data) },
            serialize_element: {
                unsafe fn serialize_element<T>(
                    data: &mut Any,
                    value: &dyn Serialize,
                ) -> Result<(), Error>
                where
                    T: serde::ser::SerializeTuple,
                {
                    unsafe { data.as_ref::<T>().serialize_element(value).map_err(erase) }
                }
                serialize_element::<T>
            },
            end: {
                unsafe fn end<T>(data: Any) -> Result<Ok, Error>
                where
                    T: serde::ser::SerializeTuple,
                {
                    unsafe { data.take::<T>().end().unsafe_map(Ok::new).map_err(erase) }
                }
                end::<T>
            },
            lifetime: PhantomData,
        }
    }
}

impl<'a> SerializeTuple for Tuple<'a> {
    type Ok = Ok;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + serde::Serialize,
    {
        unsafe { (self.serialize_element)(&mut self.data, &value) }
    }

    fn end(self) -> Result<Ok, Error> {
        unsafe { (self.end)(self.data) }
    }
}

pub struct TupleStruct<'a> {
    data: Any,
    serialize_field: unsafe fn(&mut Any, &dyn Serialize) -> Result<(), Error>,
    end: unsafe fn(Any) -> Result<Ok, Error>,
    lifetime: PhantomData<&'a dyn Serializer>,
}

impl<'a> TupleStruct<'a> {
    unsafe fn new<T>(data: T) -> Self
    where
        T: serde::ser::SerializeTupleStruct,
    {
        TupleStruct {
            data: unsafe { Any::new(data) },
            serialize_field: {
                unsafe fn serialize_field<T>(
                    data: &mut Any,
                    value: &dyn Serialize,
                ) -> Result<(), Error>
                where
                    T: serde::ser::SerializeTupleStruct,
                {
                    unsafe { data.as_ref::<T>().serialize_field(value).map_err(erase) }
                }
                serialize_field::<T>
            },
            end: {
                unsafe fn end<T>(data: Any) -> Result<Ok, Error>
                where
                    T: serde::ser::SerializeTupleStruct,
                {
                    unsafe { data.take::<T>().end().unsafe_map(Ok::new).map_err(erase) }
                }
                end::<T>
            },
            lifetime: PhantomData,
        }
    }
}

impl<'a> SerializeTupleStruct for TupleStruct<'a> {
    type Ok = Ok;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + serde::Serialize,
    {
        unsafe { (self.serialize_field)(&mut self.data, &value) }
    }

    fn end(self) -> Result<Ok, Error> {
        unsafe { (self.end)(self.data) }
    }
}

pub struct TupleVariant<'a> {
    data: Any,
    serialize_field: unsafe fn(&mut Any, &dyn Serialize) -> Result<(), Error>,
    end: unsafe fn(Any) -> Result<Ok, Error>,
    lifetime: PhantomData<&'a dyn Serializer>,
}

impl<'a> TupleVariant<'a> {
    unsafe fn new<T>(data: T) -> Self
    where
        T: serde::ser::SerializeTupleVariant,
    {
        TupleVariant {
            data: unsafe { Any::new(data) },
            serialize_field: {
                unsafe fn serialize_field<T>(
                    data: &mut Any,
                    value: &dyn Serialize,
                ) -> Result<(), Error>
                where
                    T: serde::ser::SerializeTupleVariant,
                {
                    unsafe { data.as_ref::<T>().serialize_field(value).map_err(erase) }
                }
                serialize_field::<T>
            },
            end: {
                unsafe fn end<T>(data: Any) -> Result<Ok, Error>
                where
                    T: serde::ser::SerializeTupleVariant,
                {
                    unsafe { data.take::<T>().end().unsafe_map(Ok::new).map_err(erase) }
                }
                end::<T>
            },
            lifetime: PhantomData,
        }
    }
}

impl<'a> SerializeTupleVariant for TupleVariant<'a> {
    type Ok = Ok;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + serde::Serialize,
    {
        unsafe { (self.serialize_field)(&mut self.data, &value) }
    }

    fn end(self) -> Result<Ok, Error> {
        unsafe { (self.end)(self.data) }
    }
}

pub struct Map<'a> {
    data: Any,
    serialize_key: unsafe fn(&mut Any, &dyn Serialize) -> Result<(), Error>,
    serialize_value: unsafe fn(&mut Any, &dyn Serialize) -> Result<(), Error>,
    serialize_entry: unsafe fn(&mut Any, &dyn Serialize, &dyn Serialize) -> Result<(), Error>,
    end: unsafe fn(Any) -> Result<Ok, Error>,
    lifetime: PhantomData<&'a dyn Serializer>,
}

impl<'a> Map<'a> {
    unsafe fn new<T>(data: T) -> Self
    where
        T: serde::ser::SerializeMap,
    {
        Map {
            data: unsafe { Any::new(data) },
            serialize_key: {
                unsafe fn serialize_key<T>(data: &mut Any, key: &dyn Serialize) -> Result<(), Error>
                where
                    T: serde::ser::SerializeMap,
                {
                    unsafe { data.as_ref::<T>().serialize_key(key).map_err(erase) }
                }
                serialize_key::<T>
            },
            serialize_value: {
                unsafe fn serialize_value<T>(
                    data: &mut Any,
                    value: &dyn Serialize,
                ) -> Result<(), Error>
                where
                    T: serde::ser::SerializeMap,
                {
                    unsafe { data.as_ref::<T>().serialize_value(value).map_err(erase) }
                }
                serialize_value::<T>
            },
            serialize_entry: {
                unsafe fn serialize_entry<T>(
                    data: &mut Any,
                    key: &dyn Serialize,
                    value: &dyn Serialize,
                ) -> Result<(), Error>
                where
                    T: serde::ser::SerializeMap,
                {
                    unsafe {
                        data.as_ref::<T>()
                            .serialize_entry(key, value)
                            .map_err(erase)
                    }
                }
                serialize_entry::<T>
            },
            end: {
                unsafe fn end<T>(data: Any) -> Result<Ok, Error>
                where
                    T: serde::ser::SerializeMap,
                {
                    unsafe { data.take::<T>().end().unsafe_map(Ok::new).map_err(erase) }
                }
                end::<T>
            },
            lifetime: PhantomData,
        }
    }
}

impl<'a> SerializeMap for Map<'a> {
    type Ok = Ok;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Error>
    where
        T: ?Sized + serde::Serialize,
    {
        unsafe { (self.serialize_key)(&mut self.data, &key) }
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + serde::Serialize,
    {
        unsafe { (self.serialize_value)(&mut self.data, &value) }
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Error>
    where
        K: ?Sized + serde::Serialize,
        V: ?Sized + serde::Serialize,
    {
        unsafe { (self.serialize_entry)(&mut self.data, &key, &value) }
    }

    fn end(self) -> Result<Ok, Error> {
        unsafe { (self.end)(self.data) }
    }
}

pub struct Struct<'a> {
    data: Any,
    serialize_field: unsafe fn(&mut Any, &'static str, &dyn Serialize) -> Result<(), Error>,
    end: unsafe fn(Any) -> Result<Ok, Error>,
    lifetime: PhantomData<&'a dyn Serializer>,
}

impl<'a> Struct<'a> {
    unsafe fn new<T>(data: T) -> Self
    where
        T: serde::ser::SerializeStruct,
    {
        Struct {
            data: unsafe { Any::new(data) },
            serialize_field: {
                unsafe fn serialize_field<T>(
                    data: &mut Any,
                    key: &'static str,
                    value: &dyn Serialize,
                ) -> Result<(), Error>
                where
                    T: serde::ser::SerializeStruct,
                {
                    unsafe {
                        data.as_ref::<T>()
                            .serialize_field(key, value)
                            .map_err(erase)
                    }
                }
                serialize_field::<T>
            },
            end: {
                unsafe fn end<T>(data: Any) -> Result<Ok, Error>
                where
                    T: serde::ser::SerializeStruct,
                {
                    unsafe { data.take::<T>().end().unsafe_map(Ok::new).map_err(erase) }
                }
                end::<T>
            },
            lifetime: PhantomData,
        }
    }
}

impl<'a> SerializeStruct for Struct<'a> {
    type Ok = Ok;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + serde::Serialize,
    {
        unsafe { (self.serialize_field)(&mut self.data, key, &value) }
    }

    fn end(self) -> Result<Ok, Error> {
        unsafe { (self.end)(self.data) }
    }
}

pub struct StructVariant<'a> {
    data: Any,
    serialize_field: unsafe fn(&mut Any, &'static str, &dyn Serialize) -> Result<(), Error>,
    end: unsafe fn(Any) -> Result<Ok, Error>,
    lifetime: PhantomData<&'a dyn Serializer>,
}

impl<'a> StructVariant<'a> {
    unsafe fn new<T>(data: T) -> Self
    where
        T: serde::ser::SerializeStructVariant,
    {
        StructVariant {
            data: unsafe { Any::new(data) },
            serialize_field: {
                unsafe fn serialize_field<T>(
                    data: &mut Any,
                    key: &'static str,
                    value: &dyn Serialize,
                ) -> Result<(), Error>
                where
                    T: serde::ser::SerializeStructVariant,
                {
                    unsafe {
                        data.as_ref::<T>()
                            .serialize_field(key, value)
                            .map_err(erase)
                    }
                }
                serialize_field::<T>
            },
            end: {
                unsafe fn end<T>(data: Any) -> Result<Ok, Error>
                where
                    T: serde::ser::SerializeStructVariant,
                {
                    unsafe { data.take::<T>().end().unsafe_map(Ok::new).map_err(erase) }
                }
                end::<T>
            },
            lifetime: PhantomData,
        }
    }
}

impl<'a> SerializeStructVariant for StructVariant<'a> {
    type Ok = Ok;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + serde::Serialize,
    {
        unsafe { (self.serialize_field)(&mut self.data, key, &value) }
    }

    fn end(self) -> Result<Ok, Error> {
        unsafe { (self.end)(self.data) }
    }
}

// IMPL ERASED SERDE FOR ERASED SERDE //////////////////////////////////////////

macro_rules! deref_erased_serializer {
    (<'a $(, $T:ident)*> Serializer for $ty:ty $(where $($where:tt)*)?) => {
        impl<'a $(, $T)*> Serializer for $ty $(where $($where)*)? {
            fn erased_serialize_bool(&mut self, v: bool, out: &mut MaybeUninit<Result<Ok, Error>>) {
                (**self).erased_serialize_bool(v, out);
            }

            fn erased_serialize_i8(&mut self, v: i8, out: &mut MaybeUninit<Result<Ok, Error>>) {
                (**self).erased_serialize_i8(v, out);
            }

            fn erased_serialize_i16(&mut self, v: i16, out: &mut MaybeUninit<Result<Ok, Error>>) {
                (**self).erased_serialize_i16(v, out);
            }

            fn erased_serialize_i32(&mut self, v: i32, out: &mut MaybeUninit<Result<Ok, Error>>) {
                (**self).erased_serialize_i32(v, out);
            }

            fn erased_serialize_i64(&mut self, v: i64, out: &mut MaybeUninit<Result<Ok, Error>>) {
                (**self).erased_serialize_i64(v, out);
            }

            fn erased_serialize_i128(&mut self, v: i128, out: &mut MaybeUninit<Result<Ok, Error>>) {
                (**self).erased_serialize_i128(v, out);
            }

            fn erased_serialize_u8(&mut self, v: u8, out: &mut MaybeUninit<Result<Ok, Error>>) {
                (**self).erased_serialize_u8(v, out);
            }

            fn erased_serialize_u16(&mut self, v: u16, out: &mut MaybeUninit<Result<Ok, Error>>) {
                (**self).erased_serialize_u16(v, out);
            }

            fn erased_serialize_u32(&mut self, v: u32, out: &mut MaybeUninit<Result<Ok, Error>>) {
                (**self).erased_serialize_u32(v, out);
            }

            fn erased_serialize_u64(&mut self, v: u64, out: &mut MaybeUninit<Result<Ok, Error>>) {
                (**self).erased_serialize_u64(v, out);
            }

            fn erased_serialize_u128(&mut self, v: u128, out: &mut MaybeUninit<Result<Ok, Error>>) {
                (**self).erased_serialize_u128(v, out);
            }

            fn erased_serialize_f32(&mut self, v: f32, out: &mut MaybeUninit<Result<Ok, Error>>) {
                (**self).erased_serialize_f32(v, out);
            }

            fn erased_serialize_f64(&mut self, v: f64, out: &mut MaybeUninit<Result<Ok, Error>>) {
                (**self).erased_serialize_f64(v, out);
            }

            fn erased_serialize_char(&mut self, v: char, out: &mut MaybeUninit<Result<Ok, Error>>) {
                (**self).erased_serialize_char(v, out);
            }

            fn erased_serialize_str(&mut self, v: &str, out: &mut MaybeUninit<Result<Ok, Error>>) {
                (**self).erased_serialize_str(v, out);
            }

            fn erased_serialize_bytes(&mut self, v: &[u8], out: &mut MaybeUninit<Result<Ok, Error>>) {
                (**self).erased_serialize_bytes(v, out);
            }

            fn erased_serialize_none(&mut self, out: &mut MaybeUninit<Result<Ok, Error>>) {
                (**self).erased_serialize_none(out);
            }

            fn erased_serialize_some(&mut self, value: &dyn Serialize, out: &mut MaybeUninit<Result<Ok, Error>>) {
                (**self).erased_serialize_some(value, out);
            }

            fn erased_serialize_unit(&mut self, out: &mut MaybeUninit<Result<Ok, Error>>) {
                (**self).erased_serialize_unit(out);
            }

            fn erased_serialize_unit_struct(&mut self, name: &'static str, out: &mut MaybeUninit<Result<Ok, Error>>) {
                (**self).erased_serialize_unit_struct(name, out);
            }

            fn erased_serialize_unit_variant(&mut self, name: &'static str, variant_index: u32, variant: &'static str, out: &mut MaybeUninit<Result<Ok, Error>>) {
                (**self).erased_serialize_unit_variant(name, variant_index, variant, out);
            }

            fn erased_serialize_newtype_struct(&mut self, name: &'static str, value: &dyn Serialize, out: &mut MaybeUninit<Result<Ok, Error>>) {
                (**self).erased_serialize_newtype_struct(name, value, out);
            }

            fn erased_serialize_newtype_variant(&mut self, name: &'static str, variant_index: u32, variant: &'static str, value: &dyn Serialize, out: &mut MaybeUninit<Result<Ok, Error>>) {
                (**self).erased_serialize_newtype_variant(name, variant_index, variant, value, out);
            }

            fn erased_serialize_seq(&mut self, len: Option<usize>, out: &mut MaybeUninit<Result<Seq, Error>>) {
                (**self).erased_serialize_seq(len, out);
            }

            fn erased_serialize_tuple(&mut self, len: usize, out: &mut MaybeUninit<Result<Tuple, Error>>) {
                (**self).erased_serialize_tuple(len, out);
            }

            fn erased_serialize_tuple_struct(&mut self, name: &'static str, len: usize, out: &mut MaybeUninit<Result<TupleStruct, Error>>) {
                (**self).erased_serialize_tuple_struct(name, len, out);
            }

            fn erased_serialize_tuple_variant(&mut self, name: &'static str, variant_index: u32, variant: &'static str, len: usize, out: &mut MaybeUninit<Result<TupleVariant, Error>>) {
                (**self).erased_serialize_tuple_variant(name, variant_index, variant, len, out);
            }

            fn erased_serialize_map(&mut self, len: Option<usize>, out: &mut MaybeUninit<Result<Map, Error>>) {
                (**self).erased_serialize_map(len, out);
            }

            fn erased_serialize_struct(&mut self, name: &'static str, len: usize, out: &mut MaybeUninit<Result<Struct, Error>>) {
                (**self).erased_serialize_struct(name, len, out);
            }

            fn erased_serialize_struct_variant(&mut self, name: &'static str, variant_index: u32, variant: &'static str, len: usize, out: &mut MaybeUninit<Result<StructVariant, Error>>) {
                (**self).erased_serialize_struct_variant(name, variant_index, variant, len, out);
            }

            fn erased_is_human_readable(&self) -> bool {
                (**self).erased_is_human_readable()
            }
        }

        impl<'a $(, $T)*> sealed::serializer::Sealed for $ty $(where $($where)*)? {}
    };
}

deref_erased_serializer!(<'a> Serializer for Box<dyn Serializer + 'a>);
deref_erased_serializer!(<'a> Serializer for Box<dyn Serializer + Send + 'a>);
deref_erased_serializer!(<'a> Serializer for Box<dyn Serializer + Sync + 'a>);
deref_erased_serializer!(<'a> Serializer for Box<dyn Serializer + Send + Sync + 'a>);
deref_erased_serializer!(<'a, T> Serializer for &'a mut T where T: ?Sized + Serializer);

// TEST ////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{vec, vec::Vec};
    use serde_derive::Serialize;

    fn test_json<T>(t: T)
    where
        T: serde::Serialize,
    {
        let expected = serde_json::to_vec(&t).unwrap();

        // test borrowed trait object
        {
            let obj: &dyn Serialize = &t;

            let mut buf = Vec::new();

            {
                let mut ser = serde_json::Serializer::new(&mut buf);
                let ser: &mut dyn Serializer = &mut <dyn Serializer>::erase(&mut ser);

                obj.erased_serialize(ser).unwrap();
            }

            assert_eq!(buf, expected);
        }

        // test boxed trait object
        {
            let obj: Box<dyn Serialize> = Box::new(t);

            let mut buf = Vec::new();

            {
                let mut ser = serde_json::Serializer::new(&mut buf);
                let mut ser: Box<dyn Serializer> = Box::new(<dyn Serializer>::erase(&mut ser));

                obj.erased_serialize(&mut ser).unwrap();
            }

            assert_eq!(buf, expected);
        }
    }

    #[test]
    fn test_vec() {
        test_json(vec!["a", "b"]);
    }

    #[test]
    fn test_struct() {
        #[derive(Serialize)]
        struct S {
            f: usize,
        }

        test_json(S { f: 256 });
    }

    #[test]
    fn test_enum() {
        #[derive(Serialize)]
        enum E {
            Unit,
            Newtype(bool),
            Tuple(bool, bool),
            Struct { t: bool, f: bool },
        }

        test_json(E::Unit);
        test_json(E::Newtype(true));
        test_json(E::Tuple(true, false));
        test_json(E::Struct { t: true, f: false });
    }

    #[test]
    fn assert_serialize() {
        fn assert<T: serde::Serialize>() {}

        assert::<&dyn Serialize>();
        assert::<&(dyn Serialize + Send)>();
        assert::<&(dyn Serialize + Sync)>();
        assert::<&(dyn Serialize + Send + Sync)>();
        assert::<&(dyn Serialize + Sync + Send)>();
        assert::<Vec<&dyn Serialize>>();
        assert::<Vec<&(dyn Serialize + Send)>>();

        assert::<Box<dyn Serialize>>();
        assert::<Box<dyn Serialize + Send>>();
        assert::<Box<dyn Serialize + Sync>>();
        assert::<Box<dyn Serialize + Send + Sync>>();
        assert::<Box<dyn Serialize + Sync + Send>>();
        assert::<Vec<Box<dyn Serialize>>>();
        assert::<Vec<Box<dyn Serialize + Send>>>();
    }
}
