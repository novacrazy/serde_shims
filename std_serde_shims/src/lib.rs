//! Potentially useful shims for the standard library
//!
//! To enable `std` shims, add it to the crate features list:
//!
//! ```toml
//! [dependencies]
//! std_serde_shims = "0.2"
//! ```

extern crate serde;

use serde::ser::{Serialize, Serializer};

/// `Option` shims
pub mod option {
    use super::*;

    /// Serializes `None` as `Some(Default)`
    ///
    /// Useful for avoiding null values with integers.
    ///
    /// Full Example:
    ///
    /// ```
    /// #[macro_use]
    /// extern crate serde_derive;
    /// extern crate serde_json;
    /// extern crate std_serde_shims;
    ///
    /// #[derive(Debug, PartialEq, Serialize, Deserialize)]
    /// struct Test {
    ///     /// Many web APIs consider zero to be no limit, but we want our API to use `Option` instead
    ///     #[serde(serialize_with = "std_serde_shims::option::serialize_none_as_default")]
    ///     item_limit: Option<u64>,
    /// }
    ///
    /// fn main() {
    ///     let none_test = Test {
    ///         item_limit: None
    ///     };
    ///
    ///     assert_eq!(serde_json::to_string(&none_test).unwrap(), r#"{"item_limit":0}"#);
    /// }
    /// ```
    pub fn serialize_none_as_default<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize + Default,
    {
        match value {
            Some(value) => value.serialize(serializer),
            None => T::default().serialize(serializer),
        }
    }
}
