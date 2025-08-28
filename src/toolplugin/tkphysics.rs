use crate::*;
use bevy::{math::VectorSpace, prelude::*};
// Plugins//
//
//Ini adalah Plugin yang mana Plugin ini akan berjalan ketika
pub struct TkPhysicsPlugin;

impl Plugin for TkPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_collision.run_if(in_state(GameState::Play)));
    }
}

// Kita akan menngubah fungsinya dari Oop ke Ecs
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
}

#[derive(Component)]
pub struct EntityColliding {
    colliding: bool,
}

impl EntityColliding {
    pub fn new() -> Self {
        Self { colliding: false }
    }
}

pub fn check_collision(
    mut aabb_query: Query<
        (&mut EntityColliding, &TkRectangle, &Transform, Entity),
        With<QuadtreeUnit>,
    >,
) { //

    //for (mut entycoll, rectang, tr, entiti) in &aabb_query {
    //    let this_min_x = tr.translation.x - rectang.width / 2.0;
    //    let this_max_x = tr.translation.x + rectang.width / 2.0;
    //    let this_min_y = tr.translation.y - rectang.height / 2.0;
    //    let this_max_y = tr.translation.y + rectang.height / 2.0;
    //    for (mut other_entycoll, other_rectang, other_tr, other_entiti) in &aabb_query {
    //        let other_min_x = other_tr.translation.x - other_rectang.width / 2.0;
    //        let other_max_x = other_tr.translation.x + other_rectang.width / 2.0;
    //        let other_min_y = other_tr.translation.y - other_rectang.height / 2.0;
    //        let other_max_y = other_tr.translation.y + other_rectang.height / 2.0;
    //        if entiti != other_entiti {
    //            if this_min_x <= other_max_x
    //                && this_max_x >= other_min_x
    //                && this_min_y <= other_max_y
    //                && this_max_y >= other_min_y
    //            {
    //                println!("Other min x: {}, This max x: {}", other_min_x, this_max_x);
    //                println!("Other min y: {}, This max y: {}", other_min_y, this_max_y);
    //                println!("Other max x: {}, This min x: {}", other_max_x, this_min_x);
    //                println!("Other max y: {}, This min y: {}", other_max_y, this_min_y);
    //                println!("Ditabrak dengan {}", other_entiti);
    //                println!("Terdapat Tabrakan")
    //            }
    //        }
    //    }
    //}
}
