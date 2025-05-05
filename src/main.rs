use macroquad::prelude::*;

mod character_entity;
mod tools;
mod world_gen;
use crate::character_entity::*;

mod tk_system;
use crate::tk_system::*;

mod tilemaps;

const PAN_SPEED: f32 = 10.0;

enum GameState {
    Play,
    Menu,
    GameOver,
}

#[macroquad::main("Throwback Kingdom")]
async fn main() {
    let mut accel = 0.0;

    let game_start_status = GameState::Play;

    let mut character_list: Vec<CharacterEntity> = vec![
        CharacterEntity::new(
            // character entity 1 / main
            "King Anton".to_string(),
            100,
            300.0,
            screen_width() / 2.0,
            screen_height() / 2.0,
            30.0,
            CharacterType::PLAYER,
            SpriteTypes::Placeholder,
            0,
        ),
        CharacterEntity::new(
            // character entity 2
            "Waiter Alfred".to_string(),
            100,
            300.0,
            screen_width() / 4.0,
            screen_height() / 2.0,
            30.0,
            CharacterType::PLAYER,
            SpriteTypes::Placeholder,
            1,
        ),
        CharacterEntity::new(
            "Waiter Alfred".to_string(),
            100,
            300.0,
            screen_width() / -2.0,
            screen_height() / 2.0,
            30.0,
            CharacterType::PLAYER,
            SpriteTypes::Placeholder,
            3,
        ),
    ];
    // character_list.remove(2);

    let mut character_main_id = 0;
    let mut camera_zoom_mode: f32 = 2.0;

    let mut ls_pos_cam: [f32; 2] = [0.0, 0.0]; // inisialisasi Posisi Camera Berdasarkan Player
    let mut final_rpg_position: [f32; 2] = [0.0, 0.0];
    let mut current_mouse_position: (f32, f32) = (0.0, 0.0); // Mouse Position For Difference
    let mut mous_mov_pos: (f32, f32) = (1.0, 1.0); // ini untuk melakukan pengecekan apakah posisi mouse saat ini sudah sama atau tidak

    let mut game_mode: bool = true; // true = RPG, false = RTS

    loop {
        let delta_time = get_frame_time();

        let fps = get_fps();
        match game_start_status {
            GameState::Menu => {}

            GameState::Play => {
                // test mouse
                let mospos = mouse_position();

                clear_background(SKYBLUE);

                // Game Mode Switch
                if is_key_pressed(KeyCode::Tab) {
                    game_mode = !game_mode; // Mengganti Mode dari RPG ke RTS atau sebaliknya
                    final_rpg_position = [
                        character_list[character_main_id].x,
                        character_list[character_main_id].y,
                    ]; // untuk posisi camera terakhir dari player ketika transisi
                    if game_mode == false {
                        camera_zoom_mode = 1.0
                    };
                }

                match game_mode {
                    true => {
                        //  RPG MODE //
                        camera_zoom_mode = 2.0;
                        // Player Movement
                        let mut mvspeed = accel + f32::powf(accel, 3.0);
                        let movement_pressed = count_key_pressed(
                            is_key_down(KeyCode::W),
                            is_key_down(KeyCode::A),
                            is_key_down(KeyCode::S),
                            is_key_down(KeyCode::D),
                        );
                        let vector_pressed = get_vector(
                            is_key_down(KeyCode::S),
                            is_key_down(KeyCode::A),
                            is_key_down(KeyCode::W),
                            is_key_down(KeyCode::D),
                        ); // ini ada untuk mendapatkan nilai dari tombol2 yang dipencet (khusus untuk pergerakan)

                        if is_key_pressed(KeyCode::F) {
                            character_main_id += 1;
                        }
                        character_main_id = character_main_id % character_list.len();

                        if is_key_pressed(KeyCode::P) {
                            println!("{:?}", test_id(&character_list[character_main_id]))
                        }

                        if movement_pressed > 0 {
                            accel += 1.5 * (delta_time * 2.0);
                            accel = clamp(accel, 0.0, 0.685);
                        } else {
                            accel = 0.0
                        }

                        mvspeed = if movement_pressed >= 2 {
                            mvspeed / 1.5
                        } else {
                            mvspeed
                        };

                        character_list[character_main_id].x += vector_pressed[0]
                            * (mvspeed * delta_time * character_list[character_main_id].speed);
                        character_list[character_main_id].y += vector_pressed[1]
                            * (mvspeed * delta_time * character_list[character_main_id].speed);

                        ls_pos_cam = [
                            character_list[character_main_id].x,
                            character_list[character_main_id].y,
                        ]; // ini untuk mendapatkan nilai ketika plyaer bergerak pada RPG mode

                        // if character_list.iter().any(|character_list| player.collide_with(character_list)){
                        // println!("coollide!!")} // ini untuk collision dalam vector

                        // <-#c must find a better way to implement colision
                        for i in 0..character_list.len() {
                            //  Collision
                            if i == character_main_id {
                                continue;
                            }
                            let coll =
                                character_list[character_main_id].colided_with(&character_list[i]);
                            if coll.0 {
                                match coll.1.w > coll.1.h {
                                    true => {
                                        match character_list[character_main_id].y
                                            > character_list[i].y
                                        {
                                            true => character_list[character_main_id].y += coll.1.h,
                                            false => {
                                                character_list[character_main_id].y -= coll.1.h
                                            }
                                        }
                                    }
                                    false => {
                                        match character_list[character_main_id].x
                                            > character_list[i].x
                                        {
                                            true => character_list[character_main_id].x += coll.1.w,
                                            false => {
                                                character_list[character_main_id].x -= coll.1.w
                                            }
                                        }
                                    }
                                };
                            };
                        } // close For Loop
                    } // Close RPG Mode
                    false => {
                        // RTS MODE //

                        if is_mouse_button_pressed(MouseButton::Middle) {
                            current_mouse_position = mouse_position();
                        }

                        // Camera Panning System
                        if is_mouse_button_down(MouseButton::Middle) {
                            // masalahnya adalah ini diupdate setiap framenya sehingga terjadi signifikasi / perubahan
                            (final_rpg_position, mous_mov_pos) = mouse_pan(
                                final_rpg_position,
                                current_mouse_position,
                                mospos,
                                delta_time,
                                PAN_SPEED,
                                mous_mov_pos,
                            );
                            if mous_mov_pos == (0.0, 0.0) {
                                current_mouse_position = mouse_position();
                            }
                        }

                        ls_pos_cam = [final_rpg_position[0], final_rpg_position[1]];
                    }
                }

                // Camera

                set_camera(&Camera2D {
                    zoom: Vec2 {
                        x: camera_zoom_mode / screen_width(),
                        y: camera_zoom_mode / screen_height(),
                    },
                    target: Vec2 {
                        x: ls_pos_cam[0],
                        y: ls_pos_cam[1],
                    }, // Target akan diubah ke ls_pos_cam
                    ..Default::default()
                });

                // Placeholder Player
                (0..character_list.len()).for_each(|i| {
                    if let SpriteTypes::Placeholder = character_list[i].sprite {
                        draw_rectangle(
                            character_list[i].x,
                            character_list[i].y,
                            character_list[i].size,
                            character_list[i].size,
                            YELLOW,
                        )
                    }
                }); // bekerja sesuai ekspektasi

                draw_text(
                    format!(
                        "{} {}",
                        character_list[character_main_id].x, character_list[character_main_id].y
                    )
                    .as_str(),
                    (screen_width() / 8.0) - 100.0,
                    (screen_height() / 8.0) - 50.0,
                    30.0,
                    DARKGRAY,
                );
                draw_text(
                    format!("{}", fps).as_str(),
                    screen_width() - 30.0,
                    (screen_height() / 8.0) - 50.0,
                    30.0,
                    DARKGRAY,
                );

                set_default_camera();
                next_frame().await
            }

            GameState::GameOver => {}
        }
    }
}

fn test_delete(mut data: Vec<CharacterEntity>) -> Vec<CharacterEntity> {
    println!("ok");
    data.remove(data.len() - 1);
    data
}

fn test_id(char: &CharacterEntity) -> usize {
    char.id
}
