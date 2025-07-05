use bevy::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    App::new().run();
}
