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
use crate::{components::timer::Timer, f103::{gpio::{IO, LogOut, Pin, Port, TypePin}, spi::{NumSpi, Spi}, systick, usart::{Uart, Usarts}}};
use crate::components::{d1088bs, lcd1602a, timer};
mod components;

fn main() -> ! {
    f103::rcc::init();
    let a = Pin::new(Port::PA, 11, TypePin::Output(LogOut::Gpio(IO::PP)));
    let b = Pin::new(Port::PB, 7, TypePin::Output(LogOut::Gpio(IO::PP)));
    let c = Pin::new(Port::PB, 15, TypePin::Output(LogOut::Gpio(IO::PP)));
    let d = Pin::new(Port::PB, 13, TypePin::Output(LogOut::Gpio(IO::PP)));
    let e = Pin::new(Port::PB, 12, TypePin::Output(LogOut::Gpio(IO::PP)));
    let f = Pin::new(Port::PA, 12, TypePin::Output(LogOut::Gpio(IO::PP)));
    let g = Pin::new(Port::PA, 8, TypePin::Output(LogOut::Gpio(IO::PP)));
    let p = Pin::new(Port::PB, 14, TypePin::Output(LogOut::Gpio(IO::PP)));
    let d1 = Pin::new(Port::PA, 10, TypePin::Output(LogOut::Gpio(IO::PP)));
    let d2 = Pin::new(Port::PB, 5, TypePin::Output(LogOut::Gpio(IO::PP)));
    let d3 = Pin::new(Port::PB, 6, TypePin::Output(LogOut::Gpio(IO::PP)));
    let d4 = Pin::new(Port::PA, 9, TypePin::Output(LogOut::Gpio(IO::PP)));

    let timer = Timer::new([a, b, c, d, e, f, g, p, d1, d2, d3, d4]);
    loop {
        timer.write_num(1, false);
        timer.set_pos(1);
        systick::delay_ms(1);
        timer.clear_pos(1);

        timer.write_num(2, true);
        timer.set_pos(2);
        systick::delay_ms(1);
        timer.clear_pos(2);

        timer.write_num(0, false);
        timer.set_pos(3);
        systick::delay_ms(1);
        timer.clear_pos(3);

        timer.write_num(5, false);
        timer.set_pos(4);
        systick::delay_ms(1);
        timer.clear_pos(4);
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
