//! File:   tkquadtree.rs
//! Desc:   Ini adalah Plugin yang digunakan untuk algoritma spatial partitioning yang digunakan untuk meningkatkan performa dari collision check dan unit check untuk game Throwback Kingdoms

use crate::tkglobal_var;
use crate::tool::{QTDeleteConditions, QTDistributeConditions};
use crate::toolplugin::{tkphysics, TkRectangle};
use bevy::prelude::*;
use bevy_egui::egui::Ui;

use bevy_egui::{egui, EguiContexts, EguiPrimaryContextPass};

type MortonId = u64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuadtreeIndex {
    pub mortonid: MortonId,
    pub depthid: u8,
}

impl QuadtreeIndex {
    pub fn new(mortonid: MortonId, depthid: u8) -> Self {
        Self { mortonid, depthid }
    }
    fn from_id(&self, rhsid: MortonId) -> Self {
        Self {
            mortonid: (self.mortonid << 2) | rhsid,
            depthid: self.depthid + 1,
        }
    }
}

/// Quadtree itu sendiri
#[derive(Resource, Debug, Clone, PartialEq)]
pub struct TkQuadTree {
    // ini untuk Debug lebih mudah
    pub name: String,
    id: QuadtreeIndex,
    boundaries: Rect,
    tiles: Option<Vec<Entity>>,
    divided: bool,
    childnode: Option<[Box<TkQuadTree>; 4]>,
}

impl TkQuadTree {
    // // // Quadtree Management Stuffs

    /// Fungsi untuk menginisalisasi Quadtree
    pub fn new(
        name: String,
        id: QuadtreeIndex,
        border_x0: f32,
        border_y0: f32,
        border_x1: f32,
        border_y1: f32,
    ) -> Self {
        Self {
            name,
            id,
            boundaries: Rect::new(border_x0, border_y0, border_x1, border_y1),
            tiles: Some(Vec::new()),
            divided: false,
            childnode: None,
        }
    }

    //pub fn contains3_equal_option(&self, rhs: Option<Vec3>) -> bool {
    //    match rhs {
    //        Some(t) => {
    //            let pos = Vec2::new(t.x, t.y);
    //            (pos.cmpge(self.boundaries.min) & pos.cmple(self.boundaries.max)).all()
    //        }
    //        None => return false,
    //    }
    //}

    /// fungsi untuk mengecek apakah suatu vector ada dalam posisi quadtree tersebut
    pub fn contains3_equal(&self, rhs: Vec3) -> bool {
        let posi = vec2(rhs.x, rhs.y);
        // iya, boundaries itu sendiri bevy::Rect
        (posi.cmpge(self.boundaries.min) & posi.cmple(self.boundaries.max)).all()
    }

    /// Fungsi untuk memasukkan suatu entity ke dalam quadtree dengan menggunakan informasi dari tr
    /// atau posisi dari entity itu sendiri
    pub fn insert(&mut self, en: Entity) {
        // Melakukan pengecekan apakah quadtree ini memiliki posisi yang diberikan
        if let Some(tiles) = self.tiles.as_mut() {
            tiles.push(en);
            if tiles.len() > 4 {
                self.subdivide();
            }
        }
    }

    pub fn remerge(&mut self) {
        let all_child = self._take_all_children_unit();
        self.tiles = Some(Vec::new());
        if let Some(tiles) = &mut self.tiles {
            tiles.extend(all_child.into_iter());
        }
        self.divided = false;
        self.childnode = None;
    }

    /// metode untuk memindahkan nilai unit dari anakan ke dalam parent serta menghapus anakan itu
    /// sendiri
    pub fn _take_all_children_unit(&mut self) -> Vec<Entity> {
        // Inisialisasi nilai penampung return keseluruhan
        let mut return_value: Vec<Entity> = Vec::new();

        // apabila parent divided
        if self.divided {
            for i in self.childnode.as_mut().unwrap() {
                return_value.extend(i._take_all_children_unit().into_iter());
            }
        } else {
            // apabila parent tidak terdivide
            if let Some(tiles) = &self.tiles {
                return tiles.to_vec();
            }
        }
        return_value
    }

    /// # Fungsi untuk membangun tempat anakan dan mendeklarasikan diri bahwa diri telah terbagi
    pub fn subdivide(&mut self) {
        // Pertahanan untuk mengatasi masalah pada satu titik bersamaan
        if (self.boundaries.size().cmpgt(Vec2::new(0.5, 0.5))).all() {
            // tambahkan fungsi yang dapat mentrigger distribute sekali lagi
            //println!("Dividing Partition");
            self.divided = true;
            let center = self.boundaries.center();
            let (centerx, centery) = (center.x, center.y);
            let quadra: [Box<TkQuadTree>; 4] = [
                // Top Left
                Box::new(TkQuadTree::new(
                    format!("Top Left {}", self.id.depthid + 1),
                    self.id.from_id(0b01),
                    self.boundaries.min.x,
                    centery,
                    centerx,
                    self.boundaries.max.y,
                )),
                // Bottomleft
                Box::new(TkQuadTree::new(
                    format!("Bottom Left {}", self.id.depthid + 1),
                    self.id.from_id(0b00),
                    self.boundaries.min.x,
                    self.boundaries.min.y,
                    centerx,
                    centery,
                )),
                // TopRight
                Box::new(TkQuadTree::new(
                    format!("Top Right {}", self.id.depthid + 1),
                    self.id.from_id(0b11),
                    centerx,
                    centery,
                    self.boundaries.max.x,
                    self.boundaries.max.y,
                )),
                // Bottomright
                Box::new(TkQuadTree::new(
                    format!("Bottom Right {}", self.id.depthid + 1),
                    self.id.from_id(0b10),
                    centerx,
                    self.boundaries.min.y,
                    self.boundaries.max.x,
                    centery,
                )),
            ];
            self.childnode = Some(quadra)
        }
    }

