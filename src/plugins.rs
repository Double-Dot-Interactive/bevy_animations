use core::panic;

use bevy::prelude::*;

use crate::*;

/// A plugin for Bevy that adds support for animations.
///
/// ```
/// use bevy_animations::AnimationsPlugin;
/// use bevy::prelude::*;
///
/// App::new()
///     .add_plugins(DefaultPlugins)
///     .add_plugins(AnimationsPlugin {
///         pixels_per_meter: 20. // your desired pixels_per_meter
///     })
///     .run()
/// ```
/// Note that the `pixels_per_meter` field will be used for your [`TransformAnimation`](crate::animations::TimedAnimation)
#[derive(Debug, Default)]
pub struct AnimationsPlugin {
    /// The number of pixels per meter
    pub pixels_per_meter: f32,
}

impl Plugin for AnimationsPlugin {
    /// Builds the plugin
    fn build(&self, app: &mut App) {
        app.insert_resource(AnimationsConfig {
            pixels_per_meter: self.pixels_per_meter,
        })
        .add_event::<AnimationEvent>()
        .add_event::<ResetAnimationEvent>()
        .add_event::<FXAnimationEvent>()
        .insert_resource(Animations::default())
        .insert_resource(EntitesToRemove::default())
        .add_systems(
            Update,
            (
                catch_fx_animation_events,
                catch_animation_events,
                catch_reset_events,
                remove_entites,
            )
                .chain(),
        );
    }
}

