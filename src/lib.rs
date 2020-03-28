//! This crate provides type-erased versions of Serde's `Serialize`, `Serializer`
//! and `Deserializer` traits that can be used as [trait objects].
//!
//! [trait objects]: https://doc.rust-lang.org/book/trait-objects.html
//!
//! The usual Serde `Serialize`, `Serializer` and `Deserializer` traits cannot
//! be used as trait objects like `&dyn Serialize` or boxed trait objects like
//! `Box<dyn Serialize>` because of Rust's ["object safety" rules]. In
//! particular, all three traits contain generic methods which cannot be made
//! into a trait object.
//!
//! ["object safety" rules]: http://huonw.github.io/blog/2015/01/object-safety/
//!
//! This library should be considered a low-level building block for interacting
//! with Serde APIs in an object-safe way. Most use cases will require higher
//! level functionality such as provided by [`typetag`] which uses this crate
//! internally.
//!
//! [`typetag`]: https://github.com/dtolnay/typetag
//!
//! **The traits in this crate work seamlessly with any existing Serde
//! `Serialize` and `Deserialize` type and any existing Serde `Serializer` and
//! `Deserializer` format.**
//!
//! ## Serialization
//!
//! ```rust
//! use erased_serde::{Serialize, Serializer};
//! use std::collections::BTreeMap as Map;
//! use std::io;
//!
//! fn main() {
//!     // Construct some serializers.
//!     let json = &mut serde_json::ser::Serializer::new(io::stdout());
//!     let cbor = &mut serde_cbor::ser::Serializer::new(io::stdout());
//!
//!     // The values in this map are boxed trait objects. Ordinarily this would not
//!     // be possible with serde::Serializer because of object safety, but type
//!     // erasure makes it possible with erased_serde::Serializer.
//!     let mut formats: Map<&str, Box<dyn Serializer>> = Map::new();
//!     formats.insert("json", Box::new(Serializer::erase(json)));
//!     formats.insert("cbor", Box::new(Serializer::erase(cbor)));
//!
//!     // These are boxed trait objects as well. Same thing here - type erasure
//!     // makes this possible.
//!     let mut values: Map<&str, Box<dyn Serialize>> = Map::new();
//!     values.insert("vec", Box::new(vec!["a", "b"]));
//!     values.insert("int", Box::new(65536));
//!
//!     // Pick a Serializer out of the formats map.
//!     let format = formats.get_mut("json").unwrap();
//!
//!     // Pick a Serialize out of the values map.
//!     let value = values.get("vec").unwrap();
//!
//!     // This line prints `["a","b"]` to stdout.
//!     value.erased_serialize(format).unwrap();
//! }
//! ```
//!
//! ## Deserialization
//!
//! ```rust
//! use erased_serde::Deserializer;
//! use std::collections::BTreeMap as Map;
//!
//! fn main() {
//!     static JSON: &'static [u8] = br#"{"A": 65, "B": 66}"#;
//!     static CBOR: &'static [u8] = &[162, 97, 65, 24, 65, 97, 66, 24, 66];
//!
//!     // Construct some deserializers.
//!     let json = &mut serde_json::de::Deserializer::from_slice(JSON);
//!     let cbor = &mut serde_cbor::de::Deserializer::from_slice(CBOR);
//!
//!     // The values in this map are boxed trait objects, which is not possible
//!     // with the normal serde::Deserializer because of object safety.
//!     let mut formats: Map<&str, Box<dyn Deserializer>> = Map::new();
//!     formats.insert("json", Box::new(Deserializer::erase(json)));
//!     formats.insert("cbor", Box::new(Deserializer::erase(cbor)));
//!
//!     // Pick a Deserializer out of the formats map.
//!     let format = formats.get_mut("json").unwrap();
//!
//!     let data: Map<String, usize> = erased_serde::deserialize(format).unwrap();
//!
//!     println!("{}", data["A"] + data["B"]);
//! }
//! ```

#![doc(html_root_url = "https://docs.rs/erased-serde/0.3.10")]

#[macro_use]
mod macros;

mod any;
mod de;
mod error;
mod ser;

pub use crate::de::{deserialize, Deserializer};
pub use crate::error::{Error, Result};
pub use crate::ser::{serialize, Serialize, Serializer};

// Not public API.
#[doc(hidden)]
pub mod private;
