use crate::{components::digit, f103::{gpio::{self, IO, LogOut, Pin, Port, TypePin}, systick, spi::{NumSpi, Spi}}};

pub struct MAX7219 {
    spi : Spi,
    framebuffer : [u8; 32],
}

impl MAX7219{
    pub fn new(num_spi : NumSpi) -> Self {
        let spi = Spi::master(num_spi, crate::f103::spi::CS_Type::Prog);
        let framebuffer = [0u8; 32];
        MAX7219 { spi, framebuffer }
    }

    pub fn init(&self) {
        self.send_all(0x09, 0x00);
        systick::delay_ms(1);

        self.send_all(0x0a, 0x05);
        systick::delay_ms(1);

        self.send_all(0x0b, 0x07);
        systick::delay_ms(1);

        self.send_all(0x0c, 0x01);
        systick::delay_ms(1);

        self.send_all(0x0f, 0x00);
        systick::delay_ms(1);
    }

    pub fn send_all(&self, addr : u8, data : u8) {
        self.start();
        for _ in 0..4 {
            self.spi.send_byte(addr);
            self.spi.send_byte(data);
        }
        self.spi.flush();
        self.finish();
        systick::delay_ms(1);
    }

    pub fn send_custom(&self, addr : u8, data : [u8; 4]) {
        self.start();
        for byte in data {
            self.spi.send_byte(addr);
            self.spi.send_byte(byte);
        }
        self.finish();
    }

    fn start(&self) {
        self.spi.cs_down();
    }

    fn finish(&self) {
        self.spi.cs_up();
    }

    pub fn set_pixel(&mut self, pos : (usize, usize), on : bool) {
        let (x, y) = pos;
        if y >= 32 || x >= 8 {
            return;
        }
        if on {
            self.framebuffer[y] |= 1 << x;
        } else {
            self.framebuffer[y] &= !(1 << x);
        }
    }

    pub fn clear(&mut self) {
        self.framebuffer.fill(0);
        self.flush();
    }

    pub fn flush(&self) {
        for row in 0..8 {
            self.send_custom(row + 1, 
                [
                    self.framebuffer[row as usize],
                    self.framebuffer[(row + 8) as usize],
                    self.framebuffer[(row + 16) as usize],
                    self.framebuffer[(row + 24) as usize],
                ]
            );
        }
    }

}
