use bevy::prelude::*;

pub const OBJECTSPRITEMAXHORI: u32 = 2;
pub const OBJECTSPRITEMAXVERT: u32 = 2;

pub enum OBJECTSID {
    TRUNK,
    STONE,
}

pub fn object_conversion_index(id: OBJECTSID) -> usize {
    match id {
        OBJECTSID::TRUNK => return 0,
        OBJECTSID::STONE => return 1,
    }
}

pub fn object_atlas_index(id: OBJECTSID) -> usize {
    match id {
        OBJECTSID::TRUNK => 0,
        OBJECTSID::STONE => 2,
    }
}

#[derive(Component)]
pub struct TkObjects {
    id: OBJECTSID,
}

impl TkObjects {
    pub fn new(id: OBJECTSID) -> Self {
        Self { id }
    }
}
