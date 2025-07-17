use crate::*;
use bevy::prelude::*;

// To Handle Movement in Rpg Mode and to change character and other utility on Rpg Mode
pub fn rpg_function(
    key: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    mut heroes: Query<(&mut Transform, &HeroesId), With<Heroes>>,
    time: Res<Time>,
    mut current_id: ResMut<CurrentId>,
    mut enable_camera_motion: Single<&mut PanCam, With<MainCamera>>,
) {
    enable_camera_motion.enabled = false;
    if key.just_pressed(KeyCode::KeyR) {
        current_id.id += 1;
        current_id.id = current_id.id.rem_euclid(heroes.iter().len())
    }
    for (mut her, id) in &mut heroes {
        let mut direction = Vec2::ZERO;
        if key.pressed(KeyCode::KeyW) {
            direction.y += 1.
        }
        if key.pressed(KeyCode::KeyA) {
            direction.x -= 1.
        }
        if key.pressed(KeyCode::KeyS) {
            direction.y -= 1.
        }
        if key.pressed(KeyCode::KeyD) {
            direction.x += 1.
        }

        let move_del = direction.normalize_or_zero() * 200.0 * time.delta_secs();
        if id.id == current_id.id {
            her.translation += move_del.extend(0.);
        }
    }
    if key.just_pressed(KeyCode::KeyP) {
        println!(
            "h: {:?}, w: {:?}",
            windows.single().unwrap().resolution.height(),
            windows.single().unwrap().resolution.width()
        )
    }
}

// To Handle RPG camera Movement
pub fn rpg_camera_move(
    selected_player: Query<(&Transform, &HeroesId), With<Heroes>>,
    mut camera: Single<&mut Transform, (With<MainCamera>, Without<Heroes>)>,
    current_id: Res<CurrentId>,
    mut campos: ResMut<CameraPosition>,
    time: Res<Time>,
) {
    for (tr, id) in selected_player {
        if id.id == current_id.id {
            let Vec3 { x, y, .. } = tr.translation;
            let direction = Vec3::new(x, y, camera.translation.z);
            camera
                .translation
                .smooth_nudge(&direction, 5.0, time.delta_secs());
            campos.pos = camera.translation;
        }
    }
}
