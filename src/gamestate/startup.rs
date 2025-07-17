use crate::*;
use bevy::prelude::*;

pub fn spawn_character(
    mut command: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    command.spawn((
        Camera2d,
        MainCamera,
        PanCam {
            grab_buttons: vec![MouseButton::Middle],
            move_keys: DirectionKeys::NONE,
            speed: 500.,
            enabled: true,
            zoom_to_cursor: true,
            min_scale: 1.,
            max_scale: 30.,
            ..Default::default()
        },
    ));
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

pub fn camera_startup(
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
