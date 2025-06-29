
use crate::gpio::*;

pub fn led_init(port: u32, pin: u32) {
    //1. Enable the peripheral clock
    enable_gpio_clock(port);
    //2. Set the gpio pin mode = output mode
    set_gpio_mode_output(port, pin);
    //3. Set the output type = pushpull
    set_gpio_output_type_push_pull(port, pin);
    //4. Set the output speed (optional) 
}

pub fn led_on(port: u32, pin: u32) {
    set_gpio_pin_state(port, pin, PinState::Hight);
}

pub fn led_off(port: u32, pin: u32) {
    set_gpio_pin_state(port, pin, PinState::Low);
}

pub fn led_toggle(port: u32, pin: u32) {
    set_gpio_pin_state(port, pin, PinState::Toggle);
}

