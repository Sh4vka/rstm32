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
use crate::f103::{gpio::{Pin, Port, TypePin, LogOut, IO}, systick, spi::{NumSpi, Spi}, usart::{Usarts, Uart}};
use crate::components::{d1088bs, lcd1602a};
mod components;

fn main() -> ! {
    f103::rcc::init();
    let uart1 = Uart::new(Usarts::USART1);
    let uart2 = Uart::new(Usarts::USART2);
    let (mut data1, mut data2) = (0, 0);
    loop {
       uart1.send(0x55); 
       data1 = uart2.read();
       uart2.send(0x01);
       data2 = uart1.read();
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
