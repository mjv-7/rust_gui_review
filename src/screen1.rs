use macroquad::prelude::*;
use crate::modules::text_button::TextButton;
use crate::modules::grid::{self, draw_grid};

fn draw_grid_standard(grid_size: f32, color: Color) {
    let screen_width = screen_width();
    let screen_height = screen_height();
    
    // Draw vertical lines and labels
    for x in (0..screen_width as i32).step_by(grid_size as usize) {
        draw_line(x as f32, 0.0, x as f32, screen_height, 1.0, color);
        draw_text(&format!("{}", x), x as f32 + 2.0, 12.0, 16.0, color);
    }
    
    // Draw horizontal lines and labels
    for y in (0..screen_height as i32).step_by(grid_size as usize) {
        draw_line(0.0, y as f32, screen_width, y as f32, 1.0, color);
        draw_text(&format!("{}", y), 2.0, y as f32 + 12.0, 16.0, color);
    }
}
pub async fn run() -> String {
     let btn_text = TextButton::new(
        100.0,
        200.0,
        200.0,
        60.0,
        "Click Me",
        BLACK,
        GREEN,
        30
     );
    loop {
        clear_background(BLUE);
        draw_text("Screen 1", 20.0, 40.0, 30.0, WHITE);
    
        if btn_text.click() || is_key_down(KeyCode::Enter) {
            return "screen2".to_string();
        }
        draw_grid(50.0, BLACK);
        next_frame().await;
        
    }
}