    // NOTES: REWRITE
    /// Fungsi untuk melakukan distribusi pada Quadtree, bekerja dengan cara recursive melakukan
    /// pengecekan apakah memiliki anakan, dan apabila sudah mencapai titik akhir, melakukan insert
    pub fn cord_distribute(&mut self, en: Entity, tr: Vec3) {
        // mengecek terlebih dahulu apakah nilai tr ada di kotak ini atau tidak
        if self.contains3_equal(tr) {
            // mengecek apakah anakan / diri sendiri telah terbelah atau belum
            if self.divided {
                let child_node = self.childnode.as_mut().unwrap();

                // disini kita menggunakan
                for i in child_node {
                    i.cord_distribute(en, tr);
                }
                return;
            } else {
                info!("THIS SHOULD WORK {}", en);
                self.insert(en);
                return;
            }
        }
    }

    pub fn id_distribute(&mut self, en: Entity, rhsid: QuadtreeIndex) {
        if let Some(part) = self.id_get_partition_mut(rhsid, None) {
            warn!("\nOK MASUK, HARUSNYA INSERT\n");
            part.insert(en);
        }
    }

    // NOTES: REWRITE & UPDATES [x]
    // NOTES: REFACTOR BAGIAN SECOND LEVEL [ ]
    /// Fungsi untuk melakukan pengecekan pada unit apakah unit masuk pada yang berbeda
    pub fn nearest_partition(&mut self, tr: Vec3, rect: TkRectangle) -> Vec<(QuadtreeIndex, Vec3)> {
        let mut tmpr: Vec<Vec3> = Vec::new();
        let mut r: Vec<(QuadtreeIndex, Vec3)> = Vec::new();
        tmpr.push(tr); // always add current tr
        if let Some(bb) = self.cord_get_partition_mut(tr) {
            let qt_rect = bb.boundaries;
            let rect_cor = rect.unwrap_coord(&tr); // x0 y0 x1 y1
            if rect_cor[0] < qt_rect.min.x {
                if let Some(part) = self.cord_get_partition_mut(Vec3::new(rect_cor[0], tr.y, tr.z))
                {
                    tmpr.push(Vec3::new(rect_cor[0], tr.y, tr.z));

                    let buba = part.boundaries; // ini untuk semua yang ada disekitar
                    if rect_cor[1] < buba.min.y {
                        tmpr.push(Vec3::new(rect_cor[0], rect_cor[1], tr.z))
                    }
                    if rect_cor[3] > buba.max.y {
                        tmpr.push(Vec3::new(rect_cor[0], rect_cor[3], tr.z))
                    }
                }
            }
            if rect_cor[1] < qt_rect.min.y {
                if let Some(part) = self.cord_get_partition_mut(Vec3::new(tr.x, rect_cor[1], tr.z))
                {
                    tmpr.push(Vec3::new(tr.x, rect_cor[1], tr.z));

                    let buba = part.boundaries;
                    if rect_cor[0] < buba.min.x {
                        tmpr.push(Vec3::new(rect_cor[0], rect_cor[1], tr.z))
                    }
                    if rect_cor[2] > buba.max.x {
                        tmpr.push(Vec3::new(rect_cor[2], rect_cor[1], tr.z))
                    }
                }
            }
            if rect_cor[2] > qt_rect.max.x {
                if let Some(part) = self.cord_get_partition_mut(Vec3::new(rect_cor[2], tr.y, tr.z))
                {
                    tmpr.push(Vec3::new(rect_cor[2], tr.y, tr.z));

                    let buba = part.boundaries;
                    if rect_cor[1] < buba.min.y {
                        info!("Push It x1y0",);
                        tmpr.push(Vec3::new(rect_cor[2], rect_cor[1], tr.z))
                    }
                    if rect_cor[3] > buba.max.y {
                        tmpr.push(Vec3::new(rect_cor[2], rect_cor[3], tr.z))
                    }
                }
            }
            if rect_cor[3] > qt_rect.max.y {
                if let Some(part) = self.cord_get_partition_mut(Vec3::new(tr.x, rect_cor[3], tr.z))
                {
                    tmpr.push(Vec3::new(tr.x, rect_cor[3], tr.z));

                    let buba = part.boundaries;
                    if rect_cor[0] < buba.min.x {
                        tmpr.push(Vec3::new(rect_cor[0], rect_cor[3], tr.z))
                    }
                    if rect_cor[2] > buba.max.x {
                        tmpr.push(Vec3::new(rect_cor[2], rect_cor[3], tr.z))
                    }
                }
            }
        }
        for i in tmpr {
            if let Some(part) = self.cord_get_partition(i) {
                r.push((part.id, i));
            }
        }

        return r;
    }

