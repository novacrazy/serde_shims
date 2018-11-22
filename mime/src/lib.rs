//! Mime crate Serde shim
//!
//! To enable the Mime shim, add it to the crate features list:
//!
//! ```toml
//! [dependencies]
//! mime_serde_shim = "0.2"
//! ```
//!
//! Full example:
//!
//! ```
//! #[macro_use]
//! extern crate serde_derive;
//! extern crate serde_json;
//! extern crate mime_serde_shim;
//!
//! extern crate mime;
//!
//! use mime::{Mime, TEXT_JAVASCRIPT};
//!
//! #[derive(Debug, PartialEq, Serialize, Deserialize)]
//! struct MimeTest {
//!     #[serde(with = "mime_serde_shim")]
//!     mime: mime::Mime,
//! }
//!
//! fn main() {
//!     let test = MimeTest {
//!         mime: TEXT_JAVASCRIPT,
//!     };
//!
//!     let expected = r#"{"mime":"text/javascript"}"#;
//!
//!     assert_eq!(serde_json::to_string(&test).unwrap(), expected);
//!
//!     assert_eq!(serde_json::from_str::<MimeTest>(expected).unwrap(), test);
//!
//!     assert!(serde_json::from_str::<MimeTest>("invalid").is_err());
//! }
//! ```

extern crate mime;
extern crate serde;

use std::fmt;
use std::str::FromStr;

use serde::de::{self, Deserializer};
use serde::ser::Serializer;

use mime::Mime;

pub fn serialize<S>(mime: &Mime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(mime.as_ref())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Mime, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = Mime;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid MIME type")
        }

        fn visit_str<E>(self, value: &str) -> Result<Mime, E>
        where
            E: de::Error,
        {
            Mime::from_str(value).or_else(|e| Err(E::custom(format!("{}", e))))
        }
    }

    deserializer.deserialize_str(Visitor)
}