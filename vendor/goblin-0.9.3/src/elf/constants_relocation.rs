// x86_64 relocations
/// No reloc.
pub const R_X86_64_NONE: u32 = 0;
/// Direct 64 bit.
pub const R_X86_64_64: u32 = 1;
/// PC relative 32 bit signed.
pub const R_X86_64_PC32: u32 = 2;
/// 32 bit GOT entry.
pub const R_X86_64_GOT32: u32 = 3;
/// 32 bit PLT address.
pub const R_X86_64_PLT32: u32 = 4;
/// Copy symbol at runtime.
pub const R_X86_64_COPY: u32 = 5;
/// Create GOT entry.
pub const R_X86_64_GLOB_DAT: u32 = 6;
/// Create PLT entry.
pub const R_X86_64_JUMP_SLOT: u32 = 7;
/// Adjust by program base.
pub const R_X86_64_RELATIVE: u32 = 8;
/// 32 bit signed PC relative offset to GOT.
pub const R_X86_64_GOTPCREL: u32 = 9;
/// Direct 32 bit zero extended.
pub const R_X86_64_32: u32 = 10;
/// Direct 32 bit sign extended.
pub const R_X86_64_32S: u32 = 11;
/// Direct 16 bit zero extended.
pub const R_X86_64_16: u32 = 12;
/// 16 bit sign extended pc relative.
pub const R_X86_64_PC16: u32 = 13;
/// Direct 8 bit sign extended.
pub const R_X86_64_8: u32 = 14;
/// 8 bit sign extended pc relative.
pub const R_X86_64_PC8: u32 = 15;
/// ID of module containing symbol.
pub const R_X86_64_DTPMOD64: u32 = 16;
/// Offset in module's TLS block.
pub const R_X86_64_DTPOFF64: u32 = 17;
/// Offset in initial TLS block.
pub const R_X86_64_TPOFF64: u32 = 18;
/// 32 bit signed PC relative offset to two GOT entries for GD symbol.
pub const R_X86_64_TLSGD: u32 = 19;
/// 32 bit signed PC relative offset to two GOT entries for LD symbol.
pub const R_X86_64_TLSLD: u32 = 20;
/// Offset in TLS block.
pub const R_X86_64_DTPOFF32: u32 = 21;
/// 32 bit signed PC relative offset to GOT entry for IE symbol.
pub const R_X86_64_GOTTPOFF: u32 = 22;
/// Offset in initial TLS block.
pub const R_X86_64_TPOFF32: u32 = 23;
/// PC relative 64 bit.
pub const R_X86_64_PC64: u32 = 24;
/// 64 bit offset to GOT.
pub const R_X86_64_GOTOFF64: u32 = 25;
/// 32 bit signed pc relative offset to GOT.
pub const R_X86_64_GOTPC32: u32 = 26;
/// 64-bit GOT entry offset.
pub const R_X86_64_GOT64: u32 = 27;
/// 64-bit PC relative offset to GOT entry.
pub const R_X86_64_GOTPCREL64: u32 = 28;
/// 64-bit PC relative offset to GOT.
pub const R_X86_64_GOTPC64: u32 = 29;
/// like GOT64, says PLT entry needed.
pub const R_X86_64_GOTPLT64: u32 = 30;
/// 64-bit GOT relative offset to PLT entry.
pub const R_X86_64_PLTOFF64: u32 = 31;
/// Size of symbol plus 32-bit addend.
pub const R_X86_64_SIZE32: u32 = 32;
/// Size of symbol plus 64-bit addend.
pub const R_X86_64_SIZE64: u32 = 33;
/// GOT offset for TLS descriptor..
pub const R_X86_64_GOTPC32_TLSDESC: u32 = 34;
/// Marker for call through TLS descriptor..
pub const R_X86_64_TLSDESC_CALL: u32 = 35;
/// TLS descriptor..
pub const R_X86_64_TLSDESC: u32 = 36;
/// Adjust indirectly by program base.
pub const R_X86_64_IRELATIVE: u32 = 37;
/// 64-bit adjust by program base.
pub const R_X86_64_RELATIVE64: u32 = 38;
///Load from 32 bit signed pc relative offset to GOT entry without REX prefix, relaxable.
pub const R_X86_64_GOTPCRELX: u32 = 41;
/// Load from 32 bit signed pc relative offset to GOT entry with REX prefix, relaxable.
pub const R_X86_64_REX_GOTPCRELX: u32 = 42;
pub const R_X86_64_NUM: u32 = 43;

// Intel 80386 specific definitions

// i386 relocs
/// No reloc
pub const R_386_NONE: u32 = 0;
/// Direct 32 bit
pub const R_386_32: u32 = 1;
/// PC relative 32 bit
pub const R_386_PC32: u32 = 2;
/// 32 bit GOT entry
pub const R_386_GOT32: u32 = 3;
/// 32 bit PLT address
pub const R_386_PLT32: u32 = 4;
/// Copy symbol at runtime
pub const R_386_COPY: u32 = 5;
/// Create GOT entry
pub const R_386_GLOB_DAT: u32 = 6;
/// Create PLT entry
pub const R_386_JMP_SLOT: u32 = 7;
/// Adjust by program base
pub const R_386_RELATIVE: u32 = 8;
/// 32 bit offset to GOT
pub const R_386_GOTOFF: u32 = 9;
/// 32 bit PC relative offset to GOT
pub const R_386_GOTPC: u32 = 10;
pub const R_386_32PLT: u32 = 11;
/// Offset in static TLS block
pub const R_386_TLS_TPOFF: u32 = 14;
/// Address of GOT entry for static TLS block offset
pub const R_386_TLS_IE: u32 = 15;
/// GOT entry for static TLS block offset
pub const R_386_TLS_GOTIE: u32 = 16;
/// Offset relative to static TLS block
pub const R_386_TLS_LE: u32 = 17;
/// Direct 32 bit for GNU version of general dynamic thread local data
pub const R_386_TLS_GD: u32 = 18;
/// Direct 32 bit for GNU version of local dynamic thread local data in LE code
pub const R_386_TLS_LDM: u32 = 19;
pub const R_386_16: u32 = 20;
pub const R_386_PC16: u32 = 21;
pub const R_386_8: u32 = 22;
pub const R_386_PC8: u32 = 23;
/// Direct 32 bit for general dynamic thread local data
pub const R_386_TLS_GD_32: u32 = 24;
/// Tag for pushl in GD TLS code
pub const R_386_TLS_GD_PUSH: u32 = 25;
/// Relocation for call to __tls_get_addr()
pub const R_386_TLS_GD_CALL: u32 = 26;
/// Tag for popl in GD TLS code
pub const R_386_TLS_GD_POP: u32 = 27;
/// Direct 32 bit for local dynamic thread local data in LE code
pub const R_386_TLS_LDM_32: u32 = 28;
/// Tag for pushl in LDM TLS code
pub const R_386_TLS_LDM_PUSH: u32 = 29;
/// Relocation for call to __tls_get_addr() in LDM code
pub const R_386_TLS_LDM_CALL: u32 = 30;
/// Tag for popl in LDM TLS code
pub const R_386_TLS_LDM_POP: u32 = 31;
/// Offset relative to TLS block
pub const R_386_TLS_LDO_32: u32 = 32;
/// GOT entry for negated static TLS block offset
pub const R_386_TLS_IE_32: u32 = 33;
/// Negated offset relative to static TLS block
pub const R_386_TLS_LE_32: u32 = 34;
/// ID of module containing symbol
pub const R_386_TLS_DTPMOD32: u32 = 35;
/// Offset in TLS block
pub const R_386_TLS_DTPOFF32: u32 = 36;
/// Negated offset in static TLS block
pub const R_386_TLS_TPOFF32: u32 = 37;
/// 32-bit symbol size
pub const R_386_SIZE32: u32 = 38;
/// GOT offset for TLS descriptor.
pub const R_386_TLS_GOTDESC: u32 = 39;
/// Marker of call through TLS descriptor for relaxation
pub const R_386_TLS_DESC_CALL: u32 = 40;
/// TLS descriptor containing pointer to code and to argument, returning the TLS offset for the symbol
pub const R_386_TLS_DESC: u32 = 41;
/// Adjust indirectly by program base
pub const R_386_IRELATIVE: u32 = 42;
/// Load from 32 bit GOT entry, relaxable
pub const R_386_GOT32X: u32 = 43;
/// Keep this the last entry
pub const R_386_NUM: u32 = 44;

// AArch64 relocs
/// No relocation
pub const R_AARCH64_NONE: u32 = 0;

// ILP32 AArch64 relocs
/// Direct 32 bit
pub const R_AARCH64_P32_ABS32: u32 = 1;
/// Copy symbol at runtime
pub const R_AARCH64_P32_COPY: u32 = 180;
/// Create GOT entry
pub const R_AARCH64_P32_GLOB_DAT: u32 = 181;
/// Create PLT entry
pub const R_AARCH64_P32_JUMP_SLOT: u32 = 182;
/// Adjust by program base
pub const R_AARCH64_P32_RELATIVE: u32 = 183;
/// Module number, 32 bit
pub const R_AARCH64_P32_TLS_DTPMOD: u32 = 184;
/// Module-relative offset, 32 bit
pub const R_AARCH64_P32_TLS_DTPREL: u32 = 185;
/// TP-relative offset, 32 bit
pub const R_AARCH64_P32_TLS_TPREL: u32 = 186;
/// TLS Descriptor
pub const R_AARCH64_P32_TLSDESC: u32 = 187;
/// STT_GNU_IFUNC relocation
pub const R_AARCH64_P32_IRELATIVE: u32 = 188;

