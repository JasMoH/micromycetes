#![no_main]
#![no_std]

use core::panic::PanicInfo;

use esp32_hal::{
    clock_control::{self, CPUSource, ClockControl},
    dport::Split,
    i2c::{self},
    prelude::*,
    target::{Peripherals},
    timer::Timer,
};
use smokie::access;

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();

    let (mut dport, dport_clock_control) = dp.DPORT.split();

    // setup clocks & watchdog
    let mut clkcntrl = ClockControl::new(
        dp.RTCCNTL,
        dp.APB_CTRL,
        dport_clock_control,
        clock_control::XTAL_FREQUENCY_AUTO,
    )
        .unwrap();

    // set desired clock frequencies
    clkcntrl
        .set_cpu_frequencies(
            CPUSource::PLL,
            80.MHz(),
            CPUSource::PLL,
            240.MHz(),
            CPUSource::PLL,
            80.MHz(),
        )
        .unwrap();

    // disable RTC watchdog
    let (clkcntrl_config, mut watchdog) = clkcntrl.freeze().unwrap();
    watchdog.disable();

    // disable MST watchdogs
    let (.., mut watchdog0) = Timer::new(dp.TIMG0, clkcntrl_config);
    let (.., mut watchdog1) = Timer::new(dp.TIMG1, clkcntrl_config);
    watchdog0.disable();
    watchdog1.disable();

    let pins = dp.GPIO.split();
    let mut i2c0 = i2c::I2C::new(
        dp.I2C0,
        i2c::Pins {
            sda: pins.gpio4,
            scl: pins.gpio15,
        },
        400_000,
        &mut dport,
    );

    loop {
        access::check_id(&mut i2c0);
    }

}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
//    dprintln!("----- PANIC -----");
//    dprintln!("{:?}", info);
    loop {}
}