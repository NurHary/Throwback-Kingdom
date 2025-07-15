mod tk_entities;
mod tool;

use std::usize;

#[cfg(not(target_arch = "wasm32"))]
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
use bevy::{
    input::keyboard::{self, KeyboardInput},
    picking::window,
    prelude::*,
    window::PrimaryWindow,
};

use tk_entities::*;
use tool::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use bevy::window::WindowMode;

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    position: WindowPosition::Automatic,
                    resolution: Vec2::new(600., 600.).into(),
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            #[cfg(not(target_arch = "wasm32"))]
            Wireframe2dPlugin::default(),
        ))
        .insert_resource(CurrentId::new(0))
        .add_systems(Startup, babi)
        .add_systems(Update, (cursor_pos, rpg_move))
        .run();
}

fn babi(
    mut command: Commands,
    key: Res<ButtonInput<MouseButton>>,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    command.spawn((Camera2d, MainCamera));
    let meshu = [
        mesh.add(Rectangle::new(52., 52.)),
        mesh.add(Rectangle::new(32., 32.)),
    ];
    let colour = Color::linear_rgb(1.0, 0.0, 1.0);

    command.spawn((
        Heroes::new("Edward"),
        HeroesId::new(0),
        Unit,
        Mesh2d(mesh.add(Rectangle::new(52.0, 52.0))),
        MeshMaterial2d(material.add(colour)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    command.spawn((
        Heroes::new("Alfred"),
        HeroesId::new(1),
        Unit,
        Mesh2d(mesh.add(Rectangle::new(52.0, 52.0))),
        MeshMaterial2d(material.add(colour)),
        Transform::from_xyz(20.0, 50.0, 0.0),
    ));
}

fn cursor_pos(
    mut command: Commands,
    key: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single().unwrap();
    if key.just_pressed(MouseButton::Right) {
        if let Some(position) = window
            .cursor_position()
            .and_then(|cur| Some(camera.viewport_to_world(camera_transform, cur)))
            .map(|ray| ray.unwrap().origin.truncate())
        {
            command.spawn((
                Unit,
                Mesh2d(mesh.add(Rectangle::new(52., 52.))),
                MeshMaterial2d(material.add(Color::linear_rgb(1.0, 0.0, 0.0))),
                Transform::from_xyz(position.x, position.y, 0.0),
            ));
        }
    }
}

fn rpg_move(
    key: Res<ButtonInput<KeyCode>>,
    mut command: Commands,
    mut heroes: Query<(&mut Transform, &HeroesId), With<Heroes>>,
    time: Res<Time>,
    mut current_id: ResMut<CurrentId>,
) {
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
}

fn rpg_camera_move() {}
