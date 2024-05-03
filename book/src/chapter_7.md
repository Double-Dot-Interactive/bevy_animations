# LinearTimedAnimation

[LinearTimedAnimation](https://docs.rs/bevy_animations/latest/bevy_animations/struct.LinearTimedAnimation.html) is mainly used for animations on objects that have a very simple and linear animation. A door opening and closing would be an example of this.

The configuration properties for this animation are

* `AnimationName`
* `animation_frames`
* `frame_timings_in_secs`
* `repeating`

`AnimtionName` is explained in [chapter_2](./chapter_2.md#shared-properties) under shared properties.

* `animation_frames` is a `Vec` of `usize` numbers which are the x-index definitions for your animation on the sprite sheet
* `frame_timings_in_secs` is a `Vec` of `f32` numbers which are the time between frames. There should be one timing for each frame
* `repeating` is a bool to determine if the animation should repeat itself or not once finished

## Example

Heres an example of how you can add a `LinearTimedAnimation` to the animation pool.

```rust
animations.insert_animation(
    NewAnimation {
        handle: player_movement_texture.clone(), /* the handle for the TextureAtlas */
        animation: AnimationType::LinearTimed(
            LinearTimedAnimtion::new(
                    Vec::from(PLAYER_RUNNING_FRAMES), /* animation_frames */
                    Vec::from(PLYAER_FRAME_TIMING), /* frame_timings_in_secs */
                    true, /* repeating */
                ),
                "player_running", /* AnimationName */
        ),
    },
    Some(player_entity), /* specify an entity to add the animation to now instead of later */
)
```

## [Continue To Next Chapter ->](./chapter_8.md)
