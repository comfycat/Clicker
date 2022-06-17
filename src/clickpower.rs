use macroquad::prelude::*;

pub struct Clickpower {
    pub clickpow_add: i32,
    pub clickpow_mult: i32
}

impl Clickpower {
    pub fn new(clickpow_add: i32, clickpow_mult: i32) -> Clickpower {
        Clickpower {
            clickpow_add,
            clickpow_mult
        }
    }

    pub fn get_clickpower(&self) -> i32 {
        return self.clickpow_add * self.clickpow_mult;
    }
}