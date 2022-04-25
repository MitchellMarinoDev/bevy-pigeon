# bevy-pigeon

The bevy plugin for `carrier-pigeon`.

Building on `carrier-pigeon`, this crate provides high level network abstractions to allow making a game a breeze.
`bevy-pigeon` tries to take care of all the networking for you, so you barely have to see it or think about it.

## Compatibility
| `bevy` | `bevy-pigeon` | `carrier-pigeon` |
|:------:|:-------------:|:----------------:|
|  0.7   |      0.3      |       0.3        |

# Component Syncing

"The best networking solution is one you can't see". You can use `bevy-pigeon`'s component syncing by adding just 3 lines.
You will see this in action later on.

## NetEntity

`bevy-pigeon` needs some way of knowing what entity on one game instance corresponds with what entity on another instance.
pigeon's solution to this is the `NetEntity` component. The `NetEntity` component simply contains a u64 as an identifier.
To link entities across instances, they both need to have the component `NetEntity` with the same id.

## NetComp

The second half of the pigeon's syncing system is the component `NetComp<T, M = T>`. This is a generic component that specifies
what component should be synced and in what direction. For instance, if you want an entity's `Transform` to be synced, you 
would add the `NetComp<Transfrom>` to it. To specify the direction to sync (***To*** the peer, or ***From*** the peer),
you use `NetDirection`.

## NetDirection

`NetDirection` is an enum for describing the direction of syncing. The options are `To`, `From` or `ToFrom`(both).
Note that the ToFrom option is only available on the server. The enums have values letting you specify what CIds to 
sync to and from (These are only used on the server, and are completely ignored on the client).

### Example

Let's say you want each player to be able to control their own player. For player with connection id `1`, on the server 
the direction would be `ToFrom(Except(1), Only(1))` specifying that it should sync to all clients *except* the player, and sync
from *only* the player. The direction on the client should be `To(All)` (again the `All` could be anything as it gets ignored
on the client) to specify that it should sync *to* the server.

## Syncing

So back to the `NetComp` component. To sync our entity's transform, when creating the entity, do 
`entity.insert(NetComp::<Transform>::new(NetDirection::to()))`. It also needs the NetEntity component, so
`entity.insert(NetEntity::new(9414351989064014771))`. Lastly we need to tell `bevy-pigeon` that we want to sync transforms.
When building the app, add `app.sync_comp::<Transform, Transfrom>(&mut table, Transport::UDP)` passing
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

### Do I have to use the Component Syncing feature?

No, but right now, it is really the only thing that `bevy-pigeon` offers. If you don't want to use the component syncing,
you can just use `carrier-pigeon` directly. Keep in mind, if you use `bevy-pigeon`'s component syncing you can
still use `carrier-pigeon`'s message system; the component syncing is purely an additive feature.

## Documentation

The documentation can be found on [Docs.rs](https://docs.rs/bevy-pigeon)

### Examples

- A full 2-player pong/breakout game made with `bevy-pigeon` is available on [GitHub](https://github.com/MitchellMarinoDev/bong)
- Check out the 
[`examples/` directory](https://github.com/MitchellMarinoDev/bevy-pigeon/tree/main/examples).

## Features

- [x] Easy Component Syncing.

## Contributing

To contribute, fork the repo and make a PR. If you find a bug, feel free to open an issue. If you have any questions,
concerns or suggestions you can shoot me an email (found in Cargo.toml) or DM me on discord `@TheHourGlass34#0459`.

By contributing, you agree that your changes are subject to the license found in /LICENSE.
