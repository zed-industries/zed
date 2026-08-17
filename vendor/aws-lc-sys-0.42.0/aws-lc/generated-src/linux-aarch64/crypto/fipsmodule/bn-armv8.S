// This file is generated from a similarly-named Perl script in the BoringSSL
// source tree. Do not edit by hand.

#include <openssl/asm_base.h>

#if !defined(OPENSSL_NO_ASM) && defined(OPENSSL_AARCH64) && defined(__ELF__)
#include <openssl/arm_arch.h>

.text

// BN_ULONG bn_add_words(BN_ULONG *rp, const BN_ULONG *ap, const BN_ULONG *bp,
//                       size_t num);
.type	bn_add_words, %function
.globl	bn_add_words
.hidden	bn_add_words
.align	4
bn_add_words:
.cfi_startproc
	AARCH64_VALID_CALL_TARGET
	# Clear the carry flag.
	cmn	xzr, xzr

	# aarch64 can load two registers at a time, so we do two loop iterations at
	# at a time. Split x3 = 2 * x8 + x3. This allows loop
	# operations to use CBNZ without clobbering the carry flag.
	lsr	x8, x3, #1
	and	x3, x3, #1

	cbz	x8, .Ladd_tail
.Ladd_loop:
	ldp	x4, x5, [x1], #16
	ldp	x6, x7, [x2], #16
	sub	x8, x8, #1
	adcs	x4, x4, x6
	adcs	x5, x5, x7
	stp	x4, x5, [x0], #16
	cbnz	x8, .Ladd_loop

.Ladd_tail:
	cbz	x3, .Ladd_exit
	ldr	x4, [x1], #8
	ldr	x6, [x2], #8
	adcs	x4, x4, x6
	str	x4, [x0], #8

.Ladd_exit:
	cset	x0, cs
	ret
.cfi_endproc
.size	bn_add_words,.-bn_add_words

// BN_ULONG bn_sub_words(BN_ULONG *rp, const BN_ULONG *ap, const BN_ULONG *bp,
//                       size_t num);
.type	bn_sub_words, %function
.globl	bn_sub_words
.hidden	bn_sub_words
.align	4
bn_sub_words:
.cfi_startproc
	AARCH64_VALID_CALL_TARGET
	# Set the carry flag. Arm's borrow bit is flipped from the carry flag,
	# so we want C = 1 here.
	cmp	xzr, xzr

	# aarch64 can load two registers at a time, so we do two loop iterations at
	# at a time. Split x3 = 2 * x8 + x3. This allows loop
	# operations to use CBNZ without clobbering the carry flag.
	lsr	x8, x3, #1
	and	x3, x3, #1

	cbz	x8, .Lsub_tail
.Lsub_loop:
	ldp	x4, x5, [x1], #16
	ldp	x6, x7, [x2], #16
	sub	x8, x8, #1
	sbcs	x4, x4, x6
	sbcs	x5, x5, x7
	stp	x4, x5, [x0], #16
	cbnz	x8, .Lsub_loop

.Lsub_tail:
	cbz	x3, .Lsub_exit
	ldr	x4, [x1], #8
	ldr	x6, [x2], #8
	sbcs	x4, x4, x6
	str	x4, [x0], #8

.Lsub_exit:
	cset	x0, cc
	ret
.cfi_endproc
.size	bn_sub_words,.-bn_sub_words
#endif  // !OPENSSL_NO_ASM && defined(OPENSSL_AARCH64) && defined(__ELF__)
