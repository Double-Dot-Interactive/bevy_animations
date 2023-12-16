use std::{collections::HashMap, time::Duration, sync::{Arc, Mutex}};

use bevy::prelude::*;

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

#[derive(Component, Deref, DerefMut, Clone, Debug, Default)]
pub struct AnimationTimer(pub Timer);

#[derive(Debug)]
pub struct AnimatingEntity {
    pub entity: Entity,
    pub in_blocking_animation: bool,
    pub animations: HashMap<AnimationName, Arc<Mutex<AnimationType>>>,
    pub curr_animation: Arc<Mutex<AnimationType>>,
    pub curr_direction: AnimationDirection,
    pub last_valid_direction: AnimationDirection,
    pub curr_animation_called: bool,
    pub fx_animation: bool
}

#[derive(Default, Resource, Debug)]
pub struct Animations {
    entities: HashMap<Entity, AnimatingEntity>,
    fx_animations: HashMap<AnimationName, Arc<Mutex<AnimationType>>>
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
            self.entities.insert(key, AnimatingEntity { 
                entity: key, 
                animations: map, 
                curr_animation: value, 
                curr_direction: AnimationDirection::Still,
                last_valid_direction: AnimationDirection::default(),
                in_blocking_animation: false,
                curr_animation_called: false,
                fx_animation: false
            });
            return self;
        }
    }
    /// insert an FX animation this. In order to start the FX animation send it through an `EventWriter(FXAnimationEvent(AnimationName))`
    pub fn insert_fx_animation(&mut self, key: AnimationName, value: AnimationType) -> &mut Self {
        if let Some(_) = self.fx_animations.get(key) {
            return self;
        }
        else {
            self.fx_animations.insert(key, Arc::new(Mutex::new(value)));
            return self;
        }
    }
    /// Add an FX animation to a new `AnimatingEntity`. This will start the animation specified.
    pub fn start_fx_animation(&mut self, key: AnimationName, commads: &mut Commands, pos: Vec3) -> Result<(), ()> {
        // get the animation from the FX animations
        let Some(animation) = self.fx_animations.get(key) else { return Err(()) };
        let unlock_animation = animation.lock().unwrap();

        // grab the atlas from the animations and spawn a new entity with the atlas at the specified pos
        let atlas = unlock_animation.get_atlas();
        let entity = commads.spawn(SpriteSheetBundle {
            texture_atlas: atlas,
            transform: Transform::from_translation(pos),
            ..Default::default()
        }).id();

        // add the animation and entity to a new AnimatingEntity to be animated
        self.entities.insert(entity, AnimatingEntity {
            entity,
            in_blocking_animation: false,
            animations: HashMap::new(),
            curr_animation: animation.clone(),
            curr_direction: AnimationDirection::default(),
            last_valid_direction: AnimationDirection::Down,
            curr_animation_called: false,
            fx_animation: true,
        });
        return Ok(());
    } 
    /// returns Some(bool) if the entity exists and Some(true) if the entity is in a blocking animation
    /// 
    /// returns None if the entity was not found 
    /// 
    /// usefull to determine for example whether or not to move an entity
    pub fn in_blocking_animation(&self, entity: Entity) -> Option<bool> {
        match self.entities.get(&entity) {
            Some(animating_entity) => Some(animating_entity.in_blocking_animation),
            None => None
        }
    }
    /// returns Some(bool) if the entity exists and Some(true) if the entity is in an animation
    /// 
    /// returns None if the entity was not found 
    /// 
    /// useful for determining for example whether or not to initate another animation
    pub fn in_animation(&self, entity: Entity) -> Option<bool> {
        match self.entities.get(&entity) {
            // this is functionally the same as checking if the entity is in an animation
            Some(animating_entity) => Some(animating_entity.curr_animation_called),
            None => None
        }
    }
    /// returns Some(bool) if the entity exists and Some(true) if the entity is in the animation specified
    /// 
    /// returns None if the entity was not found 
    /// 
    /// useful for determining for example whether or not to initate another animation
    pub fn doing_animations(&self, entity: Entity, animation_name: AnimationName) -> Option<bool> {
        match self.entities.get(&entity) {
            // this is functionally the same as checking if the entity is in an animation
            Some(animating_entity) => Some(animating_entity.curr_animation_called && animating_entity.curr_animation.lock().unwrap().get_name() == animation_name),
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
    /// Mainly For Debug Purposes to see the map. Not reccomended to change item.
    pub fn get_mut_map(&mut self) -> &mut HashMap<Entity, AnimatingEntity> {
        &mut self.entities
    }
    /// Mainly For Debug Purposes to see the map. Not reccomended to change item.
    pub fn get_map(&self) -> &HashMap<Entity, AnimatingEntity> {
        &self.entities
    }
}