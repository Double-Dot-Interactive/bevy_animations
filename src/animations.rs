use crate::*;

/// This is Primarily for This Is Primarily For Animations on players or NPCs, for example shooting a bow or reloading a gun
///
/// # Example
/// ```rust
///        fn init_animation(
///            mut animations: ResMut<Animations>,
///            mut commands: Commands,
///            asset_server: ResMut<AssetServer>,
///            mut texture_atlases: ResMut<Assets<TextureAtlas>>,
///            ) {
///            let asset = asset_server.load("path/to/your/sprite_sheet");
///
///            let texture_atlas = TextureAtlas::from_grid(asset, Vec2::new(16.0, 16.0), 10, 1, None, None);
///
///            let texture_atlas_handle = texture_atlases.add(texture_atlas);
///
///            let entity = commands.spawn(AnimationDirection::default())
///                .insert(SpriteSheetBundle {
///                    texture_atlas: texture_atlas_handle.clone(),
///                    ..Default::default()
///                }).id();
///            animations.insert_animation(
///                entity, // the entity is needed to determine which `Handle<TextureAtlas>` is being manipulated
///                AnimationType::Timed(
///                    TimedAnimation::new(
///                        /* animation_frames */ vec![0, 1, 2, 3], // the x index for your frames to cycle through
///                        /* frame_timings_in_secs */ vec![0.001, 0.5, 0.5, 0.5] // Note that the the first timing is set to 0.001 so the animation starts immediately. If this value doesn't suit your needs, you can change it to another parameter.
///                        /* handle */ texture_atlas_handle, // your sprite sheet
///                        /* frame */ Vec2::new(4., 4.), // the length and height of your sprite sheet
///                        /* direction_indexes */ AnimationDirectionIndexes::IndexBased(IndexBasedDirection {
///                            left: 1,
///                            right: 1,
///                            up: 1,
///                            down: 1
///                        }), // the indexes to determine the correct sprite for the direction
///                        /* repeating */ true, // if the animation is repeating or not
///                        /* blocking */ false, // if the animation should block others
///                        /* blocking_priority */ 1 // the blocking_priority hierarchy assignment to determine which animations should block eachother
///                    ),
///                    "player_running" // the name of the animation. will be used when sending an `AnimationEvent`
///                )
///           );
///       }
/// ```
#[derive(Clone, Debug, Default)]
pub struct TimedAnimation {
    animation_tick: usize,
    previous_dir_index: usize,
    pub frame_timings_in_secs: Vec<f32>,
    pub blocking: bool,
    pub blocking_priority: i32,
    pub animation_frames: Vec<usize>,
    pub frame: Vec2,
    pub direction_indexes: AnimationDirectionIndexes,
    pub repeating: bool,
    pub animation_timer: AnimationTimer,
}

impl TimedAnimation {
    pub fn new(
        animation_frames: Vec<usize>,
        frame_timings_in_secs: Vec<f32>,
        frame: Vec2,
        direction_indexes: AnimationDirectionIndexes,
        repeating: bool,
        blocking: bool,
        blocking_priority: i32,
    ) -> Self {
        let timer_dur = *frame_timings_in_secs.get(0).unwrap();
        Self {
            animation_tick: 1,
            animation_frames,
            frame_timings_in_secs,
            frame,
            direction_indexes,
            repeating,
            previous_dir_index: 1,
            animation_timer: AnimationTimer(Timer::from_seconds(timer_dur, TimerMode::Repeating)),
            blocking,
            blocking_priority,
        }
    }
    fn get_x_index(&mut self) -> Option<usize> {
        let index = self.animation_frames.get(self.animation_tick - 1);
        if self.repeating {
            match index {
                Some(index) => {
                    return Some(*index)
                },
                None => {
                    self.animation_tick = 1;
                    let index = self.animation_frames.get(self.animation_tick - 1)
                    .expect(format!("There was A Problem Cycling Animation\nThe index is {} but The Frame Length is {}", self.animation_tick, self.animation_frames.len()).as_str());
                    return Some(*index);
                }
            }
        } else {
            match index {
                Some(index) => return Some(*index),
                None => return None,
            };
        }
    }

    fn ready_to_animate(&mut self, delta: Duration) -> bool {
        self.animation_timer.tick(delta);
        if self.animation_timer.finished() {
            return true;
        }
        false
    }

