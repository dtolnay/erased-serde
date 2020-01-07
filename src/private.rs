//! Not public API. Used as `$crate::export` by macros.

pub use serde;

pub use self::std::{
    marker::{Send, Sync},
    result::Result,
};

#[cfg(feature = "std")]
pub(crate) mod std {
    pub use std::{borrow, boxed, error, fmt, marker, mem, result, string, vec};

    #[cfg(feature = "unstable-debug")]
    pub use std::intrinsics;
}

#[cfg(not(feature = "std"))]
pub(crate) mod std {
    pub use alloc::{borrow, boxed, string, vec};

    pub use core::{fmt, marker, mem, result};

    #[cfg(feature = "unstable-debug")]
    pub use core::intrinsics;
}
