use crate::{gamestate::play::UNIT_SPEED, tkcamera, tkentities, tkglobal_var};
use bevy::{math::VectorSpace, prelude::*, window::CursorGrabMode, window::PrimaryWindow};
use bevy_pancam;

pub fn rts_play(
    key: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    mut enable_camera_motion: Single<&mut bevy_pancam::PanCam, With<tkcamera::MainCamera>>,
) {
    enable_camera_motion.enabled = true;
}

pub fn rts_handle_movement(
    mut unit_query: Query<
        (
            &tkentities::HeroesId,
            &mut Transform,
            &mut tkentities::UnitSelectable,
            &mut tkentities::TkUnit,
        ),
        With<tkentities::UnitSelectable>,
    >,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<tkcamera::MainCamera>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut ls_cursor_pos: Local<tkglobal_var::MarqueeCursorPosition>,
    mut gizmo: Gizmos,
    time: Res<Time>,
) {
    let (camera, camera_transform) = q_camera.single().unwrap();

    if mouse.just_pressed(MouseButton::Left) {
        // ini akan di eksekusi hanya ketika dipencet saja
        if let Some(position) = window
            .cursor_position()
            .and_then(|cur| Some(camera.viewport_to_world(camera_transform, cur)))
            .map(|ray| ray.unwrap().origin.truncate())
        {
            //println!("posisi.x: {:?}, posisi.y: {:?}", position.x, position.y);
            ls_cursor_pos.pos_start = Some(position);
        }
    } else if mouse.pressed(MouseButton::Left) {
        if let Some(position) = window
            .cursor_position()
            .and_then(|cur| Some(camera.viewport_to_world(camera_transform, cur)))
            .map(|ray| ray.unwrap().origin.truncate())
        {
            //println!(
            //"posisi_update.x: {:?}, posisi_update.y: {:?}",
            //position.x,
            //position.y
            //);
            ls_cursor_pos.pos_end = Some(position);
        }
    } else if mouse.just_released(MouseButton::Left) {
        // ini akan menghapus data ketika mouse telah dilepas
        if let (Some(c_start), Some(c_end)) = (ls_cursor_pos.pos_start, ls_cursor_pos.pos_end) {
            let c_minimum = c_start.min(c_end);
            let c_maximum = c_start.max(c_end);
            let rect_size = Rect::new(c_minimum.x, c_minimum.y, c_maximum.x, c_maximum.y);
            for (_, tr, mut slc, _) in &mut unit_query {
                if rect_size.contains(Vec2::new(tr.translation.x, tr.translation.y)) {
                    slc.selected = true
                } else {
                    slc.selected = false
                }
            }
            ls_cursor_pos.pos_start = None;
            ls_cursor_pos.pos_end = None;
        }
    }

    if let (Some(c_start), Some(c_end)) = (ls_cursor_pos.pos_start, ls_cursor_pos.pos_end) {
        let c_minimum = c_start.min(c_end);
        let c_maximum = c_start.max(c_end);
        let size = c_maximum - c_minimum;

        gizmo.rect_2d(
            c_minimum + size / 2.0,
            size,
            Color::linear_rgb(0.0, 0.0, 1.0),
        );
    }

    if mouse.pressed(MouseButton::Right) {
        // ini akan di eksekusi hanya ketika dipencet saja
        if let Some(position) = window
            .cursor_position()
            .and_then(|cur| Some(camera.viewport_to_world(camera_transform, cur)))
            .map(|ray| ray.unwrap().origin.truncate())
        {
            for (id, mut tr, slc, mut unit) in &mut unit_query {
                let mut direction = Vec2::ZERO;
                if slc.selected {
                    direction.x = (position.x > tr.translation.x) as i32 as f32
                        - (position.x < tr.translation.x) as i32 as f32;
                    direction.y = (position.y > tr.translation.y) as i32 as f32
                        - (position.y < tr.translation.y) as i32 as f32;

                    if direction != Vec2::ZERO {
                        unit.state = tkentities::TkUnitState::Walk
                    } else {
                        unit.state = tkentities::TkUnitState::Idle
                    }

                    // Handle Flip
                    if direction.x < 0.0 {
                        unit.flip = true
                    } else if direction.x > 0.0 {
                        unit.flip = false
                    }

                    let move_del = direction.normalize_or_zero() * UNIT_SPEED * time.delta_secs();
                    tr.translation += move_del.extend(0.);
                }
            }
        }
    }

    //for (unit, mut tr) in &unit_query {
    //    let mut direction = Vec2::ZERO;

    //direction.x = tr.translation.x
    //}
}
