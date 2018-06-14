//! Chrono crate Serde shims
//!
//! To enable the Chrono shims, add it to the crate features list:
//!
//! ```toml
//! serde_shims = { version = "*", features = ["chrono"] }
//! ```
//!
//! Full example:
//!
//! ```
//! #[macro_use]
//! extern crate serde_derive;
//! extern crate serde_json;
//! extern crate serde_shims;
//!
//! extern crate chrono;
//!
//! use chrono::NaiveDateTime;
//!
//! #[derive(Debug, PartialEq, Serialize, Deserialize)]
//! struct NaiveUnixTimestampMs(
//!     #[serde(serialize_with = "serde_shims::chrono::serialize_naivedatetime_as_millis")]
//!     #[serde(deserialize_with = "serde_shims::chrono::deserialize_naivedatetime_from_millis")]
//!     pub NaiveDateTime
//! );
//!
//! fn main() {
//!     let test = NaiveUnixTimestampMs(NaiveDateTime::from_timestamp(123, 456));
//!
//!     // Note that nanoseconds are not preserved by millisecond precision
//!     let expected = "123000";
//!
//!     assert_eq!(serde_json::to_string(&test).unwrap(), expected);
//!
//!     assert_eq!(
//!         serde_json::from_str::<NaiveUnixTimestampMs>(expected).unwrap(),
//!         NaiveUnixTimestampMs(NaiveDateTime::from_timestamp(123, 0))
//!     );
//!
//!     assert!(serde_json::from_str::<NaiveUnixTimestampMs>("invalid").is_err());
//!     assert!(serde_json::from_str::<NaiveUnixTimestampMs>("-123000").is_err());
//! }
//! ```
//!
//!

extern crate chrono;

use std::fmt;

use serde::de::{self, Deserializer};
use serde::ser::Serializer;

use self::chrono::NaiveDateTime;

pub fn serialize_naivedatetime_as_millis<S>(
    time: &NaiveDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u64(time.timestamp_millis().max(0) as u64)
}

pub fn deserialize_naivedatetime_from_millis<'de, D>(
    deserializer: D,
) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = NaiveDateTime;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "Expected unix timestamp in milliseconds")
        }

        fn visit_u64<E>(self, millis: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let secs = millis / 1000;
            let nsecs = (millis % 1000) * 1000000;

            NaiveDateTime::from_timestamp_opt(secs as i64, nsecs as u32)
                .ok_or_else(|| E::custom("invalid or out-of-range datetime"))
        }
    }

    deserializer.deserialize_u64(Visitor)
}
