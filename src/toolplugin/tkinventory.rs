use crate::toolplugin::tkitems;
use bevy::prelude::*;

use crate::*;

/// Struct inventory yang akan dipegang oleh semua unit dengan system inventory
#[derive(Clone, Component)]
pub struct TkInventory {
    slot_amount: usize,
    slot: Vec<tkitems::TkItems>,
}

impl TkInventory {
    pub fn new(slot_amount: usize) -> Self {
        Self {
            slot_amount,
            slot: Vec::new(),
        }
    }

    /// Fungsi untuk mengecek apakah ukuran slot lebih kecil daripada ukuran maksimal slotnya
    fn check_slot_size(&self) -> bool {
        if self.slot.len() < self.slot_amount {
            return true;
        }
        false
    }

    /// Fungsi untuk update jumlah slot maximum.
    /// fungsi ini akan dipanggil ketika update inventory / backpack dilakukan
    pub fn extend_maximum_slot() {}

    /// Fungnsi untuk menambahkan
    fn get_slot_to_add(&mut self, items: tkitems::TkItems) -> Vec<&mut tkitems::TkItems> {
        let mut return_value: Vec<&mut tkitems::TkItems> = Vec::new();
        for i in &mut self.slot {
            if items.check_items(i) {
                return_value.push(i);
            }
        }
        return_value
    }

    /// fungsi untuk mengecek vector slot pakah contain items tersebut, return true apabila ada,
    /// return false apabila tidak
    fn check_contains_item(&self, items: tkitems::TkItems) -> bool {
        for i in &self.slot {
            return items.check_items(i);
        }
        false
    }
}

/// Komponen yang menyimpan informasi terkait apakah unit dengan inventory ini merupakan unit
/// shared inventory. komponen ini hanya menerima satu parameter yaitu size dengan input Vec2.
// NOTE: Ada Kemungkinan saya akan menggantinya dari kotak menjadi bentuk lingkaran dengan
// memanfaatkan rumus lingkaran. mungkin itu akan jadi lebih gampang karena itu hanya mengecek
// apakah suatu unit ada **Didalam** lingkaran itu
#[derive(Copy, Clone, Component, Debug)]
pub struct TkSharedInventory(Vec2);

/// Fungsi untuk memasukkan suatu item ke dalam inventory karakter
/// tentu ini perlu prerequisites berupa Quadtree itu sendiri serta pengecekan collision untuk
/// mengecek apakah item sudah masuk ke dalam area pengumpulan karakter
pub fn insert_item_to_inventory(qr: Query<&mut TkInventory>) {}

/// Fungsi untuk (Test) memasukkan suatu item ke dalam inventory karakter
/// tentu ini perlu prerequisites berupa Quadtree itu sendiri serta pengecekan collision untuk
/// /// mengecek apakah item sudah masuk ke dalam area pengumpulan karakter. sehingga untuk fungsi tes
/// ini kita tidak akan menggunakan collision itu terlebih dahulu
pub fn test_insert_item_to_inventory(
    qr: Query<(Entity, &mut TkInventory, &HeroesId)>,
    key: Res<ButtonInput<KeyCode>>,

    // Variable dari item yang akan dimasukkan
    mut item_select: ResMut<tkitems::DemoItemsSelect>,
    current_id: Res<CurrentId>,
    mut invdsys: ResMut<InvDSys>,
) {
    if key.just_pressed(KeyCode::Digit1) {
        item_select.id = tkitems::ITEMIDS::Wood
    }
    if key.just_pressed(KeyCode::Digit2) {
        item_select.id = tkitems::ITEMIDS::Stone
    }
    if key.just_pressed(KeyCode::Digit3) {
        item_select.id = tkitems::ITEMIDS::Fiber
    }
    for (en, mut inv, id) in qr {
        if id.id.lock().unwrap().value == current_id.id {
            // Apabila P di klik make aktifkan
            if key.just_pressed(KeyCode::KeyP) {
                // insert item to inventory
                if inv.check_contains_item(item_select.into_item()) {
                    for i in &mut inv.slot {
                        if i.check_items(&item_select.into_item()) {
                            let (condition, distr) = i.add_amount(item_select.amount);
                            if condition {
                                // NOTE: Unfinished
                                invdsys.activate(item_select.id, distr);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn distribute_items(mut invdsys: ResMut<InvDSys>) {}
