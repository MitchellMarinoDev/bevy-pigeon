# Quickstart guide

This will walk you through what you need to know to use `bevy-pigeon`.

This guide assumes some familiarity with `carrier-pigeon`. Reading `carrier-pigeon`'s quickstart should do it.

## Client or Server

`bevy-pigeon` systems operate on `carrier-pigeon`s Client and Server as resources. You will need to create a client,
server, or both and add them to the app's resources in order for `bevy-pigeon`s systems to work.  To do this, see
`carrier-pigeon`s documentation.

## Plugin

You must add the plugin to the app. Add the `ClientPlugin`, `ServerPlugin` or both. These plugins will automatically 
clear the message buffer and receive new messages at the start of every frame. 

If you want more control about when to clear messages and receive new messages, don't add the plugins. Instead, you can
add the `bevy_pigeon::app::server_tick` and `bevy_pigeon::app::client_tick` systems where ever you want. Or, you could
make your own systems entirely.

## NetEntity

`bevy-pigeon` needs some way of knowing what entity on one game instance corresponds with what entity on another
instance. pigeon's solution to this is the `NetEntity` component. The `NetEntity` component simply contains a `u64`
as an identifier. To link entities across instances, they both need to have the component `NetEntity` with the same id.

## NetComp

The second half of the pigeon's syncing system is the component `NetComp<T, M = T>`. This generic component specifies
what component should be synced and in what direction. For instance, if you want an entity's `Transform` to be synced,
you would add the `NetComp<Transfrom>` to it. To specify the direction to sync
(***To*** the peer, or ***From*** the peer), you use `NetDir` enums.

## NetDirection

There are two NetDirection enums; one for the client, one for the server. The `CNetDir` (Client Network Direction)
has only 2 possible values, `To` and `From`. Pretty simple; either you sync **To** the server, or **From** the server.
The Server version is a little more complex. The `SNetDir` (Server Network Direction) enum has 3 values, `To(CIdSpec)`,
`From(CIdSpec)`, and `ToFrom(CIdSpec, CIdSpec)`. A couple of differences here: we have a bonus option `ToFrom`, and
we have the options have `CIdSpec` fields. The `CIdSpec`s specify what connection IDs to do sync to/from. For instance,
`From(CIdSpec::Only(1))` will sync that component from only client 1. `To(CIdSpec::Except(2))` will sync to all clients
except for client 2. `ToFrom(CIdSpec::Except(2), CIdSpec::Only(2))` will sync to all clients except for client 2, and
sync from client 2.

## Syncing

So back to the `NetComp` component. To sync our entity's `Transform` from the server to the client, on the client do
`entity.insert(NetComp::<Transform>::new(CNetDir::From, SNetDir::To(CIdSpec::All)))`.
It also needs the `NetEntity` component, so do `entity.insert(NetEntity::new(9414351989064014771))`.

Lastly we need to tell `bevy-pigeon` to add the system that syncs the transforms for us. When building the app, add
`app.sync_comp::<Transform, Transfrom>(&mut table, Transport::UDP)` passing in a reference to your `MsgTable`.
This will add as system to send and receive these components, and register the `Transform` type to be sent 
through `carrier-pigeon`.

However, `Transform` doesn't implement serde's `Serialize + DeserializeOwned`, so carrier-pigeon can't send it. Luckily
`bevy-pigeon` provides a network-able `Transform` (along with other common components) in the `types` module. Put 
together this looks like:
```rust
use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    let mut table = MsgTable::new();

    // Tell bevy-pigeon to sync the Transform component using the NetTransform message type.
    app.sync_comp::<Transform, NetTransform>(&mut table, Transport::UDP);

    let parts = table.build::<Connection, Response, Disconnect>().unwrap();

    // For that sake of the example, make a server.
    let server = Server::new(
        "127.0.0.1:4455".parse().unwrap(),
        parts.clone(),
        Config::default(),
    ).unwrap();
    app.insert_resource(server);
    
    // Add the plugins and run
    app.add_plugins(DefaultPlugins)
        .add_plugin(ClientPlugin)
        .add_plugin(ServerPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn()
        .insert(Transform::default())
        .insert(GlobalTranform::default())
        .insert(NetEntity::new(3061789524793635849))
        .insert(NetComp::<Transform, NetTransform>::new(CNetDir::From, SNetDir::To(CIdSpec::All)));
}
```

All this is shown in the `mvp` example if you need to see a more full example.