// LP64 AArch64 relocs
/// Direct 64 bit
pub const R_AARCH64_ABS64: u32 = 257;
/// Direct 32 bit
pub const R_AARCH64_ABS32: u32 = 258;
/// Direct 16-bit
pub const R_AARCH64_ABS16: u32 = 259;
/// PC-relative 64-bit
pub const R_AARCH64_PREL64: u32 = 260;
/// PC-relative 32-bit
pub const R_AARCH64_PREL32: u32 = 261;
/// PC-relative 16-bit
pub const R_AARCH64_PREL16: u32 = 262;
/// Dir. MOVZ imm. from bits 15:0
pub const R_AARCH64_MOVW_UABS_G0: u32 = 263;
/// Likewise for MOVK; no check
pub const R_AARCH64_MOVW_UABS_G0_NC: u32 = 264;
/// Dir. MOVZ imm. from bits 31:16
pub const R_AARCH64_MOVW_UABS_G1: u32 = 265;
/// Likewise for MOVK; no check
pub const R_AARCH64_MOVW_UABS_G1_NC: u32 = 266;
/// Dir. MOVZ imm. from bits 47:32
pub const R_AARCH64_MOVW_UABS_G2: u32 = 267;
/// Likewise for MOVK; no check
pub const R_AARCH64_MOVW_UABS_G2_NC: u32 = 268;
/// Dir. MOV{K,Z} imm. from 63:48
pub const R_AARCH64_MOVW_UABS_G3: u32 = 269;
/// Dir. MOV{N,Z} imm. from 15:0
pub const R_AARCH64_MOVW_SABS_G0: u32 = 270;
/// Dir. MOV{N,Z} imm. from 31:16
pub const R_AARCH64_MOVW_SABS_G1: u32 = 271;
/// Dir. MOV{N,Z} imm. from 47:32
pub const R_AARCH64_MOVW_SABS_G2: u32 = 272;
/// PC-rel. LD imm. from bits 20:2
pub const R_AARCH64_LD_PREL_LO19: u32 = 273;
/// PC-rel. ADR imm. from bits 20:0
pub const R_AARCH64_ADR_PREL_LO21: u32 = 274;
/// Page-rel. ADRP imm. from 32:12
pub const R_AARCH64_ADR_PREL_PG_HI21: u32 = 275;
/// Likewise; no overflow check
pub const R_AARCH64_ADR_PREL_PG_HI21_NC: u32 = 276;
/// Dir. ADD imm. from bits 11:0
pub const R_AARCH64_ADD_ABS_LO12_NC: u32 = 277;
/// Likewise for LD/ST; no check.
pub const R_AARCH64_LDST8_ABS_LO12_NC: u32 = 278;
/// PC-rel. TBZ/TBNZ imm. from 15:2
pub const R_AARCH64_TSTBR14: u32 = 279;
/// PC-rel. cond. br. imm. from 20:2.
pub const R_AARCH64_CONDBR19: u32 = 280;
/// PC-rel. B imm. from bits 27:2
pub const R_AARCH64_JUMP26: u32 = 282;
/// Likewise for CALL
pub const R_AARCH64_CALL26: u32 = 283;
/// Dir. ADD imm. from bits 11:1
pub const R_AARCH64_LDST16_ABS_LO12_NC: u32 = 284;
/// Likewise for bits 11:2
pub const R_AARCH64_LDST32_ABS_LO12_NC: u32 = 285;
/// Likewise for bits 11:3
pub const R_AARCH64_LDST64_ABS_LO12_NC: u32 = 286;
/// PC-rel. MOV{N,Z} imm. from 15:0
pub const R_AARCH64_MOVW_PREL_G0: u32 = 287;
/// Likewise for MOVK; no check
pub const R_AARCH64_MOVW_PREL_G0_NC: u32 = 288;
/// PC-rel. MOV{N,Z} imm. from 31:16.
pub const R_AARCH64_MOVW_PREL_G1: u32 = 289;
/// Likewise for MOVK; no check
pub const R_AARCH64_MOVW_PREL_G1_NC: u32 = 290;
/// PC-rel. MOV{N,Z} imm. from 47:32.
pub const R_AARCH64_MOVW_PREL_G2: u32 = 291;
/// Likewise for MOVK; no check
pub const R_AARCH64_MOVW_PREL_G2_NC: u32 = 292;
/// PC-rel. MOV{N,Z} imm. from 63:48.
pub const R_AARCH64_MOVW_PREL_G3: u32 = 293;
/// Dir. ADD imm. from bits 11:4
pub const R_AARCH64_LDST128_ABS_LO12_NC: u32 = 299;
/// GOT-rel. off. MOV{N,Z} imm. 15:0.
pub const R_AARCH64_MOVW_GOTOFF_G0: u32 = 300;
/// Likewise for MOVK; no check
pub const R_AARCH64_MOVW_GOTOFF_G0_NC: u32 = 301;
/// GOT-rel. o. MOV{N,Z} imm. 31:16
pub const R_AARCH64_MOVW_GOTOFF_G1: u32 = 302;
/// Likewise for MOVK; no check
pub const R_AARCH64_MOVW_GOTOFF_G1_NC: u32 = 303;
/// GOT-rel. o. MOV{N,Z} imm. 47:32
pub const R_AARCH64_MOVW_GOTOFF_G2: u32 = 304;
/// Likewise for MOVK; no check
pub const R_AARCH64_MOVW_GOTOFF_G2_NC: u32 = 305;
/// GOT-rel. o. MOV{N,Z} imm. 63:48
pub const R_AARCH64_MOVW_GOTOFF_G3: u32 = 306;
/// GOT-relative 64-bit
pub const R_AARCH64_GOTREL64: u32 = 307;
/// GOT-relative 32-bit
pub const R_AARCH64_GOTREL32: u32 = 308;
/// PC-rel. GOT off. load imm. 20:2
pub const R_AARCH64_GOT_LD_PREL19: u32 = 309;
/// GOT-rel. off. LD/ST imm. 14:3
pub const R_AARCH64_LD64_GOTOFF_LO15: u32 = 310;
/// P-page-rel. GOT off. ADRP 32:12
pub const R_AARCH64_ADR_GOT_PAGE: u32 = 311;
/// Dir. GOT off. LD/ST imm. 11:3
pub const R_AARCH64_LD64_GOT_LO12_NC: u32 = 312;
/// GOT-page-rel. GOT off. LD/ST 14:3
pub const R_AARCH64_LD64_GOTPAGE_LO15: u32 = 313;
/// PC-relative ADR imm. 20:0
pub const R_AARCH64_TLSGD_ADR_PREL21: u32 = 512;
/// page-rel. ADRP imm. 32:12
pub const R_AARCH64_TLSGD_ADR_PAGE21: u32 = 513;
/// direct ADD imm. from 11:0
pub const R_AARCH64_TLSGD_ADD_LO12_NC: u32 = 514;
/// GOT-rel. MOV{N,Z} 31:16
pub const R_AARCH64_TLSGD_MOVW_G1: u32 = 515;
/// GOT-rel. MOVK imm. 15:0
pub const R_AARCH64_TLSGD_MOVW_G0_NC: u32 = 516;
/// Like 512; local dynamic model
pub const R_AARCH64_TLSLD_ADR_PREL21: u32 = 517;
/// Like 513; local dynamic model
pub const R_AARCH64_TLSLD_ADR_PAGE21: u32 = 518;
/// Like 514; local dynamic model
pub const R_AARCH64_TLSLD_ADD_LO12_NC: u32 = 519;
/// Like 515; local dynamic model
pub const R_AARCH64_TLSLD_MOVW_G1: u32 = 520;
/// Like 516; local dynamic model
pub const R_AARCH64_TLSLD_MOVW_G0_NC: u32 = 521;
/// TLS PC-rel. load imm. 20:2
pub const R_AARCH64_TLSLD_LD_PREL19: u32 = 522;
/// TLS DTP-rel. MOV{N,Z} 47:32
pub const R_AARCH64_TLSLD_MOVW_DTPREL_G2: u32 = 523;
/// TLS DTP-rel. MOV{N,Z} 31:16
pub const R_AARCH64_TLSLD_MOVW_DTPREL_G1: u32 = 524;
/// Likewise; MOVK; no check
pub const R_AARCH64_TLSLD_MOVW_DTPREL_G1_NC: u32 = 525;
/// TLS DTP-rel. MOV{N,Z} 15:0
pub const R_AARCH64_TLSLD_MOVW_DTPREL_G0: u32 = 526;
/// Likewise; MOVK; no check
pub const R_AARCH64_TLSLD_MOVW_DTPREL_G0_NC: u32 = 527;
/// DTP-rel. ADD imm. from 23:12.
pub const R_AARCH64_TLSLD_ADD_DTPREL_HI12: u32 = 528;
/// DTP-rel. ADD imm. from 11:0
pub const R_AARCH64_TLSLD_ADD_DTPREL_LO12: u32 = 529;
/// Likewise; no ovfl. check
pub const R_AARCH64_TLSLD_ADD_DTPREL_LO12_NC: u32 = 530;
/// DTP-rel. LD/ST imm. 11:0
pub const R_AARCH64_TLSLD_LDST8_DTPREL_LO12: u32 = 531;
/// Likewise; no check
pub const R_AARCH64_TLSLD_LDST8_DTPREL_LO12_NC: u32 = 532;
/// DTP-rel. LD/ST imm. 11:1
pub const R_AARCH64_TLSLD_LDST16_DTPREL_LO12: u32 = 533;
/// Likewise; no check
pub const R_AARCH64_TLSLD_LDST16_DTPREL_LO12_NC: u32 = 534;
/// DTP-rel. LD/ST imm. 11:2
pub const R_AARCH64_TLSLD_LDST32_DTPREL_LO12: u32 = 535;
/// Likewise; no check
pub const R_AARCH64_TLSLD_LDST32_DTPREL_LO12_NC: u32 = 536;
/// DTP-rel. LD/ST imm. 11:3
pub const R_AARCH64_TLSLD_LDST64_DTPREL_LO12: u32 = 537;
/// Likewise; no check
pub const R_AARCH64_TLSLD_LDST64_DTPREL_LO12_NC: u32 = 538;
/// GOT-rel. MOV{N,Z} 31:16
pub const R_AARCH64_TLSIE_MOVW_GOTTPREL_G1: u32 = 539;
/// GOT-rel. MOVK 15:0
pub const R_AARCH64_TLSIE_MOVW_GOTTPREL_G0_NC: u32 = 540;
/// Page-rel. ADRP 32:12
pub const R_AARCH64_TLSIE_ADR_GOTTPREL_PAGE21: u32 = 541;
/// Direct LD off. 11:3
pub const R_AARCH64_TLSIE_LD64_GOTTPREL_LO12_NC: u32 = 542;
/// PC-rel. load imm. 20:2
pub const R_AARCH64_TLSIE_LD_GOTTPREL_PREL19: u32 = 543;
/// TLS TP-rel. MOV{N,Z} 47:32
pub const R_AARCH64_TLSLE_MOVW_TPREL_G2: u32 = 544;
/// TLS TP-rel. MOV{N,Z} 31:16
pub const R_AARCH64_TLSLE_MOVW_TPREL_G1: u32 = 545;
/// Likewise; MOVK; no check
pub const R_AARCH64_TLSLE_MOVW_TPREL_G1_NC: u32 = 546;
/// TLS TP-rel. MOV{N,Z} 15:0
pub const R_AARCH64_TLSLE_MOVW_TPREL_G0: u32 = 547;
/// Likewise; MOVK; no check
pub const R_AARCH64_TLSLE_MOVW_TPREL_G0_NC: u32 = 548;
/// TP-rel. ADD imm. 23:12
pub const R_AARCH64_TLSLE_ADD_TPREL_HI12: u32 = 549;
/// TP-rel. ADD imm. 11:0
pub const R_AARCH64_TLSLE_ADD_TPREL_LO12: u32 = 550;
/// Likewise; no ovfl. check
pub const R_AARCH64_TLSLE_ADD_TPREL_LO12_NC: u32 = 551;
/// TP-rel. LD/ST off. 11:0
pub const R_AARCH64_TLSLE_LDST8_TPREL_LO12: u32 = 552;
/// Likewise; no ovfl. check.
pub const R_AARCH64_TLSLE_LDST8_TPREL_LO12_NC: u32 = 553;
/// TP-rel. LD/ST off. 11:1
pub const R_AARCH64_TLSLE_LDST16_TPREL_LO12: u32 = 554;
/// Likewise; no check
pub const R_AARCH64_TLSLE_LDST16_TPREL_LO12_NC: u32 = 555;
/// TP-rel. LD/ST off. 11:2
pub const R_AARCH64_TLSLE_LDST32_TPREL_LO12: u32 = 556;
/// Likewise; no check
pub const R_AARCH64_TLSLE_LDST32_TPREL_LO12_NC: u32 = 557;
/// TP-rel. LD/ST off. 11:3
pub const R_AARCH64_TLSLE_LDST64_TPREL_LO12: u32 = 558;
/// Likewise; no check
pub const R_AARCH64_TLSLE_LDST64_TPREL_LO12_NC: u32 = 559;
/// PC-rel. load immediate 20:2
pub const R_AARCH64_TLSDESC_LD_PREL19: u32 = 560;
/// PC-rel. ADR immediate 20:0
pub const R_AARCH64_TLSDESC_ADR_PREL21: u32 = 561;
/// Page-rel. ADRP imm. 32:12
pub const R_AARCH64_TLSDESC_ADR_PAGE21: u32 = 562;
/// Direct LD off. from 11:3
pub const R_AARCH64_TLSDESC_LD64_LO12: u32 = 563;
/// Direct ADD imm. from 11:0
pub const R_AARCH64_TLSDESC_ADD_LO12: u32 = 564;
/// GOT-rel. MOV{N,Z} imm. 31:16
pub const R_AARCH64_TLSDESC_OFF_G1: u32 = 565;
/// GOT-rel. MOVK imm. 15:0; no ck
pub const R_AARCH64_TLSDESC_OFF_G0_NC: u32 = 566;
/// Relax LDR
pub const R_AARCH64_TLSDESC_LDR: u32 = 567;
/// Relax ADD
pub const R_AARCH64_TLSDESC_ADD: u32 = 568;
/// Relax BLR
pub const R_AARCH64_TLSDESC_CALL: u32 = 569;
/// TP-rel. LD/ST off. 11:4
pub const R_AARCH64_TLSLE_LDST128_TPREL_LO12: u32 = 570;
/// Likewise; no check
pub const R_AARCH64_TLSLE_LDST128_TPREL_LO12_NC: u32 = 571;
/// DTP-rel. LD/ST imm. 11:4.
pub const R_AARCH64_TLSLD_LDST128_DTPREL_LO12: u32 = 572;
/// Likewise; no check
pub const R_AARCH64_TLSLD_LDST128_DTPREL_LO12_NC: u32 = 573;
/// Copy symbol at runtime
pub const R_AARCH64_COPY: u32 = 1024;
/// Create GOT entry
pub const R_AARCH64_GLOB_DAT: u32 = 1025;
/// Create PLT entry
pub const R_AARCH64_JUMP_SLOT: u32 = 1026;
/// Adjust by program base
pub const R_AARCH64_RELATIVE: u32 = 1027;
/// Module number, 64 bit
pub const R_AARCH64_TLS_DTPMOD: u32 = 1028;
/// Module-relative offset, 64 bit
pub const R_AARCH64_TLS_DTPREL: u32 = 1029;
/// TP-relative offset, 64 bit
pub const R_AARCH64_TLS_TPREL: u32 = 1030;
/// TLS Descriptor
pub const R_AARCH64_TLSDESC: u32 = 1031;
/// STT_GNU_IFUNC relocation
pub const R_AARCH64_IRELATIVE: u32 = 1032;

