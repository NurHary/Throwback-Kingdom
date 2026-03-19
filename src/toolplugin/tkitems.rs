use crate::toolplugin::tkinventory;
use bevy::prelude::*;

const MAXIMUM_ITEM_STACK: u8 = 99;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ITEMIDS {
    Wood,
    Stone,
    Fiber,
}

/// Fungsi untuk mendapatkan index atlas dari item id (x)
pub fn item_conversion_index(id: ITEMIDS) -> usize {
    match id {
        ITEMIDS::Wood => return 0,
        ITEMIDS::Stone => return 1,
        ITEMIDS::Fiber => return 2,
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub struct TkItems {
    pub id: ITEMIDS,
    pub amount: u8,
}
impl TkItems {
    pub fn new(id: ITEMIDS, amount: u8) -> Self {
        Self { id, amount }
    }
    pub fn add_amount(&mut self, amount: u8) -> (bool, u8) {
        if (self.amount + amount) <= MAXIMUM_ITEM_STACK {
            self.amount += amount;
            // apabila bisa ditambah, make kita tidak perlu melakukan aksi selanjutnya
            return (false, 0);
        }
        // apabila tidak bisa ditambah, make kita perlu melakukan aksi selanjutnya
        self.amount += amount - ((self.amount + amount) - MAXIMUM_ITEM_STACK);
        (true, (self.amount + amount) - MAXIMUM_ITEM_STACK)
    }

    pub fn split_amount(&mut self, a: u8) -> Option<Self> {
        if a < self.amount {
            self.amount = a;
            return Some(Self {
                id: self.id,
                amount: self.amount - a,
            });
        }
        return None;
    }

    /// fungsi untuk mengecek apakah item sama, dan apabila iya make return selisih sisa
    /// ditampung (if (self.amnt + rhs.amnt > MAX) ? MAX - total.amnt : 0 )
    pub fn check_items(&self, rhs: &Self) -> Option<u8> {
        // Guard sehingga tidak akan dilakukan pengecekan ketika item sudah penuh
        if self.amount == MAXIMUM_ITEM_STACK {
            return None;
        }
        if self.id == rhs.id {
            // ini ada supaya disana tahu kalau item ini ada berapa jumlah yang bisa di tambahkan,
            // apabila lebih make return sisa untuk dibangun item dan insert lagi
            if self.amount + rhs.amount > MAXIMUM_ITEM_STACK {
                return Some(MAXIMUM_ITEM_STACK - (self.amount + rhs.amount));
            }
            return Some(0);
            //return Some(MAXIMUM_ITEM_STACK - rhs.amount);
        }
        return None;
    }
}
