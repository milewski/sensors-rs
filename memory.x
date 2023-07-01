/**
 * memory.x - Linker script for the STM32F103C8T6
 * Flash memory begins at 0x80000000 and has a size of 64kB
 * RAM begins at 0x20000000 and has a size of 20kB
 */
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K
  RAM : ORIGIN = 0x20000000, LENGTH = 20K
}