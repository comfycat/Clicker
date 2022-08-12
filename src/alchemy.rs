use macroquad::prelude::*;
use crate::{alchemyitems::Alchemyitems, mouse_in_rectangle, gamevalues::Gamevalues, scale_text_in_box, pets::Pets};

pub struct Alchemy {
    main_section_w: f32,
    main_section_h: f32,
    pub unlocked: bool,
    pub items: Vec<Alchemyitems>,
    cauldron_items: Vec<usize>
}

impl Alchemy {
    pub fn new(main_section_w: f32, main_section_h: f32, unlocked: bool, items: Vec<Alchemyitems>, cauldron_items: Vec<usize>) -> Alchemy {
        Alchemy {
            main_section_w,
            main_section_h,
            unlocked,
            items,
            cauldron_items
        }
    }

    // Renders the Alchemy Preview section
    pub fn render_alchemy_preview(&mut self, _left_main_section_x: f32, top_main_section_y: f32, mouse_pressed: bool,
        gamevalues: &mut Gamevalues, pets: &mut Pets) {
        // Renders the backdrop for the Alchemy Preview zone
        draw_rectangle(_left_main_section_x, top_main_section_y, self.main_section_w, self.main_section_h, LIGHTGRAY);
        
        // Renders the items which have currently been added to the cauldron
        // Sorts the vector
        self.cauldron_items.sort();
        // Pulls out the size of the cauldron items
        let cauldron_items_size: f32 = self.cauldron_items.len() as f32;
        // Uses the same side input value as the other rendering for alchemyitems
        let side = self.main_section_w * 0.14;
        // Displays the items in the vector
        // X value is based off of the number of items which are in the Vector
        // Y value is main box + 2/7 of main section as I want it in the middle between the top and the buttons
        for (i, item) in self.cauldron_items.iter_mut().enumerate() {
            // X value is determined by the number of items, and centers the items using half of the side length
            // 10% buffering on each side, and subtracts half of side to center the items
            self.items[*item].render_item(_left_main_section_x + self.main_section_w * 0.1 - side * 0.5 +
                // Divides the remaining 80% after the 10% buffer on each side up for every item
                (2.0 * (i as f32) + 1.0) * ((self.main_section_w * 0.4) / cauldron_items_size), 
            top_main_section_y + self.main_section_h * (2.0 / 7.0), side);
        }
        // Renders the empty selection button
        // X value is main box + 3/4 of the main section as it touches the right
        // Y value is main box + 5/7 of the height as it touches the bottom
        // Width is 1/4 of the main section
        // Height is 2/7 of the main section
        draw_rectangle(_left_main_section_x + self.main_section_w * 0.75, top_main_section_y + self.main_section_h * (5.0 / 7.0),
            self.main_section_w * 0.25, self.main_section_h * (2.0 / 7.0), DARKBLUE);
        // Renders the Empty text
        let empty_tuple = scale_text_in_box(self.main_section_w * 0.25, self.main_section_h * (2.0 / 7.0), 0.0, "Empty");
        draw_text("Empty", _left_main_section_x + self.main_section_w * 0.75, top_main_section_y + self.main_section_h * (5.0 / 7.0) + empty_tuple.1, 
            empty_tuple.0, PINK);
        
        // Checks if the button got clicked on, and if so, empties the cauldron, returning the items
        if mouse_pressed && mouse_in_rectangle(_left_main_section_x + self.main_section_w * 0.75, top_main_section_y + self.main_section_h * (5.0 / 7.0),
        self.main_section_w * 0.25, self.main_section_h * (2.0 / 7.0)) {
            // Refunds the player's items
            self.refund();
        }

        // Renders the craft button
        // X value is the same as main box
        // Y value is main box + 5/7 of the height as it touches the bottom
        // Width is 1/4 of the main section
        // Height is 2/7 of the main section
        draw_rectangle(_left_main_section_x, top_main_section_y + self.main_section_h * (5.0 / 7.0),
             self.main_section_w * 0.25, self.main_section_h * (2.0 / 7.0), DARKBLUE);
        // Renders the Craft text
        let craft_tuple = scale_text_in_box(self.main_section_w * 0.25, self.main_section_h * (2.0 / 7.0), 
        0.0, "Craft");
        draw_text("Craft", _left_main_section_x, top_main_section_y + self.main_section_h * (5.0 / 7.0) + craft_tuple.1, 
            craft_tuple.0, PINK);

        // Checks if the button got clicked on, and attempts to run a craft if so
        if mouse_pressed && mouse_in_rectangle(_left_main_section_x, top_main_section_y + self.main_section_h * (5.0 / 7.0),
        self.main_section_w * 0.25, self.main_section_h * (2.0 / 7.0)) {
            self.craft(gamevalues, pets);
        }
    }

