//!
//!
//! DESCRIPTION: FILE YANG MENYIMPAN SEMUA ATTRIBUTE GLOBAL VARIABLES BAIK UNTUK MENYIMPAN DATA
//! ATAUPUN UNTUK MELAKUKAN RESOURCE SWITCH
//!
//!
//!

use crate::{
    tkrun_condition::*,
    toolplugin::tkitems::{TkItems, ITEMIDS},
};
use bevy::{prelude::*, scene::ron::Options};

/// Variable Global untuk mengingat saat ini memilih karakter yang mana (RPG MODES)
#[derive(Resource)]
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

#[derive(Resource)]
struct WorldSize {
    value: f32,
}

impl WorldSize {
    pub const SMALL: f32 = 1000000.0;
    pub const MEDIUM: f32 = 100000000.0;
    pub const LARGE: f32 = 100000000000.0;
}

// // // QUADTREE // // //

pub trait QTRC {
    fn clear(&mut self, tr: Vec3);
    fn activate(&mut self, tr: Vec3);
}

/// Resource Switch yang berguna untuk memberitahu jika diperlukan operasi pendistribusian partisi pada
/// suatu Quadtree
#[derive(Resource)]
pub struct QTDistributeConditions {
    pub pos: Vec<Vec3>,
    pub condition: bool,
}
impl Default for QTDistributeConditions {
    fn default() -> Self {
        Self {
            pos: Vec::new(),
            condition: false,
        }
    }
}

impl QTRC for QTDistributeConditions {
    fn clear(&mut self, tr: Vec3) {
        self.pos.retain(|value| *value != tr);
        if self.pos.len() > 0 {
            self.condition = false;
        }
    }
    fn activate(&mut self, tr: Vec3) {
        self.pos.push(tr);
        self.condition = true
    }
}

/// Resource Switch yang berguna untuk memberitahu jika diperlukan operasi penghapusan partisi pada
/// suatu Quadtree
#[derive(Resource)]
pub struct QTDeleteConditions {
    pub pos: Vec<Vec3>,
    pub condition: bool,
}
impl Default for QTDeleteConditions {
    fn default() -> Self {
        Self {
            pos: Vec::new(),
            condition: false,
        }
    }
}
impl QTRC for QTDeleteConditions {
    fn clear(&mut self, tr: Vec3) {
        self.pos.retain(|value| *value != tr);
        if self.pos.len() > 0 {
            self.condition = false;
        }
    }
    fn activate(&mut self, tr: Vec3) {
        self.pos.push(tr);
        self.condition = true
    }
}

// // // Inventory // // //

/// Resource Switch yang digunakan ketika ada suatu items yang Ditabrak oleh suatu entitas / Unit
#[derive(Resource, Clone, Copy)]
pub struct InvDSys {
    pub items: Option<ITEMIDS>,
    pub amount: u8,
    // TODO: Menambahkan Entity dimana Entity Tersebut menunjuk pada unit yang menabrak sehingga
    // siap dilakukan penambahan inventory
    pub condition: bool,
}

impl InvDSys {
    pub fn new() -> Self {
        Self {
            items: None,
            amount: 0,
            condition: false,
        }
    }
    pub fn activate(&mut self, items: ITEMIDS, amount: u8) {
        self.items = Some(items);
        self.amount = amount
    }
    pub fn clear(&mut self) {
        self.items = None;
        self.amount = 0;
        self.condition = false
    }
}