    pub fn sprite_index(&mut self, direction: &AnimationDirection) -> usize {
        let x_index = match self.get_x_index() {
            Some(index) => index,
            None => 0,
        };

        match self.get_y_index(direction) {
            YIndex::Index(y_index) => {
                // (y_index * self.frame.y as usize) - (self.frame.x as usize - x_index)
                y_index * self.frame.x as usize + x_index
            }
            YIndex::Flip(_, y_index) => {
                // (y_index * self.frame.y as usize) - (self.frame.x as usize - x_index)
                y_index * self.frame.x as usize + x_index
            }
        }
    }

    pub fn cycle_animation(
        &mut self,
        mut sprite: Mut<TextureAtlasSprite>,
        direction: &AnimationDirection,
        delta: Duration,
    ) -> Option<()> {
        if self.ready_to_animate(delta) {
            let y_index = match self.get_y_index(direction) {
                YIndex::Index(y_index) => y_index,
                YIndex::Flip(flipped, y_index) => {
                    sprite.flip_x = flipped;
                    y_index
                }
            };
            self.previous_dir_index = y_index;
            let x_index = match self.get_x_index() {
                Some(index) => index,
                None => {
                    self.animation_tick = 1;
                    return None;
                }
            };
            // let index = (y_index * self.frame.y as usize) - (self.frame.x as usize - x_index);
            let index = y_index * self.frame.x as usize + x_index;
            sprite.index = index;
            let timing = *self
                .frame_timings_in_secs
                .get(self.animation_tick - 1)
                .expect("Error With Animation Timing");
            self.animation_timer
                .set_duration(Duration::from_secs_f32(timing));
            self.animation_timer.reset();
            self.animation_tick += 1;
            return Some(());
        }
        Some(())
    }

    pub fn reset_animation(
        &mut self,
        sprite: Option<Mut<TextureAtlasSprite>>,
        direction: Option<&AnimationDirection>,
    ) {
        self.animation_tick = 1;
        let new_dur = Duration::from_secs_f32(
            *self
                .frame_timings_in_secs
                .get(0)
                .expect("Error With Animation Timing"),
        );
        self.animation_timer.set_duration(new_dur);
        self.animation_timer.reset();
        if let (Some(mut sprite), Some(direction)) = (sprite, direction) {
            let x_index = self
                .get_x_index()
                .expect("Something Went Wrong Reseting Animation");
            match self.get_y_index(direction) {
                YIndex::Index(y_index) => {
                    sprite.index = y_index * self.frame.x as usize + x_index;
                }
                YIndex::Flip(flip, y_index) => {
                    sprite.flip_x = flip;
                    sprite.index = y_index * self.frame.x as usize + x_index;
                }
            }
        }
    }

    #[allow(unused)]
    fn get_y_index(&self, direction: &AnimationDirection) -> YIndex {
        match (direction, self.direction_indexes) {
            (AnimationDirection::Left, AnimationDirectionIndexes::IndexBased(index)) => {
                YIndex::Index(index.left)
            }
            (AnimationDirection::Right, AnimationDirectionIndexes::IndexBased(index)) => {
                YIndex::Index(index.right)
            }
            (AnimationDirection::Up, AnimationDirectionIndexes::IndexBased(index)) => {
                YIndex::Index(index.up)
            }
            (AnimationDirection::Down, AnimationDirectionIndexes::IndexBased(index)) => {
                YIndex::Index(index.down)
            }
            (AnimationDirection::Left, AnimationDirectionIndexes::FlipBased(index)) => {
                YIndex::Flip(index.left_direction_is_flipped, index.x_direction_index)
            }
            (AnimationDirection::Right, AnimationDirectionIndexes::FlipBased(index)) => {
                YIndex::Flip(!index.left_direction_is_flipped, index.x_direction_index)
            }
            (AnimationDirection::Still, AnimationDirectionIndexes::FlipBased(index)) => {
                YIndex::Flip(false, index.x_direction_index)
            }
            (_, AnimationDirectionIndexes::FX(fx_based_animation)) => {
                YIndex::Index(fx_based_animation.index)
            }
            (AnimationDirection::Still, _) => {
                YIndex::Index(self.previous_dir_index)
            },
            (_, _) => {
                YIndex::Index(1)
            },
        }
    }

    #[allow(unused)]
    fn is_out_of_bounds(&self, sprite: &Mut<TextureAtlasSprite>, index: usize) -> bool {
        if sprite.field_len() <= index {
            return true;
        }
        false
    }
}

