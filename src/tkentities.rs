//! RECENT CHANGES: ADD THE INVENTORY SLOT COMPONENT
//!
//!
//! DESCRIPTION: FILE YANG MENYIMPAN KESELURUHAN SYSTEM UNTUK ENTITAS BAIK ENTITAS KAWAN ATAUPUN
//! ENTITAS LAWAN

use bevy::prelude::*;
use std::sync::{Arc, Mutex, Weak};

// Heroes adalah mereka karakter yang dapat dikendalikan hanya dalam mode RPG
#[derive(Component)]
pub struct Heroes {
    nama: String,
}

impl Heroes {
    pub fn new(nama: &str) -> Self {
        Self {
            nama: nama.to_string(),
        }
    }
}

// Unit adalah mereka karakter yang dapat dikendalikan hanya dalam mode RTS.
// Biasanya Heroes == Unit, tapi Unit != Heroes
#[derive(Component)]
pub struct TkUnit {
    pub state: TkUnitState,
    pub flip: bool,
}

impl TkUnit {
    pub fn new(state: TkUnitState) -> Self {
        Self { state, flip: false }
    }
}

pub enum TkUnitState {
    Idle,
    Walk,
}

// TODO: mengubah system dari bool selected menjadi option<vec3> target
#[derive(Component)]
pub struct UnitSelectable {
    pub selected: bool,
}

impl UnitSelectable {
    pub fn new() -> Self {
        Self { selected: false }
    }
}

#[derive(Component)]
pub struct DynamicHeroId {
    pub id: usize,
}

impl DynamicHeroId {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

#[derive(Resource)]
pub struct DynamicIdAllocator {
    pub unit: Vec<usize>,
}

impl DynamicIdAllocator {
    pub fn new() -> Self {
        Self { unit: Vec::new() }
    }
    pub fn new_unit(&mut self) -> usize {
        if self.unit.is_empty() {
            self.unit.push(0);
            return 0;
        } else if let Some(i) = self.unit.last() {
            let apx: usize = *i + 1;
            self.unit.push(apx);
            return apx;
        } else {
            error!("KESALAHAN DALAM INIT ID");
            return 0;
        }
    }
    pub fn delete_unit(&mut self, val: usize) {
        if let Some(idx) = self.unit.iter().position(|x| *x == val) {
            self.unit.remove(idx);
        } else {
            error!("GAGAL MENGHAPUS ID")
        }
    }
    //pub fn get_idx_pos(&self, ptr: usize) {}
    pub fn get_next_values(&self, val: usize) -> usize {
        if let Some(idx) = self.unit.iter().position(|x| *x == val) {
            if self.unit.len() > idx + 1 {
                return self.unit[idx + 1];
            } else {
                return self.unit[0];
            }
        }
        return self.unit[0];
    }
}

// // // INVENTORY SLOT // // //
