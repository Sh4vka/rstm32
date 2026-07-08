const RCC_BASE  : u32 = 0x40021000;

#[repr(C)]
struct regs {
    cr      : u32,
    cfgr    : u32,
    cir     : u32,
    apb2rstr: u32,
    apb1rstr: u32,
    ahbenr  : u32,
    apb2enr : u32,
    apb1enr : u32,
    dbcr    : u32,
    csr     : u32,
    ahbrstr : u32,
    cfgr2   : u32,
}

const RCC_APB2ENR_AFIOEN    : u32 = 1 << 0;
const RCC_APB2ENR_IOPAEN    : u32 = 1 << 2;
const RCC_APB2ENR_IOPBEN    : u32 = 1 << 3;
const RCC_APB2ENR_IOPCEN    : u32 = 1 << 4;
const RCC_APB2ENR_IOPDEN    : u32 = 1 << 5;

pub fn init() {
    let rcc_ = RCC_BASE as *mut regs;
    let mask = RCC_APB2ENR_IOPAEN | RCC_APB2ENR_IOPBEN | 
               RCC_APB2ENR_IOPCEN | RCC_APB2ENR_IOPDEN;
    unsafe {
        (*rcc_).apb2enr |= mask;
    }
}
