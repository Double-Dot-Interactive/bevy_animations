use bevy::{prelude::*};

use crate::*;

/// A plugin for Bevy that adds support for animations.
/// You Can Add This Plugin To Your Bevy Game Like This
/// ```
/// use bevy_animations::AnimationsPlugin;
/// use bevy::prelude::*;
/// 
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugins(AnimationsPlugin {
///             pixels_per_meter: 20. // your desired pixels_per_meter
///         })
///         .run()
/// }
/// ```
/// Note that the `pixels_per_meter` field will be used for your [`TransformAnimation`](crate::animations::TimedAnimation)
#[derive(Debug, Default)]
pub struct AnimationsPlugin {
    /// The number of pixels per meter.
    pub pixels_per_meter: f32,
}

impl Plugin for AnimationsPlugin {
    /// Builds the plugin.
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AnimationsConfig {
                pixels_per_meter: self.pixels_per_meter,
            })
            .add_event::<AnimationEvent>()
            .add_event::<ResetAnimationEvent>()
            .insert_resource(Animations::default())
            .insert_resource(EntitesToRemove::default())
            .add_system_set(SystemSet::new()
                .with_system(catch_animation_event.label("events").before("remove"))
                .with_system(catch_reset_events.label("reset").after("events"))
                .with_system(remove_entites.label("remove").after("reset"))
            );
    }
}

/// Main System That Checks for Incoming events
/// If any incoming events are found they are checked to make sure they are new and if they are the Handle<TextureAtlas> is changed for the entity
/// After checking the events. All the [`AnimatingEntities`](crate::AnimatingEntities) have their current animations cycled
fn catch_animation_event(
    time: Res<Time>,
    mut query: Query<(
        &mut Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
        &mut Transform,
        &AnimationDirection
    )>,
    mut animations: ResMut<Animations>,
    mut entities_to_remove: ResMut<EntitesToRemove>,
    config: Res<AnimationsConfig>,
    mut animation_events: EventReader<AnimationEvent>
) {
    // our main event loop
    for event in animation_events.iter() {
        // get the animating entity from the entity passed in from our event
        let animating_entity = animations.entities.get_mut(&event.1).expect(format!("Entity Not Found in Map For {} animation make sure your adding every necessary component to the entity i.e `AnimationDirection`", event.0).as_str());
        // query the texture the sprite and the current direction of the entity
        let (mut texture_atlas, mut sprite, _, direction) = match query.get_mut(animating_entity.entity) {
            Ok(handle) => handle,
            Err(_) => {
                // if we didn't find the entity from the query it doesn't exist anymore and should be removed via the remove_entites system
                entities_to_remove.0.push(event.1);
                continue;
            }
        };
        // if incoming event is new 
        if animating_entity.curr_animation.lock().unwrap().get_name() != event.0 {
            // get the Arc pointer for the animation
            let new_animation_arc = animating_entity.animations.get(event.0).expect(format!("No Animation Found For `{}` make sure the name matches your configuration", event.0).as_str());
            // unlock the animation if we don't do this we will hit a deadlock whenever we try to unlock the Arc<Mutex<>>
            let mut new_animation = new_animation_arc.lock().unwrap();
            let mut blocking = false;
            let mut new_priority = 0;
            let mut sprite_index = 0;
            // get the temp variables above so we don't need to twice
            if let Some(new_timed_animation) = new_animation.timed_animation() {
                blocking = new_timed_animation.blocking;
                new_priority = new_timed_animation.blocking_priority;
                sprite_index = new_timed_animation.sprite_index(&animating_entity.last_valid_direction);
            }
            // if the new animation isn't a timed one we don't care about blocking or priority
            else if let Some(new_transform_animation) = new_animation.transform_animation() {
                sprite_index = new_transform_animation.sprite_index(&animating_entity.last_valid_direction);
            }
            // if we are in a blocking animation we don't want to changed our animation state
            if animating_entity.in_blocking_animation {
                // check the new animations priority from the current one
                if let Some(curr_timed_animation) = animating_entity.curr_animation.lock().unwrap().timed_animation() {
                    if curr_timed_animation.blocking_priority > new_priority {
                        continue;
                    }
                }
                else {
                    continue;
                }
            }
            animating_entity.curr_animation.lock().unwrap().reset_animation();
            animating_entity.curr_animation = new_animation_arc.clone();
            animating_entity.in_blocking_animation = blocking;
            
            sprite.index = sprite_index;
            *texture_atlas = new_animation.get_atlas();
        }
        animating_entity.curr_animation_called = true;
        // if our direction is changed we can set the current direction
        if animating_entity.curr_direction != *direction {
            animating_entity.curr_direction = direction.clone();
            // we don't want to set a Still direction to our last valid direction field because our animations won't be right
            if *direction != AnimationDirection::Still {
                animating_entity.last_valid_direction = direction.clone();
            }
        }
    } 
    // our main animating loop
    for (entity, animation_entity) in animations.entities.iter_mut() {
        // query the sprite and transform from the entity
        let (_, sprite, transform, _) = match query.get_mut(*entity) {
            Ok(query) => query,
            Err(_) => {
                // if we didn't find the entity in the query we should remove it as it doesn't exist anymore
                entities_to_remove.0.push(*entity);
                continue;
            }
        };
        // unlock the current animation once so we don't hit a deadlock
        let mut curr_animation = animation_entity.curr_animation.lock().unwrap();
        let name = curr_animation.get_name();

        // if the current animation wasn't started via an `AnimationEvent`
        if !animation_entity.curr_animation_called {
            continue;
        }

        // if the current animation is transform based we should cycle it
        if let Some(transform_animation) = curr_animation.transform_animation() {
            if animation_entity.in_blocking_animation {
                continue;
            }
            if let None = transform_animation.cycle_animation(sprite, &animation_entity.last_valid_direction, transform, config.pixels_per_meter, name) {
                animation_entity.curr_animation_called = false;
            }
        } 
        // if our current animation is timed based we should cycle it
        else if let Some(timed_animation) = curr_animation.timed_animation() {
            if let None = timed_animation.cycle_animation(sprite, &animation_entity.last_valid_direction, time.delta(), name) {
                animation_entity.in_blocking_animation = false;
                animation_entity.curr_animation_called = false;
            }
        }
        // if the current animation is linear time based we should cycle it
        else if let Some(linear_timed_animation) = curr_animation.linear_timed_animation() {
            if let None = linear_timed_animation.cycle_animation(sprite, time.delta(), name) {
                animation_entity.in_blocking_animation = false;
                animation_entity.curr_animation_called = false;
            }
        }
        // if we get here something bad happened it will most likely never hit as the typing is pretty strong
        else {
            panic!("Something Went Terribly Wrong Animating {} Check Your Configurations", curr_animation.get_name());
        } 
    }
}

