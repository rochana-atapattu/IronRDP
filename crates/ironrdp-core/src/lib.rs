#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
mod macros;

mod as_any;

// Flat API hierarchy of common traits and types

pub use self::as_any::*;

// Trait that can only be implemented within the current module
pub(crate) mod private {
    #[doc(hidden)]
    pub trait Sealed {}
}
