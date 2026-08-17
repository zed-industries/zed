#ifndef IA32_MODRM_H
#define IA32_MODRM_H

#include "libdis.h"
#include "ia32_insn.h"

size_t ia32_modrm_decode( unsigned char *buf, unsigned int buf_len,
			    x86_op_t *op, x86_insn_t *insn,
			    size_t gen_regs );

void ia32_reg_decode( unsigned char byte, x86_op_t *op, size_t gen_regs );

#endif
