use macroquad::prelude::*;

pub struct Gamevalues {
    pub counter: i32,
    pub clickpow_add: i32,
    pub clickpow_mult: i32,
    pub persecond: i32,
    pub water: f32,
    pub water_capacity: f32,
    pub alchemy_selected: usize,
    pub pets_selected: usize
}

impl Gamevalues {
    pub fn new(counter: i32, clickpow_add: i32, clickpow_mult: i32, persecond: i32, water: f32, water_capacity: f32, 
            alchemy_selected: usize, pets_selected: usize) -> Gamevalues {
        Gamevalues {
            counter,
            clickpow_add,
            clickpow_mult,
            persecond,
            water,
            water_capacity,
            alchemy_selected,
            pets_selected
        }
    }

    pub fn get_clickpower(&self) -> i32 {
        return self.clickpow_add * self.clickpow_mult;
    }
}