use crate::*;
use bevy::{ecs::entity::Entities, prelude::*};

pub const UNIT_SPEED: f32 = 50.;

pub fn maingameloop(key: Res<ButtonInput<KeyCode>>, mut gstate: ResMut<GStatus>) {
    if key.just_pressed(KeyCode::Tab) {
        gstate.mode = !gstate.mode
    }
}

// Test fungsi kursor (untuk mendapatkan posisi global dari cursor)
//pub fn cursor_pos(
//    mut command: Commands,
//    key: Res<ButtonInput<MouseButton>>,
//    window: Single<&Window, With<PrimaryWindow>>,
//    mut mesh: ResMut<Assets<Mesh>>,
//    mut material: ResMut<Assets<ColorMaterial>>,
//    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
//) {
//    let (camera, camera_transform) = q_camera.single().unwrap();
//
//    // ini untuk medapatkan posisi global
//    if key.just_pressed(MouseButton::Right) {
//        if let Some(position) = window
//            .cursor_position()
//            .and_then(|cur| Some(camera.viewport_to_world(camera_transform, cur)))
//            .map(|ray| ray.unwrap().origin.truncate())
//        {
//            command.spawn((
//                Mesh2d(mesh.add(Rectangle::new(52., 52.))),
//                MeshMaterial2d(material.add(Color::linear_rgb(1.0, 0.0, 0.0))),
//                Transform::from_xyz(position.x, position.y, 0.0),
//            ));
//        }
//    }
//    // ini untuk mendapatkan posisi local
//    if key.just_pressed(MouseButton::Left) {
//        if let Some(position) = window.cursor_position() {
//            println!(
//                "mouse height: {:?}, mouse width: {:?}",
//                -(position.y - (window.height() / 2.0)),
//                position.x - (window.width() / 2.0)
//            )
//        }
//    }
//}
