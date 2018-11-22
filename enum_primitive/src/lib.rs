//! `enum_primitive` crate Serde shims
//!
//! To enable this shim, add it to the crate features list:
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

#[doc(hidden)]
pub extern crate serde;

#[doc(hidden)]
pub extern crate enum_primitive;

/// Implements `Serialize` and `Deserialize` for an `enum_from_primitive!` generated enum.
///
/// See the [`enum_primitive`](../enum_primitive_serde_shim/index.html) shim for a full example.
#[macro_export]
macro_rules! impl_serde_for_enum_primitive {
    ($name:ident) => {
        impl $crate::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: $crate::serde::Serializer,
            {
                serializer.serialize_u64(*self as u64)
            }
        }

        impl<'de> $crate::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: $crate::serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> $crate::serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(formatter, "Expected {} as numeric value", stringify!($name))
                    }

                    fn visit_u64<E>(self, value: u64) -> ::std::result::Result<$name, E>
                    where
                        E: $crate::serde::de::Error,
                    {
                        $crate::enum_primitive::FromPrimitive::from_u64(value)
                            .ok_or_else(|| E::custom(format!("Invalid Value {} for enum {}", value, stringify!($name))))
                    }
                }

                deserializer.deserialize_u64(Visitor)
            }
        }
    };
}
