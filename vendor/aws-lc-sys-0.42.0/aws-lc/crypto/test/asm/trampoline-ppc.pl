#!/usr/bin/env perl
# Copyright (c) 2019, Google Inc.
#
# SPDX-License-Identifier: ISC

# This file defines helper functions for crypto/test/abi_test.h on ppc64le. See
# that header for details on how to use this.
#
# For convenience, this file is linked into libcrypto, where consuming builds
# already support architecture-specific sources. The static linker should drop
# this code in non-test binaries. This includes a shared library build of
# libcrypto, provided --gc-sections or equivalent is used.
#
# References:
#
# ELFv2: http://openpowerfoundation.org/wp-content/uploads/resources/leabi/leabi-20170510.pdf

use strict;

my $flavour = shift;
my $output  = shift;

$0 =~ m/(.*[\/\\])[^\/\\]+$/;
my $dir = $1;
my $xlate;
( $xlate="${dir}ppc-xlate.pl" and -f $xlate ) or
( $xlate="${dir}../../perlasm/ppc-xlate.pl" and -f $xlate) or
die "can't locate ppc-xlate.pl";

open OUT, "| \"$^X\" \"$xlate\" $flavour \"$output\"";
*STDOUT = *OUT;

unless ($flavour =~ /linux.*64le/) {
    die "This file only supports the ELFv2 ABI, used by ppc64le";
}

my $code = "";

sub load_or_store_regs {
  # $op is "l" or "st".
  my ($op, $base_reg, $base_offset) = @_;
  # Vector registers.
  foreach (20..31) {
    my $offset = $base_offset + ($_ - 20) * 16;
    # Vector registers only support indexed register addressing.
    $code .= "\tli\tr11, $offset\n";
    $code .= "\t${op}vx\tv$_, r11, $base_reg\n";
  }
  # Save general registers.
  foreach (14..31) {
    my $offset = $base_offset + 192 + ($_ - 14) * 8;
    $code .= "\t${op}d\tr$_, $offset($base_reg)\n";
  }
  # Save floating point registers.
  foreach (14..31) {
    my $offset = $base_offset + 336 + ($_ - 14) * 8;
    $code .= "\t${op}fd\tf$_, $offset($base_reg)\n";
  }
}

sub load_regs {
  my ($base_reg, $base_offset) = @_;
  load_or_store_regs("l", $base_reg, $base_offset);
}

sub store_regs {
  my ($base_reg, $base_offset) = @_;
  load_or_store_regs("st", $base_reg, $base_offset);
}

my ($func, $state, $argv, $argc) = ("r3", "r4", "r5", "r6");
$code .= <<____;
.machine	"any"
.text

# abi_test_trampoline loads callee-saved registers from |state|, calls |func|
# with |argv|, then saves the callee-saved registers into |state|. It returns
# the result of |func|. The |unwind| argument is unused.
# uint64_t abi_test_trampoline(void (*func)(...), CallerState *state,
#                              const uint64_t *argv, size_t argc,
#                              uint64_t unwind);
.globl	abi_test_trampoline
.align	5
abi_test_trampoline:
	# LR is saved into the caller's stack frame.
	mflr	r0
	std	r0, 16(r1)

	# Allocate 66*8 = 528 bytes of stack frame. From the top of the stack
	# to the bottom, the stack frame is:
	#
	#     0(r1) - Back chain pointer
	#     8(r1) - CR save area
	#    16(r1) - LR save area (for |func|)
	#    24(r1) - TOC pointer save area
	#    32(r1) - Saved copy of |state|
	#    40(r1) - Padding
	#    48(r1) - Vector register save area (v20-v31, 12 registers)
	#   240(r1) - General register save area (r14-r31, 18 registers)
	#   384(r1) - Floating point register save area (f14-f31, 18 registers)
	#
	# Note the layouts of the register save areas and CallerState match.
	#
	# In the ELFv2 ABI, the parameter save area is optional if the function
	# is non-variadic and all parameters fit in registers. We only support
	# such functions, so we omit it to test that |func| does not rely on it.
	stdu	r1, -528(r1)

	mfcr	r0
	std	r0, 8(r1)	# Save CR
	std	r2, 24(r1)	# Save TOC
	std	$state, 32(r1)	# Save |state|
