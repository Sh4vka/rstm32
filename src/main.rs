#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[repr(C)]
#[derive(Copy, Clone)]
union Vector {
    handler: unsafe extern "C" fn() -> !,
    reserved: usize,
}

#[link_section = ".vector_table"]
#[no_mangle]
static VECTORS: [Vector; 16] = [
    Vector {
        reserved: 0x20005000,
    },
    Vector { handler: Reset },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
];

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    main()
}

mod f103;
use crate::f103::{gpio::{Pin, Port, TypePin, LogOut, IO}, systick, spi::{NumSpi, Spi}};
use crate::components::{d1088bs, lcd1602a};
mod components;

fn main() -> ! {
    f103::rcc::init();
    let spi1 = Spi::master(NumSpi::SPI1);
    let spi2 = Spi::slave(NumSpi::SPI2);
    let mut data1 = 0;
    let mut data2 = 0;
    loop {
        spi2.send(0x32);
        spi1.send(0x55);
        data1 = spi2.read();
        data2 = spi1.read();
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
