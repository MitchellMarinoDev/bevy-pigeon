# bevy-pigeon

The bevy plugin for [`carrier-pigeon`](https://github.com/MitchellMarinoDev/carrier-pigeon).

Building on `carrier-pigeon`, this crate provides high level network abstractions to allow making a game a breeze.
`bevy-pigeon` takes care of most of the networking for you, so you barely have to see it or think about it.

### Add bevy-pigeon to your `Cargo.toml`:

`bevy-pigeon = "0.3.0"`

## Compatibility
| `bevy` | `bevy-pigeon` | `carrier-pigeon` |
|:------:|:-------------:|:----------------:|
|  0.7   |      0.3      |       0.3        |

## Is bevy-pigeon right for me?

Since carrier-pigeon uses TCP and UDP, it is usable and convenient for most all games. FPS games (and other games where
precise position is of the upmost importance) would benefit from a backroll solution like 
[bevy_backroll](https://crates.io/crates/bevy_backroll) or [bevy_ggrs](https://github.com/gschup/bevy_ggrs).
Though, it is certainly possible to make an FPS with `bevy-pigeon`

TCP is required for the connection cycle in carrier-pigeon, so if you want to target wasm, pigeon will not work for you.

## Component Syncing

> "The best networking solution is one you can't see"

You can use `bevy-pigeon`'s component syncing by adding just 3 lines.

This component syncing allows you to simply say "sync these components" and `bevy-pigeon` will take care of it.

### Do I have to use the Component Syncing feature?

No, but right now, it is really the only thing that `bevy-pigeon` offers. If you don't want to use the component syncing,
you can just use `carrier-pigeon` directly. Keep in mind, if you use `bevy-pigeon`'s component syncing you can
still use `carrier-pigeon`'s message system; the component syncing is purely an additive feature.

## Documentation

The documentation can be found on [Docs.rs](https://docs.rs/bevy-pigeon)

A quickstart guide that goes in more detail is found at [/Quickstart.md](Quickstart.md)

An in depth guide to the more advanced features is found at [/Advanced.md](Advanced.md)

### Examples

- A full 2-player pong/breakout game made with `bevy-pigeon` is available on 
[GitHub](https://github.com/MitchellMarinoDev/bong).
- Check out the [`examples/` directory](examples).

## Features

- [x] Easy Component Syncing.

### Planned Features

- [ ] switching TCP to webrtc with a feature flag.

## Contributing

To contribute, fork the repo and make a PR. If you find a bug, feel free to open an issue. If you have any questions,
concerns or suggestions you can shoot me an email (found in Cargo.toml) or DM me on discord `@TheHourGlass34#0459`.

By contributing, you agree that your changes are subject to the license found in /LICENSE.