/// This Is Primarily For Animations on players or NPCs, for example walking or running
///
/// # Example
///
/// ```rust
///        fn init_animation(
///            mut animations: ResMut<Animations>,
///            mut commands: Commands,
///            asset_server: ResMut<AssetServer>,
///            mut texture_atlases: ResMut<Assets<TextureAtlas>>,
///            ) {
///            let asset = asset_server.load("path/to/your/sprite_sheet");
///
///            let texture_atlas = TextureAtlas::from_grid(asset, Vec2::new(16.0, 16.0), 10, 1, None, None);
///
///            let texture_atlas_handle = texture_atlases.add(texture_atlas);
///
///            let entity = commands.spawn(AnimationDirection::default())
///                .insert(SpriteSheetBundle {
///                    texture_atlas: texture_atlas_handle.clone(),
///                    ..Default::default()
///                }).id();
///            animations.insert_animation(
///                entity, // the entity is needed to determine which `Handle<TextureAtlas>` is being manipulated
///                AnimationType::Transform(
///                    TransformAnimation::new(
///                        /* animation_frames */ vec![0, 1, 2, 3], // the x index for your frames to cycle through
///                        /* meters_per_frame */ 0.55, // your desired meters per frame
///                        /* handle */ texture_atlas_handle, // your sprite sheet
///                        /* frame */ Vec2::new(4., 4.), // the length and height of your sprite sheet
///                        /* direction_indexes */ AnimationDirectionIndexes::IndexBased(IndexBasedDirection {
///                            left: 1,
///                            right: 1,
///                            up: 1,
///                            down: 1
///                        }), // the indexes to determine the correct sprite for the direction
///                        /* repeating */ true, // if the animation is repeating or not
///                    ),
///                    "player_running" // the name of the animation. will be used when sending an `AnimationEvent`
///                )
///           );
///       }
/// ```
///
#[derive(Debug, Default, Clone)]
pub struct TransformAnimation {
    animation_tick: usize,
    previous_dir_index: usize,
    previous_transform: Transform,
    pub animation_frames: Vec<usize>,
    pub meters_per_frame: f32,
    pub frame: Vec2,
    pub direction_indexes: AnimationDirectionIndexes,
    pub repeating: bool,
}

impl TransformAnimation {
    pub fn new(
        animation_frames: Vec<usize>,
        meters_per_frame: f32,
        frame: Vec2,
        direction_indexes: AnimationDirectionIndexes,
        repeating: bool,
    ) -> Self {
        Self {
            animation_tick: 1,
            previous_dir_index: 1,
            previous_transform: Transform::from_xyz(0., 0., 0.),
            animation_frames,
            meters_per_frame,
            frame,
            direction_indexes,
            repeating,
        }
    }

    fn ready_to_animate(&self, transform: &Mut<Transform>, pixels_per_meter: f32) -> bool {
        let x_diff = (transform.translation.x - self.previous_transform.translation.x).abs();
        let y_diff = (transform.translation.y - self.previous_transform.translation.y).abs();

        let modifier = pixels_per_meter * self.meters_per_frame;

        if x_diff >= modifier || y_diff >= modifier {
            return true;
        }
        false
    }

    pub fn sprite_index(&mut self, direction: &AnimationDirection) -> usize {
        let x_index = match self.get_x_index() {
            Some(index) => index,
            None => 0,
        };
        let y_index = match self.get_y_index(direction) {
            YIndex::Index(y_index) => y_index,
            YIndex::Flip(_, y_index) => y_index,
        };
        // (y_index * self.frame.y as usize) - (self.frame.x as usize - x_index)
        y_index * self.frame.x as usize + x_index
    }

