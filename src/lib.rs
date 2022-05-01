//! # bevy-pigeon
//!
//! The bevy plugin for `carrier-pigeon`.
//!
//! Building on `carrier-pigeon`, this crate provides high level network abstractions to allow making a game a breeze.
//! `bevy-pigeon` takes care of most of the networking for you, so you barely have to see it or think about it.
//!
//! ## Examples
//!
//! Complete examples are provided in the
//! [`examples/` directory](https://github.com/MitchellMarinoDev/carrier-pigeon/blob/main/examples)
//! on the GitHub repo.

mod app;
pub mod sync;
pub mod types;

pub use app::{AppExt, ClientPlugin, ServerPlugin};
