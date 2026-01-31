//!
//! DESCRIPTIONS: FILE YANG MENAMPUNG SEMUA AKTIFITAS PHYSICS / PENDETEKSI COLLISION DIMANA
//! MEMANFAATKAN PARTISI RUANG DARI TKQUADTREE
//!

use crate::{gamestate::startup, tkglobal_var, tkitems, tkquadtree};
use bevy::{ecs::event, prelude::*};
// Plugins//
//
//Ini adalah Plugin yang mana Plugin ini akan berjalan ketika

// // // Resource / Event / Data // // //
//
#[derive(Copy, Clone)]
pub enum CollisionType {
    UNIT,
    ITEMS,
}

/// Struct Event yang memberikan signal terkait pengecekan collision kepada system handle collision
/// untuk setiap unit
#[derive(Event)]
pub struct PhysicsColisionEventHandle {
    enself: Entity,
    otheren: Entity,
    intes_size: Vec2,
    condition: BVec2,
}

/// Struct Event yang memberikan signal terkait pengecekan collision kepada system pengambilan
/// items / pemasukkan item ke dalam inventory
#[derive(Event)]
pub struct ItemCollisionEventHandle {
    pub itemen: Entity,
    pub uniten: Entity,
}

// // // Component // // //
//
/// Struct untuk bentuk segi empat
/// model dari struct ini dalam model ECS sehingga ia hanya menyimpan informasi terkait width dan
/// height saja. untuk posisinya diambil dari entity yang memegangnya (dimana pasti memiliki
/// transform)
#[derive(Component, Clone, Copy)]
pub struct TkRectangle {
    pub width: f32,
    pub height: f32,
}

// ini tidak akan mungkin mengingat posisi dari TkRectangle terletak pada entity Transform itu
// sendiri. jadi kita disini akan mengabaikan fungsi bawaannya
impl TkRectangle {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
    pub fn unwrap_position3(&self, tr: Vec3) -> [Vec3; 4] {
        let x0 = tr.x - self.width / 2.;
        let y0 = tr.y - self.height / 2.;
        let x1 = tr.x + self.width / 2.;
        let y1 = tr.y + self.height / 2.;
        let x0y0 = Vec3::new(x0, y0, tr.z);
        let x1y0 = Vec3::new(x1, y0, tr.z);
        let x0y1 = Vec3::new(x0, y1, tr.z);
        let x1y1 = Vec3::new(x1, y1, tr.z);
        [x0y0, x1y0, x0y1, x1y1]
    }
    // Fungsi yang digunakan untuk mendapatkan nilai dari cordinat x0, y0, x1, y1 dari tk rectangle
    // itu sendiri
    //
    // pada dasarnya tk rectangle menggunakaan unit Transform itu sendiri untuk
    pub fn unwrap_coord(&self, tr: &Vec3) -> [f32; 4] {
        let x0 = tr.x - self.width / 2.;
        let y0 = tr.y - self.height / 2.;
        let x1 = tr.x + self.width / 2.;
        let y1 = tr.y + self.height / 2.;
        [x0, y0, x1, y1]
    }
    /// Fungsi yang mereturn Vec2 ukuran dari dua rectangle yang intersect
    pub fn intersect_size(&self, other: &TkRectangle, self_pos: &Vec3, other_pos: &Vec3) -> Vec2 {
        let current_pos = self.unwrap_coord(self_pos);
        let next_pos = other.unwrap_coord(other_pos);
        if !(current_pos[0] <= next_pos[2]
            && current_pos[2] >= next_pos[0]
            && current_pos[1] <= next_pos[3]
            && current_pos[3] >= next_pos[1])
        {
            return Vec2::ZERO;
        }
        Vec2::new(
            current_pos[2].min(next_pos[2]) - current_pos[0].max(next_pos[0]),
            current_pos[3].min(next_pos[3]) - current_pos[1].max(next_pos[1]),
        )
    }
}

/// Struct untuk bentuk Capsules (tidur)
/// model dari struct ini dalam model ECS sehingga han ya menyimpan radius dari lingkaran tersebut
/// tanpa menyertakan posisinya. untuk posisinya diambil dari entity yang memegangnya (dimana pasti
/// memiliki transform)
#[derive(Component, Clone, Copy)]
struct TkCapsules {
    width: f32, // width ada, tapi height = rad
    rad: f32,
}

impl TkCapsules {
    /// fungsi init untuk membuat componen ECS TkCircles
    pub fn new(width: f32, rad: f32) -> Self {
        Self { width, rad }
    }
}

#[derive(Component, Clone, Copy)]
pub struct EntityColliding {
    colliding: bool,
    col_type: CollisionType,
}

