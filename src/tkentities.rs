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

// System Id
// //
// //
#[derive(Component)]
pub struct HeroesId {
    pub id: Arc<Mutex<DynamicHeroId>>,
}

impl HeroesId {
    pub fn new(id: Arc<Mutex<DynamicHeroId>>) -> Self {
        Self { id }
    }
}

// NOTE: TO FIX HERE
#[derive(Debug)]
pub struct DynamicHeroId {
    parent: Option<Weak<Mutex<Self>>>,
    pub value: usize,
    child: Option<Arc<Mutex<Self>>>,
}

impl DynamicHeroId {
    pub fn new(x: usize) -> Self {
        Self {
            parent: None,
            value: x,
            child: None,
        }
    }

    pub fn reduce(&mut self) {
        if self.value > 0 {
            self.value -= 1;
        }
        match &self.child {
            Some(x) => x.as_ref().lock().unwrap().reduce(),
            None => {}
        }
    }
}

#[derive(Clone, Debug, Resource)]
pub struct DynamicHeroList {
    val_front: Option<Arc<Mutex<DynamicHeroId>>>,
    panjang: usize,
    val_back: Option<Arc<Mutex<DynamicHeroId>>>,
}

impl DynamicHeroList {
    pub fn new() -> Self {
        Self {
            val_front: None,
            panjang: 0,
            val_back: None,
        }
    }

    pub fn add_id(&mut self) -> Arc<Mutex<DynamicHeroId>> {
        let mut new_value = Arc::new(Mutex::new(DynamicHeroId::new(self.panjang)));
        match self.val_back.take() {
            Some(y) => {
                new_value.lock().unwrap().parent = Some(Arc::downgrade(&y));
                y.lock().unwrap().child = Some(Arc::clone(&new_value));
                self.val_back = Some(Arc::clone(&new_value));
            }
            None => {
                self.val_front = Some(Arc::clone(&new_value));
                self.val_back = Some(Arc::clone(&new_value));
            }
        }
        self.panjang += 1;
        Arc::clone(&new_value)
    }

    pub fn _delete_index(&mut self, idx: Arc<Mutex<DynamicHeroId>>) {
        let (prev_val, nxt_val) = {
            let idx_guard = idx.lock().unwrap();
            (idx_guard.parent.clone(), idx_guard.child.clone())
        };

        // Previous Value
        if let Some(prev_weak) = &prev_val {
            if let Some(prev) = prev_weak.upgrade() {
                prev.lock().unwrap().child = nxt_val.clone();
            }
        } else {
            self.val_front = nxt_val.clone();
        }

        // Next Value
        if let Some(nxt) = &nxt_val {
            nxt.lock().unwrap().parent = prev_val.clone();
            nxt.lock().unwrap().reduce();
        } else {
            self.val_back = prev_val.and_then(|w| w.upgrade());
        }
    }
    //pub fn print_member(&self) {
    //    match &self.val_front {
    //        Some(x) => x.lock().unwrap().print_child(),
    //        None => {}
    //    }
    //}
}

// // // INVENTORY SLOT // // //
