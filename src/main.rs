#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;

use crate::board::*;
use crate::led::*;


mod startup_stm32f303;
mod mcu;
mod board;
mod button;
mod led;
mod reg;
mod gpio;


#[unsafe(no_mangle)]
fn main() { 
    led_init(BLUE_LED_PORT, BLUE_LED_PIN);
    
    led_on(BLUE_LED_PORT, BLUE_LED_PIN);
    for _i in 0..3 {
    }
    led_off(BLUE_LED_PORT, BLUE_LED_PIN);

    loop {
    }
}

#[panic_handler]
pub fn panic_handler(_info: &PanicInfo) -> ! {
    loop {  }
}

//button interrupt handler
#[allow(non_snake_case)]
pub fn EXTI0_Handler() {
    led_toggle(board::BLUE_LED_PORT, BLUE_LED_PIN );
}

