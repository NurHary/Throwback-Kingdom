use macroquad::prelude::*;

mod character_entity;
use crate::character_entity::*;
mod camera_tk;


enum GameState{
    Play,
    Pause,
    Menu,
    Setting,
    GameOver
}


#[macroquad::main("Throwback Kingdom")]
async fn main() {

    let mut accel = 0.0;
    let delta_time = get_frame_time();
    let fps = get_fps();
    
    let game_start_status = GameState::Play;

    let mut player = CharacterEntity{
        name: "player utama".to_string(),
        speed:  200.0,
        x: screen_width()/2.0,
        y: screen_height()/2.0,
        tipe_character: CharacterType::PLAYER
    };
    println!("nama pemain utama: {}",player.name);
    
    
    loop {
        match game_start_status{
            GameState::Menu => {},

            GameState::Play => {
                let mvspeed = accel + f32::powf(accel,3.0 );
                clear_background(SKYBLUE);


                let kunci = get_keys_down();
                println!("{:?}",kunci);
                
                // input Movement
                if is_key_down(KeyCode::D) {
                    player.x += mvspeed * delta_time * player.speed;
                    accel += 0.15;
                    accel = clamp(accel, 0.0, 0.685);
                }
                if is_key_down(KeyCode::A) {
                    player.x -= mvspeed * delta_time * player.speed;
                    accel += 0.15;
                    accel = clamp(accel, 0.0, 0.685);
                }
                if is_key_down(KeyCode::S) {
                    player.y += mvspeed * delta_time * player.speed;
                    accel += 0.15;
                    accel = clamp(accel, 0.0, 0.685);
                }
                if is_key_down(KeyCode::W) {
                    player.y -= mvspeed * delta_time * player.speed;
                    accel += 0.15;
                    accel = clamp(accel, 0.0, 0.685);
                }

                if is_key_released(KeyCode::W) {accel = 0.0;}
                if is_key_released(KeyCode::A) {accel = 0.0;}
                if is_key_released(KeyCode::D) {accel = 0.0;}
                if is_key_released(KeyCode::S) {accel = 0.0;}


                // Placeholder Player
                draw_circle(player.x, player.y, 15.0, YELLOW);

                draw_text( format!("{} {}",player.x,player.y).as_str() , (screen_width()/8.0) - 100.0, (screen_height()/8.0) - 50.0, 30.0, DARKGRAY );
                draw_text (format!("{} {}",accel, mvspeed).as_str(),(screen_width()/8.0) - 100.0,(screen_height()/8.0) - 20.0,30.0,DARKGRAY);
                draw_text (format!("{}",fps).as_str(),screen_width() - 30.0,(screen_height()/8.0) - 50.0,30.0,DARKGRAY);
                
                next_frame().await
            },

            GameState::Pause => {},

            GameState::Setting => {},

            GameState::GameOver=> {},
            
        }
    }
}

