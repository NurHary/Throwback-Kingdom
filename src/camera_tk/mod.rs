use macroquad::prelude::*;
use crate::character_entity::CharacterEntity;

pub struct CamUnit <'a>{
    unit: & 'a CharacterEntity,
    zoom:f32
}

impl <'a> CamUnit <'a>{
    pub fn new(unit: & 'a CharacterEntity, zoom:f32) -> Self{
        Self{
            unit,
            zoom
        }
    }

    pub fn initcam(self){
        set_camera(&Camera2D{
            target: Vec2 { x: self.unit.x, y: self.unit.y },
            ..Default::default()
        })
    }

    pub fn latchon(self, unit: & 'a CharacterEntity){
        
    }
}
