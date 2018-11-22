serde_shims [![Build Status](https://travis-ci.org/novacrazy/serde_shims.svg?branch=master)](https://travis-ci.org/novacrazy/serde_shims)
=============

## Community Driven Serde Shims

#### [Documentation](https://novacrazy.github.io/serde_shims/)

Many crates prefer to avoid natively providing Serde integration on the basis
it could be unstable in the future, be annoying to maintain, or stifles
innovation if there is ever a Serde competitor in the Rust serialization world.

However, keeping track of dozens or hundreds of lines of custom shim functions in
my own projects has proven to be as much or more frustrating than if I were to
add that functionality to those crates directly. The innability to share code between
binaries or libraries is especially painful.

So, as a compromise, I present this crate as a way to provide
`serialize_with`/`deserialize_with` functions or implementation macros for crates without
native Serde implementations.

To enable these shims, simply add the crate to your `Cargo.toml`:

```toml
[dependencies]
bitflags_serde_shim = "0.2"
enum_primitive_serde_shim = "0.2"
mime_serde_shim = "0.2"
std_serde_shims = "0.2"
```

If there is a crate you'd like to have a Serde shim for,
or would like to contribute your own, feel free to open an issue or pull request!