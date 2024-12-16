
pub fn count_key_pressed(y_pos: bool, x_neg: bool, y_neg: bool, x_pos: bool) -> i32{
    let mut nilai:i32 = y_pos as i32 + x_neg as i32 + y_neg as i32 + x_pos as i32;
    nilai
}

pub fn get_vector(y_pos: bool, x_neg: bool, y_neg: bool, x_pos: bool) -> [f32;2]{ //ini akan mengembalikan nilai / arah dari tombol2 yang dipencet
    let mut nilai = [0.0,0.0];

    if y_pos as i32 + x_neg as i32 + y_neg as i32 + x_pos as i32 > 0 {
        if y_pos{nilai[1] += 1.0};
        if x_pos{nilai[0] += 1.0 };
        if y_neg{nilai[1] -= 1.0};
        if x_neg{nilai[0] -= 1.0}; 
    }  
    nilai // return nilai
    
}

pub fn input_mouse_wheel<T>(input:i32, renilai:T) -> T{
    renilai
}
