//! The app extension.
use crate::sync::{CNetDir, NetCompMsg, SNetDir};
use bevy::prelude::*;
use carrier_pigeon::{Client, MsgRegError, MsgTable, Server, SortedMsgTable, Transport};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::any::Any;
use carrier_pigeon::net::{CIdSpec, NetMsg};
use crate::sync::{NetComp, NetEntity};

/// An extension trait for easy registering [`NetComp`] types.
pub trait AppExt {
    fn sync_comp<T, M>(
        &mut self,
        table: &mut MsgTable,
        transport: Transport
    ) -> &mut Self
    where
        T: Clone + Into<M> + Component,
        M: Clone + Into<T> + Any + Send + Sync + Serialize + DeserializeOwned;

    fn try_sync_comp<T, M>(
        &mut self,
        table: &mut MsgTable,
        transport: Transport,
    ) -> Result<&mut Self, MsgRegError>
    where
        T: Clone + Into<M> + Component,
        M: Clone + Into<T> + Any + Send + Sync + Serialize + DeserializeOwned;

    fn sync_comp_sorted<T, M>(
        &mut self, table: &mut SortedMsgTable, transport: Transport) -> &mut Self
        where
            T: Clone + Into<M> + Component,
            M: Clone + Into<T> + Any + Send + Sync + Serialize + DeserializeOwned;

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
    /// Registers the type `NetCompMsg<M>` into `table` and adds the
    /// system required to sync components of type `T`, using type `M`
    /// to send.
    ///
    /// Types `T` and `M` ***can*** be the same type; if the component `T`
    /// implements all the required traits, you may use it as `M`.
    ///
    /// ### Panics
    /// panics if `NetCompMsg<M>` is already registered in the table (If you
    /// call this method twice with the same `M`).
    fn sync_comp<T, M>(&mut self, table: &mut MsgTable, transport: Transport) -> &mut Self
    where
        T: Clone + Into<M> + Component,
        M: Clone + Into<T> + Any + Send + Sync + Serialize + DeserializeOwned,
    {
        table.register::<NetCompMsg<M>>(transport).unwrap();
        self.add_system(comp_send::<T, M>);
        self.add_system(comp_recv::<T, M>);
        self
    }

    /// Adds everything needed to sync component `T` using message type `M`.
    ///
    /// Same as [`sync_comp()`](App::sync_comp), but doesnt panic in the event of a [`MsgRegError`].
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
        self.add_system(comp_send::<T, M>);
        self.add_system(comp_recv::<T, M>);
        Ok(self)
    }

    /// Adds everything needed to sync component `T` using message type `M`.
    ///
    /// Registers the type `NetCompMsg<M>` into `table` and adds the
    /// system required to sync components of type `T`, using type `M`
    /// to send.
    ///
    /// Types `T` and `M` ***can*** be the same type; if the component `T`
    /// implements all the required traits, you may use it as `M`.
    ///
    /// ### Panics
    /// panics if `NetCompMsg<M>` is already registered in the table (If you
    /// call this method twice with the same `M`).
    fn sync_comp_sorted<T, M>(&mut self, table: &mut SortedMsgTable, transport: Transport) -> &mut Self
        where
            T: Clone + Into<M> + Component,
            M: Clone + Into<T> + Any + Send + Sync + Serialize + DeserializeOwned,
    {
        let id = "bevy-pigeon::".to_owned() + std::any::type_name::<M>();
        table.register::<NetCompMsg<M>>(transport, &*id).unwrap();
        self.add_system(comp_send::<T, M>);
        self.add_system(comp_recv::<T, M>);
        self
    }

    /// Adds everything needed to sync component `T` using message type `M`.
    ///
    /// Same as [`sync_comp()`](App::sync_comp), but doesnt panic in the event of a [`MsgRegError`].
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
        self.add_system(comp_send::<T, M>);
        self.add_system(comp_recv::<T, M>);
        Ok(self)
    }
}

fn comp_send<T, M>(
    server: Option<ResMut<Server>>,
    client: Option<ResMut<Client>>,
    mut q: Query<(&NetEntity, &NetComp<T, M>, &T, ChangeTrackers<T>)>,
) where
    T: Clone + Into<M> + Component,
    M: Clone + Into<T> + Any + Send + Sync,
{
    if let Some(server) = server {
        for (net_e, net_c, comp, ct) in q.iter_mut() {
            // If we are using change detection, and the component hasn't been changed, skip.
            if net_c.cd && !ct.is_changed() { continue; }

            if let Some(to_spec) = net_c.s_dir.to() {
                if let Err(e) = server.send_spec(&NetCompMsg::<M>::new(net_e.id, comp.clone().into()), *to_spec) {
                    error!("{}", e);
                }
            }
        }
    } else if let Some(client) = client {
        for (net_e, net_c, comp, ct) in q.iter_mut() {
            // If we are using change detection, and the component hasn't been changed, skip.
            if net_c.cd && !ct.is_changed() { continue; }

            if let CNetDir::To = net_c.c_dir {
                if let Err(e) = client.send(&NetCompMsg::<M>::new(net_e.id, comp.clone().into())) {
                    error!("{}", e);
                }
            }
        }
    }
}

fn comp_recv<T, M>(
    server: Option<ResMut<Server>>,
    client: Option<ResMut<Client>>,
    mut q: Query<(&NetEntity, &mut NetComp<T, M>, &mut T)>,
) where
    T: Clone + Into<M> + Component,
    M: Clone + Into<T> + Any + Send + Sync,
{
    if let Some(server) = server {
        // Cache messages
        let msgs: Vec<NetMsg<NetCompMsg<M>>> = server.recv::<NetCompMsg<M>>().unwrap().collect();
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
        let msgs: Vec<NetMsg<NetCompMsg<M>>> = client.recv::<NetCompMsg<M>>().unwrap().collect();
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

/// Gets the most recent message that matches `from_spec` for entity with `id`
/// if it is sent later that current.
fn get_latest_msg<'a, M: Any + Send + Sync>(msgs: &'a Vec<NetMsg<NetCompMsg<M>>>, current: Option<u32>, spec: CIdSpec, id: u64) -> Option<&'a NetMsg<'a, NetCompMsg<M>>> {
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
    println!("{}", latest_time);
    latest
}