    pub fn cycle_animation(
        &mut self,
        mut sprite: Mut<TextureAtlasSprite>,
        direction: &AnimationDirection,
        transform: Mut<Transform>,
        pixels_per_meter: f32,
        // name: &'static str
    ) -> Option<()> {
        let y_index = match self.get_y_index(direction) {
            YIndex::Index(y_index) => y_index,
            YIndex::Flip(_, y_index) => y_index,
        };
        if self.ready_to_animate(&transform, pixels_per_meter) || y_index != self.previous_dir_index
        {
            self.previous_transform = transform.clone();
            let x_index = match self.get_x_index() {
                Some(index) => index,
                None => return None,
            };

            let y_index = match self.get_y_index(direction) {
                YIndex::Index(y_index) => y_index,
                YIndex::Flip(flipped, y_index) => {
                    sprite.flip_x = flipped;
                    y_index
                }
            };

            self.previous_dir_index = y_index;

            // let index = (y_index * self.frame.y as usize) - (self.frame.x as usize - x_index);
            let index = y_index * self.frame.x as usize + x_index;

            sprite.index = index;

            self.animation_tick += 1;
            return Some(());
        } else if *direction == AnimationDirection::Still {
            let x_index = self.animation_frames.get(0).unwrap();

            let y_index = self.previous_dir_index;

            // sprite.index = (y_index * self.frame.y as usize) - (self.frame.x as usize - x_index);
            sprite.index = y_index * self.frame.x as usize + x_index;
            return Some(());
        }
        Some(())
    }

    fn get_x_index(&mut self) -> Option<usize> {
        let index = self.animation_frames.get(self.animation_tick - 1);
        if self.repeating {
            match index {
                Some(index) => return Some(*index),
                None => {
                    self.animation_tick = 1;
                    let index = self.animation_frames.get(self.animation_tick)
                        .expect(format!("There was A Problem Cycling Animation\nThe index is {} but The Frame Length is {}", self.animation_tick, self.animation_frames.len()).as_str());
                    return Some(*index);
                }
            }
        } else {
            match index {
                Some(index) => return Some(*index),
                None => return None,
            };
        }
    }

    #[allow(unused)]
    fn get_y_index(&self, direction: &AnimationDirection) -> YIndex {
        match (direction, self.direction_indexes) {
            (AnimationDirection::Left, AnimationDirectionIndexes::IndexBased(index)) => {
                YIndex::Index(index.left)
            }
            (AnimationDirection::Right, AnimationDirectionIndexes::IndexBased(index)) => {
                YIndex::Index(index.right)
            }
            (AnimationDirection::Up, AnimationDirectionIndexes::IndexBased(index)) => {
                YIndex::Index(index.up)
            }
            (AnimationDirection::Down, AnimationDirectionIndexes::IndexBased(index)) => {
                YIndex::Index(index.down)
            }
            (AnimationDirection::Left, AnimationDirectionIndexes::FlipBased(index)) => {
                YIndex::Flip(index.left_direction_is_flipped, index.x_direction_index)
            }
            (AnimationDirection::Right, AnimationDirectionIndexes::FlipBased(index)) => {
                YIndex::Flip(!index.left_direction_is_flipped, index.x_direction_index)
            }
            (_, AnimationDirectionIndexes::FX(fx_based_animation)) => {
                YIndex::Index(fx_based_animation.index)
            }
            (AnimationDirection::Still, _) => YIndex::Index(self.previous_dir_index),
            (_, _) => YIndex::Index(1),
        }
    }

    pub fn reset_animation(
        &mut self,
        sprite: Option<Mut<TextureAtlasSprite>>,
        direction: Option<&AnimationDirection>,
    ) {
        self.animation_tick = 1;
        if let (Some(mut sprite), Some(direction)) = (sprite, direction) {
            let x_index = self
                .get_x_index()
                .expect("Something Went Wrong Reseting Animation");
            match self.get_y_index(direction) {
                YIndex::Index(y_index) => {
                    sprite.index = y_index * self.frame.x as usize + x_index;
                }
                YIndex::Flip(flip, y_index) => {
                    sprite.flip_x = flip;
                    sprite.index = y_index * self.frame.x as usize + x_index; 
                }
            }
        }
    }
}

