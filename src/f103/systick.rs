use core::ptr::{read_volatile, write_volatile};

#[repr(C)]
struct regs {
    ctrl    : u32,
    load    : u32,
    val     : u32,
    calib   : u32,
}

fn regs() -> &'static mut regs {
    unsafe {
        &mut *(SYSTICK_BASE as *mut regs)
    }
}

pub fn delay_ms(ms : u32) {
    let regs = regs();
    unsafe {
    write_volatile(&mut regs.load, 8000 - 1);
    write_volatile(&mut regs.val, 0);
    write_volatile(&mut regs.ctrl, CLKSOURCE | ENABLE);
    for _ in 0..ms {
        while (read_volatile(&regs.ctrl) & COUNTFLAG == 0) {};
    }
    write_volatile(&mut regs.ctrl, 0);
    }
}

const SYSTICK_BASE  : u32 = 0xE000E010;

const ENABLE    : u32 = 1 << 0;
const TICKINT   : u32 = 1 << 1;
const CLKSOURCE : u32 = 1 << 2;
const COUNTFLAG : u32 = 1 << 16;

const SKEW  : u32 = 1 << 30;
const NOREF : u32 = 1 << 31;

 