//! FILE NAMES  :   tkgameui.rs
//! DESCRIPTION :   FILES DEKLARASI, DEFINISI, SERTA UNTUK MENGHANDLE UI YANG DAPAT DIGUNAKAN OLEH
//!                 PEMAIN

use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};

use crate::{
    entities::tkentities,
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
#[derive(Component)]
pub struct TkUiSprite;
// // // COMPONENT // // //

// // // EVENTS // // //
#[derive(Event)]
struct RespawnUiEvents;
// // // EVENTS // // //

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
fn ui_rpg_inv_root(
    mut builder: Commands,
    qr: Query<Entity, With<TkUiRootInv>>,
    asset_server: Res<AssetServer>,
) {
    if !qr.is_empty() {
        return;
    }
    // Init Root
    if !qr.is_empty() {
        return;
    }
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
                        ))
                        .with_children(|papar| {
                            // Slot Number
                            papar
                                .spawn(Node {
                                    position_type: PositionType::Absolute,
                                    display: Display::Flex,
                                    top: Val::Px(2.0),
                                    left: Val::Px(3.0),
                                    ..Default::default()
                                })
                                .with_children(|papapar| {
                                    papapar.spawn((
                                        Text::new(format!("{}", i + 1)),
                                        TextColor(BLACK.into()),
                                        TextFont {
                                            font: asset_server.load("fonts/Medium.ttf"),
                                            font_size: 12.0,
                                            ..Default::default()
                                        }, // JAJAL
                                    ));
                                });
                        });
                    }
                });
        });
    builder.trigger(tkglobal_var::UiRefreshRpgEvents);
}
// Fungsi untuk update dan handle Ui tentang inventories
fn ui_rpg_handle_amount_items_slots(
    _: On<tkglobal_var::UiRefreshRpgEvents>,
    mut command: Commands,
    //asset_server: Res<AssetServer>,
    qr_root: Query<(Entity, &Children), With<TkUiRootInv>>,
    qr_slot: Query<Entity, With<TkItemSlot>>,
    qr_inv: Query<
        (&tkentities::DynamicHeroId, &tkinventory::TkInventory),
        With<tkinventory::TkInventory>,
    >,
    curid: Res<tkglobal_var::CurrentId>,
    asset_server: Res<AssetServer>,
) {
    // Error Heres; No Entity
    if let Ok((root, child)) = qr_root.single() {
        let mut loc_slot_amount = 0;
        for (id, inv) in &qr_inv {
            if curid.id == id.id {
                loc_slot_amount = inv.slot_amount;
                break;
            }
        }
        let loc_curslot = child.len() as u8;

        if loc_curslot < loc_slot_amount {
            for i in loc_curslot..loc_slot_amount {
                command.entity(root).with_children(|parent| {
                    parent
                        .spawn((
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
                        ))
                        .with_children(|papar| {
                            papar
                                .spawn(Node {
                                    position_type: PositionType::Absolute,
                                    display: Display::Flex,
                                    top: Val::Px(2.0),
                                    left: Val::Px(3.0),
                                    ..Default::default()
                                })
                                .with_children(|papapar| {
                                    papapar.spawn((
                                        Text::new(format!("{}", i + 1)),
                                        TextColor(BLACK.into()),
                                        TextFont {
                                            font: asset_server.load("fonts/Medium.ttf"),
                                            font_size: 12.0,
                                            ..Default::default()
                                        }, // JAJAL
                                    ));
                                });
                            // TODO
                        });
                });
            }
        } else if loc_curslot > loc_slot_amount {
            for chilldren in child.iter().skip(loc_slot_amount.into()) {
                if qr_slot.get(chilldren).is_ok() {
                    command.entity(chilldren).despawn();
                };
            }
        }
    }
    command.trigger(tkglobal_var::InventoryItemInsertsEvents);
}

