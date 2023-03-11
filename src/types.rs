use crate::animations::*;
use crate::*;

/// This will determing which y index will be in the animating calculation
/// So if we have a sprite sheet like this
/// 
/// right facing sprites
/// up facing sprites
/// left facing sprites
/// down facing sprites
/// Our implentation would look like this
/// ```
/// AnimationDirectionIndexes::new(
///     1,  // left
///     3,  // right
///     2,  // up
///     4   // down
/// );
/// ```
#[derive(Default, Clone, Debug)]
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