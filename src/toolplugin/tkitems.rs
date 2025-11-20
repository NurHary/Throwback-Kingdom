use std::usize;

use crate::toolplugin::inventory_sys;
use bevy::prelude::*;

const MAXIMUM_ITEM_STACK: u8 = 99;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ITEMIDS {
    Wood,
    Stone,
    Fiber,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct TkItems {
    id: ITEMIDS,
    amount: usize,
}
impl TkItems {
    pub fn new(id: ITEMIDS, amount: usize) -> Self {
        Self { id, amount }
    }
    pub fn add_amount(&mut self, amount: usize) -> (bool, usize) {
        if (self.amount + amount) <= MAXIMUM_ITEM_STACK as usize {
            self.amount += amount;
            // apabila bisa ditambah, maka kita tidak perlu melakukan aksi selanjutnya
            return (false, 0);
        }
        // apabila tidak bisa ditambah, maka kita perlu melakukan aksi selanjutnya
        self.amount += amount - ((self.amount + amount) - MAXIMUM_ITEM_STACK as usize);
        (true, (self.amount + amount) - MAXIMUM_ITEM_STACK as usize)
    }
    pub fn check_items(&self, rhs: &Self) -> bool {
        if self.id == rhs.id {
            return true;
        }
        false
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct TkItemDrop {
    pub id: ITEMIDS,
    pub amount: usize,
}
impl TkItemDrop {
    pub fn new(id: ITEMIDS, amount: usize) -> Self {
        Self { id, amount }
    }
}

#[derive(Resource)]
pub struct DemoItemsSelect {
    pub id: ITEMIDS,
    pub amount: usize,
}

impl DemoItemsSelect {
    pub fn new(id: ITEMIDS, amount: usize) -> Self {
        Self { id, amount }
    }
    pub fn into_item(&self) -> TkItems {
        TkItems::new(self.id, self.amount)
    }
}
