use macroquad::prelude::*;
use crate::alchemy::Alchemy;

pub struct Gamevalues {
    pub clickpow_add: i32,
    pub clickpow_mult: i32,
    pub persecond: i32,
    pub alchemy: Alchemy
}

impl Gamevalues {
    pub fn new(clickpow_add: i32, clickpow_mult: i32, persecond: i32, alchemy: Alchemy) -> Gamevalues {
        Gamevalues {
            clickpow_add,
            clickpow_mult,
            persecond,
            alchemy,
        }
    }

    pub fn get_clickpower(&self) -> i32 {
        return self.clickpow_add * self.clickpow_mult;
    }
}