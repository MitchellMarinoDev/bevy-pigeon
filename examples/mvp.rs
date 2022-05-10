//! The "Minimum Viable Product".
//!
//! The most minimal bevy app that shows almost all of `bevy_pigeon`'s features without much
//! other code to distract from the net-code.

mod shared;

use crate::shared::*;
use bevy::prelude::*;
use bevy_pigeon::sync::{NetComp, NetEntity};
use bevy_pigeon::types::NetTransform;
use bevy_pigeon::{AppExt, ClientPlugin, ServerPlugin};
use carrier_pigeon::net::Config;
use carrier_pigeon::{Client, Server, Transport};
use std::f32::consts::PI;

const ADDR_LOCAL: &str = "127.0.0.1:7777";

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash)]
struct Spin;

fn main() {
    let mut app = App::new();
    let mut table = get_table();

    // Tell bevy_pigeon to sync the Transform component using the NetTransform message type.
    app.sync_comp::<Transform, NetTransform>(&mut table, Transport::UDP);

    let parts = table.build::<Connection, Response, Disconnect>().unwrap();

    // Multiplayer
    let multiplayer_arg = std::env::args()
        .nth(1)
        .unwrap_or("server".into())
        .to_lowercase();
    let is_server = multiplayer_arg == "server";
    let is_client = multiplayer_arg == "client";

    println!("Server: {}, Client: {}", is_server, is_client);

    if is_server {
        let server = Server::new(
            ADDR_LOCAL.parse().unwrap(),
            parts.clone(),
            Config::default(),
        )
        .unwrap();
        app.insert_resource(server);
    }
    if is_client {
        let pending_client = Client::new(
            ADDR_LOCAL.parse().unwrap(),
            parts,
            Config::default(),
            Connection::default(),
        );
        // For simplicity, just block until the connection is made. Realistically you would add the PendingConnection to
        //      The resources and poll it.
        let (client, response): (Client, Response) = pending_client.block().unwrap();
        info!("{:?}", response);
        app.insert_resource(client);
    }

    app.add_plugins(DefaultPlugins)
        .add_plugin(ClientPlugin)
        .add_plugin(ServerPlugin)
        .add_startup_system(setup)
        .add_system(handle_discon_con)
        .add_system(spin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Light
    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.2,
    });

    // Spawn cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::PINK,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Spin)
        .insert(NetEntity::new(0))
        .insert(NetComp::<Transform, NetTransform>::default());
}

fn spin(server: Option<Res<Server>>, mut q_spin: Query<&mut Transform, With<Spin>>) {
    // only spin on server
    if server.is_none() {
        return;
    }

    for mut spin in q_spin.iter_mut() {
        let mut rot = spin.rotation;
        let (x, y, mut z) = rot.to_euler(EulerRot::XYZ);
        z += PI / 72.0;
        rot = Quat::from_euler(EulerRot::XYZ, x, y, z);
        spin.rotation = rot;
    }
}

/// Handles new connections and disconnections.
fn handle_discon_con(server: Option<ResMut<Server>>) {
    if let Some(mut server) = server {
        server.handle_new_cons(&mut |cid, _c: Connection| (true, Response::Accepted(cid)));
        server.handle_disconnects(&mut |cid, status| {
            info!("Client {cid} disconnected with status: {status}");
        });
    }
}
