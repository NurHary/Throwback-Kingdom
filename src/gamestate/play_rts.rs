use crate::*;
use bevy::prelude::*;

pub fn rts_play(
    key: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    mut enable_camera_motion: Single<&mut PanCam, With<MainCamera>>,
) {
    enable_camera_motion.enabled = true;
}
