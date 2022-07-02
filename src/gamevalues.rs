use macroquad::prelude::*;

pub struct Gamevalues {
    pub clickpow_add: i32,
    pub clickpow_mult: i32,
    pub persecond: i32,
    pub water: f32,
    pub water_capacity: f32
}

impl Gamevalues {
    pub fn new(clickpow_add: i32, clickpow_mult: i32, persecond: i32, water: f32, water_capacity: f32) -> Gamevalues {
        Gamevalues {
            clickpow_add,
            clickpow_mult,
            persecond,
            water,
            water_capacity
        }
    }

    pub fn get_clickpower(&self) -> i32 {
        return self.clickpow_add * self.clickpow_mult;
    }
}