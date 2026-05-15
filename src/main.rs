mod debugtool;
mod entities;
mod gamestate;
mod tool;
mod toolplugin;

use debugtool::*;
use entities::tkentities::*;
use tool::*;
use toolplugin::*;

use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use bevy::window::WindowMode;

use bevy_egui::EguiPlugin;

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
            GameplayPlugin,
        ))
        .init_state::<GameState>()
        .run();
}
