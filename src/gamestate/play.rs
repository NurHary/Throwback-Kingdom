use crate::*;
use bevy::prelude::*;

pub const UNIT_SPEED: f32 = 500.;

pub fn maingameloop(key: Res<ButtonInput<KeyCode>>, mut gstate: ResMut<GStatus>) {
    if key.just_pressed(KeyCode::Tab) {
        gstate.mode = !gstate.mode
    }
}

// Test fungsi kursor (untuk mendapatkan posisi global dari cursor)
pub fn cursor_pos(
    mut command: Commands,
    key: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single().unwrap();

    // ini untuk medapatkan posisi global
    if key.just_pressed(MouseButton::Right) {
        if let Some(position) = window
            .cursor_position()
            .and_then(|cur| Some(camera.viewport_to_world(camera_transform, cur)))
            .map(|ray| ray.unwrap().origin.truncate())
        {
            command.spawn((
                Mesh2d(mesh.add(Rectangle::new(52., 52.))),
                MeshMaterial2d(material.add(Color::linear_rgb(1.0, 0.0, 0.0))),
                Transform::from_xyz(position.x, position.y, 0.0),
            ));
        }
    }
    // ini untuk mendapatkan posisi local
    if key.just_pressed(MouseButton::Left) {
        if let Some(position) = window.cursor_position() {
            println!(
                "mouse height: {:?}, mouse width: {:?}",
                -(position.y - (window.height() / 2.0)),
                position.x - (window.width() / 2.0)
            )
        }
    }
}

pub fn handle_animation(
    mut main_query: Query<(&TkUnit, &mut Sprite, &mut TkAnimation), Without<Mesh2d>>,
    time: Res<Time>,
) {
    for (ch_state, mut sprite, mut anim) in &mut main_query {
        match ch_state.state {
            TkUnitState::Idle => {
                if let Some(idl) = &mut anim.idle {
                    idl.time.tick(time.delta());
                    if idl.time.just_finished() {
                        if let Some(atlas) = &mut sprite.texture_atlas {
                            atlas.index = if atlas.index == idl.end_animation {
                                idl.start_animation
                            } else if atlas.index < idl.start_animation
                                || atlas.index > idl.end_animation
                            {
                                idl.start_animation
                            } else {
                                atlas.index + 1
                            }
                        }
                    }
                }
            }
            TkUnitState::Walk => {
                if let Some(wlak) = &mut anim.walk {
                    wlak.time.tick(time.delta());
                    if wlak.time.just_finished() {
                        if let Some(atlas) = &mut sprite.texture_atlas {
                            atlas.index = if atlas.index == wlak.end_animation {
                                wlak.start_animation
                            } else if atlas.index < wlak.start_animation
                                || atlas.index > wlak.end_animation
                            {
                                wlak.start_animation
                            } else {
                                atlas.index + 1
                            }
                        }
                    }
                }
            }
        }
        sprite.flip_x = ch_state.flip;
    }
}
