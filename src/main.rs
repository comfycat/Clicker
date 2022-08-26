use std::time::{Instant, Duration};

use macroquad::prelude::*;

use upgrade::Upgrade;
use gamevalues::Gamevalues;
use alchemy::Alchemy;
use alchemyitems::Alchemyitems;
use pets::Pets;
use petitems::Petitems;

mod upgrade;
mod gamevalues;
mod alchemy;
mod alchemyitems;
mod pets;
mod petitems;

#[macroquad::main("Clicker Game")]
async fn main() {
    // Creates constants for counter display
    let (_counter_x, _counter_y, counter_w, counter_h) =
        (0.0, 0.0, screen_width() * 0.375, screen_height() * 0.125);
    // Creates constants for navigation buttons
    let (navigation_button_x, _navigation_button_y, navigation_button_w, navigation_button_h, navigation_button_text_color) =
        (screen_width() * 0.375, 0.0, screen_width() * 0.125, screen_height() * 0.125, PURPLE);
    
    // Initalizes the current_render enum for determining which screen is rendered 
    enum CurrentRender {
        Fishing,
        Upgrades,
        Alchemy,
        Stars,
        Pets
    }

    // Initalizes the fishing_type enum for determining what to do with the fishing output
    enum FishingType {
        Fish,
        Tomes,
        Recipes,
        Misc
    }

    let mut current_render: &CurrentRender = &CurrentRender::Fishing;
    let mut fishing_type: &FishingType = &FishingType::Fish;
    // Creates the text boxes for navigation buttons, and pairs them with their corresponding CurrentRender values
    let navigation_text = vec![("Fishing", CurrentRender::Fishing), ("Upgrades", CurrentRender::Upgrades), 
        ("Alchemy", CurrentRender::Alchemy), ("Stars", CurrentRender::Stars), ("Pets", CurrentRender::Pets)];
    // Creates the text boxes for fishing types, and pairs them with their corresponding FishingType values
    let fishing_type_text = vec! [("Fish", FishingType::Fish), ("Tomes", FishingType::Tomes), 
        ("Recipes", FishingType::Recipes), ("Misc", FishingType::Misc)];
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

    // Creates the vector containing all pet items
    let pet_items = vec![
        Petitems::new("Water Blob", false, BLUE, Box::new(|gamevalues: &mut Gamevalues| {
            if gamevalues.water + 1.0 <= gamevalues.water_capacity {
                gamevalues.water += 1.0;
            }
        })),
        Petitems::new("Sand Cat", false, BEIGE, Box::new(|_gamevalues: &mut Gamevalues| {

        }))
    ];

    // Initalizes the pets variable as a struct
    let mut gamepets = Pets::new(main_section_w, main_section_h, pet_items);

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
    let mut gamevalues = Gamevalues::new(0, 1, 1, 0, 0.0, 
        10.0, 0, 0);
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
            // Calls any pet benefits
            gamepets.pet_persecond(&mut gamevalues);
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
            CurrentRender::Fishing => {
                // Renders the FishingSelect area, which has buttons to change the FishingType
                draw_rectangle(right_main_section_x, top_main_section_y, main_section_w, main_section_h, LIGHTGRAY);
                // Adds the text for saying the current type of fishing, with a border
                // Takes up the top quarter of the section, and the full width
                draw_rectangle(right_main_section_x, top_main_section_y, main_section_w, main_section_h * 0.25, GRAY);
                // Writes the text saying the current type of fishing
                // Creates the variable for the text, and then sets it based off of the type of fishing
                let type_text: &str;
                match fishing_type {
                    FishingType::Fish => {
                        type_text = "Fish";
                    }
                    FishingType::Tomes => {
                        type_text = "Tomes";
                    }
                    FishingType::Recipes => {
                        type_text = "Recipes";
                    }
                    FishingType::Misc => {
                        type_text = "Misc";
                    }
                }
                // Creates a tuple
                let type_tuple = scale_text_in_box(main_section_w, main_section_h * 0.25, 0.0, type_text);
                // Renders the text
                draw_text(type_text, right_main_section_x, top_main_section_y + type_tuple.1, type_tuple.0, YELLOW);

                // Renders the Fishing Type selection buttons
                for (i, (text, fish_type)) in fishing_type_text.iter().enumerate() {
                    // Establishes the x and y locations for the button
                    let current_fishing_button_x = right_main_section_x + (main_section_w * 0.25) * i as f32;
                    let current_fishing_button_y = top_main_section_y + main_section_h * 0.25;
                    // Creates the background for the button
                    draw_rectangle(current_fishing_button_x, current_fishing_button_y, main_section_w * 0.25, 
                        main_section_h * 0.25, DARKGREEN);
                    // Creates the text inside of the background
                    let fishing_text_tuple = scale_text_in_box(main_section_w * 0.25,main_section_h * 0.25, 0.0, text);
                    draw_text(text, current_fishing_button_x, current_fishing_button_y + fishing_text_tuple.1, 
                        fishing_text_tuple.0, RED);
                    
                    if mouse_pressed && mouse_in_rectangle(current_fishing_button_x, current_fishing_button_y, 
                        main_section_w * 0.25, main_section_w * 0.25) {
                        fishing_type = fish_type;
                    }
                }
                // Creates the button the player presses to go fishing, which changes based off of the FishingType
                draw_circle(button_x, button_y, button_r, button_color);
                // If the player presses the main button, it gives them a point
                if mouse_pressed && mouse_in_circle(button_x, button_y, button_r) {
                    gamevalues.counter += gamevalues.get_clickpower();  
                }

                // TEST
                // CREATES A TEST BOX USING THE NEW METHOD
                draw_bordered_text_box(right_main_section_x * 0.5, bottom_main_section_y * 0.5, main_section_w, main_section_h, BLUE, YELLOW, PINK, "Hello World!");
                draw_bordered_text_box(100.0, 330.0, 300.0, 150.0, RED, BLACK, GOLD, "hi");
            }
            // Clicker and Upgrades State
            // Creates the area for upgrades
            CurrentRender::Upgrades => {
                // Renders the upgrade zone inside of which upgrades go
                draw_rectangle(right_main_section_x, top_main_section_y, main_section_w, upgrade_zone_h, upgrade_zone_color);

                // Renders the upgrade stats zone
                draw_rectangle(_left_main_section_x, bottom_main_section_y, main_section_w, main_section_h, RED);
                // Adds STATS: text at the top
                let stat_tuple = scale_text_in_box(main_section_w, main_section_h * 0.2, 0.0, "STATS: ");
                draw_text("STATS: ", _left_main_section_x, bottom_main_section_y + stat_tuple.1, stat_tuple.0, DARKGRAY);
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
            CurrentRender::Alchemy => {
                // Makes sure that the alchemy upgrade has been purchased
                if gamealchemy.unlocked {
                    // Renders the cauldron items
                    gamealchemy.render_alchemy_preview(_left_main_section_x, top_main_section_y, 
                        mouse_pressed, &mut gamevalues, &mut gamepets);
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
            CurrentRender::Stars => {}
            
            // Pets State
            CurrentRender::Pets => {
                // Renders the Pets Details window
                gamepets.render_pet_details(_left_main_section_x, bottom_main_section_y, &mut gamevalues);

                // Renders the Pets Inventory Window
                gamepets.render_pet_inventory(right_main_section_x, bottom_main_section_y, mouse_pressed, &mut gamevalues);
            }
        }

        // Renders the Navigation buttons at the top
        for (i, (text, navigation)) in navigation_text.iter().enumerate() {
            // Establishes the x location for the button
            let current_navigation_button_x = navigation_button_x + navigation_button_w * i as f32;
            // Creates the background for the button
            draw_rectangle(current_navigation_button_x, _navigation_button_y, navigation_button_w, 
                navigation_button_h, DARKGREEN);
            // Creates the text inside of the background
            let navigation_text_tuple = scale_text_in_box(navigation_button_w,navigation_button_h, 0.0, text);
            draw_text(text, current_navigation_button_x, _navigation_button_y + navigation_text_tuple.1, 
                navigation_text_tuple.0, navigation_button_text_color);
            
            if mouse_pressed && mouse_in_rectangle(current_navigation_button_x, _navigation_button_y, navigation_button_w, 
                navigation_button_h) {
                current_render = navigation;
            }
        }
 
        // Old code: draw_rectangle(upgrade_zone_x + upgrade_zone_w * 0.05, upgrade_zone_y + upgrade_zone_y * 0.05, upgrade_w, upgrade_h, upgrade_color);

        // Draws a box to improve the looks of the points display
        draw_rectangle(_counter_x, _counter_y, counter_w, counter_h, BLACK);
        // Displays the number of points that the player has
        let player_points = format!("Counter: {}", gamevalues.counter);
        let player_points_tuple = scale_text_in_box(counter_w, counter_h, 0.0, &player_points);
        
        draw_text(&player_points, _counter_x, _counter_y + player_points_tuple.1, 
            player_points_tuple.0, PINK);
        
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
// Returns font_size, and the height of that font size
pub fn scale_text_in_box(box_w: f32, box_h: f32, y_offset: f32, input_text: &str) -> (f32, f32) {
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
    // sets the y offset to be in the middle of the box, plus half of the text height so that inputs of it 
    let return_height = macroquad::text::measure_text(input_text, Some(default_font), increment, 1.0).height;
    // Returns the font size, and the y offset
    return (increment as f32, return_height);
}

// Draws a box with text centered inside of it and an outline
// box_x, box_y, box_w, box_h, box_color, border_color, text_color, and input_text
// input_text: text to be put inside of the box
// does not return a value
pub fn draw_bordered_text_box(box_x: f32, box_y: f32, box_w: f32, box_h: f32, box_color: Color, border_color: Color, text_color:Color, input_text: &str) {
    // Draws the outer text box
    draw_rectangle(box_x, box_y, box_w, box_h, box_color);

    // Draws the border on the box at 90% of the original margins, 
    let (border_w, border_h) = (box_w * 0.96, box_h * 0.96);
    draw_rectangle_lines(box_x + box_w * 0.02, box_y + box_h * 0.02, border_w, border_h, box_h * 0.02, border_color);

    // Determines the font size to use
    // The old scale_text_in_box method
    let default_font = Font::default();
    // Creates a TextDimensions of the passed in box to test
    let box_dimensions = macroquad::text::TextDimensions {width: border_w, height: border_h, offset_y: 0.0};
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
    // Gets the height and width values out of text_dimensions to center the text box
    let height_offset = macroquad::text::measure_text(input_text, Some(default_font), increment, 1.0).height;
    let width_offset = macroquad::text::measure_text(input_text, Some(default_font), increment, 1.0).width;
    // Draws the text inside of the box
    //
    draw_text(input_text, box_x + box_w * 0.03, box_y + height_offset * 0.5 + border_h * 0.5, increment as f32, text_color);

}

// Takes in text, and returns the height offset

/* let test_texture = Texture2D::from_file_with_format (
        include_bytes!(".\\banana.png"),
        ImageFormat::from_extension("png"),
        );
 */