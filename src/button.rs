#![allow(dead_code)]
use crate::exti;
use crate::exti::*;
use crate::gpio::*;
use crate::mcu;
use crate::proc;

pub enum ButtonStatus {
    Pressed,
    Released,
}

pub enum Trigger {
    RisingEdge,
    FallingEdge,
}

pub enum Mode {
    Input,
    Interrupt(Trigger),
}

pub fn button_init(port: u32, pin: u32, mode: Mode) {
    enable_gpio_clock(port);
    set_gpio_mode_input(port, pin);
    exti::gpio::configure_syscfg(port, pin);
    match mode {
        Mode::Interrupt(trigger) => {
            match trigger {
                Trigger::FallingEdge => {
                    // configure the pin for falling edge detction
                    gpio::set_edge(pin, gpio::EdgeTrigger::Falling)
                }
                Trigger::RisingEdge => {
                    // configure the pin for Rising edge detction
                    gpio::set_edge(pin, gpio::EdgeTrigger::Rising)
                }
            }

            //enable the interrupt in exti.
            if let Some(exti_line) = exti::ExtiLine::from_pin(pin) {
                enable_interrupt(exti_line);
                if let Some(irq_num) = mcu::IRQ::from_pin(pin) {
                    proc::enable_irq(irq_num);
                }
            }
        }
        Mode::Input => {}
    }
}

pub fn button_read_status(port: u32, pin: u32) -> ButtonStatus {
    if get_gpio_pin_state(port, pin) {
        ButtonStatus::Pressed
    } else {
        ButtonStatus::Released
    }
}

pub fn button_clear_interrupt(pin: u32) {
    if let Some(exti_line) = exti::ExtiLine::from_pin(pin) {
        exti::clear_pending_interrupt(exti_line);
    }
}
