use crate::{components::digit, f103::{gpio::{self, IO, LogOut, Pin, Port, TypePin}, systick}};

pub struct Digit {
    digit    : [Pin; 7],
    point   : Pin,
}

impl Digit {
    pub fn new(arr : [(u8, Port); 8]) -> Self {
        let [a, b, c, d, e, f, g, dp] = arr.map(|(num, port)| {
            Pin::new(port, num, TypePin::Output(LogOut::Gpio(IO::PP)))
        });
        let ret = Digit {
            digit : [a, b, c, d, e, f, g],
            point : dp,
        };
        gpio::change_pins(&ret.digit, [false; 7]);
        ret.point.down();
        ret
    }

    pub fn write_hex(&self, num : u8, point : bool) {
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
            10 => gpio::change_pins(&self.digit, HA),
            11 => gpio::change_pins(&self.digit, HB),
            12 => gpio::change_pins(&self.digit, HC),
            13 => gpio::change_pins(&self.digit, HD),
            14 => gpio::change_pins(&self.digit, HE),
            15 => gpio::change_pins(&self.digit, HF),
            _ => return,
        }

        if point {
            self.point.up();
        } else {
            self.point.down();
        }
    }

    pub fn clear(&self) {
        gpio::change_pins(&self.digit, [false; 7]);
        self.point.down();
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
const HA    : [bool; 7] = [true, true, true, false, true, true, true];
const HB    : [bool; 7] = [true, true, true, true, true, true, true];
const HC    : [bool; 7] = [true, false, false, true, true, true, false];
const HD    : [bool; 7] = [true, true, true, true, true, true, false];
const HE    : [bool; 7] = [true, false, false, true, true, true, true];
const HF    : [bool; 7] = [true, false, false, false, true, true, true];