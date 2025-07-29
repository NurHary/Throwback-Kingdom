use crate::*;
use bevy::prelude::*;

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
                    .run_if(rc_gamemode),
                ((
                    gamestate::play_rts::rts_play,
                    gamestate::play_rts::rts_handle_movement,
                )
                    .chain())
                .run_if(not(rc_gamemode)),
            )
                .run_if(in_state(GameState::Play)),
        );
    }
}
