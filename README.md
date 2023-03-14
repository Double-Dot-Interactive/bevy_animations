# bevy_animations is a Lightweight 2d animations engine built for Bevy

## What bevy_animations accomplishes
1. using bevy_animations is easy and the animation configurations are simple

2. bevy_animations is fast enough to handle all of the entities you want animated

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
* specified timings or `meters_per_frame` for each frame
* user defining which y indexes are left, right, up and down facing sprites
* timed animations can block others from happening
* utilizing a priortity based system so you can define multiple ***blocking*** animations with different priorities to render

### How to define a bevy_animations animation

#### You first need to spawn an entity using `Commands` like this

```rust
use bevy_animations::*;
use bevy::prelude::*;

fn entity_setup(
    mut commands: Commands,
    animations: ResMut<Animations>
) {
    let entity = commands.spawn(
        AnimationDirection::Still // the `AnimationDirection` component is needed on the entity to determine the direction
        SpriteSheetBundle {
            texture_atlas: // your sprite sheet handle
            transform: Transform::from_xyz(0., 0., 0.) // your desired location in the `World`
        }
        /* The rest of your entity configuration */
    );
}
```
**Note** if you are using a one directional sprite you still **NEED** to add the `AminationDirection` component

**Note** if you don't add the `AnimationDirection` component to your entity it will seem as though your animations will never be inserted because `bevy_animations` is looking for the
`AnimationDirection` component in it's `Query`s

#### You can then add your animations to `ResMut<Animations>` like this

```rust
animations.insert_animation(
    entity.id(), // the entity is needed to determine which `Handle<TextureAtlas>` is being manipulated
    AnimationType::Transform(
        TransformAnimation::new(
            /* animation_frames */ vec![0, 1, 2, 3] // the x index for your frames to cycle through
            /* meters_per_frame */ 0.55 // your desired meters per frame
            /* handle */ texture_atlas_hanle // your sprite sheet
            /* frame */ Vec2::new(4., 4.) // the length and height of your sprite sheet
            /* direction_indexes */ AnimationDirectionIndexes::new(4, 3, 2, 1) // the indexes to determine the correct sprite for the direction
            /* repeating */ true // if the animation is repeating or not
        )
    ),
    "player_running" // the name of the animation. will be used when sending an `AnimationEvent`
)
```
**Note** if you have a one directional animation you can use `AnimationDirectionIndexes::default()` or set everything to 1 `AnimationDirectionIndexes::new(1, 1, 1, 1)`

#### You can also add a `TimedAnimation` like this
```rust
animations.insert_animation(entity.id(), AnimationType::Timed(
    TimedAnimation::new(
        /* animation_frames */ vec![0, 1, 2, 3] // the x index for your frames to cycle through, 
        /* frame_timings_in_secs */ vec![0.001, 0.300, 0.300, 0.250], // Note that the the first timing is set to 0.001 so the animation starts immediately. If this value doesn't suit your needs, you can change it to another parameter.
        /* handle */ texture_atlas_hanle // your sprite sheet
        /* frame */ Vec2::new(4., 4.) // the length and height of your sprite sheet 
        /* direction_indexes */ AnimationDirectionIndexes::new(4, 3, 2, 1) // the indexes to determine the correct sprite for the direction
        /* repeating */ true // if the animation is repeating or not
        /* blocking */ true, // if the animation should block others
        /* blocking_priority */ 1 // the priority for which animation should block other blocking animations
    ),
    "player_die" // the name of the animation. will be used when sending an `AnimationEvent`
))
```

#### We can then start an animation by sending it over an `EventWriter<AnimationEvent>` like this
```rust
fn move_player(
    mut event_writer: EventWriter<AnimationEvent>
) {
    // your move logic here...

    event_writer.send(AnimationEvent("player_running", entity));
}
```

* **Note** that you can send an event of the same name multiple times even while an animation is in progress without ruining it

* **Note** an animation that has been sent will animate till end or repeat forever

#### If you want to change the direction of the animation you will query it from the `AnimatingEntity` like this
```rust
fn move_player(
    mut event_writer: EventWriter<AnimationEvent>,
    mut query: Query<&mut AnimationDirection, With<Player>> // specify the `With` to get the entity associated with your custom component 
) {
    // your move logic here...

    let mut direction = query.single_mut(); // get the direction via query

    direction = AnimationDirection::Left; // the direction can be changed like this

    event_writer.send(AnimationEvent("player_running", entity));
}
```
* **Note** if you send an event with a different name the current animation of the entity will change immediately unless the current animation is blocking or has a higher priority.

#### Knowing this you can change the `player_running` animation to `player_die` in another system where you could check collisions like this
```rust
fn check_collisions(
    mut commands: Commands,
    rapier_context: Res<RapierContext> // great 2d physics engine for lots of things we are using it for collision detection
    mut event_writer: EventWriter<AnimationEvent>,
    player_query: Query<Entity, With<Player>>,
    bullet_query: Query<Entity, With<Bullet>>
) {
    let player_entity = player_query.single();

    for bullet_entity in bullet_query.iter() {
        if let Some(_) = context.contact_pair(bullet_entity, player_entity) {
            // send the event for the animating entity
            event_writer.send(AnimationEvent("player_die", entity));
            // despawn the entity after death
            commands.entity(player_entity).despawn();
            commands.entity(bullet_entity).despawn();
        }         
    }
}
```

* **Note** that `bevy_animations` will automatically remove your entity from it's own data structure if it doesn't exist in the `World` i.e when the entity despawns via `.despawn()`

* **Note** there is no functionality internally yet for doing a task like despawning an entity only after an animation is finished. This can be accomplished on your own however.

### Open Source
bevy_animations is open-source forever. You can contribute via the [`GitHub Repo`](https://github.com/y0Phoenix/bevy_animations)