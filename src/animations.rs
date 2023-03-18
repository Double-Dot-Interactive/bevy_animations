use crate::*;

// This is Primarily for This Is Primarily For Animations on players or NPCs, for example shooting a bow or reloading a gun 
#[derive(Clone, Debug, Default)]
pub struct TimedAnimation {
    animation_tick: usize,
    previous_dir_index: usize,
    pub frame_timings_in_secs: Vec<f32>,
    pub blocking: bool,
    pub blocking_priority: i32,
    pub animation_frames: Vec<usize>,
    pub handle: Handle<TextureAtlas>,
    pub frame: Vec2,
    pub direction_indexes: AnimationDirectionIndexes,
    pub repeating: bool,
    pub animation_timer: AnimationTimer
}

impl TimedAnimation {
    pub fn new(
        animation_frames: Vec<usize>,
        frame_timings_in_secs: Vec<f32>,
        handle: Handle<TextureAtlas>,
        frame: Vec2,
        direction_indexes: AnimationDirectionIndexes,
        repeating: bool,
        blocking: bool,
        blocking_priority: i32
    ) -> Self {
        let timer_dur = *frame_timings_in_secs.get(0).unwrap();
        Self { 
            animation_tick: 1,
            animation_frames, 
            frame_timings_in_secs,
            handle, 
            frame, 
            direction_indexes, 
            repeating,
            previous_dir_index: 1,
            animation_timer: AnimationTimer(Timer::from_seconds(
                timer_dur, 
                TimerMode::Repeating)),
            blocking,
            blocking_priority
        }
    }
    fn get_x_index(&mut self) -> Option<usize> {
        let index = self.animation_frames.get(self.animation_tick - 1);
        if self.repeating {
            match index {
                Some(index) => return Some(*index),
                None => {
                    self.animation_tick = 1;
                    self.animation_frames.get(self.animation_tick)
                        .expect(format!("There was A Problem Cycling Animation\nThe index is {} but The Frame Length is {}", self.animation_tick, self.animation_frames.len()).as_str());
                    return Some(self.animation_tick);
                }
            }
        }
        else {
            match index {
                Some(index) => return Some(*index),
                None => return None
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
        let x_index = match self.get_x_index(){
            Some(index) => index,
            None => 0,
        };
        let y_index = self.get_y_index(direction);
        (y_index * self.frame.y as usize) - (self.frame.x as usize - x_index)
    }

    pub fn cycle_animation(&mut self, mut sprite: Mut<TextureAtlasSprite>, direction: &AnimationDirection, delta: Duration, name: &'static str) -> Option<()> {
        if self.ready_to_animate(delta) {
            let y_index = self.get_y_index(direction);
            self.previous_dir_index = y_index;
            let x_index = match self.get_x_index() {
                Some(index) => index,
                None => {
                    self.animation_tick = 1;
                    return None
                }
            };
            let index = (y_index * self.frame.y as usize) - (self.frame.x as usize - x_index);
            sprite.index = index;
            let timing = *self.frame_timings_in_secs.get(self.animation_tick - 1).expect("Error With Animation Timing");
            self.animation_timer.set_duration(Duration::from_secs_f32(timing));
            self.animation_timer.reset();
            self.animation_tick += 1;
            return Some(());
        }
        Some(())
    }

    pub fn reset_animation(&mut self, sprite: Option<Mut<TextureAtlasSprite>>, direction: Option<&AnimationDirection>) {
        self.animation_tick = 1;
        let new_dur = Duration::from_secs_f32(*self.frame_timings_in_secs.get(0).expect("Error With Animation Timing"));
        self.animation_timer.set_duration(new_dur);
        self.animation_timer.reset();
        if let (Some(mut sprite), Some(direction)) = (sprite, direction) {
            let x_index = self.get_x_index().expect("Something Went Wrong Reseting Animation");
            let y_index = self.get_y_index(direction);
            sprite.index = (y_index * self.frame.y as usize) - (self.frame.x as usize - x_index)
        }
    }

    fn get_y_index(&self, direction: &AnimationDirection) -> usize {
        match direction {
            AnimationDirection::Left => self.direction_indexes.left,
            AnimationDirection::Right => self.direction_indexes.right,
            AnimationDirection::Up => self.direction_indexes.up,
            AnimationDirection::Down => self.direction_indexes.down,
            AnimationDirection::Still => self.previous_dir_index
        }
    }
    fn is_out_of_bounds(&self, sprite: &Mut<TextureAtlasSprite>, index: usize) -> bool {
        if sprite.field_len() <= index {
            return true;
        }
        false
    } 
}


/// This Is Primarily For Animations on players or NPCs, for example walking or running
#[derive(Debug, Default, Clone)]
pub struct TransformAnimation {
    animation_tick: usize,
    previous_dir_index: usize,
    previous_transform: Transform,
    pub animation_frames: Vec<usize>,
    pub meters_per_frame: f32,
    pub handle: Handle<TextureAtlas>,
    pub frame: Vec2,
    pub direction_indexes: AnimationDirectionIndexes,
    pub repeating: bool,
}

impl TransformAnimation {
    pub fn new(
        animation_frames: Vec<usize>,
        meters_per_frame: f32,
        handle: Handle<TextureAtlas>,
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
            handle,
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
        let x_index = match self.get_x_index(){
            Some(index) => index,
            None => 0,
        };
        let y_index = self.get_y_index(direction);
        (y_index * self.frame.y as usize) - (self.frame.x as usize - x_index)
    }

    pub fn cycle_animation(
        &mut self, 
        mut sprite: Mut<TextureAtlasSprite>, 
        direction: &AnimationDirection, 
        transform: Mut<Transform>,
        pixels_per_meter: f32,
        name: &'static str
    ) -> Option<()> {
        let y_index = self.get_y_index(direction);
        if self.ready_to_animate(&transform, pixels_per_meter) || y_index != self.previous_dir_index {
            self.previous_transform = transform.clone();
            let x_index = match self.get_x_index() {
                Some(index) => index,
                None => return None
            };

            let y_index = self.get_y_index(direction);

            self.previous_dir_index = y_index;

            let index = (y_index * self.frame.y as usize) - (self.frame.x as usize - x_index);

            sprite.index = index;

            self.animation_tick += 1;       
            return Some(())     
        }
        else if *direction == AnimationDirection::Still {
            let x_index = self.animation_frames.get(0).unwrap();

            let y_index = self.previous_dir_index;

            sprite.index = (y_index * self.frame.y as usize) - (self.frame.x as usize - x_index);
            return Some(())
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
                    self.animation_frames.get(self.animation_tick)
                        .expect(format!("There was A Problem Cycling Animation\nThe index is {} but The Frame Length is {}", self.animation_tick, self.animation_frames.len()).as_str());
                    return Some(self.animation_tick);
                }
            }
        }
        else {
            match index {
                Some(index) => return Some(*index),
                None => return None
            };
        }
    }
    fn get_y_index(&self, direction: &AnimationDirection) -> usize {
        match direction {
            AnimationDirection::Left => self.direction_indexes.left,
            AnimationDirection::Right => self.direction_indexes.right,
            AnimationDirection::Up => self.direction_indexes.up,
            AnimationDirection::Down => self.direction_indexes.down,
            AnimationDirection::Still => self.previous_dir_index
        }
    }

