use crate::*;
use bevy::prelude::*;

pub struct TkAnimationPlugin;
impl Plugin for TkAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_animation.run_if(in_state(GameState::Play)));
    }
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

#[derive(Component)]
pub struct TkEntityAnimation {
    pub idle: Option<TkAnimationStorage>,
    pub walk: Option<TkAnimationStorage>,
}

// Update Hanya ketika ada perubahan pada nyawa
#[derive(Component)]
struct TkObjectAnimations {
    pub normal_state: u8,
    pub broken_state: Vec<u8>,
}

pub fn handle_animation(
    mut main_query: Query<(&TkUnit, &mut Sprite, &mut TkEntityAnimation), Without<Mesh2d>>,
    time: Res<Time>,
) {
    for (ch_state, mut sprite, mut anim) in &mut main_query {
        match ch_state.state {
            TkUnitState::Idle => {
                if let Some(idl) = &mut anim.idle {
                    idl.time.tick(time.delta());
                    if idl.time.just_finished() {
                        if let Some(atlas) = &mut sprite.texture_atlas {
                            atlas.index = if atlas.index == idl.end_animation {
                                idl.start_animation
                            } else if atlas.index < idl.start_animation
                                || atlas.index > idl.end_animation
                            {
                                idl.start_animation
                            } else {
                                atlas.index + 1
                            }
                        }
                    }
                }
            }
            TkUnitState::Walk => {
                if let Some(wlak) = &mut anim.walk {
                    wlak.time.tick(time.delta());
                    if wlak.time.just_finished() {
                        if let Some(atlas) = &mut sprite.texture_atlas {
                            atlas.index = if atlas.index == wlak.end_animation {
                                wlak.start_animation
                            } else if atlas.index < wlak.start_animation
                                || atlas.index > wlak.end_animation
                            {
                                wlak.start_animation
                            } else {
                                atlas.index + 1
                            }
                        }
                    }
                }
            }
        }
        sprite.flip_x = ch_state.flip;
    }
} // jajal
