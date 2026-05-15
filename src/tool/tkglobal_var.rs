//! Last Change: Menghapus semua
//!
//! DESCRIPTION: FILE YANG MENYIMPAN SEMUA ATTRIBUTE GLOBAL VARIABLES BAIK UNTUK MENYIMPAN DATA
//! ATAUPUN UNTUK MELAKUKAN RESOURCE SWITCH
//!
//!
//!

use crate::{
    tkitems,
    toolplugin::{QuadtreeIndex, tkquadtree},
};
use bevy::prelude::*;

/// Variable Global untuk mengingat saat ini memilih karakter yang mana (RPG MODES)
#[derive(Resource, Debug)]
pub struct CurrentId {
    pub id: usize,
}

impl CurrentId {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

// // // GAMESTATE // // //

/// Enum Untuk Menentukan GameState Yang Dilakukan Dalam Games Seperti Main Menu, Play, Atau ketika
/// Pause Terjadi
#[derive(States, Debug, PartialEq, Eq, Hash, Default, Clone)]
pub enum GameState {
    Menu,
    #[default]
    Play,
    Pause,
}
//

// NOTE
// // // GAMESTATUS // // // (rpg / rts)

// Untuk Mode
#[derive(Resource)]
pub struct GStatus {
    pub mode: bool,
}

impl GStatus {
    pub fn new(mode: bool) -> Self {
        Self { mode }
    }
    pub fn default() -> Self {
        Self { mode: true }
    }
}

// // // MARQUE SELECT // // //

/// ini untuk menyimpan nilai sementara dari klik mouse terakhir
#[derive(Default)]
pub struct MarqueeCursorPosition {
    pub pos_start: Option<Vec2>,
    pub pos_end: Option<Vec2>,
}

// // // WORLDSIZEE // // //

//#[derive(Resource)]
//struct WorldSize {
//    value: f32,
//}

//impl WorldSize {
//    pub const SMALL: f32 = 1000000.0;
//    pub const MEDIUM: f32 = 100000000.0;
//    pub const LARGE: f32 = 100000000000.0;
//}

// // // ASSET // // //

/// Macro untuk spawn items itu sendiri dikarenakan asset server dan texture layout tidak dapat di
/// kirim ke fn lainnya
macro_rules! spawnitems {
    ($id: expr, $amount: expr, $assv: ident, $texl: ident) => {
        (
            tkphysics::ItemCollisionLayers,
            tkitems::TkItems::new($id, $amount),
            Sprite {
                image: $assv.load("test_items_atlas.png"),
                texture_atlas: Some(TextureAtlas {
                    layout: $texl.clone(),
                    index: tkitems::item_conversion_index($id),
                }),
                custom_size: Some(Vec2::splat(7.)),
                ..Default::default()
            },
        )
    };
}
pub(crate) use spawnitems;

/// Macro untuk spawn items itu sendiri dikarenakan asset server dan texture layout tidak dapat di
/// kirim ke fn lainnya
macro_rules! spawnobjects {
    ($id: expr, $assv: ident, $texl: ident) => {
        (
            tkphysics::ObjectCollisionLayers,
            entities::tkobjects::TkObjects::new($id),
            Sprite {
                image: $assv.load("test_object_atlas.png"),
                texture_atlas: Some(TextureAtlas {
                    layout: $texl.clone(),
                    index: entities::tkobjects::object_atlas_index($id),
                }),
                custom_size: Some(Vec2::splat(7.)),
                ..Default::default()
            },
        )
    };
}

pub(crate) use spawnobjects;
// // // QUADTREE // // //

// NOTE: Ubah ini menjadi event / message
/// Resource Switch yang berguna untuk memberitahu jika diperlukan operasi pendistribusian partisi pada
/// suatu Quadtree
#[derive(Event)]
pub struct QTDistributeConditions {
    pub en: Entity,
    pub pos: Vec<Vec3>,
}

impl QTDistributeConditions {
    pub fn new(en: Entity, pos: Vec<Vec3>) -> Self {
        Self { en, pos }
    }
    pub fn clear(&mut self) {
        self.pos.clear()
    }
}

/// Resource Switch yang berguna untuk memberitahu jika diperlukan operasi penghapusan partisi pada
/// suatu Quadtree
#[derive(Event)]
pub struct QTDeleteConditions {
    pub en: Entity,
    pub tr: Vec<Vec3>,
}

impl QTDeleteConditions {
    pub fn new(en: Entity, tr: Vec<Vec3>) -> Self {
        Self { en, tr }
    }
    pub fn clear(&mut self) {
        self.tr.clear()
    }
}

// // // EVENTS // // //

#[derive(Event)]
/// Struct event untuk mengirimkan sinyal kalau karater (rpg) saat ini berganti, berguna baik untuk
/// ui ataupun command mode dan lainnya
pub struct UiRefreshRpgEvents;

#[derive(Event)]
/// Struct event untuk mengirimkan sinyal kalau karater (rpg) saat ini berganti, berguna baik untuk
/// ui ataupun command mode dan lainnya
pub struct InventoryItemInsertsEvents;
