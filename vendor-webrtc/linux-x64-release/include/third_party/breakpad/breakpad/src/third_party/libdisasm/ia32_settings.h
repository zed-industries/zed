#ifndef IA32_SETTINGS_H
#define IA32_SETTINGS_H

#include "libdis.h"

typedef struct {
	/* options */
	unsigned char endian,		/* 0 = big, 1 = little */
		      wc_byte,		/* wildcard byte */
		      max_insn,		/* max insn size */
		      sz_addr,		/* default address size */
		      sz_oper,		/* default operand size */
		      sz_byte,		/* # bits in byte */
		      sz_word,		/* # bytes in machine word */
		      sz_dword;		/* # bytes in machine dword */
	unsigned int id_sp_reg,		/* id of stack pointer */
		     id_fp_reg,		/* id of frame pointer */
		     id_ip_reg,		/* id of instruction pointer */
		     id_flag_reg,	/* id of flags register */
		     offset_gen_regs,	/* start of general regs */
		     offset_seg_regs,	/* start of segment regs */
		     offset_fpu_regs;	/* start of floating point regs */
	/* user-controlled settings */
	enum x86_options options;
} ia32_settings_t;

#endif
