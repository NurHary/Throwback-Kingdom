//! File    :   tktilemap
//! Desc    :   File yang hold Plugin dimana juga hold semua implementasi untuk object - object di
//!             dalam tilemap tersebut

use bevy::{math::bool, prelude::*};

pub type Tilemaptile = Vec<Vec<usize>>;
pub type Tilemapindex = Vec<Vec<usize>>;

#[derive(Debug, Clone, Copy)]
pub struct TkTiles {
    pub pos_index: usize,
    pub tile_index: usize,
}

impl TkTiles {
    pub fn new(pos_index: usize, tile_index: usize) -> Self {
        Self {
            pos_index,
            tile_index,
        }
    }
}

// Tilemap itu sendiri
// Tilemap ini dibuat untuk menerima satu layer seperti layer ground, layer particle, dan layer lainnya
#[derive(Component)]
struct TkTilemap {
    tiles: Tilemaptile,
    index: Tilemapindex,
}

#[derive(Debug, Clone, Copy)]
pub enum LayerLevel {
    Ground,
    Biome,
    Particle,
    Object,
    Omni,
}

struct TkWorld;

// // // PLUGINS// // //

/// Tk Worlds merupakan plugin yang mengatur dunia dan objek - objek yang ada di dunia tersebut
impl Plugin for TkWorld {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

//fn
