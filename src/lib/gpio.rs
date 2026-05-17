use core::ptr::{read_volatile, write_volatile};

const GPIOB_BASE    : u32 = 0x40010C00;

#[repr(C)]
struct gpio {
    crl     : u32,
    crh     : u32,
    idr     : u32,
    odr     : u32,
    bsrr    : u32,
    brr     : u32,
    lckr    : u32,
}

const MODE1     : u32 = 0b10 << 8;

pub fn init() {
    let gpio_ = GPIOB_BASE as *mut gpio;
    unsafe {
        (*gpio_).crl = 0;
        (*gpio_).crl |= MODE1;
    }
}

pub fn up() {
    let gpio_ = GPIOB_BASE as *mut gpio;
    unsafe {
        write_volatile(&mut (*gpio_).bsrr, 1 << 2);
    }
}

pub fn down() {
    let gpio_ = GPIOB_BASE as *mut gpio;
    unsafe {
        write_volatile(&mut (*gpio_).bsrr, 1 << 18);
    }
}