/// Main System That Checks for Incoming events
/// If any incoming events are found they are checked to make sure they are new and if they are the Handle<TextureAtlas> is changed for the entity
/// After checking the events. All the [`AnimatingEntities`](crate::AnimatingEntities) have their current animations cycled
fn catch_animation_events(
    time: Res<Time>,
    mut query: Query<(
        &mut TextureAtlas,
        &mut Handle<Image>,
        &mut Sprite,
        &mut Transform,
        &Animator,
    )>,
    mut animations: ResMut<Animations>,
    mut entities_to_remove: ResMut<EntitesToRemove>,
    config: Res<AnimationsConfig>,
    mut animation_events: EventReader<AnimationEvent>,
    mut commands: Commands,
) {
    // Our main event loop
    for event in animation_events.read() {
        if !animations.has_entity(&event.1) {
            panic!("Entity Not Found in Map For {} animation make sure your adding every necessary component to the entity i.e `Animator`", event.0);
        }
        if !animations.has_animation(event.0) {
            panic!("Animation {} not found", event.0);
        }
        // Query the texture the sprite and the current direction of the entity
        let (mut texture_atlas, mut texture, _, _, animator) =
            match query.get_mut(event.1) {
                Ok(handle) => handle,
                Err(_) => {
                    // If we didn't find the entity from the query it doesn't exist anymore and should be removed via the remove_entites system
                    entities_to_remove.0.push(event.1);
                    continue;
                }
            };
        let direction = animator.get_direction();
        // If incoming event is new
        if animations
            .is_new_animation(event.0, &event.1)
            .unwrap_or_else(|| {
                panic!(
                    "Something Went Terribly Wrong Getting New Animation For {}",
                    event.0
                )
            })
        {
            let new_animation_handles = animations.get_handles(event.0).unwrap_or_else(|| {
                panic!(
                    "Something Went Terribly Wrong Getting Animation For {}",
                    event.0
                )
            });
            // Get the animating entity from the entity passed in from our event
            let animating_entity = animations.get_entity(&event.1).unwrap_or_else(|| panic!("Entity Not Found in Map For {} animation make sure your adding every necessary component to the entity i.e `AnimationDirection`", event.0));
            // info!("{} {:?} {:?}", event.0, event.1, new_animation_handle.id());

            // Get the Arc pointer for the animation
            let Some(new_animation_arc) = animating_entity.animations.get(event.0) else {
                panic!("Animation `{}` not found for {:?} make sure the name matches your configuration", event.0, animating_entity.entity);
            };

            // Unlock the animation if we don't do this we will hit a deadlock whenever we try to unlock the Arc<Mutex<>>
            let mut new_animation = new_animation_arc.lock().unwrap();
            let mut blocking = false;
            let mut new_priority = 0;
            let mut sprite_index = 0;

            // Get the temp variables above so we don't need to twice
            if let Some(new_timed_animation) = new_animation.timed_animation() {
                blocking = new_timed_animation.blocking;
                new_priority = new_timed_animation.blocking_priority;
                sprite_index =
                    new_timed_animation.sprite_index(&animating_entity.last_valid_direction);
            } else if let Some(new_singe_frame_animation) = new_animation.single_frame_animation() {
                blocking = new_singe_frame_animation.blocking;
                new_priority = new_singe_frame_animation.blocking_priority;
                sprite_index =
                    new_singe_frame_animation.sprite_index(&animating_entity.last_valid_direction);
            }
            // If the new animation isn't a timed or single_frame one we don't care about blocking or priority
            else if let Some(new_transform_animation) = new_animation.transform_animation() {
                sprite_index =
                    new_transform_animation.sprite_index(&animating_entity.last_valid_direction);
            }

            // If we are in a blocking animation we don't want to changed our animation state
            // info!("{}", animating_entity.in_blocking_animation);
            if animating_entity.in_blocking_animation {
                // Check the new animations priority from the current one
                let mut curr_animation = animating_entity.curr_animation.lock().unwrap();
                if let Some(curr_timed_animation) = curr_animation.timed_animation() {
                    if curr_timed_animation.blocking_priority > new_priority {
                        // info!("blocking animation");
                        continue;
                    }
                } else if let Some(curr_single_frame_animation) =
                    curr_animation.single_frame_animation()
                {
                    // info!("{} {} {}", curr_single_frame_animation.blocking_priority, new_priority, curr_single_frame_animation.blocking_finished);
                    if curr_single_frame_animation.blocking_priority > new_priority
                        && !curr_single_frame_animation.blocking_finished
                    {
                        // info!("blocking animation");
                        continue;
                    }
                } else {
                    // info!("blocking animation");
                    continue;
                }
            }

            animating_entity
                .curr_animation
                .lock()
                .unwrap()
                .reset_animation();
            animating_entity.curr_animation = new_animation_arc.clone();
            animating_entity.in_blocking_animation = blocking;

            texture_atlas.index = sprite_index;
            texture_atlas.layout = new_animation_handles.layout();
            *texture = new_animation_handles.image();
        }

        let animating_entity = animations.get_entity(&event.1).unwrap_or_else(|| panic!("Entity Not Found in Map For {} animation make sure your adding every necessary component to the entity i.e `AnimationDirection`", event.0));
        animating_entity.curr_animation_called = true;

        // If our direction is changed we can set the current direction
        if animating_entity.curr_direction != *direction {
            animating_entity.curr_direction = direction.clone();
            // We don't want to set a Still direction to our last valid direction field because our animations won't be right
            if *direction != AnimationDirection::Still {
                animating_entity.last_valid_direction = direction.clone();
            }
        }
    }

    // Our main animating loop
    for (entity, animation_entity) in animations.entities.iter_mut() {
        // Query the sprite and transform from the entity
        let (texture_atlas, _, sprite, transform, _) =
            match query.get_mut(*entity) {
                Ok(query) => query,
                Err(_) => {
                    // if we didn't find the entity in the query we should remove it as it doesn't exist anymore
                    if !animation_entity.fx_animation {
                        entities_to_remove.0.push(*entity);
                    }
                    continue;
                }
            };
        // unlock the current animation once so we don't hit a deadlock
        let mut curr_animation = animation_entity.curr_animation.lock().unwrap();

        // if the current animation wasn't started via an `AnimationEvent`
        if !animation_entity.curr_animation_called {
            continue;
        }

        // info!("{}", curr_animation.get_name());

        // if the current animation is transform based we should cycle it
        if let Some(transform_animation) = curr_animation.transform_animation() {
            if animation_entity.in_blocking_animation {
                continue;
            }
            if transform_animation.cycle_animation(
                sprite,
                texture_atlas,
                &animation_entity.last_valid_direction,
                transform,
                config.pixels_per_meter,
            ).is_none() {
                if animation_entity.fx_animation {
                    entities_to_remove.0.push(*entity);
                    commands.entity(*entity).despawn_recursive();
                }
                animation_entity.curr_animation_called = false;
            }
        }
        // if our current animation is timed based we should cycle it
        else if let Some(timed_animation) = curr_animation.timed_animation() {
            if timed_animation.cycle_animation(
                sprite,
                texture_atlas,
                &animation_entity.last_valid_direction,
                time.delta(),
            ).is_none() {
                if animation_entity.fx_animation {
                    entities_to_remove.0.push(*entity);
                    commands.entity(*entity).despawn_recursive();
                }
                animation_entity.in_blocking_animation = false;
                animation_entity.curr_animation_called = false;
            }
        }
        // if the current animation is linear time based we should cycle it
        else if let Some(linear_timed_animation) = curr_animation.linear_timed_animation() {
            if linear_timed_animation.cycle_animation(texture_atlas, time.delta()).is_none() {
                if animation_entity.fx_animation {
                    entities_to_remove.0.push(*entity);
                    commands.entity(*entity).despawn_recursive();
                }
                animation_entity.in_blocking_animation = false;
                animation_entity.curr_animation_called = false;
            }
        }
        // if the current animation is linear transform based we should cycle it
        else if let Some(linear_transform_animation) = curr_animation.linear_transform_animation()
        {
            if linear_transform_animation.cycle_animation(
                texture_atlas,
                transform,
                config.pixels_per_meter,
            ).is_none() {
                if animation_entity.fx_animation {
                    entities_to_remove.0.push(*entity);
                    commands.entity(*entity).despawn_recursive();
                }
                animation_entity.curr_animation_called = false;
            }
        }
        // if the current animation is a single frame animation
        else if let Some(single_frame_animation) = curr_animation.single_frame_animation() {
            single_frame_animation.cycle_animation(
                sprite,
                texture_atlas,
                &animation_entity.last_valid_direction,
                time.delta(),
            )
        }
        // if we get here something bad happened it will most likely never hit as the typing is pretty strong
        else {
            panic!(
                "Something Went Terribly Wrong Animating {} Check Your Configurations",
                curr_animation.get_name()
            );
        }
    }
}

