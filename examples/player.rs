//! A multiplayer "game" where you control a character.
//!
//! A simple example where every client connection spawns a player, despawning it when they disconnect.
//! This is more in depth than the `mvp` example. It shows connection validation,
//! and doing something on a disconnect.

use std::net::SocketAddr;
use bevy::prelude::*;
use carrier_pigeon::{CId, Transport};
use bevy_pigeon::{AppExt, ClientPlugin, ServerPlugin};
use bevy_pigeon::types::NetTransform;
use crate::connecting::ConnectingPlugin;
use crate::game::GamePlugin;
use crate::menu::MenuPlugin;
use crate::shared::*;
use serde::{Serialize, Deserialize};

mod shared;

const ADDR_LOCAL: &str = "127.0.0.1:7777";

#[derive(Clone, Eq, PartialEq, Debug)]
struct Config {
    ip: SocketAddr,
    user: String,
    pass: String,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
enum GameState {
    Menu,
    Connecting,
    Game,
}

#[derive(Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
struct NewPlayer(CId);

#[derive(Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
struct DelPlayer(CId);

fn main() {
    let mut app = App::new();
    let mut table = get_table();

    table.register::<NewPlayer>(Transport::TCP).unwrap();
    table.register::<DelPlayer>(Transport::TCP).unwrap();

    // Get IP addr
    let ip: SocketAddr = std::env::args().nth(1).unwrap_or(ADDR_LOCAL.into()).parse().expect("please enter a valid ip address and port. Ex. `192.168.0.99:4455`");
    let user = std::env::args().nth(2).unwrap_or("Player".into());
    let pass = std::env::args().nth(3).unwrap_or(String::new());
    let conf = Config { ip, user, pass };
    app.insert_resource(conf);

    // Tell bevy-pigeon to sync the Transform component using the NetTransform message type.
    app.sync_comp::<Transform, NetTransform>(&mut table, Transport::UDP);

    let parts = table.build::<Connection, Response, Disconnect>().unwrap();
    app.insert_resource(parts);

    app
        .add_state(GameState::Menu)
        .add_plugins(DefaultPlugins)
        .add_plugin(ClientPlugin)
        .add_plugin(ServerPlugin)

        .add_startup_system(setup)
        .add_plugin(MenuPlugin)
        .add_plugin(ConnectingPlugin)
        .add_plugin(GamePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::default(), Vec3::Y),
        ..PerspectiveCameraBundle::new_3d()
    });

    // UI Camera
    commands.spawn_bundle(UiCameraBundle::default());
}

/// A generic clean up system.
fn clean_up<T: Component>(mut commands: Commands, q_menu: Query<Entity, With<T>>) {
    for e in q_menu.iter() {
        commands.entity(e).despawn_recursive();
    }
}

mod menu {
    use bevy::prelude::*;
    use carrier_pigeon::{Client, MsgTableParts, Server};
    use crate::{clean_up, Config, Connection, GameState, SystemSet};
    use crate::connecting::MyCId;
    use crate::GameState::Menu;

