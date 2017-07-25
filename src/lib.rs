#![doc(html_root_url = "https://docs.rs/erased-serde/0.2.0")]

extern crate serde;

mod any;
mod de;
mod error;
mod ser;

pub use de::{deserialize, Deserializer};
pub use error::Error;
pub use ser::{Serialize, Serializer};
