#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]
#![allow(dead_code)]

//If you forget to import it, the compiler won't know which
//entry attribute you are referring to, and you'll get an error
use cortex_m_rt::{entry, exception};

use cortex_m::peripheral::Peripherals;
use cortex_m::peripheral::syst;
use panic_halt as _;

mod board;
mod gpio;
mod led;
mod mcu;
mod reg;

#[entry]
fn main() -> ! {
    led::led_init(board::BLUE_LED_PORT, board::BLUE_LED_PIN);
    led::led_on(board::BLUE_LED_PORT, board::BLUE_LED_PIN);

    let mut peripherals = Peripherals::take().unwrap();
    let systick = &mut peripherals.SYST;
    systick.set_clock_source(syst::SystClkSource::Core);
    systick.set_reload(7_000_000 - 1);
    systick.clear_current();
    systick.enable_interrupt();
    systick.enable_counter();
    loop { /* .. */ }
}

#[exception]
fn SysTick() {
    led::led_toggle(board::BLUE_LED_PORT, board::BLUE_LED_PIN);
}
