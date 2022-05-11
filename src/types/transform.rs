//! Multiple network-able types for transforms, all containing different combinations of fields.
//!
//! Types in this file:
//! - [NetTransform]
//! - [NetTransformTR]
//! - [NetTransformT]
//! - [NetTransform2d]
//! - [NetTransform2dTR]
//! - [NetTransform2dT]

use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// The network-able version of [Transform].
///
/// Contains all fields.
///
/// Several different versions with different fields are available:
/// - [NetTransform]
/// - [NetTransformTR]
/// - [NetTransformT]
/// - [NetTransform2d]
/// - [NetTransform2dTR]
/// - [NetTransform2dT]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NetTransform {
    /// Position of the entity. In 2d, the last value of the `Vec3` is used for z-ordering.
    pub translation: Vec3,
    /// Rotation of the entity.
    pub rotation: Quat,
    /// Scale of the entity.
    pub scale: Vec3,
}

impl From<Transform> for NetTransform {
    fn from(o: Transform) -> Self {
        NetTransform {
            translation: o.translation,
            rotation: o.rotation,
            scale: o.scale,
        }
    }
}

impl From<NetTransform> for Transform {
    fn from(o: NetTransform) -> Self {
        Transform {
            translation: o.translation,
            rotation: o.rotation,
            scale: o.scale,
        }
    }
}

/// The network-able version of [Transform].
///
/// Contains translation and rotation.
///
/// Only works if scale is always `Vec3::ONE`.
///
/// Several different versions with different fields are available:
/// - [NetTransform]
/// - [NetTransformTR]
/// - [NetTransformT]
/// - [NetTransform2d]
/// - [NetTransform2dTR]
/// - [NetTransform2dT]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NetTransformTR {
    /// Position of the entity. In 2d, the last value of the `Vec3` is used for z-ordering.
    pub translation: Vec3,
    /// Rotation of the entity.
    pub rotation: Quat,
}

impl From<Transform> for NetTransformTR {
    fn from(o: Transform) -> Self {
        NetTransformTR {
            translation: o.translation,
            rotation: o.rotation,
        }
    }
}

impl From<NetTransformTR> for Transform {
    fn from(o: NetTransformTR) -> Self {
        Transform {
            translation: o.translation,
            rotation: o.rotation,
            ..default()
        }
    }
}

/// The network-able version of [Transform].
///
/// Contains only translation.
///
/// Only works if rotation always is `Quat::identity()` and scale is always `Vec3::ONE`.
///
/// Several different versions with different fields are available:
/// - [NetTransform]
/// - [NetTransformTR]
/// - [NetTransformT]
/// - [NetTransform2d]
/// - [NetTransform2dTR]
/// - [NetTransform2dT]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NetTransformT {
    /// Position of the entity. In 2d, the last value of the `Vec3` is used for z-ordering.
    pub translation: Vec3,
}

impl From<Transform> for NetTransformT {
    fn from(o: Transform) -> Self {
        NetTransformT {
            translation: o.translation,
        }
    }
}

impl From<NetTransformT> for Transform {
    fn from(o: NetTransformT) -> Self {
        Transform {
            translation: o.translation,
            ..default()
        }
    }
}

/// The network-able version of [Transform].
///
/// Contains all fields, using `Vec2`s instead of `Vec3`s.
///
/// Only works if translation.z is always `0`, and scale.z is always `1`.
///
/// Several different versions with different fields are available:
/// - [NetTransform]
/// - [NetTransformTR]
/// - [NetTransformT]
/// - [NetTransform2d]
/// - [NetTransform2dTR]
/// - [NetTransform2dT]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NetTransform2d {
    /// Position of the entity. In 2d, the last value of the `Vec3` is used for z-ordering.
    pub translation: Vec2,
    /// Rotation of the entity.
    pub rotation: Quat,
    /// Scale of the entity.
    pub scale: Vec2,
}

impl From<Transform> for NetTransform2d {
    fn from(o: Transform) -> Self {
        NetTransform2d {
            translation: o.translation.xy(),
            rotation: o.rotation,
            scale: o.scale.xy(),
        }
    }
}

impl From<NetTransform2d> for Transform {
    fn from(o: NetTransform2d) -> Self {
        Transform {
            translation: o.translation.extend(0.0),
            rotation: o.rotation,
            scale: o.scale.extend(0.0),
        }
    }
}

/// The network-able version of [Transform].
///
/// Contains only translation and rotation, using `Vec2`s instead of `Vec3`s.
///
/// Only works if translation.z is always `0`, and scale is `Vec3::ONE`.
///
/// Several different versions with different fields are available:
/// - [NetTransform]
/// - [NetTransformTR]
/// - [NetTransformT]
/// - [NetTransform2d]
/// - [NetTransform2dTR]
/// - [NetTransform2dT]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NetTransform2dTR {
    /// Position of the entity. In 2d, the last value of the `Vec3` is used for z-ordering.
    pub translation: Vec2,
    /// Rotation of the entity.
    pub rotation: Quat,
}

impl From<Transform> for NetTransform2dTR {
    fn from(o: Transform) -> Self {
        NetTransform2dTR {
            translation: o.translation.xy(),
            rotation: o.rotation,
        }
    }
}

impl From<NetTransform2dTR> for Transform {
    fn from(o: NetTransform2dTR) -> Self {
        Transform {
            translation: o.translation.extend(0.0),
            rotation: o.rotation,
            ..default()
        }
    }
}

/// The network-able version of [Transform].
///
/// Contains only translation, using `Vec2`s instead of `Vec3`s.
///
/// Only works if rotation always is `Quat::identity()`, translation.z is always `0`,
/// and scale is `Vec3::ONE`.
///
/// Several different versions with different fields are available:
/// - [NetTransform]
/// - [NetTransformTR]
/// - [NetTransformT]
/// - [NetTransform2d]
/// - [NetTransform2dTR]
/// - [NetTransform2dT]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NetTransform2dT {
    /// Position of the entity. In 2d, the last value of the `Vec3` is used for z-ordering.
    pub translation: Vec2,
}

impl From<Transform> for NetTransform2dT {
    fn from(o: Transform) -> Self {
        NetTransform2dT {
            translation: o.translation.xy(),
        }
    }
}

impl From<NetTransform2dT> for Transform {
    fn from(o: NetTransform2dT) -> Self {
        Transform {
            translation: o.translation.extend(0.0),
            ..default()
        }
    }
}
