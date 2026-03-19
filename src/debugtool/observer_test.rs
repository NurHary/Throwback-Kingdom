use bevy::prelude::*;

#[derive(Event)]
struct Jajal;

fn test_trigger(key: Res<ButtonInput<KeyCode>>, mut command: Commands) {
    if key.just_pressed(KeyCode::KeyT) {
        command.trigger(Jajal);
    }
}

fn test_observer(_: On<Jajal>) {
    info!("Dipencet t");
}

pub struct TkDebugObserver;
impl Plugin for TkDebugObserver {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, test_trigger);
        app.add_observer(test_observer);
    }
}
