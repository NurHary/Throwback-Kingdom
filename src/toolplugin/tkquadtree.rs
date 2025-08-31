use bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;

use crate::{
    tool::{qt_distribute, QTDistributeChild},
    toolplugin::TkRectangle,
};

// Quadtree itu sendiri
// pertanyaannya adalah bagaimana cara mengimplementasikannya
#[derive(Resource, Debug, Clone, PartialEq)]
pub struct TkQuadTree {
    boundaries: Rect,
    tiles: Option<Vec<Entity>>,
    divided: bool,
    childnode: Option<[Box<TkQuadTree>; 4]>, // UL, LL, UR, lR
}

impl TkQuadTree {
    // Fungsi untuk menginisalisasi Quadtree
    pub fn new(border_x0: f32, border_y0: f32, border_x1: f32, border_y1: f32) -> Self {
        Self {
            boundaries: Rect::new(border_x0, border_y0, border_x1, border_y1),
            tiles: Some(Vec::new()),
            divided: false,
            childnode: None,
        }
    }
    //
    pub fn insert(&mut self, rhs: Entity, tr: Vec3) -> Option<Vec3> {
        if self.contains3(tr) {
            if let Some(mut tiles) = self.tiles.take() {
                if tiles.len() >= 4 {
                    // jika > maka itu akan dihitung ketika kita sudah
                    // menambahkan yang ke empat

                    // apabila tiles lebih dari 4 dan belum terpisah, maka kita akan langsung memisah quadtree menjadi 4
                    if !self.divided {
                        self.subdivide();
                    }
                    // setelah setidaknya sudah ada anakan, maka kita kemudian melakukan distribute
                    self.distribute(rhs, tr);
                    return Some(tr);
                    // kemudian kita akan langsung memindahkan setiap nilai dalam tile ke dalam child
                    // node nya secara langsung dan langsung memisahkan
                } else {
                    tiles.push(rhs);
                    return None;
                }
            }
        } // else tidak akan melakukan apa - apa jika objek tidak dalam posisi itu
        None
    }

    // # Fungsi untuk membangun anakan berdasarkan ukuran diri sendiri
    fn subdivide(&mut self) {
        // tambahkan fungsi yang dapat mentrigger distribute sekali lagi
        self.divided = true;
        let center = self.boundaries.center();
        let (centerx, centery) = (center.x, center.y);
        let quadra: [Box<TkQuadTree>; 4] = [
            // Top Left
            Box::new(TkQuadTree::new(
                self.boundaries.min.x,
                centery,
                centerx,
                self.boundaries.max.y,
            )),
            // Bottomleft
            Box::new(TkQuadTree::new(
                self.boundaries.min.x,
                self.boundaries.min.y,
                centerx,
                centery,
            )),
            // TopRight
            Box::new(TkQuadTree::new(
                centerx,
                centery,
                self.boundaries.max.x,
                self.boundaries.max.y,
            )),
            // Bottomright
            Box::new(TkQuadTree::new(
                centerx,
                self.boundaries.min.y,
                self.boundaries.max.x,
                centery,
            )),
        ];
        self.childnode = Some(quadra)
    }

    // fungsi untuk mengecek apakah suatu vector
    pub fn contains3(&self, rhs: Vec3) -> bool {
        let posi = vec2(rhs.x, rhs.y);
        // iya, boundaries itu sendiri bevy::Rect
        self.boundaries.contains(posi)
    }
    pub fn distribute(&mut self, en: Entity, tr: Vec3) -> Option<Vec3> {
        // mengecek terlebih dahulu apakah nilai tr ada di kotak ini atau tidak
        if self.contains3(tr) {
            // mengecek apakah anakan / diri sendiri telah terbelah atau belum
            if self.divided == true {
                let child_node = self.childnode.as_mut().unwrap();

                // disini kita menggunakan
                for i in child_node {
                    i.distribute(en, tr);
                    return Some(tr);
                }
            } else {
                self.insert(en, tr);
                return None;
            }
        }
        None
    }
    // Fungsi yang digunakan untuk mendapatkan suatu partisi berdasarkan posisi yang kau berika
    // pada parameter fungsi tersebut. hanya menerima Vec3 untuk saat ini
    pub fn get_partition(&self, tr: Vec3) -> Option<&TkQuadTree> {
        if self.contains3(tr) {
            if self.divided {
                let child_node = self.childnode.as_ref().unwrap();
                for i in child_node {
                    if let Some(part) = i.get_partition(tr) {
                        return Some(part);
                    }
                }
                None
            } else {
                return Some(self);
            }
        } else {
            return None;
        }
    }

