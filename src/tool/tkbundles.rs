use crate::{entities::tkentities, tkphysics, tkquadtree};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct HeroesBundles {
    hero: tkentities::Heroes,
    id: tkentities::DynamicHeroId,
    unit: tkentities::TkUnit,
}

impl HeroesBundles {
    pub fn new(
        hero: tkentities::Heroes,
        id: tkentities::DynamicHeroId,
        st: tkentities::TkUnitState,
    ) -> Self {
        Self {
            hero,
            id,
            unit: tkentities::TkUnit::new(st),
        }
    }
}

/// ini bundle untuk memberikan suatu unit kemampuan untuk collision serta menaruh unit ke dalam
/// quadtree serta menjadi suatu yang dapat colliding
#[derive(Bundle)]
pub struct ColliderBundles {
    // ukuran dari collision tersebut.
    // kita disini hanya akan menggunakan ukuran kotak karena ukuran kotak adalah ukuran yang akan
    // kita gunakan secara utama
    collision_area: tkphysics::TkRectangle,
    // untuk mengecek apakah unit tersebut tengah melakukan tabrakan dengan objek lainnya
    is_colliding: tkphysics::EntityColliding,

    quadtree: tkquadtree::QuadtreeUnit,
    quadtree_position: tkquadtree::QuadtreeUnitPosition,
    quadtree_states: tkquadtree::QuadtreeUnitStates,
}

impl ColliderBundles {
    pub fn new(coltype: tkphysics::COLLISIONTYPE, width: f32, height: f32) -> Self {
        Self {
            collision_area: tkphysics::TkRectangle::new(width, height),
            is_colliding: tkphysics::EntityColliding::new(coltype),
            quadtree: tkquadtree::QuadtreeUnit,
            quadtree_position: tkquadtree::QuadtreeUnitPosition::new(),
            quadtree_states: tkquadtree::QuadtreeUnitStates::new(),
        }
    }
}
