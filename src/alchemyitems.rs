use::macroquad::prelude::*;

use crate::{gamevalues::Gamevalues, upgrade::Upgrade};

pub struct Alchemyitems {
    pub name: String,
    pub owned: i32,
    color: Color,
    func: Box<dyn Fn(&mut Gamevalues)>
}

impl Alchemyitems {
    pub fn new (name: &str, owned: i32, color: Color, func: Box<dyn Fn(&mut Gamevalues)>) -> Alchemyitems {
        Alchemyitems {
            name: name.to_owned(),
            owned,
            color,
            func
        }
    }

    // Renders the item in a box, with some padding
    pub fn render_item (&self, render_x: f32, render_y: f32, side: f32) {
        // Creates a constant for the ratio between the padding and the inner box
        let x_ratio: f32 = 10.0 / 14.0;
        let padding_ratio: f32 = side * 1.0 / 7.0;
        // Renders the item's padding
        draw_rectangle(render_x, render_y, side, side, DARKBROWN);

        // Draws an outline on the padding's borders to separate the items
        // Top line
        draw_line(render_x, render_y, render_x + side, render_y, 2.0, GOLD);
        // Right line
        draw_line(render_x + side, render_y, render_x + side, render_y + side, 2.0, GOLD);
        // Bottom line
        draw_line(render_x + side, render_y + side, render_x, render_y + side, 2.0, GOLD);
        // Left line
        draw_line(render_x, render_y + side, render_x, render_y, 2.0, GOLD);
        
        // Renders the item inside with the relative ratio, but adds back the 2% left hand padding to the x and y values
        draw_rectangle(render_x + padding_ratio, render_y + padding_ratio, 
        side * x_ratio, side * x_ratio, self.color);
    }

    // Operates the func field
    pub fn use_item(&mut self, gamevalues: &mut Gamevalues, upgrades: &mut Vec<Upgrade>) {
        // Checks to see if there is an item to use
        if self.owned > 0 {
            // Use the func field
            (self.func)(gamevalues);
            // Reduce the amount of the item owned by one
            self.owned -= 1;

            // Find the corresponding upgrade by name
            for (_i, upgrade) in upgrades.iter_mut().enumerate() {
                // Reduces the amount of the corresponding upgrade owned by 1
                if upgrade.text == self.name {
                    upgrade.owned -= 1;
                }
            }
        }
    }
}