fn catch_reset_events(
    mut query: Query<(&mut Sprite, &mut TextureAtlas, &Animator)>,
    mut animations: ResMut<Animations>,
    mut entities_to_remove: ResMut<EntitesToRemove>,
    mut animation_events: EventReader<ResetAnimationEvent>,
) {
    for event in animation_events.read() {
        // If the entity wasn't found in the query we want to remove it from our data structure
        let (sprite, texture_atlas, animator) = match query.get_mut(event.0) {
            Ok(q) => q,
            Err(_) => {
                entities_to_remove.0.push(event.0);
                continue;
            }
        };
        let direction = animator.get_direction();
        let mut curr_animation = animations
            .entities
            .get_mut(&event.0)
            .expect("Entity Not Found from `ResetAnimationEvent`")
            .curr_animation
            .lock()
            .unwrap();
        // Try and get the current animation
        // If it is time based
        if let Some(timed_animation) = curr_animation.timed_animation() {
            timed_animation.reset_animation(Some(sprite), Some(texture_atlas), Some(direction));
        }
        // If it is transform based
        else if let Some(transform_animation) = curr_animation.transform_animation() {
            transform_animation.reset_animation(Some(sprite), Some(texture_atlas), Some(direction));
        }
        // If it is linear time based
        else if let Some(linear_timed_animation) = curr_animation.linear_timed_animation() {
            linear_timed_animation.reset_animation(Some(texture_atlas));
        }
        // If it is linear transform based
        else if let Some(linear_transform_animation) = curr_animation.linear_transform_animation()
        {
            linear_transform_animation.reset_animation(Some(texture_atlas));
        }
        // If it is single frame based
        else if let Some(single_frame_animation) = curr_animation.single_frame_animation() {
            single_frame_animation.reset_animation(Some(sprite), Some(direction));
        } else {
            panic!("Something went terribly wrong resetting the current animation");
        }
    }
}

fn catch_fx_animation_events(
    mut event_reader: EventReader<FXAnimationEvent>,
    mut commands: Commands,
    mut animations: ResMut<Animations>,
) {
    for event in event_reader.read() {
        let entity = commands.spawn(Animator::default()).id();
        let Ok(sprite_sheet_bundle) = animations.start_fx_animation(entity, event.0, event.1)
        else {
            warn!("There was a problem spawning your FXAnimation {}", event.0);
            continue;
        };

        commands
            .entity(entity)
            .insert(sprite_sheet_bundle)
            .insert(FXAnimation)
            .insert(Name::new("FX Animation"));
    }
}

/// This system is for any cleanup of despawned entities
fn remove_entites(
    mut animations: ResMut<Animations>,
    mut entities_to_remove: ResMut<EntitesToRemove>,
) {
    for entity in entities_to_remove.0.iter() {
        animations.entities.remove(entity);
    }
    entities_to_remove.0.clear();
}
