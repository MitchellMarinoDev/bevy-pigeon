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

use crate::tick::{client_tick, server_tick};
use bevy::prelude::*;

mod app;
pub mod sync;
mod tick;

pub use app::AppExt;

pub struct ClientPlugin;
pub struct ServerPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::First, client_tick);
    }
}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::First, server_tick);
    }
}
