//! Provides network-able types for common bevy types.
//!
//! Types:
//!  - [Transform]
//!  - [OrthographicProjection]
//!  - [AmbientLight]
//!  - [DirectionalLight]
//!  - [PointLight]
//!  - [Name]
//!  - [Visibility]
//!  - [AlphaMode]
//!  - [EulerRot]
//!
//! If you think other network-able types would be helpful to many users,
//! and think it should be included here, please send a PR.

mod light;
mod misc;

pub use light::*;
pub use misc::*;
