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

const AFIORST   : u32 = 1 << 0;
const IOPARST    : u32 = 1 << 2;
const IOPBRST    : u32 = 1 << 3;
const IOPCRST    : u32 = 1 << 4;
const IOPDRST    : u32 = 1 << 5;
const IOPERST   : u32 = 1 << 6;
const ADC1RST   : u32 = 1 << 9;
const ADC2RST   : u32 = 1 << 10;
const TIM1RST   : u32 = 1 << 11;
const SPI1RST   : u32 = 1 << 12;
const USART1RST : u32 = 1 << 14;

const TIM2RST   : u32 = 1 << 0;
const TIM3RST   : u32 = 1 << 1;
const TIM4RST   : u32 = 1 << 2;
const TIM5RST   : u32 = 1 << 3;
const TIM6RST   : u32 = 1 << 4;
const TIM7RST   : u32 = 1 << 5;
const WWDGRST   : u32 = 1 << 11;
const SPI2RST   : u32 = 1 << 14;
const SPI3RST   : u32 = 1 << 15;
const USART2RST : u32 = 1 << 17;
const USART3RST : u32 = 1 << 18;
const UART4RST  : u32 = 1 << 19;
const UART5RST  : u32 = 1 << 20;
const I2C1RST   : u32 = 1 << 21;
const I2C2RST   : u32 = 1 << 22;
const CAN1RST   : u32 = 1 << 25;
const CAN2RST   : u32 = 1 << 26;
const BKPRST    : u32 = 1 << 27;
const PWRRST    : u32 = 1 << 28;
const DACRST    : u32 = 1 << 29;

pub fn init() {
    let rcc_ = RCC_BASE as *mut regs;
    let mask = IOPARST | IOPBRST | 
                IOPCRST | IOPDRST |
                IOPERST | SPI1RST;
    unsafe {
        (*rcc_).apb2enr |= mask;
    }
    let mask = SPI2RST;
    unsafe {
        (*rcc_).apb1enr |= mask
    }
}
