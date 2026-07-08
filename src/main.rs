#![no_std]
#![no_main]

use core::panic::PanicInfo;

use crate::f103::gpio::LogPin;

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

fn delay_ms(mut count: u32) {
    while count > 1 {
        count -= 1;
    }
}

mod f103;
use f103::gpio::{Pin, Port, TypePin};

fn main() -> ! {
    f103::rcc::init();
    let a0 = Pin::new(Port::PA, 0, TypePin::Output(LogPin::Gpio));
    let a1 = Pin::new(Port::PA, 1, TypePin::Output(LogPin::Gpio));
    loop {
        a0.up();
        a1.down();
        delay_ms(5000000);
        a1.up();
        a0.down();
        delay_ms(5000000);
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
