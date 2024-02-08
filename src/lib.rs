use std::{collections::HashMap, time::Duration, sync::{Arc, Mutex}};

use bevy::prelude::*;

mod animations;
mod plugins;
mod types;

pub use animations::*;
pub use plugins::*;
pub use types::*;

pub mod prelude {
    pub use crate::animations::{TimedAnimation, TransformAnimation, LinearTransformAnimation, LinearTimedAnimation, SingleFrameAnimation};
    pub use crate::plugins::AnimationsPlugin;
    pub use crate::types::{
        AnimationDirection,
        AnimationEvent,
        FXAnimationEvent,
        ResetAnimationEvent,
        Animation,
        FXBasedDirection,
        AnimationName,
        AnimationDirectionIndexes,
        FlipBasedDirection,
        IndexBasedDirection,
        YIndex,
        NewAnimation,
        AnimationType,
        Animator
    };
    pub use crate::{AnimationsConfig, Animations};
}

#[derive(Component, Clone)]
struct FXAnimation;

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
    animations: HashMap<AnimationName, Animation>,
    fx_animations: HashMap<AnimationName, Animation>
}

impl Animations {
    /// Adds a new animation to the animation pool. 
    /// 
    /// Can optionally add an entity to the animation.
    /// 
    /// # Panics
    /// when an animation with the same name already exists
    pub fn insert_animation(&mut self, animation: NewAnimation, entity: Option<Entity>) -> &mut Self {
        let name = animation.animation.get_name();
        let animation = Animation {
            handle: animation.handle,
            animation: Arc::new(Mutex::new(animation.animation))
        };
        let new_animation = Arc::clone(&animation.animation);
        let animation = animation;
        if let None = self.animations.get_mut(&name) {
            self.animations.insert(name, animation);
        }
        if let Some(entity) = entity {
            if let Some(animating_entity) = self.entities.get_mut(&entity) {
                animating_entity.animations.insert(name, Arc::clone(&new_animation));
            }
            else {
                let mut map = HashMap::new();
                map.insert(name, Arc::clone(&new_animation));
                self.entities.insert(entity, AnimatingEntity { 
                    entity, 
                    animations: map, 
                    curr_animation: new_animation, 
                    curr_direction: AnimationDirection::Still,
                    last_valid_direction: AnimationDirection::default(),
                    in_blocking_animation: false,
                    curr_animation_called: false,
                    fx_animation: false
                });
            }
        }
        return self;
    }
    /// Add an [Entity] to the pool without a current animation specified 
    /// 
    /// returns [Result<(), String>] an [Err(String)] if the entity already exists in the pool
    pub fn insert_entity(&mut self, entity: Entity) -> Result<(), String> {
        if let Some(_) = self.entities.get(&entity) {
            return Err(format!("Entity {:?} already exists in `Animations`", entity));
        }
        self.entities.insert(entity, AnimatingEntity {
            entity,
            animations: HashMap::new(),
            curr_animation: Arc::new(Mutex::new(AnimationType::default())),
            curr_direction: AnimationDirection::Still,
            last_valid_direction: AnimationDirection::default(),
            in_blocking_animation: false,
            curr_animation_called: false,
            fx_animation: false
        });
        Ok(())
    }
    /// Add an animation to an [Entity] 
    /// 
    /// returns [Result<(), String>] an [Err(String)] if the animation already exists on the entity specified 
    pub fn add_animation_to_entity(&mut self, animation_name: AnimationName, entity: Entity) -> Result<(), String> {
        if let Some(animation) = self.animations.get_mut(&animation_name) {
            let ac_animation = Arc::new(Mutex::new(animation.animation.lock().unwrap().clone()));
            if let Some(entity) = self.entities.get_mut(&entity) {
                if let Some(_) = entity.animations.get(&animation_name) {
                    return Err(format!("Animation {:?} already exists on entity {:?}", animation_name, entity.entity));
                }
                if entity.curr_animation.lock().unwrap().is_none() {
                    entity.curr_animation = Arc::clone(&ac_animation);
                }
                entity.animations.insert(animation_name, ac_animation);
            }
            else {
                let mut map = HashMap::new();
                map.insert(animation_name, Arc::clone(&ac_animation));
                self.entities.insert(entity, AnimatingEntity { 
                    entity, 
                    animations: map, 
                    curr_animation: ac_animation, 
                    curr_direction: AnimationDirection::Still,
                    last_valid_direction: AnimationDirection::default(),
                    in_blocking_animation: false,
                    curr_animation_called: false,
                    fx_animation: false
                });
            }
        }
        return Ok(());
    }
    /// gets a clone of the `TextureAtlas` handle for the animation specified 
    /// 
    /// returns [None] if the animation does not exist
    pub fn get_handle(&self, animation_name: AnimationName) -> Option<Handle<TextureAtlas>> {
        if let Some(animation) = self.animations.get(&animation_name) {
            return Some(animation.handle.clone());
        }
        None
    }
    /// gets a clone of the `TextureAtlas` handle for the fx_animation specified 
    /// 
    /// returns [None] if the animation does not exist
    pub fn get_fx_handle(&self, animation_name: AnimationName) -> Option<Handle<TextureAtlas>> {
        if let Some(animation) = self.fx_animations.get(&animation_name) {
            return Some(animation.handle.clone());
        }
        None
    }
    /// gets the animating entity from the entity specified 
    /// 
    /// returns [None] if the entity does not exist in the pool
    pub fn get_entity(&mut self, entity: &Entity) -> Option<&mut AnimatingEntity> {
        if let Some(animating_entity) = self.entities.get_mut(&entity) {
            return Some(animating_entity);
        }
        None
    }
    /// if the animation specified is not animating on the entity specified currently
    /// 
    /// returns [None] if the entity does not exist in the pool
    pub fn new_animation(&self, animation_name: AnimationName, entity: &Entity) -> Option<bool> {
        if let Some(animating_entity) = self.entities.get(entity) {
            if animating_entity.curr_animation.lock().unwrap().get_name() != animation_name {
                return Some(true);
            }
            return Some(false);
        }
        None
    }
    /// if the entity specified exists in the pool
    pub fn has_entity(&self, entity: &Entity) -> bool {
        if let Some(_) = self.entities.get(entity) {
            return true;
        }
        false
    }
    /// insert an FX animation this. In order to start the FX animation send it through an [EventWriter(FXAnimationEvent(AnimationName))]
    pub fn insert_fx_animation(&mut self, value: NewAnimation) -> &mut Self {
        let key = value.animation.get_name();
        if let Some(_) = self.fx_animations.get(key) {
            return self;
        }
        else {
            let animation = Animation {
                handle: value.handle,
                animation: Arc::new(Mutex::new(value.animation))
            };
            self.fx_animations.insert(key, animation);
            return self;
        }
    }

