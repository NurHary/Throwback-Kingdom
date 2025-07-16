use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct TransitionCamera;

// Ini untuk menghandle input
// output -> (Direction, Posisi terakhir)
pub fn mouse_pan(fin_pos: Vec3, cur_mpos: Vec3, last_mpos: Vec3, mov_pos: Vec3) -> (Vec3, Vec3) {
    if mov_pos == cur_mpos {
        (fin_pos, Vec3::ZERO)
    } else {
        (fin_pos + (last_mpos - cur_mpos), cur_mpos)
    }
}

#[derive(Resource)]
pub struct CameraPosition {
    pub pos: Vec3,
}
