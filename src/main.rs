#![no_std]
#![no_main]

extern crate panic_halt;
pub extern crate stm32h7xx_hal;

use cortex_m_rt::entry;
use hal::prelude::*;
use rtt_logger::RTTLogger;
pub use stm32h7xx_hal as hal;

use log::info;

#[entry]
fn main() -> ! {
    static LOGGER: RTTLogger = RTTLogger::new(log::LevelFilter::Trace);
    rtt_target::rtt_init_print!();
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log::LevelFilter::Trace))
        .unwrap();

    let dp = hal::pac::Peripherals::take().unwrap();

    // Constrain and Freeze power
    info!("Setup PWR...                  ");
    let pwr = dp.PWR.constrain();
    let vos = pwr.freeze();

    // Constrain and Freeze clock
    info!("Setup RCC...                  ");
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sysclk(400.mhz()).freeze(vos, &dp.SYSCFG);
    info!("CPU clock: {:?}", ccdr.clocks.c_ck());

    info!("delay test - time difference between 't0' and 't1' should be 10 seconds");
    info!("t0");
    // 4_000_000_000 cycles delay should be exactly 10 seconds with a 400 MHz core clock.
    cortex_m::asm::delay(4_000_000_000);
    info!("t1");

    loop {}
}
