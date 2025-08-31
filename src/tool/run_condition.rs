use crate::global_var::*;
use bevy::prelude::*;

pub fn rc_gamemode(mode: Res<GStatus>) -> bool {
    // ini digunakan untuk mentrigger perubahan game mode
    mode.mode
}
pub fn qt_distribute(need: Res<QTDistributeChild>) -> bool {
    // ini digunakan untuk mentrigger Panggilan untuk melakukan distribute pada anakan quadtree
    need.condition
}
