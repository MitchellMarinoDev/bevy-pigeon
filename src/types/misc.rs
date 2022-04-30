#![allow(deprecated)]

//! Types in this file:
//! - [Transform]
//! - [OrthographicProjection]
//! - [Name]
//! - [Visibility]
//! - [AlphaMode]
//! - [EulerRot]

// TODO: more transforms.

use serde::{Serialize, Deserialize};
use bevy::reflect::FromReflect;
use bevy::prelude::*;
use bevy::render::camera::{DepthCalculation, ScalingMode, WindowOrigin};

/// The network-able version of [Transform].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Reflect, FromReflect)]
#[reflect(PartialEq, Serialize, Deserialize)]
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

/// The network-able version of [OrthographicProjection].
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub struct NetOrthographicProjection {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
    pub window_origin: WindowOrigin,
    pub scaling_mode: ScalingMode,
    pub scale: f32,
    pub depth_calculation: DepthCalculation,
}

impl From<OrthographicProjection> for NetOrthographicProjection {
    fn from(o: OrthographicProjection) -> Self {
        NetOrthographicProjection {
            left: o.left,
            right: o.right,
            bottom: o.bottom,
            top: o.top,
            near: o.near,
            far: o.far,
            window_origin: o.window_origin,
            scaling_mode: o.scaling_mode,
            scale: o.scale,
            depth_calculation: o.depth_calculation,
        }
    }
}

impl From<NetOrthographicProjection> for OrthographicProjection {
    fn from(o: NetOrthographicProjection) -> Self {
        OrthographicProjection {
            left: o.left,
            right: o.right,
            bottom: o.bottom,
            top: o.top,
            near: o.near,
            far: o.far,
            window_origin: o.window_origin,
            scaling_mode: o.scaling_mode,
            scale: o.scale,
            depth_calculation: o.depth_calculation,
        }
    }
}

/// The network-able version of [Name].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect, FromReflect)]
#[reflect(PartialEq, Serialize, Deserialize)]
pub struct NetName {
    pub name: String,
}

impl From<Name> for NetName {
    fn from(o: Name) -> Self {
        NetName {
            name: o.as_str().into()
        }
    }
}

impl From<NetName> for Name {
    fn from(o: NetName) -> Self {
        Name::new(o.name)
    }
}

/// The network-able version of [Visibility].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Reflect, FromReflect)]
#[reflect(PartialEq, Serialize, Deserialize)]
pub struct NetVisibility {
    pub is_visible: bool,
}

impl From<Visibility> for NetVisibility {
    fn from(o: Visibility) -> Self {
        NetVisibility {
            is_visible: o.is_visible,
        }
    }
}

impl From<NetVisibility> for Visibility {
    fn from(o: NetVisibility) -> Self {
        Visibility {
            is_visible: o.is_visible,
        }
    }
}

/// The network-able version of [AlphaMode].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Reflect, FromReflect)]
#[reflect(PartialEq, Serialize, Deserialize)]
pub enum NetAlphaMode {
    Opaque,
    /// An alpha cutoff must be supplied where alpha values >= the cutoff
    /// will be fully opaque and < will be fully transparent
    Mask(f32),
    Blend,
}

impl From<AlphaMode> for NetAlphaMode {
    fn from(o: AlphaMode) -> Self {
        match o {
            AlphaMode::Opaque => NetAlphaMode::Opaque,
            AlphaMode::Mask(v) => NetAlphaMode::Mask(v),
            AlphaMode::Blend => NetAlphaMode::Blend,
        }
    }
}

impl From<NetAlphaMode> for AlphaMode {
    fn from(o: NetAlphaMode) -> Self {
        match o {
            NetAlphaMode::Opaque => AlphaMode::Opaque,
            NetAlphaMode::Mask(v) => AlphaMode::Mask(v),
            NetAlphaMode::Blend => AlphaMode::Blend,
        }
    }
}

/// The network-able version of [EulerRot].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Reflect, FromReflect)]
#[reflect(PartialEq, Serialize, Deserialize)]
pub enum NetEulerRot {
    /// Intrinsic three-axis rotation ZYX
    ZYX,
    /// Intrinsic three-axis rotation ZXY
    ZXY,
    /// Intrinsic three-axis rotation YXZ
    YXZ,
    /// Intrinsic three-axis rotation YZX
    YZX,
    /// Intrinsic three-axis rotation XYZ
    XYZ,
    /// Intrinsic three-axis rotation XZY
    XZY,
    /// Intrinsic two-axis rotation ZYZ
    #[deprecated(note = "Untested! Use at own risk!")]
    ZYZ,
    /// Intrinsic two-axis rotation ZXZ
    #[deprecated(note = "Untested! Use at own risk!")]
    ZXZ,
    /// Intrinsic two-axis rotation YXY
    #[deprecated(note = "Untested! Use at own risk!")]
    YXY,
    /// Intrinsic two-axis rotation YZY
    #[deprecated(note = "Untested! Use at own risk!")]
    YZY,
    /// Intrinsic two-axis rotation XYX
    #[deprecated(note = "Untested! Use at own risk!")]
    XYX,
    /// Intrinsic two-axis rotation XZX
    #[deprecated(note = "Untested! Use at own risk!")]
    XZX,
}

impl From<EulerRot> for NetEulerRot {
    fn from(o: EulerRot) -> Self {
        match o {
            EulerRot::ZYX => NetEulerRot::ZYX,
            EulerRot::ZXY => NetEulerRot::ZXY,
            EulerRot::YXZ => NetEulerRot::YXZ,
            EulerRot::YZX => NetEulerRot::YZX,
            EulerRot::XYZ => NetEulerRot::XYZ,
            EulerRot::XZY => NetEulerRot::XZY,

            EulerRot::ZYZ => NetEulerRot::ZYZ,
            EulerRot::ZXZ => NetEulerRot::ZXZ,
            EulerRot::YXY => NetEulerRot::YXY,
            EulerRot::YZY => NetEulerRot::YZY,
            EulerRot::XYX => NetEulerRot::XYX,
            EulerRot::XZX => NetEulerRot::XZX,
        }
    }
}

impl From<NetEulerRot> for EulerRot {
    fn from(o: NetEulerRot) -> Self {
        match o {
            NetEulerRot::ZYX => EulerRot::ZYX,
            NetEulerRot::ZXY => EulerRot::ZXY,
            NetEulerRot::YXZ => EulerRot::YXZ,
            NetEulerRot::YZX => EulerRot::YZX,
            NetEulerRot::XYZ => EulerRot::XYZ,
            NetEulerRot::XZY => EulerRot::XZY,

            NetEulerRot::ZYZ => EulerRot::ZYZ,
            NetEulerRot::ZXZ => EulerRot::ZXZ,
            NetEulerRot::YXY => EulerRot::YXY,
            NetEulerRot::YZY => EulerRot::YZY,
            NetEulerRot::XYX => EulerRot::XYX,
            NetEulerRot::XZX => EulerRot::XZX,
        }
    }
}