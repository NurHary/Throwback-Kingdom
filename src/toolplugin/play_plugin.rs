use crate::*;
use bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;

pub struct GameplayPlugin;
impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Play),
            (
                gamestate::startup::spawn_character,
                gamestate::startup::camera_startup,
            )
                .chain(),
        );
        app.add_systems(
            Update,
            (
                //gamestate::play::cursor_pos,
                gamestate::play::maingameloop,
                (
                    gamestate::play_rpg::rpg_function,
                    gamestate::play_rpg::rpg_camera_move,
                )
                    .chain()
                    .run_if(rc_gamemode), // run hanya apabila gamemode = rpg
                ((
                    gamestate::play_rts::rts_play,
                    gamestate::play_rts::rts_handle_movement,
                )
                    .chain())
                .run_if(not(rc_gamemode)), // run hanya apabila game mode = rts
            )
                .run_if(in_state(GameState::Play)), // ini hanya akan berjalan ketika game state
                                                    // adalah play
        );
        app.add_systems(EguiPrimaryContextPass, show_current_position);
    }
}
