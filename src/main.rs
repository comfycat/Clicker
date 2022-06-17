use std::time::{Instant, Duration};

use macroquad::prelude::*;

use upgrade::Upgrade;
use gamevalues::Gamevalues;

mod upgrade;
mod gamevalues;

#[macroquad::main("Clicker Game")]
async fn main() {
    // Creates constants for button location
    let (button_x, button_y, button_r, button_color) = 
        (screen_width() * 0.25, screen_height() * 0.5, 50.0, BLUE);
    // Creates constants for upgrade box
    let (upgrade_zone_x, upgrade_zone_y, upgrade_zone_w, upgrade_zone_h, upgrade_zone_color) =
        (screen_width() * 0.55, screen_height() * 0.1, screen_width() * 0.4, screen_height() * 0.8, GRAY);
    // Creates constants for hide upgrades box 
    let (hide_upgrade_x, hide_upgrade_y, hide_upgrade_w, hide_upgrade_h, hide_upgrade_color, 
        hide_upgrade_text_y, hide_upgrade_text_color) =
        (screen_width() * 0.85, screen_height() * 0.02, screen_width() * 0.1, screen_height() * 0.08, DARKGREEN, 
        screen_height() * 0.08, PURPLE);
    // Creates constants for inner upgrade size
    let (upgrade_padding_x, upgrade_padding_y, upgrade_w, upgrade_h) = 
        (upgrade_zone_x + upgrade_zone_w * 0.05, upgrade_zone_h * 0.06, 
        upgrade_zone_w * 0.9, upgrade_zone_h * 0.1);
    // Initalizes the hidden variable for hiding the upgrades screen
    let mut hidden = false;
    // Initalizes the counter variable for counting the player's points
    let mut counter = 0;
    // Initalizes the gamevalues variable as a struct for maximizing player value from upgrades
    // TESTING Initalizing the persecond variable to determine points gained per second in the gamevalues struct
    let mut gamevalues = Gamevalues::new(1, 1, 0);
    // Creates the reference for counting seconds with
    let mut game_timer = Instant::now();
    
    // Creates a vector containing all of the upgrades
    let mut upgrades = vec![
        // pub fn new(width: f32, height: f32, cost: i32, onetime: i32, owned: bool, text: &str) -> Upgrade
        Upgrade::new(upgrade_w, upgrade_h, 5, 0, true, "Upgrade 1", Box::new(|gamevalues: &mut Gamevalues| {
            gamevalues.clickpow_add += 1;
        })),
        Upgrade::new(upgrade_w, upgrade_h, 30, 0, true, "Upgrade 2", Box::new(|gamevalues: &mut Gamevalues| {
            gamevalues.clickpow_mult *= 2;
        })),
        Upgrade::new(upgrade_w, upgrade_h, 20, 0, false, "Upgrade 3", Box::new(|gamevalues: &mut Gamevalues| {
            gamevalues.persecond += 1;
        })),
        // ...
    ];

    // The main loop which creates the game
    loop {
        // Colors the background
        clear_background(PINK);
        // Checks if the mouse was pressed on this frame
        let mouse_pressed = is_mouse_button_pressed(MouseButton::Left);

        // Creates the button the player presses to get points
        draw_circle(button_x, button_y, button_r, button_color);
        // If the player presses the main button, it gives them a point
        if mouse_pressed && mouse_in_circle(button_x, button_y, button_r) {
            counter += gamevalues.get_clickpower();  
        }
        
        // Checks to see if a second has passed for timing, if one has, resets the time since the last second was counted
        // For adding income per second upgrades
        if game_timer.elapsed() > Duration::from_secs(1) {
            // Adds the income per second in the Gamevalues struct
            counter += gamevalues.persecond;
            game_timer = Instant::now();
        }

        // Upgrades Rendering
        // Checks if the upgrades are set to be visible, does not render if not
        if !hidden {
            // Creates the area for upgrades
            draw_rectangle(upgrade_zone_x, upgrade_zone_y, upgrade_zone_w, upgrade_zone_h, upgrade_zone_color);
            // Renders the upgrades, and checks for purchases
            for (i, upgrade) in upgrades.iter_mut().enumerate() {
                // Factors out each upgrade's x, y, width, and height for use in the mouse_in_rectangle function
                let upgrade_x = upgrade_zone_x + upgrade_zone_w * 0.05;
                let upgrade_y = upgrade_zone_y + upgrade_padding_y * (2 * i + 1) as f32;
                // Renders the upgrades
                upgrade.render(upgrade_x, upgrade_y);
                // If the player clicks on an upgrade, it tries to purchase that upgrade
                // Deducts the number of points spent, which is returned by the purchase function
                if mouse_pressed && mouse_in_rectangle(upgrade_x, upgrade_y, upgrade_w, upgrade_h){
                    let deduction = upgrade.purchase(counter, &mut gamevalues);
                    counter -= deduction;
                }
            }
        }

        // Renders the hide upgrade button
        draw_rectangle(hide_upgrade_x, hide_upgrade_y, hide_upgrade_w, hide_upgrade_h, hide_upgrade_color);
        // Renders Show / Hide based off of hide status
        // Uses the hide_upgrade_x value because text coordinates start at the left
        if hidden {     
            draw_text("Show", hide_upgrade_x, hide_upgrade_text_y, 22.0, hide_upgrade_text_color);
        } else {
            draw_text("Hide", hide_upgrade_x, hide_upgrade_text_y, 22.0, hide_upgrade_text_color);
        }
        // If the player clicks on the hide option, it hides the upgrade zone
        // Deducts the number of points spent, which is returned by the purchase function
        if mouse_pressed && mouse_in_rectangle(hide_upgrade_x, hide_upgrade_y, hide_upgrade_w, hide_upgrade_h) {
            hidden = !hidden;
        }
        // Old code: draw_rectangle(upgrade_zone_x + upgrade_zone_w * 0.05, upgrade_zone_y + upgrade_zone_y * 0.05, upgrade_w, upgrade_h, upgrade_color);

        // Displays the number of points that the player has
        let player_points = format!("Counter: {}", counter);
        draw_text(&player_points, 40.0, 70.0, 30.0, DARKGRAY);

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