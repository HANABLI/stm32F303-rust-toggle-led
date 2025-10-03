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
