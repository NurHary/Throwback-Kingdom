use crate::run_condition::*;
use bevy::{prelude::*, scene::ron::Options};

// Variabel Global
// Untuk pemilihan Karakter
#[derive(Resource)]
pub struct CurrentId {
    pub id: usize,
}

impl CurrentId {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

// Enum GameState
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

// Variabel Local
// ini untuk menyimpan nilai sementara dari klik mouse terakhir
#[derive(Default)]
pub struct MarqueeCursorPosition {
    pub pos_start: Option<Vec2>,
    pub pos_end: Option<Vec2>,
}

// NOTE
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

// NOTE
// // // QUADTREE // // //

pub trait QTRC {
    fn clear(&mut self, tr: Vec3);
    fn activate(&mut self, tr: Vec3);
}

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
