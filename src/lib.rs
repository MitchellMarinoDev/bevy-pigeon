use crate::tick::{client_tick, server_tick};
use bevy::prelude::*;

mod app;
mod sync;
mod tick;

pub use app::AppExt;
pub use sync::NetComp;
pub use sync::NetDirection;
pub use sync::NetEntity;

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
