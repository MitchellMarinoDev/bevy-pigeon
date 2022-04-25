# Quickstart guide

This will walk you through what you need to know to use `bevy-pigeon`.

In this guide you will learn how to add Networking to your game with `bevy-pigeon`.

This guide assumes some familiarity with `carrier-pigeon`. Reading `carrier-pigeon`'s quickstart
should do it.

## NetEntity

`bevy-pigeon` needs some way of knowing what entity on one game instance corresponds with what entity on another instance.
pigeon's solution to this is the `NetEntity` component. The `NetEntity` component simply contains a `u64` as an identifier.
To link entities across instances, they both need to have the component `NetEntity` with the same id.

## NetComp

The second half of the pigeon's syncing system is the component `NetComp<T, M = T>`. This generic component specifies
what component should be synced and in what direction. For instance, if you want an entity's `Transform` to be synced, you
would add the `NetComp<Transfrom>` to it. To specify the direction to sync (***To*** the peer, or ***From*** the peer),
you use `NetDirection`.

## NetDirection

`NetDirection` is an enum for describing the direction of syncing. The options are `To`, `From` or `ToFrom`(both).
Note that the `ToFrom` option is only available on the server. These enums have values letting you specify what CIds to
sync to and from (These are only used on the server, and are completely ignored on the client).

### Example

Let's say you want each player to be able to control their own character. For player with connection id 1, on the server
the direction would be `ToFrom(Except(1), Only(1))` specifying that it should sync to all clients *except* the player, and sync
from *only* the player. The direction on the client should be `To(All)` (again the `All` could be anything as it gets ignored
on the client) to specify that it should sync *to* the server.

## Syncing

So back to the `NetComp` component. To sync our entity's transform from the server to the client, on the client do
`entity.insert(NetComp::<Transform>::new(NetDirection::from()))`, or on the server do
`entity.insert(NetComp::<Transform>::new(NetDirection::to()))`. It also needs the `NetEntity` component, so do
`entity.insert(NetEntity::new(9414351989064014771))`. Lastly we need to tell `bevy-pigeon` that we want it to sync
transforms. When building the app, add `app.sync_comp::<Transform, Transfrom>(&mut table, Transport::UDP)` passing
in a reference to your `MsgTable`. This will register the Transform type to be sent through `carrier-pigeon`.
However, Transform doesn't implement serde's `Serialize + DeserializeOwned`. This means carrier-pigeon can't send
this. The solution to this is make a custom type that is used as the message.

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
`MyTransform` for sending. `entity.insert(NetComp::<Transform, MyTransform>::new(NetDirection::to()))`. We must also
change the `sync_comp` call as follows: `app.sync_comp::<Transform, MyTransform>(&mut table, Transport::UDP)`.

## Cheat-Sheet

To send a component that can be networked (implements `Any + Send + Sync + Serialize + DesrializeOwned`):

