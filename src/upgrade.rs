use macroquad::prelude::*;

use crate::clickpower::Clickpower;

pub struct Upgrade {
    width: f32,
    height: f32,
    cost: i32,
    owned: i32,
    onetime: bool,
    text: String,
    func: Box<dyn Fn(&mut Clickpower)>
}

impl Upgrade {
    pub fn new(width: f32, height: f32, cost: i32, owned: i32, onetime: bool, text: &str, func: Box<dyn Fn(&mut Clickpower)>) -> Upgrade {
        Upgrade {
            width,
            height,
            cost,
            owned,
            onetime,
            text: text.to_owned(),
            func
        }
    }

    // Renders the upgrade at the given x and y values
    pub fn render(&self, render_x: f32, render_y: f32) {
        // Sets the color the upgrade renders as based on type
        let render_color = if self.owned == 0 {RED} 
            else if self.onetime == false {BLUE} else {GREEN};
        draw_rectangle(render_x, render_y, self.width, self.height, render_color);
        // If upgrade can be purchased multiple times, displays the number owned
        if !self.onetime {
            let output_text: String = format!("{}({})", &self.text, self.owned);
            draw_text(&output_text, render_x + (self.width * 0.1), render_y + 30.0, 25.0, DARKGRAY);
        } else {
            draw_text(&self.text, render_x + (self.width * 0.1), render_y + 30.0, 30.0, DARKGRAY);
        }
    }

    /* Idea for a variable font size
    // Returns the maximum font size which still fits in the upgrade box
    fn render_text_size(input_text: &str) -> f32 {
        
        return -1.0;
    }
    */

    // Attempts to purchase the upgrade
    // - Respects onetime property
    // - Verifies player has enough points to afford
    pub fn purchase(&mut self, counter: i32, clickpower: &mut Clickpower) -> i32 {
        // Onetime purchase is already owned
        if self.onetime && self.owned == 1 {
            return 0;
        // Onetime purchase is not owned and player has enough points to purchase
        // or non-onetime purchase is not owned and player does not have enough points to purchase
        } else if self.cost <= counter {
            // Applies the purchase
            (self.func)(clickpower);
            self.owned += 1;
            return self.cost;
        } else {
            return 0;
        }
    }
    /*
    // Does not mutate object
    pub fn get_cost(&self) -> i32 {
        return self.cost;
    }

    // Does mutate object, so `mut self`
    pub fn set_cost(&mut self, new_cost: i32) {
        self.cost = new_cost;
    }

    pub fn method(&self) {
        let the_cost = self.cost;
    }
    */
}




 