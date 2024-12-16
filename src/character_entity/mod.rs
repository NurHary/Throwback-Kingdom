
pub struct CharacterEntity{
    pub name: String,
    pub nyawa: i32,
    pub speed: f32,
    pub x: f32,
    pub y:f32,
    pub tipe_character: CharacterType,
    pub sprite: SpriteTypes,
    pub id: usize,
}


impl CharacterEntity{
    pub fn new(nama:String, nyowo:i32, kecepatan: f32, x_pos:f32, y_pos:f32, tipe:CharacterType, sprite: SpriteTypes, id:usize) -> Self{
        Self { name: nama, nyawa:nyowo, speed: kecepatan, x: x_pos , y: y_pos, tipe_character: tipe, sprite, id}
    }
    pub fn move_current(mut self){
        
    }

    pub fn take_damage(mut self){
        
    }
}

pub enum CharacterType{
    PLAYER,
    NPC(i32)
}

pub struct NonCharacterEntity{
    pub name: String,
    pub id:u16,
    pub tipe: NonCharacterType,
    pub speed: f32,
    pub sprite: SpriteTypes,
}

impl NonCharacterEntity{
    pub fn new(name:String, id:u16, tipe: NonCharacterType, speed:f32, sprite:SpriteTypes)-> Self{
        Self{name, id, tipe, speed, sprite}
    }

    pub fn move_current(){
        
    }
}

pub enum NonCharacterType{
    Ally(i32, i32, Option<i32>), //id, nyawa, dan damage
    Enemy(i32, i32, Option<i32>),//id, nyawa, dan damage
    Wild(i32, i32, Option<i32>) //id, nyawa, dan damage
}


pub enum SpriteTypes{
    Animation,
    Singular,
    Placeholder,
}
