//! FILES       :   tkinventory.rs
//! DESCRIPTION :   FILES PENAMPUNG FUNGSI DAN COMPONENT UNTUK SYSTEM inventory

use crate::{
    tkentities, tkitems, tkphysics,
    tool::{tkglobal_var, CurrentId},
    toolplugin::TkItems,
};
use bevy::prelude::*;
use bevy_egui::EguiContexts;

// // // // // // // // // //
// // // COMPONENT // // //
// // // // // // // // // //

/// Struct Component inventory yang akan dipegang oleh semua unit dengan system inventory dimana
/// memberikan akses slot untuk semua
#[derive(Clone, Component, Debug)]
pub struct TkInventory {
    pub slot_amount: u8,
    pub slot: Vec<tkitems::TkItems>,
}

impl TkInventory {
    pub fn new(slot_amount: u8) -> Self {
        Self {
            slot_amount,
            slot: Vec::new(),
        }
    }

    /// Fungsi untuk mengecek apakah ukuran slot lebih kecil daripada ukuran maksimal slotnya
    fn check_slot_size(&self) -> bool {
        if self.slot.len() < self.slot_amount.into() {
            return true;
        }
        false
    }

    /// Fungsi untuk update jumlah slot maximum.
    /// fungsi ini akan dipanggil ketika update inventory / backpack dilakukan
    pub fn extend_maximum_slot() {}

    /// fungsi untuk melakukan cek apakah ada suatu items di slot, apabila ada make
    /// ia akan mereturn index dan juga stacks sisanya
    /// apabila stack sisa negatif, make lakukan operasi penambahan serta append pada slots
    fn check_contains_item(&self, items: &tkitems::TkItems) -> Option<(usize, u8)> {
        for i in 0..self.slot.len() {
            // apabila ada ataupun non None, make dapatkan sinyal
            if let Some(chk) = items.check_items(&self.slot[i]) {
                return Some((i, chk));
            }
        }
        return None;
    }

    /// fungsi untuk menambahkan data items pada yang sudah ada
    pub fn append_items_to_items(&mut self, idx: usize, items: &tkitems::TkItems) {
        println!("BOKEP BOKEP BOKEP BOKEP");
        // copy shits bruh
    }

    /// fungsi untuk menambahkan data items pada slots kosong, return bool untuk memberikan
    /// informamsi terkait keberhasilan proses fungsi, false apabila slot penuh
    pub fn append_items_to_slots(&mut self, items: &tkitems::TkItems) -> bool {
        // Cek apabila masih ada ruang kosong
        if self.check_slot_size() {
            self.slot
                .push(tkitems::TkItems::new(items.id, items.amount));
            return true;
        }
        return false;
    }
}

/// Komponen yang menyimpan informasi terkait apakah unit dengan inventory ini merupakan unit
/// shared inventory. komponen ini hanya menerima satu parameter yaitu size dengan input Vec2.
// NOTE: Ada Kemungkinan saya akan menggantinya dari kotak menjadi bentuk lingkaran dengan
// memanfaatkan rumus lingkaran. mungkin itu akan jadi lebih gampang karena itu hanya mengecek
// apakah suatu unit ada **Didalam** lingkaran itu
//#[derive(Copy, Clone, Component, Debug)]
//pub struct TkSharedInventory(Vec2);

// // // // // // // // //
// // // PLUGINS // // //
// // // // // // // // //

pub struct TkInventoryPlugins;
impl Plugin for TkInventoryPlugins {
    fn build(&self, app: &mut App) {
        // Implementation
        app.add_systems(Update, debug_print_invslot);
        app.add_observer(test_items_collision);
    }
}

// // // // // // // // // // //
// // // IMPLEMENTATION // // //
// // // // // // // // // // //

/// Fungsi untuk memasukkan suatu item ke dalam inventory karakter
/// tentu ini perlu prerequisites berupa Quadtree itu sendiri serta pengecekan collision untuk
/// mengecek apakah item sudah masuk ke dalam area pengumpulan karakter
pub fn insert_item_to_inventory(qr: Query<&mut TkInventory>) {}

/// Fungsi untuk (Test) memasukkan suatu item ke dalam inventory karakter
/// tentu ini perlu prerequisites berupa Quadtree itu sendiri serta pengecekan collision untuk
/// /// mengecek apakah item sudah masuk ke dalam area pengumpulan karakter. sehingga untuk fungsi tes
/// ini kita tidak akan menggunakan collision itu terlebih dahulu

pub fn test_items_collision(
    mut invc: On<tkphysics::ItemCollisionEventHandle>,
    mut qritem: Query<(Entity, &mut tkitems::TkItems)>,
    mut qrinv: Query<(Entity, &mut TkInventory)>,
    mut command: Commands,
    //qrunit: Query<(Entity, &mut TkInventory)>,
) {
    if let Some(itemada) = invc.itemen {
        if let Ok((item_en, mut items)) = qritem.get_mut(itemada) {
            if let Ok((_, mut inv)) = qrinv.get_mut(invc.uniten) {
                // Cek Inventory apabila ada
                if let Some((it_index, remainder)) = inv.check_contains_item(&items) {
                    if remainder != 0 {
                        // apabila lebih
                        if let Some(split_items) = items.split_amount(remainder) {
                            inv.append_items_to_items(it_index, &split_items);
                            if (inv.append_items_to_slots(&items)) {
                                command.entity(itemada).despawn();
                            }
                        }
                    } else {
                        // Apabila == 0
                        inv.append_items_to_items(it_index, &items);
                        command.entity(itemada).despawn();
                    }
                } else {
                    // apabila tidak ada items sama sekali
                    if inv.slot.len() < inv.slot_amount.into() {
                        if inv.append_items_to_slots(&items) {
                            command.entity(itemada).despawn();
                        }
                    }
                }
                command.trigger(tkglobal_var::InventoryItemInserts);
            }
        }
    }
}

// Membuat system untuk melihat inventory dengan menggunakan egui
fn debug_show_inventoryzero(
    mut contest: EguiContexts,
    qr: Query<(&tkentities::DynamicHeroId, &TkInventory)>,
    gid: Res<CurrentId>,
) {
    for (id, inv) in qr {
        if id.id == gid.id {}
    }
}

/// Debug function untuk menerima input "t" lalu print setiap item yang ada di inventory slot pada
/// character yang dipilih
fn debug_print_invslot(
    key: Res<ButtonInput<KeyCode>>,
    qr: Query<(&tkentities::DynamicHeroId, &TkInventory)>,
    curid: Res<tkglobal_var::CurrentId>,
) {
    if key.just_pressed(KeyCode::KeyT) {
        for (id, inv) in &qr {
            if id.id == curid.id {
                info!("Items Terdiri Dari {:?}", inv.slot);
            }
        }
    }
}
