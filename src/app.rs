//! Contains the plugins, systems, and components for the bevy app.

use crate::sync::{CNetDir, NetCompMsg, SNetDir};
use crate::sync::{NetComp, NetEntity};
use bevy::prelude::*;
use carrier_pigeon::net::{CIdSpec, NetMsg};
use carrier_pigeon::{Client, MsgRegError, MsgTable, Server, SortedMsgTable, Transport};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::any::Any;
use std::marker::PhantomData;

/// An event that forces a sync of component `T`.
///
/// This can be used if you need to force a sync of component `T` with message type `M`. This is
/// most useful if you are using the change detection; you may want to force a sync of components
/// when a new client joins.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Default)]
pub struct SyncC<T> {
    _pd: PhantomData<T>,
}

/// A label that is applied to all networking systems.
#[derive(SystemLabel, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash)]
pub struct NetLabel;

/// The client plugin.
///
/// Automatically clears client's message buffer and receive new messages at the start of every
/// frame.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash)]
pub struct ClientPlugin;

/// The server plugin.
///
/// Automatically clears server's message buffer and receive new messages at the start of every
/// frame.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash)]
pub struct ServerPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::First, client_tick.label(NetLabel));
    }
}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::First, server_tick.label(NetLabel));
    }
}

/// Clears client's message buffer and receive new messages.
pub fn client_tick(client: Option<ResMut<Client>>) {
    if let Some(mut client) = client {
        client.clear_msgs();
        client.recv_msgs();
    }
}

/// Clears server's message buffer and receive new messages.
pub fn server_tick(server: Option<ResMut<Server>>) {
    if let Some(mut server) = server {
        server.clear_msgs();
        server.recv_msgs();
    }
}

/// An extension trait for easy registering [`NetComp`] types.
pub trait AppExt {
    /// Adds everything needed to sync component `T` using message type `M`.
    ///
    /// Registers the type `NetCompMsg<M>` into `table` and adds the system required to sync
    /// components of type `T`, using type `M` to send.
    ///
    /// Types `T` and `M` ***can*** be the same type; if the component `T` implements all the
    /// required traits, you may use it as `M`.
    ///
    /// ### Panics
    /// panics if `NetCompMsg<M>` is already registered in the table
    /// (If you call this method twice with the same `M`).
    fn sync_comp<T, M>(&mut self, table: &mut MsgTable, transport: Transport) -> &mut Self
    where
        T: Clone + Into<M> + Component,
        M: Clone + Into<T> + Any + Send + Sync + Serialize + DeserializeOwned;

    /// Adds everything needed to sync component `T` using message type `M`.
    ///
    /// Same as [`sync_comp()`](App::sync_comp), but doesn't panic in the event of a [`MsgRegError`].
    fn try_sync_comp<T, M>(
        &mut self,
        table: &mut MsgTable,
        transport: Transport,
    ) -> Result<&mut Self, MsgRegError>
    where
        T: Clone + Into<M> + Component,
        M: Clone + Into<T> + Any + Send + Sync + Serialize + DeserializeOwned;

    /// Adds everything needed to sync component `T` using message type `M`.
    ///
    /// Registers the type `NetCompMsg<M>` into `table` and adds the system required to sync
    /// components of type `T`, using type `M` to send.
    ///
    /// Types `T` and `M` ***can*** be the same type; if the component `T` implements all the
    /// required traits, you may use it as `M`.
    ///
    /// ### Panics
    /// panics if `NetCompMsg<M>` is already registered in the table
    /// (If you call this method twice with the same `M`).
    fn sync_comp_sorted<T, M>(
        &mut self,
        table: &mut SortedMsgTable,
        transport: Transport,
    ) -> &mut Self
    where
        T: Clone + Into<M> + Component,
        M: Clone + Into<T> + Any + Send + Sync + Serialize + DeserializeOwned;

    /// Adds everything needed to sync component `T` using message type `M`.
    ///
    /// Same as [`sync_comp()`](App::sync_comp), but doesn't panic in the event of a [`MsgRegError`].
    fn try_sync_comp_sorted<T, M>(
        &mut self,
        table: &mut SortedMsgTable,
        transport: Transport,
    ) -> Result<&mut Self, MsgRegError>
    where
        T: Clone + Into<M> + Component,
        M: Clone + Into<T> + Any + Send + Sync + Serialize + DeserializeOwned;
}

