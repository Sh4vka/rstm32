#![no_std]
#![no_main]

use core::panic::PanicInfo;

use crate::f103::spi::Spi;
use crate::{calculator::Calc, f103::systick};

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

mod calculator;
mod components;
mod f103;

use crate::components::max7219::{self, MAX7219};

fn main() -> ! {
    f103::rcc::init();

    let mut max = MAX7219::new(f103::spi::NumSpi::SPI2);
    max.init();
    max.clear();
    max.set_pixel((1, 2), true);

    loop {

    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