    /// Add an FX animation to a new [AnimatingEntity]. This will start the animation specified.
    /// 
    /// # Note
    /// 
    /// This method is used for the backend and shouldn't be called directly. If you need to start an fx animation use [FXAnimationEvent] instead.
    pub fn start_fx_animation(&mut self, key: Entity, animation: AnimationName, pos: Vec3) -> Result<SpriteSheetBundle, ()> {
        let name = animation;
        let Some(animation) = self.fx_animations.get(animation) else { return Err(()) };
        let mut animation = animation.animation.lock().unwrap().clone();

        // grab the atlas from the animations and spawn a new entity with the atlas at the specified pos
        let atlas = self.get_fx_handle(name).expect(format!("There was a problem starting FX animation {}", name).as_str());
        let sprite_index = if let Some(timed_animation) = animation.timed_animation() {
            timed_animation.sprite_index(&AnimationDirection::default())
        }
        else if let Some(transform_animation) = animation.transform_animation() {
            transform_animation.sprite_index(&AnimationDirection::default())
        }
        else if let Some(linear_timed_animation) = animation.linear_timed_animation() {
            linear_timed_animation.sprite_index(&AnimationDirection::default())
        }
        else if let Some(linear_transform_animation) = animation.linear_transform_animation() {
            linear_transform_animation.sprite_index(&AnimationDirection::default())
        }
        else if let Some(single_frame_animation) = animation.single_frame_animation() {
            single_frame_animation.sprite_index(&AnimationDirection::default())
        }
        else {
            panic!("Something Went Terribly Wrong Starting FX Animation");
        };
        // add the animation and entity to a new AnimatingEntity to be animated
        self.entities.insert(key, AnimatingEntity {
            entity: key,
            in_blocking_animation: false,
            animations: HashMap::new(),
            curr_animation: Arc::new(Mutex::new(animation)),
            curr_direction: AnimationDirection::default(),
            last_valid_direction: AnimationDirection::default(),
            curr_animation_called: true,
            fx_animation: true,
        });
        return Ok(SpriteSheetBundle {
            texture_atlas: atlas,
            sprite: TextureAtlasSprite::new(sprite_index),
            transform: Transform::from_translation(pos),
            ..Default::default()
        });
    }
    /// if the animation exists in the pool
    pub fn has_animation(&self, animation_name: AnimationName) -> bool {
        if let Some(_) = self.animations.get(animation_name) {
            return true;
        }
        false
    }
    /// returns [Some(())] if the animation already exists on the entity specified
    /// 
    /// returns [None] if the entity was not found in the pool
    /// 
    /// returns [None] if the animation was not found on the entity specified
    pub fn entity_has_animation(&self, animation_name: &AnimationName, entity: Entity) -> Option<()> {
        if let Some(animating_entity) = self.entities.get(&entity) {
            if let Some(_) = animating_entity.animations.get(animation_name) {
                return Some(());
            }
            return None;
        }
        None
    }
    /// returns [Some(bool)] if the entity exists and [Some(true)] if the entity is in a blocking animation
    /// 
    /// returns [None] if the entity was not found 
    /// 
    /// usefull to determine for example whether or not to move an entity
    pub fn in_blocking_animation(&self, entity: Entity) -> Option<bool> {
        match self.entities.get(&entity) {
            Some(animating_entity) => Some(animating_entity.in_blocking_animation),
            None => None
        }
    }
    /// returns [Some(bool)] if the entity exists and [Some(true)] if the entity is in an animation
    /// 
    /// returns [None] if the entity was not found 
    /// 
    /// useful for determining for example whether or not to initate another animation
    pub fn in_animation(&self, entity: Entity) -> Option<bool> {
        match self.entities.get(&entity) {
            // this is functionally the same as checking if the entity is in an animation
            Some(animating_entity) => Some(animating_entity.curr_animation_called),
            None => None
        }
    }
    /// returns [Some(bool)] if the entity exists and [Some(true)] if the entity is in the animation specified
    /// 
    /// returns [None] if the entity was not found 
    /// 
    /// useful for determining for example whether or not to initate another animation
    pub fn doing_animation(&self, entity: Entity, animation_name: AnimationName) -> Option<bool> {
        match self.entities.get(&entity) {
            // this is functionally the same as checking if the entity is in an animation
            Some(animating_entity) => Some(animating_entity.curr_animation_called && animating_entity.curr_animation.lock().unwrap().get_name() == animation_name),
            None => None
        }
    }
    /// Returns `true` if the [Entity] exists in the [Animations] map
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