impl AppExt for App {
    /// Adds everything needed to sync component `T` using message type `M`.
    ///
    /// Registers the type `NetCompMsg<M>` into `table` and adds the system required to sync
    /// components of type `T`, using type `M` to send.
    ///
    /// Types `T` and `M` ***can*** be the same type; if the component `T` implements all the
    /// required traits, you may use it as `M`.
    ///
    /// ### Panics
    /// panics if `NetCompMsg<M>` is already registered in the table
    /// (If you call this method twice with the same `M`).
    fn sync_comp<T, M>(&mut self, table: &mut MsgTable, transport: Transport) -> &mut Self
    where
        T: Clone + Into<M> + Component,
        M: Clone + Into<T> + Any + Send + Sync + Serialize + DeserializeOwned,
    {
        table.register::<NetCompMsg<M>>(transport).unwrap();

        self.add_event::<SyncC<T>>();
        self.add_system(send_on_event::<T, M>.label(NetLabel));
        self.add_system(comp_send::<T, M>.label(NetLabel));
        self.add_system(comp_recv::<T, M>.label(NetLabel));
        self
    }

    /// Adds everything needed to sync component `T` using message type `M`.
    ///
    /// Same as [`sync_comp()`](App::sync_comp), but doesn't panic in the event of a [`MsgRegError`].
    fn try_sync_comp<T, M>(
        &mut self,
        table: &mut MsgTable,
        transport: Transport,
    ) -> Result<&mut Self, MsgRegError>
    where
        T: Clone + Into<M> + Component,
        M: Clone + Into<T> + Any + Send + Sync + Serialize + DeserializeOwned,
    {
        table.register::<NetCompMsg<M>>(transport)?;

        self.add_event::<SyncC<T>>();
        self.add_system(send_on_event::<T, M>.label(NetLabel));
        self.add_system(comp_send::<T, M>.label(NetLabel));
        self.add_system(comp_recv::<T, M>.label(NetLabel));
        Ok(self)
    }

    /// Adds everything needed to sync component `T` using message type `M`.
    ///
    /// Registers the type `NetCompMsg<M>` into `table` and adds the system required to sync
    /// components of type `T`, using type `M` to send.
    ///
    /// Types `T` and `M` ***can*** be the same type; if the component `T` implements all the
    /// required traits, you may use it as `M`.
    ///
    /// ### Panics
    /// panics if `NetCompMsg<M>` is already registered in the table
    /// (If you call this method twice with the same `M`).
    fn sync_comp_sorted<T, M>(
        &mut self,
        table: &mut SortedMsgTable,
        transport: Transport,
    ) -> &mut Self
    where
        T: Clone + Into<M> + Component,
        M: Clone + Into<T> + Any + Send + Sync + Serialize + DeserializeOwned,
    {
        let id = "bevy-pigeon::".to_owned() + std::any::type_name::<M>();
        table.register::<NetCompMsg<M>>(transport, &*id).unwrap();

        self.add_event::<SyncC<T>>();
        self.add_system(send_on_event::<T, M>.label(NetLabel));
        self.add_system(comp_send::<T, M>.label(NetLabel));
        self.add_system(comp_recv::<T, M>.label(NetLabel));
        self
    }

    /// Adds everything needed to sync component `T` using message type `M`.
    ///
    /// Same as [`sync_comp()`](App::sync_comp), but doesn't panic in the event of a [`MsgRegError`].
    fn try_sync_comp_sorted<T, M>(
        &mut self,
        table: &mut SortedMsgTable,
        transport: Transport,
    ) -> Result<&mut Self, MsgRegError>
    where
        T: Clone + Into<M> + Component,
        M: Clone + Into<T> + Any + Send + Sync + Serialize + DeserializeOwned,
    {
        let id = "bevy-pigeon::".to_owned() + std::any::type_name::<M>();
        table.register::<NetCompMsg<M>>(transport, &*id)?;

        self.add_event::<SyncC<T>>();
        self.add_system(send_on_event::<T, M>.label(NetLabel));
        self.add_system(comp_send::<T, M>.label(NetLabel));
        self.add_system(comp_recv::<T, M>.label(NetLabel));
        Ok(self)
    }
}

/// A system that forces a sync of a certain component.
fn send_on_event<T, M>(
    mut er: EventReader<SyncC<T>>,
    server: Option<ResMut<Server>>,
    client: Option<ResMut<Client>>,
    q: Query<(&NetEntity, &NetComp<T, M>, &T)>,
) where
    T: Clone + Into<M> + Component,
    M: Clone + Into<T> + Any + Send + Sync,
{
    if er.iter().count() == 0 {
        return;
    }
    trace!("Force Syncing {}", std::any::type_name::<T>());

    // Almost copy-paste from [`comp_send`] ignoring change detection
    if let Some(server) = server {
        for (net_e, net_c, comp) in q.iter() {
            if let Some(to_spec) = net_c.s_dir.to() {
                if let Err(e) = server.send_spec(
                    *to_spec,
                    &NetCompMsg::<M>::new(net_e.id, comp.clone().into()),
                ) {
                    error!("{}", e);
                }
            }
        }
    } else if let Some(client) = client {
        for (net_e, net_c, comp) in q.iter() {
            if let CNetDir::To = net_c.c_dir {
                if let Err(e) = client.send(&NetCompMsg::<M>::new(net_e.id, comp.clone().into())) {
                    error!("{}", e);
                }
            }
        }
    }
}

