use std::time::{Instant, Duration};

use macroquad::prelude::*;

use upgrade::Upgrade;
use gamevalues::Gamevalues;
use alchemy::Alchemy;
use alchemyitems::Alchemyitems;

mod upgrade;
mod gamevalues;
mod alchemy;
mod alchemyitems;

#[macroquad::main("Clicker Game")]
async fn main() {
    // Creates constants for button location
    let (button_x, button_y, button_r, button_color) = 
        (screen_width() * 0.25, screen_height() * 0.55, 50.0, BLUE);
    // Creates constants for upgrade box
    let (upgrade_zone_x, upgrade_zone_y, upgrade_zone_w, upgrade_zone_h, upgrade_zone_color) =
        (screen_width() * 0.55, screen_height() * 0.1, screen_width() * 0.4, screen_height() * 0.8, GRAY);
    // Creates constants for hide upgrades box 
    let (hide_upgrade_x, hide_upgrade_y, hide_upgrade_w, hide_upgrade_h, hide_upgrade_color, 
        hide_upgrade_text_y, hide_upgrade_text_color) =
        (screen_width() * 0.85, screen_height() * 0.02, screen_width() * 0.1, screen_height() * 0.08, DARKGREEN, 
        screen_height() * 0.08, PURPLE);
    // Creates constants for inner upgrade size
    let (_upgrade_padding_x, upgrade_padding_y, upgrade_w, upgrade_h) = 
        (upgrade_zone_x + upgrade_zone_w * 0.05, upgrade_zone_h * 0.06, 
        upgrade_zone_w * 0.9, upgrade_zone_h * 0.1);
    // Creates constants for the alchemy icon, and alchemy zone
    let (alchemy_icon_x, alchemy_icon_y, alchemy_icon_w, alchemy_icon_h, alchemy_icon_color) = 
        (0.0, screen_height() * 0.9, screen_width() * 0.1, screen_height() * 0.1, BLACK);
    let (alchemy_zone_x, alchemy_zone_y, alchemy_zone_w, alchemy_zone_h, alchemy_zone_color) = 
        (0.0, screen_height() * 0.4, screen_width() * 0.5, screen_height() * 0.5, GRAY);
    // Initalizes the hidden variable for hiding the upgrades screen
    let mut hidden = false;
    // Initalizes the counter variable for counting the player's points
    let mut counter = 0;
    let alchemy_items = vec![
        Alchemyitems::new("Water Bottle", 0, BLUE, Box::new(|| {
            
        })),
        Alchemyitems::new("Water Bottle", 0, BEIGE, Box::new(|| {

        })),
        Alchemyitems::new("Water Bottle", 0, BROWN, Box::new(|| {
            
        })),
        Alchemyitems::new("Water Bottle", 0, YELLOW, Box::new(|| {
            
        })),
        Alchemyitems::new("Water Bottle", 0, RED, Box::new(|| {
            
        }))
        ];
    // Initalizes the Alchemy variable as a struct
    let gamealchemy = Alchemy::new(alchemy_zone_w, alchemy_zone_h, false, false, 
        0.0, 10.0, alchemy_items);
    // Initalizes the gamevalues variable as a struct for maximizing player value from upgrades
    // TESTING Initalizing the persecond variable to determine points gained per second in the gamevalues struct
    let mut gamevalues = Gamevalues::new(1, 1, 0, gamealchemy);
    // Creates the reference for counting seconds with
    let mut game_timer = Instant::now();
    
    // Creates a vector containing all of the upgrades
    let mut upgrades = vec![
        // pub fn new(width: f32, height: f32, cost: i32, owned: i32, onetime: bool, text: &str) -> Upgrade
        Upgrade::new(upgrade_w, upgrade_h, 5, 0, true, "Increased Click Power", Box::new(|gamevalues: &mut Gamevalues| {
            gamevalues.clickpow_add += 1;
        })),
        Upgrade::new(upgrade_w, upgrade_h, 30, 0, true, "Double Click Power", Box::new(|gamevalues: &mut Gamevalues| {
            gamevalues.clickpow_mult *= 2;
        })),
        Upgrade::new(upgrade_w, upgrade_h, 20, 0, false, "Points Per Second", Box::new(|gamevalues: &mut Gamevalues| {
            gamevalues.persecond += 1;
        })),
        Upgrade::new(upgrade_w, upgrade_h, 1, 0, true, "Alchemy", Box::new(|gamevalues: &mut Gamevalues| {
            gamevalues.alchemy.unlocked = true;
        })),
        Upgrade::new(upgrade_w, upgrade_h, 1, 0, false, "Water Bottle", Box::new(|gamevalues: &mut Gamevalues| {
            gamevalues.alchemy.items[0].owned += 1;
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
        // If the alchemy screen is open, the main button is hidden so clicking should not give any points
        if !gamevalues.alchemy.visible {
            if mouse_pressed && mouse_in_circle(button_x, button_y, button_r) {
                counter += gamevalues.get_clickpower();  
            }
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

        // Alchemy Rendering
        // Checks if Alchemy has been unlocked, if yes shows the alchemy icon
        if gamevalues.alchemy.unlocked {
            // Checks to see if the alchemy screen is currently opened
            // If yes, renders the alchemy screen and the icon to close, if no, just renders the icon to open
            if gamevalues.alchemy.visible {
                // Renders the alchemy zone
                draw_rectangle(alchemy_zone_x, alchemy_zone_y, alchemy_zone_w, alchemy_zone_h, alchemy_zone_color);
                // Renders the water bar
                gamevalues.alchemy.render_water(alchemy_zone_x, alchemy_zone_y);
                // Renders the alchemy items, passing upgrades to see if it got used
                gamevalues.alchemy.render_items(alchemy_zone_x, alchemy_zone_y, mouse_pressed);
            }
            
            // Checks the mouse clicked on the alchemy icon to open the alchemy zone
            if mouse_pressed && mouse_in_rectangle(alchemy_icon_x, alchemy_icon_y, alchemy_icon_w, alchemy_icon_h) {
                gamevalues.alchemy.visible = !gamevalues.alchemy.visible;
            }
            // Creates the icon whether the alchemy screen is open or not
            draw_rectangle(alchemy_icon_x, alchemy_icon_y, alchemy_icon_w, alchemy_icon_h, alchemy_icon_color);
        }

        // Renders the hide upgrade button
        draw_rectangle(hide_upgrade_x, hide_upgrade_y, hide_upgrade_w, hide_upgrade_h, hide_upgrade_color);
        // Renders Show / Hide based off of hide status
        // Uses the hide_upgrade_x value because text coordinates start at the left
        if hidden {     
            draw_text("Show", hide_upgrade_x, hide_upgrade_text_y, scale_text_in_box(hide_upgrade_w,
                 hide_upgrade_h, 0.0, "Show"), hide_upgrade_text_color);
        } else {
            draw_text("Hide", hide_upgrade_x, hide_upgrade_text_y, scale_text_in_box(hide_upgrade_w, 
                hide_upgrade_h, 0.0, "Hide"), hide_upgrade_text_color);
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
pub fn mouse_in_rectangle(x: f32, y: f32, w: f32, h: f32) -> bool {
    let (mouse_x, mouse_y) = mouse_position();
    let mouse_x_check = x < mouse_x && mouse_x < x + w;
    let mouse_y_check = y < mouse_y && mouse_y < y + h;
    return mouse_x_check && mouse_y_check;
}

// Checks for the maximum font size that will fit in the given box, and returns that size
// box_w, box_h, y_offset: width, height, and y offset of box to test
// input_text: text to be put inside of box
pub fn scale_text_in_box(box_w: f32, box_h: f32, y_offset: f32, input_text: &str) -> f32 {
    // Creates a local copy of the default font to pass in
    let default_font = Font::default();
    // Creates a TextDimensions of the passed in box to test
    let box_dimensions = macroquad::text::TextDimensions {width: box_w, height: box_h, offset_y: y_offset};
    // Creates the value which is incremented to make the text size bigger
    let mut increment = 1;
    // Creates the TextDimensions which is compared against box_dimensions and has its text size incremented
    let mut test_dimensions = macroquad::text::measure_text(input_text, Some(default_font), increment, 1.0);
    // Loops while the dimensions of test_dimensions is smaller than the box's dimensions
    // Every loop, the dimensions of test_dimensions gets bigger until they are ~= the box's dimensions
    while test_dimensions.width <= box_dimensions.width && test_dimensions.height <= box_dimensions.height {
        // Makes the text size larger
        increment += 1;
        // Recreates test_dimensions with the larger text size, one larger so the text is always returned smaller than the box it is in
        test_dimensions = macroquad::text::measure_text(input_text, Some(default_font), increment + 1, 1.0);
    }
    return increment as f32;
}