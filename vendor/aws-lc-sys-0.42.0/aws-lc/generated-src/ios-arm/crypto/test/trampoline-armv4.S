// This file is generated from a similarly-named Perl script in the BoringSSL
// source tree. Do not edit by hand.

#include <openssl/asm_base.h>

#if !defined(OPENSSL_NO_ASM) && defined(OPENSSL_ARM) && defined(__APPLE__)
.syntax	unified




.text

@ abi_test_trampoline loads callee-saved registers from |state|, calls |func|
@ with |argv|, then saves the callee-saved registers into |state|. It returns
@ the result of |func|. The |unwind| argument is unused.
@ uint32_t abi_test_trampoline(void (*func)(...), CallerState *state,
@                              const uint32_t *argv, size_t argc,
@                              int unwind);

.globl	_abi_test_trampoline
.private_extern	_abi_test_trampoline
.align	4
_abi_test_trampoline:
	@ Save parameters and all callee-saved registers. For convenience, we
	@ save r9 on iOS even though it's volatile.
	vstmdb	sp!, {d8,d9,d10,d11,d12,d13,d14,d15}
	stmdb	sp!, {r0,r1,r2,r3,r4,r5,r6,r7,r8,r9,r10,r11,lr}

	@ Reserve stack space for six (10-4) stack parameters, plus an extra 4
	@ bytes to keep it 8-byte-aligned (see AAPCS, section 5.3).
	sub	sp, sp, #28

	@ Every register in AAPCS is either non-volatile or a parameter (except
	@ r9 on iOS), so this code, by the actual call, loses all its scratch
	@ registers. First fill in stack parameters while there are registers
	@ to spare.
	cmp	r3, #4
	bls	Lstack_args_done
	mov	r4, sp				@ r4 is the output pointer.
	add	r5, r2, r3, lsl #2	@ Set r5 to the end of argv.
	add	r2, r2, #16		@ Skip four arguments.
Lstack_args_loop:
	ldr	r6, [r2], #4
	cmp	r2, r5
	str	r6, [r4], #4
	bne	Lstack_args_loop

Lstack_args_done:
	@ Load registers from |r1|.
	vldmia	r1!, {d8,d9,d10,d11,d12,d13,d14,d15}
#if defined(__APPLE__)
	@ r9 is not volatile on iOS.
	ldmia	r1!, {r4,r5,r6,r7,r8,r10-r11}
#else
	ldmia	r1!, {r4,r5,r6,r7,r8,r9,r10,r11}