/// This Is Primarily For Animations on objects, for example doors to open and close
///
/// **Note** the sprite sheets for these animations should have 1 column. It's okay if they have more however is't functionally irrelevant
///
/// # Example
/// ```rust
///        fn init_animation(
///            mut animations: ResMut<Animations>,
///            mut commands: Commands,
///            asset_server: ResMut<AssetServer>,
///            mut texture_atlases: ResMut<Assets<TextureAtlas>>,
///            ) {
///            let asset = asset_server.load("path/to/your/sprite_sheet");
///
///            let texture_atlas = TextureAtlas::from_grid(asset, Vec2::new(16.0, 16.0), 10, 1, None, None);
///
///            let texture_atlas_handle = texture_atlases.add(texture_atlas);
///
///            let entity = commands.spawn(AnimationDirection::default())
///                .insert(SpriteSheetBundle {
///                    texture_atlas: texture_atlas_handle.clone(),
///                    ..Default::default()
///                }).id();
///            animations.insert_animation(
///                entity, // the entity is needed to determine which `Handle<TextureAtlas>` is being manipulated
///                AnimationType::LinearTimed(
///                    LinearTimedAnimation::new(
///                        /* animation_frames */ vec![0, 1, 2, 3], // the x index for your frames to cycle through
///                        /* frame_timings_in_secs */ vec![0.001, 0.5, 0.5, 0.5] // Note that the the first timing is set to 0.001 so the animation starts immediately. If this value doesn't suit your needs, you can change it to another parameter.
///                        /* handle */ texture_atlas_handle, // your sprite sheet
///                        /* frame */ Vec2::new(4., 4.), // the length and height of your sprite sheet
///                        /* repeating */ true, // if the animation is repeating or not
///                    ),
///                    "player_running" // the name of the animation. will be used when sending an `AnimationEvent`
///                )
///           );
///       }
/// ```
#[derive(Debug, Default, Clone)]
pub struct LinearTimedAnimation {
    animation_tick: usize,
    animation_timer: AnimationTimer,
    pub frame_timings_in_secs: Vec<f32>,
    pub animation_frames: Vec<usize>,
    pub repeating: bool,
}

impl LinearTimedAnimation {
    pub fn new(
        animation_frames: Vec<usize>,
        frame_timings_in_secs: Vec<f32>,
        repeating: bool,
    ) -> Self {
        Self {
            animation_tick: 1,
            animation_timer: AnimationTimer(Timer::from_seconds(
                *frame_timings_in_secs
                    .get(0)
                    .expect("Something went wrong retrieving timings"),
                TimerMode::Repeating,
            )),
            animation_frames,
            frame_timings_in_secs,
            repeating,
        }
    }

    fn get_x_index(&mut self) -> Option<usize> {
        match self.animation_frames.get(self.animation_tick) {
            Some(index) => Some(*index),
            None => {
                self.animation_tick = 1;
                if self.repeating {
                    return Some(0)
                }
                None
            }
        }
    }

    pub fn sprite_index(&mut self, _direction: &AnimationDirection) -> usize {
        if let Some(index) = self.get_x_index() { index } else { 0 }
    }

    pub fn cycle_animation(
        &mut self,
        mut sprite: Mut<TextureAtlasSprite>,
        delta: Duration,
    ) -> Option<()> {
        self.animation_timer.tick(delta);
        if self.animation_timer.finished() {
            let x_index = match self.get_x_index() {
                Some(index) => index,
                None => return None,
            };

            let new_dur = Duration::from_secs_f32(
                *self
                    .frame_timings_in_secs
                    .get(self.animation_tick)
                    .expect("There Was A Problem Getting New Timing Check Your Timing Configs"),
            );
            self.animation_timer.set_duration(new_dur);
            self.animation_timer.reset();

            sprite.index = x_index;

            self.animation_tick += 1;

            return Some(());
        }
        Some(())
    }

    #[allow(unused)]
    pub fn reset_animation(&mut self, mut sprite: Option<Mut<TextureAtlasSprite>>) {
        self.animation_tick = 1;
        let new_dur = Duration::from_secs_f32(
            *self
                .frame_timings_in_secs
                .get(0)
                .expect("Error With Animation Timing"),
        );
        self.animation_timer.set_duration(new_dur);
        self.animation_timer.reset();
        if let Some(mut sprite) = sprite {
            let x_index = self
                .get_x_index()
                .expect("Something Went Wrong Reseting Animation");
            sprite.index = x_index
        }
    }
}