____
# Save registers to the stack.
store_regs("r1", 48);
# Load registers from the caller.
load_regs($state, 0);
$code .= <<____;
	# Load CR from |state|.
	ld	r0, 480($state)
	mtcr	r0

	# Move parameters into temporary registers so they are not clobbered.
	addi	r11, $argv, -8	# Adjust for ldu below
	mr	r12, $func

	# Load parameters into registers.
	cmpdi	$argc, 0
	beq	.Largs_done
	mtctr	$argc
	ldu	r3, 8(r11)
	bdz	.Largs_done
	ldu	r4, 8(r11)
	bdz	.Largs_done
	ldu	r5, 8(r11)
	bdz	.Largs_done
	ldu	r6, 8(r11)
	bdz	.Largs_done
	ldu	r7, 8(r11)
	bdz	.Largs_done
	ldu	r8, 8(r11)
	bdz	.Largs_done
	ldu	r9, 8(r11)
	bdz	.Largs_done
	ldu	r10, 8(r11)

.Largs_done:
	li	r2, 0		# Clear TOC to test |func|'s global entry point
	mtctr	r12
	bctrl
	ld	r2, 24(r1)	# Restore TOC

	ld	$state, 32(r1)	# Reload |state|
____
# Output resulting registers to the caller.
store_regs($state, 0);
# Restore registers from the stack.
load_regs("r1", 48);
$code .= <<____;
	mfcr	r0
	std	r0, 480($state)	# Output CR to caller
	ld	r0, 8(r1)
	mtcrf	0b00111000, r0	# Restore CR2-CR4
	addi	r1, r1, 528
	ld	r0, 16(r1)	# Restore LR
	mtlr	r0
	blr
.size	abi_test_trampoline,.-abi_test_trampoline
____

# abi_test_clobber_* clobbers the corresponding register. These are used to test
# the ABI-testing framework.
foreach (0..31) {
  # r1 is the stack pointer. r13 is the thread pointer.
  next if ($_ == 1 || $_ == 13);
  $code .= <<____;
.globl	abi_test_clobber_r$_
.align	5
abi_test_clobber_r$_:
	li	r$_, 0
	blr
.size	abi_test_clobber_r$_,.-abi_test_clobber_r$_
____
}

foreach (0..31) {
  $code .= <<____;
.globl	abi_test_clobber_f$_
.align	4
abi_test_clobber_f$_:
	li	r0, 0
	# Use the red zone.
	std	r0, -8(r1)
	lfd	f$_, -8(r1)
	blr
.size	abi_test_clobber_f$_,.-abi_test_clobber_f$_
____
}

foreach (0..31) {
  $code .= <<____;
.globl	abi_test_clobber_v$_
.align	4
abi_test_clobber_v$_:
	vxor	v$_, v$_, v$_
	blr
.size	abi_test_clobber_v$_,.-abi_test_clobber_v$_
____
}

foreach (0..7) {
  # PPC orders CR fields in big-endian, so the mask is reversed from what one
  # would expect.
  my $mask = 1 << (7 - $_);
  $code .= <<____;
.globl	abi_test_clobber_cr$_
.align	4
abi_test_clobber_cr$_:
	# Flip the bits on cr$_ rather than setting to zero. With a four-bit
	# register, zeroing it will do nothing 1 in 16 times.
	mfcr	r0
	not	r0, r0
	mtcrf	$mask, r0
	blr
.size	abi_test_clobber_cr$_,.-abi_test_clobber_cr$_
____
}

$code .= <<____;
.globl	abi_test_clobber_ctr
.align	4
abi_test_clobber_ctr:
	li	r0, 0
	mtctr	r0
	blr
.size	abi_test_clobber_ctr,.-abi_test_clobber_ctr

.globl	abi_test_clobber_lr
.align	4
abi_test_clobber_lr:
	mflr	r0
	mtctr	r0
	li	r0, 0
	mtlr	r0
	bctr
.size	abi_test_clobber_lr,.-abi_test_clobber_lr

____

print $code;
close STDOUT or die "error closing STDOUT: $!";
