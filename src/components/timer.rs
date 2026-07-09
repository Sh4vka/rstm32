use crate::f103::{gpio::{self, LogOut, Pin, Port, TypePin, IO}, systick};

pub struct Timer {
    digit   : [Pin; 7],
    point   : Pin,
    pos     : [Pin; 4],
}

impl Timer {
    pub fn new(arr : [Pin; 12]) -> Self {
        let [a, b, c, d, e, f, g, p, d1, d2, d3, d4] = arr;
        let tim = Timer {
            digit : [a, b, c, d, e, f, g], 
            point : p, 
            pos : [d1, d2, d3, d4]
        };
        gpio::change_pins(&tim.pos, [true; 4]);
        gpio::change_pins(&tim.digit, [false; 7]);
        tim.point.down();
        tim
    }

    pub fn write_num(&self, num : u8, point : bool) {
        match num {
            0 => gpio::change_pins(&self.digit, ZERO),
            1 => gpio::change_pins(&self.digit, ONE),
            2 => gpio::change_pins(&self.digit, TWO),
            3 => gpio::change_pins(&self.digit, THREE),
            4 => gpio::change_pins(&self.digit, FOUR),
            5 => gpio::change_pins(&self.digit, FIVE),
            6 => gpio::change_pins(&self.digit, SIX),
            7 => gpio::change_pins(&self.digit, SEVEN),
            8 => gpio::change_pins(&self.digit, EIGHT),
            9 => gpio::change_pins(&self.digit, NINE),
            _ => return,
        }

        if point {
            self.point.up();
        } else {
            self.point.down();
        }
    }

    pub fn set_pos(&self, pos : u8) {
        if pos < 5 {
            self.pos[pos as usize -1].down();
        }
    }

    pub fn clear_pos(&self, pos : u8) {
        if pos < 5 {
            self.pos[pos as usize -1].up();
        }
    }
}

const ZERO  : [bool; 7] = [true, true, true, true, true, true, false];
const ONE   : [bool; 7] = [false, true, true, false, false, false, false];
const TWO   : [bool; 7] = [true, true, false, true, true, false, true];
const THREE : [bool; 7] = [true, true, true, true, false, false, true];
const FOUR  : [bool; 7] = [false, true, true, false, false, true, true];
const FIVE  : [bool; 7] = [true, false, true, true, false, true, true];
const SIX   : [bool; 7] = [true, false, true, true, true, true, true];
const SEVEN : [bool; 7] = [true, true, true, false, false, false, false];
const EIGHT : [bool; 7] = [true, true, true, true, true, true, true];
const NINE  : [bool; 7] = [true, true, true, true, false, true, true];