/// This Is Primarily For Animations on objects, for example a projectile
///
/// # Example
///
/// ```rust
///        fn init_animation(
///            mut animations: ResMut<Animations>,
///            mut commands: Commands,
///            asset_server: ResMut<AssetServer>,
///            mut texture_atlases: ResMut<Assets<TextureAtlas>>,
///            ) {
///            let asset = asset_server.load("path/to/your/sprite_sheet");
///
///            let texture_atlas = TextureAtlas::from_grid(asset, Vec2::new(16.0, 16.0), 10, 1, None, None);
///
///            let texture_atlas_handle = texture_atlases.add(texture_atlas);
///
///            let entity = commands.spawn(AnimationDirection::default())
///                .insert(SpriteSheetBundle {
///                    texture_atlas: texture_atlas_handle.clone(),
///                    ..Default::default()
///                }).id();
///            animations.insert_animation(
///                entity, // the entity is needed to determine which `Handle<TextureAtlas>` is being manipulated
///                AnimationType::LinearTransform(
///                    LinearTransformAnimation::new(
///                        /* animation_frames */ vec![0, 1, 2, 3], // the x index for your frames to cycle through
///                        /* meters_per_frame */ 0.55, // your desired meters per frame
///                        /* handle */ texture_atlas_handle, // your sprite sheet
///                        /* frame */ Vec2::new(4., 4.), // the length and height of your sprite sheet
///                        /* direction_indexes */ AnimationDirectionIndexes::IndexBased(IndexBasedDirection {
///                            left: 1,
///                            right: 1,
///                            up: 1,
///                            down: 1
///                        }), // the indexes to determine the correct sprite for the direction
///                        /* repeating */ true, // if the animation is repeating or not
///                    ),
///                    "player_running" // the name of the animation. will be used when sending an `AnimationEvent`
///                )
///           );
///       }
/// ```
#[derive(Debug, Default, Clone)]
pub struct LinearTransformAnimation {
    animation_tick: usize,
    previous_transform: Transform,
    previous_dir_index: usize,
    pub animation_frames: Vec<usize>,
    pub meters_per_frame: f32,
    pub frame: Vec2,
    pub repeating: bool,
    pub direction_indexes: AnimationDirectionIndexes,
}

#[allow(unused)]
impl LinearTransformAnimation {
    fn new(
        animation_frames: Vec<usize>,
        meters_per_frame: f32,
        frame: Vec2,
        direction_indexes: AnimationDirectionIndexes,
        repeating: bool,
    ) -> Self {
        Self {
            animation_tick: 1,
            previous_dir_index: 0,
            previous_transform: Transform::from_xyz(0., 0., 0.),
            animation_frames,
            meters_per_frame,
            frame,
            direction_indexes,
            repeating,
        }
    }

    fn ready_to_animate(&self, transform: &Mut<Transform>, pixels_per_meter: f32) -> bool {
        let x_diff = (transform.translation.x - self.previous_transform.translation.x).abs();
        let y_diff = (transform.translation.y - self.previous_transform.translation.y).abs();

        let modifier = pixels_per_meter * self.meters_per_frame;

        if x_diff >= modifier || y_diff >= modifier {
            return true;
        }
        false
    }

    #[allow(unused)]
    fn get_y_index(&self, direction: &AnimationDirection) -> YIndex {
        match (direction, self.direction_indexes) {
            (AnimationDirection::Left, AnimationDirectionIndexes::IndexBased(index)) => {
                YIndex::Index(index.left)
            }
            (AnimationDirection::Right, AnimationDirectionIndexes::IndexBased(index)) => {
                YIndex::Index(index.right)
            }
            (AnimationDirection::Up, AnimationDirectionIndexes::IndexBased(index)) => {
                YIndex::Index(index.up)
            }
            (AnimationDirection::Down, AnimationDirectionIndexes::IndexBased(index)) => {
                YIndex::Index(index.down)
            }
            (AnimationDirection::Left, AnimationDirectionIndexes::FlipBased(index)) => {
                YIndex::Flip(index.left_direction_is_flipped, index.x_direction_index)
            }
            (AnimationDirection::Right, AnimationDirectionIndexes::FlipBased(index)) => {
                YIndex::Flip(!index.left_direction_is_flipped, index.x_direction_index)
            }
            (_, AnimationDirectionIndexes::FX(fx_based_animation)) => {
                YIndex::Index(fx_based_animation.index)
            }
            (AnimationDirection::Still, _) => YIndex::Index(self.previous_dir_index),
            (_, _) => YIndex::Index(1),
        }
    }

    pub fn sprite_index(&mut self, direction: &AnimationDirection) -> usize {
        let x_index = match self.get_x_index() {
            Some(index) => index,
            None => 0,
        };
        let y_index = match self.get_y_index(direction) {
            YIndex::Index(y_index) => y_index,
            YIndex::Flip(_, y_index) => y_index,
        };
        // Some((y_index * self.frame.y as usize) - (self.frame.x as usize - x_index))
        y_index * self.frame.x as usize + x_index
    }

