use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

#[derive(Debug, Resource)]
pub struct DebugCurrentPosition {
    pub pos: Vec3,
}

pub fn show_current_position(
    mut contest: EguiContexts,
    current_pos: Res<DebugCurrentPosition>,
) -> Result {
    egui::Window::new("Hello Jajal").show(contest.ctx_mut()?, |ui| {
        let isian_posisi = format!(
            "x: {}, y:{}, z:{}",
            current_pos.pos.x, current_pos.pos.y, current_pos.pos.z
        );
        ui.label(isian_posisi);
    });
    Ok(())
}
