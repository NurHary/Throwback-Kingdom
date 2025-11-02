//! Ini adalah Plugin yang digunakan untuk algoritma spatial partitioning yang digunakan untuk

use crate::global_var::*;
use bevy::color::palettes::css::WHITE;
use bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;

use crate::tool::qt_delete;
use crate::{
    tool::{qt_distribute, QTDeleteConditions},
    toolplugin::TkRectangle,
};

// Quadtree itu sendiri
// pertanyaannya adalah bagaimana cara mengimplementasikannya
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

    /// fungsi untuk mengecek apakah suatu vector ada dalam posisi quadtree tersebut
    pub fn contains3(&self, rhs: Vec3) -> bool {
        let posi = vec2(rhs.x, rhs.y);
        // iya, boundaries itu sendiri bevy::Rect
        self.boundaries.contains(posi)
    }

    /// Fungsi untuk memasukkan suatu entity ke dalam quadtree dengan menggunakan informasi dari tr
    /// atau posisi dari entity itu sendiri
    pub fn insert(&mut self, en: Entity, tr: Vec3) -> Option<Vec3> {
        //println!("Memasukkan posisi \n");
        // Melakukan pengecekan apakah quadtree ini memiliki posisi yang diberikan
        if self.contains3(tr) {
            // jika > maka itu akan dihitung ketika kita sudah
            // menambahkan yang ke empat
            if let Some(tiles) = self.tiles.as_mut() {
                tiles.push(en);
                println!(
                    "Berhasil Memasukkan Entity ke quadtree dengan sebagai berikut= en: {}, tr: {:?}, ke {}",
                    en, tr, self.name
                );
                if tiles.len() > 4 {
                    self.subdivide();
                    return Some(tr);
                }
            }
        } // else tidak akan melakukan apa - apa jika objek tidak dalam posisi itu
        None
    }

    /// # Fungsi untuk membangun tempat anakan dan mendeklarasikan diri bahwa diri telah terbagi
    pub fn subdivide(&mut self) {
        // tambahkan fungsi yang dapat mentrigger distribute sekali lagi
        println!("Dividing Partition");
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
        if self.contains3(tr) {
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
                println!(
                    "Berhasil Distribute Entity ke quadtree dengan sebagai berikut= en: {}, tr: {:?}, ke {} /n",
                    en, tr, self.name
                );
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
        if self.contains3(tr) {
            // cek sekali lagi untuk memastikan jika diri sendiri benar2 parent
            if self.divided {
                // melakukan check child apakah child bercabang atau tidak
                if self.check_child_not_divided(tr) {
                    // jika tidak bercabang maka return diri sendiri
                    return Some(self);
                } else {
                    // jika bercabang dan di titik yang dicari
                    if let Some(inner) = self.childnode.as_mut() {
                        // maka kita akan melakukan fungsi get_parent_mut pada anakan tersebut
                        for i in inner {
                            if let Some(hasil_return) = i.get_parent_mut(tr) {
                                println!("Getting parent mutable");
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

    ///// Fungsi untuk mendapatkan Partisi
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

    /// Fungi untuk mendapatkan partisi yang utable
    pub fn get_partition_mut(&mut self, tr: Vec3) -> Option<&mut TkQuadTree> {
        // cek apakah partisi ini mengandung tr, apabila tidak return none
        if self.contains3(tr) {
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
        self.tiles = Some(Vec::new())
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
                //println!("Check Entity keluar sebagai: ada {en}\n");
                return true;
            }
        }
        false
    }

    /// Fungsi untuk mengecek keberadaan suatu titik di partisi ini, dan jika ada maka kita akan
    /// menghapus nilai itu di vec
    pub fn check_remove(&mut self, en: Entity) -> bool {
        if let Some(tile) = &self.tiles {
            if tile.contains(&en) {
                self.tiles.as_mut().unwrap().retain(|value| *value != en);
                return true;
            }
        }
        false
    }

    /// Fungsi untuk melakukan cek apakah quadtree ini memiliki tiles atau tidak
    pub fn check_tiles(&self) -> bool {
        // apabila tiles tidak kosong maka mengembalikan nilai true
        if self.tiles != None {
            return false;
        }
        // apabila kosong (None), maka kita akan return false
        println!("Tiles Kosong");
        true
    }

    /// ini untuk pengecekan pada suatu quadtree apakah Quadree tersebut memiliki anakan yang
    /// bercabang atau tidak.
    ///
    /// ini akan mereturn true ketika ke anakan dari partisi yang dipilih ini tidak terdivide, dan false
    /// apabila terdivide
    pub fn check_child_not_divided(&self, tr: Vec3) -> bool {
        if let Some(anakan) = self.childnode.as_ref() {
            let mut return_kondisi: bool = true;
            for i in anakan {
                if !i.divided && i.contains3(tr) {
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
    /// Fungsi yang akan melakukan pengecekan pada ke emoat anakan dimana ini akan mereturn true
    /// apabila tidak ada yang terdivide dan akan mengembalikan false apabila ada satu yang
    /// memiliki nilai di tilesnya
    pub fn check_child_branch(&self) -> bool {
        let mut return_value = true;
        for i in self.childnode.as_ref().unwrap() {
            if i.divided {
                // apabila anakan terdivide, maka kita akan cek satu persatu
                return_value = i.check_child_branch();
                if return_value == false {
                    return return_value;
                }
            } else {
                return_value = i.check_tiles();
                if return_value == false {
                    return return_value;
                }
                // apabila anakan tidak terdivide
            }
        }
        return_value
    }
}

#[derive(Component)]
pub struct QuadtreeUnit;

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
                    distribute_qt_child.run_if(qt_distribute), // hanya dijalankan ketika anakan
                    // lebih dari 4 dan terjadi
                    // subdivide
                    delete_qt_partition.run_if(qt_delete), // hanya akan di trigger ketika
                                                           // terjadi perpindahan anakan dari
                                                           // suatu titik ke titik lainnya
                                                           // beserta ketika terjadi kematian
                )
                    .chain(),
                draw_quadtree, // ini untuk menunjukkan quadtree tersebut
            ),
        );
    }
}

fn unit_to_quadtree(
    mut qt: ResMut<TkQuadTree>,
    mut qtdc: ResMut<QTDistributeConditions>,
    unit_entity: Query<(Entity, &Transform), (With<QuadtreeUnit>, Added<QuadtreeUnit>)>,
) {
    for (en, tr) in &unit_entity {
        // apabila Quadtree telah terpartisi, maka kita akan melakukan distribute saja
        println!("Memasukkan {en}");
        if qt.divided {
            //
            if let Some(distribusi) = qt.distribute(en, tr.translation) {
                //qtdc.activate(distribusi);
            }
        } else {
            if let Some(distribusi) = qt.insert(en, tr.translation) {
                qtdc.activate(distribusi);
            }
        }
    }
}

/// Fungsi ini ada untuk mengupdate posisi dari apapun yang memiliki komponen QuadtreeUnit dimana
/// ketika komponen itu bergerak keluar dari suatu partisi, maka fungsi ini akan menghapus
/// keberadaan dari dirinya di partisi sebelumnya lalu menambahkan keberadaannya pada partisi baru
fn update_quadtree_unit(
    qr: Query<(Entity, &Transform, &TkRectangle), (With<QuadtreeUnit>, Changed<Transform>)>,
    mut qt: ResMut<TkQuadTree>,
    mut qtdc: ResMut<QTDistributeConditions>,
    mut qtdec: ResMut<QTDeleteConditions>,
) {
    // iterasikan query
    for (en, tr, trec) in &qr {
        // mendapatkan posisi partisi dimana entity saat ini berada
        if let Some(part) = qt.get_partition_mut(tr.translation) {
            // # -> Ini adalah tempat dimana hal akan di run terus menerus, kita tidak mau ini <- #

            // apabila posisi dari patisi saat ini tidak memiliki entity itu, maka kemungkinan
            // partisi ini adalah partisi yang baru saja dimasuki oleh entity itu sendiri

            // NOTE: Kenapa ini membaca ketika entity itu ada dalam suatu partisi itu
            if !part.check_entity(en) {
                //println!("Membaca jika tidak ada");
                // NOTE:
                // Aku menghapus bagian Menghapusnya disini

                // setelah dihapus dari antara ke empat posisi sebelumnnya, maka kita akan
                // menginsert entity itu pada posisi saat ini
                if qt.divided {
                    if let Some(distribusi) = qt.distribute(en, tr.translation) {
                        println!("Update Quadtree distribute \n");
                        qtdc.activate(distribusi);
                    }
                } else {
                    if let Some(distribusi) = qt.insert(en, tr.translation) {
                        println!("Update Quadtree insert \n");
                        qtdc.activate(distribusi);
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
    qr: Query<(Entity, &Transform), With<QuadtreeUnit>>,
) {
    // ini untuk mendapatkan nilai dari quadtree yang meminta untuk dilakukan distribute
    if let Some(inner) = qtdc.pos {
        println!("Mendapatkan Posisi untuk Distribute");
        // ini untuk mendapatkan quadtree yang di cari untuk di distribute
        if let Some(sqt) = search_qt_to_distribute(&mut qt, inner) {
            // kemudian kita mengiterasikan setiap anakannya lalu kita menghapus tiles itu sendiri
            for (en, tr) in &qr {
                if sqt.check_entity(en) {
                    sqt.distribute(en, tr.translation);
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
    if qt.divided && qt.tiles != None {
        println!("Search Qt to Distribute: Menemukan partisi untuk di distribute");
        return Some(qt);
    }
    // Ketika ini divided tapi tidak ada nilai didalamnya
    else if qt.divided && qt.tiles == None {
        // kita akan mengecek setiap anakannya apakah setiap kotak anakannya dapat menampung tr
        for i in qt.childnode.as_mut().unwrap() {
            if i.contains3(tr) {
                return search_qt_to_distribute(i, tr);
            }
        }
    }

    None
}

/// Ini adalah fungsi untuk menghapus partisi pada suatu partisi di quadtree
fn delete_qt_partition(mut qt: ResMut<TkQuadTree>, mut qtdec: ResMut<QTDeleteConditions>) {}

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
