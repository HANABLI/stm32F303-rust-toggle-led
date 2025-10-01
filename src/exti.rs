#![allow(dead_code)]
use crate::mcu::*;
use crate::reg::*;
pub mod gpio {
    use super::*;

    pub enum EdgeTrigger {
        Rising,
        Falling,
    }

    pub fn set_edge(pin: u32, edge_trigger: EdgeTrigger) {
        let exti_rtsr1_addr = (EXTI_BASE + 0x08) as *mut u32;
        let exti_ftsr1_addr = (EXTI_BASE + 0x0c) as *mut u32;

        match edge_trigger {
            EdgeTrigger::Falling => {
                reg_set_bit(exti_ftsr1_addr, pin, true);
            }

            EdgeTrigger::Rising => {
                reg_set_bit(exti_rtsr1_addr, pin, true);
            }
        }
    }
    ///
    /// SYSCFG external interrupt configuration register 1
    /// (SYSCFG_EXTICR1)
    ///
    pub fn configure_syscfg(port: u32, pin: u32) {
        let mut offset = 0;
        match pin {
            0..=3 => offset = 0x08,
            4..=7 => offset = 0xc,
            8..=11 => offset = 0x10,
            12..=15 => offset = 0x14,
            _ => (),
        }
        let bit_position = (pin % 4) * 4;
        let syscfg_reg_addr = (SYSCFG_BASE + offset) as *mut u32;

        match port {
            GPIOA_BASE => {
                reg_set_bits(syscfg_reg_addr, 0, bit_position, 4);
            }

            GPIOB_BASE => {
                reg_set_bits(syscfg_reg_addr, 1, bit_position, 4);
            }

            GPIOC_BASE => {
                reg_set_bits(syscfg_reg_addr, 2, bit_position, 4);
            }
            //include more match arms realted to other gopio ports like GPIOC, D, E, etc
            _ => (),
        }
    }
}

pub enum ExtiLine {
    Line0 = 0,   //PORTx0
    Line1 = 1,   //PORTx1
    Line2 = 2,   //PORTx2
    Line3 = 3,   //PORTx3
    Line4 = 4,   //PORTx4
    Line5 = 5,   //PORTx5
    Line6 = 6,   //PORTx6
    Line7 = 7,   //PORTx7
    Line8 = 8,   //PORTx8
    Line9 = 9,   //PORTx9
    Line10 = 10, //PORTx10
    Line11 = 11, //PORTx11
    Line12 = 12, //PORTx12
    Line13 = 13, //PORTx13
    Line14 = 14, //PORTx14
    Line15 = 15, //PORTx15
    Line16 = 16, // PVD output
    Line17 = 17, // RTC Alarm Event
    Line18 = 18, // USB Device FS
    Line19 = 19, // RTC tamper and timestamps
    Line20 = 20, // RTC wake-up timer
    Line21 = 21, // Comparator 1 output
    Line22 = 22, // Comparator 2 output
    Line23 = 23, // I2C1 wake-up
    Line24 = 24, // I2C2 wake-up
    Line25 = 25, // USART1 wake-up
    Line26 = 26, // USART2 wake-up
    Line27 = 27, // I2C3
    Line28 = 28, // USART3
    Line29 = 29, // Comparator 3 output
    Line30 = 30, // Comparator 4 output
    Line31 = 31, // Comparator 5 output
    Line32 = 32, // Comparator 6 output
    Line33 = 33, // Comparator 7 output
    Line34 = 34, // UART4 wake-up
    Line35 = 35, // UART5 wake-up
}

impl ExtiLine {
    pub fn from_pin(pin: u32) -> Option<ExtiLine> {
        match pin {
            0 => Some(ExtiLine::Line0),
            1 => Some(ExtiLine::Line1),
            2 => Some(ExtiLine::Line2),
            3 => Some(ExtiLine::Line3),
            4 => Some(ExtiLine::Line4),
            5 => Some(ExtiLine::Line5),
            6 => Some(ExtiLine::Line6),
            7 => Some(ExtiLine::Line7),
            8 => Some(ExtiLine::Line8),
            9 => Some(ExtiLine::Line9),
            10 => Some(ExtiLine::Line10),
            11 => Some(ExtiLine::Line11),
            12 => Some(ExtiLine::Line12),
            13 => Some(ExtiLine::Line13),
            14 => Some(ExtiLine::Line14),
            15 => Some(ExtiLine::Line15),
            _ => None,
        }
    }
}

fn configure_interrupt(exti_line: ExtiLine, is_enable: bool) {
    let exti_imr1_addr = (EXTI_BASE + 0x00) as *mut u32;
    let exti_imr2_addr = (EXTI_BASE + 0x20) as *mut u32;
    let line = exti_line as u32;

    match line {
        0..=31 => {
            reg_set_bit(exti_imr1_addr, line, is_enable);
        }

        32..=35 => {
            reg_set_bit(exti_imr2_addr, line, is_enable);
        }

        _ => {}
    }
}

pub fn enable_interrupt(exti_line: ExtiLine) {
    configure_interrupt(exti_line, true);
}

pub fn disable_interrupt(exti_line: ExtiLine) {
    configure_interrupt(exti_line, false);
}

pub fn clear_pending_interrupt(exti_line: ExtiLine) {
    let exti_per1_addr = (EXTI_BASE + 0x14) as *mut u32;
    let exti_per2_addr = (EXTI_BASE + 0x34) as *mut u32;

    let line = exti_line as u32;

    match line {
        0..=31 => {
            reg_set_bit(exti_per1_addr, line, true);
        }

        32..=33 => {
            reg_set_bit(exti_per2_addr, line, true);
        }

        _ => (),
    }
}
