mod debugtool;
mod gamestate;
mod tkentities;
mod tool;
mod toolplugin;

use debugtool::*;
use tkentities::*;
use tool::*;
use toolplugin::*;

#[cfg(not(target_arch = "wasm32"))]
use bevy::window::WindowMode;
use bevy::{prelude::*, window::PrimaryWindow};

use bevy_egui::{egui, EguiPlugin};

use bevy_pancam::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use bevy::window::WindowResolution;

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        position: WindowPosition::Automatic,
                        resolution: WindowResolution::new(600, 600),
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            #[cfg(not(target_arch = "wasm32"))]
            PanCamPlugin::default(),
            EguiPlugin::default(),
            TkAnimationPlugin,
            GameplayPlugin,
        ))
        .init_state::<GameState>()
        .run();
}
