//! # bevy_pigeon
//!
//! The bevy plugin for `carrier_pigeon`.
//!
//! Building on `carrier_pigeon`, this crate provides high level network abstractions to allow making a game a breeze.
//! `bevy_pigeon` takes care of most of the networking for you, so you barely have to see it or think about it.
//!
//! ### Add bevy_pigeon to your `Cargo.toml`:
//!
//! `bevy_pigeon = "0.3.0"`
//!
//! ## Examples
//!
//! Complete examples are provided in the
//! [`examples/` directory](https://github.com/MitchellMarinoDev/carrier_pigeon/blob/main/examples)
//! on the GitHub repo.

#[warn(missing_debug_implementations, missing_copy_implementations)]
pub mod app;
pub mod sync;
#[cfg(feature = "types")]
pub mod types;

pub use app::{AppExt, ClientPlugin, NetLabel, ServerPlugin, SyncC};
