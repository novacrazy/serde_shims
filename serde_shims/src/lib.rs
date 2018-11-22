//! Meta package for all Serde shims, mostly for documentation but does re-export shims like version `0.1`.
//!
//! To use the crate individually:
//!
//! ```toml
//! [dependencies]
//! bitflags_serde_shim = "0.2"
//! enum_primitive_serde_shim = "0.2"
//! mime_serde_shim = "0.2"
//! std_serde_shims = "0.2"
//! ```
//!
//! or as a legacy feature-based import with this meta crate:
//!
//! ```toml
//! [dependencies]
//! serde_shims = { version = "0.2", features = ["std", "mime"] }
//! ```

/// `bitflags` crate Serde shims
#[cfg(feature = "bitflags")]
pub extern crate bitflags_serde_shim as bitflags;

/// `enum_primitive` crate Serde shims
#[cfg(feature = "enum_primitive")]
pub extern crate enum_primitive_serde_shim as enum_primitive;

/// Mime crate Serde shim
#[cfg(feature = "mime")]
pub extern crate mime_serde_shim as mime;

/// Potentially useful shims for the standard library
#[cfg(feature = "std")]
pub extern crate std_serde_shims as std_shims;

#[cfg(feature = "bitflags")]
pub use bitflags::impl_serde_for_bitflags;

#[cfg(feature = "enum_primitive")]
pub use enum_primitive::impl_serde_for_enum_primitive;
