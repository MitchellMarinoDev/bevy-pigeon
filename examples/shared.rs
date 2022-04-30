//! Code shared between the examples to reduce duplicated code.
//! Contains things like messages that can be useful for multiple examples.

use serde::{Serialize, Deserialize};
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

impl Connection {
    pub fn new(user: impl Into<String>, pass: Option<impl Into<String>>) -> Connection {
        let user = user.into();
        let pass = pass.map(|i| i.into());
        Connection { user, pass }
    }
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
