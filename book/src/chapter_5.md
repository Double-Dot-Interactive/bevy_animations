# TimedAnimation

[TimedAnimation](https://docs.rs/bevy_animations/latest/bevy_animations/struct.TimedAnimation.html) is the most versatile animation. It's great for idle animations, walking animations, attacking animations and so much more.

The configuration properties for this animation are

* `AnimationName`
* `animation_frames`
* `frame_timings_in_secs`
* `frame`
* `direction_indexes`
* `repeating`
* `blocking`
* `blocking_priority`

`AnimtionName`, `frame`, `direction_indexes`, `blocking`, and `blockin_priority` are explained in [chapter_2](./chapter_2.md#shared-properties) under shared properties.

* `animation_frames` is a `Vec` of `usize` numbers which are the x-index definitions for your animation on the sprite sheet
* `frame_timings_in_secs` is a `Vec` of `f32` numbers which are the time between frames. There should be one timing for each frame
* `repeating` is a bool to determine if the animation should repeat itself or not once finished

## Example

Heres an example of how you can add a `TimedAnimation` to the animation pool.

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

## [Continue To Next Chapter ->](./chapter_6.md)
