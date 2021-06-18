//! [![github]](https://github.com/dtolnay/erased-serde)&ensp;[![crates-io]](https://crates.io/crates/erased-serde)&ensp;[![docs-rs]](https://docs.rs/erased-serde)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
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
//!     let json = &mut serde_json::Serializer::new(io::stdout());
//!     let cbor = &mut serde_cbor::Serializer::new(serde_cbor::ser::IoWrite::new(io::stdout()));
//!
//!     // The values in this map are boxed trait objects. Ordinarily this would not
//!     // be possible with serde::Serializer because of object safety, but type
//!     // erasure makes it possible with erased_serde::Serializer.
//!     let mut formats: Map<&str, Box<dyn Serializer>> = Map::new();
//!     formats.insert("json", Box::new(<dyn Serializer>::erase(json)));
//!     formats.insert("cbor", Box::new(<dyn Serializer>::erase(cbor)));
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
//!     let json = &mut serde_json::Deserializer::from_slice(JSON);
//!     let cbor = &mut serde_cbor::Deserializer::from_slice(CBOR);
//!
//!     // The values in this map are boxed trait objects, which is not possible
//!     // with the normal serde::Deserializer because of object safety.
//!     let mut formats: Map<&str, Box<dyn Deserializer>> = Map::new();
//!     formats.insert("json", Box::new(<dyn Deserializer>::erase(json)));
//!     formats.insert("cbor", Box::new(<dyn Deserializer>::erase(cbor)));
//!
//!     // Pick a Deserializer out of the formats map.
//!     let format = formats.get_mut("json").unwrap();
//!
//!     let data: Map<String, usize> = erased_serde::deserialize(format).unwrap();
//!
//!     println!("{}", data["A"] + data["B"]);
//! }
//! ```

#![doc(html_root_url = "https://docs.rs/erased-serde/0.3.16")]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(
    clippy::items_after_statements,
    clippy::missing_errors_doc,
    clippy::needless_doctest_main,
    clippy::semicolon_if_nothing_returned, // https://github.com/rust-lang/rust-clippy/issues/7324
    clippy::unused_self,
    clippy::wildcard_imports
)]

mod alloc {
    #[cfg(not(feature = "std"))]
    extern crate alloc;

    #[cfg(feature = "std")]
    use std as alloc;

    pub use self::alloc::borrow::ToOwned;
    pub use self::alloc::boxed::Box;
    pub use self::alloc::string::{String, ToString};
    pub use self::alloc::{vec, vec::Vec};
}

#[macro_use]
mod macros;

mod any;
mod de;
mod error;
mod features_check;
mod ser;

pub use crate::de::{deserialize, Deserializer};
pub use crate::error::{Error, Result};
pub use crate::ser::{serialize, Serialize, Serializer};

// Not public API.
#[doc(hidden)]
pub mod private;
