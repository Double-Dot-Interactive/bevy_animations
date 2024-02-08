# bevy_animations is a Lightweight 2d animations engine built for Bevy

Bevy Animations is still in beta and may incurr major API and backend changes in the future.

## What bevy_animations accomplishes

* Fully incorporated with Bevy ECS
* Easy to use builder pattern syntax
* Creates animations to use on entities with custom configuration
* Automatic dropping of animating entites

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
* user defining which y indexes are left, right, up and down facing sprites or if the sprites should be flipped instead
* certain animations can block others from happening
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
        Animator::default(), // the `AnimationDirection` component is needed on the entity to determine the direction
        SpriteSheetBundle {
            texture_atlas: // your sprite sheet handle
            transform: Transform::from_xyz(0., 0., 0.) // your desired location in the `World`
        }
        /* The rest of your entity configuration */
    );
}
```

**Note** if you don't add the `AnimationDirection` component to your entity it will seem as though your animations will never be inserted because `bevy_animations` is looking for the
`AnimationDirection` component in it's `Query`s

#### You can then add your animations to `ResMut<Animations>` like this

```rust
animations.insert_animation(
    NewAnimation {
        handle: player_movement_texture.clone(), /* the handle for the TextureAtlas */
        animation: AnimationType::Timed(
            TransformAnimation::new(
                    Vec::from(PLAYER_RUNNING_FRAMES), /* animation_frames */
                    PLAYER_RUNNING_METERS_PER_FRAME, /* meters_per_frame */
                    Vec2::new(14., 38.), /* frame */
                    AnimationDirectionIndexes::FlipBased(FlipBasedDirection { /* direction_indexes */
                        left_direction_is_flipped: true,
                        x_direction_index: 3,
                    }),
                    true, /* repeating */
                ),
                "player_running", /* AnimationName */
        ),
    },
    Some(player_entity), /* specify an entity to add the animation to now instead of later */
)
```

**Note** if you have a one directional animation you can use `AnimationDirectionIndexes::one_directional()`

**Note** it is on you to make sure you are passing the correct strings to bevy_animations to animate your entity

#### You can also add a `TimedAnimation` like this

```rust
animations.insert_animation(
    NewAnimation {
        handle: player_movement_texture.clone(), /* the handle for the TextureAtlas */
        animation: AnimationType::Timed(
            TimedAnimation::new(
                    Vec::from(PLAYER_RUNNING_FRAMES), /* animation_frames */
                    Vec::from(PLAYER_RUNNING_TIMINGS), /* frame_timings_in_secs */
                    Vec2::new(14., 38.), /* frame */
                    AnimationDirectionIndexes::FlipBased(FlipBasedDirection { /* direction_indexes */
                        left_direction_is_flipped: true,
                        x_direction_index: 3,
                    }),
                    true, /* repeating */
                    false, /* blocking */
                    0 /* blocking_priory */
                ),
                "player_running", /* AnimationName */
        ),
    },
    Some(player_entity), /* specify an entity to add the animation to now instead of later */
)
```

#### We can then start an animation by sending it over an `EventWriter<AnimationEvent>` like this

```rust
fn move_player(
    mut event_writer: EventWriter<AnimationEvent>,
    player_query: Query<Entity, With<Player>>
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
    mut query: Query<&mut Animator, With<Player>> // specify the `With` to get the entity associated with your custom component 
) {
    // your move logic here...

    let mut animator = query.single_mut(); // get the direction via query

    animator.change_direction(AnimationDirection::Left); // the direction can be changed like this

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

### Versioning

| bevy  | bevy_animations  |
| ----- | ---------------  |
| 0.12.x | 0.5.x             |
| 0.11.x | 0.4.x             |
| 0.10.x | 0.3.x             |
| 0.9.x  | 0.2.x             |

### More Documentation

If you need more in depth Documentation and more examples for all of the current implementations read the [bevy_animations book](https://github.com/Double-Dot-Interactive/bevy_animations/tree/master/book/src/README.md) or visit the [api docs](https://docs.rs/bevy_animations/latest/bevy_animations/)

### Open Source

bevy_animations is open-source forever. You can contribute via the [`GitHub Repo`](https://github.com/Double-Dot-Interactive/bevy_animations)
