//! ## Community Driven Serde Shims
//!
//! Many crates prefer to avoid natively providing Serde integration on the basis
//! it could be unstable in the future, be annoying to maintain, or stifles
//! innovation if there is ever a Serde competitor in the Rust serialization world.
//!
//! However, keeping track of dozens or hundreds of lines of custom shim functions in
//! my own projects has proven to be as much or more frustrating than if I were to
//! add that functionality to those crates directly. The innability to share code between
//! binaries or libraries is especially painful.
//!
//! So, as a compromise, I present this crate as a way to provide
//! `serialize_with`/`deserialize_with` functions for crates without
//! native Serde implementations.
//!
//! To enable these shims, simply add the crate to your `Cargo.toml`
//! with the desired crate shims as the crate features:
//!
//! ```toml
//! [dependencies]
//! serde_shims = { version = "*", features = ["std", "mime"] }
//! ```
//!

extern crate serde;

#[cfg(feature = "std")]
pub mod std_shims;

#[cfg(feature = "mime")]
pub mod mime;

#[cfg(feature = "chrono")]
pub mod chrono;
