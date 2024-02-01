# Installing and Initialization

To install `bevy_animations` in your Bevy project just simply add the following to your `Cargo.toml` file.

```toml
[dependencies]
bevy_animations = "0.5.6"
```

You'll also need to reference this versioning for bevy

| bevy  | bevy_animations  |
| ----- | ---------------  |
| 0.12.x | 0.5.x             |
| 0.11.x | 0.4.x             |
| 0.10.x | 0.3.x             |
| 0.9.x  | 0.2.x             |

Since this library is in the early stages of development, there may be some major changes that aren't reflected via semver changes. ie. 0.5.4 may be drastically different than 0.5.5. So always be care upgrading even minor semver changes, because they may actually be major.

## Initialization with Bevy

The following code shows all you need to do in order to add `bevy_animations` to the Bevy app.

```rust
use bevy_animations::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AnimationsPlugin {
            pixels_per_meter: 20. // your desired pixels_per_meter
        })
        .run()
}
```

The `pixels_per_meter` will be used to determine how far an entity has gone for some of the `Transform` based animations you'll learn about later

## [Coninue To Next Chapter ->](./chapter_2.md)
