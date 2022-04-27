# Quickstart guide

This will walk you through what you need to know to use `bevy-pigeon`.

This guide assumes some familiarity with `carrier-pigeon`. Reading `carrier-pigeon`'s quickstart
should do it.

## Plugin

First, you must add the plugin. Add either `ClientPlugin`, `ServerPlugin` or both. This will automatically clear messages
and receive new messages at the start of every frame.

## NetEntity

`bevy-pigeon` needs some way of knowing what entity on one game instance corresponds with what entity on another instance.
pigeon's solution to this is the `NetEntity` component. The `NetEntity` component simply contains a `u64` as an identifier.
To link entities across instances, they both need to have the component `NetEntity` with the same id.

## NetComp

The second half of the pigeon's syncing system is the component `NetComp<T, M = T>`. This generic component specifies
what component should be synced and in what direction. For instance, if you want an entity's `Transform` to be synced, you
would add the `NetComp<Transfrom>` to it. To specify the direction to sync (***To*** the peer, or ***From*** the peer),
you use `NetDir` enums.

## NetDirection

There are two NetDirection enums; one for the client, one for the server. The `CNetDir` (Client Network Direction)
has only 2 possible values, `To` and `From`. Pretty simple; either you sync **To** the peer, or **From** the peer.
The Server version is a little more complex. The `SNetDir` (Server Network Direction) enum has 3 values, `To(CIdSpec)`,
`From(CIdSpec)`, and `ToFrom(CIdSpec, CIdSpec)`. A couple of differences here: we have a bonus option `ToFrom`, and 
we have the options have `CIdSpec`s. The `CIdSpec`s specify what connection IDs to do sync to/from. For instance,
`From(CIdSpec::Only(1))` will sync that component from only client 1. `To(CIdSpec::Except(2))` will sync to all clients
except for client 2. `ToFrom(CIdSpec::Except(2), CIdSpec::Only(2))` will sync to all clients except for client 2, and sync
from client 2.

## Syncing

So back to the `NetComp` component. To sync our entity's transform from the server to the client, on the client do
`entity.insert(NetComp::<Transform>::new(CNetDir::From, SNetDir::To(CIdSpec::All)))`.
It also needs the `NetEntity` component, so do `entity.insert(NetEntity::new(9414351989064014771))`. 
Lastly we need to tell `bevy-pigeon` that we want it to sync transforms. When building the app, add 
`app.sync_comp::<Transform, Transfrom>(&mut table, Transport::UDP)` passing in a reference to your 
`MsgTable`. This will add as system to send and receive these components, and register the Transform 
type to be sent through `carrier-pigeon`. However, Transform doesn't implement serde's 
`Serialize + DeserializeOwned`, so carrier-pigeon can't send it. The solution to this is make a custom
type that is used as the message.

## Custom message types

Sometimes, you want to use `bevy-pigeon`'s `NetComp` to easily sync your components, but the component you want
to sync can't be sent by `carrier-pigeon`, or you want custom control over how to serialize it (to help save bandwidth).
The solution to this is to make a custom type that can be sent by `carrier-pigeon`, and tell `bevy-pigeon` to use that.
For example, to send Transforms:
```rust
#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub struct MyTransform {
    pub translation: Vec3,
    pub rotation: Quat,
    // If you dont use scale, it can be taken out to reduce bandwidth.
}

// You must impl these conversion types so `pigeon` can convert it.
impl From<Transform> for MyTransform {
    fn from(o: Transform) -> Self {
        MyTransform {
            translation: o.translation,
            rotation: o.rotation,
        }
    }
}

impl From<MyTransform> for Transform {
    fn from(o: MyTransform) -> Self {
        Transform {
            translation: o.translation,
            rotation: o.rotation,
            ..default()
        }
    }
}
```
Now, to finish up with our syncing transforms example we will change the `NetComp` as follows, to tell pigeon to use
`MyTransform` for sending. `entity.insert(NetComp::<Transform>::new(CNetDir::From, SNetDir::To(CIdSpec::All)))`. We must also
change the `sync_comp` call as follows: `app.sync_comp::<Transform, MyTransform>(&mut table, Transport::UDP)`. All this
is shown in the mvp example if you need to see it all put together.
