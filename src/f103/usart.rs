use core::ptr::{read_volatile, write_volatile};

use crate::f103::{gpio::{self, LogOut, Pin, Port, TypePin, IO}};

#[repr(C)]
struct regs {
    sr      : u32,
    dr      : u32,
    brr     : u32,
    cr1     : u32,
    cr2     : u32,
    cr3     : u32,
    gtpr    : u32,
}

pub enum Usarts {
    USART1,
    USART2,
    USART3,
}

pub struct Uart {
    tx  : Pin,
    rx  : Pin,
    num : Usarts,
}

impl Usarts {
    fn regs(&self) -> &'static mut regs {
        let addr = match self {
            Usarts::USART1 => USART1_BASE,
            Usarts::USART2 => USART2_BASE,
            Usarts::USART3 => USART3_BASE,
        };
        unsafe {
            &mut *(addr as *mut regs) 
        }
    }
}

impl Uart {
    pub fn new(num : Usarts) -> Self {
        let regs = num.regs();
        let (tx, rx) = match num {
            Usarts::USART1 => (
                Pin::new(Port::PA, 9, TypePin::Output(LogOut::Alt(IO::PP))),
                Pin::new(Port::PA, 10, TypePin::Input),
            ),
            Usarts::USART2 => (
                Pin::new(Port::PA, 2, TypePin::Output(LogOut::Alt(IO::PP))),
                Pin::new(Port::PA, 3, TypePin::Input),
            ),
            Usarts::USART3 => (
                Pin::new(Port::PB, 10, TypePin::Output(LogOut::Alt(IO::PP))),
                Pin::new(Port::PB, 11, TypePin::Input),
            )
        };
        let uart = Uart{tx, rx, num};
        unsafe {
            write_volatile(&mut regs.cr1, UE | TE | RE);
            write_volatile(&mut regs.brr, 0x271);
        }
        uart
    }

    pub fn send(&self, data : u8) {
        let regs = self.num.regs();
        unsafe {
            while read_volatile(&regs.sr) & TXE == 0 {}
            write_volatile(&mut regs.dr, data as u32);
        }
    }

    pub fn read(&self) -> u8 {
        let regs = self.num.regs();
        unsafe {
            while read_volatile(&regs.sr) & RXNE == 0 {}
            read_volatile(&regs.dr) as u8
        }
    }
}

const USART1_BASE : u32 = 0x40013800;
const USART2_BASE : u32 = 0x40004400;
const USART3_BASE : u32 = 0x40004800;

const UE    : u32 = 1 << 13;
const TE    : u32 = 1 << 3;
const RE    : u32 = 1 << 2;

const RXNE  : u32 = 1 << 5;
const TXE   : u32 = 1 << 7;