    fn get_x_index(&mut self) -> Option<usize> {
        match self.animation_frames.get(self.animation_tick) {
            Some(index) => Some(*index),
            None => {
                self.animation_tick = 1;
                if self.repeating {
                    return Some(0)
                }
                None
            },
        }
    }

    pub fn cycle_animation(
        &mut self,
        mut sprite: Mut<TextureAtlasSprite>,
        direction: &AnimationDirection,
        transform: Mut<Transform>,
        pixels_per_meter: f32,
    ) -> Option<()> {
        if self.ready_to_animate(&transform, pixels_per_meter) {
            self.previous_transform = transform.clone();
            let x_index = match self.get_x_index() {
                Some(index) => index,
                None => return None,
            };

            let y_index = match self.get_y_index(direction) {
                YIndex::Index(y_index) => y_index,
                YIndex::Flip(flipped, y_index) => {
                    sprite.flip_x = flipped;
                    y_index
                }
            };

            self.previous_dir_index = y_index;

            // let index = (y_index * self.frame.y as usize) - (self.frame.x as usize - x_index);
            let index = y_index * self.frame.x as usize + x_index;

            sprite.index = index;

            self.animation_tick += 1;
            return Some(());
        } else if *direction == AnimationDirection::Still {
            let x_index = self.animation_frames.get(0).unwrap();

            let y_index = self.previous_dir_index;

            sprite.index = y_index * self.frame.x as usize + x_index;
            return Some(());
        }
        Some(())
    }

    fn is_out_of_bounds(&self, sprite: &Mut<TextureAtlasSprite>, index: usize) -> bool {
        if sprite.field_len() <= index {
            return true;
        }
        false
    }

    pub fn reset_animation(
        &mut self,
        sprite: Option<Mut<TextureAtlasSprite>>,
        direction: Option<&AnimationDirection>,
    ) {
        self.animation_tick = 1;
        if let (Some(mut sprite), Some(direction)) = (sprite, direction) {
            let x_index = self
                .get_x_index()
                .expect("Something Went Wrong Reseting Animation");
            match self.get_y_index(direction) {
                YIndex::Index(y_index) => {
                    sprite.index = y_index * self.frame.x as usize + x_index;
                }
                YIndex::Flip(flip, y_index) => {
                    sprite.flip_x = flip;
                    sprite.index = y_index * self.frame.x as usize + x_index;
                }
            }
        }
    }
}

/// Single Frame Animations. These are easy versatile animations to add to any entity or FX animation 
/// 
/// # Example
/// ```rust
///        fn init_animation(
///            mut animations: ResMut<Animations>,
///            mut commands: Commands,
///            asset_server: ResMut<AssetServer>,
///            mut texture_atlases: ResMut<Assets<TextureAtlas>>,
///            ) {
///            let asset = asset_server.load("path/to/your/sprite_sheet");
///
///            let texture_atlas = TextureAtlas::from_grid(asset, Vec2::new(16.0, 16.0), 10, 1, None, None);
///
///            let texture_atlas_handle = texture_atlases.add(texture_atlas);
///
///            let entity = commands.spawn(AnimationDirection::default())
///                .insert(SpriteSheetBundle {
///                    texture_atlas: texture_atlas_handle.clone(),
///                    ..Default::default()
///                }).id();
///            animations.insert_animation(
///                entity, // the entity is needed to determine which `Handle<TextureAtlas>` is being manipulated
///                AnimationType::SingleFrame(
///                    SingleFrameAnimation::new(
///                        /* blocking */ true, // if the animation should block others
///                        /* blocking_priority */ 1, // the priority at which it will block other blocking animations
///                        /* blocking_duration_in_sec */ 0.15, // the duration of the blocking timer because a single frame animation doesn't have timed frames
///                        /* frame */ Vec2::new(4., 4.), // the length and height of your sprite sheet
///                        /* x_index_pos */ 0 // the x position of the single frame you want rendered by 0th index
///                        /* direction_indexes */ AnimationDirectionIndexes::IndexBased(FlipBased {
///                            x_direction_is_flipped: true, // if the left direction is a flipped sprite
///                            x_direction_index: 0, // to Determine which Index the Horizontal Directions Sprites are 
///                        }), // the indexes to determine the correct sprite for the direction
///                    ),
///                    "player_fall" // the name of the animation. will be used when sending an `AnimationEvent`
///                )
///           );
///       }
/// ```
#[derive(Debug, Default, Clone)]
pub struct SingleFrameAnimation {
    pub blocking: bool,
    pub blocking_priority: i32,
    pub blocking_timer: AnimationTimer,
    pub blocking_finished: bool,
    pub frame: Vec2,
    x_index_pos: usize,
    previous_dir_index: usize,
    direction_indexes: AnimationDirectionIndexes,
}