    pub fn reset_animation(&mut self, sprite: Option<Mut<TextureAtlasSprite>>, direction: Option<&AnimationDirection>) {
        self.animation_tick = 1;
        if let (Some(mut sprite), Some(direction)) = (sprite, direction) {
            let x_index = self.get_x_index().expect("Something Went Wrong Reseting Animation");
            let y_index = self.get_y_index(direction);
            sprite.index = (y_index * self.frame.y as usize) - (self.frame.x as usize - x_index)
        }
    } 
}

/// This Is Primarily For Animations on objects, for example doors to open and close
/// 
/// **Note** the sprite sheets for these animations should have 1 column. It's okay if they have more however is't functionally irrelevant
#[derive(Debug, Default, Clone)]
pub struct LinearTimedAnimation {
    animation_tick: usize,
    animation_timer: AnimationTimer,
    pub frame_timings_in_secs: Vec<f32>,
    pub animation_frames: Vec<usize>,
    pub handle: Handle<TextureAtlas>,
    pub repeating: bool,
}

impl LinearTimedAnimation {
    pub fn new(
        animation_frames: Vec<usize>,
        frame_timings_in_secs: Vec<f32>,
        handle: Handle<TextureAtlas>,
        repeating: bool
    ) -> Self {
        Self {
            animation_tick: 1,
            animation_timer: AnimationTimer(Timer::from_seconds(*frame_timings_in_secs.get(0).expect("Something went wrong retrieving timings"), TimerMode::Repeating)),
            animation_frames,
            frame_timings_in_secs,
            handle,
            repeating
        }
    }

