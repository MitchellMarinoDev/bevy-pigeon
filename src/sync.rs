//! The things needed to sync components.

use bevy::prelude::Component;
use carrier_pigeon::net::CIdSpec;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::marker::PhantomData;

/// A component that tells `bevy-pigeon` to sync the component `T` which is sent as `M`.
#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct NetComp<T, M = T>
where
    T: Clone + Into<M> + Component,
    M: Clone + Into<T> + Any + Send + Sync,
{
    /// Change detection.
    ///
    /// If enabled, this only sends a message if the component changed. This uses bevy's change
    /// detection, which may detect false positives.
    pub cd: bool,
    /// The timestamp of the last message received and written to this component.
    pub last: Option<u32>,
    /// The net direction for the client.
    pub c_dir: CNetDir,
    /// The net direction for the server.
    pub s_dir: SNetDir,
    _pd: PhantomData<(T, M)>,
}

impl<T, M> Default for NetComp<T, M>
where
    T: Clone + Into<M> + Component,
    M: Clone + Into<T> + Any + Send + Sync,
{
    fn default() -> Self {
        NetComp {
            cd: true,
            last: None,
            c_dir: CNetDir::From,
            s_dir: SNetDir::To(CIdSpec::All),
            _pd: PhantomData,
        }
    }
}

impl<T, M> NetComp<T, M>
where
    T: Clone + Into<M> + Component,
    M: Clone + Into<T> + Any + Send + Sync,
{
    /// Creates a new [`NetComp`] with the given net directions.
    /// Change detection (cd) defaults to true.
    pub fn new(cd: bool, c_dir: CNetDir, s_dir: SNetDir) -> Self {
        NetComp {
            cd,
            last: None,
            c_dir,
            s_dir,
            _pd: PhantomData,
        }
    }
}

/// Client Net Direction.
///
/// The synchronizing direction for data on the Client.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum CNetDir {
    /// Synchronize data **to** the peer, from this instance.
    To,
    /// Synchronize data **from** the peer, to this instance.
    From,
}

/// Server Net Direction.
///
/// The synchronizing direction for data on the Server.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum SNetDir {
    /// Synchronize data **to** the peer, from this instance.
    ///
    /// The [`CIdSpec`] is used to specify who to send the data to.
    To(CIdSpec),
    /// Synchronize data **from** the peer, to this instance.
    ///
    /// The [`CIdSpec`] is used to specify who to receive the data from.
    From(CIdSpec),
    /// Synchronize data **to** and **from** the peer, from this instance.
    ///
    /// The [`CIdSpec`]s are used to specify who to send/receive the data to/from.
    ToFrom(CIdSpec, CIdSpec),
}

impl SNetDir {
    /// Shorthand for [`SNetDir::To(CIdSpec::All)`].
    pub fn to_all() -> Self {
        SNetDir::To(CIdSpec::All)
    }

    /// Shorthand for [`SNetDir::From(CIdSpec::All)`].
    pub fn from_all() -> Self {
        SNetDir::From(CIdSpec::All)
    }

    /// Gets the to component of the `SNetDir`.
    pub fn to(&self) -> Option<&CIdSpec> {
        match self {
            SNetDir::To(spec) => Some(spec),
            SNetDir::From(_) => None,
            SNetDir::ToFrom(spec, _) => Some(spec),
        }
    }

    /// Gets the to component of the `SNetDir`.
    pub fn from(&self) -> Option<&CIdSpec> {
        match self {
            SNetDir::To(_) => None,
            SNetDir::From(spec) => Some(spec),
            SNetDir::ToFrom(_, spec) => Some(spec),
        }
    }
}

/// A networked entity.
///
/// Used to link entities across connected instances.
///
/// Any entity using [`NetComp`] needs to have one of these.
#[derive(Component, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct NetEntity {
    /// A unique identifier that needs to be the same on all connected instances of the game.
    /// A random `u64` provides a very low collision rate.
    pub id: u64,
}

impl NetEntity {
    /// Creates a new [`NetEntity`] with `id`.
    pub fn new(id: u64) -> Self {
        NetEntity { id }
    }
}

/// The message type to be sent.
///
/// This wraps the component message type with the entity's `id`.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub(crate) struct NetCompMsg<M: Any + Send + Sync> {
    pub(crate) id: u64,
    pub(crate) msg: M,
}

impl<M: Any + Send + Sync> NetCompMsg<M> {
    pub(crate) fn new(id: u64, msg: M) -> Self {
        NetCompMsg { id, msg }
    }
}
