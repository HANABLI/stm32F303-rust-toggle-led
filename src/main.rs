#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;

use board::*;
use button::*;
use led::*;

mod board;
mod button;
mod exti;
mod gpio;
mod led;
mod mcu;
mod proc;
mod reg;
mod startup_stm32f303;

fn main() {
    led_init(BLUE_LED_PORT, BLUE_LED_PIN);

    led_on(BLUE_LED_PORT, BLUE_LED_PIN);
    // for _i in 0..100 {}
    // led_off(BLUE_LED_PORT, BLUE_LED_PIN);

    //TODO Add button code

    button::button_init(
        USER_BTN_PORT,
        USER_BTN_PIN,
        Mode::Interrupt(Trigger::RisingEdge),
    );

    loop {}
}

#[panic_handler]
pub fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

//button interrupt handler
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
extern "C" fn EXTI15_10_Handler() {
    led_off(BLUE_LED_PORT, BLUE_LED_PIN);
    //clear the pending interrupt in the EXTI
    button_clear_interrupt(USER_BTN_PIN);
}
