//! Code shared between the examples to reduce boilerplate.
//! Contains things like messages that can be useful for multiple examples.

/// The connection message.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug, Default)]
struct Connection {
    user: String,
    pass: Option<String>,
}

/// The response message.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug, Default)]
enum Response {
    Accepted,
    Rejected(RejectReason),
}

/// The Reason for being rejected.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug, Default)]
enum RejectReason {
    IncorrectPassword,
    MaxPlayersReached,
}

/// The disconnect message.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug, Default)]
enum Disconnect {
    PlayerDisconnect,
    GameEnd,
}