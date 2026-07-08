use core::ptr::write_volatile;
#[repr(C)]
struct regs {
    crl     : u32,
    crh     : u32,
    idr     : u32,
    odr     : u32,
    bsrr    : u32,
    brr     : u32,
    lckr    : u32,
}

#[derive(Copy, Clone)]
pub enum Port {
    PA,
    PB,
    PC,
    PD,
    PE,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum LogPin {
    Alt,
    Gpio,
}

#[derive(Copy, Clone)]
pub enum TypePin {
    Input,
    Output(LogPin)
}

pub struct Pin {
    port : Port,
    pin : u8,
    ptype : TypePin,
}

impl Port {
    fn regs(self) -> &'static mut regs {
        let addr = match self {
            Port::PA => GPIOA_BASE,
            Port::PB => GPIOB_BASE,
            Port::PC => GPIOC_BASE,
            Port::PD => GPIOD_BASE,
            Port::PE => GPIOE_BASE,
        };
        unsafe {
            &mut *(addr as *mut regs) 
        }
    }
}

impl Pin {
    pub fn new(port : Port, pin : u8, ptype : TypePin) -> Self {
        let gpio = port.regs();
        let shift = (pin % 8) * 4;
        let reg = if pin < 8 {
            &mut gpio.crl
        } else {
            &mut gpio.crh
        };
        match ptype {
            TypePin::Input => {
                *reg &= !(0b1111 << shift);
                *reg |= INPUT_FLOATING << shift;
            },
            TypePin::Output(mode) => {
                if mode == LogPin::Alt {
                    *reg &= !(0b1111 << shift);
                    *reg |= AF_PP_2MHZ << shift;
                } else {
                    *reg &= !(0b1111 << shift);
                    *reg |= OUTPUT_PP_2MHZ << shift;
                }
            }
        }
        Pin { port, pin, ptype }
    }

    pub fn up(&self) {
        let gpio = self.port.regs();
        unsafe {
            write_volatile(&mut gpio.bsrr, 1 << self.pin as u32);
        }
    }

    pub fn down(&self) {
        let gpio = self.port.regs();
        unsafe {
            write_volatile(&mut gpio.bsrr, 1 << (self.pin + 16) as u32);
        }
    }
}

// TODO сделать привязку к порту
pub fn change_pins<const N: usize>(pins : &[Pin; N], states : [bool; N]) {
    for i in 0..N {
        match states[i] {
            true => pins[i].up(),
            false => pins[i].down(),
        }
    }
}

const GPIOA_BASE    : u32 = 0x40010800;
const GPIOB_BASE    : u32 = 0x40010C00;
const GPIOC_BASE    : u32 = 0x40011000;
const GPIOD_BASE    : u32 = 0x40011400;
const GPIOE_BASE    : u32 = 0x40011800;

const INPUT_ANALOG  : u32 = 0b0000;
const INPUT_FLOATING: u32 = 0b0100;
const INPUT_PULL    : u32 = 0b1000;

const OUTPUT_PP_2MHZ: u32 = 0b0010;
const OUTPUT_OD_2MHZ: u32 = 0b0110;
const AF_PP_2MHZ    : u32 = 0b1010;
const AF_OD_2MHZ    : u32 = 0b1110;