fn catch_reset_events(
    mut query: Query<(
        &mut TextureAtlasSprite,
        &AnimationDirection
    )>,
    mut animations: ResMut<Animations>,
    mut entities_to_remove: ResMut<EntitesToRemove>,
    mut animation_events: EventReader<ResetAnimationEvent>
) {
    for event in animation_events.iter() {
        // if the entity wasn't found in the query we want to remove it from our data structure
        let (mut sprite, direction) = match query.get_mut(event.0) {
            Ok(q) => q,
            Err(_) => {
                entities_to_remove.0.push(event.0);
                continue;
            }
        };
        let mut curr_animation = animations.entities
            .get_mut(&event.0)
            .expect("Entity Not Found from `ResetAnimationEvent`")
            .curr_animation
            .lock()
            .unwrap()
        ;
        // try and get the current animation
        // if it is time based
        if let Some(timed_animation) = curr_animation.timed_animation() {
            timed_animation.reset_animation(Some(sprite), Some(direction));
        }
        // if it is transform based
        else if let Some(transform_animation) = curr_animation.transform_animation() {
            transform_animation.reset_animation(Some(sprite), Some(direction));
        }
        // if it is lineartime based
        else if let Some(linear_timed_animation) = curr_animation.linear_timed_animation() {
            linear_timed_animation.reset_animation(Some(sprite));
        }
        else {
            panic!("Something went terribly wrong getting the current animation");
        }
    }
}

/// This system is for any cleanup of despawned entities
fn remove_entites(
    mut animations: ResMut<Animations>,
    mut entities_to_remove: ResMut<EntitesToRemove>
) {
    for entity in entities_to_remove.0.iter() {
        println!("removing {:?}", entity);
        animations.entities.remove(&entity);
    }
    entities_to_remove.0.clear();
}