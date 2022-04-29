//! Provides network-able types for common bevy types.

use bevy::prelude::*;

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub struct NetTransform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl From<Transform> for NetTransform {
    fn from(o: Transform) -> Self {
        MyTransform {
            translation: o.translation,
            rotation: o.rotation,
            scale: o.scale,
        }
    }
}

impl From<NetTransform> for Transform {
    fn from(o: MyTransform) -> Self {
        Transform {
            translation: o.translation,
            rotation: o.rotation,
            scale: o.scale,
        }
    }
}
