SECTIONS
{
  .text : { 
    *(.text .text.* .gnu.linkonce.t.*)
  }
  .rodata : { 
    *(.rodata .rodata.* .gnu.linkonce.r.*)
  }
  .data : { 
    *(.data .data.* .gnu.linkonce.d.*)
  }
  .bss : { 
    *(.bss .bss.* .gnu.linkonce.b.* COMMON)
  }
}