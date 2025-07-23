use bevy::prelude::*;

#[derive(Component)]
pub struct TkAnimation {
    pub idle: Option<TkAnimationStorage>,
    pub walk: Option<TkAnimationStorage>,
}

// Animation
#[derive(Clone)]
pub struct TkAnimationStorage {
    pub time: Timer,
    pub start_animation: usize,
    pub end_animation: usize,
}

impl TkAnimationStorage {
    pub fn new(time: Timer, start_animation: usize, end_animation: usize) -> Self {
        Self {
            time,
            start_animation,
            end_animation,
        }
    }
}
