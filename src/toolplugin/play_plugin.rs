use crate::*;
use bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;

use toolplugin::tkinventory;

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

                // // // // Main GameLoop, berjalan tak peduli di mode apapun
                gamestate::play::maingameloop,
                tkgameui::handle_rpg_slot_items,
                //(
                //tkinventory::test_insert_item_to_inventory,
                //tkinventory::distribute_items.run_if(inv_distribute),
                //)
                //    .chain(),
                // // // // RPG GameLoop, berjalan hanya ketika ada dalam mode rpg
                (
                    gamestate::play_rpg::rpg_play,
                    gamestate::play_rpg::rpg_camera_move,
                )
                    .chain()
                    .run_if(rc_gamemode), // run hanya apabila gamemode = rpg
                // // // // RTS GameLoop, berjalan hanya ketika ada dalam mode rts
                (
                    gamestate::play_rts::rts_play,
                    gamestate::play_rts::rts_handle_movement,
                )
                    .chain()
                    .run_if(not(rc_gamemode)),
            )
                .run_if(in_state(GameState::Play)), // ini hanya akan berjalan ketika game state
                                                    // adalah play
        );
        app.add_systems(EguiPrimaryContextPass, show_current_position);
    }
}
