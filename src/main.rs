#![no_std]
#![no_main]

use core::panic::PanicInfo;

use crate::f103::{gpio::{self, LogPin}, systick};

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
use f103::gpio::{Pin, Port, TypePin};

fn main() -> ! {
    f103::rcc::init();
    let a0 = Pin::new(Port::PA, 0, TypePin::Output(LogPin::Gpio));
    let a1 = Pin::new(Port::PA, 1, TypePin::Output(LogPin::Gpio));
    let c15 = Pin::new(Port::PC, 15, TypePin::Output(LogPin::Gpio));
    let pins = [c15, a0, a1];
    loop {
        gpio::change_pins(&pins, [true, false, true]);
        systick::delay_ms(10);
        gpio::change_pins(&pins, [false, true, false]);
        systick::delay_ms(10);
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
