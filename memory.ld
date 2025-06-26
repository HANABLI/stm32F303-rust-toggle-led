/* memory.ld */

/* ENTRY */
ENTRY(reset_handler)
/* Define memory regions */
MEMORY
{
  FLASH (rx)    : ORIGIN = 0x08000000, LENGTH = 512K
  RAM (rwx)     : ORIGIN = 0x20000000, LENGTH = 64K
  /* EEPROM (rwx)  : ORIGIN = 0x80080000, LENGTH = 4K */
  CCMRAM (rwx)  : ORIGIN = 0x10000000, LENGTH = 16K
  /* BATTRAM (rw)  : ORIGIN = 0x40024000, LENGTH = 4K   /* Battery backend RAM */
}

_start_of_stack = ORIGIN(RAM) + LENGTH(RAM);
_min_stack_size = 0x400;
_min_heap_size  = 0x400;

/*
 * SECTIONS
 * SECTIONS command in a linker script specifies where each section
 * (e.g., .text, .data, .bss) should be placed in memory.
*/  

SECTIONS
{
  /*
  1. Define where each program section is palced in the microcontroller's memory.

  2. The linker assigns addresses based on the SECTIONS command,
     creating an ELF file with exact memory locations.
     for example , if FLASH starts at 0x08000000 and the.text section is placed in the FLASH,
     the linker will assign address starting fom 0x080000000 for the .text section.
  */

  .text :
  {
    /* . = 0X0800_0000 */
    /* Here you collect all executable code. */
    . = ORIGIN(FLASH);
    LONG(_start_of_stack)
    KEEP(*(.isr_vector))
    *(.text)
    *(.text.*) /* . ==> 0X0800_0021 */
    . = ALIGN(4); /* . ==> 0X0800_0024 */
  } > FLASH

  /* Constant data will go into FLASH */
  .rodata :
  {
    . = ALIGN(4);
    *(.rodata)
    *(.rodata*)
    . = ALIGN(4);
  } > FLASH

  /* Initialized data will be in FLASH and will also be in RAM during runtime */
  .data :
  { 
    _sidata = LOADADDR(.data); /* This returns the FLUSH (LMA) */
    . = ALIGN(4);
    _sdata = .; /* start of data section in VMA(RAM) */
    *(.data)
    *(.data*)
    . = ALIGN(4);
    _edata = .;
  } > RAM AT> FLASH

  /* un-initialized data will be RAM */
  .bss :
  {
    . = ALIGN(4);
    _sbss = .; /* start of bss section in RAM (VMA) */
    *(.bss)
    *(.bss*)
    . = ALIGN(4);
    _ebss = .;
  } > RAM

  .ram_usage_check :
  {
    . = ALIGN(8);
    . = . + _min_stack_size;
    . = . + _min_heap_size;
    . = ALIGN(8);
  } > RAM
}

