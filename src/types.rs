use crate::animations::*;
use crate::*;

/// This will determing which y index will be in the animating calculation
/// So if we have a sprite sheet like [`this`](https://github.com/y0Phoenix/bevy_animations/blob/master/example%20sprites/example.png?raw=true)
/// 
/// Our code implementation will look like this
/// ```
/// AnimationDirectionIndexes::new(
///     4,  // left
///     3,  // right
///     2,  // up
///     1   // down
/// );
/// ```
/// **Note** that bevy_animations uses a **1st index** basis for the DirectionIndexes instead of **0th index**
/// 
/// ###
/// We can then take that code and call `insert_animation()` from the Animations Resource like this
/// ```
/// use bevy::prelude::*;
/// use bevy_animations::*
/// 
/// fn setup_entity(
///     mut commands: Commands,
///     mut animations: ResMut<Animations>
/// ) {
///     let entity = commands.spawn(
///         AnimationDirection::Still // the `AnimationDirection` component is needed on the entity to determine the direction
///         SpriteSheetBundle {
///             texture_atlas: // your sprite sheet handle
///             transform: Transform::from_xyz(0., 0., 0.) // your desired location in the `World`
///         }
///         /* The rest of your entity configuration */
///     );
/// 
///     animations.insert_animation(
///         entity.id(),
///         AnimationType::Transform(
///             TransformAnimation::new(
///                 /* animation_frames */ vec![0, 1, 2, 3] // the x index for your frames to cycle through
///                 /* meters per frame */ 0.55 // your desired meters per frame
///                 /* handle */ texture_atlas_hanle // your sprite sheet
///                 /* frame */ Vec2::new(4., 4.) // the length and height of your sprite sheet
///                 /* direction_indexes */ AnimationDirectionIndexes::new(4, 3, 2, 1) // from the example above
///                 /* repeating */ true // if the animation is repeating or not
///             )
///         )
///     )
/// }
/// ```
/// 
/// **Note** how the `animation_frames` field from the animation definition above is 0th index based
#[derive(Clone, Debug)]
pub struct AnimationDirectionIndexes {
    pub left: usize,
    pub right: usize,
    pub up: usize,
    pub down: usize,
}

impl AnimationDirectionIndexes {
    pub fn new(left: usize, right: usize, up: usize, down: usize) -> Self{
        Self { left, right, up, down }
    }
}

impl Default for AnimationDirectionIndexes {
    fn default() -> Self {
        Self { left: 1, right: 1, up: 1, down: 1 }
    }
}

#[derive(Debug, Component)]
pub enum AnimationType {
    Timed(TimedAnimation, &'static str),
    Transform(TransformAnimation, &'static str)
}

impl AnimationType {
    pub fn get_atlas(&self) -> Handle<TextureAtlas> {
        match self {
            AnimationType::Timed(animation, _) => animation.handle.clone(),
            AnimationType::Transform(animation, _) => animation.handle.clone()
        }
    }
    pub fn timed_animation(&mut self) -> Option<&mut TimedAnimation> {
        match self {
            AnimationType::Timed(timing_animation, _) => Some(timing_animation),
            _ => None
        }
    }
    pub fn transform_animation(&mut self) -> Option<&mut TransformAnimation> {
        match self {
            AnimationType::Transform(transform_animation, _) => Some(transform_animation),
            _ => None
        }
    }
    pub fn get_name(&self) -> &'static str {
        match self {
            AnimationType::Timed(_, name) => &name,
            AnimationType::Transform(_, name) => &name
        }
    }
}

#[derive(Debug)]
pub struct AnimationEvent(pub &'static str, pub Entity);

#[derive(Debug, PartialEq, Eq, Clone, Default, Component)]
pub enum AnimationDirection {
    Left,
    Right,
    Up,
    Down,
    #[default]
    Still
}

impl AnimationDirection {
    const LEFT: Vec2 = Vec2::new(-1., 0.);
    const RIGHT: Vec2 = Vec2::new(1., 0.);
    const UP: Vec2 = Vec2::new(0., 1.);
    const DOWN: Vec2 = Vec2::new(0., -1.);
    const STILL: Vec2 = Vec2::new(0., 0.);
    pub fn get_direction(direction: &Self) -> Vec2 {
        match direction {
            AnimationDirection::Left => AnimationDirection::LEFT,
            AnimationDirection::Right => AnimationDirection::RIGHT,
            AnimationDirection::Up => AnimationDirection::UP,
            AnimationDirection::Down => AnimationDirection::DOWN,
            AnimationDirection::Still => AnimationDirection::STILL
        }
    }
}