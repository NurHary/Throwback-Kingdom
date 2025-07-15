use bevy::prelude::*;

// Heroes adalah mereka karakter yang dapat dikendalikan hanya dalam mode RPG
#[derive(Component)]
pub struct Heroes {
    nama: String,
}

impl Heroes {
    pub fn new(nama: &str) -> Self {
        Self {
            nama: nama.to_string(),
        }
    }
}

// Unit adalah mereka karakter yang dapat dikendalikan hanya dalam mode RTS.
// Biasanya Heroes == Unit, tapi Unit != Heroes
#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct HeroesId {
    pub id: usize,
}

impl HeroesId {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}
