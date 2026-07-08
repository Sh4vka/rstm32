use crate::f103::{gpio::{Pin, Port, TypePin, LogPin}, gpio};

pub struct Matrix {
    rows        : [Pin; 8],
    columns     : [Pin; 8],
    cur_rows    : [bool; 8],
    cur_cols    : [bool; 8],
}

impl Matrix {
    pub fn new() -> Self {
        let r5 = Pin::new(Port::PA, 0, TypePin::Output(LogPin::Gpio));
        let r7 = Pin::new(Port::PA, 1, TypePin::Output(LogPin::Gpio));
        let c2 = Pin::new(Port::PA, 2, TypePin::Output(LogPin::Gpio));
        let c3 = Pin::new(Port::PA, 3, TypePin::Output(LogPin::Gpio));
        let r8 = Pin::new(Port::PA, 4, TypePin::Output(LogPin::Gpio));
        let c5 = Pin::new(Port::PA, 5, TypePin::Output(LogPin::Gpio));
        let r6 = Pin::new(Port::PA, 6, TypePin::Output(LogPin::Gpio));
        let r3 = Pin::new(Port::PA, 7, TypePin::Output(LogPin::Gpio));
        
        let r1 = Pin::new(Port::PB, 12, TypePin::Output(LogPin::Gpio));
        let c4 = Pin::new(Port::PB, 13, TypePin::Output(LogPin::Gpio));
        let c6 = Pin::new(Port::PB, 14, TypePin::Output(LogPin::Gpio));
        let r4 = Pin::new(Port::PB, 15, TypePin::Output(LogPin::Gpio));
        let c1 = Pin::new(Port::PA, 8, TypePin::Output(LogPin::Gpio));
        let r2 = Pin::new(Port::PA, 9, TypePin::Output(LogPin::Gpio));
        let c7 = Pin::new(Port::PA, 10, TypePin::Output(LogPin::Gpio));
        let c8 = Pin::new(Port::PA, 11, TypePin::Output(LogPin::Gpio));

        let mut m = Matrix {
            rows    : [r1, r2, r3, r4, r5, r6, r7, r8],
            columns : [c1, c2, c3, c4, c5, c6, c7, c8],
            cur_rows : [false; 8],
            cur_cols : [true; 8]
        }; 
        m.update();
        m
    }

    pub fn write(&mut self, x : usize, y : usize) {
        if x < 8 && y < 8 {
            self.cur_rows[y] = true;
            self.cur_cols[x] = false;
        }
        self.update();
    }

    pub fn erase(&mut self, x : usize, y : usize) {
        if x < 8 && y < 8 {
            self.cur_rows[y] = false;
            self.cur_cols[x] = true;
        }
        self.update();
    }

    fn update(&mut self) {
        gpio::change_pins(&self.rows, self.cur_rows);
        gpio::change_pins(&self.columns, self.cur_cols);
    }
}
