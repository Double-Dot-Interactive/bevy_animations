use std::{collections::HashMap, time::Duration, sync::{Arc, Mutex}};

use bevy::{prelude::*};

mod animations;
mod plugins;
mod types;

pub use animations::*;
pub use plugins::*;
pub use types::*;

#[derive(Debug, Resource, Default)]
pub struct AnimationsConfig {
    pixels_per_meter: f32
}

#[derive(Debug, Resource, Default)]
pub struct EntitesToRemove(Vec<Entity>);

#[derive(Component, Deref, DerefMut, Clone, Debug)]
pub struct AnimationTimer(pub Timer);

#[derive(Debug)]
pub struct AnimatingEntities {
    entity: Entity,
    in_blocking_animation: bool,
    animations: HashMap<&'static str, Arc<Mutex<AnimationType>>>,
    curr_animation: Arc<Mutex<AnimationType>>,
    curr_direction: AnimationDirection,
    last_valid_direction: AnimationDirection
}

#[derive(Default, Resource, Debug)]
pub struct Animations {
    entities: HashMap<Entity, AnimatingEntities>,
}