// ARM relocs
/// No reloc
pub const R_ARM_NONE: u32 = 0;
/// Deprecated PC relative 26 bit branch
pub const R_ARM_PC24: u32 = 1;
/// Direct 32 bit
pub const R_ARM_ABS32: u32 = 2;
/// PC relative 32 bit
pub const R_ARM_REL32: u32 = 3;
pub const R_ARM_PC13: u32 = 4;
/// Direct 16 bit
pub const R_ARM_ABS16: u32 = 5;
/// Direct 12 bit
pub const R_ARM_ABS12: u32 = 6;
/// Direct & 0x7C (LDR, STR)
pub const R_ARM_THM_ABS5: u32 = 7;
/// Direct 8 bit
pub const R_ARM_ABS8: u32 = 8;
pub const R_ARM_SBREL32: u32 = 9;
/// PC relative 24 bit (Thumb32 BL)
pub const R_ARM_THM_PC22: u32 = 10;
/// PC relative & 0x3FC(Thumb16 LDR, ADD, ADR).
pub const R_ARM_THM_PC8: u32 = 11;
pub const R_ARM_AMP_VCALL9: u32 = 12;
/// Obsolete static relocation
pub const R_ARM_SWI24: u32 = 13;
/// Dynamic relocation
pub const R_ARM_TLS_DESC: u32 = 13;
/// Reserved
pub const R_ARM_THM_SWI8: u32 = 14;
/// Reserved
pub const R_ARM_XPC25: u32 = 15;
/// Reserved
pub const R_ARM_THM_XPC22: u32 = 16;
/// ID of module containing symbol
pub const R_ARM_TLS_DTPMOD32: u32 = 17;
/// Offset in TLS block
pub const R_ARM_TLS_DTPOFF32: u32 = 18;
/// Offset in static TLS block
pub const R_ARM_TLS_TPOFF32: u32 = 19;
/// Copy symbol at runtime
pub const R_ARM_COPY: u32 = 20;
/// Create GOT entry
pub const R_ARM_GLOB_DAT: u32 = 21;
/// Create PLT entry
pub const R_ARM_JUMP_SLOT: u32 = 22;
/// Adjust by program base
pub const R_ARM_RELATIVE: u32 = 23;
/// 32 bit offset to GOT
pub const R_ARM_GOTOFF: u32 = 24;
/// 32 bit PC relative offset to GOT
pub const R_ARM_GOTPC: u32 = 25;
/// 32 bit GOT entry
pub const R_ARM_GOT32: u32 = 26;
/// Deprecated, 32 bit PLT address
pub const R_ARM_PLT32: u32 = 27;
/// PC relative 24 bit (BL, BLX)
pub const R_ARM_CALL: u32 = 28;
/// PC relative 24 bit (B, BL&lt;cond&gt;)
pub const R_ARM_JUMP24: u32 = 29;
/// PC relative 24 bit (Thumb32 B.W)
pub const R_ARM_THM_JUMP24: u32 = 30;
/// Adjust by program base
pub const R_ARM_BASE_ABS: u32 = 31;
/// Obsolete
pub const R_ARM_ALU_PCREL_7_0: u32 = 32;
/// Obsolete
pub const R_ARM_ALU_PCREL_15_8: u32 = 33;
/// Obsolete
pub const R_ARM_ALU_PCREL_23_15: u32 = 34;
/// Deprecated, prog. base relative
pub const R_ARM_LDR_SBREL_11_0: u32 = 35;
/// Deprecated, prog. base relative
pub const R_ARM_ALU_SBREL_19_12: u32 = 36;
/// Deprecated, prog. base relative
pub const R_ARM_ALU_SBREL_27_20: u32 = 37;
pub const R_ARM_TARGET1: u32 = 38;
/// Program base relative
pub const R_ARM_SBREL31: u32 = 39;
pub const R_ARM_V4BX: u32 = 40;
pub const R_ARM_TARGET2: u32 = 41;
/// 32 bit PC relative
pub const R_ARM_PREL31: u32 = 42;
/// Direct 16-bit (MOVW)
pub const R_ARM_MOVW_ABS_NC: u32 = 43;
/// Direct high 16-bit (MOVT)
pub const R_ARM_MOVT_ABS: u32 = 44;
/// PC relative 16-bit (MOVW)
pub const R_ARM_MOVW_PREL_NC: u32 = 45;
/// PC relative (MOVT)
pub const R_ARM_MOVT_PREL: u32 = 46;
/// Direct 16 bit (Thumb32 MOVW)
pub const R_ARM_THM_MOVW_ABS_NC: u32 = 47;
/// Direct high 16 bit (Thumb32 MOVT)
pub const R_ARM_THM_MOVT_ABS: u32 = 48;
/// PC relative 16 bit (Thumb32 MOVW)
pub const R_ARM_THM_MOVW_PREL_NC: u32 = 49;
/// PC relative high 16 bit (Thumb32 MOVT)
pub const R_ARM_THM_MOVT_PREL: u32 = 50;
/// PC relative 20 bit (Thumb32 B&lt;cond&gt;.W)
pub const R_ARM_THM_JUMP19: u32 = 51;
/// PC relative X & 0x7E (Thumb16 CBZ, CBNZ)
pub const R_ARM_THM_JUMP6: u32 = 52;
/// PC relative 12 bit (Thumb32 ADR.W)
pub const R_ARM_THM_ALU_PREL_11_0: u32 = 53;
/// PC relative 12 bit (Thumb32 LDR{D,SB,H,SH})
pub const R_ARM_THM_PC12: u32 = 54;
/// Direct 32-bit
pub const R_ARM_ABS32_NOI: u32 = 55;
/// PC relative 32-bit
pub const R_ARM_REL32_NOI: u32 = 56;
/// PC relative (ADD, SUB)
pub const R_ARM_ALU_PC_G0_NC: u32 = 57;
/// PC relative (ADD, SUB)
pub const R_ARM_ALU_PC_G0: u32 = 58;
/// PC relative (ADD, SUB)
pub const R_ARM_ALU_PC_G1_NC: u32 = 59;
/// PC relative (ADD, SUB)
pub const R_ARM_ALU_PC_G1: u32 = 60;
/// PC relative (ADD, SUB)
pub const R_ARM_ALU_PC_G2: u32 = 61;
/// PC relative (LDR,STR,LDRB,STRB)
pub const R_ARM_LDR_PC_G1: u32 = 62;
/// PC relative (LDR,STR,LDRB,STRB)
pub const R_ARM_LDR_PC_G2: u32 = 63;
/// PC relative (STR{D,H},LDR{D,SB,H,SH})
pub const R_ARM_LDRS_PC_G0: u32 = 64;
/// PC relative (STR{D,H},LDR{D,SB,H,SH})
pub const R_ARM_LDRS_PC_G1: u32 = 65;
/// PC relative (STR{D,H},LDR{D,SB,H,SH})
pub const R_ARM_LDRS_PC_G2: u32 = 66;
/// PC relative (LDC, STC)
pub const R_ARM_LDC_PC_G0: u32 = 67;
/// PC relative (LDC, STC)
pub const R_ARM_LDC_PC_G1: u32 = 68;
/// PC relative (LDC, STC)
pub const R_ARM_LDC_PC_G2: u32 = 69;
/// Program base relative (ADD,SUB)
pub const R_ARM_ALU_SB_G0_NC: u32 = 70;
/// Program base relative (ADD,SUB)
pub const R_ARM_ALU_SB_G0: u32 = 71;
/// Program base relative (ADD,SUB)
pub const R_ARM_ALU_SB_G1_NC: u32 = 72;
/// Program base relative (ADD,SUB)
pub const R_ARM_ALU_SB_G1: u32 = 73;
/// Program base relative (ADD,SUB)
pub const R_ARM_ALU_SB_G2: u32 = 74;
/// Program base relative (LDR,STR, LDRB, STRB)
pub const R_ARM_LDR_SB_G0: u32 = 75;
/// Program base relative (LDR, STR, LDRB, STRB)
pub const R_ARM_LDR_SB_G1: u32 = 76;
/// Program base relative (LDR, STR, LDRB, STRB)
pub const R_ARM_LDR_SB_G2: u32 = 77;
/// Program base relative (LDR, STR, LDRB, STRB)
pub const R_ARM_LDRS_SB_G0: u32 = 78;
/// Program base relative (LDR, STR, LDRB, STRB)
pub const R_ARM_LDRS_SB_G1: u32 = 79;
/// Program base relative (LDR, STR, LDRB, STRB)
pub const R_ARM_LDRS_SB_G2: u32 = 80;
/// Program base relative (LDC,STC)
pub const R_ARM_LDC_SB_G0: u32 = 81;
/// Program base relative (LDC,STC)
pub const R_ARM_LDC_SB_G1: u32 = 82;
/// Program base relative (LDC,STC)
pub const R_ARM_LDC_SB_G2: u32 = 83;
/// Program base relative 16 bit (MOVW)
pub const R_ARM_MOVW_BREL_NC: u32 = 84;
/// Program base relative high 16 bit (MOVT)
pub const R_ARM_MOVT_BREL: u32 = 85;
/// Program base relative 16 bit (MOVW)
pub const R_ARM_MOVW_BREL: u32 = 86;
/// Program base relative 16 bit (Thumb32 MOVW)
pub const R_ARM_THM_MOVW_BREL_NC: u32 = 87;
/// Program base relative high 16 bit (Thumb32 MOVT)
pub const R_ARM_THM_MOVT_BREL: u32 = 88;
/// Program base relative 16 bit (Thumb32 MOVW)
pub const R_ARM_THM_MOVW_BREL: u32 = 89;
pub const R_ARM_TLS_GOTDESC: u32 = 90;
pub const R_ARM_TLS_CALL: u32 = 91;
/// TLS relaxation
pub const R_ARM_TLS_DESCSEQ: u32 = 92;
pub const R_ARM_THM_TLS_CALL: u32 = 93;
pub const R_ARM_PLT32_ABS: u32 = 94;
/// GOT entry
pub const R_ARM_GOT_ABS: u32 = 95;
/// PC relative GOT entry
pub const R_ARM_GOT_PREL: u32 = 96;
/// GOT entry relative to GOT origin (LDR)
pub const R_ARM_GOT_BREL12: u32 = 97;
/// 12 bit, GOT entry relative to GOT origin (LDR, STR)
pub const R_ARM_GOTOFF12: u32 = 98;
pub const R_ARM_GOTRELAX: u32 = 99;
pub const R_ARM_GNU_VTENTRY: u32 = 100;
pub const R_ARM_GNU_VTINHERIT: u32 = 101;
/// PC relative & 0xFFE (Thumb16 B)
pub const R_ARM_THM_PC11: u32 = 102;
/// PC relative & 0x1FE (Thumb16 B/B&lt;cond&gt;)
pub const R_ARM_THM_PC9: u32 = 103;
/// PC-rel 32 bit for global dynamic thread local data
pub const R_ARM_TLS_GD32: u32 = 104;
/// PC-rel 32 bit for local dynamic thread local data
pub const R_ARM_TLS_LDM32: u32 = 105;
/// 32 bit offset relative to TLS block
pub const R_ARM_TLS_LDO32: u32 = 106;
/// PC-rel 32 bit for GOT entry of static TLS block offset
pub const R_ARM_TLS_IE32: u32 = 107;
/// 32 bit offset relative to static TLS block
pub const R_ARM_TLS_LE32: u32 = 108;
/// 12 bit relative to TLS block (LDR, STR)
pub const R_ARM_TLS_LDO12: u32 = 109;
/// 12 bit relative to static TLS block (LDR, STR)
pub const R_ARM_TLS_LE12: u32 = 110;
/// 12 bit GOT entry relative to GOT origin (LDR)
pub const R_ARM_TLS_IE12GP: u32 = 111;
/// Obsolete
pub const R_ARM_ME_TOO: u32 = 128;
pub const R_ARM_THM_TLS_DESCSEQ: u32 = 129;
pub const R_ARM_THM_TLS_DESCSEQ16: u32 = 129;
pub const R_ARM_THM_TLS_DESCSEQ32: u32 = 130;
/// GOT entry relative to GOT origin, 12 bit (Thumb32 LDR)
pub const R_ARM_THM_GOT_BREL12: u32 = 131;
pub const R_ARM_IRELATIVE: u32 = 160;
pub const R_ARM_RXPC25: u32 = 249;
pub const R_ARM_RSBREL32: u32 = 250;
pub const R_ARM_THM_RPC22: u32 = 251;
pub const R_ARM_RREL32: u32 = 252;
pub const R_ARM_RABS22: u32 = 253;
pub const R_ARM_RPC24: u32 = 254;
pub const R_ARM_RBASE: u32 = 255;
/// Keep this the last entry
pub const R_ARM_NUM: u32 = 256;

///////////////////
// OpenRisc
///////////////////
pub const R_OR1K_NONE: u32 = 0;
pub const R_OR1K_32: u32 = 1;
pub const R_OR1K_16: u32 = 2;
pub const R_OR1K_8: u32 = 3;
pub const R_OR1K_LO_16_IN_INSN: u32 = 4;
pub const R_OR1K_HI_16_IN_INSN: u32 = 5;
pub const R_OR1K_INSN_REL_26: u32 = 6;
pub const R_OR1K_GNU_VTENTRY: u32 = 7;
pub const R_OR1K_GNU_VTINHERIT: u32 = 8;
pub const R_OR1K_32_PCREL: u32 = 9;
pub const R_OR1K_16_PCREL: u32 = 10;
pub const R_OR1K_8_PCREL: u32 = 11;
pub const R_OR1K_GOTPC_HI16: u32 = 12;
pub const R_OR1K_GOTPC_LO16: u32 = 13;
pub const R_OR1K_GOT16: u32 = 14;
pub const R_OR1K_PLT26: u32 = 15;
pub const R_OR1K_GOTOFF_HI16: u32 = 16;
pub const R_OR1K_GOTOFF_LO16: u32 = 17;
pub const R_OR1K_COPY: u32 = 18;
pub const R_OR1K_GLOB_DAT: u32 = 19;
pub const R_OR1K_JMP_SLOT: u32 = 20;
pub const R_OR1K_RELATIVE: u32 = 21;
pub const R_OR1K_TLS_GD_HI16: u32 = 22;
pub const R_OR1K_TLS_GD_LO16: u32 = 23;
pub const R_OR1K_TLS_LDM_HI16: u32 = 24;
pub const R_OR1K_TLS_LDM_LO16: u32 = 25;
pub const R_OR1K_TLS_LDO_HI16: u32 = 26;
pub const R_OR1K_TLS_LDO_LO16: u32 = 27;
pub const R_OR1K_TLS_IE_HI16: u32 = 28;
pub const R_OR1K_TLS_IE_LO16: u32 = 29;
pub const R_OR1K_TLS_LE_HI16: u32 = 30;
pub const R_OR1K_TLS_LE_LO16: u32 = 31;
pub const R_OR1K_TLS_TPOFF: u32 = 32;
pub const R_OR1K_TLS_DTPOFF: u32 = 33;
pub const R_OR1K_TLS_DTPMOD: u32 = 34;
pub const R_OR1K_NUM: u32 = 35;

