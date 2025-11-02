use crate::*;
use bevy::{prelude::*, render::texture};

pub fn spawn_character(
    mut command: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
    mut her_id: ResMut<DynamicHeroList>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    time: Res<Time>,
) {
    // Test
    // // //
    let testure: Handle<Image> = asset_server.load("atlas_test.png");

    let layout: TextureAtlasLayout =
        TextureAtlasLayout::from_grid(UVec2::splat(16), 2, 2, None, None);
    let texture_atlas: Handle<TextureAtlasLayout> = texture_atlas_layout.add(layout);

    // Test Texture Atlas untuk tilemap
    command.spawn((
        Sprite {
            image: testure,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas,
                index: 2,
            }),
            custom_size: Some(Vec2::splat(7.)),
            ..Default::default()
        },
        Transform::from_xyz(-25.0, -100.0, 0.0),
    ));

    // Test
    // // //

    command.spawn((
        Camera2d,
        MainCamera,
        PanCam {
            grab_buttons: vec![MouseButton::Middle],
            move_keys: DirectionKeys::NONE,
            speed: 500.,
            enabled: true,
            zoom_to_cursor: true,
            min_scale: 0.1,
            max_scale: 30.,
            ..Default::default()
        },
    ));

    let player_texture: Handle<Image> = asset_server.load("test_chara.png");
    let player_atlas: Handle<TextureAtlasLayout> = texture_atlas_layout.add(
        TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 2, None, None),
    );

    // Test spawn Character
    command.spawn((
        HeroesBundles::new(
            Heroes::new("King Edward"),
            HeroesId::new(her_id.add_id()),
            TkUnitState::Idle,
        ),
        Sprite {
            image: player_texture,
            texture_atlas: Some(TextureAtlas {
                layout: player_atlas,
                index: 0,
            }),
            image_mode: SpriteImageMode::Auto,
            custom_size: Some(Vec2::splat(7.)),
            ..Default::default()
        },
        TkAnimation {
            idle: Some(TkAnimationStorage::new(
                Timer::from_seconds(0.4, TimerMode::Repeating),
                0,
                3,
            )),
            walk: Some(TkAnimationStorage::new(
                Timer::from_seconds(0.2, TimerMode::Repeating),
                4,
                7,
            )),
        },
        Selectable::new(),
        ColliderBundles::new(100.0, 100.0),
        Transform::from_xyz(25.0, 25.0, 0.0),
    ));
    command.spawn((
        HeroesBundles::new(
            Heroes::new("Alfred"),
            HeroesId::new(her_id.add_id()),
            TkUnitState::Idle,
        ),
        Sprite {
            image: asset_server.load("test_chara.png"),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                    UVec2::splat(32),
                    4,
                    2,
                    None,
                    None,
                )),
                index: 0,
            }),
            image_mode: SpriteImageMode::Auto,
            custom_size: Some(Vec2::splat(7.)),
            color: Color::Hsla(Hsla::new(141.0, 0.86, 0.77, 1.0)),
            ..Default::default()
        },
        TkAnimation {
            idle: Some(TkAnimationStorage::new(
                Timer::from_seconds(0.4, TimerMode::Repeating),
                0,
                3,
            )),
            walk: Some(TkAnimationStorage::new(
                Timer::from_seconds(0.2, TimerMode::Repeating),
                4,
                7,
            )),
        },
        Selectable::new(),
        ColliderBundles::new(100.0, 100.0),
        Transform::from_xyz(30.0, -20.0, 0.0),
    ));
    command.spawn((
        HeroesBundles::new(
            Heroes::new("Fulan"),
            HeroesId::new(her_id.add_id()),
            TkUnitState::Idle,
        ),
        Sprite {
            image: asset_server.load("test_chara.png"),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                    UVec2::splat(32),
                    4,
                    2,
                    None,
                    None,
                )),
                index: 0,
            }),
            image_mode: SpriteImageMode::Auto,
            custom_size: Some(Vec2::splat(7.)),
            color: Color::Hsla(Hsla::new(12.0, 0.86, 0.77, 1.0)),
            ..Default::default()
        },
        TkAnimation {
            idle: Some(TkAnimationStorage::new(
                Timer::from_seconds(0.4, TimerMode::Repeating),
                0,
                3,
            )),
            walk: Some(TkAnimationStorage::new(
                Timer::from_seconds(0.2, TimerMode::Repeating),
                4,
                7,
            )),
        },
        Selectable::new(),
        ColliderBundles::new(100.0, 100.0),
        Transform::from_xyz(-90.0, 25.0, 0.0),
    ));
    // NOTE Nonaktifkan Sementara
    command.spawn((
        HeroesBundles::new(
            Heroes::new("Fulan"),
            HeroesId::new(her_id.add_id()),
            TkUnitState::Idle,
        ),
        Sprite {
            image: asset_server.load("test_chara.png"),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                    UVec2::splat(32),
                    4,
                    2,
                    None,
                    None,
                )),
                index: 0,
            }),
            image_mode: SpriteImageMode::Auto,
            custom_size: Some(Vec2::splat(7.)),
            color: Color::Hsla(Hsla::new(12.0, 0.86, 0.77, 1.0)),
            ..Default::default()
        },
        TkAnimation {
            idle: Some(TkAnimationStorage::new(
                Timer::from_seconds(0.4, TimerMode::Repeating),
                0,
                3,
            )),
            walk: Some(TkAnimationStorage::new(
                Timer::from_seconds(0.2, TimerMode::Repeating),
                4,
                7,
            )),
        },
        Selectable::new(),
        ColliderBundles::new(100.0, 100.0),
        Transform::from_xyz(220.0, 0.0, 0.0),
    ));
    command.spawn((
        HeroesBundles::new(
            Heroes::new("Fulan"),
            HeroesId::new(her_id.add_id()),
            TkUnitState::Idle,
        ),
        Sprite {
            image: asset_server.load("test_chara.png"),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                    UVec2::splat(32),
                    4,
                    2,
                    None,
                    None,
                )),
                index: 0,
            }),
            image_mode: SpriteImageMode::Auto,
            custom_size: Some(Vec2::splat(7.)),
            color: Color::Hsla(Hsla::new(12.0, 0.86, 0.77, 1.0)),
            ..Default::default()
        },
        TkAnimation {
            idle: Some(TkAnimationStorage::new(
                Timer::from_seconds(0.4, TimerMode::Repeating),
                0,
                3,
            )),
            walk: Some(TkAnimationStorage::new(
                Timer::from_seconds(0.2, TimerMode::Repeating),
                4,
                7,
            )),
        },
        Selectable::new(),
        ColliderBundles::new(100.0, 100.0),
        Transform::from_xyz(-100.0, -50.0, 0.0),
    ));
}

pub fn camera_startup(
    mut camera: Single<&mut Transform, (With<MainCamera>, Without<TkUnit>)>,
    king_edward: Query<(&Transform, &HeroesId), With<TkUnit>>,
) {
    for (tr, id) in king_edward {
        if id.id.lock().unwrap().value == 0 {
            let Vec3 { x, y, .. } = tr.translation;
            let mc_position = Vec3::new(x, y, camera.translation.z);

            camera.translation = mc_position;
        }
    }
}
