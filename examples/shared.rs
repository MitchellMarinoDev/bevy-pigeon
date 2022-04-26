//! Code shared between the examples to reduce boilerplate.
//! Contains things like messages that can be useful for multiple examples.

use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use carrier_pigeon::MsgTable;

pub fn get_table() -> MsgTable {
    let table = MsgTable::new();

    table
}


/// The connection message.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug, Default)]
pub struct Connection {
    user: String,
    pass: Option<String>,
}

/// The response message.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Response {
    Accepted,
    Rejected(RejectReason),
}

/// The Reason for being rejected.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
pub enum RejectReason {
    IncorrectPassword,
    MaxPlayersReached,
}

/// The disconnect message.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Disconnect {
    PlayerDisconnect,
    GameEnd,
}

/// A reduced [`Transform`] component that can be networked.
///
/// Scale is cut out to show a way to reduce bandwidth by getting
/// rid of unused parts of the component.
#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub struct NetTransform {
    pub translation: Vec3,
    pub rotation: Quat,
    // Scale not used.
}

impl From<Transform> for NetTransform {
    fn from(o: Transform) -> Self {
        NetTransform {
            translation: o.translation,
            rotation: o.rotation,
        }
    }
}

impl From<NetTransform> for Transform {
    fn from(o: NetTransform) -> Self {
        Transform {
            translation: o.translation,
            rotation: o.rotation,
            ..default()
        }
    }
}
