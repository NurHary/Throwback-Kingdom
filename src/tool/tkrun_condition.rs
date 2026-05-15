//! Run Condition adalah module yang menyimpan struct logic dari Run Condition yang dapat
//! memungkinkan trigger suatu fungsi ketika kondisi terpenuhi

use crate::tkglobal_var::*;
use bevy::prelude::*;

/// ini digunakan untuk mentrigger perubahan game mode
pub fn rc_gamemode(mode: Res<GStatus>) -> bool {
    mode.mode // apabila true, make rpg
}
