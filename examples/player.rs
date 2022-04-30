//! A multiplayer "game" where you control a character.
//!
//! A simple example where every client spawns a player, despawning it when they disconnect.
//! This is a little more of an in depth than the `mvp` example. It shows connection validation,
//! and doing something on a disconnect.

use std::net::SocketAddr;
use bevy::prelude::*;
use carrier_pigeon::Transport;
use bevy_pigeon::{AppExt, ClientPlugin, ServerPlugin};
use bevy_pigeon::types::NetTransform;
use crate::connecting::ConnectingPlugin;
use crate::game::GamePlugin;
use crate::menu::MenuPlugin;
use crate::shared::*;

mod shared;

const ADDR_LOCAL: &str = "127.0.0.1:7777";

struct Config {
    ip: SocketAddr,
    user: String,
    pass: Option<String>,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
enum GameState {
    Menu,
    Connecting,
    Game,
}

fn main() {
    let mut app = App::new();
    let mut table = get_table();

    // Get IP addr
    let ip: SocketAddr = std::env::args().nth(1).unwrap_or(ADDR_LOCAL.into()).parse().expect("please enter a valid ip address and port. Ex. `192.168.0.99:4455`");
    let user = std::env::args().nth(1).unwrap_or("Player".into());
    let pass = std::env::args().nth(1);
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
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.transform = Transform::from_xyz(0.0, 4.0, -4.0);
    commands.spawn_bundle(camera);

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
    use carrier_pigeon::{OptionPendingClient};
    use crate::{clean_up, GameState, Response};
    use crate::GameState::Connecting;

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
        client: Option<ResMut<OptionPendingClient>>,
        mut game_state: ResMut<State<GameState>>
    ) {
        if client.is_none() {
            // No client (server only). No Connecting needed.
            let _ = game_state.set(GameState::Game);
            return;
        }
        let mut client = client.unwrap();

        if client.done().unwrap() {
            let con_result = client.take::<Response>().unwrap();
            let (client, response) = con_result.expect("IO Error occurred while connecting.");
            match response {
                Response::Accepted => {
                    println!("Connection successful");
                    commands.insert_resource(client);
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
    use crate::{clean_up, SystemSet};
    use crate::GameState::Game;

    /// A marker component so that we can clean up easily.
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Component)]
    pub struct GameItem;

    pub struct GamePlugin;
    impl Plugin for GamePlugin {
        fn build(&self, app: &mut App) {
            app
                .add_system_set(
                    SystemSet::on_enter(Game)
                        .with_system(setup_game)
                )
                .add_system_set(
                    SystemSet::on_update(Game)
                )
                .add_system_set(
                    SystemSet::on_exit(Game)
                        .with_system(clean_up::<GameItem>)
                )
            ;
        }
    }

    fn setup_game(

    ) {

    }
}