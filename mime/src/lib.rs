//! Mime crate Serde shim
//!
//! To enable the Mime shim, add it as a dependency:
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
//!     mime: Mime,
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
//!
//! Or you could use the provided `Wrapper` type:
//!
//! ```
//! #[macro_use]
//! extern crate serde_derive;
//! extern crate serde_json;
//! extern crate mime_serde_shim;
//! extern crate mime;
//!
//! use mime_serde_shim::Wrapper as Mime;
//!
//! use mime::TEXT_JAVASCRIPT;
//!
//! #[derive(Debug, PartialEq, Serialize, Deserialize)]
//! struct MimeTest {
//!     mime: Option<Vec<Mime>>, // Wrapper easily supports nested types
//! }
//!
//! fn main() {
//!     let test = MimeTest {
//!         mime: Some(vec![TEXT_JAVASCRIPT.into()]), // Note the `.into()`
//!     };
//!
//!     let expected = r#"{"mime":["text/javascript"]}"#;
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

use std::str::FromStr;
use std::{fmt, ops};

use serde::de::{self, Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

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

/// Wrapper type for `Mime` that provides Serde functionality
/// while attempting to remain transparent.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Wrapper(pub Mime);

impl Serialize for Wrapper {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self::serialize(&self.0, serializer)
    }
}

impl<'de> Deserialize<'de> for Wrapper {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Wrapper, D::Error>
    where
        D: Deserializer<'de>,
    {
        self::deserialize(deserializer).map(|mime| Wrapper(mime))
    }
}

impl ops::Deref for Wrapper {
    type Target = Mime;

    #[inline]
    fn deref(&self) -> &Mime {
        &self.0
    }
}

impl ops::DerefMut for Wrapper {
    #[inline]
    fn deref_mut(&mut self) -> &mut Mime {
        &mut self.0
    }
}

impl From<Mime> for Wrapper {
    #[inline]
    fn from(mime: Mime) -> Wrapper {
        Wrapper(mime)
    }
}

impl Into<Mime> for Wrapper {
    #[inline]
    fn into(self) -> Mime {
        self.0
    }
}

impl fmt::Debug for Wrapper {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for Wrapper {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl AsRef<str> for Wrapper {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl AsRef<Mime> for Wrapper {
    #[inline]
    fn as_ref(&self) -> &Mime {
        &self.0
    }
}

impl FromStr for Wrapper {
    type Err = mime::FromStrError;

    #[inline]
    fn from_str(s: &str) -> Result<Wrapper, Self::Err> {
        Mime::from_str(s).map(|mime| Wrapper(mime))
    }
}

impl<'a> PartialEq<&'a str> for Wrapper {
    #[inline]
    fn eq(&self, s: &&'a str) -> bool {
        self.0.eq(s)
    }
}

impl<'a> PartialEq<Wrapper> for &'a str {
    #[inline]
    fn eq(&self, mime: &Wrapper) -> bool {
        self.eq(&mime.0)
    }
}

impl PartialEq<Mime> for Wrapper {
    #[inline]
    fn eq(&self, mime: &Mime) -> bool {
        self.0.eq(mime)
    }
}
