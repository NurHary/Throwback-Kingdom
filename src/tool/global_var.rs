use bevy::prelude::*;

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

#[derive(Resource)]
struct CurrentCursorPos {
    pos: Vec3,
}

// Enum GameMode
pub enum GameMode {
    RTS,
    RPG,
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
    pub mode: GameMode,
}

impl GStatus {
    pub fn new(mode: GameMode, state: GameState) -> Self {
        Self { mode, state }
    }
    pub fn default() -> Self {
        Self {
            mode: GameMode::RPG,
            state: GameState::Play,
        }
    }
}