    /// Fungsi yang digunakan untuk mendapatkan suatu partisi mutable berdasarkan posisi yang kau berikan
    /// pada parameter fungsi tersebut. hanya menerima Vec3 untuk saat ini
    pub fn get_parent_mut(&mut self, tr: Vec3) -> Option<&mut TkQuadTree> {
        if self.contains3_equal(tr) {
            // cek sekali lagi untuk memastikan jika diri sendiri benar2 parent
            if self.divided {
                // melakukan check child apakah child bercabang atau tidak
                if self.check_child_not_divided() {
                    //println!(
                    //"Get Parent Mut {} tidak memiliki anakan yang terdivide",
                    //self.name
                    //);
                    // jika tidak bercabang make return diri sendiri
                    return Some(self);
                } else {
                    // jika bercabang dan di titik yang dicari
                    if let Some(inner) = self.childnode.as_mut() {
                        // make kita akan melakukan fungsi get_parent_mut pada anakan tersebut
                        for i in inner {
                            if let Some(hasil_return) = i.get_parent_mut(tr) {
                                return Some(hasil_return);
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Fungsi untuk membandingkan dua partisi, apabila dua partisi sama make return True
    pub fn two_partition_equal(&self, old_tr: Vec3, new_tr: Vec3) -> bool {
        let mut oldname: String = "Zero".into();
        let mut newname: String = "Non_Zero".into();

        if let Some(old_part) = &self.cord_get_partition(old_tr) {
            oldname = old_part.name.clone();
        }
        if let Some(new_part) = &self.cord_get_partition(new_tr) {
            newname = new_part.name.clone();
        }

        if newname == oldname {
            return true;
        }
        false
    }

    /// Fungsi untuk mendapatkan Partisi
    pub fn cord_get_partition(&self, tr: Vec3) -> Option<&TkQuadTree> {
        // cek apakah partisi ini mengandung tr, apabila tidak return none
        if self.contains3_equal(tr) {
            // cek apakah diri sendiri divided, apabila tidak make return diri sendiri dan
            // menghentikan rekursi
            if self.divided {
                // kita akan iterasikan anakan dari quadtree ini apabila memiliki anakan
                let child_node = self.childnode.as_ref().unwrap();
                for i in child_node {
                    // kita akan iterasi tiap anakan, disini kebanyakan akan berhenti ketika
                    // pengecekan posisi / contains dari quadtree itu sendiri
                    if let Some(part) = i.cord_get_partition(tr) {
                        // tentu apabila ada make kita akan mengembalikan self
                        return Some(part);
                    }
                }
                None
            } else {
                // return diri sendiri ketika tidak memiliki anakan
                return Some(self);
            }
        } else {
            return None;
        }
    }

    /// Fungi untuk mendapatkan partisi yang mutable
    pub fn cord_get_partition_mut(&mut self, tr: Vec3) -> Option<&mut TkQuadTree> {
        // cek apakah partisi ini mengandung tr, apabila tidak return none
        if self.contains3_equal(tr) {
            // cek apakah diri sendiri divided, apabila tidak make return diri sendiri dan
            // menghentikan rekursi
            if self.divided {
                // kita akan iterasikan anakan dari quadtree ini apabila memiliki anakan
                let child_node = self.childnode.as_mut().unwrap();
                for i in child_node {
                    // kita akan iterasi tiap anakan, disini kebanyakan akan berhenti ketika
                    // pengecekan posisi / contains dari quadtree itu sendiri
                    if let Some(part) = i.cord_get_partition_mut(tr) {
                        // tentu apabila ada make kita akan mengembalikan self
                        return Some(part);
                    }
                }
                None
            } else {
                // return diri sendiri ketika tidak memiliki anakan
                return Some(self);
            }
        } else {
            return None;
        }
    }

    /// Fungsi untuk mendapatkan reference dari partisi dari qt dengan menggunakan index morton dan
    /// index depth
    pub fn id_get_partition(&self, rhsid: QuadtreeIndex) -> Option<&TkQuadTree> {
        if self.id.mortonid == rhsid.mortonid {
            return Some(self);
        }
        let mut curid = rhsid;
        if self.divided {
            let curidx: MortonId = (rhsid.mortonid >> (rhsid.depthid - 1) * 2) & 0b11;
            curid.depthid -= 1;
            match curidx {
                0b00 => {
                    return self.childnode.as_ref().unwrap()[1].id_get_partition(curid);
                }
                0b01 => {
                    return self.childnode.as_ref().unwrap()[0].id_get_partition(curid);
                }
                0b10 => {
                    return self.childnode.as_ref().unwrap()[3].id_get_partition(curid);
                }
                0b11 => {
                    return self.childnode.as_ref().unwrap()[2].id_get_partition(curid);
                }
                _ => {
                    return None;
                }
            }
        }
        return None;
    }

    /// REWRITE / FIX
    /// Fungsi untuk mendapatkan reference mutable dari partisi dari qt dengan menggunakan index morton dan
    /// index depth
    pub fn id_get_partition_mut(
        &mut self,
        rhsid: QuadtreeIndex,
        conditionalidx: Option<u8>,
    ) -> Option<&mut TkQuadTree> {
        if self.id == rhsid {
            return Some(self);
        }
        let shift: u8;
        if let Some(val) = conditionalidx {
            shift = val;
        } else {
            shift = rhsid.depthid
        }
        if self.divided {
            let curidx: MortonId = (rhsid.mortonid >> (shift) * 2) & 0b11;
            match curidx {
                0b00 => {
                    info!("-x -y");
                    return self.childnode.as_mut().unwrap()[1]
                        .id_get_partition_mut(rhsid, Some(shift));
                }
                0b01 => {
                    info!("-x +y");
                    return self.childnode.as_mut().unwrap()[0]
                        .id_get_partition_mut(rhsid, Some(shift));
                }
                0b10 => {
                    info!("+x -y");
                    return self.childnode.as_mut().unwrap()[3]
                        .id_get_partition_mut(rhsid, Some(shift));
                }
                0b11 => {
                    info!("+x +y");
                    return self.childnode.as_mut().unwrap()[2]
                        .id_get_partition_mut(rhsid, Some(shift));
                }
                _ => {
                    return None;
                }
            }
        }
        return None;
    }

    /// Fungsi untuk menghapus suatu partisi dan mengubahnya kembali menjadi partisi biasa atau
    /// leaf nodes tanpa cabang
    pub fn delete_partition(&mut self) {
        self.divided = false;
        self.childnode = None;
        self.tiles = Some(Vec::new());
        //println!("Partisi {} berhasil dihapus dan menjadi normal", self.name);
    }

    // On Process
    // Fungsi yang akan mengembalikan n partisi berdasarkan posisi titik a dan titik b serta ray
    // cast dari kedua titik tersebut. fungsi ini akan menerima dua parameter yaitu posisi saat ini
    // dan posisi yang dituju, dimana nantinya kita akan langsung mereturn
    //
    // fungsi ini ada untuk digunakan pada path finding seperti A* Algorithm
    //pub fn ray_partition(&self, tr: Vec3, rhs: Vec3) {}

    /// Fungsi untuk mengecek apakah entity ada di dalam partisi ini
    pub fn check_entity(&self, en: Entity) -> bool {
        if let Some(tile) = &self.tiles {
            if tile.contains(&en) {
                return true;
            }
        }
        false
    }

    /// Fungsi untuk menghapus suatu unit dari quadtree
    pub fn remove_unit(&mut self, en: Entity, tr: Vec3) {
        // Pencarian secara rekursif untuk mencari di mana yang harus dihapus
        if self.divided {
            for i in self.childnode.as_mut().unwrap() {
                if i.contains3_equal(tr) {
                    i.remove_unit(en, tr);
                }
            }
            // apabila sudah tidak ada lagi yang bisa di cari, make lakukan berikut
        } else {
            if self.contains3_equal(tr) {
                // Mendapatkan tiles dari quadtree itu
                if let Some(tiles) = &mut self.tiles {
                    // check apakah entitas ada di situ
                    if tiles.contains(&en) {
                        // hapus entitas dari tiles tersebut
                        tiles.retain(|value| *value != en);
                    }
                }
            }
        }
    }

    pub fn remove_unit_tiles(&mut self, en: Entity) {
        if let Some(tiles) = &mut self.tiles {
            if tiles.contains(&en) {
                // hapus entitas dari tiles tersebut
                tiles.retain(|value| *value != en);
            }
        }
    }

    /// Fungsi untuk melakukan cek apakah quadtree ini memiliki tiles atau tidak
    pub fn check_if_tiles_empty(&self) -> bool {
        // apabila tiles tidak kosong make mengembalikan nilai false
        if self.tiles != None {
            return false;
        }
        // apabila kosong, make kita return true
        true
    }

    /// ini untuk pengecekan pada suatu quadtree apakah Quadree tersebut memiliki anakan yang
    /// bercabang atau tidak.
    ///
    /// ini akan mereturn true ketika ke anakan dari partisi yang dipilih ini tidak terdivide, dan false
    /// apabila terdivide
    fn check_child_not_divided(&self) -> bool {
        if let Some(anakan) = self.childnode.as_ref() {
            let mut return_kondisi: bool = true;
            for i in anakan {
                if !i.divided {
                    return_kondisi = true;
                } else {
                    return_kondisi = false;
                    break;
                }
            }
            return return_kondisi;
        }
        false
    }

    /// Fungsi untuk mengecek apakah anakan dari suatu qt memiliki anak dan itu lebih dari 4
    /// false apabila total unit di anakan dibawah 4 (0, 1, 2, 3)
    /// dan akan true apabila lebih dari 4 (0, 1, 2, 3, 4);
    pub fn check_child_branch_exceed_four(&self) -> bool {
        let hasil = self.check_child_amount();
        //println!("\n \nJumlah tiles pada anakan adalah: {}", hasil);
        if hasil > 4 {
            return false;
        }
        true
    }
    /// Fungsi untuk mengembalikan anakan berdasarkan jumlahnya secara recursive
    fn check_child_amount(&self) -> usize {
        let mut return_amount: usize = 0;
        //println!("\nCheck Child partisi: {}", self.name);
        if self.divided {
            for i in self.childnode.as_ref().unwrap() {
                return_amount += i.check_child_amount()
            }
        } else {
            if let Some(tiles) = &self.tiles {
                //println!(
                //    "partisi {} memiliki tiles dengan jumlah: {}",
                //self.name,
                //    tiles.len()
                //);
                return_amount += tiles.len();
            }
        }
        return_amount
    }

    // // // Physics Stuff // // //

    /// fungsi untuk mendapatkan semua entity dalam partisi dari quadtree
    fn recursive_entity_get(&self) -> Option<&Vec<Entity>> {
        if !self.divided {
            return self.tiles.as_ref();
        }
        return None;
    }
    /// fungsi untuk mendapatkan semua entity (tiles) dan return dalam bentuk vector yang
    /// digabungkan
    pub fn get_all_entity(&self) -> Option<Vec<&Vec<Entity>>> {
        // Temporary Values untuk stores semua tiles
        let mut temp: Vec<&Vec<Entity>> = Vec::new();
        if !self.divided {
            // Apabila Tiles Kosong, make return None
            if self.check_if_tiles_empty() {
                return None;
            }
            // Apabila Tiles ada, make return vec wrapped
            temp.push(self.tiles.as_ref().unwrap());
            return Some(temp);
        }
        // pengecekan setiap anakan (rekursif)
        for nil in self.childnode.as_ref().unwrap() {
            // apabila ada nilainya, make iterasikan
            if let Some(all_en) = nil.get_all_entity() {
                // unwrap vec wrap nya
                for i in all_en {
                    // gabungkan vec tiles yang ada dengan temp saat ini
                    temp.push(i)
                }
            }
        }
        return Some(temp);
    }
}

#[derive(Component)]
pub struct QuadtreeUnit;

// NOTES: UPDATE
#[derive(Component, Debug)]
pub struct QuadtreeUnitPosition {
    pub new_val: (Option<Vec<QuadtreeIndex>>, Option<Vec<Vec3>>),
    pub old_val: (Option<Vec<QuadtreeIndex>>, Option<Vec<Vec3>>),
}

impl QuadtreeUnitPosition {
    pub fn new() -> Self {
        Self {
            new_val: (None, None),
            old_val: (None, None),
        }
    }
    pub fn replace_old_id(&mut self, rhs_id: Vec<QuadtreeIndex>) {
        match self.old_val.0 {
            Some(_) => *self.old_val.0.as_mut().unwrap() = rhs_id,
            None => self.old_val.0 = Some(rhs_id),
        }
    }
    pub fn replace_old_pos(&mut self, rhs_pos: Vec<Vec3>) {
        match self.old_val.1 {
            Some(_) => *self.old_val.1.as_mut().unwrap() = rhs_pos,
            None => self.old_val.1 = Some(rhs_pos),
        }
    }
    pub fn push_old_id(&mut self, id: Vec<QuadtreeIndex>) {
        if let Some(selfid) = self.old_val.0.as_mut() {
            for i in id {
                selfid.push(i)
            }
        }
    }
    pub fn push_old_pos(&mut self, pos: Vec<Vec3>) {
        if let Some(selfid) = self.old_val.1.as_mut() {
            for i in pos {
                selfid.push(i)
            }
        }
    }
    pub fn assign_values_id(&mut self, rhs_id: Vec<QuadtreeIndex>) {
        match self.old_val.0 {
            Some(_) => match self.new_val.0 {
                Some(_) => {
                    println!("\n \n");
                    error!("Holy Shit, kok nambah Id");
                    panic!("Holy Shit, kok nambah Id {:?}", rhs_id)
                }
                None => {
                    self.new_val.0 = Some(rhs_id);
                }
            },
            None => {
                self.old_val.0 = Some(rhs_id);
            }
        }
    }
    pub fn assign_values_pos(&mut self, rhs_pos: Vec<Vec3>) {
        match self.old_val.1 {
            Some(_) => match self.new_val.1 {
                Some(_) => {
                    println!("\n \n");
                    error!("Holy Shit, kok nambah Pos");
                    panic!("Holy Shit, kok nambah Pos {:?}", rhs_pos)
                }
                None => {
                    self.new_val.1 = Some(rhs_pos);
                }
            },
            None => {
                self.old_val.1 = Some(rhs_pos);
            }
        }
    }
    /// Fungsi dengan tujuan untuk menghapus value lama dan mengubah value baru menjadi value llama
    /// old_val = new_val; new_val = None
    pub fn update_values_id(&mut self) {
        if self.new_val.0 != None {
            self.old_val.0 = self.new_val.0.clone();
            self.new_val.0 = None;
        } else {
        }
    }
    pub fn update_values_pos(&mut self) {
        if self.new_val.1 != None {
            self.old_val.1 = self.new_val.1.clone();
            self.new_val.1 = None;
        } else {
        }
    }

    pub fn update_values_both(&mut self) {
        self.update_values_pos();
        self.update_values_id();
    }

    /// something wrongs, i can feel it
    pub fn compare_values_id(&self, v: &Vec<QuadtreeIndex>) -> bool {
        let val = self.old_val.0.as_ref().unwrap(); // NOTES: INGAT, KARENA INI UNWRAP: PASTIKAN NILAINYA
                                                    // ADA DI WAKTU PERTAMA KALI MEMASUKKAN UNIT PADA
                                                    // QUADTREE
        return val != v;
    }
}

#[derive(Component)]
pub struct QuadtreeUnitStates {
    pub isonborder: bool,
    pub leaveborder: bool,
}

impl QuadtreeUnitStates {
    pub fn new() -> Self {
        Self {
            isonborder: false,
            leaveborder: false,
        }
    }
    pub fn offborder(&mut self) {
        self.isonborder = false;
    }
    pub fn justin(&mut self) {
        self.isonborder = true;
        self.leaveborder = true
    }
    pub fn justleaves(&mut self) {
        self.leaveborder = false
    }
}

// // // // PLUGIN // // // //

// Plugin yang digunakan untuk memasukkan QuadtreeUnit kedalam Resource Quadree
pub struct TkQuadTreePlugin;
impl Plugin for TkQuadTreePlugin {
    fn build(&self, app: &mut App) {
        // TO FIX: PEMILIHAN WORLD SIZENYA NANTI SAJA
        app.insert_resource(TkQuadTree::new(
            "Root".into(),
            QuadtreeIndex::new(0b00, 0),
            -200.0,
            -200.0,
            200.0,
            200.0,
        )); // Init the quadtree
        app.add_systems(
            Update,
            (
                (unit_to_quadtree, update_quadtree_unit).chain(),
                draw_quadtree, // ini untuk menunjukkan quadtree tersebut
            ),
        );
        app.add_systems(
            EguiPrimaryContextPass,
            (show_current_quadtree_unit, show_current_qtup),
        );
        app.add_observer(qt_distribute_child);
        app.add_observer(qt_delete_child);
    }
}

// Call Once
// NOTES: UPDATE
fn unit_to_quadtree(
    mut qt: ResMut<TkQuadTree>,
    mut unit_entity: Query<
        (
            Entity,
            &Transform,
            &mut QuadtreeUnitPosition,
            &tkphysics::TkRectangle,
        ),
        (With<QuadtreeUnit>, Added<QuadtreeUnit>),
    >,
    mut command: Commands,
) {
    for (en, tr, mut qtup, rect) in &mut unit_entity {
        // workflow :
        //  1. dapatkan semua nilai, baik diri sendiri ataupun partisi sekitar
        //  2. assign pada qtup
        //  3. trigger distribusi untuk insert dan distribusi

        let (valid, valpos): (Vec<QuadtreeIndex>, Vec<Vec3>) = qt
            .nearest_partition(tr.translation, *rect)
            .into_iter()
            .unzip(); // if found

        qtup.assign_values_pos(valpos.clone());
        qtup.assign_values_id(valid);
        command.trigger(QTDistributeConditions::new(en, valpos));
    }
}

// NOTES: REWRITE   [x]
// NOTES: DEBUG     [ ]
/// Fungsi ini ada untuk mengupdate posisi dari apapun yang memiliki komponen QuadtreeUnit dimana
/// ketika komponen itu bergerak keluar dari suatu partisi, make fungsi ini akan menghapus
/// keberadaan dari dirinya di partisi sebelumnya lalu menambahkan keberadaannya pada partisi baru
fn update_quadtree_unit(
    mut qr: Query<
        (
            Entity,
            &Transform,
            &mut QuadtreeUnitPosition,
            &tkphysics::TkRectangle,
            &mut QuadtreeUnitStates,
        ),
        (With<QuadtreeUnit>, Changed<Transform>), // CHANGED SANGAT PENTING
    >,
    mut qt: ResMut<TkQuadTree>,
    mut command: Commands,
) {
    // iterasikan query
    for (en, tr, mut qtup, rect, mut qtus) in &mut qr {
        if let Some(part) = qt.cord_get_partition_mut(tr.translation) {
            let baba = part.boundaries;
            let record = rect.unwrap_coord(&tr.translation);

            if record[0] < baba.min.x // cek apakah kotak dari unit ada yang tabrakan
                || record[1] < baba.min.y
                || record[2] > baba.max.x
                || record[3] > baba.max.y
            {
                if !qtus.leaveborder {
                    qtus.leaveborder = true;
                }
                // GET POS ID AND POS
                let (valid, valpos): (Vec<QuadtreeIndex>, Vec<Vec3>) = qt
                    .nearest_partition(tr.translation, *rect)
                    .into_iter()
                    .unzip(); // if found

                if qtup.compare_values_id(&valid) {
                    info!("\nCompared Values Is Differents");
                    qtup.assign_values_id(valid);
                    qtup.assign_values_pos(valpos.clone());
                    command.trigger(QTDistributeConditions::new(en, valpos));
                }
            } else {
                if qtus.leaveborder {
                    //info!("THINGS A MA BOB {en} OUT FROM BORDOR");
                    if let Some(indx) = qr_last_check(&mut qt, tr, rect, &mut qtup) {
                        command.trigger(QTDistributeConditions::new(en, indx));
                    }
                    qtus.leaveborder = false;
                }
            }
        }
    }
}

fn qr_last_check(
    qt: &mut TkQuadTree,
    tr: &Transform,
    rect: &tkphysics::TkRectangle,
    qtup: &mut QuadtreeUnitPosition,
) -> Option<Vec<Vec3>> {
    let (valid, valpos): (Vec<QuadtreeIndex>, Vec<Vec3>) = qt
        .nearest_partition(tr.translation, *rect)
        .into_iter()
        .unzip(); // if found

    if qtup.compare_values_id(&valid) {
        info!("\nCompared Values Is Differents");
        qtup.assign_values_id(valid);
        qtup.assign_values_pos(valpos.clone());
        return Some(valpos);
    }
    None
}

// NOTES: REWRITE   [x]
// NOTES: DEBUG     [ ]
/// fungsi yang akan  mendistribusikan suatu anakan ketika terjadi
fn qt_distribute_child(
    mut qtdc: On<tkglobal_var::QTDistributeConditions>,
    mut qt: ResMut<TkQuadTree>,
    mut command: Commands,
    mut qr: Query<
        (
            Entity,
            &mut QuadtreeUnitPosition,
            &Transform,
            &tkphysics::TkRectangle,
        ),
        With<QuadtreeUnit>,
    >,
) {
    // Pastikan Insert terlebih dahulu

    for qtdc_pos in qtdc.pos.clone() {
        if let Some(part) = qt.cord_get_partition_mut(qtdc_pos) {
            if !part.check_entity(qtdc.en) {
                info!("\n\nFIRST TIME INSERTION {} \n\n", qtdc.en);
                part.cord_distribute(qtdc.en, qtdc_pos);
                if let Ok((_, mut qtup, _, _)) = qr.get_mut(qtdc.en) {
                    qtup.replace_old_id(vec![part.id]);
                }
            }
        }
        // -------- //

        // WHAT THIS THING DO:
        // mengecek semua nilai yang ada di qtdc_pos lalu lakukan pengecekan tingkat
        // partisi untuk melihat apakah nilainya lebih dari 4, apabila iya maka get all
        // unit dari partisi itu la lu distribute pada anakan dari partisi
        let mut _retvecen: Vec<Entity> = Vec::new();
        if let Some(sqt) = __search_qt_to_distribute(&mut qt, qtdc_pos) {
            println!("\n\n\n\n\n==================================");
            error!("SQT TILES: {:?}", sqt.tiles);
            let tiles = sqt.tiles.as_ref().unwrap();
            for i in tiles {
                _retvecen.push(*i);
            }
            //let old_pos = qtup.old_val.1.as_ref().unwrap(); // Unwrap

            //_retvecen.push(en);
            //_retvecpos.push(tr.translation);
            sqt.tiles = None;
        }
        for i in 0.._retvecen.len() {
            if let Ok((_, mut qtup, tr, rect)) = qr.get_mut(_retvecen[i]) {
                let (valid, valpos): (Vec<QuadtreeIndex>, Vec<Vec3>) = qt
                    .nearest_partition(tr.translation, *rect)
                    .into_iter()
                    .unzip(); // if found
                info!(
                    "EN: {}, VALPOS: {:?}, VALID: {:?}",
                    _retvecen[i], valpos, valid
                );
                for jpos in &valpos {
                    if let Some(papart) = qt.cord_get_partition_mut(*jpos) {
                        //warn!(
                        //    "\n\nPAPART TILES BEFORE: {:?}, Name: {}, ID: {:?}",
                        //    papart.tiles, papart.name, papart.id
                        //);
                        //info!(
                        //    "DISTRIBUTE INSERTION {}, NAME: {} , ID: {:?}",
                        //    _retvecen[i], papart.name, papart.id
                        //);
                        if !papart.check_entity(_retvecen[i]) {
                            papart.cord_distribute(_retvecen[i], *jpos);
                            //warn!(
                            //    "PAPART TILES AFTER: {:?}, Name: {}, ID: {:?}\n\n",
                            //    papart.tiles, papart.name, papart.id
                            //);
                        }
                    }
                }
                if qtup.new_val.0 == None {
                    qtup.replace_old_id(valid);
                }
                if qtup.new_val.1 == None {
                    qtup.replace_old_pos(valpos)
                }
            }
        }
    }

    //--------
    if let Ok((_, mut qtup, _, _)) = qr.get_mut(qtdc.en) {
        // Dapatkan Vec yang dari old value yang tidak ada di new value
        if let Some(qtup_new_val) = &qtup.new_val.0 {
            let qtup_old_id = qtup.old_val.0.as_ref().unwrap();
            let qtup_old_pos = qtup.old_val.1.as_ref().unwrap();
            let mut r: Vec<Vec3> = Vec::new();
            for i in 0..qtup_old_id.len() {
                if !qtup_new_val.contains(&qtup_old_id[i]) {
                    r.push(qtup_old_pos[i])
                }
            }
            info!(
                "------------\nSEND THIS TO DELETE: EN: {}, R: {:?}\n",
                qtdc.en, r
            );
            command.trigger(QTDeleteConditions::new(qtdc.en, r));
            qtup.update_values_both();
        }
    }

    //// ini untuk mendapatkan nilai dari quadtree yang meminta untuk dilakukan distribute
    //for qtdc_pos in qtdc.pos.clone() {
    //    // ini untuk mendapatkan quadtree yang di cari untuk di distribute
    //    //let sqt = __search_qt_to_distribute(&mut qt, qtdc_pos);
    //    for (en, tr, mut qtup, rect) in &mut qr {
    //        let mut val = qt.nearest_partition(en, tr.translation, *rect);
    //        if let Some(sqt) = __search_qt_to_distribute(&mut qt, qtdc_pos) {
    //            // kemudian kita mengiterasikan setiap anakannya lalu kita menghapus tiles itu sendiri
    //            if sqt.check_entity(en) {
    //                if let Some(pos) = sqt.distribute(en, tr.translation) {
    //                    val.push(pos);
    //                    if let Some(old_pos) = qtup.old_val.clone() {
    //                        for i in 0..old_pos.len() {
    //                            if sqt.contains3_equal(old_pos[i]) {
    //                                //println!("\n \n \n {en} Nomor 1");
    //                                qtup.replace_old(val.clone());
    //                            } else {
    //                                if !sqt.contains3_equal(old_pos[i]) {}
    //                            }
    //                        }
    //                    } else {
    //                        qtup.replace_old(val);
    //                        //println!("\n \n \n {en} Nomor 2");
    //                    }
    //                }
    //            }
    //            // menghapus tile untuk menunjukkan jika partition yang sudah terdivide tidak boleh punya tiles
    //            // lagi selain anakan
    //            sqt.tiles = None;
    //        }
    //    }
    //    qtdc.clear();
    //}
}

/// fungsi yang berjalan secara recursive untuk mencari anakan sesuai dengan Transform
fn __search_qt_to_distribute(qt: &mut TkQuadTree, tr: Vec3) -> Option<&mut TkQuadTree> {
    // Ketika ini divided tapi masih memiliki nilai
    if qt.divided && !qt.check_if_tiles_empty() {
        //println!("Search Qt to Distribute: Menemukan partisi untuk di distribute");
        return Some(qt);
    }
    // Ketika ini divided tapi tidak ada nilai didalamnya
    else if qt.divided && qt.check_if_tiles_empty() {
        // kita akan mengecek setiap anakannya apakah setiap kotak anakannya dapat menampung tr
        for i in qt.childnode.as_mut().unwrap() {
            if i.contains3_equal(tr) {
                return __search_qt_to_distribute(i, tr);
            }
        }
    }

    None
}

// NOTES: REWRITE [x]
// NOTES: DEBUG [ ]
/// Ini adalah fungsi untuk menghapus partisi pada suatu partisi di quadtree serta menghapus nilai
/// dari suatu point pada partisi lamanya
fn qt_delete_child(
    mut qtdec: On<tkglobal_var::QTDeleteConditions>,
    mut qr: Query<(Entity, &mut QuadtreeUnitPosition), With<QuadtreeUnit>>,
    mut qt: ResMut<TkQuadTree>,
) {
    for qtdec_pos in qtdec.tr.clone() {
        info!(
            "\n\nQTDEC GET INFO; En: {}, x: {}, y: {}, z: {}\n\n",
            qtdec.en, qtdec_pos.x, qtdec_pos.y, qtdec_pos.z
        );
        if let Some(part) = __search_unit_to_delete(&mut qt, qtdec_pos) {
            part.remove_unit(qtdec.en, qtdec_pos);
        }
        if let Some(papart) = qt.get_parent_mut(qtdec_pos) {
            if papart.check_child_branch_exceed_four() {
                warn!("REMERGE HAPPENS");
                papart.remerge();
            }
        }
    }

    // apabila qtdec memiliki posisi, make dilanjutkan
    //for qtdec_pos in qtdec.pos.clone() {
    //    let to_delete = __search_unit_to_delete(&mut qt, qtdec_pos);
    //
    //    // REWRITE: SUMBER ERROR MENGHAPUS SEMUA UNIT LAINNYA
    //    for (en, mut qtup) in &mut qr {
    //        for check_deleting_nodes in &to_delete {
    //            if check_deleting_nodes == &en {
    //                // pada mereka yang memiliki 2 values posisi
    //                if qtup.new_val != None {
    //                    // NOTE: Ini sumber masalahnya, kita harus menambahkan fungsi untuk
    //                    // mengecek kedua titik partisi apakah sama
    //                    // Hapus node
    //                    for i in 0..qtup.old_val.as_ref().unwrap().len() {
    //                        if !qt.two_partition_equal(
    //                            qtup.old_val.as_ref().unwrap()[i],
    //                            qtup.new_val.as_ref().unwrap()[i],
    //                        ) {
    //                            qt.remove_unit(en, qtup.old_val.as_ref().unwrap()[i]);
    //                        }
    //                        // cari partisi parentnya
    //                        if let Some(part) = qt.get_parent_mut(qtup.old_val.as_ref().unwrap()[i])
    //                        {
    //                            //println!(
    //                            //    "Mendapatkan Partisi berupa: {}======================",
    //                            //    part.name
    //                            //);
    //
    //                            // apabila anakan dari parentnya ada di atas 4
    //                            if !part.check_child_branch_exceed_four() {
    //                                //println!("{} memiliki jumlah anakan diatas 4", part.name);
    //                            } else {
    //                                //println!("{} memiliki jumlah anakan dibawah 4", part.name);
    //                                // Make kita akan meremerge semua anakan di bawahnya untuk masuk
    //                                // menjadi ankan dari parent itu sendiri
    //                                part.remerge();
    //                            }
    //                        }
    //                        // update qtup
    //                    }
    //                    qtup.update_values();
    //                }
    //            }
    //        }
    //        qtdec.clear();
    //    }
    //    // mencari posisi dari partisi yang terjadi perpindahan itu sendiri
    //}
}

fn __search_unit_to_delete(qt: &mut TkQuadTree, id: Vec3) -> Option<&mut TkQuadTree> {
    if let Some(part) = qt.cord_get_partition_mut(id) {
        if let Some(_) = &part.tiles {
            return Some(part);
        }
    }
    None
}

/// Fungsi untuk menggambar border dari quadtree dengan menggunakan gizmo
fn draw_quadtree(qt: Res<TkQuadTree>, mut gizmos: Gizmos) {
    gizmos.rect_2d(
        qt.boundaries.center(),
        qt.boundaries.size(),
        Color::linear_rgb(0.0, 0.0, 1.0),
    );
    if qt.divided {
        for i in qt.childnode.as_ref().unwrap() {
            draw_quadtree_gizmos(&i, &mut gizmos);
        }
    }
}

fn draw_quadtree_gizmos(qt: &TkQuadTree, gizmos: &mut Gizmos) {
    gizmos.rect_2d(
        qt.boundaries.center(),
        qt.boundaries.size(),
        Color::linear_rgb(0.0, 0.0, 1.0),
    );
    if qt.divided {
        for i in qt.childnode.as_ref().unwrap() {
            draw_quadtree_gizmos(&i, gizmos);
        }
    }
}

fn show_current_quadtree_unit(mut contest: EguiContexts, qt: Res<TkQuadTree>) -> Result {
    egui::Window::new("Quadtree").show(contest.ctx_mut()?, |ui| {
        recursively_show_unit(&qt, ui);
    });
    Ok(())
}

fn recursively_show_unit(qt: &TkQuadTree, ui: &mut Ui) {
    ui.collapsing(format!("{}", qt.name), |ui| {
        if qt.divided {
            if let Some(childnode) = &qt.childnode {
                for i in childnode {
                    recursively_show_unit(i, ui);
                }
            }
        } else {
            if let Some(tiles) = &qt.tiles {
                for i in tiles {
                    ui.label(format!("{:?}", i));
                }
            }
        };
    });
}

fn show_current_qtup(
    mut contest: EguiContexts,
    qr: Query<(Entity, &QuadtreeUnitPosition)>,
) -> Result {
    egui::Window::new("Entity & QTUP").show(contest.ctx_mut()?, |ui| {
        for (en, qtup) in qr {
            ui.label(format!("En:{}, qtup: {:?}", en, qtup));
        }
    });
    Ok(())
}
