//! FILE NAMES  :   tkgameui.rs
//! DESCRIPTION :   FILES DEKLARASI, DEFINISI, SERTA UNTUK MENGHANDLE UI YANG DAPAT DIGUNAKAN OLEH
//!                 PEMAIN

use bevy::prelude::*;

use crate::{
    tkentities,
    tool::{tkglobal_var, tkrun_condition},
    toolplugin::{tkinventory, tkitems},
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

// Inventory Root
#[derive(Component)]
pub struct TkUiRootInv;

// // // UI LAYOUT // // //
//
// // UNIVERSAL // //
pub fn minimap_ui() {}
pub fn action_ui() {}
pub fn sidebar_access_button_ui() {}
pub fn operation_minipanel_ui() {}

// // RPG ONLY // //
// Inventory Systems //

// Fungsi Startup yang membuat system Root untuk membangun Child slots
fn ui_rpg_inv_root(mut builder: Commands, qr: Query<Entity, With<TkUiRootInv>>) {
    if !qr.is_empty() {
        return;
    }
    // Init Root
    info!("Init Inv Root");
    if !qr.is_empty() {
        return;
    }
    builder.trigger(tkglobal_var::IsHeroesChanged);
    builder
        .spawn((Node {
            height: Val::Percent(100.),
            width: Val::Percent(100.),
            padding: UiRect::all(Val::Px(20.)),
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },))
        .with_children(|parent| {
            parent
                .spawn((
                    TkUiRootInv,
                    Node {
                        padding: UiRect::all(Val::Px(10.)),
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        height: Val::Percent(12.),
                        width: Val::Percent(32.),
                        ..Default::default()
                    },
                ))
                .with_children(|parr| {
                    for i in 0u8..5u8 {
                        parr.spawn((
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
                        ));
                    }
                });
        });
}
// Fungsi untuk update dan handle Ui tentang inventories
fn ui_rpg_handle_amount_items_slots(
    _: On<tkglobal_var::IsHeroesChanged>,
    mut command: Commands,
    //asset_server: Res<AssetServer>,
    qr_root: Query<(Entity, &Children), With<TkUiRootInv>>,
    qr_slot: Query<Entity, With<TkItemSlot>>,
    qr_inv: Query<
        (&tkentities::DynamicHeroId, &tkinventory::TkInventory),
        With<tkinventory::TkInventory>,
    >,
    curid: Res<tkglobal_var::CurrentId>,
) {
    info!("Init Thing Ui");
    // Error Heres; No Entity
    if let Ok((root, child)) = qr_root.single() {
        info!("Init Getting Root {root}");

        let mut loc_slot_amount = 0;
        for (id, inv) in &qr_inv {
            if curid.id == id.id {
                loc_slot_amount = inv.slot_amount;
                break;
            }
        }
        let loc_curslot = child.len() as u8;
        info!("Test Print Banyak {loc_curslot}, {loc_slot_amount}");

        if loc_curslot < loc_slot_amount {
            //info!("Test Kurang Banyak {loc_curslot}, {loc_slot_amount}");
            for i in loc_curslot..loc_slot_amount {
                command.entity(root).with_children(|parent| {
                    parent.spawn((
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
                    ));
                });
            }
        } else if loc_curslot > loc_slot_amount {
            //info!("Test terlalu Banyak {loc_curslot}, {loc_slot_amount}");
            for chilldren in child.iter().skip(loc_slot_amount.into()) {
                if qr_slot.get(chilldren).is_ok() {
                    //info!("Delete Shit");
                    command.entity(chilldren).despawn();
                };
            }
        }
    }
}

fn ui_rpg_handle_items_sprite_slots(
    _: On<tkglobal_var::InventoryItemInserts>,
    mut command: Commands,
    curid: Res<tkglobal_var::CurrentId>,
    qr_inv: Query<
        (&tkentities::DynamicHeroId, &tkinventory::TkInventory),
        With<tkinventory::TkInventory>,
    >,
    mut qr_slot: Query<(Entity, &mut TkItemSlot)>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    info!("Init Items Ui");
    for (id, inv) in &qr_inv {
        if id.id == curid.id {
            for (sloten, mut slot) in &qr_slot {
                // Guard untuk tidak melebihi sisanya
                if inv.slot.len() < (slot.id + 1) as usize {
                    break;
                };
                command.entity(sloten).with_children(|parpar| {
                    // TODO
                    parpar.spawn(Sprite {
                        image: asset_server.load("atlas_test.png"),
                        texture_atlas: Some(TextureAtlas {
                            layout: texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                                UVec2::splat(32),
                                3,
                                1,
                                None,
                                None,
                            )),
                            index: tkitems::item_conversion_index(inv.slot[slot.id as usize].id),
                        }),
                        custom_size: Some(Vec2::splat(3.)),
                        ..Default::default()
                    });
                });
            }
        }
    }
}

// Health Systems //
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
                println!("Click Button {}", iteslot.id)
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
        app.add_systems(Startup, ui_rpg_inv_root);
        app.add_systems(
            Update,
            (
                handle_ui_changes,
                (handle_rpg_slot_items).run_if(tkrun_condition::rc_gamemode),
            ),
        );
        app.add_observer(ui_rpg_handle_amount_items_slots);
        app.add_observer(ui_rpg_handle_items_sprite_slots);
    }
}
