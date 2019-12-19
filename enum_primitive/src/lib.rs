//! `enum_primitive` crate Serde shims
//!
//! To enable this shim, add it as a dependency:
//!
//! ```toml
//! [dependencies]
//! enum_primitive_serde_shim = "0.2"
//! ```
//!
//! Full example:
//!
//! ```
//! #[macro_use]
//! extern crate serde_derive;
//! extern crate serde_json;
//!
//! #[macro_use]
//! extern crate enum_primitive;
//! #[macro_use] // required for impl_serde_for_enum_primitive
//! extern crate enum_primitive_serde_shim;
//!
//! enum_from_primitive! {
//!     #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
//!     enum Codes {
//!         CodeA = 0,
//!         CodeB = 1,
//!         CodeC = 5,
//!         CodeD = 8,
//!     }
//! }
//!
//! impl_serde_for_enum_primitive!(Codes);
//!
//! fn main() {
//!     let code = Codes::CodeC;
//!
//!     assert_eq!(serde_json::to_string(&code).unwrap(), "5");
//!
//!     assert_eq!(serde_json::from_str::<Codes>("8").unwrap(), Codes::CodeD);
//!
//!     assert!(serde_json::from_str::<Codes>("16").is_err());
//! }
//! ```
#![cfg_attr(not(feature = "std"), no_std)]

#[doc(hidden)]
pub extern crate serde;

#[doc(hidden)]
#[cfg(not(feature = "std"))]
pub use core::{fmt, result::Result};

#[doc(hidden)]
#[cfg(feature = "std")]
pub use std::{result::Result, fmt};

#[doc(hidden)]
pub extern crate enum_primitive;

/// Implements `Serialize` and `Deserialize` for an `enum_from_primitive!` generated enum.
///
/// See the [`enum_primitive`](../enum_primitive_serde_shim/index.html) shim for a full example.
#[macro_export]
macro_rules! impl_serde_for_enum_primitive {
    ($name:ident) => {
        impl $crate::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> $crate::Result<S::Ok, S::Error>
            where
                S: $crate::serde::Serializer,
            {
                serializer.serialize_u64(*self as u64)
            }
        }

        impl<'de> $crate::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> $crate::Result<Self, D::Error>
            where
                D: $crate::serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> $crate::serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
                        formatter.write_str(stringify!(Expected $name as primitive numeric value))
                    }

                    #[cfg(feature = "std")]
                    fn visit_u64<E>(self, value: u64) -> $crate::Result<$name, E>
                    where
                        E: $crate::serde::de::Error,
                    {
                        $crate::enum_primitive::FromPrimitive::from_u64(value)
                            .ok_or_else(|| E::custom(format!("Invalid primitive value {} for enum {}", value, stringify!($name))))
                    }

                    #[cfg(not(feature = "std"))]
                    fn visit_u64<E>(self, value: u64) -> $crate::Result<$name, E>
                    where
                        E: $crate::serde::de::Error,
                    {
                        $crate::enum_primitive::FromPrimitive::from_u64(value)
                            .ok_or_else(|| E::custom(stringify!(Invalid primitive value for enum $name)))
                    }
                }

                deserializer.deserialize_u64(Visitor)
            }
        }
    };
}
