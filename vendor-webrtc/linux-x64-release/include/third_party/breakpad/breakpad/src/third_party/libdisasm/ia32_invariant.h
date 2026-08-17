#ifndef IA32_INVARIANT_H
#define IA32_INVARIANT_H

#include "libdis.h"

size_t ia32_disasm_invariant( unsigned char *buf, size_t buf_len, 
		x86_invariant_t *inv);

size_t ia32_disasm_size( unsigned char *buf, size_t buf_len );

#endif
