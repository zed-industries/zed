#!/usr/bin/env perl
# Copyright (c) 2019, Google Inc.
#
# SPDX-License-Identifier: ISC

# This file defines helper functions for crypto/test/abi_test.h on 32-bit
# ARM. See that header for details on how to use this.
#
# For convenience, this file is linked into libcrypto, where consuming builds
# already support architecture-specific sources. The static linker should drop
# this code in non-test binaries. This includes a shared library build of
# libcrypto, provided --gc-sections (ELF), -dead_strip (iOS), or equivalent is
# used.
#
# References:
#
# AAPCS: http://infocenter.arm.com/help/topic/com.arm.doc.ihi0042f/IHI0042F_aapcs.pdf
# iOS ARMv6: https://developer.apple.com/library/archive/documentation/Xcode/Conceptual/iPhoneOSABIReference/Articles/ARMv6FunctionCallingConventions.html
# iOS ARMv7: https://developer.apple.com/library/archive/documentation/Xcode/Conceptual/iPhoneOSABIReference/Articles/ARMv7FunctionCallingConventions.html
# Linux: http://sourcery.mentor.com/sgpp/lite/arm/portal/kbattach142/arm_gnu_linux_%20abi.pdf

use strict;

my $flavour = shift;
my $output  = shift;

$0 =~ m/(.*[\/\\])[^\/\\]+$/;
my $dir = $1;
my $xlate;
( $xlate="${dir}arm-xlate.pl" and -f $xlate ) or
( $xlate="${dir}../../perlasm/arm-xlate.pl" and -f $xlate) or
die "can't locate arm-xlate.pl";

open OUT, "| \"$^X\" \"$xlate\" $flavour \"$output\"";
*STDOUT = *OUT;

my ($func, $state, $argv, $argc) = ("r0", "r1", "r2", "r3");
my $code = <<____;
.syntax	unified

.arch	armv7-a
.fpu	vfp

.text

@ abi_test_trampoline loads callee-saved registers from |state|, calls |func|
@ with |argv|, then saves the callee-saved registers into |state|. It returns
@ the result of |func|. The |unwind| argument is unused.
@ uint32_t abi_test_trampoline(void (*func)(...), CallerState *state,
@                              const uint32_t *argv, size_t argc,
@                              int unwind);
.type	abi_test_trampoline, %function
.globl	abi_test_trampoline
.align	4
abi_test_trampoline:
	@ Save parameters and all callee-saved registers. For convenience, we
	@ save r9 on iOS even though it's volatile.
	vstmdb	sp!, {d8-d15}
	stmdb	sp!, {r0-r11,lr}

	@ Reserve stack space for six (10-4) stack parameters, plus an extra 4
	@ bytes to keep it 8-byte-aligned (see AAPCS, section 5.3).
	sub     sp, sp, #28

	@ Every register in AAPCS is either non-volatile or a parameter (except
	@ r9 on iOS), so this code, by the actual call, loses all its scratch
	@ registers. First fill in stack parameters while there are registers
	@ to spare.
	cmp	$argc, #4
	bls	.Lstack_args_done
	mov	r4, sp				@ r4 is the output pointer.
	add	r5, $argv, $argc, lsl #2	@ Set r5 to the end of argv.
	add	$argv, $argv, #16		@ Skip four arguments.
.Lstack_args_loop:
	ldr	r6, [$argv], #4
	cmp	$argv, r5
	str	r6, [r4], #4
	bne	.Lstack_args_loop

.Lstack_args_done:
	@ Load registers from |$state|.
	vldmia	$state!, {d8-d15}
#if defined(__APPLE__)
	@ r9 is not volatile on iOS.
	ldmia	$state!, {r4-r8,r10-r11}
#else
	ldmia	$state!, {r4-r11}
#endif

	@ Load register parameters. This uses up our remaining registers, so we
	@ repurpose lr as scratch space.
	ldr	$argc, [sp, #40]	@ Reload argc.
	ldr	lr, [sp, #36]		@ Load argv into lr.
	cmp	$argc, #3
	bhi	.Larg_r3
	beq	.Larg_r2
	cmp	$argc, #1
	bhi	.Larg_r1
	beq	.Larg_r0
	b	.Largs_done

.Larg_r3:
	ldr	r3, [lr, #12]	@ argv[3]
.Larg_r2:
	ldr	r2, [lr, #8]	@ argv[2]
.Larg_r1:
	ldr	r1, [lr, #4]	@ argv[1]
.Larg_r0:
	ldr	r0, [lr]	@ argv[0]
.Largs_done:

	@ With every other register in use, load the function pointer into lr
	@ and call the function.
	ldr	lr, [sp, #28]
	blx	lr

	@ r1-r3 are free for use again. The trampoline only supports
	@ single-return functions. Pass r4-r11 to the caller.
	ldr	$state, [sp, #32]
	vstmia	$state!, {d8-d15}
#if defined(__APPLE__)
	@ r9 is not volatile on iOS.
	stmia	$state!, {r4-r8,r10-r11}
#else
	stmia	$state!, {r4-r11}
#endif

	@ Unwind the stack and restore registers.
	add	sp, sp, #44		@ 44 = 28+16
	ldmia	sp!, {r4-r11,lr}	@ Skip r0-r3 (see +16 above).
	vldmia	sp!, {d8-d15}

	bx	lr
.size	abi_test_trampoline,.-abi_test_trampoline
____

# abi_test_clobber_* zeros the corresponding register. These are used to test
# the ABI-testing framework.
foreach (0..12) {
  # This loop skips r13 (sp), r14 (lr, implicitly clobbered by every call), and
  # r15 (pc).
  $code .= <<____;
.type	abi_test_clobber_r$_, %function
.globl	abi_test_clobber_r$_
.align	4
abi_test_clobber_r$_:
	mov	r$_, #0
	bx	lr
.size	abi_test_clobber_r$_,.-abi_test_clobber_r$_
____
}

foreach (0..15) {
  my $lo = "s".(2*$_);
  my $hi = "s".(2*$_+1);
  $code .= <<____;
.type	abi_test_clobber_d$_, %function
.globl	abi_test_clobber_d$_
.align	4
abi_test_clobber_d$_:
	mov	r0, #0
	vmov	$lo, r0
	vmov	$hi, r0
	bx	lr
.size	abi_test_clobber_d$_,.-abi_test_clobber_d$_
____
}

print $code;
close STDOUT or die "error closing STDOUT: $!";
