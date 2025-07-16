mod gamestate;
mod tk_entities;
mod tool;

use gamestate::*;
use tk_entities::*;
use tool::*;

#[cfg(not(target_arch = "wasm32"))]
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
use bevy::{
    input::keyboard::{self, KeyboardInput},
    picking::window,
    prelude::*,
    window::PrimaryWindow,
};

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
        .insert_resource(CameraPosition { pos: Vec3::ZERO })
        .insert_resource(GStatus::default())
        .add_systems(Startup, (spawn_character, camera_startup).chain())
        .add_systems(
            Update,
            (
                cursor_pos,
                (rpg_function, rpg_camera_move).chain().run_if(rc_gamemode),
            ),
        )
        .run();
}

fn spawn_character(
    mut command: Commands,
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
        Transform::from_xyz(1000.0, 50.0, 0.0),
    ));
}

fn camera_startup(
    mut camera: Single<&mut Transform, (With<MainCamera>, Without<Unit>)>,
    king_alfred: Query<(&Transform, &HeroesId), With<Unit>>,
    mut campos: ResMut<CameraPosition>,
) {
    for (tr, id) in king_alfred {
        if id.id == 0 {
            let Vec3 { x, y, .. } = tr.translation;
            let mc_position = Vec3::new(x, y, camera.translation.z);

            camera.translation = mc_position;
            campos.pos = camera.translation;
        }
    }
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
    if key.just_pressed(MouseButton::Left) {
        if let Some(position) = window.cursor_position() {
            println!("mouse: {:?}", position)
        }
    }
}

// Migh Be used
//fn mainloop() {}

// To Handle Movement in Rpg Mode and to change character and other utility on Rpg Mode
fn rpg_function(
    key: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
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
    if key.just_pressed(KeyCode::KeyP) {
        println!(
            "h: {:?}, w: {:?}",
            windows.single().unwrap().resolution.height(),
            windows.single().unwrap().resolution.width()
        )
    }
}

// To Handle RPG camera Movement
fn rpg_camera_move(
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

// To Handle Movement in RTS Mode and to change character and other utility on RTS Mode
fn rts_move(
    time: Res<Time>,
    key: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut campos: ResMut<CameraPosition>,
    windows: Query<&Window>,
) {
    // Pan
    let mut cur_mos: Vec3 = Vec3::ZERO;
    if mouse.just_pressed(MouseButton::Middle) {}

    if mouse.pressed(MouseButton::Middle) {
        println!("{:?}", windows.single().unwrap().resolution.height())
    }

    // Dota Move
    if mouse.just_pressed(MouseButton::Right) {}

    // Select / Draw Select
    if mouse.pressed(MouseButton::Left) {}
}

// To Handle RTS camera Movement
//fn rts_camera_move() {}
