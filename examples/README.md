# Examples

- [MVP](examples/mvp.rs): The most minimal bevy app that shows almost all of `bevy_pigeon`'s features without much
other code to distract from the net-code.
    - Defaults to a server. To change this, run `cargo run --example mvp -- client`.
- [player](examples/player.rs): A simple example where every client connection spawns a player, 
despawning it when they disconnect. This is more in depth than the `mvp` example. It shows connection validation,
and doing something on a disconnect.
  - Due to bevy's UI not having a text field, the ip addr, username, and password must be entered through command line args. 
To change this, run `cargo run --example player -- 192.168.0.99:4455 Username password`.
