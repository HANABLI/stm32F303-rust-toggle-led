#![allow(dead_code)]
pub const GPIOA_BASE: u32 = 0x4800_0000;
pub const GPIOB_BASE: u32 = 0x4800_0400;
pub const GPIOC_BASE: u32 = 0x4800_0800;

pub const RCC_BASE: u32 = 0x4002_1000;
pub const EXTI_BASE: u32 = 0x4001_0400;
pub const SYSCFG_BASE: u32 = 0x4001_0000;

pub const GPIO_PIN_0: u32 = 0;
// pub const GPIO_PIN_1: u32 = 1;
// pub const GPIO_PIN_2: u32 = 2;
// pub const GPIO_PIN_3: u32 = 3;
// pub const GPIO_PIN_4: u32 = 4;
pub const GPIO_PIN_5: u32 = 5;
// pub const GPIO_PIN_6: u32 = 6;
pub const GPIO_PIN_13: u32 = 13;

#[allow(non_camel_case_types)]
pub enum IRQ {
    WWDG = 0,                //Windows watchdog interrupt
    PVD = 1,                 //PVD through EXTI Line16 detection interrupt
    TAMPER_STAMP = 2,        //Tamper and TimeStamp interrupts through EXTI Line 19
    RTC_WKUP = 3,            // RTC wakeup timer interrupt through EXTI line20
    FLASH = 4,               // Flash global interrupt
    RCC = 5,                 // RCC global interrupt
    EXTI0 = 6,               // EXTI Line0 interrupt
    EXTI1 = 7,               // EXTI Line1 interrupt
    EXTI2_TS = 8,            // EXTI Line2 and Touch sensing interrupts
    EXTI3 = 9,               // EXTI Line3
    EXTI4 = 10,              // EXTI Line4
    DMA1_Channel1 = 11,      // DMA1 channel 1 interrupt
    DMA1_Channel2 = 12,      // DMA1 channel 2 interrupt
    DMA1_Channel3 = 13,      // DMA1 channel 3 interrupt
    DMA1_Channel4 = 14,      // DMA1 channel 4 interrupt
    DMA1_Channel5 = 15,      // DMA1 channel 5 interrupt
    DMA1_Channel6 = 16,      // DMA1 channel 6 interrupt
    DMA1_Channel7 = 17,      // DMA1 channel 7 interrupt
    ADC1_2 = 18,             // ADC1 and ADC2 global interrupt
    USB_HP_CAN_TX = 19,      // USB high priority/CAN_TX interrupts
    USB_LP_CAN_RX0 = 20,     // USB low priority/CAN_RX0 interrupts
    CAN_RX1 = 21,            // CAN_RX1 interrupt
    CAN_SCE = 22,            // CAN_SCE interrupt
    EXTI9_5 = 23,            // EXTI Line[9:5] interrupts
    TIM1_BRK_TIM15 = 24,     // TIM1 break/TIM15 global interrupts
    TIM1_UP_TIM16 = 25,      // TIM1 update/TIM16 global interrupts
    TIM1_TRG_COM_TIM17 = 26, // TIM1 trigger and commutation/ TIM17 interrupts
    TIM1_CC = 27,            // TIM1 capture compare interrupt
    TIM2 = 28,               // TIM2 global Interrupt
    TIM3 = 29,               // TIM3 global Interrupt
    TIM4 = 30,               // TIM4 global Interrupt
    I2C1_EV = 31,            // I2C1 Event Interrupt & EXTI Line23 Interrupt
    I2C1_ER = 32,            // I2C1 Error Interrupt
    I2C2_EV = 33,            // I2C2 Event Interrupt & EXTI Line24 Interrupt
    I2C2_ER = 34,            // I2C2 Error Interrupt
    SPI1 = 35,               // SPI1 global Interrupt
    SPI2 = 36,               // SPI2 global Interrupt
    USART1 = 37,             // USART1 global Interrupt & EXTI Line25
    USART2 = 38,             // USART2 global Interrupt & EXTI Line26
    USART3 = 39,             // USART3 global Interrupt & EXTI Line28
    EXTI15_10 = 40,          // External Line[15:10] Interrupts
    RTC_Alarm = 41,          // RTC Alarm (A and B) through EXTI Line 17
    USBWakeUp = 42,          // USB Wakeup Interrupt
    TIM8_BRK = 43,           // TIM8 Break Interrupt
    TIM8_UP = 44,            // TIM8 Update Interrupt
    TIM8_TRG_COM = 45,       // TIM8 Trigger and Commutation Interrupt
    TIM8_CC = 46,            // TIM8 Capture Compare Interrupt
    ADC3 = 47,               // ADC3 global Interrupt
    SPI3 = 51,               // SPI3 global Interrupt
    UART4 = 52,              // UART4 global Interrupt & EXTI Line34
    UART5 = 53,              // UART5 global Interrupt & EXTI Line35
    TIM6_DAC = 54,           // TIM6 global and DAC underrun error Interrupt
    TIM7 = 55,               // TIM7 global Interrupt
    DMA2_Channel1 = 56,      // DMA2 Channel 1 global Interrupt
    DMA2_Channel2 = 57,      // DMA2 Channel 2 global Interrupt
    DMA2_Channel3 = 58,      // DMA2 Channel 3 global Interrupt
    DMA2_Channel4 = 59,      // DMA2 Channel 4 global Interrupt
    DMA2_Channel5 = 60,      // DMA2 Channel 5 global Interrupt
    ADC4 = 61,               // ADC4 global Interrupt
    COMP1_2_3 = 64,          // COMP1, COMP2, COMP3 Interrupts via EXTI Lines
    COMP4_5_6 = 65,          // COMP4, COMP5, COMP6 Interrupts via EXTI Lines
    COMP7 = 66,              // COMP7 global Interrupt via EXTI Line33
    USB_HP = 74,             // USB High Priority global Interrupt
    USB_LP = 75,             // USB Low Priority global Interrupt
    USBWakeUp_RMP = 76,      // USB Wakeup Interrupt remap
    FPU = 81,                // Floating point Interrupt
}

impl IRQ {
    pub fn from_pin(pin: u32) -> Option<u32> {
        match pin {
            0 => Some(IRQ::EXTI0 as u32),
            1 => Some(IRQ::EXTI1 as u32),
            2 => Some(IRQ::EXTI2_TS as u32),
            3 => Some(IRQ::EXTI3 as u32),
            4 => Some(IRQ::EXTI4 as u32),
            5..=9 => Some(IRQ::EXTI9_5 as u32),
            10..=15 => Some(IRQ::EXTI15_10 as u32),
            _ => None,
        }
    }
}
