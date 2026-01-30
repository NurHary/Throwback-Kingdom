//!
//!
//!
//! DESCRIPTION: FILES DEKLARASI, DEFINISI, SERTA UNTUK MENGHANDLE UI YANG DAPAT DIGUNAKAN OLEH
//! PEMAIN
//!
//!
//!

use bevy::prelude::*;

use crate::{
    tkentities,
    tool::{tkglobal_var, tkrun_condition},
    toolplugin::TkInventory,
};

// // // COMPONENT // // //
#[derive(Component, Copy, Clone)]
pub struct TkItemSlot {
    pub id: u8,
}

impl TkItemSlot {
    pub fn new(id: u8) -> Self {
        Self { id }
    }
}

#[derive(Component)]
pub struct TkRpgUi;
#[derive(Component)]
pub struct TkRtsUi;

// // // UI LAYOUT // // //
//
// // UNIVERSAL // //
pub fn minimap_ui() {}
pub fn action_ui() {}
pub fn sidebar_access_button_ui() {}
pub fn operation_minipanel_ui() {}

// // RPG ONLY // //
pub fn rpg_slot_items_ui(
    builder: &mut Commands,
    asset_server: Res<AssetServer>,
    // HeroesId untuk mendapatkan unit yang saat ini dikendalikan
    qr: Query<(&tkentities::HeroesId, &TkInventory), With<TkInventory>>,
    curid: Res<tkglobal_var::CurrentId>,
) {
    builder
        .spawn((
            TkRpgUi,
            Node {
                height: Val::Percent(100.),
                width: Val::Percent(100.),
                padding: UiRect::all(Val::Px(20.)),
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            //BackgroundColor(Color::linear_rgba(0., 0.5, 0.5, 0.15)),
        ))
        .with_children(|root_parent| {
            root_parent
                .spawn((
                    Node {
                        padding: UiRect::all(Val::Px(10.)),
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        height: Val::Percent(12.),
                        width: Val::Percent(32.),
                        ..Default::default()
                    },
                    //BackgroundColor(Color::linear_rgba(
                    //    0.94901960784,
                    //    0.94901960784,
                    //    0.94901960784,
                    //    0.12,
                    //)),
                ))
                .with_children(|parentsecond| {
                    for i in 0u8..5u8 {
                        parentsecond.spawn((
                            Node {
                                //justify_content: JustifyContent::Stretch,
                                width: Val::Px(60.),
                                height: Val::Px(60.),
                                margin: UiRect::all(Val::Px(2.)),
                                ..Default::default()
                            },
                            BackgroundColor(Color::linear_rgb(
                                0.55294117647,
                                0.55294117647,
                                0.55294117647,
                            )),
                            TkItemSlot::new(i),
                            Button,
                            // TODO: Menambahkan untuk menggambar Sprite
                        ));
                    }
                });
        });
}
pub fn rpg_healtbar_ui() {}

// // RTS ONLY // //
pub fn rts_bottom_bar_ui() {}
pub fn quick_info_minipanel_ui() {}

// // // IMPLEMENTATION // // //

/// Fungsi untuk meghandle (Despawn) ui tergantung pada game modenya
fn handle_ui_changes(
    mut cmd: Commands,
    gamemode: Res<tkglobal_var::GStatus>,
    qr_rpg: Query<Entity, With<TkRpgUi>>,
    qr_rts: Query<Entity, With<TkRtsUi>>,
) {
    if gamemode.mode {
        for i in &qr_rts {
            cmd.entity(i).despawn();
        }
    } else {
        for i in &qr_rpg {
            cmd.entity(i).despawn();
        }
    }
}

/// Fungsi untuk Menghandle Slot Item Button serta input angka untuk mengakses button tersebut
fn handle_rpg_slot_items(
    mut qr: Query<(&Interaction, &mut BackgroundColor, &TkItemSlot), Changed<Interaction>>,
) {
    for (inter, mut bck, iteslot) in &mut qr {
        match inter {
            Interaction::Pressed => {
                println!("Click Button")
                // TODO handle drag and drop?
            }
            Interaction::Hovered => {
                *bck = Color::WHITE.into()
                // TODO tambahkan informasi item yang ada di situ
            }
            Interaction::None => {
                *bck = Color::linear_rgb(0.55294117647, 0.55294117647, 0.55294117647).into()
            }
        }
    }
}

// // // PLUGIN // // //
pub struct TkGameUiPlugin;
impl Plugin for TkGameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_ui_changes,
                (handle_rpg_slot_items).run_if(tkrun_condition::rc_gamemode),
            ),
        );
    }
}
