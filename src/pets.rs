use::macroquad::prelude::*;

use crate::{gamevalues::{Gamevalues, self}, petitems::Petitems, mouse_in_rectangle, scale_text_in_box};

pub struct Pets {
    main_section_w: f32,
    main_section_h: f32,
    pub pet_list: Vec<Petitems>
}

impl Pets {
    pub fn new (main_section_w: f32, main_section_h: f32, pet_list: Vec<Petitems>) -> Pets {
        Pets {
            main_section_w,
            main_section_h,
            pet_list
        }
    }

    // Renders the Pet Details Section
    pub fn render_pet_details(&self, _left_main_section_x: f32, bottom_main_section_y: f32, gamevalues: &mut Gamevalues) {
        // Renders the Pet Details zone
        draw_rectangle(_left_main_section_x, bottom_main_section_y, self.main_section_w, self.main_section_h, GRAY);
        
        // Display the name of the currently selected pet
        // The name box is the top half of the box, and the name gets 2/7 of the top of the box, so it is 1/7 of the full height
        // It is also 3/5 of the full width
        let pet_details_render_tuple = scale_text_in_box(self.main_section_w * 0.6, self.main_section_h / 7.0, 0.0, 
            &self.pet_list[gamevalues.pets_selected].name);
        draw_text(&self.pet_list[gamevalues.pets_selected].name, _left_main_section_x + 5.0, bottom_main_section_y + pet_details_render_tuple.1, 
            pet_details_render_tuple.0, PINK);
        
        // Displays if the pet is owned
        // X value is the same as main box
        // Y value is main box + 1/2 of the height as it starts after the description area which takes up half
        // Width is 3/8 of main section
        // Height is 3/14 of main section
        let pet_owned_tuple = scale_text_in_box(self.main_section_w * 0.375, 
            self.main_section_h * (3.0 / 14.0), 0.0, &self.pet_list[gamevalues.pets_selected].name);
        draw_text(&format!("Owned: ({})", self.pet_list[gamevalues.pets_selected].owned), _left_main_section_x, 
            bottom_main_section_y + self.main_section_h * 0.5 + pet_owned_tuple.1, pet_owned_tuple.0, PINK);
        
    }

    // Renders the Pet Inventory Section
    pub fn render_pet_inventory(&mut self, right_main_section_x: f32, bottom_main_section_y: f32, mouse_pressed: bool, 
        gamevalues: &mut Gamevalues) {
        // Renders the backdrop for the pet inventory zone
        draw_rectangle(right_main_section_x, bottom_main_section_y, self.main_section_w, self.main_section_h, DARKGRAY);

        // Renders each item within the Pet Inventory zone with the same ratios as the Alchemy Items zone bc im lazy
        // Relative ratios for items within the zone
        // 3% 14% 2% 14% 2% 14% 2% 14% 2% 14% 2% 14% 3%
        // x values to be passed into the rendering function: 3% 19% 35% 51% 67% 83%
        for (i, item) in self.pet_list.iter_mut().enumerate() {
            // Factors out each upgrade's x, y, width, and height for use in the mouse_in_rectangle function
            // items_x starts at 3% of the with, and then for every iteration adds 16% more as that is the width of an item plus padding plus a 2% space
            // It also has a potential offset of right_main_section_x which is currently set to 0 in the main method, but can be changed in case i want to move it around later
            // Once a new row is made, resets i to 0
            let items_x = right_main_section_x + (self.main_section_w * 0.03) + (self.main_section_w * 0.16 * (i % 6) as f32);
            
            // Padding is very hypothetical as items are rendered as square
            // Relative ratios for items within the zone
            // 15% 28% 14% 28% 15%
            // y values to be passed into the rendering function: 15% 53%
            // Initalizes the items_y variable now to be accessed outside of the if statement
            let mut items_y = 0.0;
            // Checks if the values would be less than 6 to give it the upper row's height
            if i <= 5 {
                // Adding 15% of the item zone as padding
                items_y = bottom_main_section_y + self.main_section_h * 0.15;
            // Checks if the values would be less than 12 instead to give it the lower row's height
            } else if i <= 11 {
                // Adding 53% of the item zone as padding
                // THIS VALUE CAN BE CHANGED IF MORE ITEMS ARE IN THE INVENTORY
                items_y = bottom_main_section_y + self.main_section_h * 0.53; 
            }
            // Renders the upgrade with the values in the current loop
            // The side value of 14% of self.width is because the item accounts for 10% of the width, with 2% padding. 
            // The 2% padding is accounted for in the render_item function itself
            let side = self.main_section_w * 0.14;
            item.render_item(items_x, items_y, side);

            // Checks if the item got clicked on, and if so sets it as the selected value
            if mouse_pressed && mouse_in_rectangle(items_x, items_y, side, side) {
                gamevalues.pets_selected = i;
            }
        }
    }

    pub fn pet_persecond(&mut self, gamevalues: &mut Gamevalues) {
        for (i, item) in self.pet_list.iter_mut().enumerate() {
            item.use_pet(gamevalues);
        }
    }
}