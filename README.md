# bevy_animations is a Lightweight 2d animations engine built for Bevy

## What bevy_animations accomplished
1. using bevy_animations is easy and the animation configurations are simple

2. bevy_animation is fast enough to handle all of the entities you want animated

### Add bevy_animations to your Bevy App
```rust
use bevy_animations::AnimationsPlugin;
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

### How bevy_animations animations work
* they work based off of left to right based animations from sprite sheets
* specified timings or `meters_per_frame` for each frame
* user defining which y indexes are left, right, up and down facing sprites
* timed animations can block others from happening and utilized a priortity based system so you can define multiple ***blocking*** animations with different priority to render

### How to define a bevy_animations animation
You first need to spawn an entity using `Commands` like this

```rust
use bevy_animations::*;
use bevy::prelude::*;

let entity = commands.spawn(
    AnimationDirection::Still // the `AnimationDirection` component is needed on the entity to determine the direction
    SpriteSheetBundle {
        texture_atlas: // your sprite sheet handle
        transform: Transform::from_xyz(0., 0., 0.) // your desired location in the `World`
    }
    /* The rest of your entity configuration */
);
```

You can then add your animations to `Res<Animations>` like this

```rust
animations.insert_animation(
    entity.id(), // the entity is needed to determine which `Handle<TextureAtlas>` is being manipulated
    AnimationType::Transform(
        TransformAnimation::new(
            /* animation_frames */ vec![0, 1, 2, 3] // the x index for your frames to cycle through
            /* meters per frame */ 0.55 // your desired meters per frame
            /* handle */ texture_atlas_hanle // your sprite sheet
            /* frame */ Vec2::new(4., 4.) // the length and height of your sprite sheet
            /* direction_indexes */ AnimationDirectionIndexes::new(4, 3, 2, 1) // from the example above
            /* repeating */ true // if the animation is repeating or not
        )
    ),
    "player_running" // the name of the animation. will be used when sending an `AnimationEvent`
)
```

You can also add a `TimedAnimation` like this
```rust
animations.insert_animation(entity.id(), AnimationType::Timed(
    TimedAnimation::new(
        /* animation_frames */ vec![0, 1, 2, 3] // the x index for your frames to cycle through, 
        /* frame_timings_in_secs */ vec![0.001, 0.300, 0.300, 0.250], // Note that the the first timing is set to 0.001 so the animation starts immediately. If this value doesn't suit your needs, you can change it to another parameter.
        /* handle */ texture_atlas_hanle // your sprite sheet
        /* frame */ Vec2::new(4., 4.) // the length and height of your sprite sheet 
        /* direction_indexes */ AnimationDirectionIndexes::new(4, 3, 2, 1) // from the example above
        /* repeating */ true // if the animation is repeating or not
        /* blocking */ true, // if the animation should block others
        /* blocking_priority */ 1 // the priority for which animation should block other blocking animations
    ),
    "player_harvesting" // the name of the animation. will be used when sending an `AnimationEvent`
))
```

We can then start an animation by sending it over an `EventWriter<AnimationEvent>` like this
```rust
fn move_player(
    mut event_writer: EventWriter<AnimationEvent>
) {
    // your move logic here
    event_write.send(AnimationEvent("player_running", entity));
}
```

* **Note** that you can send an event of the same name multiple times even during animation without ruining it

* **Note** if you send an event with a different name the current animation of the entity will change immediately. 

Knowing this you can change the `player_running` animation in another system where I am checking collisions like this
```rust
fn check_collisions(
    mut commands: Commands,
    rapier_context: Res<RapierContext> // great 2d physics engine for lots of things we are using it for collision detection
    mut event_sender: EventWriter<AnimationEvent>
) {
    for pair in rapier_context.contact_pairs() {
        if pair.has_any_active_contacts() {

            let entity = pair.collider1();
            
            // send the event for the animating entity
            event_sender.send(AnimationEvent("player_die", entity));
            // despawn the entity after death
            commands.entity(entity).despawn();
            return;
        }
    }
}
```

* **Note** that `bevy_animations` will automatically remove your entity from it's own data structure if it doesn't exist in the `World` i.e when the entity despawns via `.despawn()`

* **Note** there is no functionality internally yet for doing a task like despawning an entity only after an animation is finished. This can be accomplished on your own however.

### Open Source
bevy_animations is open-source forever. You can contribute via the [`GitHub Repo`](https://github.com/y0Phoenix/bevy_animations)