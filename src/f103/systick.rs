use core::ptr::write_volatile;

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

pub fn init(us : u32) {
    let regs = regs();
        regs.load = us;
        regs.val = 0;
        regs.ctrl = CLKSOURCE | ENABLE;
}

pub fn wait() {
    let regs = regs();
    while (regs.ctrl & COUNTFLAG == 0) {}
}

const SYSTICK_BASE  : u32 = 0xE000E010;

const ENABLE    : u32 = 1 << 0;
const TICKINT   : u32 = 1 << 1;
const CLKSOURCE : u32 = 1 << 2;
const COUNTFLAG : u32 = 1 << 16;

const SKEW  : u32 = 1 << 30;
const NOREF : u32 = 1 << 31;

 