/////////////////////
// MIPS
/////////////////////
/// No reloc
pub const R_MIPS_NONE: u32 = 0;
/// Direct 16 bit
pub const R_MIPS_16: u32 = 1;
/// Direct 32 bit
pub const R_MIPS_32: u32 = 2;
/// PC relative 32 bit
pub const R_MIPS_REL32: u32 = 3;
/// Direct 26 bit shifted
pub const R_MIPS_26: u32 = 4;
/// High 16 bit
pub const R_MIPS_HI16: u32 = 5;
/// Low 16 bit
pub const R_MIPS_LO16: u32 = 6;
/// GP relative 16 bit
pub const R_MIPS_GPREL16: u32 = 7;
/// 16 bit literal entry
pub const R_MIPS_LITERAL: u32 = 8;
/// 16 bit GOT entry
pub const R_MIPS_GOT16: u32 = 9;
/// PC relative 16 bit
pub const R_MIPS_PC16: u32 = 10;
/// 16 bit GOT entry for function
pub const R_MIPS_CALL16: u32 = 11;
/// GP relative 32 bit
pub const R_MIPS_GPREL32: u32 = 12;
pub const R_MIPS_SHIFT5: u32 = 16;
pub const R_MIPS_SHIFT6: u32 = 17;
pub const R_MIPS_64: u32 = 18;
pub const R_MIPS_GOT_DISP: u32 = 19;
pub const R_MIPS_GOT_PAGE: u32 = 20;
pub const R_MIPS_GOT_OFST: u32 = 21;
pub const R_MIPS_GOT_HI16: u32 = 22;
pub const R_MIPS_GOT_LO16: u32 = 23;
pub const R_MIPS_SUB: u32 = 24;
pub const R_MIPS_INSERT_A: u32 = 25;
pub const R_MIPS_INSERT_B: u32 = 26;
pub const R_MIPS_DELETE: u32 = 27;
pub const R_MIPS_HIGHER: u32 = 28;
pub const R_MIPS_HIGHEST: u32 = 29;
pub const R_MIPS_CALL_HI16: u32 = 30;
pub const R_MIPS_CALL_LO16: u32 = 31;
pub const R_MIPS_SCN_DISP: u32 = 32;
pub const R_MIPS_REL16: u32 = 33;
pub const R_MIPS_ADD_IMMEDIATE: u32 = 34;
pub const R_MIPS_PJUMP: u32 = 35;
pub const R_MIPS_RELGOT: u32 = 36;
pub const R_MIPS_JALR: u32 = 37;
/// Module number 32 bit
pub const R_MIPS_TLS_DTPMOD32: u32 = 38;
/// Module-relative offset 32 bit
pub const R_MIPS_TLS_DTPREL32: u32 = 39;
/// Module number 64 bit
pub const R_MIPS_TLS_DTPMOD64: u32 = 40;
/// Module-relative offset 64 bit
pub const R_MIPS_TLS_DTPREL64: u32 = 41;
/// 16 bit GOT offset for GD
pub const R_MIPS_TLS_GD: u32 = 42;
/// 16 bit GOT offset for LDM
pub const R_MIPS_TLS_LDM: u32 = 43;
/// Module-relative offset, high 16 bits
pub const R_MIPS_TLS_DTPREL_HI16: u32 = 44;
/// Module-relative offset, low 16 bits
pub const R_MIPS_TLS_DTPREL_LO16: u32 = 45;
/// 16 bit GOT offset for IE
pub const R_MIPS_TLS_GOTTPREL: u32 = 46;
/// TP-relative offset, 32 bit6
pub const R_MIPS_TLS_TPREL32: u32 = 47;
/// TP-relative offset, 64 bit
pub const R_MIPS_TLS_TPREL64: u32 = 48;
/// TP-relative offset, high 16 bits
pub const R_MIPS_TLS_TPREL_HI16: u32 = 49;
/// TP-relative offset, low 16 bits
pub const R_MIPS_TLS_TPREL_LO16: u32 = 50;
pub const R_MIPS_GLOB_DAT: u32 = 51;
pub const R_MIPS_COPY: u32 = 126;
pub const R_MIPS_JUMP_SLOT: u32 = 127;
pub const R_MIPS_NUM: u32 = 128;

///////////////////
// RISC-V
// See https://github.com/riscv/riscv-elf-psabi-doc
///////////////////
/// None
pub const R_RISCV_NONE: u32 = 0;
/// Runtime relocation: word32 = S + A
pub const R_RISCV_32: u32 = 1;
/// Runtime relocation: word64 = S + A
pub const R_RISCV_64: u32 = 2;
/// Runtime relocation: word32,64 = B + A
pub const R_RISCV_RELATIVE: u32 = 3;
/// Runtime relocation: must be in executable, not allowed in shared library
pub const R_RISCV_COPY: u32 = 4;
/// Runtime relocation: word32,64 = S; handled by PLT unless LD_BIND_NOW
pub const R_RISCV_JUMP_SLOT: u32 = 5;
/// TLS relocation: word32 = S->TLSINDEX
pub const R_RISCV_TLS_DTPMOD32: u32 = 6;
/// TLS relocation: word64 = S->TLSINDEX
pub const R_RISCV_TLS_DTPMOD64: u32 = 7;
/// TLS relocation: word32 = TLS + S + A - TLS_TP_OFFSET
pub const R_RISCV_TLS_DTPREL32: u32 = 8;
/// TLS relocation: word64 = TLS + S + A - TLS_TP_OFFSET
pub const R_RISCV_TLS_DTPREL64: u32 = 9;
/// TLS relocation: word32 = TLS + S + A + S_TLS_OFFSET - TLS_DTV_OFFSET
pub const R_RISCV_TLS_TPREL32: u32 = 10;
/// TLS relocation: word64 = TLS + S + A + S_TLS_OFFSET - TLS_DTV_OFFSET
pub const R_RISCV_TLS_TPREL64: u32 = 11;
/// PC-relative branch (SB-Type)
pub const R_RISCV_BRANCH: u32 = 16;
/// PC-relative jump (UJ-Type)
pub const R_RISCV_JAL: u32 = 17;
/// PC-relative call: MACRO call,tail (auipc+jalr pair)
pub const R_RISCV_CALL: u32 = 18;
/// PC-relative call (PLT): MACRO call,tail (auipc+jalr pair) PIC
pub const R_RISCV_CALL_PLT: u32 = 19;
/// PC-relative GOT reference: MACRO la
pub const R_RISCV_GOT_HI20: u32 = 20;
/// PC-relative TLS IE GOT offset: MACRO la.tls.ie
pub const R_RISCV_TLS_GOT_HI20: u32 = 21;
/// PC-relative TLS GD reference: MACRO la.tls.gd
pub const R_RISCV_TLS_GD_HI20: u32 = 22;
/// PC-relative reference: %pcrel_hi(symbol) (U-Type)
pub const R_RISCV_PCREL_HI20: u32 = 23;
/// PC-relative reference: %pcrel_lo(symbol) (I-Type)
pub const R_RISCV_PCREL_LO12_I: u32 = 24;
/// PC-relative reference: %pcrel_lo(symbol) (S-Type)
pub const R_RISCV_PCREL_LO12_S: u32 = 25;
/// Absolute address: %hi(symbol) (U-Type)
pub const R_RISCV_HI20: u32 = 26;
/// Absolute address: %lo(symbol) (I-Type)
pub const R_RISCV_LO12_I: u32 = 27;
/// Absolute address: %lo(symbol) (S-Type)
pub const R_RISCV_LO12_S: u32 = 28;
/// TLS LE thread offset: %tprel_hi(symbol) (U-Type)
pub const R_RISCV_TPREL_HI20: u32 = 29;
/// TLS LE thread offset: %tprel_lo(symbol) (I-Type)
pub const R_RISCV_TPREL_LO12_I: u32 = 30;
/// TLS LE thread offset: %tprel_lo(symbol) (S-Type)
pub const R_RISCV_TPREL_LO12_S: u32 = 31;
/// TLS LE thread usage: %tprel_add(symbol)
pub const R_RISCV_TPREL_ADD: u32 = 32;
/// 8-bit label addition: word8 = S + A
pub const R_RISCV_ADD8: u32 = 33;
/// 16-bit label addition: word16 = S + A
pub const R_RISCV_ADD16: u32 = 34;
/// 32-bit label addition: word32 = S + A
pub const R_RISCV_ADD32: u32 = 35;
/// 64-bit label addition: word64 = S + A
pub const R_RISCV_ADD64: u32 = 36;
/// 8-bit label subtraction: word8 = S - A
pub const R_RISCV_SUB8: u32 = 37;
/// 16-bit label subtraction: word16 = S - A
pub const R_RISCV_SUB16: u32 = 38;
/// 32-bit label subtraction: word32 = S - A
pub const R_RISCV_SUB32: u32 = 39;
/// 64-bit label subtraction: word64 = S - A
pub const R_RISCV_SUB64: u32 = 40;
/// GNU C++ vtable hierarchy
pub const R_RISCV_GNU_VTINHERIT: u32 = 41;
/// GNU C++ vtable member usage
pub const R_RISCV_GNU_VTENTRY: u32 = 42;
/// Alignment statement
pub const R_RISCV_ALIGN: u32 = 43;
/// PC-relative branch offset (CB-Type)
pub const R_RISCV_RVC_BRANCH: u32 = 44;
/// PC-relative jump offset (CJ-Type)
pub const R_RISCV_RVC_JUMP: u32 = 45;
/// Absolute address (CI-Type)
pub const R_RISCV_RVC_LUI: u32 = 46;
/// GP-relative reference (I-Type)
pub const R_RISCV_GPREL_I: u32 = 47;
/// GP-relative reference (S-Type)
pub const R_RISCV_GPREL_S: u32 = 48;
/// TP-relative TLS LE load (I-Type)
pub const R_RISCV_TPREL_I: u32 = 49;
/// TP-relative TLS LE store (S-Type)
pub const R_RISCV_TPREL_S: u32 = 50;
/// Instruction pair can be relaxed
pub const R_RISCV_RELAX: u32 = 51;
/// Local label subtraction
pub const R_RISCV_SUB6: u32 = 52;
/// Local label subtraction
pub const R_RISCV_SET6: u32 = 53;
/// Local label subtraction
pub const R_RISCV_SET8: u32 = 54;
/// Local label subtraction
pub const R_RISCV_SET16: u32 = 55;
/// Local label subtraction
pub const R_RISCV_SET32: u32 = 56;

