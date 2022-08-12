use::macroquad::prelude::*;

use crate::{gamevalues::{Gamevalues, self}};

pub struct Petitems {
    pub name: String,
    pub owned: bool,
    color: Color,
    func: Box<dyn Fn(&mut Gamevalues)>
}

impl Petitems {
    pub fn new (name: &str, owned: bool, color: Color, func: Box<dyn Fn(&mut Gamevalues)>) -> Petitems {
        Petitems {
            name: name.to_owned(),
            owned,
            color,
            func
        }
    }

    // i just copy and pasted the alchemy item rendering lol
    // Renders the item in a box, with some padding
    pub fn render_item(&self, render_x: f32, render_y: f32, side: f32) {
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

    pub fn use_pet(&mut self, gamevalues: &mut Gamevalues) {
        // Makes sure that the pet is owned
        if self.owned == true {
            // Use the func field to run the effect
            (self.func)(gamevalues);
        }
    }
}