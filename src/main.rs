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
use crate::{components::{digit::Digit, timer::Timer}, f103::{gpio::{IO, LogOut, Pin, Port, TypePin}, spi::{NumSpi, Spi}, systick, usart::{Uart, Usarts}}};
use crate::components::{d1088bs, lcd1602a, timer, digit, keypad};
mod components;

fn main() -> ! {
    f103::rcc::init();
    let digit = Digit::new(
        [
        (2, Port::PA),
        (3, Port::PA),
        (6, Port::PA),
        (5, Port::PA),
        (4, Port::PA),
        (1, Port::PA),
        (0, Port::PA),
        (7, Port::PA)
        ]
    );

    let keypad = keypad::KeyPad::new(
        [
            (8, Port::PA),
            (9, Port::PA),
            (10, Port::PA),
            (11, Port::PA),
        ],
        [
            (15, Port::PB),
            (14, Port::PB),
            (12, Port::PB),
            (13, Port::PB),
        ]
    );

    loop {
        if let Some(num) = keypad.get_key() {
            digit.write_hex(num, false);
        }
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
