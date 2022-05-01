# Advanced guide

This will walk you through the more advanced features of `bevy-pigeon`.

This guide assumes you've read and understand the Quickstart Guide.

## Change Detection.

Change detection is an optimization were the sync messages are only sent if the component changes. It uses bevy's
change detection. It is also enabled by default. The field `cd` on `NetComp` is what turns the change detection on 
or off. It is public and can be changed at any time, or you can use the `.no_cd()` method on construction.

It can cause issues if the last packet before a component stops changing is lost, or if a new player joins 
while a component is not changing.

## Message table registration.

When calling `app.sync_comp::<T, M>(&mut table, UDP)` or any of its variants, it will not register type `M` into 
the table. It will actually register `NetCompMsg<M>`. This doesn't matter for the most part. It does mean that you
can register type `M` into the table in addition to calling `sync_comp`. This also means that sending a message of
type `M` will not be applied to the component on the other end.

## Dynamically Creating Networked Entities.

Dynamically creating networked entities is possible, but it is not an included feature (for good reason).
To do this, you should create a message type for spawning an entity of that type. For an example we will use
an example of spawning a bullet. You would create a message type like so:
```rust
struct SpawnBullet {
    /// The id of the NetEntity.
    id: u64,
    /// The amount of damage the bullet does when it hits the target.
    damage: u32,
    // Add any other info that you need to know to construct the bullet.
}
```
Don't forget to register it in the MsgTable.
Then, when you want to spawn a bullet, just send it using the client or server. You do, of course do have to make 
a system that handles spawning the bullets on the other end.
```rust
fn spawn_bullets(
    mut commands: Commands,
    client: Res<carrier_pigeon::Client>,
) {
    for msg in client.recv::<SpawnBullet>().unwrap() {
        commands.spawn()
            .insert(NetEntity::new(msg.id)
            .insert(Damage(msg.damage))
            //.insert texture/collider/mesh
            //.insert other things bullet has
        ;
    }
}
```
The reason `bevy-pigeon` can't have this feature built in, is that you have to specify how to make a bullet. 
Especially the non-networked components. There is too much that is application specific to make it a generic feature.

### Picking an id.

How do you pick the id for the NetEntity? It depends. `bevy-pigeon` uses a `u64` for the id so that a random number
would have a ***very*** low chance of collision. Using a random `u64` is definitely a valid strategy. A random `u64` 
has a `1/(1.84*10^19)` or `5.42*10^-20` chance of colliding with any other existing id. This is incredibly small, 
but if it is critical for absolutely no collisions to happen, you could keep a list of existing ids and generate 
one that is guaranteed not to collide. An incrementing integer would also work if only one connected instance 
(likely the server) generates new ids.

### What happens if there is a collision?

Though this is unspecified behavior, I imagine a number of things could happen depending on the circumstance;
your game will likely not crash.

- If the 2 entities with the id collisions both have the same synced component (i.e. they are both syncing transform),
those components will likely get written to the same thing (If there are two bullets syncing transform, they will
be synced to the same transform).
- If the 2 entities with the id collision have different synced components (i.e. a bottle syncing water level and
a bullet syncing transform), then this will probably behave normally.

## Labels.

Networking systems are labeled with the `NetLabel` label.