    // Renders the water section
    pub fn render_water(&self, _left_main_section_x: f32, bottom_main_section_y: f32, gamevalues: &mut Gamevalues) {
        // Renders the water zone
        draw_rectangle(_left_main_section_x, bottom_main_section_y, self.main_section_w, self.main_section_h, GRAY);
        // Renders the greyed out bar for the maximum capacity
        draw_rectangle(_left_main_section_x + self.main_section_w * 0.05, bottom_main_section_y + self.main_section_h * 0.1, 
            self.main_section_w * 0.9, self.main_section_h * 0.1, DARKBLUE);
        // Renders the bar relative to the amount of water owned
        let water_percent: f32 = gamevalues.water / gamevalues.water_capacity;
        draw_rectangle(_left_main_section_x + self.main_section_w * 0.05, bottom_main_section_y + self.main_section_h * 0.1,
            self.main_section_w * 0.9 * water_percent, self.main_section_h * 0.1, BLUE);
    }

    // Renders the box with the item info, taking into account the currently selected item
    pub fn render_item_info(&mut self, right_main_section_x: f32, top_main_section_y: f32, mouse_pressed: bool,
        gamevalues: &mut Gamevalues) {
        // Renders the backdrop for the alchemy item info zone
        draw_rectangle(right_main_section_x, top_main_section_y, self.main_section_w, self.main_section_h, DARKGRAY);
        // Display the name of the currently selected alchemy item
        // The name box is the top half of the box, and the name gets 2/7 of the top of the box, so it is 1/7 of the full height
        // It is also 3/5 of the full width
        let item_render_tuple = scale_text_in_box(self.main_section_w * 0.6, self.main_section_h / 7.0, 0.0, 
            &self.items[gamevalues.alchemy_selected].name);
        draw_text(&self.items[gamevalues.alchemy_selected].name, right_main_section_x + 5.0, top_main_section_y + item_render_tuple.1, 
            item_render_tuple.0, PINK);
        // Renders the amount of the item owned
        // X value is the same as main box
        // Y value is main box + 1/2 of the height as it starts after the description area which takes up half
        // Width is 3/8 of main section
        // Height is 3/14 of main section
        let item_owned_tuple = scale_text_in_box(self.main_section_w * 0.375, 
            self.main_section_h * (3.0 / 14.0), 0.0, &self.items[gamevalues.alchemy_selected].name);
        draw_text(&format!("Owned: ({})", self.items[gamevalues.alchemy_selected].owned), right_main_section_x, 
            top_main_section_y + self.main_section_h * 0.5 + item_owned_tuple.1, item_owned_tuple.0, PINK);
        
        // Renders the cost of the selected item
        // x value is the same as main box
        // Y value is main box + 1/2 of the height, plus 1/8 of the height so 5/8 of the height
        let item_cost_tuple = scale_text_in_box(self.main_section_w * 0.375, 
            self.main_section_h * (3.0 / 14.0), 0.0, &self.items[gamevalues.alchemy_selected].name);
        draw_text(&format!("Cost: {}", self.items[gamevalues.alchemy_selected].owned), right_main_section_x, 
            top_main_section_y + self.main_section_h * 0.5 + item_owned_tuple.1 + item_cost_tuple.1, item_owned_tuple.0, PINK);
        // Renders the Purchase item button
        // X value is the same as main box
        // Y value is main box + 5/7 of the height as it touches the bottom
        // Width is 1/4 of the main section
        // Height is 2/7 of the main section
        draw_rectangle(right_main_section_x, top_main_section_y + self.main_section_h * (5.0 / 7.0),
             self.main_section_w * 0.25, self.main_section_h * (2.0 / 7.0), DARKGREEN);
        // Renders the Purchase text
        let purchase_tuple = scale_text_in_box(self.main_section_w * 0.25, self.main_section_h * (2.0 / 7.0), 
        0.0, "Purchase");
        draw_text("Purchase", right_main_section_x, top_main_section_y + self.main_section_h * (5.0 / 7.0) + purchase_tuple.1, 
            purchase_tuple.0, PINK);
        
        // Checks if purchase button got clicked on, and if so attempts to purchase it
        if mouse_pressed && mouse_in_rectangle(right_main_section_x, top_main_section_y + self.main_section_h * (5.0 / 7.0),
        self.main_section_w * 0.25, self.main_section_h * (2.0 / 7.0)) {
            // If player has enough points to purchase, increased owned and deduct points
            if self.items[gamevalues.alchemy_selected].cost <= gamevalues.counter {
                    // Purchases the item
                    self.items[gamevalues.alchemy_selected].owned += 1;
                    gamevalues.counter -= self.items[gamevalues.alchemy_selected].cost;
            }        
        }

        // Renders the Add Item to Cauldron button
        // X value is main box + 3/4 of the width as it touches the right side
        // Y value is main box + 5/7 of the height as it touches the bottom
        // Width is 1/4 of the main section
        // Height is 2/7 of the main section
        draw_rectangle(right_main_section_x + self.main_section_w * 0.75, top_main_section_y + self.main_section_h * (5.0 / 7.0),
             self.main_section_w * 0.25, self.main_section_h * (2.0 / 7.0), DARKGREEN);
        // Renders the Add Item to Cauldron text
        let add_tuple = scale_text_in_box(self.main_section_w * 0.25, self.main_section_h * (2.0 / 7.0), 
        0.0, "Add");
        draw_text("Add", right_main_section_x + self.main_section_w * 0.75, top_main_section_y + self.main_section_h * (5.0 / 7.0) + add_tuple.1, 
            add_tuple.0, PINK);
        
        // Checks if the item got clicked on, and if any are owned, adds them to the cauldron
        if mouse_pressed && mouse_in_rectangle(right_main_section_x + self.main_section_w * 0.75, top_main_section_y + self.main_section_h * (5.0 / 7.0),
        self.main_section_w * 0.25, self.main_section_h * (2.0 / 7.0)) {
            // Makes sure one of the items is owned
            if self.items[gamevalues.alchemy_selected].owned >= 1 {
                 // If player has too many items in cauldron, does not add
                if self.cauldron_items.len() < 5 {
                    // Adds an item to the list of items in the cauldron
                    self.cauldron_items.push(gamevalues.alchemy_selected);
                    // Removes one of the items from the inventory
                    self.items[gamevalues.alchemy_selected].owned -= 1;
                }
            }
        }
    }


