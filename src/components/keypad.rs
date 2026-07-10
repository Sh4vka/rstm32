use crate::{components::digit, f103::{gpio::{self, IO, LogOut, Pin, Port, TypePin}, systick}};

pub struct KeyPad {
    cols : [Pin; 4],
    rows : [Pin; 4]
}

impl KeyPad {
    pub fn new(cols : [(u8, Port); 4], rows : [(u8, Port); 4]) -> Self {
        let [c1, c2, c3, c4] = cols.map(|(num, port)| {
            Pin::new(port, num, TypePin::Input)
        });
        let [r1, r2, r3, r4] = rows.map(|(num, port)| {
            Pin::new(port, num, TypePin::Output(LogOut::Gpio(IO::PP)))
        });
        let ret = KeyPad {
            cols : [c1, c2, c3, c4],
            rows : [r1, r2, r3, r4]
        };
        gpio::change_pins(&ret.rows, [false; 4]);
        ret
    }

    pub fn get_key(&self) -> Option<u8> {
        for row in 0..self.rows.len() {
            self.rows[row].up();
            systick::delay_ms(1);
            for col in 0..self.cols.len() {
                if self.cols[col].read() {
                    self.rows[row].down();
                    return Some(KEYS[row][col]);
                }
            }
            self.rows[row].down();
        }
        None
    }
}

const KEYS : [[u8; 4]; 4] = [
    [0x0, 0x1, 0x2, 0x3],
    [0x4, 0x5, 0x6, 0x7],
    [0x8, 0x9, 0xa, 0xb],
    [0xc, 0xd, 0xe, 0xf],
];