
pub struct CharacterEntity{
    pub name: String,
    pub speed: f32,
    pub x: f32,
    pub y:f32,
    pub tipe_character: CharacterType,
}


impl CharacterEntity{
    pub fn new(nama:String, kecepatan: f32, x_pos:f32, y_pos:f32, tipe:CharacterType) -> Self{
        Self { name: nama, speed: kecepatan, x: x_pos , y: y_pos, tipe_character: tipe }
    }
}
pub enum CharacterType{
    PLAYER,
    NPC,
    ALLY(i32),
    ENEMY(i32),
    
}
