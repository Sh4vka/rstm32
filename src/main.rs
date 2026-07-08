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
use f103::{gpio::{Pin, Port, TypePin, LogPin}, systick, gpio};

use crate::components::d1088bs;
mod components;

fn main() -> ! {
    f103::rcc::init();
    let mut m = d1088bs::Matrix::new();
    loop {
        for y in 0..8 {
            for x in 0..8 {
                m.write(x, y);
                systick::delay_ms(100);
                m.erase(x, y);
            }
        }
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