/////////////////////
// PowerPC
// See: https://www.nxp.com/docs/en/reference-manual/E500ABIUG.pdf
/////////////////////
pub const R_PPC_NONE: u32 = 0;
pub const R_PPC_ADDR32: u32 = 1;
pub const R_PPC_ADDR24: u32 = 2;
pub const R_PPC_ADDR16: u32 = 3;
pub const R_PPC_ADDR16_LO: u32 = 4;
pub const R_PPC_ADDR16_HI: u32 = 5;
pub const R_PPC_ADDR16_HA: u32 = 6;
pub const R_PPC_ADDR14: u32 = 7;
pub const R_PPC_ADDR14_BRTAKEN: u32 = 8;
pub const R_PPC_ADDR14_BRNTAKEN: u32 = 9;
pub const R_PPC_REL24: u32 = 10;
pub const R_PPC_REL14: u32 = 11;
pub const R_PPC_REL14_BRTAKEN: u32 = 12;
pub const R_PPC_REL14_BRNTAKEN: u32 = 13;
pub const R_PPC_GOT16: u32 = 14;
pub const R_PPC_GOT16_LO: u32 = 15;
pub const R_PPC_GOT16_HI: u32 = 16;
pub const R_PPC_GOT16_HA: u32 = 17;
pub const R_PPC_PLTREL24: u32 = 18;
pub const R_PPC_COPY: u32 = 19;
pub const R_PPC_GLOB_DAT: u32 = 20;
pub const R_PPC_JMP_SLOT: u32 = 21;
pub const R_PPC_RELATIVE: u32 = 22;
pub const R_PPC_LOCAL24PC: u32 = 23;
pub const R_PPC_UADDR32: u32 = 24;
pub const R_PPC_UADDR16: u32 = 25;
pub const R_PPC_REL32: u32 = 26;
pub const R_PPC_PLT32: u32 = 27;
pub const R_PPC_PLTREL32: u32 = 28;
pub const R_PPC_PLT16_LO: u32 = 29;
pub const R_PPC_PLT16_HI: u32 = 30;
pub const R_PPC_PLT16_HA: u32 = 31;
pub const R_PPC_SDAREL16: u32 = 32;
pub const R_PPC_SECTOFF: u32 = 33;
pub const R_PPC_SECTOFF_LO: u32 = 34;
pub const R_PPC_SECTOFF_HI: u32 = 35;
pub const R_PPC_SECTOFF_HA: u32 = 36;
pub const R_PPC_ADDR30: u32 = 37;
pub const R_PPC_EMB_NADDR32: u32 = 101;
pub const R_PPC_EMB_NADDR16: u32 = 102;
pub const R_PPC_EMB_NADDR16_LO: u32 = 103;
pub const R_PPC_EMB_NADDR16_HI: u32 = 104;
pub const R_PPC_EMB_NADDR16_HA: u32 = 105;
pub const R_PPC_EMB_SDA_I16: u32 = 106;
pub const R_PPC_EMB_SDA2_I16: u32 = 107;
pub const R_PPC_EMB_SDA2REL: u32 = 108;
pub const R_PPC_EMB_SDA21: u32 = 109;
pub const R_PPC_EMB_MRKREF: u32 = 110;
pub const R_PPC_EMB_RELSEC16: u32 = 111;
pub const R_PPC_EMB_RELST_LO: u32 = 112;
pub const R_PPC_EMB_RELST_HI: u32 = 113;
pub const R_PPC_EMB_RELST_HA: u32 = 114;
pub const R_PPC_EMB_BIT_FLD: u32 = 115;
pub const R_PPC_EMB_RELSDA: u32 = 116;
pub const R_PPC_EMB_RELOC_120: u32 = 120;
pub const R_PPC_EMB_RELOC_121: u32 = 121;
pub const R_PPC_DIAB_SDA21_LO: u32 = 180;
pub const R_PPC_DIAB_SDA21_HI: u32 = 181;
pub const R_PPC_DIAB_SDA21_HA: u32 = 182;
pub const R_PPC_DIAB_RELSDA_LO: u32 = 183;
pub const R_PPC_DIAB_RELSDA_HI: u32 = 184;
pub const R_PPC_DIAB_RELSDA_HA: u32 = 185;
pub const R_PPC_EMB_SPE_DOUBLE: u32 = 201;
pub const R_PPC_EMB_SPE_WORD: u32 = 202;
pub const R_PPC_EMB_SPE_HALF: u32 = 203;
pub const R_PPC_EMB_SPE_DOUBLE_SDAREL: u32 = 204;
pub const R_PPC_EMB_SPE_WORD_SDAREL: u32 = 205;
pub const R_PPC_EMB_SPE_HALF_SDAREL: u32 = 206;
pub const R_PPC_EMB_SPE_DOUBLE_SDA2REL: u32 = 207;
pub const R_PPC_EMB_SPE_WORD_SDA2REL: u32 = 208;
pub const R_PPC_EMB_SPE_HALF_SDA2REL: u32 = 209;
pub const R_PPC_EMB_SPE_DOUBLE_SDA0REL: u32 = 210;
pub const R_PPC_EMB_SPE_WORD_SDA0REL: u32 = 211;
pub const R_PPC_EMB_SPE_HALF_SDA0REL: u32 = 212;
pub const R_PPC_EMB_SPE_DOUBLE_SDA: u32 = 213;
pub const R_PPC_EMB_SPE_WORD_SDA: u32 = 214;
pub const R_PPC_EMB_SPE_HALF_SDA: u32 = 215;


