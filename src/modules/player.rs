use::macroquad::prelude::*;
use crate::modules::still_image::StillImage;
pub struct Player {
    view: StillImage,
    move_speed: f32,
    //movement: Vec2

}

impl Player {
    pub async fn new(
        asset_path: String,
        move_speed: f32,
        x: f32, 
        y: f32,
        width: f32,
        height: f32
    /*movement: Vec2*/) -> Self{
        Player {
            view:StillImage::new(&asset_path, 50.0, 50.0, x, y, true, 1.0).await,
            move_speed: 3.0
        
        }
    }
    pub fn key_press(&mut self) -> &Self{
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
      // Apply movement based on frame time
    let movement = move_dir * self.move_speed * get_frame_time();
    self
    }
    pub fn get_pos(&mut self, x: f32, y:f32) -> &Self {
        self.view.set_x(x);
        self.view.set_y(y);
        self
    }
    pub fn position(&self) -> Vec2 {
        vec2(self.view.get_x(), self.view.get_y())
    }
    pub fn get_x(&self, x: f32) {
        self.x
    }
    pub fn get_y(){
        
    }
    pub fn set_x(){
        
    }
    pub fn set_y(){
        
    }   
    pub fn draw(&mut self){
        self.view.draw();
    }
}