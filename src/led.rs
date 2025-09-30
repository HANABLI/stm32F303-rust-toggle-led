//! # LED Control Module
//!
//! This module provides basic functions to initialize and control a LED
//! connected to a GPIO pin.

use crate::gpio::*;
/// This function is used to initialise the led peripheral configuration.
/// It Enable the peripheral clock, set the gpio pin mode to output mode and
/// set the output to pushpull type.
/// ! setting the output speed is optional.
///
/// # Parameters
///
/// - *port* : The GPIO port address to which the LED is connected.
/// - *pin* : The GPIO pin number to which the LED is connected.
///
/// # Example
/// ```
/// led_init(BLUE_LED_PORT, BLUE_LED_PIN)
///
/// ```
///
pub fn led_init(port: u32, pin: u32) {
    //1. Enable the peripheral clock
    enable_gpio_clock(port);
    //2. Set the gpio pin mode = output mode
    set_gpio_mode_output(port, pin);
    //3. Set the output type = pushpull
    set_gpio_output_type_push_pull(port, pin);
    //4. Set the output speed (optional)
}
/// This function is used to set the led to ON.
///
/// # Parameters
///
/// - *port* : The GPIO port address to which the LED is connected.
/// - *pin* : The GPIO pin number to which the LED is connected.
///
/// # Example
///
/// ```
/// led_on(BLUE_LED_PORT, BLUE_LED_PIN)
///
/// ```
pub fn led_on(port: u32, pin: u32) {
    set_gpio_pin_state(port, pin, PinState::Hight);
}
/// This function is used to set the led to OFF.
/// # Parameters
///
/// - *port* : The GPIO port address to which the LED is connected.
/// - *pin* : The GPIO pin number to which the LED is connected.
///
/// # Example:
/// ```
/// led_off(BLUE_LED_PORT, BLUE_LED_PIN)
///
/// ```
pub fn led_off(port: u32, pin: u32) {
    set_gpio_pin_state(port, pin, PinState::Low);
}
/// This function is used to toggle the led.
/// # Parameters
///
/// - *port* : The GPIO port address to which the LED is connected.
/// - *pin* : The GPIO pin number to which the LED is connected.
///
/// # Example:
/// ```
/// led_toggle(BLUE_LED_PORT, BLUE_LED_PIN)
///
/// ```
pub fn led_toggle(port: u32, pin: u32) {
    set_gpio_pin_state(port, pin, PinState::Toggle);
}
