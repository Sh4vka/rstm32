use core::{iter::Scan, ptr::{read_volatile, write_volatile}};

use crate::f103::{gpio::{self, LogOut, Pin, Port, TypePin, IO}};

#[repr(C)]
struct regs {
    cr1         : u32,
    cr2         : u32,
    sr          : u32,
    dr          : u32,
    crcpr       : u32,
    spi_txcrcr  : u32,
    spi_i2scfgr : u32,
    spi_i2spr   : u32,
}

pub struct Spi {
    cs      : Pin,
    sck     : Pin,
    miso    : Pin,
    mosi    : Pin,
    num     : NumSpi,
    cs_type : CS_Type,
}

pub enum NumSpi {
    SPI1,
    SPI2,
}

#[derive(PartialEq)]
pub enum CS_Type {
    Prog,
    Alt,
}

impl NumSpi {
    fn regs(&self) -> &'static mut regs {
        let addr = match self {
            NumSpi::SPI1 => SPI1_BASE,
            NumSpi::SPI2 => SPI2_BASE,
        };
        unsafe {
            &mut *(addr as *mut regs) 
        }
    }
}

impl Spi {
    pub fn master(num : NumSpi, cs_type : CS_Type) -> Self {
        let regs = num.regs();
        let (cs, sck, miso, mosi) = match num {
            NumSpi::SPI1 => (
                match cs_type {
                    CS_Type::Alt => Pin::new(Port::PA, 4, TypePin::Output(LogOut::Alt(IO::PP))),
                    CS_Type::Prog => Pin::new(Port::PA, 4, TypePin::Output(LogOut::Gpio(IO::PP))),
                },
                Pin::new(Port::PA, 5, TypePin::Output(LogOut::Alt(IO::PP))),
                Pin::new(Port::PA, 6, TypePin::Input),
                Pin::new(Port::PA, 7, TypePin::Output(LogOut::Alt(IO::PP))),
            ),
            NumSpi::SPI2 => (
                match cs_type {
                    CS_Type::Alt => Pin::new(Port::PB, 12, TypePin::Output(LogOut::Alt(IO::PP))),
                    CS_Type::Prog => Pin::new(Port::PB, 12, TypePin::Output(LogOut::Gpio(IO::PP))),
                },
                Pin::new(Port::PB, 13, TypePin::Output(LogOut::Alt(IO::PP))),
                Pin::new(Port::PB, 14, TypePin::Input),
                Pin::new(Port::PB, 15, TypePin::Output(LogOut::Alt(IO::PP))),
            ),
        };
        let spi = Spi {cs, sck, mosi, miso, num, cs_type};
        unsafe {
            match spi.cs_type {
                CS_Type::Alt => {
                write_volatile(&mut regs.cr1, MSTR | (0b100 << 3));
                write_volatile(&mut regs.cr2, SSOE);
                }
                CS_Type::Prog => {
                    write_volatile(&mut regs.cr1, MSTR | SSI | SSM | (0b100 << 3));
                }
            }
        }
        spi.enable();
        spi
    }

    pub fn slave(num : NumSpi) -> Self {
        let regs = num.regs();
        let (cs, sck, miso, mosi) = match num {
            NumSpi::SPI1 => (
                Pin::new(Port::PA, 4, TypePin::Input),
                Pin::new(Port::PA, 5, TypePin::Input),
                Pin::new(Port::PA, 6, TypePin::Output(LogOut::Alt(IO::PP))),
                Pin::new(Port::PA, 7, TypePin::Input),
            ),
            NumSpi::SPI2 => (
                Pin::new(Port::PB, 12, TypePin::Input),
                Pin::new(Port::PB, 13, TypePin::Input),
                Pin::new(Port::PB, 14, TypePin::Output(LogOut::Alt(IO::PP))),
                Pin::new(Port::PB, 15, TypePin::Input),
            ),
        };
        let spi = Spi {cs, sck, mosi, miso, num, cs_type : CS_Type::Alt};
        spi.enable();
        spi
    }

    pub fn send_byte(&self, data : u8) {
        let regs = self.num.regs();
        while self.check_status() & TXE == 0 {};
        unsafe {
            write_volatile(&mut regs.dr, data as u32);
        }
    }

    pub fn flush(&self) {
        let regs = self.num.regs();
        loop {
            let stat = self.check_status();
            if stat & BSY == 0 && stat & TXE != 0 {
                return;
            }
        }
    }
    
    pub fn send(&self, data : u8) {
        self.send_byte(data);
        self.flush();
    }

    pub fn read(&self) -> u8 {
        let regs = self.num.regs();
        unsafe {
            read_volatile(&regs.dr) as u8
        }
    }

    pub fn cs_up(&self) {
        if self.cs_type == CS_Type::Prog {
            self.cs.up();
        }
    }

    pub fn cs_down(&self) {
        if self.cs_type == CS_Type::Prog {
            self.cs.down();
        }
    }
    
    fn enable(&self) {
        let regs = self.num.regs();
        unsafe {
            let mut cr1 = read_volatile(&regs.cr1);
            cr1 |= SPE;
            write_volatile(&mut regs.cr1, cr1);
        }
    }

    fn disable(&self) {
        let regs = self.num.regs();
        unsafe {
            let mask = read_volatile(&regs.cr1);
            write_volatile(&mut regs.cr1, (mask & !SPE));
        }
    }

    fn check_status(&self) -> u32 {
        let regs = self.num.regs();
        unsafe {
            read_volatile(&regs.sr)
        }
    }
}

const SPI1_BASE : u32 = 0x40013000;
const SPI2_BASE : u32 = 0x40003800;

const MSTR  : u32 = 1 << 2;
const SPE   : u32 = 1 << 6;
const LSBT  : u32 = 1 << 7;
const SSI   : u32 = 1 << 8;
const SSM   : u32 = 1 << 9;

const SSOE  : u32 = 1 << 2;

const RXNE  : u32 = 1 << 0;
const TXE   : u32 = 1 << 1;
const BSY   : u32 = 1 << 7;