/// A system that sends component `T` using messages of type `M`.
///
/// Most of the time, you will call [`sync_comp`](AppExt::sync_comp) which will add this system.
/// Only add it manually if you know what you are doing and want custom control over when it runs.
pub fn comp_send<T, M>(
    server: Option<ResMut<Server>>,
    client: Option<ResMut<Client>>,
    q: Query<(&NetEntity, &NetComp<T, M>, &T, ChangeTrackers<T>)>,
) where
    T: Clone + Into<M> + Component,
    M: Clone + Into<T> + Any + Send + Sync,
{
    if let Some(server) = server {
        for (net_e, net_c, comp, ct) in q.iter() {
            // If we are using change detection, and the component hasn't been changed, skip.
            if net_c.cd && !ct.is_changed() {
                continue;
            }

            if let Some(to_spec) = net_c.s_dir.to() {
                if let Err(e) = server.send_spec(
                    *to_spec,
                    &NetCompMsg::<M>::new(net_e.id, comp.clone().into()),
                ) {
                    error!("{}", e);
                }
            }
        }
    } else if let Some(client) = client {
        for (net_e, net_c, comp, ct) in q.iter() {
            // If we are using change detection, and the component hasn't been changed, skip.
            if net_c.cd && !ct.is_changed() {
                continue;
            }

            if let CNetDir::To = net_c.c_dir {
                if let Err(e) = client.send(&NetCompMsg::<M>::new(net_e.id, comp.clone().into())) {
                    error!("{}", e);
                }
            }
        }
    }
}

/// A system that receives messages of type `M` and applies it to component `T`.
///
/// Most of the time, you will call [`sync_comp`](AppExt::sync_comp) which will add this system.
/// Only add it manually if you know what you are doing and want custom control over when it runs.
pub fn comp_recv<T, M>(
    server: Option<ResMut<Server>>,
    client: Option<ResMut<Client>>,
    mut q: Query<(&NetEntity, &mut NetComp<T, M>, &mut T)>,
) where
    T: Clone + Into<M> + Component,
    M: Clone + Into<T> + Any + Send + Sync,
{
    if let Some(server) = server {
        // Cache messages
        let msgs: Vec<NetMsg<NetCompMsg<M>>> = server.recv::<NetCompMsg<M>>().collect();
        for (net_e, mut net_c, mut comp) in q.iter_mut() {
            if let Some(&spec) = net_c.s_dir.from() {
                if let Some(valid_msg) = get_latest_msg(&msgs, net_c.last, spec, net_e.id) {
                    net_c.last = valid_msg.time;
                    *comp = valid_msg.msg.clone().into();
                }
            }
            // Warn on overlap
            if let SNetDir::ToFrom(to_spec, from_spec) = net_c.s_dir {
                if to_spec.overlaps(from_spec) {
                    warn!("NetEntity {{ id: {} }} has overlapping `CIdSpec`s in NetDirection::ToFrom. Applying anyway.", net_e.id);
                }
            }
        }
    } else if let Some(client) = client {
        // Cache messages
        let msgs: Vec<NetMsg<NetCompMsg<M>>> = client.recv::<NetCompMsg<M>>().collect();
        for (net_e, mut net_c, mut comp) in q.iter_mut() {
            if net_c.c_dir == CNetDir::From {
                if let Some(valid_msg) = get_latest_msg(&msgs, net_c.last, CIdSpec::All, net_e.id) {
                    net_c.last = valid_msg.time;
                    *comp = valid_msg.msg.clone().into();
                }

                if let Some(valid_msg) = msgs.iter().filter(|msg| msg.id == net_e.id).last() {
                    *comp = valid_msg.msg.clone().into();
                }
            }
        }
    }
}

/// Helper function that gets the most recent message that matches `from_spec` for entity with `id`
/// if it is sent later that current.
fn get_latest_msg<'a, M: Any + Send + Sync>(
    msgs: &'a Vec<NetMsg<NetCompMsg<M>>>,
    current: Option<u32>,
    spec: CIdSpec,
    id: u64,
) -> Option<&'a NetMsg<'a, NetCompMsg<M>>> {
    let mut latest_time = current.unwrap_or(0);
    let mut latest = None;
    for m in msgs.iter().filter(|m| spec.matches(m.cid) && m.id == id) {
        if let Some(time) = m.time {
            // If this packet has a send time, get the last.
            if time > latest_time {
                latest_time = time;
                latest = Some(m);
            }
        } else {
            // If this does not have a send time, just get the last one received.
            latest = Some(m);
        }
    }
    latest
}
