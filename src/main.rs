use macroquad::prelude::*;

use upgrade::Upgrade;

mod upgrade;

#[macroquad::main("BasicShapes")]
async fn main() {
    // Creates constants for button location
    let (button_x, button_y, button_r, button_color) = 
        (screen_width() / 4.0, screen_height() / 2.0, 50.0, BLUE);
    // Creates constants for upgrade box
    let (upgrade_zone_x, upgrade_zone_y, upgrade_zone_w, upgrade_zone_h, upgrade_zone_color) =
        (screen_width() * 0.55, screen_height() * 0.1, screen_width() * 0.4, screen_height() * 0.8, GRAY);
    // Creates constants for inner upgrade size
    let (upgrade_padding_x, upgrade_padding_y, upgrade_w, upgrade_h) = 
        (upgrade_zone_x + upgrade_zone_w * 0.05, upgrade_zone_h * 0.06, 
        upgrade_zone_w * 0.9, upgrade_zone_h * 0.1);
    // Initalizes the counter variable for counting the player's points
    let mut counter = 0;

    let mut upgrades = vec![
        // pub fn new(width: f32, height: f32, cost: i32, owned: i32, onetime: bool, text: &str) -> Upgrade
        Upgrade::new(upgrade_w, upgrade_h, 5, 0, true, "Upgrade 1"),
        Upgrade::new(upgrade_w, upgrade_h, 10, 0, true, "Upgrade 2"),
        // ...
    ];

    loop {
        // Colors the background
        clear_background(PINK);
        // Checks if the mouse was pressed on this frame
        let mouse_pressed = is_mouse_button_pressed(MouseButton::Left);

        // Creates the button the player presses to get points
        draw_circle(button_x, button_y, button_r, button_color);
        // If the player presses the main button, it gives them a point
        if mouse_pressed && mouse_in_circle(button_x, button_y, button_r) {
            counter += 1;
        }

        // Creates the area for upgrades
        draw_rectangle(upgrade_zone_x, upgrade_zone_y, upgrade_zone_w, upgrade_zone_h, upgrade_zone_color);
        // Renders the upgrades, and checks for purchases
        for (i, upgrade) in upgrades.iter_mut().enumerate() {
            // Factor out upgrades' x, y, width, height? We need it in the call to mouse_in_rectangle
            let upgrade_x = upgrade_zone_x + upgrade_zone_w * 0.05;
            let upgrade_y = upgrade_zone_y + upgrade_padding_y * (2 * i + 1) as f32;
            // Renders the upgrades
            upgrade.render(upgrade_x, upgrade_y);
            // If the player clicks on an upgrade, it tries to purchase that upgrade
            // Deducts the number of points spent, which is returned by the purchase function
            if mouse_pressed && mouse_in_rectangle(upgrade_x, upgrade_y, upgrade_w, upgrade_h){
                let deduction = upgrade.purchase(counter);
                counter -= deduction;
            }
        }
        // Old code: draw_rectangle(upgrade_zone_x + upgrade_zone_w * 0.05, upgrade_zone_y + upgrade_zone_y * 0.05, upgrade_w, upgrade_h, upgrade_color);

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

// Returns `true` if the mouse is inside the given 
// x, y: top left coordinates of rectangle
// w, h: width and height of rectangle
fn mouse_in_rectangle(x: f32, y: f32, w: f32, h: f32) -> bool {
    let (mouse_x, mouse_y) = mouse_position();
    let mouse_x_check = x < mouse_x && mouse_x < x + w;
    let mouse_y_check = y < mouse_y && mouse_y < y + h;
    return mouse_x_check && mouse_y_check;
}