    /// A marker component so that we can clean up easily.
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Component)]
    pub struct MenuItem;

    #[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Hash)]
    enum MenuButton {
        Server,
        Host,
        Client,
    }

    pub struct MenuPlugin;
    impl Plugin for MenuPlugin {
        fn build(&self, app: &mut App) {
            app
                .add_system_set(
                    SystemSet::on_enter(Menu)
                        .with_system(setup_menu)
                )
                .add_system_set(
                    SystemSet::on_update(Menu)
                        .with_system(handle_buttons)
                )
                .add_system_set(
                    SystemSet::on_exit(Menu)
                        .with_system(clean_up::<MenuItem>)
                )
            ;
        }
    }

    fn handle_buttons(
        conf: Res<Config>,
        parts: Res<MsgTableParts>,
        q_button: Query<(&Interaction, &MenuButton), Changed<Interaction>>,
        mut game_state: ResMut<State<GameState>>,
        mut commands: Commands,
    ) {
        for (interaction, menu_button) in q_button.iter() {
            if *interaction == Interaction::Clicked {
                match menu_button {
                    MenuButton::Server => {
                        let server = Server::new(conf.ip, (*parts).clone()).expect("Failed to start a server.");
                        commands.insert_resource(server);
                    }
                    MenuButton::Host => {
                        let server = Server::new(conf.ip, (*parts).clone()).expect("Failed to start a server.");
                        commands.insert_resource(server);
                        let client = Client::new(conf.ip, (*parts).clone(), Connection::new(conf.user.clone(), conf.pass.clone()));
                        commands.insert_resource(client.option());
                        commands.insert_resource(MyCId(1));
                    }
                    MenuButton::Client => {
                        let client = Client::new(conf.ip, (*parts).clone(), Connection::new(conf.user.clone(), conf.pass.clone()));
                        commands.insert_resource(client.option());
                    }
                }
                game_state.set(GameState::Connecting).unwrap()
            }
        }
    }

    fn setup_menu(
        mut commands: Commands,
        assets: Res<AssetServer>,
    ) {
        println!("Setting up");
        let font = assets.load("FiraMono-Medium.ttf");
        let text_style = TextStyle {
            font,
            color: Color::BLACK,
            font_size: 60.0,
        };
        let button_style = Style {
            size: Size::new(Val::Px(1000.0), Val::Px(100.0)),
            margin: Rect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    margin: Rect::all(Val::Auto),
                    padding: Rect::all(Val::Px(10.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::Center,
                    size: Size {
                        width: Val::Percent(100.0),
                        height: Val::Auto,
                    },
                    ..default()
                },
                color: Color::CRIMSON.into(),
                ..default()
            })
            .insert(MenuItem)
            .with_children(|parent| {
                // Title
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect {
                            bottom: Val::Px(0.0),
                            ..Rect::all(Val::Px(20.0))
                        },
                        ..default()
                    },
                    text: Text::with_section(
                        "Player Example",
                        TextStyle {
                            color: Color::WHITE,
                            font_size: 100.0,
                            ..text_style.clone()
                        },
                        TextAlignment::default()
                    ),
                    ..default()
                });

                parent
                    .spawn_bundle(ButtonBundle {
                        color: UiColor(Color::rgb_u8(255, 255, 255)),
                        style: button_style.clone(),
                        // transform: Transform::from_xyz(100.0, 0.0, 0.0),
                        ..Default::default()
                    })
                    .insert(MenuButton::Server)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Start Server",
                                text_style.clone(),
                                TextAlignment::default(),
                            ),
                            ..Default::default()
                        });
                    });

                parent
                    .spawn_bundle(ButtonBundle {
                        color: UiColor(Color::rgb_u8(255, 255, 255)),
                        style: button_style.clone(),
                        // transform: Transform::from_xyz(100.0, 0.0, 0.0),
                        ..Default::default()
                    })
                    .insert(MenuButton::Host)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Start Host",
                                text_style.clone(),
                                TextAlignment::default(),
                            ),
                            ..Default::default()
                        });
                    });

                parent
                    .spawn_bundle(ButtonBundle {
                        color: UiColor(Color::rgb_u8(255, 255, 255)),
                        style: button_style,
                        // transform: Transform::from_xyz(100.0, 0.0, 0.0),
                        ..Default::default()
                    })
                    .insert(MenuButton::Client)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Start Client",
                                text_style,
                                TextAlignment::default(),
                            ),
                            ..Default::default()
                        });
                    });
            });
    }
}

mod connecting {
    use bevy::prelude::*;
    use carrier_pigeon::{CId, OptionPendingClient, Server};
    use crate::{clean_up, GameState, Response};
    use crate::GameState::Connecting;

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub struct MyCId(pub CId);

