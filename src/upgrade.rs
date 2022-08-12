use macroquad::prelude::*;

use crate::{gamevalues::Gamevalues, scale_text_in_box, alchemy::Alchemy};

pub struct Upgrade {
    width: f32,
    height: f32,
    cost: i32,
    pub owned: i32,
    onetime: bool,
    pub text: String,
    func: Box<dyn Fn(&mut Gamevalues, &mut Alchemy)>
}

impl Upgrade {
    pub fn new(width: f32, height: f32, cost: i32, owned: i32, onetime: bool, text: &str, func: Box<dyn Fn(&mut Gamevalues, &mut Alchemy)>) -> Upgrade {
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
        let upgrade_tuple = scale_text_in_box(self.width, self.height, 0.0, &self.text);
        // Sets the color the upgrade renders as based on type
        let render_color = if self.owned == 0 {RED} 
            else if self.onetime == false {BLUE} else {GREEN};
        draw_rectangle(render_x, render_y, self.width, self.height, render_color);
        // If upgrade can be purchased multiple times, displays the number owned
        if !self.onetime {
            let output_text: String = format!("{}({})", &self.text, self.owned);
            let onetime_tuple = scale_text_in_box(self.width, self.height, 0.0, &output_text);
            draw_text(&output_text, render_x, render_y + onetime_tuple.1, onetime_tuple.0, DARKGRAY);
        } else {
            draw_text(&self.text, render_x, render_y + upgrade_tuple.1, upgrade_tuple.0, DARKGRAY);
        }
    }

    // Attempts to purchase the upgrade
    // - Respects onetime property
    // - Verifies player has enough points to afford
    pub fn purchase(&mut self, gamevalues: &mut Gamevalues, gamealchemy: &mut Alchemy) -> i32 {
        // Onetime purchase is already owned
        if self.onetime && self.owned == 1 {
            return 0;
        // Onetime purchase is not owned and player has enough points to purchase
        // or non-onetime purchase is not owned and player does not have enough points to purchase
        } else if self.cost <= gamevalues.counter {
            // Applies the purchase
            (self.func)(gamevalues, gamealchemy);
            self.owned += 1;
            return self.cost;
        } else {
            return 0;
        }
    }
}




 