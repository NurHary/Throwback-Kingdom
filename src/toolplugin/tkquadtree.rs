//! Ini adalah Plugin yang digunakan untuk algoritma spatial partitioning yang digunakan untuk

use crate::global_var::*;
use bevy::prelude::*;
use bevy_egui::egui::epaint::EllipseShape;
use bevy_egui::egui::Ui;

use crate::tool::qt_delete;
use crate::tool::{qt_distribute, QTDeleteConditions};

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPrimaryContextPass};

//#[derive(Debug, Clone, PartialEq)]
//struct TkBoundaries {
//    min: Vec2,
//    max: Vec2,
//}
//
///// TkRectangle untuk contains lebih simple
//impl TkBoundaries {
//    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
//        Self::from_corners(Vec2::new(x0, y0), Vec2::new(x1, y1))
//    }
//
//    fn from_corners(p0: Vec2, p1: Vec2) -> Self {
//        Self {
//            min: p0.min(p1),
//            max: p0.max(p1),
//        }
//    }
//    pub fn size(&self) -> Vec2 {
//        self.max - self.min
//    }
//    pub fn center(&self) -> Vec2 {
//        (self.min + self.max) * 0.5
//    }
//    pub fn contains(&self, point: Vec2) -> bool {
//        (point.cmpgt(self.min) & point.cmplt(self.max)).all()
//    }
//}

/// Quadtree itu sendiri
#[derive(Resource, Debug, Clone, PartialEq)]
pub struct TkQuadTree {
    // ini untuk Debug lebih mudah
    pub name: String,
    pub id: usize,
    // ini fungsi utama
    boundaries: Rect,
    tiles: Option<Vec<Entity>>,
    divided: bool,
    childnode: Option<[Box<TkQuadTree>; 4]>,
}

