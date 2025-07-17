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
pub enum GameState {
    Menu,
    Play,
    Pause,
}

// Untuk Mode
#[derive(Resource)]
pub struct GStatus {
    pub state: GameState,
    pub mode: bool,
}

impl GStatus {
    pub fn new(mode: bool, state: GameState) -> Self {
        Self { mode, state }
    }
    pub fn default() -> Self {
        Self {
            mode: true,
            state: GameState::Play,
        }
    }
}

// Variabel Local
// ini untuk menyimpan nilai sementara dari klik mouse terakhir
#[derive(Default)]
pub struct CurrentCursorPos {
    pub pos: Vec3,
}
