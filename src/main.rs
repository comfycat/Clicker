use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    // Creates constants for button location
    let (button_x, button_y, button_r, button_color) = 
        (screen_width() / 2.0, screen_height() / 2.0, 50.0, BLUE);
    // Initalizes the counter variable for counting the player's points
    let mut counter = 0;

    loop {
        // Colors the background
        clear_background(PINK);

        // Creates the button the player presses to get points
        draw_circle(button_x, button_y, button_r, button_color);
        // Checks if the mouse was pressed on this frame
        let mouse_pressed = is_mouse_button_pressed(MouseButton::Left);
        // If the player presses the main button, it gives them a point
        if mouse_pressed && mouse_in_circle(button_x, button_y, button_r) {
            counter += 1;
        }
        // Displays the number of points that the player has
        let text = format!("Counter: {}", counter);
        draw_text(&text, 40.0, 70.0, 30.0, DARKGRAY);
        // Waits until it's time to draw the next frame
        next_frame().await
    }
}

// Returns `true` if the mouse is inside the given circle
// Reusable if we add multiple buttons in the future
// x, y: coordinates of circle
// r: radius of circle
fn mouse_in_circle(x: f32, y: f32, r: f32) -> bool {
    let (mouse_x, mouse_y) = mouse_position();
    let distance_from_center = ((y-mouse_y).powi(2) + (x-mouse_x).powi(2)).sqrt();
    return distance_from_center <= r;
}