impl TkQuadTree {
    /// Fungsi untuk menginisalisasi Quadtree
    pub fn new(
        name: String,
        id: usize,
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

    pub fn contains3_equal_option(&self, rhs: Option<Vec3>) -> bool {
        match rhs {
            Some(t) => {
                let pos = Vec2::new(t.x, t.y);
                (pos.cmpge(self.boundaries.min) & pos.cmple(self.boundaries.max)).all()
            }
            None => return false,
        }
    }

    /// fungsi untuk mengecek apakah suatu vector ada dalam posisi quadtree tersebut
    pub fn contains3_equal(&self, rhs: Vec3) -> bool {
        let posi = vec2(rhs.x, rhs.y);
        // iya, boundaries itu sendiri bevy::Rect
        (posi.cmpge(self.boundaries.min) & posi.cmple(self.boundaries.max)).all()
    }

    /// Fungsi untuk memasukkan suatu entity ke dalam quadtree dengan menggunakan informasi dari tr
    /// atau posisi dari entity itu sendiri
    pub fn insert(&mut self, en: Entity, tr: Vec3) -> Option<Vec3> {
        // Melakukan pengecekan apakah quadtree ini memiliki posisi yang diberikan
        if self.contains3_equal(tr) {
            // jika > maka itu akan dihitung ketika kita sudah
            // menambahkan yang ke empat
            if let Some(tiles) = self.tiles.as_mut() {
                tiles.push(en);
                //println!(
                //    "Berhasil Memasukkan Entity ke quadtree dengan sebagai berikut= en: {}, tr: {:?}, ke {}",
                //    en, tr, self.name
                //);
                if tiles.len() > 4 {
                    self.subdivide();
                    return Some(tr);
                }
            }
        } // else tidak akan melakukan apa - apa jika objek tidak dalam posisi itu
        None
    }

    pub fn remerge(&mut self) {
        let all_child = self.take_all_children_unit();
        println!("\n take all Children {:?}", all_child);
        self.tiles = Some(Vec::new());
        if let Some(tiles) = &mut self.tiles {
            tiles.extend(all_child.into_iter());
        }
        self.divided = false;
        self.childnode = None;
    }

    /// metode untuk memindahkan nilai unit dari anakan ke dalam parent serta menghapus anakan itu
    /// sendiri
    pub fn take_all_children_unit(&mut self) -> Vec<Entity> {
        // Inisialisasi nilai penampung return keseluruhan
        let mut return_value: Vec<Entity> = Vec::new();

        // apabila parent divided
        if self.divided {
            for i in self.childnode.as_mut().unwrap() {
                return_value.extend(i.take_all_children_unit().into_iter());
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
        // tambahkan fungsi yang dapat mentrigger distribute sekali lagi
        //println!("Dividing Partition");
        self.divided = true;
        let center = self.boundaries.center();
        let (centerx, centery) = (center.x, center.y);
        let quadra: [Box<TkQuadTree>; 4] = [
            // Top Left
            Box::new(TkQuadTree::new(
                format!("Top Left {}", self.id + 1),
                self.id + 1,
                self.boundaries.min.x,
                centery,
                centerx,
                self.boundaries.max.y,
            )),
            // Bottomleft
            Box::new(TkQuadTree::new(
                format!("Bottom Left {}", self.id + 1),
                self.id + 1,
                self.boundaries.min.x,
                self.boundaries.min.y,
                centerx,
                centery,
            )),
            // TopRight
            Box::new(TkQuadTree::new(
                format!("Top Right {}", self.id + 1),
                self.id + 1,
                centerx,
                centery,
                self.boundaries.max.x,
                self.boundaries.max.y,
            )),
            // Bottomright
            Box::new(TkQuadTree::new(
                format!("Bottom Right {}", self.id + 1),
                self.id + 1,
                centerx,
                self.boundaries.min.y,
                self.boundaries.max.x,
                centery,
            )),
        ];
        self.childnode = Some(quadra)
    }

    /// Fungsi untuk melakukan distribusi pada
    pub fn distribute(&mut self, en: Entity, tr: Vec3) -> Option<Vec3> {
        // mengecek terlebih dahulu apakah nilai tr ada di kotak ini atau tidak
        if self.contains3_equal(tr) {
            // mengecek apakah anakan / diri sendiri telah terbelah atau belum
            if self.divided == true {
                let child_node = self.childnode.as_mut().unwrap();

                // disini kita menggunakan
                for i in child_node {
                    i.distribute(en, tr);
                }
                return Some(tr);
            } else {
                self.insert(en, tr);
                return None;
            }
        }
        None
    }

    ///// Fungsi yang digunakan untuk mendapatkan suatu partisi berdasarkan posisi yang diberikan
    ///// pada parameter fungsi tersebut. hanya menerima Vec3 untuk saat ini
    //pub fn get_parent(&self, tr: Vec3) -> Option<&TkQuadTree> {
    //    // cek apakah partisi ini memiliki tr, apabila tidak return none
    //    if self.contains3(tr) {
    //        // cek apakah diri sendiri terbagi
    //        if self.divided {
    //            //
    //            let child_node = self.childnode.as_ref().unwrap();
    //            for i in child_node {
    //                if let Some(part) = i.get_parent(tr) {
    //                    return Some(part);
    //                }
    //            }
    //            None
    //        } else {
    //            return Some(self);
    //        }
    //    } else {
    //        return None;
    //    }
    //}

    /// Fungsi yang digunakan untuk mendapatkan suatu partisi mutable berdasarkan posisi yang kau berikan
    /// pada parameter fungsi tersebut. hanya menerima Vec3 untuk saat ini
    pub fn get_parent_mut(&mut self, tr: Vec3) -> Option<&mut TkQuadTree> {
        if self.contains3_equal(tr) {
            // cek sekali lagi untuk memastikan jika diri sendiri benar2 parent
            if self.divided {
                // melakukan check child apakah child bercabang atau tidak
                if self.check_child_not_divided() {
                    println!(
                        "Get Parrent Mut {} tidak memiliki anakan yang terdivide",
                        self.name
                    );
                    // jika tidak bercabang maka return diri sendiri
                    return Some(self);
                } else {
                    // jika bercabang dan di titik yang dicari
                    if let Some(inner) = self.childnode.as_mut() {
                        // maka kita akan melakukan fungsi get_parent_mut pada anakan tersebut
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

    // NOTE: Mungkin kita harus membersihkan ini terlebih dahulu supaya ini bisa dipakai tanpa
    // perlu melakukan rekursif

    /// Fungsi untuk mendapatkan Partisi
    //pub fn get_partition(&self, tr: Vec3) -> Option<&TkQuadTree> {
    //    // cek apakah partisi ini mengandung tr, apabila tidak return none
    //    if self.contains3(tr) {
    //        // cek apakah diri sendiri divided, apabila tidak maka return diri sendiri dan
    //        // menghentikan rekursi
    //        if self.divided {
    //            // kita akan iterasikan anakan dari quadtree ini apabila memiliki anakan
    //            let child_node = self.childnode.as_ref().unwrap();
    //            for i in child_node {
    //                // kita akan iterasi tiap anakan, disini kebanyakan akan berhenti ketika
    //                // pengecekan posisi / contains dari quadtree itu sendiri
    //                if let Some(part) = i.get_partition(tr) {
    //                    // tentu apabila ada maka kita akan mengembalikan self
    //                    return Some(part);
    //                }
    //            }
    //            None
    //        } else {
    //            // return diri sendiri ketika tidak memiliki anakan
    //            return Some(self);
    //        }
    //    } else {
    //        return None;
    //    }
    //}

    /// Fungi untuk mendapatkan partisi yang mutable
    pub fn get_partition_mut(&mut self, tr: Vec3) -> Option<&mut TkQuadTree> {
        // cek apakah partisi ini mengandung tr, apabila tidak return none
        if self.contains3_equal(tr) {
            // cek apakah diri sendiri divided, apabila tidak maka return diri sendiri dan
            // menghentikan rekursi
            if self.divided {
                // kita akan iterasikan anakan dari quadtree ini apabila memiliki anakan
                let child_node = self.childnode.as_mut().unwrap();
                for i in child_node {
                    // kita akan iterasi tiap anakan, disini kebanyakan akan berhenti ketika
                    // pengecekan posisi / contains dari quadtree itu sendiri
                    if let Some(part) = i.get_partition_mut(tr) {
                        // tentu apabila ada maka kita akan mengembalikan self
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

    /// Fungsi untuk menghapus suatu partisi dan mengubahnya kembali menjadi partisi biasa atau
    /// leaf nodes tanpa cabang
    pub fn delete_partition(&mut self) {
        self.divided = false;
        self.childnode = None;
        self.tiles = Some(Vec::new());
        println!("Partisi {} berhasil dihapus dan menjadi normal", self.name);
    }

    // On Proccess
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
            // apabila sudah tidak ada lagi yang bisa di cari, maka lakukan berikut
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

    // /// Fungsi untuk mengecek keberadaan suatu titik di partisi ini, dan jika ada maka kita akan
    // /// menghapus nilai itu di vec
    //pub fn check_remove(&mut self, en: Entity) -> bool {
    //    if let Some(tile) = &self.tiles {
    //        if tile.contains(&en) {
    //            self.tiles.as_mut().unwrap().retain(|value| *value != en);
    //            return true;
    //        }
    //    }
    //    false
    //}

    /// Fungsi untuk melakukan cek apakah quadtree ini memiliki tiles atau tidak
    pub fn check_if_tiles_empty(&self) -> bool {
        // apabila tiles tidak kosong maka mengembalikan nilai false
        if self.tiles != None {
            return false;
        }
        // apabila kosong, maka kita return true
        true
    }

    /// ini untuk pengecekan pada suatu quadtree apakah Quadree tersebut memiliki anakan yang
    /// bercabang atau tidak.
    ///
    /// ini akan mereturn true ketika ke anakan dari partisi yang dipilih ini tidak terdivide, dan false
    /// apabila terdivide
    pub fn check_child_not_divided(&self) -> bool {
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
        println!("\n \nJumlah tiles pada anakan adalah: {}", hasil);
        if hasil > 4 {
            return false;
        }
        true
    }
    /// Fungsi untuk mengembalikan anakan berdasarkan jumlahnya secara recursive
    fn check_child_amount(&self) -> usize {
        let mut return_amount: usize = 0;
        println!("\nCheck Child partisi: {}", self.name);
        if self.divided {
            for i in self.childnode.as_ref().unwrap() {
                return_amount += i.check_child_amount()
            }
        } else {
            if let Some(tiles) = &self.tiles {
                println!(
                    "partisi {} memiliki tiles dengan jumlah: {}",
                    self.name,
                    tiles.len()
                );
                return_amount += tiles.len();
            }
        }
        return_amount
    }
}

#[derive(Component)]
pub struct QuadtreeUnit;

#[derive(Component, Debug)]
pub struct QuadtreeUnitPosition {
    pub new_val: Option<Vec3>,
    pub old_val: Option<Vec3>,
}

impl QuadtreeUnitPosition {
    pub fn new() -> Self {
        Self {
            new_val: None,
            old_val: None,
        }
    }
    pub fn replace_old(&mut self, rhs: Vec3) {
        match self.old_val {
            Some(_) => *self.old_val.as_mut().unwrap() = rhs,
            None => self.old_val = Some(rhs),
        }
    }
    pub fn assign_values(&mut self, rhs_val: Vec3) {
        match self.old_val {
            Some(_) => match self.new_val {
                Some(_) => {
                    println!("\n \n");
                    panic!("Holy Shit, kok nambah {}", rhs_val)
                }
                None => {
                    //println!(
                    //    "nilai saat ini sebelum update {:?}, {:?}",
                    //    self.new_val, self.old_val
                    //);
                    self.new_val = Some(rhs_val);
                    //println!(
                    //    "Assign pada new Value {}, dengan nilai saat ini adalah {:?}, {:?} \n",
                    //    rhs_val, self.new_val, self.old_val
                    //);
                }
            },
            None => {
                //println!(
                //    "nilai saat ini sebelum update {:?}, {:?}",
                //    self.new_val, self.old_val
                //);
                self.old_val = Some(rhs_val);
                //println!(
                //    "Assign pada old Value {}, dengan nilai saat ini adalah {:?}, {:?} \n",
                //    rhs_val, self.new_val, self.old_val
                //);
            }
        }
    }
    /// Fungsi dengan tujuan untuk menghapus value lama dan mengubah value baru menjadi value lama
    /// old_val = new_val; new_val = None
    pub fn update_values(&mut self) {
        //println!(
        //    "Melakukan Update pada {:?} dengan {:?}",
        //    self.old_val, self.new_val
        //);
        if self.new_val != None {
            self.old_val = self.new_val;
            self.new_val = None;
        } else {
            //println!(
            //    "Error: ada yang salah pada qtup dimana kau mencoba update qtup dengan new_val yang kosong"
            //);
        }
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
            0,
            -200.0,
            -200.0,
            200.0,
            200.0,
        )); // Init the quadtree
        app.insert_resource(QTDistributeConditions::default());
        app.insert_resource(QTDeleteConditions::default());
        app.add_systems(
            Update,
            (
                (
                    unit_to_quadtree,
                    update_quadtree_unit,
                    // terjadi perpindahan anakan dari
                    // suatu titik ke titik lainnya
                    // beserta ketika terjadi kematian
                    delete_qt.run_if(qt_delete), // hanya akan di trigger ketika
                    distribute_qt_child.run_if(qt_distribute), // hanya dijalankan ketika anakan
                                                 // lebih dari 4 dan terjadi
                                                 // subdivide
                )
                    .chain(),
                draw_quadtree, // ini untuk menunjukkan quadtree tersebut
            ),
        );
        app.add_systems(
            EguiPrimaryContextPass,
            (show_current_quadtree_unit, show_current_qtup),
        );
    }
}

fn unit_to_quadtree(
    mut qt: ResMut<TkQuadTree>,
    mut qtdc: ResMut<QTDistributeConditions>,
    mut unit_entity: Query<
        (Entity, &Transform, &mut QuadtreeUnitPosition),
        (With<QuadtreeUnit>, Added<QuadtreeUnit>),
    >,
) {
    for (en, tr, mut qtup) in &mut unit_entity {
        // apabila Quadtree telah terpartisi, maka kita akan melakukan distribute saja
        //println!("Memasukkan {en}");
        if qt.divided {
            //
            if let Some(distribusi) = qt.distribute(en, tr.translation) {
                //println!("Assign This To qtup distribusi start");
                qtup.assign_values(distribusi);
                qtdc.activate(distribusi);
            }
        } else {
            if let Some(distribusi) = qt.insert(en, tr.translation) {
                qtdc.activate(distribusi);
            }
            if qt.contains3_equal(tr.translation) {
                qtup.assign_values(tr.translation);
            }
        }
    }
}

/// Fungsi ini ada untuk mengupdate posisi dari apapun yang memiliki komponen QuadtreeUnit dimana
/// ketika komponen itu bergerak keluar dari suatu partisi, maka fungsi ini akan menghapus
/// keberadaan dari dirinya di partisi sebelumnya lalu menambahkan keberadaannya pada partisi baru
fn update_quadtree_unit(
    mut qr: Query<
        (Entity, &Transform, &mut QuadtreeUnitPosition),
        (With<QuadtreeUnit>, Changed<Transform>),
    >,
    mut qt: ResMut<TkQuadTree>,
    mut qtdec: ResMut<QTDeleteConditions>,
    mut qtdc: ResMut<QTDistributeConditions>,
) {
    // iterasikan query
    for (en, tr, mut qtup) in &mut qr {
        // mendapatkan posisi partisi dimana entity saat ini berada
        if let Some(part) = qt.get_partition_mut(tr.translation) {
            // # -> Ini adalah tempat dimana hal akan di run terus menerus, kita tidak mau ini <- #

            // apabila posisi dari patisi saat ini tidak memiliki entity itu, maka kemungkinan
            // partisi ini adalah partisi yang baru saja dimasuki oleh entity itu sendiri

            if !part.check_entity(en) {
                // setelah dihapus dari antara ke empat posisi sebelumnnya, maka kita akan
                // menginsert entity itu pada posisi saat ini
                if qt.divided {
                    if let Some(distribusi) = qt.distribute(en, tr.translation) {
                        //println!("Update Quadtree distribute \n");
                        qtdec.activate(distribusi);
                        qtdc.activate(distribusi);
                        qtup.assign_values(distribusi);
                    }
                } else {
                    if let Some(distribusi) = qt.insert(en, tr.translation) {
                        //println!("Update Quadtree insert \n");
                        //println!("Assign This To qtup insert update");
                        qtdec.activate(distribusi);
                        qtdc.activate(distribusi);
                        //qtup.assign_values(distribusi);
                    }
                }
            }
        } else { // apabila gagal / objek tersebut telah keluar dari quadtree
        }
    }
}

// fungsi yang dibuat untuk mengecek anakan dari quadtree tersebut dengan menggunakan bevy gizmo
//fn debug_quadtree() {}
//

/// fungsi yang akan  mendistribusikan suatu anakan ketika terjadi
fn distribute_qt_child(
    mut qt: ResMut<TkQuadTree>,
    mut qtdc: ResMut<QTDistributeConditions>,
    mut qr: Query<(Entity, &Transform, &mut QuadtreeUnitPosition), With<QuadtreeUnit>>,
) {
    // ini untuk mendapatkan nilai dari quadtree yang meminta untuk dilakukan distribute
    if let Some(inner) = qtdc.pos {
        //println!("Mendapatkan Posisi untuk Distribute");
        // ini untuk mendapatkan quadtree yang di cari untuk di distribute
        if let Some(sqt) = search_qt_to_distribute(&mut qt, inner) {
            // kemudian kita mengiterasikan setiap anakannya lalu kita menghapus tiles itu sendiri
            for (en, tr, mut qtup) in &mut qr {
                if sqt.check_entity(en) {
                    match sqt.distribute(en, tr.translation) {
                        Some(pos) => {
                            // NOTE Masih ada error di sini
                            match qtup.old_val {
                                Some(old_pos) => {
                                    if sqt.contains3_equal(old_pos) {
                                        //println!("\n \n \n {en} Nomor 1");
                                        qtup.replace_old(pos);
                                    } else {
                                        if !sqt.contains3_equal(old_pos) {
                                            //println!("\n \n \n {en} Nomor 2");
                                            //qtup.replace_old(pos);
                                            //qtup.assign_values(pos);
                                        }
                                    }
                                }
                                None => {
                                    qtup.replace_old(pos);
                                    //println!("\n \n \n {en} Nomor 2");
                                }
                            }
                        }
                        None => {}
                    }
                }
                // menghapus tile untuk menunjukkan jika partition yang sudah terdivide tidak boleh punya tiles
                // lagi selain anakan
            }
            sqt.tiles = None;
        }
        qtdc.clear();
    } else {
    }
}

/// fungsi yang berjalan secara recursive untuk mencari anakan sesuai dengan Transform
fn search_qt_to_distribute(qt: &mut TkQuadTree, tr: Vec3) -> Option<&mut TkQuadTree> {
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
                return search_qt_to_distribute(i, tr);
            }
        }
    }

    None
}

/// Ini adalah fungsi untuk menghapus partisi pada suatu partisi di quadtree serta menghapus nilai
/// dari suatu poin pada partisi lamanya
fn delete_qt(
    mut qr: Query<(Entity, &mut QuadtreeUnitPosition), With<QuadtreeUnit>>,
    mut qt: ResMut<TkQuadTree>,
    mut qtdec: ResMut<QTDeleteConditions>,
) {
    // apabila qtdec memiliki posisi, maka dilanjutkan
    if let Some(position) = qtdec.pos {
        let to_delete = search_unit_to_delete(&mut qt, position);
        //println!(
        //    "Mendapatkan unit apa saja yang harus di cek untuk delete, {:?}",
        //to_delete
        //);
        for (en, mut qtup) in &mut qr {
            // NOTE: Mungkin ini akar alasannya karena si
            if to_delete.contains(&en) {
                // pada mereka yang memiliki 2 values posisi
                if qtup.new_val != None {
                    // Menghapus unit dari posisi lamanya
                    qt.remove_unit(en, *qtup.old_val.as_ref().unwrap());
                    // cari partisi parentnya
                    if let Some(part) = qt.get_parent_mut(*qtup.old_val.as_ref().unwrap()) {
                        println!(
                            "Mendapatkan Partisi berupa: {}======================",
                            part.name
                        );
                        // apabila anakan dari parentnya ada di atas 4
                        if !part.check_child_branch_exceed_four() {
                            println!("{} memiliki jumlah anakan diatas 4", part.name);
                        } else {
                            println!("{} memiliki jumlah anakan dibawah 4", part.name);
                            // Maka kita akan meremerge semua anakan di bawahnya untuk masuk
                            // menjadi ankan dari parent itu sendiri
                            part.remerge();
                        }
                    }
                    // update qtup
                    qtup.update_values();
                }
                qtdec.clear();
            }
        }
        // mencari posisi dari partisi yang terjadi perpindahan itu sendiri
    }
}

fn search_unit_to_delete(qt: &mut TkQuadTree, tr: Vec3) -> Vec<Entity> {
    //Ini akan return Vector dari en mana
    //saja yang akan dilakukan penghapusan
    let mut nilai: Vec<Entity> = Vec::new();
    if let Some(part) = qt.get_partition_mut(tr) {
        if let Some(tiles) = &part.tiles {
            for i in tiles {
                nilai.push(*i);
            }
        }
    }
    nilai
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