PROVIDE(mem_manager_handler = default_handler);
PROVIDE( busfault_handler = default_handler);
PROVIDE( PendSV_Handler = default_handler);
PROVIDE( SVCall_Handler = default_handler);
PROVIDE( SysTick_Handler = default_handler);
PROVIDE( usagefault_handler = default_handler);
PROVIDE( ADC1_2_Handler = default_handler);
PROVIDE( ADC3_Handler = default_handler);
PROVIDE( ADC4_Handler = default_handler);
PROVIDE( CAN_RX1_Handler = default_handler);
PROVIDE( CAN_SCE_Handler = default_handler);
PROVIDE( COMP123_Handler = default_handler);
PROVIDE( COMP456_Handler = default_handler);
PROVIDE( COMP7_Handler = default_handler);
PROVIDE( DMA1_CH1_Handler = default_handler);
PROVIDE( DMA1_CH2_Handler = default_handler);
PROVIDE( DMA1_CH3_Handler = default_handler);
PROVIDE( DMA1_CH4_Handler = default_handler);
PROVIDE( DMA1_CH5_Handler = default_handler);
PROVIDE( DMA1_CH6_Handler = default_handler);
PROVIDE( DMA1_CH7_Handler = default_handler);
PROVIDE( DMA2_CH1_Handler = default_handler);
PROVIDE( DMA2_CH2_Handler = default_handler);
PROVIDE( DMA2_CH3_Handler = default_handler);
PROVIDE( DMA2_CH4_Handler = default_handler);
PROVIDE( DMA2_CH5_Handler = default_handler);
PROVIDE( EXTI0_Handler = default_handler);
PROVIDE( EXTI15_10_Handler = default_handler);
PROVIDE( EXTI1_Handler = default_handler);
PROVIDE( EXTI2_TSC_Handler = default_handler);
PROVIDE( EXTI3_Handler = default_handler);
PROVIDE( EXTI4_Handler = default_handler);
PROVIDE( EXTI9_5_Handler = default_handler);
PROVIDE( FLASH_Handler = default_handler);
PROVIDE( FMC_Handler = default_handler);
PROVIDE( I2C1_ER_Handler = default_handler);
PROVIDE( I2C1_EV_EXTI23_Handler = default_handler);
PROVIDE( I2C2_ER_Handler = default_handler);
PROVIDE( I2C2_EV_EXTI24_Handler = default_handler);
PROVIDE( I2C3_ER_Handler = default_handler);
PROVIDE( I2C3_EV_Handler = default_handler);
PROVIDE( PVD_Handler = default_handler);
PROVIDE( RCC_Handler = default_handler);
PROVIDE( RTCAlarm_Handler = default_handler);
PROVIDE( RTC_WKUP_Handler = default_handler);
PROVIDE( SPI1_Handler = default_handler);
PROVIDE( SPI2_Handler = default_handler);
PROVIDE( SPI3_Handler = default_handler);
PROVIDE( SPI4_Handler = default_handler);
PROVIDE( TAMP_STAMP_Handler = default_handler);
PROVIDE( TIM1_BRK_TIM15_Handler = default_handler);
PROVIDE( TIM1_CC_Handler = default_handler);
PROVIDE( TIM1_TRG_COM_TIM17_Handler = default_handler);
PROVIDE( TIM1_UP_TIM16_Handler = default_handler);
PROVIDE( TIM20_BRK_Handler = default_handler);
PROVIDE( TIM20_CC_Handler = default_handler);
PROVIDE( TIM20_TRG_COM_Handler = default_handler);
PROVIDE( TIM20_UP_Handler = default_handler);
PROVIDE( TIM2_Handler = default_handler);
PROVIDE( TIM3_Handler = default_handler);
PROVIDE( TIM4_Handler = default_handler);
PROVIDE( TIM6_DACUNDER_Handler = default_handler);
PROVIDE( TIM7_Handler = default_handler);
PROVIDE( TIM8_BRK_Handler = default_handler);
PROVIDE( TIM8_CC_Handler = default_handler);
PROVIDE( TIM8_TRG_COM_Handler = default_handler);
PROVIDE( TIM8_UP_Handler = default_handler);
PROVIDE( UART4_EXTI34_Handler = default_handler);
PROVIDE( UART5_EXTI35_Handler = default_handler);
PROVIDE( USART1_EXTI25_Handler = default_handler);
PROVIDE( USART2_EXTI26_Handler = default_handler);
PROVIDE( USART3_EXTI28_Handler = default_handler);
PROVIDE( USB_HP_CAN_TX_Handler = default_handler);
PROVIDE( USB_HP_Handler = default_handler);
PROVIDE( USB_LP_CAN_RX0_Handler = default_handler);
PROVIDE( USB_LP_Handler = default_handler);
PROVIDE( USB_WKUP_EXTI_Handler = default_handler);
PROVIDE( USB_WKUP_Handler = default_handler);
PROVIDE( WWDG_Handler = default_handler);