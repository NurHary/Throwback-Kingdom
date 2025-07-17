use crate::global_var::*;
use bevy::prelude::*;

pub fn rc_gamemode(mode: Res<GStatus>) -> bool {
    mode.mode
}

pub fn rc_gamestate(mode: Res<GStatus>) -> usize {
    match mode.state {
        GameState::Menu => 0,
        GameState::Play => 1,
        GameState::Pause => 2,
    }
}
