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

fn delay_ms(mut count: u32) {
    while count > 1 {
        count -= 1;
    }
}

mod lib;

fn main() -> ! {
    lib::rcc::init();
    lib::gpio::init();
    loop {
        lib::gpio::up();
        delay_ms(50);
        lib::gpio::down();
        delay_ms(50);
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
