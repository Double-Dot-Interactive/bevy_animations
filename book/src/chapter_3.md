# API

We will talk about the major features that the API provides for getting information about animations, inserting animations, starting animations, etc.

This will be a lengthy chapter that will give some examples of use cases for each element of the API to hopefully help with comprehension.

There are three main areas of the API that should be interacted with.

* Initilization which was talked about in [chapter_1](./chapter_1.md)
* Interacting with the [Animations](https://docs.rs/bevy_animations/latest/bevy_animations/struct.Animations.html) resource. Which will be used to create animations, add animations to entities, and get information about animations like if they're playing or not.
* Starting and Reseting animations via [AnimationEvent](https://docs.rs/bevy_animations/latest/bevy_animations/struct.AnimationEvent.html), [FXAnimationEvent](https://docs.rs/bevy_animations/latest/bevy_animations/struct.FXAnimationEvent.html), and [ResetAnimationEvent](https://docs.rs/bevy_animations/latest/bevy_animations/struct.ResetAnimationEvent.html)

There is also an arbitrary way of interacting with the API via [AnimationDirection](https://docs.rs/bevy_animations/latest/bevy_animations/enum.AnimationDirection.html). When the [AnimationDirection](https://docs.rs/bevy_animations/latest/bevy_animations/enum.AnimationDirection.html) is changed, the direction of the sprite will change based off of your configuration for the animation itself.

## Animations Resource

There are many methods on the [Animations](https://docs.rs/bevy_animations/latest/bevy_animations/struct.Animations.html) resoure. We will talk in depth about each one here.

**Note** that each animation will be discussed more in other chapters so don't worry about the creating the animations yet. Right now we will just talk about how to use these methods and what they accomplish.

### [insert_animation()](https://docs.rs/bevy_animations/latest/bevy_animations/struct.Animations.html#method.insert_animation) takes two parameters

* [NewAnimation](https://docs.rs/bevy_animations/latest/bevy_animations/struct.NewAnimation.html) which is the animation your inserting
* `Option<Entity>` which can be `Some(Entity)` if you want to add an entity to your animation straight away.

Heres an example of using this method.

```rust
fn spawn_player(
    mut commands: Commands,
    mut animations: ResMut<Animations>
) {
    let player_entity = commands.spawn(TransformBundle::from_transform(Transform::from_xyz(0., 0., 0.))).id();

    animations.insert_animation(
        NewAnimation {
            handle: player_movement_texture.clone(), /* the handle for the TextureAtlas */
            animation: AnimationType::Timed(
                TimedAnimation::new(
                        PLAYER_RUNNING_FRAMES.into(), /* animation_frames */
                        PLAYER_RUNNING_TIMINGS.into(), /* frame_timings_in_secs */
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
}
```

After an animation is added you can add it to an entity if you haven't already. You can only start an animation once it is added to an entity.

### [insert_entity()](https://docs.rs/bevy_animations/latest/bevy_animations/struct.Animations.html#method.insert_entity) takes one parameter

* [Entity](https://docs.rs/bevy/latest/bevy/prelude/struct.Entity.html) which will be the entity you wish to insert into the entity pool.

Here's an example of using this method.

```rust
fn spawn_player(
    mut commands: Commands,
    mut animations: ResMut<Animations>
) {
    let player_entity = commands.spawn(TransformBundle::from_transform(Transform::from_xyz(0., 0., 0.))).id();
    animation.insert_entity(player_entity).unwrap(); // returns Err if the entity already exists in the entity pool
}
```

In this example we define a system that will spawn an entity which will be the player. We then insert the `player_entity` entity into the `Animations` entity pool for later use when adding animations to it. Also if you want to add an already defined animation at the same time as the entity you can use `add_animation_to_entity()` below.

### [add_animation_to_entity()](https://docs.rs/bevy_animations/latest/bevy_animations/struct.Animations.html#method.add_animation_to_entity) takes two parameter

* [AnimationName](https://docs.rs/bevy_animations/latest/bevy_animations/type.AnimationName.html) which is the name of the animation
* [Entity](https://docs.rs/bevy/latest/bevy/prelude/struct.Entity.html) which will be the entity you are adding the animation to

Here's an example of using this method.

```rust
fn fn spawn_player(
    mut commands: Commands,
    mut animations: ResMut<Animations>
) {
    let player_entity = commands.spawn(TransformBundle::from_transform(Transform::from_xyz(0., 0., 0.))).id();
    animation.add_animation_to_entity("player_idle", player_entity).unwrap(); // returns Err if the entity already exists in the entity pool
}
```

In this example we define a system which spawns the player in and adds the `player_idle` animation to the players entity. This will also add the entity to the pool if it doesn't exist already. Knowing this will prevent you from using two lines of code if you inserted the entity before utilizing this method.

### [insert_fx_animation()](https://docs.rs/bevy_animations/latest/bevy_animations/struct.Animations.html#method.insert_fx_animation) only takes one parameter

* [NewAnimation](https://docs.rs/bevy_animations/latest/bevy_animations/struct.NewAnimation.html) which is the animation you inserting.

Here's an example of using this method.

```rust
fn spawn_fx_animations(
    mut animations: ResMut<Animations>
) {
    animations.insert_fx_animation(NewAnimation {
        handle: fx_handle.clone(), /* the handle for the TextureAtlas */
        animation: AnimationType::Timed(
            TimedAnimation::new(
                JUMP_START_FRAMES.into(), /* animation_frames */
                JUMP_START_TIMINGS.into(), /* frame_timings_in_secs */
                Vec2::new(9., 16.), /* frame */
                AnimationDirectionIndexes::FX(FXBasedDirection { /* direction_indexes */
                    index: 7
                }),
                false, /* repeating */
                false, /* blocking */
                0 /* blocking_priory */
            ), 
            "jump_start" /* AnimationName */
        )
    })
    ;
}
```

Once the fx animation is inserted into the pool it can be started via [FXAnimationEvent](https://docs.rs/bevy_animations/latest/bevy_animations/struct.FXAnimationEvent.html).

### [in_blocking_animation()](https://docs.rs/bevy_animations/latest/bevy_animations/struct.Animations.html#method.in_blocking_animation) takes one parameter

* [Entity](https://docs.rs/bevy/latest/bevy/prelude/struct.Entity.html) which is the entity you want to check whether or not is currently playing a blocking animation. It returns a `Option<Bool>`, the value is `None` if the entity specified doesn't exist in the entity pool, and `Some(bool)` if the entity is playing a blocking animation.

Here's an example of using this method.

```rust
fn is_blocking(
    query: Query<Entity, With<Player>>,
    animations: Res<Animations>
) -> bool {
    let Ok(entity) = query.get_single();
    !animations.in_blocking_animation(entity).unwrap()
}
```

In this example we define a run condition system that can be used to determine whether or not to run another system. If the player is in a blocking animation we won't run whatever system we use this on, and vise versa.

### [in_animation()](https://docs.rs/bevy_animations/latest/bevy_animations/struct.Animations.html#method.in_animation) takes one parameter

* [Entity](https://docs.rs/bevy/latest/bevy/prelude/struct.Entity.html) which is the entity you want to check whether or not is currently playing an animation.

Here's an example of using this method.

```rust
fn is_animating(
    mut commands: Commands,
    query: Query<Entity, With<Player>>,
    animations: Res<Animations>
) {
    let entity = query.get_single().unwrap();
    if !animations.in_animation(entity).unwrap() {
        commands.despawn(entity);
    }
}
```

In this example we define a system that will despawn an entity once they aren't in an animation (in other words they are done animating and have completed their lifecycle in your game).

### [doing_animation()](https://docs.rs/bevy_animations/latest/bevy_animations/struct.Animations.html#method.doing_animation) takes two parameters

* [Entity](https://docs.rs/bevy/latest/bevy/prelude/struct.Entity.html) which is the entity you want to check whether or not is currently playing the animation specified
* [AnimationName](https://docs.rs/bevy_animations/latest/bevy_animations/type.AnimationName.html) which is the name of the animation

Here's an example of using this method.

```rust
fn in_perry_animation(
    query: Query<Entity, With<Player>>,
    animations: Res<Animations>
) -> bool {
    let Ok(entity) = query.get_single();
    !animations.doing_animation(entity, "player_perry").unwrap()
}
```

In this example we define a run condition system that can be used to determine whether or not to run another system. If the player is in the `player_perry` animation we won't run whatever system we use this on, and vise versa. This can be usefull for determining whether or not to remove health from the player in a damage system.

### [has_entity()](https://docs.rs/bevy_animations/latest/bevy_animations/struct.Animations.html#method.has_entity) takes one parameter

* [Entity](https://docs.rs/bevy/latest/bevy/prelude/struct.Entity.html) which is the entity you want to check whether or not is inserted in the `bevy_animation` entity pool.

Here's an example of using this method.

```rust
fn load_foliage(
    query: Query<Entity, With<Foliage>>,
    mut animations: ResMut<Animations>
) {
    for entity in query.iter(
        if !animations.is_inserted(entity) {
            animations.add_animation_to_entity("foliage_idle", entity);
        }
    )
}
```

In this example we are defining a system in which we add the `foliage_idle` animation to a `Foliage` entity if it isn't in the entity pool. This example can be usefull for transitioning into different loading states. Let's say you are entering an area which has only just loaded. Well you'll need to add the `foliage_idle` animation to only the foliage that has just loaded.

### [entity_has_animation()](https://docs.rs/bevy_animations/latest/bevy_animations/struct.Animations.html#method.entity_has_animation) takes two parameters

* [AnimationName](https://docs.rs/bevy_animations/latest/bevy_animations/type.AnimationName.html) which is the name of the animation
* [Entity](https://docs.rs/bevy/latest/bevy/prelude/struct.Entity.html) which is the entity you want to check whether or not has the animation specified.

Here's an example of using this method.

```rust
fn load_foliage(
    query: Query<Entity, With<Foliage>>,
    mut animations: ResMut<Animations>
) {
    for entity in query.iter(
        if !animations.entity_has_animation(entity, "foliage_idle") {
            animations.add_animation_to_entity("foliage_idle", entity);
        }
    )
}
```

In this example we are defining a system in which we add the `foliage_idle` animation to a `Foliage` entity if the entity doesn't have the animation.

### [get_mut_map()](https://docs.rs/bevy_animations/latest/bevy_animations/struct.Animations.html#method.is_inserted) takes no parameters and should only be used for debugging purposes and not for production use

### [get_map()](https://docs.rs/bevy_animations/latest/bevy_animations/struct.Animations.html#method.is_inserted) takes no parameters and should only be used for debugging purposes and not for production use

## Animation Events

In order for an animation to start, we need to send either an [AnimationEvent](https://docs.rs/bevy_animations/latest/bevy_animations/struct.AnimationEvent.html) or an [FXAnimationEvent](https://docs.rs/bevy_animations/latest/bevy_animations/struct.FXAnimationEvent.html) over an [EventWriter](https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.EventWriter.html).

Here's an example of managing a players animations.

```rust
fn animate_player(
    query: Query<(Entity, &Player, &AnimationDirection, &Transform)>,
    mut animations: ResMut<Animations>,
    mut event_writer: EventWriter<AnimationEvent>
) {
    for (entity, player, direction, transform) in query.iter() {
        let translation = transform.translation;
        // if the player is moving
        if *direction != AnimationDirection::None {
            if player.running {
                event_writer.send(AnimationEvent("player_running", entity));
            }
            else {
                event_writer.send(AnimationEvent("player_walking", entity));
            }
        }
        // else we play the idle animation
        else {
            event_writer.send(AnimationEvent("player_idle", entity));
        }
        // if the player just started a jump
        if player.jump == JumpType::Started {
            event_writer.send(FXAnimationEvent("jump_start", translation));
        }
        // if the player just landed
        else if player.jump == JumpType::Landed {
            player.jump.jump_type = JumpType::None;
            event_writer.send(FXAnimationEvent("jump_land", translation));
        }
    }
}
```

In this example we are doing some simple logic to determine which animation to play for the player. This is really all it takes, `bevy_animations` will take care of cycling frames and making sure the animation that is requested is worthy of playing.

When an `FXAnimationEvent` is sent, `bevy_animations` will spawn a new entity with the `FXAnimation` attached to it, after the FX is finished the entity will be despawned unless the FX is repeatable.

You can also reset an animation on an entity if that's something you require. Here's an example of this.

```rust
fn animate_player(
    query: Query<(Entity, &Player)>,
    mut animations: ResMut<Animations>,
    mut event_writer: EventWriter<ResetAnimationEvent>,
    input: Res<Input>
) {
    for (entity, player) in query.iter() {
        // if we are attacking and we get hit and the user is trying to attack again we should restart the attack animation
        if player.hit && player.attacking && animations.doing_animation(entity, "player_attack").unwrap() && input.pressed(KeyCode::Z) {
            event_writer.send(ResetAnimationEvent(entity))
        }
    }
}
```

In this example we are defining a system that will reset the `player_attack` animation if the player gets hit while they are attacking and also while the user is trying to attack again (they are holding the key down). This would be functionally important for your game if you want the player to stop the attack animation if they get hit.

## [Coninue To Next Chapter ->](./chapter_4.md)