    /// A marker component so that we can clean up easily.
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Component)]
    pub struct ConnectingItem;

    pub struct ConnectingPlugin;
    impl Plugin for ConnectingPlugin {
        fn build(&self, app: &mut App) {
            app
                .add_system_set(
                    SystemSet::on_enter(Connecting)
                        .with_system(setup)
                )
                .add_system_set(
                    SystemSet::on_update(Connecting)
                        .with_system(check_connecting)
                )
                .add_system_set(
                    SystemSet::on_exit(Connecting)
                        .with_system(clean_up::<ConnectingItem>)
                )
            ;
        }
    }

    fn check_connecting(
        mut commands: Commands,
        server: Option<Res<Server>>,
        client: Option<ResMut<OptionPendingClient>>,
        mut game_state: ResMut<State<GameState>>
    ) {
        if server.is_some() {
            // If we have a server, no need to connect.
            let _ = game_state.set(GameState::Game);
            return;
        }
        let mut client = client.unwrap();

        if client.done().unwrap() {
            let con_result = client.take::<Response>().unwrap();
            let (client, response) = con_result.expect("IO Error occurred while connecting.");
            match response {
                Response::Accepted(cid) => {
                    println!("Connection successful. Our CId {cid}");
                    commands.insert_resource(client);
                    commands.insert_resource(MyCId(cid));
                    let _ = game_state.set(GameState::Game);
                    commands.remove_resource::<OptionPendingClient>();
                }
                Response::Rejected(reason) => {
                    println!("Connection rejected for reason: {:?}", reason);
                    let _ = game_state.set(GameState::Menu);
                    commands.remove_resource::<OptionPendingClient>();
                }
            }
        }
    }

    fn setup(
        mut commands: Commands,
        assets: Res<AssetServer>,
    ) {
        let font = assets.load("FiraMono-Medium.ttf");
        let text_style = TextStyle {
            font,
            color: Color::WHITE,
            font_size: 60.0,
        };

        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    margin: Rect::all(Val::Auto),
                    padding: Rect::all(Val::Px(10.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::Center,
                    size: Size {
                        width: Val::Percent(100.0),
                        height: Val::Auto,
                    },
                    ..default()
                },
                color: Color::CRIMSON.into(),
                ..default()
            })
            .insert(ConnectingItem)
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect {
                            bottom: Val::Px(0.0),
                            ..Rect::all(Val::Px(20.0))
                        },
                        ..default()
                    },
                    text: Text::with_section(
                        "Connecting...",
                        text_style,
                        TextAlignment::default()
                    ),
                    ..default()
                });
            });
    }
}

mod game {
    use bevy::prelude::*;
    use bevy::utils::HashMap;
    use carrier_pigeon::{CId, Client, Server};
    use carrier_pigeon::net::CIdSpec;
    use carrier_pigeon::net::CIdSpec::{Except, Only};
    use bevy_pigeon::sync::{CNetDir, NetComp, NetEntity, SNetDir};
    use bevy_pigeon::types::NetTransform;
    use crate::{clean_up, Config, Connection, DelPlayer, NewPlayer, RejectReason, Response, SystemSet};
    use crate::connecting::MyCId;
    use crate::GameState::Game;

    /// A marker component for a player.
    #[derive(Clone, Debug, Default, Component)]
    struct Player;

    /// A marker component for a player.
    #[derive(Clone, Debug, Default, Component)]
    struct MyPlayer;

    /// Maps a connection ID to a username.
    #[derive(Clone, Debug, Default)]
    struct Players(pub HashMap<CId, String>);