impl EntityColliding {
    pub fn new(coltype: CollisionType) -> Self {
        Self {
            colliding: false,
            col_type: coltype,
        }
    }
}

// // // IMPLEMENTATION // // //

/// Fungsi untuk mengakses Quadtree serta melakukan pengecekan collision berdasarkan isi dari
/// Quadtree tersebut.
pub fn access_quadtree_physics(
    mut command: Commands,
    mut qr: Query<(&EntityColliding, &TkRectangle, &mut Transform), With<tkquadtree::QuadtreeUnit>>,
    qt: Res<tkquadtree::TkQuadTree>,
) {
    //println!("List: {:?}", qt.get_all_entity());
    // mendapatkan semua entity dalam quadtree
    if let Some(all_en) = qt.get_all_entity() {
        // iterasikan vector tersebut untuk mengakses vector di dalamnya
        for part_all_en in all_en {
            // apabila len 1, make skip
            if part_all_en.len() == 1 {
                continue;
            }
            // 2D Array Iteration
            // NOTE Uncheck
            //println!("Cek Pada {:?}", part_all_en);
            for i in part_all_en {
                if let Ok((current_ecol, current_rectang, current_tr)) = qr.get(*i) {
                    // Pastikan yang saat ini adalah Unit dan bukan items
                    match current_ecol.col_type {
                        CollisionType::ITEMS => {
                            continue;
                        }
                        _ => {}
                    }
                    // Mendapatkan Kordinat Kotak untuk current
                    let current_pos = current_rectang.unwrap_coord(&current_tr.translation);
                    for j in part_all_en {
                        // if check untuk memastikan entity i bukanlah entity i itu sendiri
                        if i != j {
                            // if check untuk memastikan entity i bukanlah entity i itu sendiri
                            if let Ok((next_ecol, next_rectang, next_tr)) = qr.get(*j) {
                                // Mendapatkan Kordinat Kotak untuk next
                                let next_pos = next_rectang.unwrap_coord(&next_tr.translation);

                                if current_pos[0] <= next_pos[2]
                                    && current_pos[2] >= next_pos[0]
                                    && current_pos[1] <= next_pos[3]
                                    && current_pos[3] >= next_pos[1]
                                {
                                    match next_ecol.col_type {
                                        CollisionType::UNIT => {
                                            let overlap_value = current_rectang.intersect_size(
                                                next_rectang,
                                                &current_tr.translation,
                                                &next_tr.translation,
                                            );
                                            command.trigger(PhysicsColisionEventHandle {
                                                enself: *i,
                                                otheren: *j,
                                                intes_size: overlap_value,
                                                condition: BVec2::new(
                                                    current_tr.translation.x
                                                        < next_tr.translation.x,
                                                    current_tr.translation.y
                                                        < next_tr.translation.y,
                                                ),
                                            });
                                        }
                                        CollisionType::ITEMS => {
                                            command.trigger(ItemCollisionEventHandle {
                                                itemen: *j,
                                                uniten: *i,
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Fungsi untuk menghandle collision yang
fn handle_collision(
    colval: On<PhysicsColisionEventHandle>,
    mut qr: Query<(&TkRectangle, &mut Transform), With<tkquadtree::QuadtreeUnit>>,
) {
    if let Ok((_, mut current_tr)) = qr.get_mut(colval.enself) {
        if colval.intes_size.x < colval.intes_size.y {
            if colval.condition.x {
                current_tr.translation.x -= colval.intes_size.x
            } else {
                current_tr.translation.x += colval.intes_size.x
            }
        } else {
            if colval.condition.y {
                current_tr.translation.y -= colval.intes_size.y
            } else {
                current_tr.translation.y += colval.intes_size.y
            }
        }
    }
}

pub fn tk_show_collision_box(
    qr: Query<(&Transform, &TkRectangle), (With<EntityColliding>)>,
    mut gizmos: Gizmos,
) {
    for (tr, re) in &qr {
        gizmos.rect_2d(
            Vec2::new(tr.translation.x, tr.translation.y),
            Vec2::new(re.width, re.height),
            Color::linear_rgb(1.0, 0.0, 0.0),
        );
    }
}

// // // PLUGIN // // //
//
pub struct TkPhysicsPlugin;

impl Plugin for TkPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                access_quadtree_physics.run_if(in_state(tkglobal_var::GameState::Play)),
                tk_show_collision_box.run_if(in_state(tkglobal_var::GameState::Play)),
            ), // ini hanya akan berjalan ketika game state
               // adalah play
        );
        app.add_observer(handle_collision);
    }
}
