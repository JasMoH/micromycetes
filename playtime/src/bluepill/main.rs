#![no_main]
#![no_std]

// set the panic handler
extern crate panic_semihosting;

use core::sync::atomic::{AtomicBool, Ordering};
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use stm32f1xx_hal::prelude::*;

static TOGGLE_LED: AtomicBool = AtomicBool::new(false);

use smokie::access;

#[entry]
fn main() -> ! {
    let mut core = cortex_m::Peripherals::take().unwrap();
    let device = stm32f1xx_hal::stm32::Peripherals::take().unwrap();
    let mut rcc = device.RCC.constrain();
    let mut flash = device.FLASH.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(16.mhz())
        .freeze(&mut flash.acr);

    // configure the user led
    let mut gpioc = device.GPIOC.split(&mut rcc.apb2);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    //configure i2c
    let mut gpiob = device.GPIOB.split(&mut rcc.apb2); //acquire gpiob

    let i2c_pins = (
        gpiob.pb10.into_alternate_open_drain(&mut gpiob.crh), //SCL2
        gpiob.pb11.into_alternate_open_drain(&mut gpiob.crh), //SDA2
    );

    let i2c_mode = stm32f1xx_hal::i2c::Mode::Standard {
        frequency: stm32f1xx_hal::time::Hertz(100000),
    };

    let mut i2c_bus = stm32f1xx_hal::i2c::BlockingI2c::i2c2(
        device.I2C2,
        i2c_pins,
        i2c_mode,
        clocks,
        &mut rcc.apb1,
        //timeouts
        100,
        100,
        100,
        100,
    );

    let _id = access::device_id(&mut i2c_bus).unwrap();

    // configure SysTick to generate an exception every second
    core.SYST.set_clock_source(SystClkSource::Core);
    core.SYST.set_reload(clocks.sysclk().0);
    core.SYST.enable_counter();
    core.SYST.enable_interrupt();

    loop {
        // sleep
        cortex_m::asm::wfi();
        if TOGGLE_LED.swap(false, Ordering::AcqRel) {
            led.toggle().unwrap();
        }
    }
}

#[exception]
fn SysTick() {
    TOGGLE_LED.store(true, Ordering::Release);
}
