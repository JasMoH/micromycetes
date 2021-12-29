#![no_main]
#![no_std]

// set the panic handler
extern crate panic_semihosting;

use cortex_m_rt::{entry};
//use stm32g0xx_hal::prelude::*;



//shim second hardware for the moment
#[entry]
fn main() -> ! {
    loop {
        //do nothing in particular
    }

}

