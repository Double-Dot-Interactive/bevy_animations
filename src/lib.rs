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

impl Animations {
    pub fn insert_animation(&mut self, key: Entity, value: AnimationType) -> &mut Self {
        if let Some(entity) = self.entities.get_mut(&key) {
            entity.animations.insert(value.get_name(), Arc::new(Mutex::new(value)));
            return self;
        }
        else {
            let value = Arc::new(Mutex::new(value));
            let mut map = HashMap::new();
            map.insert(value.lock().unwrap().get_name(), Arc::clone(&value));
            self.entities.insert(key, AnimatingEntities { 
                entity: key, 
                animations: map, 
                curr_animation: value, 
                curr_direction: AnimationDirection::Still,
                last_valid_direction: AnimationDirection::Down,
                in_blocking_animation: false
            });
            return self;
        }
    }
    pub fn in_blocking_animation(&self, entity: Entity) -> Option<bool> {
        match self.entities.get(&entity) {
            Some(animating_entity) => Some(animating_entity.in_blocking_animation),
            None => None
        }
    }
    /// Returns `true` if the `Entity` exists in the `Animations` map
    pub fn is_inserted(&self, key: &Entity) -> bool {
        if let Some(_) = self.entities.get(key) {
            return true;
        }
        false
    }
}