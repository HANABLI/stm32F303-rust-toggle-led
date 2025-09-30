use crate::mcu::*;
use crate::reg::*;

pub fn enable_gpio_clock(port: u32) {
    let rcc_ahb_enr_addr = (RCC_BASE + 0x14) as *mut u32;

    match port {
        GPIOA_BASE => {
            // enable the 17th bit of the rcc_ahb_enr_addr
            reg_set_bit(rcc_ahb_enr_addr, 17, true);
        }

        GPIOB_BASE => {
            // enable the 18th bit of the rcc_ahb_enr_addr
            reg_set_bit(rcc_ahb_enr_addr, 18, true);
        }

        GPIOC_BASE => {
            // enable the 19th bit of the rcc_ahb_enr_addr
            reg_set_bit(rcc_ahb_enr_addr, 19, true);
        }

        _ => {
            todo!("Implement GPIOx_BASE port! do nothings for ports other than GPIOx_BASE.")
        }
    }
}

pub fn set_gpio_mode_output(port: u32, pin: u32) {
    let gpio_mode_reg_addr = (port + 0x00) as *mut u32;
    let bit_position = pin * 2;
    let mode_value = 0x1;
    reg_set_bits(gpio_mode_reg_addr, mode_value, bit_position, 2);
}

pub fn set_gpio_mode_input(port: u32, pin: u32) {
    let gpio_mode_reg_addr = (port + 0x00) as *mut u32;
    let bit_position = pin * 2;
    let mode_value = 0;

    reg_set_bits(gpio_mode_reg_addr, mode_value, bit_position, 2);
}

pub fn set_gpio_output_type_push_pull(port: u32, pin: u32) {
    let gpio_op_type_reg_addr = (port + 0x04) as *mut u32;
    let bit_position = pin;
    let bit_value = 0;

    reg_set_bits(gpio_op_type_reg_addr, bit_value, bit_position, 2);
}

pub enum PinState {
    Hight,
    Low,
    Toggle,
}

pub fn set_gpio_pin_state(port: u32, pin: u32, state: PinState) {
    let gpio_bsrr_addr = (port + 0x18) as *mut u32;

    match state {
        PinState::Hight => {
            reg_set_val(gpio_bsrr_addr, 1 << pin);
        }
        PinState::Low => {
            reg_set_val(gpio_bsrr_addr, 1 << (pin + 16));
        }
        PinState::Toggle => {
            let gpio_odr_addr = (port + 0x14) as *mut u32;
            if reg_read_bit(gpio_odr_addr, pin) {
                reg_set_val(gpio_bsrr_addr, 1 << (pin + 16));
            } else {
                reg_set_val(gpio_bsrr_addr, 1 << pin);
            }
        }
    }
}

pub fn get_gpio_pin_state(port: u32, pin: u32) -> bool {
    let gpio_bsrr_addr = (port + 0x10) as *mut u32;
    reg_read_bit(gpio_bsrr_addr, pin)
}
