use crate::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct HeroesBundles {
    hero: Heroes,
    id: HeroesId,
    unit: TkUnit,
}

impl HeroesBundles {
    pub fn new(hero: Heroes, id: HeroesId, st: TkUnitState) -> Self {
        Self {
            hero,
            id,
            unit: TkUnit::new(st),
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
    collision_area: TkRectangle,
    // untuk mengecek apakah unit tersebut tengah melakukan tabrakan dengan objek lainnya
    is_colliding: EntityColliding,

    quadtree: QuadtreeUnit,
    quadtree_position: QuadtreeUnitPosition,
}

impl ColliderBundles {
    pub fn new(coltype: CollisionType, width: f32, height: f32) -> Self {
        Self {
            collision_area: TkRectangle::new(width, height),
            is_colliding: EntityColliding::new(coltype),
            quadtree: QuadtreeUnit,
            quadtree_position: QuadtreeUnitPosition::new(),
        }
    }
}
