mod gamestate;
mod tk_entities;
mod tool;

use tk_entities::*;
use tool::*;

#[cfg(not(target_arch = "wasm32"))]
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
use bevy::{prelude::*, window::PrimaryWindow};

use bevy_egui::{egui, EguiPlugin};

use bevy_pancam::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use bevy::window::WindowMode;

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        position: WindowPosition::Automatic,
                        resolution: Vec2::new(600., 600.).into(),
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            #[cfg(not(target_arch = "wasm32"))]
            Wireframe2dPlugin::default(),
            PanCamPlugin::default(),
            EguiPlugin::default(),
            TkAnimationPlugin,
        ))
        .insert_resource(CurrentId::new(0))
        .insert_resource(GStatus::default()) // ini untuk menentukan rts atau rpg
        .insert_resource(DynamicHeroList::new()) // ini untuk memberikan id ke setiap heroes
        .init_state::<GameState>()
        .add_systems(
            Startup,
            (
                gamestate::startup::spawn_character,
                gamestate::startup::camera_startup,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                //gamestate::play::cursor_pos,
                gamestate::play::maingameloop,
                (
                    gamestate::play_rpg::rpg_function,
                    gamestate::play_rpg::rpg_camera_move,
                )
                    .chain()
                    .run_if(rc_gamemode),
                ((
                    gamestate::play_rts::rts_play,
                    gamestate::play_rts::rts_handle_movement,
                )
                    .chain())
                .run_if(not(rc_gamemode)),
            ),
        )
        .run();
}
