use bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;
use crate::global_var::*;

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
    pub fn insert(&mut self, en: Entity, tr: Vec3) -> Option<Vec3> {
        if self.contains3(tr) {
            let tailu = self.tiles.as_mut().unwrap();
            if tailu.len() >= 4 {
                // jika > maka itu akan dihitung ketika kita sudah
                // menambahkan yang ke empat

                // apabila tiles lebih dari 4 dan belum terpisah, maka kita akan langsung memisah quadtree menjadi 4
                if !self.divided {
                    self.subdivide();
                }
                // setelah setidaknya sudah ada anakan, maka kita kemudian melakukan distribute
                self.distribute(en, tr);
                return Some(tr);
                // kemudian kita akan langsung memindahkan setiap nilai dalam tile ke dalam child
                // node nya secara langsung dan langsung memisahkan
            } else {
                tailu.push(en);
                println!("Berhasil Memasukkan Entity ke quadtree dengan sebagai berikut= en: {}, tr: {:?}", en, tr);
                return None;
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
                    println!("Berhasil Distribute Entity ke quadtree dengan sebagai berikut= en: {}, tr: {:?}", en, tr);
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
    pub fn get_parent(&self, tr: Vec3) -> Option<&TkQuadTree> {
        if self.contains3(tr) {
            if self.divided {
                let child_node = self.childnode.as_ref().unwrap();
                for i in child_node {
                    if let Some(part) = i.get_parent(tr) {
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

    // Fungsi yang digunakan untuk mendapatkan suatu partisi mutable berdasarkan posisi yang kau berikan
    // pada parameter fungsi tersebut. hanya menerima Vec3 untuk saat ini
    pub fn get_parent_mut(&mut self, tr: Vec3) -> Option<&mut TkQuadTree> {
        if self.contains3(tr){
            let childnode = self.childnode.as_mut().unwrap();
            for i in childnode{
                if i.divided{
                    return i.get_parent_mut(tr);
                }
                else{
                    return Some(i);
                }
            }
        }
        None
    }

    pub fn get_partition(&self, tr: Vec3) -> Option<&TkQuadTree>{
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
    pub fn get_partition_mut(&mut self, tr: Vec3) -> Option<&mut TkQuadTree>{
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

    pub fn delete_partition(&mut self, tr: Vec3){

    }

    // On Proccess
    // Fungsi yang akan mengembalikan n partisi berdasarkan posisi titik a dan titik b serta ray
    // cast dari kedua titik tersebut. fungsi ini akan menerima dua parameter yaitu posisi saat ini
    // dan posisi yang dituju, dimana nantinya kita akan langsung mereturn
    //
    // fungsi ini ada untuk digunakan pada path finding seperti A* Algorithm
    //pub fn ray_partition(&self, tr: Vec3, rhs: Vec3) {}


    pub fn check_entity(&self, en: Entity) -> bool {
        if let Some(tile) = &self.tiles{
            if tile.contains(&en){
            return true;
            }
        }
        false
    }
    pub fn check_remove(&mut self, en: Entity) -> bool {
        if let Some(tile) = &self.tiles{
            if tile.contains(&en){

            self.tiles.as_mut().unwrap().retain(|value| *value != en);
            return true;
            }
        }
        false
    }
    pub fn check_child(&self){}
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
                print_the_quadtree // ini untuk menunjukkan quadtree tersebut
            ),
        );
    }
}

fn unit_to_quadtree(
    mut qt: ResMut<TkQuadTree>,
    mut qtdc: ResMut<QTDistributeChild>,
    unit_entity: Query<(Entity, &Transform), (With<QuadtreeUnit>, Added<QuadtreeUnit>)>,
) {
    // ~ To Fix: Kita Harus menambahakan cara supaya ini tidak terus - terusan menambahkan anakan ~
    // itu sudah di atasi dengan menggunakan Added<>
    for (en, tr) in &unit_entity {
        if qt.divided {
            if let Some(distribusi) = qt.distribute(en, tr.translation) {
                qtdc.activate(distribusi);
            }
        } else {
            if let Some(distribusi) = qt.insert(en, tr.translation){
                qtdc.activate(distribusi);
            }
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
    mut qtdc: ResMut<QTDistributeChild>
) {
    // iterasikan query
    for (en, tr, trec) in &qr {
        // mendapatkan posisi patisi dimana entity saat ini berada
        if let Some(part) = qt.get_parent_mut(tr.translation) {
            // apabila posisi dari patisi saat ini tidak memiliki enntity itu, maka kemungkinan
            // partisi ini adaalh partisi yang baru saja dimasuki oleh entity itu sendiri
            if !part.check_entity(en) {
                // mendapatkan keempat posisi dari rect itu sendiri
                let rect_pos = trec.unwrap_position3(tr.translation);

                // lalu kita akan berusaha menghapus posisi entity yang ada di pertisi sebelumnya
                // dengan menggunakan pengecekan pada empat / posisi terdekatnya
                // 
                // iterasi ke empat posisi rect
                for i in rect_pos {
                    // NOTE: Sepertinya ini kurang efisien
                    // melakukan pengulangan dan check remove setiap entity dari ke empat partisi
                    // yang ada di posisi itu
                    if let Some(inner) = qt.get_parent_mut(i) {
                        if inner.check_remove(en){
                            // NOTE: Disini kita akan melakukan pengecekan terhadap partisi itu
                            


                        }
                    }
                }

                if let Some(distribusi) = qt.insert(en, tr.translation){
                    qtdc.activate(distribusi);
                }
            }
        } else { // apabila gagal / objek tersebut telah keluar dari quadtree
        }
    }
}

// fungsi yang dibuat untuk mengecek anakan dari quadtree tersebut dengan menggunakan bevy gizmo
//fn debug_quadtree() {}
//

// fungsi yang akan  mendistribusikan suatu anakan ketika terjadi
fn distribute_qt_child(
    mut qt: ResMut<TkQuadTree>,
    mut qtdc: ResMut<QTDistributeChild>,
    qr: Query<(Entity, &Transform), With<QuadtreeUnit>>,
) {
    // ini untuk mendapatkan nilai dari quadtree yang meminta untuk dilakukan distribute
    if let Some(inner) = qtdc.pos {
        // ini untuk mendapatkan quadtree yang di cari untuk di distribute
        if let Some(sqt) = search_qt_to_distribute(&mut qt, qtdc.pos.unwrap()) {
            // kemudian kita mengiterasikan setiap anakannya lalu kita menghapus tiles itu sendiri
            for (en, tr) in &qr {
                if sqt.check_entity(en){sqt.distribute(en, tr.translation);}
                // menghapus tile untuk menunjukkan jika partition yang sudah terdivide tidak boleh punya tiles 
                // lagi selain anakan
                sqt.tiles = None;
                qtdc.clear();
            }
        }
    } else {
    }
}


// fungsi yang berjalan secara recursive untuk mencari anakan sesuai dengan Transform
fn search_qt_to_distribute(mut qt: &mut TkQuadTree, tr: Vec3) -> Option<&mut TkQuadTree> {

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

fn search_qt_to_delete(qt: ResMut<TkQuadTree>){}

fn print_the_quadtree(qt: Res<TkQuadTree>){}

