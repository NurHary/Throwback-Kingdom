//! Run Condition adalah module yang menyimpan struct logic dari Run Condition yang dapat
//! memungkinkan trigger suatu fungsi ketika kondisi terpenuhi

use crate::global_var::*;
use bevy::prelude::*;

/// ini digunakan untuk mentrigger perubahan game mode
pub fn rc_gamemode(mode: Res<GStatus>) -> bool {
    mode.mode
}

/// ini digunakan untuk mentrigger Panggilan untuk melakukan distribute pada anakan quadtree
pub fn qt_distribute(need: Res<QTDistributeConditions>) -> bool {
    need.condition
}

/// ini digunakan untuk mentrigger Panggilan untuk melakukan penghapusan partisi
pub fn qt_delete(need: Res<QTDeleteConditions>) -> bool {
    need.condition
}

pub fn inv_distribute(need: Res<InvDSys>) -> bool {
    need.condition
}
