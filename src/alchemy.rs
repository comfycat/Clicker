use macroquad::prelude::*;
use crate::{alchemyitems::Alchemyitems, mouse_in_rectangle, gamevalues::Gamevalues, upgrade::Upgrade};

pub struct Alchemy {
    main_section_w: f32,
    main_section_h: f32,
    pub unlocked: bool,
    pub items: Vec<Alchemyitems>
}

impl Alchemy {
    pub fn new(main_section_w: f32, main_section_h: f32, unlocked: bool, items: Vec<Alchemyitems>) -> Alchemy {
        Alchemy {
            main_section_w,
            main_section_h,
            unlocked,
            items
        }
    }

    // Renders the water bar
    pub fn render_water(&self, left_main_section_x: f32, bottom_main_section_y: f32, gamevalues: &mut Gamevalues) {
        // Renders the greyed out bar for the maximum capacity
        draw_rectangle(left_main_section_x + self.main_section_w * 0.05, bottom_main_section_y + self.main_section_w * 0.1, 
            self.main_section_w * 0.9, self.main_section_h * 0.1, DARKBLUE);
        // Renders the bar relative to the amount of water owned
        let water_percent: f32 = gamevalues.water / gamevalues.water_capacity;
        draw_rectangle(left_main_section_x + self.main_section_w * 0.05, bottom_main_section_y + self.main_section_h * 0.1,
            self.main_section_w * 0.9 * water_percent, self.main_section_h * 0.1, BLUE);
    }

    // Renders the inventory space, and the owned items inside
    // Currently can only handle up to 12 items, will break otherwise
    pub fn render_items(&mut self, right_main_section_x: f32, bottom_main_section_y: f32, mouse_pressed: bool, 
        gamevalues: &mut Gamevalues, upgrades: &mut Vec<Upgrade>) {
        
        // Renders the backdrop for the alchemy items zone
        draw_rectangle(right_main_section_x, bottom_main_section_y, self.main_section_w, self.main_section_h, DARKPURPLE);

        // Renders each item within the alchemy items zone
        // Relative ratios for items within the zone
        // 3% 14% 2% 14% 2% 14% 2% 14% 2% 14% 2% 14% 3%
        // x values to be passed into the rendering function: 3% 19% 35% 51% 67% 83%
        for (i, item) in self.items.iter_mut().enumerate() {
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

            // Checks if the item got clicked on, and if so runs its value
            if mouse_pressed && mouse_in_rectangle(items_x, items_y, side, side) {
                item.use_item(gamevalues, upgrades);
            }
        }
    }
}