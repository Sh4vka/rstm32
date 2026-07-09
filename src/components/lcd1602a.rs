use crate::f103::{gpio::{self, LogPin, Pin, Port, TypePin, IO}, systick};

pub struct LCD8 {
    rs  : Pin,
    en  : Pin,
    bus : [Pin; 8],
}

impl LCD8 {
    pub fn new() -> Self{
        let a0 = Pin::new(Port::PA, 0, TypePin::Output(LogPin::Gpio(IO::PP)));
        let a1 = Pin::new(Port::PA, 1, TypePin::Output(LogPin::Gpio(IO::PP)));
        let a2 = Pin::new(Port::PA, 2, TypePin::Output(LogPin::Gpio(IO::PP)));
        let a3 = Pin::new(Port::PA, 3, TypePin::Output(LogPin::Gpio(IO::PP)));
        let a4 = Pin::new(Port::PA, 4, TypePin::Output(LogPin::Gpio(IO::PP)));
        let a5 = Pin::new(Port::PA, 5, TypePin::Output(LogPin::Gpio(IO::PP)));
        let a6 = Pin::new(Port::PA, 6, TypePin::Output(LogPin::Gpio(IO::PP)));
        let a7 = Pin::new(Port::PA, 7, TypePin::Output(LogPin::Gpio(IO::PP)));

        let b5 = Pin::new(Port::PB, 5, TypePin::Output(LogPin::Gpio(IO::PP)));
        let b6 = Pin::new(Port::PB, 6, TypePin::Output(LogPin::Gpio(IO::PP)));

        LCD8 {rs : b5, 
              en : b6,
              bus : [a0, a1, a2, a3, a4, a5, a6, a7]
        }
    }

    pub fn start(&self) {
        systick::delay_ms(15);
        for _ in 0..3 {
            self.write_command(0x38);
            systick::delay_ms(1);
        }
        self.write_command(0x08);
        self.write_command(CLEAR);
        systick::delay_ms(10);
        self.write_command(ENTINC);
        self.write_command(DISPLAYON);
    }

    pub fn print(&self, text : &str) {
        for ch in text.bytes() {
            self.write_data(ch);
        }
    }

    pub fn print_u32(&self, n : u32) {
        let mut buff = [0u8; 10];
        let mut i = 0;
        let mut _n = n;
        if n == 0 {
            self.write_data(b'0');
            return;
        }
        while _n > 0 {
            buff[i] = (_n % 10) as u8 + b'0';
            _n /= 10;
            i += 1;
        }
        while i > 0 {
            i -= 1;
            self.write_data(buff[i]);
        }
    }

    pub fn print_i32(&self, n : i32) {
        let mut buff = [0u8; 10];
        let mut i = 0;
        let mut _n = n;
        if n == 0 {
            self.write_data(b'0');
            return;
        }
        if n < 0 {
            self.write_data(b'-');
            _n *= -1;
        }
        while _n > 0 {
            buff[i] = (_n % 10) as u8 + b'0';
            _n /= 10;
            i += 1;
        }
        while i > 0 {
            i -= 1;
            self.write_data(buff[i]);
        }
    }

    pub fn set_cursor(&self, col : u8, row : u8) {
        let addr = match row  {
            0 => col,
            1 => 0x40 + col,
            _ => return,
        };
        self.write_command(0x80 | addr);
    }
    
    pub fn clear(&self) {
        self.write_command(CLEAR);
        systick::delay_ms(10);
    }

    fn pulse_enable(&self) {
        systick::delay_ms(10);
        self.en.up();
        systick::delay_ms(10);
        self.en.down();
    }

    fn write_command(&self, data : u8) {
        self.rs.down();
        systick::delay_ms(1);
        gpio::change_pins(&self.bus, self.from_u8(data));
        self.pulse_enable();
    }

    fn write_data(&self, data : u8) {
        self.rs.up();
        systick::delay_ms(1);
        gpio::change_pins(&self.bus, self.from_u8(data));
        self.pulse_enable();
    }

    fn from_u8(&self, num : u8) -> [bool; 8] {
        let mut arr = [false; 8];
        for i in 0..8 {
            arr[i] = (num >> i) & 1 == 1
        }
        arr
    }
}

const CLEAR     : u8 = 0x01;
const RETURN    : u8 = 0x02;
const ENTDEC    : u8 = 0x04;
const ENTINC    : u8 = 0x06;
const ENTSHIFT  : u8 = 0x05;
const DISPLAYON : u8 = 0x0c;
