mod gamestate;
mod tk_entities;
mod tool;
mod toolplugin;

use tk_entities::*;
use tool::*;
use toolplugin::*;

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
            GameplayPlugin,
        ))
        .insert_resource(CurrentId::new(0))
        .insert_resource(GStatus::default()) // ini untuk menentukan rts atau rpg
        .insert_resource(DynamicHeroList::new()) // ini untuk memberikan id ke setiap heroes
        .init_state::<GameState>()
        .run();
}