    fn get_x_index(&mut self) -> Option<usize> {
        match self.animation_frames.get(self.animation_tick) {
            Some(index) => Some(*index),
            None => {
                self.animation_tick = 1;
                None
            }
        }
    }

    pub fn cycle_animation(&mut self, mut sprite: Mut<TextureAtlasSprite>, delta: Duration, name: &'static str) -> Option<()> {
        self.animation_timer.tick(delta);
        if self.animation_timer.finished() {
            let x_index = match self.get_x_index() {
                Some(index) => index,
                None => return None
            };
            
            let new_dur = Duration::from_secs_f32(*self.frame_timings_in_secs.get(self.animation_tick).expect("There Was A Problem Getting New Timing Check Your Timing Configs"));
            self.animation_timer.set_duration(new_dur);
            self.animation_timer.reset();

            sprite.index = x_index;

            self.animation_tick += 1;

            return Some(())
        }
        Some(())
    }

    pub fn reset_animation(&mut self, mut sprite: Option<Mut<TextureAtlasSprite>>) {
        self.animation_tick = 1;
        let new_dur = Duration::from_secs_f32(*self.frame_timings_in_secs.get(0).expect("Error With Animation Timing"));
        self.animation_timer.set_duration(new_dur);
        self.animation_timer.reset();
        if let Some(mut sprite) = sprite {
            let x_index = self.get_x_index().expect("Something Went Wrong Reseting Animation");
            sprite.index = x_index
        }
    }
}


/// This Is Primarily For Animations on objects, for example a projectile
#[derive(Debug, Default, Clone)]
pub struct LinearTransformAnimation {
    animation_tick: usize,
    previous_transform: Transform,
    previous_dir_index: usize,
    pub animation_frames: Vec<usize>,
    pub meters_per_frame: f32,
    pub handle: Handle<TextureAtlas>,
    pub frame: Vec2,
    pub repeating: bool,
    pub direction_indexes: AnimationDirectionIndexes
}

impl LinearTransformAnimation {
    fn new(
        animation_frames: Vec<usize>,
        meters_per_frame: f32,
        handle: Handle<TextureAtlas>,
        frame: Vec2,
        direction_indexes: AnimationDirectionIndexes,
        repeating: bool
    ) -> Self {
        Self { 
            animation_tick: 1, 
            previous_dir_index: 0,
            previous_transform: Transform::from_xyz(0., 0., 0.), 
            animation_frames, 
            meters_per_frame, 
            handle, 
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

    fn get_y_index(&self, direction: &AnimationDirection) -> usize {
        match direction {
            AnimationDirection::Left => self.direction_indexes.left,
            AnimationDirection::Right => self.direction_indexes.right,
            AnimationDirection::Up => self.direction_indexes.up,
            AnimationDirection::Down => self.direction_indexes.down,
            AnimationDirection::Still => self.previous_dir_index
        }
    }

    pub fn sprite_index(&mut self, direction: &AnimationDirection) -> Option<usize> {
        let x_index = match self.get_x_index(){
            Some(index) => index,
            None => return None,
        };
        let y_index = self.get_y_index(direction);
        Some((y_index * self.frame.y as usize) - (self.frame.x as usize - x_index))
    }

    fn get_x_index(&self) -> Option<usize> {
        match self.animation_frames.get(self.animation_tick) {
            Some(index) => Some(*index),
            None => None
        }
    }

    pub fn cycle_animation(
        &mut self, 
        mut sprite: Mut<TextureAtlasSprite>, 
        direction: &AnimationDirection, 
        transform: Mut<Transform>,
        pixels_per_meter: f32
    ) {
        if self.ready_to_animate(&transform, pixels_per_meter) {
            self.previous_transform = transform.clone();
            let x_index = match self.get_x_index() {
                Some(index) => index,
                None => return
            };

            let y_index = self.get_y_index(direction);

            self.previous_dir_index = y_index;

            let index = (y_index * self.frame.y as usize) - (self.frame.x as usize - x_index);


            sprite.index = index;

            self.animation_tick += 1;            
        }
        else if *direction == AnimationDirection::Still {
            let x_index = self.animation_frames.get(0).unwrap();

            let y_index = self.previous_dir_index;

            sprite.index = (y_index * self.frame.y as usize) - (self.frame.x as usize - x_index);
        }
    }

    fn is_out_of_bounds(&self, sprite: &Mut<TextureAtlasSprite>, index: usize) -> bool {
        if sprite.field_len() <= index {
            return true;
        }
        false
    } 
}