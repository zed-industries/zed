SECTIONS
{
  .text : {
    BORINGSSL_bcm_text_start = .;
    *(.text)
    /* gcc puts some code in sections named ".text.unlikely", ".text.exit" and ".text.startup". */
    *(.text.unlikely)
    *(.text.exit)
    *(.text.startup)
    BORINGSSL_bcm_text_end = .;
  }
  .rodata : {
    BORINGSSL_bcm_rodata_start = .;
    *(.rodata)
    *(.rodata.*)
    BORINGSSL_bcm_rodata_end = .;
  }

  /DISCARD/ : {
    /* These sections shouldn't exist. In order to catch any slip-ups, direct
     * the linker to discard them. */
    *(.rela.dyn)
    *(.data)
    *(.rel.ro)
    *(*.text.*)
    *(*.data.*)
  }
}
