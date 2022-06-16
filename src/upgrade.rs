use macroquad::prelude::*;

pub struct Upgrade {
    width: f32,
    height: f32,
    cost: i32,
    onetime: bool,
    owned: i32,
    text: String
}

impl Upgrade {
    pub fn new(width: f32, height: f32, cost: i32, owned: i32, onetime: bool, text: &str) -> Upgrade {
        Upgrade {
            width,
            height,
            cost,
            onetime,
            owned,
            text: text.to_owned()
        }
    }

    // Renders the upgrade at the given x and y values
    pub fn render(&self, render_x: f32, render_y: f32) {
        // Sets the color the upgrade renders as based on type
        let render_color = if self.owned == 0 {RED} 
            else if self.onetime == false {BLUE} else {GREEN};
        draw_rectangle(render_x, render_y, self.width, self.height, render_color);
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
    pub fn purchase(&mut self, counter: i32) -> i32 {
        // Onetime purchase is already owned
        if self.onetime && self.owned == 1 {
            return 0;
        // Onetime purchase is not owned and player has enough points to purchase
        // or non-onetime purchase is not owned and player does not have enough points to purchase
        } else if self.cost <= counter {
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




 