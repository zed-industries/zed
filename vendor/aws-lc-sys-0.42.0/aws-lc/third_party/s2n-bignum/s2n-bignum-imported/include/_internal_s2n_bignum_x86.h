// Auto-added during import for AWS-LC symbol prefixing support
#include <openssl/boringssl_prefix_symbols_asm.h>

#ifdef __APPLE__
#   define S2N_BN_SYMBOL(NAME) _##NAME
#else
#   define S2N_BN_SYMBOL(name) name
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

// Enable indirect branch tracking support unless explicitly disabled
// with -DNO_IBT. If the platform supports CET, simply inherit this from
// the usual header. Otherwise manually define _CET_ENDBR, used at each
// x86 entry point, to be the ENDBR64 instruction, with an explicit byte
// sequence for compilers/assemblers that don't know about it. Note that
// it is safe to use ENDBR64 on all platforms, since the encoding is by
// design interpreted as a NOP on all pre-CET x86_64 processors. The only
// downside is a small increase in code size and potentially a modest
// slowdown from executing one more instruction.

#if NO_IBT
#   if defined(_CET_ENDBR)
#     error "The s2n-bignum build option NO_IBT was configured, but _CET_ENDBR is defined in this compilation unit. That is weird, so failing the build."
#   endif
#   define _CET_ENDBR
#elif defined(__CET__)
#   include <cet.h>
#elif !defined(_CET_ENDBR)
#   define _CET_ENDBR .byte 0xf3,0x0f,0x1e,0xfa
#endif

// Variants of instructions including CFI (call frame information) annotations

#define CFI_START .cfi_startproc
#define CFI_RET ret ; .cfi_endproc

#define CFI_CALL(target) call    target

#define CFI_PUSH(reg) push    reg ; .cfi_adjust_cfa_offset 8 ; .cfi_rel_offset reg, 0
#define CFI_POP(reg) pop     reg ; .cfi_adjust_cfa_offset -8 ; .cfi_restore reg

#define CFI_INC_RSP(offset) add     rsp, offset ; .cfi_adjust_cfa_offset -offset
#define CFI_DEC_RSP(offset) sub     rsp, offset ; .cfi_adjust_cfa_offset offset

#define CFI_STACKSAVE(reg,offset) mov [rsp+(offset)], reg ; .cfi_rel_offset reg, offset
#define CFI_STACKLOAD(reg,offset) mov reg, [rsp+(offset)] ; .cfi_restore reg
#define CFI_STACKSAVEU(reg,offset) movups [rsp+(offset)], reg ; .cfi_rel_offset reg, offset
#define CFI_STACKLOADU(reg,offset) movups reg, [rsp+(offset)] ; .cfi_restore reg
