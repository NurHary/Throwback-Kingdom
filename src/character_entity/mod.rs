use macroquad::prelude::*;

pub struct CharacterEntity{
    pub name: String,
    pub nyawa: i32,
    pub speed: f32,
    pub x: f32,
    pub y:f32,
    pub size:f32,
    pub tipe_character: CharacterType,
    pub sprite: SpriteTypes,
    pub id: usize,
}


impl CharacterEntity{
    pub fn new(nama:String, nyowo:i32, kecepatan: f32, x_pos:f32, y_pos:f32, size:f32, tipe:CharacterType, sprite: SpriteTypes, id:usize) -> Self{
        Self { name: nama, nyawa:nyowo, speed: kecepatan, x: x_pos , y: y_pos, size, tipe_character: tipe, sprite, id}
    }
    pub fn move_current(mut self){
        
    }

    pub fn take_damage(mut self){
        
    }

    pub fn colided_with(&self, other: &Self) -> bool{
        self.rect().overlaps(&other.rect())
    }

    pub fn rect(&self) -> Rect{ // ini untuk collision secara traditionalnya
        macroquad::math::Rect{
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h:self.size
        }
    }
}

pub enum CharacterType{
    PLAYER,
    NPC(i32)
}


pub enum SpriteTypes{
    Animation,
    Singular,
    Placeholder,
}