    /// A marker component so that we can clean up easily.
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Component)]
    pub struct GameItem;

    pub struct GamePlugin;
    impl Plugin for GamePlugin {
        fn build(&self, app: &mut App) {
            app
                .insert_resource(Players::default())
                .add_system_set(
                    SystemSet::on_enter(Game)
                        .with_system(setup_game)
                )
                .add_system_set(
                    SystemSet::on_update(Game)
                        .with_system(handle_cons)
                        .with_system(add_del_players)
                        .with_system(move_player)
                )
                .add_system_set(
                    SystemSet::on_exit(Game)
                        .with_system(clean_up::<GameItem>)
                )
            ;
        }
    }

    fn setup_game(
        my_cid: Option<Res<MyCId>>,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        // Ground plane.
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        });

        if let Some(cid) = my_cid {
            println!("setup");
            spawn_player(cid.0, true, &mut commands, &mut *meshes, &mut *materials);
        }
    }

    fn add_del_players(
        mut commands: Commands,
        client: Option<Res<Client>>,
        q_player: Query<(Entity, &NetEntity), With<Player>>,
        // For spawning player
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        if let Some(client) = client {
            for msg in client.recv::<DelPlayer>().unwrap() {
                if let Some((entity, _net_e)) = q_player.iter().filter(|(_e, net_e)| net_e.id == msg.0 as u64).next() {
                    commands.entity(entity).despawn_recursive();
                }
            }

            for msg in client.recv::<NewPlayer>().unwrap() {
                spawn_player(msg.0, false, &mut commands, &mut *meshes, &mut *materials);
            }
        }
    }

    fn move_player(
        mut q_player: Query<&mut Transform, With<MyPlayer>>,
        time: Res<Time>,
        input: Res<Input<KeyCode>>,
    ) {
        if let Ok(mut transform) = q_player.get_single_mut() {
            if input.pressed(KeyCode::W) || input.pressed(KeyCode::Up) {
                transform.translation -= Vec3::new(0.0, 0.0, 1.0) * time.delta_seconds();
            }
            if input.pressed(KeyCode::S) || input.pressed(KeyCode::Down) {
                transform.translation += Vec3::new(0.0, 0.0, 1.0) * time.delta_seconds();
            }
            if input.pressed(KeyCode::A) || input.pressed(KeyCode::Left) {
                transform.translation -= Vec3::new(1.0, 0.0, 0.0) * time.delta_seconds();
            }
            if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
                transform.translation += Vec3::new(1.0, 0.0, 0.0) * time.delta_seconds();
            }
        }
    }

    fn handle_cons(
        my_cid: Option<Res<MyCId>>,
        conf: Res<Config>,
        mut players: ResMut<Players>,
        server: Option<ResMut<Server>>,
        // For spawning player
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        if let Some(mut server) = server {
            let mut discon = vec![];
            server.handle_disconnects(&mut |cid, status| {
                info!("Connection {cid} disconnected with status: \"{status}\"");
                discon.push(cid);
            });
            for cid in discon {
                server.broadcast(&DelPlayer(cid)).unwrap();
                players.0.remove(&cid);
            }

            let mut new_players = vec![];
            server.handle_new_cons(&mut |cid, con: Connection| {
                if con.pass != conf.pass {
                    (false, Response::Rejected(RejectReason::IncorrectPassword))
                } else if players.0.len() > 2 {
                    (false, Response::Rejected(RejectReason::MaxPlayersReached))
                } else {
                    new_players.push((cid, con.user));
                    (true, Response::Accepted(cid))
                }
            });

            for (cid, user) in new_players {
                // Tell the new client about all the existing clients.
                for p_cid in players.0.keys() {
                    server.send_to(&NewPlayer(*p_cid), cid).unwrap();
                }
                // Tell the other players about the new player.
                server.send_spec(&NewPlayer(cid), CIdSpec::Except(cid));

                players.0.insert(cid, user);

                // Stops the host from spawning two players for itself.
                if my_cid.is_some() && my_cid.as_ref().unwrap().0 != cid {
                    spawn_player(cid, false, &mut commands, &mut *meshes, &mut *materials);
                }
            }
        }
    }

    fn spawn_player(
        cid: CId,
        my_player: bool,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) {
        info!("Spawning player. CId: {cid}, mine? {my_player}.");

        let net_comp = if my_player {
            NetComp::<Transform, NetTransform>::new(CNetDir::To, SNetDir::ToFrom(Except(cid), Only(cid)))
        } else {
            NetComp::<Transform, NetTransform>::new(CNetDir::From, SNetDir::ToFrom(Except(cid), Only(cid)))
        };

        let id =
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
            .insert(NetEntity::new(cid as u64))
            .insert(net_comp)
            .insert(GameItem)
            .insert(Player)
            .id();

        if my_player {
            commands.entity(id).insert(MyPlayer);
        }

    }
}