    // Fungsi yang digunakan untuk mendapatkan suatu partisi mutable berdasarkan posisi yang kau berika
    // pada parameter fungsi tersebut. hanya menerima Vec3 untuk saat ini
    pub fn get_partition_mut(&mut self, tr: Vec3) -> Option<&mut TkQuadTree> {
        if self.contains3(tr) {
            if self.divided {
                let child_node = self.childnode.as_mut().unwrap();
                for i in child_node {
                    if let Some(part) = i.get_partition_mut(tr) {
                        return Some(part);
                    }
                }
                None
            } else {
                return Some(self);
            }
        } else {
            return None;
        }
    }

    // Fungsi yang digunakan untuk mendapatkan 4 nilai terdekat dari suatu partisi tanpa
    //pub fn change_position_remove(&mut self, tr: [Vec3; 4]) {
    //    for i in tr {
    //        if !self.contains3(i) {
    //            if let Some(inner) =
    //        }
    //    }
    //}

    // On Proccess
    // Fungsi yang akan mengembalikan n partisi berdasarkan posisi titik a dan titik b serta ray
    // cast dari kedua titik tersebut. fungsi ini akan menerima dua parameter yaitu posisi saat ini
    // dan posisi yang dituju, dimana nantinya kita akan langsung mereturn
    //
    // fungsi ini ada untuk digunakan pada path finding seperti A* Algorithm
    //pub fn ray_partition(&self, tr: Vec3, rhs: Vec3) {}

    //.
    pub fn check_entity(&self, en: Entity) -> bool {
        if self.tiles.as_ref().unwrap().contains(&en) {
            return true;
        }
        false
    }
    pub fn check_remove(&mut self, en: Entity) {
        if self.tiles.as_ref().unwrap().contains(&en) {
            self.tiles.as_mut().unwrap().retain(|value| *value != en);
        }
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
            -10000000.0,
            -10000000.0,
            10000000.0,
            10000000.0,
        )); // Init the quadtree
        app.insert_resource(QTDistributeChild::default());
        app.add_systems(
            Update,
            (
                unit_to_quadtree,
                (
                    update_quadtree_unit,
                    distribute_qt_child.run_if(qt_distribute), // hanya dijalankan ketika anakan
                                                               // lebih dari 4 dan terjadi
                                                               // subdivide
                )
                    .chain(),
            ),
        );
    }
}

fn unit_to_quadtree(
    mut qt: ResMut<TkQuadTree>,
    unit_entity: Query<(Entity, &Transform), (With<QuadtreeUnit>, Added<QuadtreeUnit>)>,
) {
    // ~ To Fix: Kita Harus menambahakan cara supaya ini tidak terus - terusan menambahkan anakan ~
    // itu sudah di atasi dengan menggunakan Added<>
    for (entiti, tr) in &unit_entity {
        if qt.divided {
            qt.distribute(entiti, tr.translation);
        } else {
            qt.insert(entiti, tr.translation);
        }
    }
}

// Fungsi ini ada untuk mengupdate posisi dari apapun yang memiliki komponen QuadtreeUnit dimana
// ketika komponen itu bergerak keluar dari suatu partisi, maka fungsi ini akan menghapus
// keberadaan dari dirinya di partisi sebelumnya lalu menambahkan keberadaannya pada partisi baru
fn update_quadtree_unit(
    qr: Query<(Entity, &Transform, &TkRectangle), (With<QuadtreeUnit>, Changed<Transform>)>,
    //lqr: Query<(Entity, &Transform), With<QuadtreeUnit>>, // aku lupa ini untuk apa
    mut qt: ResMut<TkQuadTree>,
) {
    //for (en, tr) in &lqr {}

    for (en, tr, trec) in &qr {
        if let Some(part) = qt.get_partition_mut(tr.translation) {
            // jadi konsepnya adalah: kan ini partisi baru, nah seharusnya
            if !part.check_entity(en) {
                // mendapatkan keempat posisi dari rect itu sendiri
                let rect_pos = trec.unwrap_position3(tr.translation);

                for i in rect_pos {
                    if let Some(inner) = qt.get_partition_mut(i) {
                        inner.check_remove(en)
                    }
                }

                qt.insert(en, tr.translation);
            }
        } else { // apabila gagal / objek tersebut telah keluar dari quadtree
        }
    }
}

// fungsi yang dibuat untuk mengecek anakan dari quadtree tersebut dengan menggunakan egui
//fn debug_quadtree() {}
//

fn distribute_qt_child(
    mut qt: ResMut<TkQuadTree>,
    mut qdc: ResMut<QTDistributeChild>,
    qr: Query<(Entity, &Transform), With<QuadtreeUnit>>,
) {
    // jajal
    if let Some(inner) = qdc.pos {
        let sqt = search_qt_to_distribute(&mut qt, qdc.pos.unwrap()); // searched quadtree
    } else {
    }
}

fn search_qt_to_distribute(mut qt: &mut TkQuadTree, tr: Vec3) -> Option<&mut TkQuadTree> {
    // fungsi untuk mendapatkan quadtree yang dicari, Diluar Plugin

    // Ketika ini divided tapi masih memiliki nilai
    if qt.divided && qt.tiles != None {
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
