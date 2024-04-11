# SingleFrameAnimation

[SingleFrameAnimation](https://docs.rs/bevy_animations/latest/bevy_animations/struct.SingleFrameAnimation.html) can be a useful animation for when you have specific stages of an animation. Jump, fall, and land animations are examples where a [SingleFrameAnimation](https://docs.rs/bevy_animations/latest/bevy_animations/struct.SingleFrameAnimation.html) would be used. All of them could be 1 frame but there time at which they happen can vary drastically based on the user input.

The configuration properties for this animation are

* `AnimationName`
* `direction_indexes`
* `blocking`
* `blocking_priority`
* `blocking_duration_in_sec`
* `frame`

`AnimtionName`, `direction_indexes`, `blocking`, `blocking_priority`, and `frame` are explained in [chapter_2](./chapter_2.md#shared-properties) under shared properties.

* `x_index_pos`is the x position of the single frame you want rendered by 0th index
* `blocking_duration_in_sec` is the duration of the blocking timer because a single frame animation doesn't have timed frames. So if this animation was blocking we need to now for how long.

## Example

Heres an example of how you can add a `SingleFrameAnimation` to the animation pool.

```rust
animations.insert_animation(
    NewAnimation {
        handle: player_movement_texture.clone(), /* the handle for the TextureAtlas */
        animation: AnimationType::SingleFrame(
            SingleFrameAnimation::new(
                    0, /* x_index_pos */
                    AnimationDirectionIndexes::IndexBased(FlipBased { /* direction_indexes */
                        x_direction_is_flipped: true,
                        x_direction_index: 0,
                    }),
                    true, /* blocking */
                    1, /* blocking_priority */
                    0.25, /* blocking_duration_in_sec */
                    Vec2::new(4., 4.) /* frame */
                ),
                "player_running", /* AnimationName */
        ),
    },
    Some(player_entity), /* specify an entity to add the animation to now instead of later */
)
```
