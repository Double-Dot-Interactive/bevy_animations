use crate::*;

pub type AnimationName = &'static str;

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
#[derive(Clone, Debug, Copy)]
pub enum AnimationDirectionIndexes {
    IndexBased(IndexBasedDirection),
    FlipBased(FlipBasedDirection)
}

/// Used to define the direction index. If you don't have sprites with multiple directions this will
/// flip them when the `AnimationDirection` changes
///
/// **Note** if you have each direction already in a sprite sheet use `IndexBasedDirection` as it
/// is functionally more proper
#[derive(Debug, Clone, Copy)]
pub struct FlipBasedDirection {
    // /// To Determine if the Sprite Should be FLipped
    // pub is_flipped: bool,
    /// To Determine if the Left Facing Sprites are Left Facing When Flipped or Not 
    pub left_direction_is_flipped: bool,
    /// To Determine which Index the Horizontal Directions Sprites are
    pub x_direction_index: usize,

}

/// Used to define the direction indexes for the animations
///
/// **Note** for this functionality to work properly your sprite sheet should be formatted in a certain way
/// It should have frames for each direction you need as the y index on the grid.
#[derive(Debug, Clone, Copy)]
pub struct IndexBasedDirection {
    /// The Y index on the Sprite Sheet for Left Facing Sprites
    pub left: usize,
    /// The Y index on the Sprite Sheet for Right Facing Sprites
    pub right: usize,
    /// The Y index on the Sprite Sheet for Upward Facing Sprites
    pub up: usize,
    /// The Y index on the Sprite Sheet for Downward Facing Sprites
    pub down: usize,

}

/// Used to be a sortof Option type for getting the y-index of a sprite on a sprite sheet
pub enum YIndex {
    Index(usize),
    Flip(bool, usize)
}

// impl AnimationDirectionIndexes {
//     pub fn new(left: usize, right: usize, up: usize, down: usize) -> Self{
//         Self { left, right, up, down }
//     }
// }

impl Default for AnimationDirectionIndexes {
    fn default() -> Self {
        Self::IndexBased(IndexBasedDirection { left: 1, right: 1, up: 1, down: 1 }) 
    }
}

#[derive(Debug, Component)]
pub enum AnimationType {
    // This is Primarily for This Is Primarily For Animations on players or NPCs, for example shooting a bow or reloading a gun 
    Timed(TimedAnimation, AnimationName),
    /// This Is Primarily For Animations on players or NPCs, for example walking or running
    Transform(TransformAnimation, AnimationName),
    /// This Is Primarily For Animations on objects, for example doors to open and close
    LinearTimed(LinearTimedAnimation, AnimationName),
    /// This Is Primarily For Animations on objects, for example a projectile
    LinearTransform(LinearTransformAnimation, AnimationName),
}

impl AnimationType {
    pub fn get_atlas(&self) -> Handle<TextureAtlas> {
        match self {
            AnimationType::Timed(animation, _) => animation.handle.clone(),
            AnimationType::Transform(animation, _) => animation.handle.clone(),
            AnimationType::LinearTimed(animation, _) => animation.handle.clone(),
            AnimationType::LinearTransform(animation, _) => animation.handle.clone(),
        }
    }
    pub fn timed_animation(&mut self) -> Option<&mut TimedAnimation> {
        match self {
            AnimationType::Timed(timed_animation, _) => Some(timed_animation),
            _ => None
        }
    }
    pub fn linear_timed_animation(&mut self) -> Option<&mut LinearTimedAnimation> {
        match self {
            AnimationType::LinearTimed(linear_timed_animation, _) => Some(linear_timed_animation),
            _ => None
        }
    }
    pub fn linear_transform_animation(&mut self) -> Option<&mut LinearTransformAnimation> {
        match self {
            AnimationType::LinearTransform(linear_transform_animation, _) => Some(linear_transform_animation),
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
            AnimationType::Transform(_, name) => &name,
            AnimationType::LinearTimed(_, name) => &name,
            AnimationType::LinearTransform(_, name) => &name,
        }
    }
    pub fn reset_animation(&mut self) {
        match self {
            AnimationType::Timed(animation, _) => animation.reset_animation(None, None),
            AnimationType::Transform(animation, _) => animation.reset_animation(None, None),
            AnimationType::LinearTimed(animation, _) => animation.reset_animation(None),
            AnimationType::LinearTransform(animation, _) => animation.reset_animation(None, None)
        }
    }
}

/// Send a request to animate the `Entity` with the animation dictated by the `AnimationName`
/// 
/// # Example
/// ```rust
/// use bevy::prelude::*;
/// use bevy_animations::*;
/// 
/// fn move_player(
///     player_query: Query<(&Transform Entity), With<Player>>,
///     mut animation_event_writer: EventWriter<AnimationEvent>
/// ) {
///     let (mut transform, player_entity) = player_query.single_mut();    
/// 
///     /* you move logic here... */
/// 
///     animation_event_writer.send(AnimationEvent("player_running", player_entity));
/// }
/// ```
/// 
/// * **Note** that you can send an event of the same name multiple times even while an animation is in progress without ruining it
///
/// * **Note** an animation that has been sent will animate till end or repeat forever
#[derive(Debug)]
pub struct AnimationEvent(pub AnimationName, pub Entity);

/// Send a request to reset the animation of an `Entity`
/// 
/// # Example
/// ```rust
/// use bevy::prelude::*;
/// use bevy_animations::*;
/// 
/// fn move_player(
///     player_query: Query<(&Transform Entity), With<Player>>,
///     mut reset_animation_event_writer: EventWriter<ResetAnimationEvent>
/// ) {
///     let (mut transform, player_entity) = player_query.single_mut();    
/// 
///     /* you move logic here... */
/// 
///     reset_animation_event_writer.send(ResetAnimationEvent(player_entity));
/// }
/// ```
/// 
/// * **Note** this will overwrite an animation request in the same frame
#[derive(Debug)]
pub struct ResetAnimationEvent(pub Entity);

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
