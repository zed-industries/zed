#ifndef IA32_REG_H
#define IA32_REG_H

#include <sys/types.h>	/* for size_t */
#include "libdis.h"	/* for x86_reg_t */

/* NOTE these are used in opcode tables for hard-coded registers */
#define REG_DWORD_OFFSET 	 1	/* 0 + 1 */
#define REG_ECX_INDEX		 2	/* 0 + 1 + 1 */
#define REG_ESP_INDEX		 5	/* 0 + 4 + 1 */
#define REG_EBP_INDEX		 6	/* 0 + 5 + 1 */
#define REG_ESI_INDEX		 7	/* 0 + 6 + 1 */
#define REG_EDI_INDEX		 8	/* 0 + 7 + 1 */
#define REG_WORD_OFFSET 	 9	/* 1 * 8 + 1 */
#define REG_BYTE_OFFSET 	17	/* 2 * 8 + 1 */
#define REG_MMX_OFFSET 		25	/* 3 * 8 + 1 */
#define REG_SIMD_OFFSET 	33	/* 4 * 8 + 1 */
#define REG_DEBUG_OFFSET 	41	/* 5 * 8 + 1 */
#define REG_CTRL_OFFSET 	49	/* 6 * 8 + 1 */
#define REG_TEST_OFFSET 	57	/* 7 * 8 + 1 */
#define REG_SEG_OFFSET 		65	/* 8 * 8 + 1 */
#define REG_LDTR_INDEX		71	/* 8 * 8 + 1 + 1 */
#define REG_GDTR_INDEX		72	/* 8 * 8 + 2 + 1 */
#define REG_FPU_OFFSET 		73	/* 9 * 8 + 1 */
#define REG_FLAGS_INDEX 	81	/* 10 * 8 + 1 */
#define REG_FPCTRL_INDEX 	82	/* 10 * 8 + 1 + 1 */
#define REG_FPSTATUS_INDEX 	83	/* 10 * 8 + 2 + 1 */
#define REG_FPTAG_INDEX 	84	/* 10 * 8 + 3 + 1 */
#define REG_EIP_INDEX 		85	/* 10 * 8 + 4 + 1 */
#define REG_IP_INDEX 		86	/* 10 * 8 + 5 + 1 */
#define REG_IDTR_INDEX		87	/* 10 * 8 + 6 + 1 */
#define REG_MXCSG_INDEX		88	/* 10 * 8 + 7 + 1 */
#define REG_TR_INDEX		89	/* 10 * 8 + 8 + 1 */
#define REG_CSMSR_INDEX		90	/* 10 * 8 + 9 + 1 */
#define REG_ESPMSR_INDEX	91	/* 10 * 8 + 10 + 1 */
#define REG_EIPMSR_INDEX	92	/* 10 * 8 + 11 + 1 */

void ia32_handle_register( x86_reg_t *reg, size_t id );
size_t ia32_true_register_id( size_t id );

#endif
