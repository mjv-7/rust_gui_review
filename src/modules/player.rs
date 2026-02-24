use::macroquad::prelude::*;
use crate::modules::still_image::StillImage;
pub struct Player {
    view: StillImage,
    move_speed: f32,

}

impl Player {
    pub fn key_press(){
                let mut move_dir = vec2(0.0, 0.0);

    // Keyboard input
    if is_key_down(KeyCode::D) {
        move_dir.x += 1.0;
    }
    if is_key_down(KeyCode::A) {
        move_dir.x -= 1.0;
    }
    if is_key_down(KeyCode::S) {
        move_dir.y += 1.0;
    }
    if is_key_down(KeyCode::W) {
        move_dir.y -= 1.0;
    }

    // Normalize the movement to prevent faster diagonal movement
    if move_dir.length() > 0.0 {
        move_dir = move_dir.normalize();
    }
    }
    pub fn new(&mut self, ){

    }
}