#endif

	@ Load register parameters. This uses up our remaining registers, so we
	@ repurpose lr as scratch space.
	ldr	r3, [sp, #40]	@ Reload argc.
	ldr	lr, [sp, #36]		@ Load argv into lr.
	cmp	r3, #3
	bhi	Larg_r3
	beq	Larg_r2
	cmp	r3, #1
	bhi	Larg_r1
	beq	Larg_r0
	b	Largs_done

Larg_r3:
	ldr	r3, [lr, #12]	@ argv[3]
Larg_r2:
	ldr	r2, [lr, #8]	@ argv[2]
Larg_r1:
	ldr	r1, [lr, #4]	@ argv[1]
Larg_r0:
	ldr	r0, [lr]	@ argv[0]
Largs_done:

	@ With every other register in use, load the function pointer into lr
	@ and call the function.
	ldr	lr, [sp, #28]
	blx	lr

	@ r1-r3 are free for use again. The trampoline only supports
	@ single-return functions. Pass r4-r11 to the caller.
	ldr	r1, [sp, #32]
	vstmia	r1!, {d8,d9,d10,d11,d12,d13,d14,d15}
#if defined(__APPLE__)
	@ r9 is not volatile on iOS.
	stmia	r1!, {r4,r5,r6,r7,r8,r10-r11}
#else
	stmia	r1!, {r4,r5,r6,r7,r8,r9,r10,r11}
#endif

	@ Unwind the stack and restore registers.
	add	sp, sp, #44		@ 44 = 28+16
	ldmia	sp!, {r4,r5,r6,r7,r8,r9,r10,r11,lr}	@ Skip r0-r3 (see +16 above).
	vldmia	sp!, {d8,d9,d10,d11,d12,d13,d14,d15}

	bx	lr


.globl	_abi_test_clobber_r0
.private_extern	_abi_test_clobber_r0
.align	4
_abi_test_clobber_r0:
	mov	r0, #0
	bx	lr


.globl	_abi_test_clobber_r1
.private_extern	_abi_test_clobber_r1
.align	4
_abi_test_clobber_r1:
	mov	r1, #0
	bx	lr


.globl	_abi_test_clobber_r2
.private_extern	_abi_test_clobber_r2
.align	4
_abi_test_clobber_r2:
	mov	r2, #0
	bx	lr


.globl	_abi_test_clobber_r3
.private_extern	_abi_test_clobber_r3
.align	4
_abi_test_clobber_r3:
	mov	r3, #0
	bx	lr


.globl	_abi_test_clobber_r4
.private_extern	_abi_test_clobber_r4
.align	4
_abi_test_clobber_r4:
	mov	r4, #0
	bx	lr


.globl	_abi_test_clobber_r5
.private_extern	_abi_test_clobber_r5
.align	4
_abi_test_clobber_r5:
	mov	r5, #0
	bx	lr


.globl	_abi_test_clobber_r6
.private_extern	_abi_test_clobber_r6
.align	4
_abi_test_clobber_r6:
	mov	r6, #0
	bx	lr


.globl	_abi_test_clobber_r7
.private_extern	_abi_test_clobber_r7
.align	4
_abi_test_clobber_r7:
	mov	r7, #0
	bx	lr


.globl	_abi_test_clobber_r8
.private_extern	_abi_test_clobber_r8
.align	4
_abi_test_clobber_r8:
	mov	r8, #0
	bx	lr


.globl	_abi_test_clobber_r9
.private_extern	_abi_test_clobber_r9
.align	4
_abi_test_clobber_r9:
	mov	r9, #0
	bx	lr


.globl	_abi_test_clobber_r10
.private_extern	_abi_test_clobber_r10
.align	4
_abi_test_clobber_r10:
	mov	r10, #0
	bx	lr


.globl	_abi_test_clobber_r11
.private_extern	_abi_test_clobber_r11
.align	4
_abi_test_clobber_r11:
	mov	r11, #0
	bx	lr


.globl	_abi_test_clobber_r12
.private_extern	_abi_test_clobber_r12
.align	4
_abi_test_clobber_r12:
	mov	r12, #0
	bx	lr


.globl	_abi_test_clobber_d0
.private_extern	_abi_test_clobber_d0
.align	4
_abi_test_clobber_d0:
	mov	r0, #0
	vmov	s0, r0
	vmov	s1, r0
	bx	lr


.globl	_abi_test_clobber_d1
.private_extern	_abi_test_clobber_d1
.align	4
_abi_test_clobber_d1:
	mov	r0, #0
	vmov	s2, r0
	vmov	s3, r0
	bx	lr


.globl	_abi_test_clobber_d2
.private_extern	_abi_test_clobber_d2
.align	4
_abi_test_clobber_d2:
	mov	r0, #0
	vmov	s4, r0
	vmov	s5, r0
	bx	lr


.globl	_abi_test_clobber_d3
.private_extern	_abi_test_clobber_d3
.align	4
_abi_test_clobber_d3:
	mov	r0, #0
	vmov	s6, r0
	vmov	s7, r0
	bx	lr


.globl	_abi_test_clobber_d4
.private_extern	_abi_test_clobber_d4
.align	4
_abi_test_clobber_d4:
	mov	r0, #0
	vmov	s8, r0
	vmov	s9, r0
	bx	lr


.globl	_abi_test_clobber_d5
.private_extern	_abi_test_clobber_d5
.align	4
_abi_test_clobber_d5:
	mov	r0, #0
	vmov	s10, r0
	vmov	s11, r0
	bx	lr


.globl	_abi_test_clobber_d6
.private_extern	_abi_test_clobber_d6
.align	4
_abi_test_clobber_d6:
	mov	r0, #0
	vmov	s12, r0
	vmov	s13, r0
	bx	lr


.globl	_abi_test_clobber_d7
.private_extern	_abi_test_clobber_d7
.align	4
_abi_test_clobber_d7:
	mov	r0, #0
	vmov	s14, r0
	vmov	s15, r0
	bx	lr


.globl	_abi_test_clobber_d8
.private_extern	_abi_test_clobber_d8
.align	4
_abi_test_clobber_d8:
	mov	r0, #0
	vmov	s16, r0
	vmov	s17, r0
	bx	lr


.globl	_abi_test_clobber_d9
.private_extern	_abi_test_clobber_d9
.align	4
_abi_test_clobber_d9:
	mov	r0, #0
	vmov	s18, r0
	vmov	s19, r0
	bx	lr


.globl	_abi_test_clobber_d10
.private_extern	_abi_test_clobber_d10
.align	4
_abi_test_clobber_d10:
	mov	r0, #0
	vmov	s20, r0
	vmov	s21, r0
	bx	lr


.globl	_abi_test_clobber_d11
.private_extern	_abi_test_clobber_d11
.align	4
_abi_test_clobber_d11:
	mov	r0, #0
	vmov	s22, r0
	vmov	s23, r0
	bx	lr


.globl	_abi_test_clobber_d12
.private_extern	_abi_test_clobber_d12
.align	4
_abi_test_clobber_d12:
	mov	r0, #0
	vmov	s24, r0
	vmov	s25, r0
	bx	lr


.globl	_abi_test_clobber_d13
.private_extern	_abi_test_clobber_d13
.align	4
_abi_test_clobber_d13:
	mov	r0, #0
	vmov	s26, r0
	vmov	s27, r0
	bx	lr


.globl	_abi_test_clobber_d14
.private_extern	_abi_test_clobber_d14
.align	4
_abi_test_clobber_d14:
	mov	r0, #0
	vmov	s28, r0
	vmov	s29, r0
	bx	lr


.globl	_abi_test_clobber_d15
.private_extern	_abi_test_clobber_d15
.align	4
_abi_test_clobber_d15:
	mov	r0, #0
	vmov	s30, r0
	vmov	s31, r0
	bx	lr

#endif  // !OPENSSL_NO_ASM && defined(OPENSSL_ARM) && defined(__APPLE__)
