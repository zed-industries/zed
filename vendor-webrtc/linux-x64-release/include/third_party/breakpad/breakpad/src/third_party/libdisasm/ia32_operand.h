#ifndef IA32_OPERAND_H
#define IA32_OPERAND_H

#include "libdis.h"
#include "ia32_insn.h"

size_t ia32_decode_operand( unsigned char *buf, size_t buf_len,
			      x86_insn_t *insn, unsigned int raw_op, 
			      unsigned int raw_flags, unsigned int prefixes,
			      unsigned char modrm );
#endif
