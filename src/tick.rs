use bevy::prelude::*;
use carrier_pigeon::{Client, Server};

pub fn client_tick(client: Option<ResMut<Client>>) {
    if let Some(mut client) = client {
        client.clear_msgs();
        client.recv_msgs();
    }
}

pub fn server_tick(server: Option<ResMut<Server>>) {
    if let Some(mut server) = server {
        server.clear_msgs();
        server.recv_msgs();
    }
}
