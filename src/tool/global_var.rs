use bevy::prelude::*;

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

#[derive(Resource)]
struct WorldSize {
    value: f32,
}

impl WorldSize {
    pub const SMALL: f32 = 1000000.0;
    pub const MEDIUM: f32 = 100000000.0;
    pub const LARGE: f32 = 100000000000.0;
}

#[derive(Resource)]
pub struct QTDistributeChild {
    pub pos: Option<Vec3>,
    pub condition: bool,
}
impl Default for QTDistributeChild {
    fn default() -> Self {
        Self {
            pos: None,
            condition: false,
        }
    }
}
//impl QuadtreeDistributeChild {
//    pub fn need_to_distribute(&mut self, tr: Vec3) {
//        self.pos = Some(tr);
//        self.condition = true
//    }
//    pub fn clear(&mut self) {
//        self.pos = None;
//        self.condition = false;
//    }
//}
