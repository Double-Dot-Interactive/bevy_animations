# TransformAnimation

[TransformAnimation](https://docs.rs/bevy_animations/latest/bevy_animations/struct.TransformAnimation.html) is another versatile animation that can be used on movement based animations like walking and running. This can also be usefull on entities that have speed changes which should affect how the animation looks.

The configuration properties for this animation are

* `AnimationName`
* `animation_frames`
* `meters_per_frame`
* `frame`
* `direction_indexes`
* `repeating`

`AnimtionName`, `frame`, and `direction_indexes` are explained in [chapter_2](./chapter_2.md#shared-properties) under shared properties.

* `animation_frames` is a `Vec` of `usize` numbers which are the x-index definitions for your animation on the sprite sheet
* `meters_per_frame` is an `f32` which determines how many meters should the entity advance before moving to the next frame
* `repeating` is a bool to determine if the animation should repeat itself or not once finished

## Example

Heres an example of how you can add a `TransformAnimation` to the animation pool.

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

## [Coninue To Next Chapter ->](./chapter_7.md)
