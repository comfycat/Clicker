use macroquad::prelude::*;

pub struct Alchemy {
    width: f32,
    height: f32,
    pub unlocked: bool,
    pub visible: bool,
    pub water: f32,
    water_capacity: f32
}

impl Alchemy {
    pub fn new(width: f32, height: f32, unlocked: bool, visible: bool, water: f32, water_capacity: f32) -> Alchemy {
        Alchemy {
            width,
            height,
            unlocked,
            visible,
            water,  
            water_capacity          
        }
    }

    // Renders the water bar
    pub fn render_water(&self, render_x: f32, render_y: f32) {
        // Renders the greyed out bar for the maximum capacity
        draw_rectangle(render_x + self.width * 0.05, render_y + self.height * 0.1, self.width * 0.9, 
            self.height * 0.1, DARKBLUE);
        // Renders the bar relative to the amount of water owned
        let water_percent: f32 = self.water / self.water_capacity;
        draw_rectangle(render_x + self.width * 0.05, render_y + self.height * 0.1, self.width * 0.9 * water_percent, 
            self.height * 0.1, BLUE);
    }
}