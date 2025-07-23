use crate::global_var::*;
use bevy::prelude::*;

pub fn rc_gamemode(mode: Res<GStatus>) -> bool {
    mode.mode
}
