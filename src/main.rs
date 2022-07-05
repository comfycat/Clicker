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
    // Creates constants for counter display
    let (_counter_x, _counter_y, counter_w, counter_h) =
        (0.0, 0.0, screen_width() * 0.375, screen_height() * 0.125);
    // Creates constants for navigation buttons
    let (navigation_button_x, _navigation_button_y, navigation_button_w, navigation_button_h, navigation_button_text_color) =
        (screen_width() * 0.375, 0.0, screen_width() * 0.125, screen_height() * 0.125, PURPLE);
    // Creates the text boxes for navigation buttons
    let mut navigation_text = vec!["Fishing", "Upgrades", "Alchemy", "Stars", "Pets"];
    // Creates constants for the top and bottom halves of main sections
    let (_left_main_section_x, right_main_section_x, top_main_section_y, bottom_main_section_y, main_section_w, main_section_h) =
        (0.0, screen_width() * 0.5, screen_height() * 0.125, screen_height() * 0.5625, screen_width() * 0.5, screen_height() * 0.4375);
    // Creates a separate constant for the upgrade zone height as currently it is twice as large as others, also does color
    let (upgrade_zone_h, upgrade_zone_color) = (screen_height() * 0.875, GRAY);
    // Creates constants for clicker button location / size
    let (button_x, button_y, button_r, button_color) = 
        (screen_width() * 0.25, screen_height() * 0.3125, screen_height() * 0.125, BLUE);
    // Creates constants for inner upgrade size
    let (_upgrade_padding_x, upgrade_padding_y, upgrade_w, upgrade_h) = 
        (right_main_section_x + main_section_w * 0.05, upgrade_zone_h * 0.06, 
        main_section_w * 0.9, upgrade_zone_h * 0.1);

    // Initalizes the current_render variable for determining which screen is rendered
    let mut current_render = "Fishing";

    // Creates the vector containing all alchemy items
    let alchemy_items = vec![
        Alchemyitems::new("Water Bottle", 1, 0, BLUE, Box::new(|gamevalues: &mut Gamevalues| {
            if gamevalues.water + 1.0 <= gamevalues.water_capacity {
                gamevalues.water += 1.0;
            } 
        })),
        Alchemyitems::new("Bag of Sand", 1, 0, BEIGE, Box::new(|_gamevalues: &mut Gamevalues| {

        })),
        Alchemyitems::new("Bottle of Fire", 1, 0, RED, Box::new(|_gamevalues: &mut Gamevalues| {
            
        })),
        Alchemyitems::new("Dandelion Petals", 1, 0, YELLOW, Box::new(|_gamevalues: &mut Gamevalues| {
            
        })),
        Alchemyitems::new("Coffee Grounds", 1, 0, BROWN, Box::new(|_gamevalues: &mut Gamevalues| {
            
        })),
        Alchemyitems::new("Piece of Soul", 5, 0, PURPLE, Box::new(|_gamevalues: &mut Gamevalues| {
            
        })),
        Alchemyitems::new("Concrete Powder", 1, 0, GRAY, Box::new(|_gamevalues: &mut Gamevalues| {
            
        })),
        Alchemyitems::new("Brown Boot", 1, 0, BROWN, Box::new(|_gamevalues: &mut Gamevalues| {
            
        }))
    ];
    // Initalizes the Alchemy variable as a struct
    // Passes the width and height of a main section for use in calculations
    let mut gamealchemy = Alchemy::new(main_section_w, main_section_h, false, alchemy_items, vec![]);
    // Initalizes the gamevalues variable as a struct for maximizing player value from upgrades
    // TESTING Initalizing the persecond variable to determine points gained per second in the gamevalues struct
    let mut gamevalues = Gamevalues::new(0, 1, 1, 0, 0.0, 10.0, 0);
    // Creates the reference for counting seconds with
    let mut game_timer = Instant::now();
    
    // Creates a vector containing all of the upgrades
    let mut upgrades = vec![
        // pub fn new(width: f32, height: f32, cost: i32, owned: i32, onetime: bool, text: &str) -> Upgrade
        Upgrade::new(upgrade_w, upgrade_h, 5, 0, true, "Increased Click Power", Box::new(|gamevalues: &mut Gamevalues, _gamealchemy: &mut Alchemy| {
            gamevalues.clickpow_add += 1;
        })),
        Upgrade::new(upgrade_w, upgrade_h, 30, 0, true, "Double Click Power", Box::new(|gamevalues: &mut Gamevalues, _gamealchemy: &mut Alchemy| {
            gamevalues.clickpow_mult *= 2;
        })),
        Upgrade::new(upgrade_w, upgrade_h, 20, 0, false, "Points Per Second", Box::new(|gamevalues: &mut Gamevalues, _gamealchemy: &mut Alchemy| {
            gamevalues.persecond += 1;
        })),
        Upgrade::new(upgrade_w, upgrade_h, 1, 0, true, "Alchemy", Box::new(|_gamevalues: &mut Gamevalues, gamealchemy: &mut Alchemy| {
            gamealchemy.unlocked = true;
        })),
        /* 
        Upgrade::new(upgrade_w, upgrade_h, 1, 0, false, "Water Bottle", Box::new(|_gamevalues: &mut Gamevalues, gamealchemy: &mut Alchemy| {
            gamealchemy.items[0].owned += 1;
        })),
        Upgrade::new(upgrade_w, upgrade_h, 1, 0, false, "Bag of Sand", Box::new(|_gamevalues: &mut Gamevalues, gamealchemy: &mut Alchemy| {
            gamealchemy.items[1].owned += 1;
        })),
        Upgrade::new(upgrade_w, upgrade_h, 1, 0, false, "Bottle of Fire", Box::new(|_gamevalues: &mut Gamevalues, gamealchemy: &mut Alchemy| {
            gamealchemy.items[2].owned += 1;
        })),
        */
        // ...
    ];
    
    // Creates the background image
    /*
    let _background = Texture2D::from_file_with_format (
        include_bytes!(".\\background.png"),
        ImageFormat::from_extension("png"),
        );
    // Sets the background to be an image
    draw_texture(background, 0.0, 0.0, WHITE);
    */

    // The main loop which creates the game
    loop {
        // Colors the background
        clear_background(PINK);

        // Checks if the mouse was pressed on this frame
        let mouse_pressed = is_mouse_button_pressed(MouseButton::Left);

        // Checks to see if a second has passed for timing, if one has, resets the time since the last second was counted
        // For adding income per second upgrades
        if game_timer.elapsed() > Duration::from_secs(1) {
            // Adds the income per second in the Gamevalues struct
            gamevalues.counter += gamevalues.persecond;
            game_timer = Instant::now();
        }

        //
        //
        // RENDERING
        //
        //
        // Checks the current rendering screen, and renders the proper items
        match current_render {
            // Clicker State
            "Fishing" => {
                // Creates the button the player presses to get points
                // This may be moved to another tab in the future
                draw_circle(button_x, button_y, button_r, button_color);
                // If the player presses the main button, it gives them a point
                if mouse_pressed && mouse_in_circle(button_x, button_y, button_r) {
                    gamevalues.counter += gamevalues.get_clickpower();  
                }
            }
            // Clicker and Upgrades State
            // Creates the area for upgrades
            "Upgrades" => {
                // Renders the upgrade zone inside of which upgrades go
                draw_rectangle(right_main_section_x, top_main_section_y, main_section_w, upgrade_zone_h, upgrade_zone_color);

                // Creates the zone to show current Per Second upgrades, to be added in the future
                draw_rectangle(_left_main_section_x, bottom_main_section_y, main_section_w, main_section_h, BLUE);
                // Renders the upgrades, and checks for purchases
                for (i, upgrade) in upgrades.iter_mut().enumerate() {
                    // Factors out each upgrade's x, y, width, and height for use in the mouse_in_rectangle function
                    let upgrade_x = right_main_section_x + main_section_w * 0.05;
                    let upgrade_y = top_main_section_y + upgrade_padding_y * (2 * i + 1) as f32;
                    // Renders the upgrades
                    upgrade.render(upgrade_x, upgrade_y);
                    // If the player clicks on an upgrade, it tries to purchase that upgrade
                    // Deducts the number of points spent, which is returned by the purchase function
                    if mouse_pressed && mouse_in_rectangle(upgrade_x, upgrade_y, upgrade_w, upgrade_h) {
                        let deduction = upgrade.purchase(&mut gamevalues, &mut gamealchemy);
                        gamevalues.counter -= deduction;
                    }
                }
            }

            // Alchemy State
            "Alchemy" => {
                // Makes sure that the alchemy upgrade has been purchased
                if gamealchemy.unlocked {
                    // Renders the cauldron items
                    gamealchemy.render_alchemy_preview(_left_main_section_x, top_main_section_y, 
                        mouse_pressed, &mut gamevalues);
                    // Renders the water section
                    gamealchemy.render_water(_left_main_section_x, bottom_main_section_y, &mut gamevalues);
                    // Renders the alchemy item descrption box, passing mouse_pressed to buy / use items
                    gamealchemy.render_item_info(right_main_section_x, top_main_section_y, mouse_pressed, &mut gamevalues);
                    // Renders the alchemy item inventory, passing mouse_pressed to see if it got used
                    // mouse_pressed will be to select a different item
                    // Also passes the upgrades vector to reduce the amount owned if the upgrade got purchased
                    gamealchemy.render_item_inventory(right_main_section_x, bottom_main_section_y,
                        mouse_pressed, &mut gamevalues);
                }
            }
            "Stars" => {}
            "Pets" => {}
            _ => {}
        }

        // Renders the Navigation buttons at the top
        for (i, text) in navigation_text.iter_mut().enumerate() {
            // Establishes the x location for the button
            let current_navigation_button_x = navigation_button_x + navigation_button_w * i as f32;
            // Creates the background for the button
            draw_rectangle(current_navigation_button_x, _navigation_button_y, navigation_button_w, 
                navigation_button_h, DARKGREEN);
            // Creates the text inside of the background
            draw_text(text, current_navigation_button_x, _navigation_button_y + navigation_button_h * 0.5, 
                scale_text_in_box(navigation_button_w,navigation_button_h, 0.0, text), navigation_button_text_color);
            
            if mouse_pressed && mouse_in_rectangle(current_navigation_button_x, _navigation_button_y, navigation_button_w, 
                navigation_button_h) {
                current_render = text;
            }
        }
 
        // Old code: draw_rectangle(upgrade_zone_x + upgrade_zone_w * 0.05, upgrade_zone_y + upgrade_zone_y * 0.05, upgrade_w, upgrade_h, upgrade_color);

        // Draws a box to improve the looks of the points display
        draw_rectangle(_counter_x, _counter_y, counter_w, counter_h, BLACK);
        // Displays the number of points that the player has
        let player_points = format!("Counter: {}", gamevalues.counter);
        draw_text(&player_points, _counter_x, _counter_y + counter_h * 0.75, 
            scale_text_in_box(counter_w, counter_h, 0.0, &player_points), PINK);
        
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


/* let test_texture = Texture2D::from_file_with_format (
        include_bytes!(".\\banana.png"),
        ImageFormat::from_extension("png"),
        );
 */