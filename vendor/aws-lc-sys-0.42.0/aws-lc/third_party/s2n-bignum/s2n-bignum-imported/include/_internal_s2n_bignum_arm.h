// Auto-added during import for AWS-LC symbol prefixing support
#include <openssl/boringssl_prefix_symbols_asm.h>

#ifdef __APPLE__
#   define S2N_BN_SYMBOL(NAME) _##NAME
#   if defined(__AARCH64EL__) || defined(__ARMEL__)
#     define __LF %%
#   else
#     define __LF ;
#   endif
#else
#   define S2N_BN_SYMBOL(name) name
#   define __LF ;
#endif

#define S2N_BN_SYM_VISIBILITY_DIRECTIVE(name) .globl S2N_BN_SYMBOL(name)

#ifdef S2N_BN_HIDE_SYMBOLS
#   ifdef __APPLE__
#      define S2N_BN_SYM_PRIVACY_DIRECTIVE(name) .private_extern S2N_BN_SYMBOL(name)
#   else
#      define S2N_BN_SYM_PRIVACY_DIRECTIVE(name) .hidden S2N_BN_SYMBOL(name)
#   endif
#else
#   define S2N_BN_SYM_PRIVACY_DIRECTIVE(name)  /* NO-OP: S2N_BN_SYM_PRIVACY_DIRECTIVE */
#endif

#ifdef __APPLE__
#   define S2N_BN_FUNCTION_TYPE_DIRECTIVE(name) /* Not used in Mach-O */
#else
#   define S2N_BN_FUNCTION_TYPE_DIRECTIVE(name) .type name, %function
#endif

#ifdef __APPLE__
#   define S2N_BN_SIZE_DIRECTIVE(name) /* Not used in Mach-O */
#else
#   define S2N_BN_SIZE_DIRECTIVE(name) .size S2N_BN_SYMBOL(name), .-S2N_BN_SYMBOL(name)
#endif

// Variants of instructions including CFI (call frame information) annotations

#define CFI_START .cfi_startproc
#define CFI_RET ret __LF .cfi_endproc

#define CFI_BL(target) bl target

#define CFI_PUSH2(lo,hi) stp     lo, hi, [sp, #-16]! __LF .cfi_adjust_cfa_offset 16 __LF .cfi_rel_offset lo, 0 __LF .cfi_rel_offset hi, 8
#define CFI_PUSH1Z(reg) stp     reg, xzr, [sp, #-16]! __LF .cfi_adjust_cfa_offset 16 __LF .cfi_rel_offset reg, 0

#define CFI_POP2(lo,hi) ldp     lo, hi, [sp], #16 __LF .cfi_adjust_cfa_offset -16 __LF .cfi_restore lo __LF .cfi_restore hi
#define CFI_POP1Z(reg) ldp     reg, xzr, [sp], #16 __LF .cfi_adjust_cfa_offset -16 __LF .cfi_restore reg

#define CFI_STACKSAVE2(lo,hi,offset) stp     lo, hi, [sp, #(offset)] __LF .cfi_rel_offset lo, offset __LF .cfi_rel_offset hi, offset+8

// This is an alternative to CFI_STACKSAVE2 to work around delocator problems
// in the AWS-LC FIPS build, avoiding certain composite expressions. It is
// expected that offset8 = offset+8 as in an invocation of CFI_STACKSAVE2.
// Likewise the (offset+0) oddities in the following macros are driven by
// delocator problems.

#define CFI_STACKSAVE2X(lo,hi,offset,offset8) stp     lo, hi, [sp, #(offset+0)] __LF .cfi_rel_offset lo, offset __LF .cfi_rel_offset hi, offset8

#define CFI_STACKSAVE1Z(reg,offset) stp     reg, xzr, [sp, #(offset+0)] __LF .cfi_rel_offset reg, offset

#define CFI_STACKLOAD2(lo,hi,offset) ldp     lo, hi, [sp, #(offset+0)] __LF .cfi_restore lo __LF .cfi_restore hi
#define CFI_STACKLOAD1Z(reg,offset) ldp     reg, xzr, [sp, #(offset+0)] __LF .cfi_restore reg

// It would be better to use -(offset) not -offset, but again there seem
// to be delocator issues. We adopt a discipline of not using dangerous
// composite expressions in this macro, e.g. parenthesizing the argument
// or just using numeric constants or products where the association is
// not a problem.

#define CFI_INC_SP(offset) add     sp, sp, #(offset+0) __LF .cfi_adjust_cfa_offset -offset
#define CFI_DEC_SP(offset) sub     sp, sp, #(offset+0) __LF .cfi_adjust_cfa_offset offset