#[inline]
pub fn r_to_str(typ: u32, machine: u16) -> &'static str {
    use crate::elf::header::*;
    match machine {
        // x86
        EM_386 => { match typ {
        R_386_NONE => "386_NONE",
        R_386_32 => "386_32",
        R_386_PC32 => "386_PC32",
        R_386_GOT32 => "386_GOT32",
        R_386_PLT32 => "386_PLT32",
        R_386_COPY => "386_COPY",
        R_386_GLOB_DAT => "386_GLOB_DAT",
        R_386_JMP_SLOT => "386_JMP_SLOT",
        R_386_RELATIVE => "386_RELATIVE",
        R_386_GOTOFF => "386_GOTOFF",
        R_386_GOTPC => "386_GOTPC",
        R_386_32PLT => "386_32PLT",
        R_386_TLS_TPOFF => "386_TLS_TPOFF",
        R_386_TLS_IE => "386_TLS_IE",
        R_386_TLS_GOTIE => "386_TLS_GOTIE",
        R_386_TLS_LE => "386_TLS_LE",
        R_386_TLS_GD => "386_TLS_GD",
        R_386_TLS_LDM => "386_TLS_LDM",
        R_386_16 => "386_16",
        R_386_PC16 => "386_PC16",
        R_386_8 => "386_8",
        R_386_PC8 => "386_PC8",
        R_386_TLS_GD_32 => "386_TLS_GD_32",
        R_386_TLS_GD_PUSH => "386_TLS_GD_PUSH",
        R_386_TLS_GD_CALL => "386_TLS_GD_CALL",
        R_386_TLS_GD_POP => "386_TLS_GD_POP",
        R_386_TLS_LDM_32 => "386_TLS_LDM_32",
        R_386_TLS_LDM_PUSH => "386_TLS_LDM_PUSH",
        R_386_TLS_LDM_CALL => "386_TLS_LDM_CALL",
        R_386_TLS_LDM_POP => "386_TLS_LDM_POP",
        R_386_TLS_LDO_32 => "386_TLS_LDO_32",
        R_386_TLS_IE_32 => "386_TLS_IE_32",
        R_386_TLS_LE_32 => "386_TLS_LE_32",
        R_386_TLS_DTPMOD32 => "386_TLS_DTPMOD32",
        R_386_TLS_DTPOFF32 => "386_TLS_DTPOFF32",
        R_386_TLS_TPOFF32 => "386_TLS_TPOFF32",
        R_386_SIZE32 => "386_SIZE32",
        R_386_TLS_GOTDESC => "386_TLS_GOTDESC",
        R_386_TLS_DESC_CALL => "386_TLS_DESC_CALL",
        R_386_TLS_DESC => "386_TLS_DESC",
        R_386_IRELATIVE => "386_IRELATIVE",
        R_386_GOT32X => "386_GOT32X",
        _ => "R_UNKNOWN_386",
        }},
        EM_X86_64 => { match typ {
        R_X86_64_64 => "X86_64_64",
        R_X86_64_PC32 => "X86_64_PC32",
        R_X86_64_GOT32 => "X86_64_GOT32",
        R_X86_64_PLT32 => "X86_64_PLT32",
        R_X86_64_COPY => "X86_64_COPY",
        R_X86_64_GLOB_DAT => "X86_64_GLOB_DAT",
        R_X86_64_JUMP_SLOT => "X86_64_JUMP_SLOT",
        R_X86_64_RELATIVE => "X86_64_RELATIVE",
        R_X86_64_GOTPCREL => "X86_64_GOTPCREL",
        R_X86_64_32 => "X86_64_32",
        R_X86_64_32S => "X86_64_32S",
        R_X86_64_16 => "X86_64_16",
        R_X86_64_PC16 => "X86_64_PC16",
        R_X86_64_8 => "X86_64_8",
        R_X86_64_PC8 => "X86_64_PC8",
        R_X86_64_DTPMOD64 => "X86_64_DTPMOD64",
        R_X86_64_DTPOFF64 => "X86_64_DTPOFF64",
        R_X86_64_TPOFF64 => "X86_64_TPOFF64",
        R_X86_64_TLSGD => "X86_64_TLSGD",
        R_X86_64_TLSLD => "X86_64_TLSLD",
        R_X86_64_DTPOFF32 => "X86_64_DTPOFF32",
        R_X86_64_GOTTPOFF => "X86_64_GOTTPOFF",
        R_X86_64_TPOFF32 => "X86_64_TPOFF32",
        R_X86_64_PC64 => "X86_64_PC64",
        R_X86_64_GOTOFF64 => "X86_64_GOTOFF64",
        R_X86_64_GOTPC32 => "X86_64_GOTPC32",
        R_X86_64_GOT64 => "X86_64_GOT64",
        R_X86_64_GOTPCREL64 => "X86_64_GOTPCREL64",
        R_X86_64_GOTPC64 => "X86_64_GOTPC64",
        R_X86_64_GOTPLT64 => "X86_64_GOTPLT64",
        R_X86_64_PLTOFF64 => "X86_64_PLTOFF64",
        R_X86_64_SIZE32 => "X86_64_SIZE32",
        R_X86_64_SIZE64 => "X86_64_SIZE64",
        R_X86_64_GOTPC32_TLSDESC => "X86_64_GOTPC32_TLSDESC",
        R_X86_64_TLSDESC_CALL => "X86_64_TLSDESC_CALL",
        R_X86_64_TLSDESC => "X86_64_TLSDESC",
        R_X86_64_IRELATIVE => "X86_64_IRELATIVE",
        R_X86_64_RELATIVE64 => "X86_64_RELATIVE64",
        R_X86_64_GOTPCRELX => "R_X86_64_GOTPCRELX",
        R_X86_64_REX_GOTPCRELX => "R_X86_64_REX_GOTPCRELX",
        _ => "R_UNKNOWN_X86_64",
        }},
        // openrisc
        EM_OPENRISC => { match typ {
        R_OR1K_NONE => "OR1K_NONE",
        R_OR1K_32 => "OR1K_32",
        R_OR1K_16 => "OR1K_16",
        R_OR1K_8 => "OR1K_8",
        R_OR1K_LO_16_IN_INSN => "OR1K_LO_16_IN_INSN",
        R_OR1K_HI_16_IN_INSN => "OR1K_HI_16_IN_INSN",
        R_OR1K_INSN_REL_26 => "OR1K_INSN_REL_26",
        R_OR1K_GNU_VTENTRY => "OR1K_GNU_VTENTRY",
        R_OR1K_GNU_VTINHERIT => "OR1K_GNU_VTINHERIT",
        R_OR1K_32_PCREL => "OR1K_32_PCREL",
        R_OR1K_16_PCREL => "OR1K_16_PCREL",
        R_OR1K_8_PCREL => "OR1K_8_PCREL",
        R_OR1K_GOTPC_HI16 => "OR1K_GOTPC_HI16",
        R_OR1K_GOTPC_LO16 => "OR1K_GOTPC_LO16",
        R_OR1K_GOT16 => "OR1K_GOT16",
        R_OR1K_PLT26 => "OR1K_PLT26",
        R_OR1K_GOTOFF_HI16 => "OR1K_GOTOFF_HI16",
        R_OR1K_GOTOFF_LO16 => "OR1K_GOTOFF_LO16",
        R_OR1K_COPY => "OR1K_COPY",
        R_OR1K_GLOB_DAT => "OR1K_GLOB_DAT",
        R_OR1K_JMP_SLOT => "OR1K_JMP_SLOT",
        R_OR1K_RELATIVE => "OR1K_RELATIVE",
        R_OR1K_TLS_GD_HI16 => "OR1K_TLS_GD_HI16",
        R_OR1K_TLS_GD_LO16 => "OR1K_TLS_GD_LO16",
        R_OR1K_TLS_LDM_HI16 => "OR1K_TLS_LDM_HI16",
        R_OR1K_TLS_LDM_LO16 => "OR1K_TLS_LDM_LO16",
        R_OR1K_TLS_LDO_HI16 => "OR1K_TLS_LDO_HI16",
        R_OR1K_TLS_LDO_LO16 => "OR1K_TLS_LDO_LO16",
        R_OR1K_TLS_IE_HI16 => "OR1K_TLS_IE_HI16",
        R_OR1K_TLS_IE_LO16 => "OR1K_TLS_IE_LO16",
        R_OR1K_TLS_LE_HI16 => "OR1K_TLS_LE_HI16",
        R_OR1K_TLS_LE_LO16 => "OR1K_TLS_LE_LO16",
        R_OR1K_TLS_TPOFF => "OR1K_TLS_TPOFF",
        R_OR1K_TLS_DTPOFF => "OR1K_TLS_DTPOFF",
        R_OR1K_TLS_DTPMOD => "OR1K_TLS_DTPMOD",
        _ => "R_UNKNOWN_OR1K",
        }},
        // arm64
        EM_AARCH64 => { match typ {
        R_AARCH64_P32_ABS32 => "AARCH64_P32_ABS32",
        R_AARCH64_P32_COPY => "AARCH64_P32_COPY",
        R_AARCH64_P32_GLOB_DAT => "AARCH64_P32_GLOB_DAT",
        R_AARCH64_P32_JUMP_SLOT => "AARCH64_P32_JUMP_SLOT",
        R_AARCH64_P32_RELATIVE => "AARCH64_P32_RELATIVE",
        R_AARCH64_P32_TLS_DTPMOD => "AARCH64_P32_TLS_DTPMOD",
        R_AARCH64_P32_TLS_DTPREL => "AARCH64_P32_TLS_DTPREL",
        R_AARCH64_P32_TLS_TPREL => "AARCH64_P32_TLS_TPREL",
        R_AARCH64_P32_TLSDESC => "AARCH64_P32_TLSDESC",
        R_AARCH64_P32_IRELATIVE => "AARCH64_P32_IRELATIVE",
        R_AARCH64_ABS64 => "AARCH64_ABS64",
        R_AARCH64_ABS32 => "AARCH64_ABS32",
        R_AARCH64_ABS16 => "AARCH64_ABS16",
        R_AARCH64_PREL64 => "AARCH64_PREL64",
        R_AARCH64_PREL32 => "AARCH64_PREL32",
        R_AARCH64_PREL16 => "AARCH64_PREL16",
        R_AARCH64_MOVW_UABS_G0 => "AARCH64_MOVW_UABS_G0",
        R_AARCH64_MOVW_UABS_G0_NC => "AARCH64_MOVW_UABS_G0_NC",
        R_AARCH64_MOVW_UABS_G1 => "AARCH64_MOVW_UABS_G1",
        R_AARCH64_MOVW_UABS_G1_NC => "AARCH64_MOVW_UABS_G1_NC",
        R_AARCH64_MOVW_UABS_G2 => "AARCH64_MOVW_UABS_G2",
        R_AARCH64_MOVW_UABS_G2_NC => "AARCH64_MOVW_UABS_G2_NC",
        R_AARCH64_MOVW_UABS_G3 => "AARCH64_MOVW_UABS_G3",
        R_AARCH64_MOVW_SABS_G0 => "AARCH64_MOVW_SABS_G0",
        R_AARCH64_MOVW_SABS_G1 => "AARCH64_MOVW_SABS_G1",
        R_AARCH64_MOVW_SABS_G2 => "AARCH64_MOVW_SABS_G2",
        R_AARCH64_LD_PREL_LO19 => "AARCH64_LD_PREL_LO19",
        R_AARCH64_ADR_PREL_LO21 => "AARCH64_ADR_PREL_LO21",
        R_AARCH64_ADR_PREL_PG_HI21 => "AARCH64_ADR_PREL_PG_HI21",
        R_AARCH64_ADR_PREL_PG_HI21_NC => "AARCH64_ADR_PREL_PG_HI21_NC",
        R_AARCH64_ADD_ABS_LO12_NC => "AARCH64_ADD_ABS_LO12_NC",
        R_AARCH64_LDST8_ABS_LO12_NC => "AARCH64_LDST8_ABS_LO12_NC",
        R_AARCH64_TSTBR14 => "AARCH64_TSTBR14",
        R_AARCH64_CONDBR19 => "AARCH64_CONDBR19",
        R_AARCH64_JUMP26 => "AARCH64_JUMP26",
        R_AARCH64_CALL26 => "AARCH64_CALL26",
        R_AARCH64_LDST16_ABS_LO12_NC => "AARCH64_LDST16_ABS_LO12_NC",
        R_AARCH64_LDST32_ABS_LO12_NC => "AARCH64_LDST32_ABS_LO12_NC",
        R_AARCH64_LDST64_ABS_LO12_NC => "AARCH64_LDST64_ABS_LO12_NC",
        R_AARCH64_MOVW_PREL_G0 => "AARCH64_MOVW_PREL_G0",
        R_AARCH64_MOVW_PREL_G0_NC => "AARCH64_MOVW_PREL_G0_NC",
        R_AARCH64_MOVW_PREL_G1 => "AARCH64_MOVW_PREL_G1",
        R_AARCH64_MOVW_PREL_G1_NC => "AARCH64_MOVW_PREL_G1_NC",
        R_AARCH64_MOVW_PREL_G2 => "AARCH64_MOVW_PREL_G2",
        R_AARCH64_MOVW_PREL_G2_NC => "AARCH64_MOVW_PREL_G2_NC",
        R_AARCH64_MOVW_PREL_G3 => "AARCH64_MOVW_PREL_G3",
        R_AARCH64_LDST128_ABS_LO12_NC => "AARCH64_LDST128_ABS_LO12_NC",
        R_AARCH64_MOVW_GOTOFF_G0 => "AARCH64_MOVW_GOTOFF_G0",
        R_AARCH64_MOVW_GOTOFF_G0_NC => "AARCH64_MOVW_GOTOFF_G0_NC",
        R_AARCH64_MOVW_GOTOFF_G1 => "AARCH64_MOVW_GOTOFF_G1",
        R_AARCH64_MOVW_GOTOFF_G1_NC => "AARCH64_MOVW_GOTOFF_G1_NC",
        R_AARCH64_MOVW_GOTOFF_G2 => "AARCH64_MOVW_GOTOFF_G2",
        R_AARCH64_MOVW_GOTOFF_G2_NC => "AARCH64_MOVW_GOTOFF_G2_NC",
        R_AARCH64_MOVW_GOTOFF_G3 => "AARCH64_MOVW_GOTOFF_G3",
        R_AARCH64_GOTREL64 => "AARCH64_GOTREL64",
        R_AARCH64_GOTREL32 => "AARCH64_GOTREL32",
        R_AARCH64_GOT_LD_PREL19 => "AARCH64_GOT_LD_PREL19",
        R_AARCH64_LD64_GOTOFF_LO15 => "AARCH64_LD64_GOTOFF_LO15",
        R_AARCH64_ADR_GOT_PAGE => "AARCH64_ADR_GOT_PAGE",
        R_AARCH64_LD64_GOT_LO12_NC => "AARCH64_LD64_GOT_LO12_NC",
        R_AARCH64_LD64_GOTPAGE_LO15 => "AARCH64_LD64_GOTPAGE_LO15",
        R_AARCH64_TLSGD_ADR_PREL21 => "AARCH64_TLSGD_ADR_PREL21",
        R_AARCH64_TLSGD_ADR_PAGE21 => "AARCH64_TLSGD_ADR_PAGE21",
        R_AARCH64_TLSGD_ADD_LO12_NC => "AARCH64_TLSGD_ADD_LO12_NC",
        R_AARCH64_TLSGD_MOVW_G1 => "AARCH64_TLSGD_MOVW_G1",
        R_AARCH64_TLSGD_MOVW_G0_NC => "AARCH64_TLSGD_MOVW_G0_NC",
        R_AARCH64_TLSLD_ADR_PREL21 => "AARCH64_TLSLD_ADR_PREL21",
        R_AARCH64_TLSLD_ADR_PAGE21 => "AARCH64_TLSLD_ADR_PAGE21",
        R_AARCH64_TLSLD_ADD_LO12_NC => "AARCH64_TLSLD_ADD_LO12_NC",
        R_AARCH64_TLSLD_MOVW_G1 => "AARCH64_TLSLD_MOVW_G1",
        R_AARCH64_TLSLD_MOVW_G0_NC => "AARCH64_TLSLD_MOVW_G0_NC",
        R_AARCH64_TLSLD_LD_PREL19 => "AARCH64_TLSLD_LD_PREL19",
        R_AARCH64_TLSLD_MOVW_DTPREL_G2 => "AARCH64_TLSLD_MOVW_DTPREL_G2",
        R_AARCH64_TLSLD_MOVW_DTPREL_G1 => "AARCH64_TLSLD_MOVW_DTPREL_G1",
        R_AARCH64_TLSLD_MOVW_DTPREL_G1_NC => "AARCH64_TLSLD_MOVW_DTPREL_G1_NC",
        R_AARCH64_TLSLD_MOVW_DTPREL_G0 => "AARCH64_TLSLD_MOVW_DTPREL_G0",
        R_AARCH64_TLSLD_MOVW_DTPREL_G0_NC => "AARCH64_TLSLD_MOVW_DTPREL_G0_NC",
        R_AARCH64_TLSLD_ADD_DTPREL_HI12 => "AARCH64_TLSLD_ADD_DTPREL_HI12",
        R_AARCH64_TLSLD_ADD_DTPREL_LO12 => "AARCH64_TLSLD_ADD_DTPREL_LO12",
        R_AARCH64_TLSLD_ADD_DTPREL_LO12_NC => "AARCH64_TLSLD_ADD_DTPREL_LO12_NC",
        R_AARCH64_TLSLD_LDST8_DTPREL_LO12 => "AARCH64_TLSLD_LDST8_DTPREL_LO12",
        R_AARCH64_TLSLD_LDST8_DTPREL_LO12_NC => "AARCH64_TLSLD_LDST8_DTPREL_LO12_NC",
        R_AARCH64_TLSLD_LDST16_DTPREL_LO12 => "AARCH64_TLSLD_LDST16_DTPREL_LO12",
        R_AARCH64_TLSLD_LDST16_DTPREL_LO12_NC => "AARCH64_TLSLD_LDST16_DTPREL_LO12_NC",
        R_AARCH64_TLSLD_LDST32_DTPREL_LO12 => "AARCH64_TLSLD_LDST32_DTPREL_LO12",
        R_AARCH64_TLSLD_LDST32_DTPREL_LO12_NC => "AARCH64_TLSLD_LDST32_DTPREL_LO12_NC",
        R_AARCH64_TLSLD_LDST64_DTPREL_LO12 => "AARCH64_TLSLD_LDST64_DTPREL_LO12",
        R_AARCH64_TLSLD_LDST64_DTPREL_LO12_NC => "AARCH64_TLSLD_LDST64_DTPREL_LO12_NC",
        R_AARCH64_TLSIE_MOVW_GOTTPREL_G1 => "AARCH64_TLSIE_MOVW_GOTTPREL_G1",
        R_AARCH64_TLSIE_MOVW_GOTTPREL_G0_NC => "AARCH64_TLSIE_MOVW_GOTTPREL_G0_NC",
        R_AARCH64_TLSIE_ADR_GOTTPREL_PAGE21 => "AARCH64_TLSIE_ADR_GOTTPREL_PAGE21",
        R_AARCH64_TLSIE_LD64_GOTTPREL_LO12_NC => "AARCH64_TLSIE_LD64_GOTTPREL_LO12_NC",
        R_AARCH64_TLSIE_LD_GOTTPREL_PREL19 => "AARCH64_TLSIE_LD_GOTTPREL_PREL19",
        R_AARCH64_TLSLE_MOVW_TPREL_G2 => "AARCH64_TLSLE_MOVW_TPREL_G2",
        R_AARCH64_TLSLE_MOVW_TPREL_G1 => "AARCH64_TLSLE_MOVW_TPREL_G1",
        R_AARCH64_TLSLE_MOVW_TPREL_G1_NC => "AARCH64_TLSLE_MOVW_TPREL_G1_NC",
        R_AARCH64_TLSLE_MOVW_TPREL_G0 => "AARCH64_TLSLE_MOVW_TPREL_G0",
        R_AARCH64_TLSLE_MOVW_TPREL_G0_NC => "AARCH64_TLSLE_MOVW_TPREL_G0_NC",
        R_AARCH64_TLSLE_ADD_TPREL_HI12 => "AARCH64_TLSLE_ADD_TPREL_HI12",
        R_AARCH64_TLSLE_ADD_TPREL_LO12 => "AARCH64_TLSLE_ADD_TPREL_LO12",
        R_AARCH64_TLSLE_ADD_TPREL_LO12_NC => "AARCH64_TLSLE_ADD_TPREL_LO12_NC",
        R_AARCH64_TLSLE_LDST8_TPREL_LO12 => "AARCH64_TLSLE_LDST8_TPREL_LO12",
        R_AARCH64_TLSLE_LDST8_TPREL_LO12_NC => "AARCH64_TLSLE_LDST8_TPREL_LO12_NC",
        R_AARCH64_TLSLE_LDST16_TPREL_LO12 => "AARCH64_TLSLE_LDST16_TPREL_LO12",
        R_AARCH64_TLSLE_LDST16_TPREL_LO12_NC => "AARCH64_TLSLE_LDST16_TPREL_LO12_NC",
        R_AARCH64_TLSLE_LDST32_TPREL_LO12 => "AARCH64_TLSLE_LDST32_TPREL_LO12",
        R_AARCH64_TLSLE_LDST32_TPREL_LO12_NC => "AARCH64_TLSLE_LDST32_TPREL_LO12_NC",
        R_AARCH64_TLSLE_LDST64_TPREL_LO12 => "AARCH64_TLSLE_LDST64_TPREL_LO12",
        R_AARCH64_TLSLE_LDST64_TPREL_LO12_NC => "AARCH64_TLSLE_LDST64_TPREL_LO12_NC",
        R_AARCH64_TLSDESC_LD_PREL19 => "AARCH64_TLSDESC_LD_PREL19",
        R_AARCH64_TLSDESC_ADR_PREL21 => "AARCH64_TLSDESC_ADR_PREL21",
        R_AARCH64_TLSDESC_ADR_PAGE21 => "AARCH64_TLSDESC_ADR_PAGE21",
        R_AARCH64_TLSDESC_LD64_LO12 => "AARCH64_TLSDESC_LD64_LO12",
        R_AARCH64_TLSDESC_ADD_LO12 => "AARCH64_TLSDESC_ADD_LO12",
        R_AARCH64_TLSDESC_OFF_G1 => "AARCH64_TLSDESC_OFF_G1",
        R_AARCH64_TLSDESC_OFF_G0_NC => "AARCH64_TLSDESC_OFF_G0_NC",
        R_AARCH64_TLSDESC_LDR => "AARCH64_TLSDESC_LDR",
        R_AARCH64_TLSDESC_ADD => "AARCH64_TLSDESC_ADD",
        R_AARCH64_TLSDESC_CALL => "AARCH64_TLSDESC_CALL",
        R_AARCH64_TLSLE_LDST128_TPREL_LO12 => "AARCH64_TLSLE_LDST128_TPREL_LO12",
        R_AARCH64_TLSLE_LDST128_TPREL_LO12_NC => "AARCH64_TLSLE_LDST128_TPREL_LO12_NC",
        R_AARCH64_TLSLD_LDST128_DTPREL_LO12 => "AARCH64_TLSLD_LDST128_DTPREL_LO12",
        R_AARCH64_TLSLD_LDST128_DTPREL_LO12_NC => "AARCH64_TLSLD_LDST128_DTPREL_LO12_NC",
        R_AARCH64_COPY => "AARCH64_COPY",
        R_AARCH64_GLOB_DAT => "AARCH64_GLOB_DAT",
        R_AARCH64_JUMP_SLOT => "AARCH64_JUMP_SLOT",
        R_AARCH64_RELATIVE => "AARCH64_RELATIVE",
        R_AARCH64_TLS_DTPMOD => "AARCH64_TLS_DTPMOD",
        R_AARCH64_TLS_DTPREL => "AARCH64_TLS_DTPREL",
        R_AARCH64_TLS_TPREL => "AARCH64_TLS_TPREL",
        R_AARCH64_TLSDESC => "AARCH64_TLSDESC",
        R_AARCH64_IRELATIVE => "AARCH64_IRELATIVE",
         _ => "R_UNKNOWN_AARCH64",
        }},
        // arm
        EM_ARM => { match typ {
        R_ARM_PC24 => "ARM_PC24",
        R_ARM_ABS32 => "ARM_ABS32",
        R_ARM_REL32 => "ARM_REL32",
        R_ARM_PC13 => "ARM_PC13",
        R_ARM_ABS16 => "ARM_ABS16",
        R_ARM_ABS12 => "ARM_ABS12",
        R_ARM_THM_ABS5 => "ARM_THM_ABS5",
        R_ARM_ABS8 => "ARM_ABS8",
        R_ARM_SBREL32 => "ARM_SBREL32",
        R_ARM_THM_PC22 => "ARM_THM_PC22",
        R_ARM_THM_PC8 => "ARM_THM_PC8",
        R_ARM_AMP_VCALL9 => "ARM_AMP_VCALL9",
        R_ARM_TLS_DESC => "ARM_TLS_DESC",
        R_ARM_THM_SWI8 => "ARM_THM_SWI8",
        R_ARM_XPC25 => "ARM_XPC25",
        R_ARM_THM_XPC22 => "ARM_THM_XPC22",
        R_ARM_TLS_DTPMOD32 => "ARM_TLS_DTPMOD32",
        R_ARM_TLS_DTPOFF32 => "ARM_TLS_DTPOFF32",
        R_ARM_TLS_TPOFF32 => "ARM_TLS_TPOFF32",
        R_ARM_COPY => "ARM_COPY",
        R_ARM_GLOB_DAT => "ARM_GLOB_DAT",
        R_ARM_JUMP_SLOT => "ARM_JUMP_SLOT",
        R_ARM_RELATIVE => "ARM_RELATIVE",
        R_ARM_GOTOFF => "ARM_GOTOFF",
        R_ARM_GOTPC => "ARM_GOTPC",
        R_ARM_GOT32 => "ARM_GOT32",
        R_ARM_PLT32 => "ARM_PLT32",
        R_ARM_CALL => "ARM_CALL",
        R_ARM_JUMP24 => "ARM_JUMP24",
        R_ARM_THM_JUMP24 => "ARM_THM_JUMP24",
        R_ARM_BASE_ABS => "ARM_BASE_ABS",
        R_ARM_ALU_PCREL_7_0 => "ARM_ALU_PCREL_7_0",
        R_ARM_ALU_PCREL_15_8 => "ARM_ALU_PCREL_15_8",
        R_ARM_ALU_PCREL_23_15 => "ARM_ALU_PCREL_23_15",
        R_ARM_LDR_SBREL_11_0 => "ARM_LDR_SBREL_11_0",
        R_ARM_ALU_SBREL_19_12 => "ARM_ALU_SBREL_19_12",
        R_ARM_ALU_SBREL_27_20 => "ARM_ALU_SBREL_27_20",
        R_ARM_TARGET1 => "ARM_TARGET1",
        R_ARM_SBREL31 => "ARM_SBREL31",
        R_ARM_V4BX => "ARM_V4BX",
        R_ARM_TARGET2 => "ARM_TARGET2",
        R_ARM_PREL31 => "ARM_PREL31",
        R_ARM_MOVW_ABS_NC => "ARM_MOVW_ABS_NC",
        R_ARM_MOVT_ABS => "ARM_MOVT_ABS",
        R_ARM_MOVW_PREL_NC => "ARM_MOVW_PREL_NC",
        R_ARM_MOVT_PREL => "ARM_MOVT_PREL",
        R_ARM_THM_MOVW_ABS_NC => "ARM_THM_MOVW_ABS_NC",
        R_ARM_THM_MOVT_ABS => "ARM_THM_MOVT_ABS",
        R_ARM_THM_MOVW_PREL_NC => "ARM_THM_MOVW_PREL_NC",
        R_ARM_THM_MOVT_PREL => "ARM_THM_MOVT_PREL",
        R_ARM_THM_JUMP19 => "ARM_THM_JUMP19",
        R_ARM_THM_JUMP6 => "ARM_THM_JUMP6",
        R_ARM_THM_ALU_PREL_11_0 => "ARM_THM_ALU_PREL_11_0",
        R_ARM_THM_PC12 => "ARM_THM_PC12",
        R_ARM_ABS32_NOI => "ARM_ABS32_NOI",
        R_ARM_REL32_NOI => "ARM_REL32_NOI",
        R_ARM_ALU_PC_G0_NC => "ARM_ALU_PC_G0_NC",
        R_ARM_ALU_PC_G0 => "ARM_ALU_PC_G0",
        R_ARM_ALU_PC_G1_NC => "ARM_ALU_PC_G1_NC",
        R_ARM_ALU_PC_G1 => "ARM_ALU_PC_G1",
        R_ARM_ALU_PC_G2 => "ARM_ALU_PC_G2",
        R_ARM_LDR_PC_G1 => "ARM_LDR_PC_G1",
        R_ARM_LDR_PC_G2 => "ARM_LDR_PC_G2",
        R_ARM_LDRS_PC_G0 => "ARM_LDRS_PC_G0",
        R_ARM_LDRS_PC_G1 => "ARM_LDRS_PC_G1",
        R_ARM_LDRS_PC_G2 => "ARM_LDRS_PC_G2",
        R_ARM_LDC_PC_G0 => "ARM_LDC_PC_G0",
        R_ARM_LDC_PC_G1 => "ARM_LDC_PC_G1",
        R_ARM_LDC_PC_G2 => "ARM_LDC_PC_G2",
        R_ARM_ALU_SB_G0_NC => "ARM_ALU_SB_G0_NC",
        R_ARM_ALU_SB_G0 => "ARM_ALU_SB_G0",
        R_ARM_ALU_SB_G1_NC => "ARM_ALU_SB_G1_NC",
        R_ARM_ALU_SB_G1 => "ARM_ALU_SB_G1",
        R_ARM_ALU_SB_G2 => "ARM_ALU_SB_G2",
        R_ARM_LDR_SB_G0 => "ARM_LDR_SB_G0",
        R_ARM_LDR_SB_G1 => "ARM_LDR_SB_G1",
        R_ARM_LDR_SB_G2 => "ARM_LDR_SB_G2",
        R_ARM_LDRS_SB_G0 => "ARM_LDRS_SB_G0",
        R_ARM_LDRS_SB_G1 => "ARM_LDRS_SB_G1",
        R_ARM_LDRS_SB_G2 => "ARM_LDRS_SB_G2",
        R_ARM_LDC_SB_G0 => "ARM_LDC_SB_G0",
        R_ARM_LDC_SB_G1 => "ARM_LDC_SB_G1",
        R_ARM_LDC_SB_G2 => "ARM_LDC_SB_G2",
        R_ARM_MOVW_BREL_NC => "ARM_MOVW_BREL_NC",
        R_ARM_MOVT_BREL => "ARM_MOVT_BREL",
        R_ARM_MOVW_BREL => "ARM_MOVW_BREL",
        R_ARM_THM_MOVW_BREL_NC => "ARM_THM_MOVW_BREL_NC",
        R_ARM_THM_MOVT_BREL => "ARM_THM_MOVT_BREL",
        R_ARM_THM_MOVW_BREL => "ARM_THM_MOVW_BREL",
        R_ARM_TLS_GOTDESC => "ARM_TLS_GOTDESC",
        R_ARM_TLS_CALL => "ARM_TLS_CALL",
        R_ARM_TLS_DESCSEQ => "ARM_TLS_DESCSEQ",
        R_ARM_THM_TLS_CALL => "ARM_THM_TLS_CALL",
        R_ARM_PLT32_ABS => "ARM_PLT32_ABS",
        R_ARM_GOT_ABS => "ARM_GOT_ABS",
        R_ARM_GOT_PREL => "ARM_GOT_PREL",
        R_ARM_GOT_BREL12 => "ARM_GOT_BREL12",
        R_ARM_GOTOFF12 => "ARM_GOTOFF12",
        R_ARM_GOTRELAX => "ARM_GOTRELAX",
        R_ARM_GNU_VTENTRY => "ARM_GNU_VTENTRY",
        R_ARM_GNU_VTINHERIT => "ARM_GNU_VTINHERIT",
        R_ARM_THM_PC11 => "ARM_THM_PC11",
        R_ARM_THM_PC9 => "ARM_THM_PC9",
        R_ARM_TLS_GD32 => "ARM_TLS_GD32",
        R_ARM_TLS_LDM32 => "ARM_TLS_LDM32",
        R_ARM_TLS_LDO32 => "ARM_TLS_LDO32",
        R_ARM_TLS_IE32 => "ARM_TLS_IE32",
        R_ARM_TLS_LE32 => "ARM_TLS_LE32",
        R_ARM_TLS_LDO12 => "ARM_TLS_LDO12",
        R_ARM_TLS_LE12 => "ARM_TLS_LE12",
        R_ARM_TLS_IE12GP => "ARM_TLS_IE12GP",
        R_ARM_ME_TOO => "ARM_ME_TOO",
        R_ARM_THM_TLS_DESCSEQ16 => "ARM_THM_TLS_DESCSEQ16",
        R_ARM_THM_TLS_DESCSEQ32 => "ARM_THM_TLS_DESCSEQ32",
        R_ARM_THM_GOT_BREL12 => "ARM_THM_GOT_BREL12",
        R_ARM_IRELATIVE => "ARM_IRELATIVE",
        R_ARM_RXPC25 => "ARM_RXPC25",
        R_ARM_RSBREL32 => "ARM_RSBREL32",
        R_ARM_THM_RPC22 => "ARM_THM_RPC22",
        R_ARM_RREL32 => "ARM_RREL32",
        R_ARM_RABS22 => "ARM_RABS22",
        R_ARM_RPC24 => "ARM_RPC24",
        R_ARM_RBASE => "ARM_RBASE",
         _ => "R_UNKNOWN_ARM",
        }},
        // MIPS
        EM_MIPS | EM_MIPS_RS3_LE | EM_MIPS_X => { match typ {
        R_MIPS_NONE => "R_MIPS_NONE",
        R_MIPS_16 => "R_MIPS_16",
        R_MIPS_32 => "R_MIPS_32",
        R_MIPS_REL32 => "R_MIPS_REL32",
        R_MIPS_26 => "R_MIPS_26",
        R_MIPS_HI16 => "R_MIPS_HI16",
        R_MIPS_LO16 => "R_MIPS_LO16",
        R_MIPS_GPREL16 => "R_MIPS_GPREL16",
        R_MIPS_LITERAL => "R_MIPS_LITERAL",
        R_MIPS_GOT16 => "R_MIPS_GOT16",
        R_MIPS_PC16 => "R_MIPS_PC16",
        R_MIPS_CALL16 => "R_MIPS_CALL16",
        R_MIPS_GPREL32 => "R_MIPS_GPREL32",
        R_MIPS_SHIFT5 => "R_MIPS_SHIFT5",
        R_MIPS_SHIFT6 => "R_MIPS_SHIFT6",
        R_MIPS_64 => "R_MIPS_64",
        R_MIPS_GOT_DISP => "R_MIPS_GOT_DISP",
        R_MIPS_GOT_PAGE => "R_MIPS_GOT_PAGE",
        R_MIPS_GOT_OFST => "R_MIPS_GOT_OFST",
        R_MIPS_GOT_HI16 => "R_MIPS_GOT_HI16",
        R_MIPS_GOT_LO16 => "R_MIPS_GOT_LO16",
        R_MIPS_SUB => "R_MIPS_SUB",
        R_MIPS_INSERT_A => "R_MIPS_INSERT_A",
        R_MIPS_INSERT_B => "R_MIPS_INSERT_B",
        R_MIPS_DELETE => "R_MIPS_DELETE",
        R_MIPS_HIGHER => "R_MIPS_HIGHER",
        R_MIPS_HIGHEST => "R_MIPS_HIGHEST",
        R_MIPS_CALL_HI16 => "R_MIPS_CALL_HI16",
        R_MIPS_CALL_LO16 => "R_MIPS_CALL_LO16",
        R_MIPS_SCN_DISP => "R_MIPS_SCN_DISP",
        R_MIPS_REL16 => "R_MIPS_REL16",
        R_MIPS_ADD_IMMEDIATE => "R_MIPS_ADD_IMMEDIATE",
        R_MIPS_PJUMP => "R_MIPS_PJUMP",
        R_MIPS_RELGOT => "R_MIPS_RELGOT",
        R_MIPS_JALR => "R_MIPS_JALR",
        R_MIPS_TLS_DTPMOD32 => "R_MIPS_TLS_DTPMOD32",
        R_MIPS_TLS_DTPREL32 => "R_MIPS_TLS_DTPREL32",
        R_MIPS_TLS_DTPMOD64 => "R_MIPS_TLS_DTPMOD64",
        R_MIPS_TLS_DTPREL64 => "R_MIPS_TLS_DTPREL64",
        R_MIPS_TLS_GD => "R_MIPS_TLS_GD",
        R_MIPS_TLS_LDM => "R_MIPS_TLS_LDM",
        R_MIPS_TLS_DTPREL_HI16 => "R_MIPS_TLS_DTPREL_HI16",
        R_MIPS_TLS_DTPREL_LO16 => "R_MIPS_TLS_DTPREL_LO16",
        R_MIPS_TLS_GOTTPREL => "R_MIPS_TLS_GOTTPREL",
        R_MIPS_TLS_TPREL32 => "R_MIPS_TLS_TPREL32",
        R_MIPS_TLS_TPREL64 => "R_MIPS_TLS_TPREL64",
        R_MIPS_TLS_TPREL_HI16 => "R_MIPS_TLS_TPREL_HI16",
        R_MIPS_TLS_TPREL_LO16 => "R_MIPS_TLS_TPREL_LO16",
        R_MIPS_GLOB_DAT => "R_MIPS_GLOB_DAT",
        R_MIPS_COPY => "R_MIPS_COPY",
        R_MIPS_JUMP_SLOT => "R_MIPS_JUMP_SLOT",
        _ => "R_UNKNOWN_MIPS",
        }},
        // RISC-V
        EM_RISCV => { match typ {
        R_RISCV_NONE => "R_RISCV_NONE",
        R_RISCV_32 => "R_RISCV_32",
        R_RISCV_64 => "R_RISCV_64",
        R_RISCV_RELATIVE => "R_RISCV_RELATIVE",
        R_RISCV_COPY => "R_RISCV_COPY",
        R_RISCV_JUMP_SLOT => "R_RISCV_JUMP_SLOT",
        R_RISCV_TLS_DTPMOD32 => "R_RISCV_TLS_DTPMOD32",
        R_RISCV_TLS_DTPMOD64 => "R_RISCV_TLS_DTPMOD64",
        R_RISCV_TLS_DTPREL32 => "R_RISCV_TLS_DTPREL32",
        R_RISCV_TLS_DTPREL64 => "R_RISCV_TLS_DTPREL64",
        R_RISCV_TLS_TPREL32 => "R_RISCV_TLS_TPREL32",
        R_RISCV_TLS_TPREL64 => "R_RISCV_TLS_TPREL64",
        R_RISCV_BRANCH => "R_RISCV_BRANCH",
        R_RISCV_JAL => "R_RISCV_JAL",
        R_RISCV_CALL => "R_RISCV_CALL",
        R_RISCV_CALL_PLT => "R_RISCV_CALL_PLT",
        R_RISCV_GOT_HI20 => "R_RISCV_GOT_HI20",
        R_RISCV_TLS_GOT_HI20 => "R_RISCV_TLS_GOT_HI20",
        R_RISCV_TLS_GD_HI20 => "R_RISCV_TLS_GD_HI20",
        R_RISCV_PCREL_HI20 => "R_RISCV_PCREL_HI20",
        R_RISCV_PCREL_LO12_I => "R_RISCV_PCREL_LO12_I",
        R_RISCV_PCREL_LO12_S => "R_RISCV_PCREL_LO12_S",
        R_RISCV_HI20 => "R_RISCV_HI20",
        R_RISCV_LO12_I => "R_RISCV_LO12_I",
        R_RISCV_LO12_S => "R_RISCV_LO12_S",
        R_RISCV_TPREL_HI20 => "R_RISCV_TPREL_HI20",
        R_RISCV_TPREL_LO12_I => "R_RISCV_TPREL_LO12_I",
        R_RISCV_TPREL_LO12_S => "R_RISCV_TPREL_LO12_S",
        R_RISCV_TPREL_ADD => "R_RISCV_TPREL_ADD",
        R_RISCV_ADD8 => "R_RISCV_ADD8",
        R_RISCV_ADD16 => "R_RISCV_ADD16",
        R_RISCV_ADD32 => "R_RISCV_ADD32",
        R_RISCV_ADD64 => "R_RISCV_ADD64",
        R_RISCV_SUB8 => "R_RISCV_SUB8",
        R_RISCV_SUB16 => "R_RISCV_SUB16",
        R_RISCV_SUB32 => "R_RISCV_SUB32",
        R_RISCV_SUB64 => "R_RISCV_SUB64",
        R_RISCV_GNU_VTINHERIT => "R_RISCV_GNU_VTINHERIT",
        R_RISCV_GNU_VTENTRY => "R_RISCV_GNU_VTENTRY",
        R_RISCV_ALIGN => "R_RISCV_ALIGN",
        R_RISCV_RVC_BRANCH => "R_RISCV_RVC_BRANCH",
        R_RISCV_RVC_JUMP => "R_RISCV_RVC_JUMP",
        R_RISCV_RVC_LUI => "R_RISCV_RVC_LUI",
        R_RISCV_GPREL_I => "R_RISCV_GPREL_I",
        R_RISCV_GPREL_S => "R_RISCV_GPREL_S",
        R_RISCV_TPREL_I => "R_RISCV_TPREL_I",
        R_RISCV_TPREL_S => "R_RISCV_TPREL_S",
        R_RISCV_RELAX => "R_RISCV_RELAX",
        R_RISCV_SUB6 => "R_RISCV_SUB6",
        R_RISCV_SET6 => "R_RISCV_SET6",
        R_RISCV_SET8 => "R_RISCV_SET8",
        R_RISCV_SET16 => "R_RISCV_SET16",
        R_RISCV_SET32 => "R_RISCV_SET32",
        _ => "R_UNKNOWN_RISCV",
        }},
        // Power-PC
        EM_PPC => { match typ {
        R_PPC_NONE => "R_PPC_NONE",
        R_PPC_ADDR32 => "R_PPC_ADDR32",
        R_PPC_ADDR24 => "R_PPC_ADDR24",
        R_PPC_ADDR16 => "R_PPC_ADDR16",
        R_PPC_ADDR16_LO => "R_PPC_ADDR16_LO",
        R_PPC_ADDR16_HI => "R_PPC_ADDR16_HI",
        R_PPC_ADDR16_HA => "R_PPC_ADDR16_HA",
        R_PPC_ADDR14 => "R_PPC_ADDR14",
        R_PPC_ADDR14_BRTAKEN => "R_PPC_ADDR14_BRTAKEN",
        R_PPC_ADDR14_BRNTAKEN => "R_PPC_ADDR14_BRNTAKEN",
        R_PPC_REL24 => "R_PPC_REL24",
        R_PPC_REL14 => "R_PPC_REL14",
        R_PPC_REL14_BRTAKEN => "R_PPC_REL14_BRTAKEN",
        R_PPC_REL14_BRNTAKEN => "R_PPC_REL14_BRNTAKEN",
        R_PPC_GOT16 => "R_PPC_GOT16",
        R_PPC_GOT16_LO => "R_PPC_GOT16_LO",
        R_PPC_GOT16_HI => "R_PPC_GOT16_HI",
        R_PPC_GOT16_HA => "R_PPC_GOT16_HA",
        R_PPC_PLTREL24 => "R_PPC_PLTREL24",
        R_PPC_COPY => "R_PPC_COPY",
        R_PPC_GLOB_DAT => "R_PPC_GLOB_DAT",
        R_PPC_JMP_SLOT => "R_PPC_JMP_SLOT",
        R_PPC_RELATIVE => "R_PPC_RELATIVE",
        R_PPC_LOCAL24PC => "R_PPC_LOCAL24PC",
        R_PPC_UADDR32 => "R_PPC_UADDR32",
        R_PPC_UADDR16 => "R_PPC_UADDR16",
        R_PPC_REL32 => "R_PPC_REL32",
        R_PPC_PLT32 => "R_PPC_PLT32",
        R_PPC_PLTREL32 => "R_PPC_PLTREL32",
        R_PPC_PLT16_LO => "R_PPC_PLT16_LO",
        R_PPC_PLT16_HI => "R_PPC_PLT16_HI",
        R_PPC_PLT16_HA => "R_PPC_PLT16_HA",
        R_PPC_SDAREL16 => "R_PPC_SDAREL16",
        R_PPC_SECTOFF => "R_PPC_SECTOFF",
        R_PPC_SECTOFF_LO => "R_PPC_SECTOFF_LO",
        R_PPC_SECTOFF_HI => "R_PPC_SECTOFF_HI",
        R_PPC_SECTOFF_HA => "R_PPC_SECTOFF_HA",
        R_PPC_ADDR30 => "R_PPC_ADDR30",
        R_PPC_EMB_NADDR32 => "R_PPC_EMB_NADDR32",
        R_PPC_EMB_NADDR16 => "R_PPC_EMB_NADDR16",
        R_PPC_EMB_NADDR16_LO => "R_PPC_EMB_NADDR16_LO",
        R_PPC_EMB_NADDR16_HI => "R_PPC_EMB_NADDR16_HI",
        R_PPC_EMB_NADDR16_HA => "R_PPC_EMB_NADDR16_HA",
        R_PPC_EMB_SDA_I16 => "R_PPC_EMB_SDA_I16",
        R_PPC_EMB_SDA2_I16 => "R_PPC_EMB_SDA2_I16",
        R_PPC_EMB_SDA2REL => "R_PPC_EMB_SDA2REL",
        R_PPC_EMB_SDA21 => "R_PPC_EMB_SDA21",
        R_PPC_EMB_MRKREF => "R_PPC_EMB_MRKREF",
        R_PPC_EMB_RELSEC16 => "R_PPC_EMB_RELSEC16",
        R_PPC_EMB_RELST_LO => "R_PPC_EMB_RELST_LO",
        R_PPC_EMB_RELST_HI => "R_PPC_EMB_RELST_HI",
        R_PPC_EMB_RELST_HA => "R_PPC_EMB_RELST_HA",
        R_PPC_EMB_BIT_FLD => "R_PPC_EMB_BIT_FLD",
        R_PPC_EMB_RELSDA => "R_PPC_EMB_RELSDA",
        R_PPC_EMB_RELOC_120 => "R_PPC_EMB_RELOC_120",
        R_PPC_EMB_RELOC_121 => "R_PPC_EMB_RELOC_121",
        R_PPC_DIAB_SDA21_LO => "R_PPC_DIAB_SDA21_LO",
        R_PPC_DIAB_SDA21_HI => "R_PPC_DIAB_SDA21_HI",
        R_PPC_DIAB_SDA21_HA => "R_PPC_DIAB_SDA21_HA",
        R_PPC_DIAB_RELSDA_LO => "R_PPC_DIAB_RELSDA_LO",
        R_PPC_DIAB_RELSDA_HI => "R_PPC_DIAB_RELSDA_HI",
        R_PPC_DIAB_RELSDA_HA => "R_PPC_DIAB_RELSDA_HA",
        R_PPC_EMB_SPE_DOUBLE => "R_PPC_EMB_SPE_DOUBLE",
        R_PPC_EMB_SPE_WORD => "R_PPC_EMB_SPE_WORD",
        R_PPC_EMB_SPE_HALF => "R_PPC_EMB_SPE_HALF",
        R_PPC_EMB_SPE_DOUBLE_SDAREL => "R_PPC_EMB_SPE_DOUBLE_SDAREL",
        R_PPC_EMB_SPE_WORD_SDAREL => "R_PPC_EMB_SPE_WORD_SDAREL",
        R_PPC_EMB_SPE_HALF_SDAREL => "R_PPC_EMB_SPE_HALF_SDAREL",
        R_PPC_EMB_SPE_DOUBLE_SDA2REL => "R_PPC_EMB_SPE_DOUBLE_SDA2REL",
        R_PPC_EMB_SPE_WORD_SDA2REL => "R_PPC_EMB_SPE_WORD_SDA2REL",
        R_PPC_EMB_SPE_HALF_SDA2REL => "R_PPC_EMB_SPE_HALF_SDA2REL",
        R_PPC_EMB_SPE_DOUBLE_SDA0REL => "R_PPC_EMB_SPE_DOUBLE_SDA0REL",
        R_PPC_EMB_SPE_WORD_SDA0REL => "R_PPC_EMB_SPE_WORD_SDA0REL",
        R_PPC_EMB_SPE_HALF_SDA0REL => "R_PPC_EMB_SPE_HALF_SDA0REL",
        R_PPC_EMB_SPE_DOUBLE_SDA => "R_PPC_EMB_SPE_DOUBLE_SDA",
        R_PPC_EMB_SPE_WORD_SDA => "R_PPC_EMB_SPE_WORD_SDA",
        R_PPC_EMB_SPE_HALF_SDA => "R_PPC_EMB_SPE_HALF_SDA",
        _ => "R_UNKNOWN_PPC",
        }},
        _ => "R_UNKNOWN",
    }
}
