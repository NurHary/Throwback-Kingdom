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
