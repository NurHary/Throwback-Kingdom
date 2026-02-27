use crate::{gamestate::play::UNIT_SPEED, *};
use bevy::prelude::*;

/// To Handle Movement in Rpg Mode and to change character and other utility on Rpg Mode
pub fn rpg_play(
    asset_server: Res<AssetServer>,
    key: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    mut qr: Query<(Entity, &mut Transform, &DynamicHeroId, &mut TkUnit), With<Heroes>>,
    time: Res<Time>,
    mut current_id: ResMut<tkglobal_var::CurrentId>,
    mut enable_camera_motion: Single<&mut PanCam, With<MainCamera>>,
    mut debug_pos: ResMut<DebugCurrentPosition>,
    mut command: Commands,
    mut dynid: ResMut<DynamicIdAllocator>,
) {
    enable_camera_motion.enabled = false;
    if key.just_pressed(KeyCode::KeyR) {
        // TODO Ganti ke DynamicIdAllocator
        current_id.id = dynid.get_next_values(current_id.id);
        //current_id.id = current_id.id.rem_euclid(dynid.unit.len());
        info!("SUATU HAL TERJADI {:?}", current_id.id);
    }
    if current_id.id > *dynid.unit.last().unwrap() {
        current_id.id = 0
    }
    for (en, mut her, id, mut unit) in &mut qr {
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

        // check if move

        // check direction untuk sementara

        let move_del = direction.normalize_or_zero() * UNIT_SPEED * time.delta_secs();

        if id.id == current_id.id {
            // Handle Animation & Movement
            if direction != Vec2::ZERO {
                unit.state = TkUnitState::Walk;
                her.translation += move_del.extend(0.);
                debug_pos.pos = her.translation;
            } else {
                unit.state = TkUnitState::Idle
            }

            // Handle Flip
            if direction.x < 0.0 {
                unit.flip = true
            } else if direction.x > 0.0 {
                unit.flip = false
            }
            if key.just_pressed(KeyCode::KeyB) {
                // Test deletion
                command.entity(en).despawn();
                dynid.delete_unit(id.id);
                break;
            }

            // Handle Movement
        }
    }
    if key.just_pressed(KeyCode::KeyP) {
        println!(
            "h: {:?}, w: {:?}",
            windows.single().unwrap().resolution.height(),
            windows.single().unwrap().resolution.width()
        )
    }

    // TODO: Menambahkan Fungsi untuk mengecek apakah dalam mode rts atau rpg
    // dan apabila ada dalam mode rts, make despawn fungsi dibawah ini
}

/// To Handle RPG camera Movement
pub fn rpg_camera_move(
    selected_player: Query<(&Transform, &DynamicHeroId, &mut TkUnit), With<Heroes>>,
    mut camera: Single<&mut Transform, (With<MainCamera>, Without<Heroes>)>,
    current_id: Res<CurrentId>,
    time: Res<Time>,
    proj: Single<&mut Projection, With<Camera>>,
) {
    match *proj.into_inner() {
        Projection::Orthographic(ref mut orthographic) => {
            orthographic
                .scale
                .smooth_nudge(&0.1, 5.0, time.delta_secs())
        }
        Projection::Perspective(ref mut perspective) => {
            perspective.fov.smooth_nudge(&0.1, 5.0, time.delta_secs())
        }
        _ => {}
    };

    for (tr, id, un) in selected_player {
        if id.id == current_id.id {
            let Vec3 { x, y, .. } = tr.translation;
            let direction = Vec3::new(x, y, camera.translation.z);
            camera
                .translation
                .smooth_nudge(&direction, 5.0, time.delta_secs());
        }
    }
}