impl SingleFrameAnimation {
    pub fn new(
        blocking: bool, 
        blocking_priority: i32,
        blocking_duration_in_sec: f32,
        frame: Vec2,
        x_index_pos: usize, 
        direction_indexes: AnimationDirectionIndexes,
    ) -> Self {
        Self { 
            blocking,
            blocking_priority,
            blocking_timer: AnimationTimer(Timer::from_seconds(blocking_duration_in_sec, TimerMode::Repeating)),
            direction_indexes,
            frame,
            x_index_pos,
            ..Default::default()
        }
    }
    // pub get_y_index(&self, )
    pub fn cycle_animation(&mut self, mut sprite: Mut<TextureAtlasSprite>, direction: &AnimationDirection, delta: Duration) {
        if !self.blocking_timer.duration().is_zero() {
            self.blocking_timer.tick(delta);
        }
        if self.blocking_timer.finished() {
            self.blocking_finished = true;
        }

        let index;
        if *direction != AnimationDirection::Still {
            self.previous_dir_index = AnimationDirection::get_direction(direction).y as usize;
        } 
        match self.direction_indexes {
            AnimationDirectionIndexes::IndexBased(index_based_direction) => {
                match *direction {
                    AnimationDirection::Still => index = self.frame.x as usize * self.previous_dir_index + self.x_index_pos,
                    AnimationDirection::Down => index = self.frame.x as usize * index_based_direction.down + self.x_index_pos,
                    AnimationDirection::Left => index = self.frame.x as usize * index_based_direction.left + self.x_index_pos,
                    AnimationDirection::Right => index = self.frame.x as usize * index_based_direction.right + self.x_index_pos,
                    AnimationDirection::Up => index = self.frame.x as usize * index_based_direction.up + self.x_index_pos,
                }
            },
            AnimationDirectionIndexes::FlipBased(flip_based_direction) => {
                index = self.frame.x as usize * flip_based_direction.x_direction_index + self.x_index_pos;
                if flip_based_direction.left_direction_is_flipped {
                    match *direction {
                        AnimationDirection::Left => sprite.flip_x = true,
                        AnimationDirection::Right => sprite.flip_x = false,
                        _ => {}
                    }
                }
                else  {
                    match *direction {
                        AnimationDirection::Left => sprite.flip_x = false,
                        AnimationDirection::Right => sprite.flip_x = true,
                        _ => {}
                    }
                }
            },
            AnimationDirectionIndexes::FX(fx_based_animation)=> index = fx_based_animation.index
        }
        sprite.index = index;
    }

    pub fn sprite_index(&self, direction: &AnimationDirection) -> usize {
        match self.direction_indexes {
            AnimationDirectionIndexes::IndexBased(index_based_direction) => {
                match *direction {
                    AnimationDirection::Still => self.frame.x as usize * self.previous_dir_index + self.x_index_pos,
                    AnimationDirection::Down => self.frame.x as usize * index_based_direction.down + self.x_index_pos,
                    AnimationDirection::Left => self.frame.x as usize * index_based_direction.left + self.x_index_pos,
                    AnimationDirection::Right => self.frame.x as usize * index_based_direction.right + self.x_index_pos,
                    AnimationDirection::Up => self.frame.x as usize * index_based_direction.up + self.x_index_pos,
                }
            },
            AnimationDirectionIndexes::FlipBased(flip_based_direction) => {
                self.frame.x as usize * flip_based_direction.x_direction_index + self.x_index_pos
            },
            AnimationDirectionIndexes::FX(fx_based_animation)=> fx_based_animation.index
        }
    }

    pub fn reset_animation(
        &mut self,
        sprite: Option<Mut<TextureAtlasSprite>>,
        _direction: Option<&AnimationDirection>
    ) {
        self.blocking_timer.reset();
        self.blocking_finished = false;

        if let Some(mut sprite) = sprite {
            sprite.flip_x = false;
            sprite.flip_y = false;
        }
    }
}