    // Renders the inventory space, and the owned items inside
    // Currently can only handle up to 12 items, will break otherwise
    pub fn render_item_inventory(&mut self, right_main_section_x: f32, bottom_main_section_y: f32, mouse_pressed: bool, 
        gamevalues: &mut Gamevalues) {
        // Renders the backdrop for the alchemy item inventory zone
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

            // Checks if the item got clicked on, and if so sets it as the selected value
            if mouse_pressed && mouse_in_rectangle(items_x, items_y, side, side) {
                gamevalues.alchemy_selected = i;
            }
        }
    }

    // Attempts to use the player's cauldron items to complete a craft
    pub fn craft(&mut self, gamevalues: &mut Gamevalues, pets: &mut Pets) {
        // Fill water craft: only one water inside
        if self.cauldron_items == vec![0] {
            // Makes sure there is room in the water gauge
            if gamevalues.water + 1.0 <= gamevalues.water_capacity {
                // If there is, fills the gauge
                gamevalues.water += 1.0;
                // Empties the cauldron items vector
                self.cauldron_items.clear();
            } else {
                // Otherwise, returns the bottle to the player's inventory
                self.items[0].owned += 1;
                self.cauldron_items.clear();
            }
        // Current Create Water Blob craft
        } else if self.cauldron_items == vec![0, 5] {
            // Makes sure that the pet is not already owned
            if !pets.pet_list[0].owned {
                // Trying to create another constraint, needs 2 water in cauldron to craft as well
                if gamevalues.water >= 2.0 {
                    // Creates the water blob
                    pets.pet_list[0].owned = true;
                    // Reduces the water by the amount
                    gamevalues.water -= 2.0;
                    // Empties the cauldron items vector
                    self.cauldron_items.clear();
                } else {
                    // Otherwise, returns the items to the player's inventory
                    self.items[0].owned += 1;
                    self.items[5].owned += 1;
                    self.cauldron_items.clear();
                }
            } else {
                // Otherwise, returns the items to the player's inventory
                self.refund();
            }
        } else {
            // If the craft was not valid, empties the cauldron back into the player's inventory
            self.refund();
        }
    }

    // Empties the current crafting queue, refunding all items inside back to the player's alchemy inventory
    pub fn refund(&mut self) {
        // Iterates through the cauldron items vec, taking each item and returning it to the inventory.
        for (_i, value) in self.cauldron_items.iter_mut().enumerate() {
            self.items[*value].owned += 1;
        }
        // Clears the inventory
        self.cauldron_items.clear();
    }
}