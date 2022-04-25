//! The things needed to sync components.
use bevy::prelude::Component;
use carrier_pigeon::net::CIdSpec;
use std::any::Any;
use std::marker::PhantomData;
use serde::{Serialize, Deserialize};

/// A component that tells `bevy-pigeon` to sync the component `T`
/// which is sent as `M`.
#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct NetComp<T, M = T>
where
    T: Clone + Into<M> + Component,
    M: Clone + Into<T> + Any + Send + Sync,
{
    // TODO: Add option for changed only.
    pub dir: NetDirection,
    _pd: PhantomData<(T, M)>,
}

impl<T, M> Default for NetComp<T, M>
where
    T: Clone + Into<M> + Component,
    M: Clone + Into<T> + Any + Send + Sync,
{
    fn default() -> Self {
        NetComp {
            dir: NetDirection::From(CIdSpec::All),
            _pd: PhantomData,
        }
    }
}

impl<T, M> NetComp<T, M>
where
    T: Clone + Into<M> + Component,
    M: Clone + Into<T> + Any + Send + Sync,
{
    /// Creates a new [`NetComp`] with the given [`NetDirection`].
    pub fn new(dir: NetDirection) -> Self {
        NetComp {
            dir,
            _pd: PhantomData::default(),
        }
    }
}

/// The synchronizing direction for data.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum NetDirection {
    /// Synchronize data **to** the peer, from this instance.
    ///
    /// On a server, the [`CIdSpec`] is used to specify who to send the data to.
    To(CIdSpec),
    /// Synchronize data **from** the peer, to this instance.
    ///
    /// On a server, the [`CIdSpec`] is used to specify who to receive the data from.
    From(CIdSpec),
    /// Synchronize data to the peer, and form the peer. **This option is not valid on a client.**
    ///
    /// On a server, the [`CIdSpec`]s are used to specify who to send/receive the data to/from.
    ToFrom(CIdSpec, CIdSpec),
}

impl NetDirection {
    /// Shorthand for [`NetDirection::To(CIdSpec::All)`].
    pub fn to() -> Self {
        NetDirection::To(CIdSpec::All)
    }

    /// Shorthand for [`NetDirection::From(CIdSpec::All)`].
    pub fn from() -> Self {
        NetDirection::From(CIdSpec::All)
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
