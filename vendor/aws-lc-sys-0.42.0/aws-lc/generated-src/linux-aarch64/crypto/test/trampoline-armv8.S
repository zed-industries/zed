// This file is generated from a similarly-named Perl script in the BoringSSL
// source tree. Do not edit by hand.

#include <openssl/asm_base.h>

#if !defined(OPENSSL_NO_ASM) && defined(OPENSSL_AARCH64) && defined(__ELF__)
#include <openssl/arm_arch.h>

.text

// abi_test_trampoline loads callee-saved registers from |state|, calls |func|
// with |argv|, then saves the callee-saved registers into |state|. It returns
// the result of |func|. The |unwind| argument is unused.
// uint64_t abi_test_trampoline(void (*func)(...), CallerState *state,
//                              const uint64_t *argv, size_t argc,
//                              uint64_t unwind);
.type	abi_test_trampoline, %function
.globl	abi_test_trampoline
.hidden	abi_test_trampoline
.align	4
abi_test_trampoline:
.Labi_test_trampoline_begin:
	AARCH64_SIGN_LINK_REGISTER
	// Stack layout (low to high addresses)
	//   x29,x30 (16 bytes)
	//    d8-d15 (64 bytes)
	//   x19-x28 (80 bytes)
	//    x1 (8 bytes)
	//   padding (8 bytes)
	stp	x29, x30, [sp, #-176]!
	mov	x29, sp

	// Saved callee-saved registers and |state|.
	stp	d8, d9, [sp, #16]
	stp	d10, d11, [sp, #32]
	stp	d12, d13, [sp, #48]
	stp	d14, d15, [sp, #64]
	stp	x19, x20, [sp, #80]
	stp	x21, x22, [sp, #96]
	stp	x23, x24, [sp, #112]
	stp	x25, x26, [sp, #128]
	stp	x27, x28, [sp, #144]
	str	x1, [sp, #160]

	// Load registers from |state|, with the exception of x29. x29 is the
	// frame pointer and also callee-saved, but AAPCS64 allows platforms to
	// mandate that x29 always point to a frame. iOS64 does so, which means
	// we cannot fill x29 with entropy without violating ABI rules
	// ourselves. x29 is tested separately below.
	ldp	d8, d9, [x1], #16
	ldp	d10, d11, [x1], #16
	ldp	d12, d13, [x1], #16
	ldp	d14, d15, [x1], #16
	ldp	x19, x20, [x1], #16
	ldp	x21, x22, [x1], #16
	ldp	x23, x24, [x1], #16
	ldp	x25, x26, [x1], #16
	ldp	x27, x28, [x1], #16

	// Move parameters into temporary registers.
	mov	x9, x0
	mov	x10, x2
	mov	x11, x3

	// Load parameters into registers.
	cbz	x11, .Largs_done
	ldr	x0, [x10], #8
	subs	x11, x11, #1
	b.eq	.Largs_done
	ldr	x1, [x10], #8
	subs	x11, x11, #1
	b.eq	.Largs_done
	ldr	x2, [x10], #8
	subs	x11, x11, #1
	b.eq	.Largs_done
	ldr	x3, [x10], #8
	subs	x11, x11, #1
	b.eq	.Largs_done
	ldr	x4, [x10], #8
	subs	x11, x11, #1
	b.eq	.Largs_done
	ldr	x5, [x10], #8
	subs	x11, x11, #1
	b.eq	.Largs_done
	ldr	x6, [x10], #8
	subs	x11, x11, #1
	b.eq	.Largs_done
	ldr	x7, [x10], #8

.Largs_done:
	blr	x9

	// Reload |state| and store registers.
	ldr	x1, [sp, #160]
	stp	d8, d9, [x1], #16
	stp	d10, d11, [x1], #16
	stp	d12, d13, [x1], #16
	stp	d14, d15, [x1], #16
	stp	x19, x20, [x1], #16
	stp	x21, x22, [x1], #16
	stp	x23, x24, [x1], #16
	stp	x25, x26, [x1], #16
	stp	x27, x28, [x1], #16

	// |func| is required to preserve x29, the frame pointer. We cannot load
	// random values into x29 (see comment above), so compare it against the
	// expected value and zero the field of |state| if corrupted.
	mov	x9, sp
	cmp	x29, x9
	b.eq	.Lx29_ok
	str	xzr, [x1]

.Lx29_ok:
	// Restore callee-saved registers.
	ldp	d8, d9, [sp, #16]
	ldp	d10, d11, [sp, #32]
	ldp	d12, d13, [sp, #48]
	ldp	d14, d15, [sp, #64]
	ldp	x19, x20, [sp, #80]
	ldp	x21, x22, [sp, #96]
	ldp	x23, x24, [sp, #112]
	ldp	x25, x26, [sp, #128]
	ldp	x27, x28, [sp, #144]

	ldp	x29, x30, [sp], #176
	AARCH64_VALIDATE_LINK_REGISTER
	ret
.size	abi_test_trampoline,.-abi_test_trampoline
.type	abi_test_clobber_x0, %function
.globl	abi_test_clobber_x0
.hidden	abi_test_clobber_x0
.align	4
abi_test_clobber_x0:
	AARCH64_VALID_CALL_TARGET
	mov	x0, xzr
	ret
.size	abi_test_clobber_x0,.-abi_test_clobber_x0
.type	abi_test_clobber_x1, %function
.globl	abi_test_clobber_x1
.hidden	abi_test_clobber_x1
.align	4
abi_test_clobber_x1:
	AARCH64_VALID_CALL_TARGET
	mov	x1, xzr
	ret
.size	abi_test_clobber_x1,.-abi_test_clobber_x1
.type	abi_test_clobber_x2, %function
.globl	abi_test_clobber_x2
.hidden	abi_test_clobber_x2
.align	4
abi_test_clobber_x2:
	AARCH64_VALID_CALL_TARGET
	mov	x2, xzr
	ret
.size	abi_test_clobber_x2,.-abi_test_clobber_x2
.type	abi_test_clobber_x3, %function
.globl	abi_test_clobber_x3
.hidden	abi_test_clobber_x3
.align	4
abi_test_clobber_x3:
	AARCH64_VALID_CALL_TARGET
	mov	x3, xzr
	ret
.size	abi_test_clobber_x3,.-abi_test_clobber_x3
.type	abi_test_clobber_x4, %function
.globl	abi_test_clobber_x4
.hidden	abi_test_clobber_x4
.align	4
abi_test_clobber_x4:
	AARCH64_VALID_CALL_TARGET
	mov	x4, xzr
	ret
.size	abi_test_clobber_x4,.-abi_test_clobber_x4
.type	abi_test_clobber_x5, %function
.globl	abi_test_clobber_x5
.hidden	abi_test_clobber_x5
.align	4
abi_test_clobber_x5:
	AARCH64_VALID_CALL_TARGET
	mov	x5, xzr
	ret
.size	abi_test_clobber_x5,.-abi_test_clobber_x5
.type	abi_test_clobber_x6, %function
.globl	abi_test_clobber_x6
.hidden	abi_test_clobber_x6
.align	4
abi_test_clobber_x6:
	AARCH64_VALID_CALL_TARGET
	mov	x6, xzr
	ret
.size	abi_test_clobber_x6,.-abi_test_clobber_x6
.type	abi_test_clobber_x7, %function
.globl	abi_test_clobber_x7
.hidden	abi_test_clobber_x7
.align	4
abi_test_clobber_x7:
	AARCH64_VALID_CALL_TARGET
	mov	x7, xzr
	ret
.size	abi_test_clobber_x7,.-abi_test_clobber_x7
.type	abi_test_clobber_x8, %function
.globl	abi_test_clobber_x8
.hidden	abi_test_clobber_x8
.align	4
abi_test_clobber_x8:
	AARCH64_VALID_CALL_TARGET
	mov	x8, xzr
	ret
.size	abi_test_clobber_x8,.-abi_test_clobber_x8
.type	abi_test_clobber_x9, %function
.globl	abi_test_clobber_x9
.hidden	abi_test_clobber_x9
.align	4
abi_test_clobber_x9:
	AARCH64_VALID_CALL_TARGET
	mov	x9, xzr
	ret
.size	abi_test_clobber_x9,.-abi_test_clobber_x9
.type	abi_test_clobber_x10, %function
.globl	abi_test_clobber_x10
.hidden	abi_test_clobber_x10
.align	4
abi_test_clobber_x10:
	AARCH64_VALID_CALL_TARGET
	mov	x10, xzr
	ret
.size	abi_test_clobber_x10,.-abi_test_clobber_x10
.type	abi_test_clobber_x11, %function
.globl	abi_test_clobber_x11
.hidden	abi_test_clobber_x11
.align	4
abi_test_clobber_x11:
	AARCH64_VALID_CALL_TARGET
	mov	x11, xzr
	ret
.size	abi_test_clobber_x11,.-abi_test_clobber_x11
.type	abi_test_clobber_x12, %function
.globl	abi_test_clobber_x12
.hidden	abi_test_clobber_x12
.align	4
abi_test_clobber_x12:
	AARCH64_VALID_CALL_TARGET
	mov	x12, xzr
	ret
.size	abi_test_clobber_x12,.-abi_test_clobber_x12
.type	abi_test_clobber_x13, %function
.globl	abi_test_clobber_x13
.hidden	abi_test_clobber_x13
.align	4
abi_test_clobber_x13:
	AARCH64_VALID_CALL_TARGET
	mov	x13, xzr
	ret
.size	abi_test_clobber_x13,.-abi_test_clobber_x13
.type	abi_test_clobber_x14, %function
.globl	abi_test_clobber_x14
.hidden	abi_test_clobber_x14
.align	4
abi_test_clobber_x14:
	AARCH64_VALID_CALL_TARGET
	mov	x14, xzr
	ret
.size	abi_test_clobber_x14,.-abi_test_clobber_x14
.type	abi_test_clobber_x15, %function
.globl	abi_test_clobber_x15
.hidden	abi_test_clobber_x15
.align	4
abi_test_clobber_x15:
	AARCH64_VALID_CALL_TARGET
	mov	x15, xzr
	ret
.size	abi_test_clobber_x15,.-abi_test_clobber_x15
.type	abi_test_clobber_x16, %function
.globl	abi_test_clobber_x16
.hidden	abi_test_clobber_x16
.align	4
abi_test_clobber_x16:
	AARCH64_VALID_CALL_TARGET
	mov	x16, xzr
	ret
.size	abi_test_clobber_x16,.-abi_test_clobber_x16
.type	abi_test_clobber_x17, %function
.globl	abi_test_clobber_x17
.hidden	abi_test_clobber_x17
.align	4
abi_test_clobber_x17:
	AARCH64_VALID_CALL_TARGET
	mov	x17, xzr
	ret
.size	abi_test_clobber_x17,.-abi_test_clobber_x17
.type	abi_test_clobber_x19, %function
.globl	abi_test_clobber_x19
.hidden	abi_test_clobber_x19
.align	4
abi_test_clobber_x19:
	AARCH64_VALID_CALL_TARGET
	mov	x19, xzr
	ret
.size	abi_test_clobber_x19,.-abi_test_clobber_x19
.type	abi_test_clobber_x20, %function
.globl	abi_test_clobber_x20
.hidden	abi_test_clobber_x20
.align	4
abi_test_clobber_x20:
	AARCH64_VALID_CALL_TARGET
	mov	x20, xzr
	ret
.size	abi_test_clobber_x20,.-abi_test_clobber_x20
.type	abi_test_clobber_x21, %function
.globl	abi_test_clobber_x21
.hidden	abi_test_clobber_x21
.align	4
abi_test_clobber_x21:
	AARCH64_VALID_CALL_TARGET
	mov	x21, xzr
	ret
.size	abi_test_clobber_x21,.-abi_test_clobber_x21
.type	abi_test_clobber_x22, %function
.globl	abi_test_clobber_x22
.hidden	abi_test_clobber_x22
.align	4
abi_test_clobber_x22:
	AARCH64_VALID_CALL_TARGET
	mov	x22, xzr
	ret
.size	abi_test_clobber_x22,.-abi_test_clobber_x22
.type	abi_test_clobber_x23, %function
.globl	abi_test_clobber_x23
.hidden	abi_test_clobber_x23
.align	4
abi_test_clobber_x23:
	AARCH64_VALID_CALL_TARGET
	mov	x23, xzr
	ret
.size	abi_test_clobber_x23,.-abi_test_clobber_x23
.type	abi_test_clobber_x24, %function
.globl	abi_test_clobber_x24
.hidden	abi_test_clobber_x24
.align	4
abi_test_clobber_x24:
	AARCH64_VALID_CALL_TARGET
	mov	x24, xzr
	ret
.size	abi_test_clobber_x24,.-abi_test_clobber_x24
.type	abi_test_clobber_x25, %function
.globl	abi_test_clobber_x25
.hidden	abi_test_clobber_x25
.align	4
abi_test_clobber_x25:
	AARCH64_VALID_CALL_TARGET
	mov	x25, xzr
	ret
.size	abi_test_clobber_x25,.-abi_test_clobber_x25
.type	abi_test_clobber_x26, %function
.globl	abi_test_clobber_x26
.hidden	abi_test_clobber_x26
.align	4
abi_test_clobber_x26:
	AARCH64_VALID_CALL_TARGET
	mov	x26, xzr
	ret
.size	abi_test_clobber_x26,.-abi_test_clobber_x26
.type	abi_test_clobber_x27, %function
.globl	abi_test_clobber_x27
.hidden	abi_test_clobber_x27
.align	4
abi_test_clobber_x27:
	AARCH64_VALID_CALL_TARGET
	mov	x27, xzr
	ret
.size	abi_test_clobber_x27,.-abi_test_clobber_x27
.type	abi_test_clobber_x28, %function
.globl	abi_test_clobber_x28
.hidden	abi_test_clobber_x28
.align	4
abi_test_clobber_x28:
	AARCH64_VALID_CALL_TARGET
	mov	x28, xzr
	ret
.size	abi_test_clobber_x28,.-abi_test_clobber_x28
.type	abi_test_clobber_x29, %function
.globl	abi_test_clobber_x29
.hidden	abi_test_clobber_x29
.align	4
abi_test_clobber_x29:
	AARCH64_VALID_CALL_TARGET
	mov	x29, xzr
	ret
.size	abi_test_clobber_x29,.-abi_test_clobber_x29
.type	abi_test_clobber_d0, %function
.globl	abi_test_clobber_d0
.hidden	abi_test_clobber_d0
.align	4
abi_test_clobber_d0:
	AARCH64_VALID_CALL_TARGET
	fmov	d0, xzr
	ret
.size	abi_test_clobber_d0,.-abi_test_clobber_d0
.type	abi_test_clobber_d1, %function
.globl	abi_test_clobber_d1
.hidden	abi_test_clobber_d1
.align	4
abi_test_clobber_d1:
	AARCH64_VALID_CALL_TARGET
	fmov	d1, xzr
	ret
.size	abi_test_clobber_d1,.-abi_test_clobber_d1
.type	abi_test_clobber_d2, %function
.globl	abi_test_clobber_d2
.hidden	abi_test_clobber_d2
.align	4
abi_test_clobber_d2:
	AARCH64_VALID_CALL_TARGET
	fmov	d2, xzr
	ret
.size	abi_test_clobber_d2,.-abi_test_clobber_d2
.type	abi_test_clobber_d3, %function
.globl	abi_test_clobber_d3
.hidden	abi_test_clobber_d3
.align	4
abi_test_clobber_d3:
	AARCH64_VALID_CALL_TARGET
	fmov	d3, xzr
	ret
.size	abi_test_clobber_d3,.-abi_test_clobber_d3
.type	abi_test_clobber_d4, %function
.globl	abi_test_clobber_d4
.hidden	abi_test_clobber_d4
.align	4
abi_test_clobber_d4:
	AARCH64_VALID_CALL_TARGET
	fmov	d4, xzr
	ret
.size	abi_test_clobber_d4,.-abi_test_clobber_d4
.type	abi_test_clobber_d5, %function
.globl	abi_test_clobber_d5
.hidden	abi_test_clobber_d5
.align	4
abi_test_clobber_d5:
	AARCH64_VALID_CALL_TARGET
	fmov	d5, xzr
	ret
.size	abi_test_clobber_d5,.-abi_test_clobber_d5
.type	abi_test_clobber_d6, %function
.globl	abi_test_clobber_d6
.hidden	abi_test_clobber_d6
.align	4
abi_test_clobber_d6:
	AARCH64_VALID_CALL_TARGET
	fmov	d6, xzr
	ret
.size	abi_test_clobber_d6,.-abi_test_clobber_d6
.type	abi_test_clobber_d7, %function
.globl	abi_test_clobber_d7
.hidden	abi_test_clobber_d7
.align	4
abi_test_clobber_d7:
	AARCH64_VALID_CALL_TARGET
	fmov	d7, xzr
	ret
.size	abi_test_clobber_d7,.-abi_test_clobber_d7
.type	abi_test_clobber_d8, %function
.globl	abi_test_clobber_d8
.hidden	abi_test_clobber_d8
.align	4
abi_test_clobber_d8:
	AARCH64_VALID_CALL_TARGET
	fmov	d8, xzr
	ret
.size	abi_test_clobber_d8,.-abi_test_clobber_d8
.type	abi_test_clobber_d9, %function
.globl	abi_test_clobber_d9
.hidden	abi_test_clobber_d9
.align	4
abi_test_clobber_d9:
	AARCH64_VALID_CALL_TARGET
	fmov	d9, xzr
	ret
.size	abi_test_clobber_d9,.-abi_test_clobber_d9
.type	abi_test_clobber_d10, %function
.globl	abi_test_clobber_d10
.hidden	abi_test_clobber_d10
.align	4
abi_test_clobber_d10:
	AARCH64_VALID_CALL_TARGET
	fmov	d10, xzr
	ret
.size	abi_test_clobber_d10,.-abi_test_clobber_d10
.type	abi_test_clobber_d11, %function
.globl	abi_test_clobber_d11
.hidden	abi_test_clobber_d11
.align	4
abi_test_clobber_d11:
	AARCH64_VALID_CALL_TARGET
	fmov	d11, xzr
	ret
.size	abi_test_clobber_d11,.-abi_test_clobber_d11
.type	abi_test_clobber_d12, %function
.globl	abi_test_clobber_d12
.hidden	abi_test_clobber_d12
.align	4
abi_test_clobber_d12:
	AARCH64_VALID_CALL_TARGET
	fmov	d12, xzr
	ret
.size	abi_test_clobber_d12,.-abi_test_clobber_d12
.type	abi_test_clobber_d13, %function
.globl	abi_test_clobber_d13
.hidden	abi_test_clobber_d13
.align	4
abi_test_clobber_d13:
	AARCH64_VALID_CALL_TARGET
	fmov	d13, xzr
	ret
.size	abi_test_clobber_d13,.-abi_test_clobber_d13
.type	abi_test_clobber_d14, %function
.globl	abi_test_clobber_d14
.hidden	abi_test_clobber_d14
.align	4
abi_test_clobber_d14:
	AARCH64_VALID_CALL_TARGET
	fmov	d14, xzr
	ret
.size	abi_test_clobber_d14,.-abi_test_clobber_d14
.type	abi_test_clobber_d15, %function
.globl	abi_test_clobber_d15
.hidden	abi_test_clobber_d15
.align	4
abi_test_clobber_d15:
	AARCH64_VALID_CALL_TARGET
	fmov	d15, xzr
	ret
.size	abi_test_clobber_d15,.-abi_test_clobber_d15
.type	abi_test_clobber_d16, %function
.globl	abi_test_clobber_d16
.hidden	abi_test_clobber_d16
.align	4
abi_test_clobber_d16:
	AARCH64_VALID_CALL_TARGET
	fmov	d16, xzr
	ret
.size	abi_test_clobber_d16,.-abi_test_clobber_d16
.type	abi_test_clobber_d17, %function
.globl	abi_test_clobber_d17
.hidden	abi_test_clobber_d17
.align	4
abi_test_clobber_d17:
	AARCH64_VALID_CALL_TARGET
	fmov	d17, xzr
	ret
.size	abi_test_clobber_d17,.-abi_test_clobber_d17
.type	abi_test_clobber_d18, %function
.globl	abi_test_clobber_d18
.hidden	abi_test_clobber_d18
.align	4
abi_test_clobber_d18:
	AARCH64_VALID_CALL_TARGET
	fmov	d18, xzr
	ret
.size	abi_test_clobber_d18,.-abi_test_clobber_d18
.type	abi_test_clobber_d19, %function
.globl	abi_test_clobber_d19
.hidden	abi_test_clobber_d19
.align	4
abi_test_clobber_d19:
	AARCH64_VALID_CALL_TARGET
	fmov	d19, xzr
	ret
.size	abi_test_clobber_d19,.-abi_test_clobber_d19
.type	abi_test_clobber_d20, %function
.globl	abi_test_clobber_d20
.hidden	abi_test_clobber_d20
.align	4
abi_test_clobber_d20:
	AARCH64_VALID_CALL_TARGET
	fmov	d20, xzr
	ret
.size	abi_test_clobber_d20,.-abi_test_clobber_d20
.type	abi_test_clobber_d21, %function
.globl	abi_test_clobber_d21
.hidden	abi_test_clobber_d21
.align	4
abi_test_clobber_d21:
	AARCH64_VALID_CALL_TARGET
	fmov	d21, xzr
	ret
.size	abi_test_clobber_d21,.-abi_test_clobber_d21
.type	abi_test_clobber_d22, %function
.globl	abi_test_clobber_d22
.hidden	abi_test_clobber_d22
.align	4
abi_test_clobber_d22:
	AARCH64_VALID_CALL_TARGET
	fmov	d22, xzr
	ret
.size	abi_test_clobber_d22,.-abi_test_clobber_d22
.type	abi_test_clobber_d23, %function
.globl	abi_test_clobber_d23
.hidden	abi_test_clobber_d23
.align	4
abi_test_clobber_d23:
	AARCH64_VALID_CALL_TARGET
	fmov	d23, xzr
	ret
.size	abi_test_clobber_d23,.-abi_test_clobber_d23
.type	abi_test_clobber_d24, %function
.globl	abi_test_clobber_d24
.hidden	abi_test_clobber_d24
.align	4
abi_test_clobber_d24:
	AARCH64_VALID_CALL_TARGET
	fmov	d24, xzr
	ret
.size	abi_test_clobber_d24,.-abi_test_clobber_d24
.type	abi_test_clobber_d25, %function
.globl	abi_test_clobber_d25
.hidden	abi_test_clobber_d25
.align	4
abi_test_clobber_d25:
	AARCH64_VALID_CALL_TARGET
	fmov	d25, xzr
	ret
.size	abi_test_clobber_d25,.-abi_test_clobber_d25
.type	abi_test_clobber_d26, %function
.globl	abi_test_clobber_d26
.hidden	abi_test_clobber_d26
.align	4
abi_test_clobber_d26:
	AARCH64_VALID_CALL_TARGET
	fmov	d26, xzr
	ret
.size	abi_test_clobber_d26,.-abi_test_clobber_d26
.type	abi_test_clobber_d27, %function
.globl	abi_test_clobber_d27
.hidden	abi_test_clobber_d27
.align	4
abi_test_clobber_d27:
	AARCH64_VALID_CALL_TARGET
	fmov	d27, xzr
	ret
.size	abi_test_clobber_d27,.-abi_test_clobber_d27
.type	abi_test_clobber_d28, %function
.globl	abi_test_clobber_d28
.hidden	abi_test_clobber_d28
.align	4
abi_test_clobber_d28:
	AARCH64_VALID_CALL_TARGET
	fmov	d28, xzr
	ret
.size	abi_test_clobber_d28,.-abi_test_clobber_d28
.type	abi_test_clobber_d29, %function
.globl	abi_test_clobber_d29
.hidden	abi_test_clobber_d29
.align	4
abi_test_clobber_d29:
	AARCH64_VALID_CALL_TARGET
	fmov	d29, xzr
	ret
.size	abi_test_clobber_d29,.-abi_test_clobber_d29
.type	abi_test_clobber_d30, %function
.globl	abi_test_clobber_d30
.hidden	abi_test_clobber_d30
.align	4
abi_test_clobber_d30:
	AARCH64_VALID_CALL_TARGET
	fmov	d30, xzr
	ret
.size	abi_test_clobber_d30,.-abi_test_clobber_d30
.type	abi_test_clobber_d31, %function
.globl	abi_test_clobber_d31
.hidden	abi_test_clobber_d31
.align	4
abi_test_clobber_d31:
	AARCH64_VALID_CALL_TARGET
	fmov	d31, xzr
	ret
.size	abi_test_clobber_d31,.-abi_test_clobber_d31
.type	abi_test_clobber_v8_upper, %function
.globl	abi_test_clobber_v8_upper
.hidden	abi_test_clobber_v8_upper
.align	4
abi_test_clobber_v8_upper:
	AARCH64_VALID_CALL_TARGET
	fmov	v8.d[1], xzr
	ret
.size	abi_test_clobber_v8_upper,.-abi_test_clobber_v8_upper
.type	abi_test_clobber_v9_upper, %function
.globl	abi_test_clobber_v9_upper
.hidden	abi_test_clobber_v9_upper
.align	4
abi_test_clobber_v9_upper:
	AARCH64_VALID_CALL_TARGET
	fmov	v9.d[1], xzr
	ret
.size	abi_test_clobber_v9_upper,.-abi_test_clobber_v9_upper
.type	abi_test_clobber_v10_upper, %function
.globl	abi_test_clobber_v10_upper
.hidden	abi_test_clobber_v10_upper
.align	4
abi_test_clobber_v10_upper:
	AARCH64_VALID_CALL_TARGET
	fmov	v10.d[1], xzr
	ret
.size	abi_test_clobber_v10_upper,.-abi_test_clobber_v10_upper
.type	abi_test_clobber_v11_upper, %function
.globl	abi_test_clobber_v11_upper
.hidden	abi_test_clobber_v11_upper
.align	4
abi_test_clobber_v11_upper:
	AARCH64_VALID_CALL_TARGET
	fmov	v11.d[1], xzr
	ret
.size	abi_test_clobber_v11_upper,.-abi_test_clobber_v11_upper
.type	abi_test_clobber_v12_upper, %function
.globl	abi_test_clobber_v12_upper
.hidden	abi_test_clobber_v12_upper
.align	4
abi_test_clobber_v12_upper:
	AARCH64_VALID_CALL_TARGET
	fmov	v12.d[1], xzr
	ret
.size	abi_test_clobber_v12_upper,.-abi_test_clobber_v12_upper
.type	abi_test_clobber_v13_upper, %function
.globl	abi_test_clobber_v13_upper
.hidden	abi_test_clobber_v13_upper
.align	4
abi_test_clobber_v13_upper:
	AARCH64_VALID_CALL_TARGET
	fmov	v13.d[1], xzr
	ret
.size	abi_test_clobber_v13_upper,.-abi_test_clobber_v13_upper
.type	abi_test_clobber_v14_upper, %function
.globl	abi_test_clobber_v14_upper
.hidden	abi_test_clobber_v14_upper
.align	4
abi_test_clobber_v14_upper:
	AARCH64_VALID_CALL_TARGET
	fmov	v14.d[1], xzr
	ret
.size	abi_test_clobber_v14_upper,.-abi_test_clobber_v14_upper
.type	abi_test_clobber_v15_upper, %function
.globl	abi_test_clobber_v15_upper
.hidden	abi_test_clobber_v15_upper
.align	4
abi_test_clobber_v15_upper:
	AARCH64_VALID_CALL_TARGET
	fmov	v15.d[1], xzr
	ret
.size	abi_test_clobber_v15_upper,.-abi_test_clobber_v15_upper
#endif  // !OPENSSL_NO_ASM && defined(OPENSSL_AARCH64) && defined(__ELF__)
