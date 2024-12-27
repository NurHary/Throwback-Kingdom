
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

pub fn mouse_pan(fin_pos:[f32;2], last_mpos:(f32, f32), cur_mpos:(f32, f32), delt_t: f32, pan_sp: f32, mov_pos: (f32, f32)) -> ([f32;2], (f32, f32)){
    if mov_pos == cur_mpos{
        return (fin_pos, (0.0, 0.0))
    }
    else{
        return ([fin_pos[0] + (last_mpos.0 - cur_mpos.0) * delt_t * pan_sp , fin_pos[1] + (last_mpos.1 - cur_mpos.1) * delt_t * pan_sp], cur_mpos)
    }
}
