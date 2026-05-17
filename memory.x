ENTRY(Reset)

MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH = 64K
    RAM   : ORIGIN = 0x20000000, LENGTH = 20K
}

SECTIONS
{
    .vector_table ORIGIN(FLASH) :
    {
        KEEP(*(.vector_table))
    } > FLASH

    .text :
    {
        *(.text*)
        *(.rodata*)
    } > FLASH

    .data :
    {
        *(.data*)
    } > RAM AT > FLASH

    .bss :
    {
        *(.bss*)
    } > RAM
}