fn ui_rpg_handle_items_sprite_slots(
    _: On<tkglobal_var::InventoryItemInsertsEvents>,
    mut command: Commands,
    curid: Res<tkglobal_var::CurrentId>,
    qr_inv: Query<
        (&tkentities::DynamicHeroId, &tkinventory::TkInventory),
        With<tkinventory::TkInventory>,
    >,
    qr_slot: Query<(Entity, &TkItemSlot)>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    qr_items: Query<Entity, With<TkUiSprite>>,
) {
    for en in &qr_items {
        command.entity(en).despawn();
    }

    for (id, inv) in &qr_inv {
        if id.id == curid.id {
            for (sloten, slot) in &qr_slot {
                // Guard untuk tidak melebihi sisanya
                if slot.id as usize >= inv.slot.len() {
                    continue;
                };
                command.entity(sloten).with_children(|parpar| {
                    // TODO
                    parpar.spawn((
                        ImageNode {
                            image: asset_server.load("test_items_atlas.png"),
                            texture_atlas: Some(TextureAtlas {
                                layout: texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                                    UVec2::splat(32),
                                    tkitems::ITEMSPRITEMAXHORI,
                                    tkitems::ITEMSPRITEMAXVERT,
                                    None,
                                    None,
                                )),
                                index: tkitems::item_conversion_index(
                                    inv.slot[slot.id as usize].id,
                                ),
                            }),

                            ..Default::default()
                        },
                        TkUiSprite,
                    ));
                    parpar
                        .spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                bottom: Val::Px(2.0),
                                right: Val::Px(3.0),
                                //display: Display::Flex,
                                //justify_content: JustifyContent::FlexEnd,
                                //align_items: AlignItems::FlexEnd,
                                ..Default::default()
                            },
                            TkUiSprite,
                        ))
                        .with_children(|itemamountbuilder| {
                            itemamountbuilder.spawn((
                                Text::new(format!("{}", inv.slot[slot.id as usize].amount)),
                                TextColor(BLACK.into()),
                                TextFont {
                                    font: asset_server.load("fonts/Medium.ttf"),
                                    font_size: 12.0,
                                    ..Default::default()
                                },
                            ));
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
    qr_invrpg: Query<Entity, With<TkUiRootInv>>,
    qr_rts: Query<Entity, With<TkRtsUi>>,
    asset_server: Res<AssetServer>,
) {
    if gamemode.mode {
        // if mode == rpg
        for i in &qr_rts {
            cmd.entity(i).despawn();
        }
        cmd.trigger(RespawnUiEvents);
        cmd.trigger(tkglobal_var::UiRefreshRpgEvents);
    } else {
        for i in &qr_invrpg {
            cmd.entity(i).despawn_children();
        }
    }
}

fn ui_respawn_refresh(
    _: On<RespawnUiEvents>,
    mut command: Commands,
    gamemode: Res<tkglobal_var::GStatus>,
    qr_invrpg: Query<Entity, With<TkUiRootInv>>,
    asset_server: Res<AssetServer>,
) {
    if gamemode.mode {
        for i in &qr_invrpg {
            command.entity(i).with_children(|reversal| {
                reversal
                    .spawn((
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
                        TkItemSlot::new(0),
                        Button,
                    ))
                    .with_children(|papar| {
                        // Slot Number
                        papar
                            .spawn(Node {
                                position_type: PositionType::Absolute,
                                display: Display::Flex,
                                top: Val::Px(2.0),
                                left: Val::Px(3.0),
                                ..Default::default()
                            })
                            .with_children(|papapar| {
                                papapar.spawn((
                                    Text::new(format!("{}", 0 + 1)),
                                    TextColor(BLACK.into()),
                                    TextFont {
                                        font: asset_server.load("fonts/Medium.ttf"),
                                        font_size: 12.0,
                                        ..Default::default()
                                    }, // JAJAL
                                ));
                            });
                    });
            });
        }
    } else {
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
        app.add_observer(ui_respawn_refresh);
    }
}
