#!/usr/bin/env perl
# Copyright (c) 2018, Google Inc.
#
# SPDX-License-Identifier: ISC

# This file defines helper functions for crypto/test/abi_test.h on x86_64. See
# that header for details on how to use this.
#
# For convenience, this file is linked into libcrypto, where consuming builds
# already support architecture-specific sources. The static linker should drop
# this code in non-test binaries. This includes a shared library build of
# libcrypto, provided --gc-sections (ELF), -dead_strip (Mac), or equivalent is
# used.
#
# References:
#
# SysV ABI: https://github.com/hjl-tools/x86-psABI/wiki/x86-64-psABI-1.0.pdf
# Win64 ABI: https://docs.microsoft.com/en-us/cpp/build/x64-software-conventions?view=vs-2017

use strict;

my $flavour = shift;
my $output  = shift;

my $win64 = 0;
$win64 = 1 if ($flavour =~ /[nm]asm|mingw64/ || $output =~ /\.asm$/);

$0 =~ m/(.*[\/\\])[^\/\\]+$/;
my $dir = $1;
my $xlate;
( $xlate="${dir}x86_64-xlate.pl" and -f $xlate ) or
( $xlate="${dir}../../perlasm/x86_64-xlate.pl" and -f $xlate) or
die "can't locate x86_64-xlate.pl";

open OUT, "| \"$^X\" \"$xlate\" $flavour \"$output\"";
*STDOUT = *OUT;

# @inp is the registers used for function inputs, in order.
my @inp = $win64 ? ("%rcx", "%rdx", "%r8", "%r9") :
                   ("%rdi", "%rsi", "%rdx", "%rcx", "%r8", "%r9");

# @caller_state is the list of registers that the callee must preserve for the
# caller. This must match the definition of CallerState in abi_test.h.
my @caller_state = ("%rbx", "%rbp", "%r12", "%r13", "%r14", "%r15");
if ($win64) {
  @caller_state = ("%rbx", "%rbp", "%rdi", "%rsi", "%r12", "%r13", "%r14",
                   "%r15", "%xmm6", "%xmm7", "%xmm8", "%xmm9", "%xmm10",
                   "%xmm11", "%xmm12", "%xmm13", "%xmm14", "%xmm15");
}

# $caller_state_size is the size of CallerState, in bytes.
my $caller_state_size = 0;
foreach (@caller_state) {
  if (/^%r/) {
    $caller_state_size += 8;
  } elsif (/^%xmm/) {
    $caller_state_size += 16;
  } else {
    die "unknown register $_";
  }
}

# load_caller_state returns code which loads a CallerState structure at
# $off($reg) into the respective registers. No other registers are touched, but
# $reg may not be a register in CallerState. $cb is an optional callback to
# add extra lines after each movq or movdqa. $cb is passed the offset, relative
# to $reg, and name of each register.
sub load_caller_state {
  my ($off, $reg, $cb) = @_;
  my $ret = "";
  foreach (@caller_state) {
    my $old_off = $off;
    if (/^%r/) {
      $ret .= "\tmovq\t$off($reg), $_\n";
      $off += 8;
    } elsif (/^%xmm/) {
      $ret .= "\tmovdqa\t$off($reg), $_\n";
      $off += 16;
    } else {
      die "unknown register $_";
    }
    $ret .= $cb->($old_off, $_) if (defined($cb));
  }
  return $ret;
}

# store_caller_state behaves like load_caller_state, except that it writes the
# current values of the registers into $off($reg).
sub store_caller_state {
  my ($off, $reg, $cb) = @_;
  my $ret = "";
  foreach (@caller_state) {
    my $old_off = $off;
    if (/^%r/) {
      $ret .= "\tmovq\t$_, $off($reg)\n";
      $off += 8;
    } elsif (/^%xmm/) {
      $ret .= "\tmovdqa\t$_, $off($reg)\n";
      $off += 16;
    } else {
      die "unknown register $_";
    }
    $ret .= $cb->($old_off, $_) if (defined($cb));
  }
  return $ret;
}

# $max_params is the maximum number of parameters abi_test_trampoline supports.
my $max_params = 10;

# Windows reserves stack space for the register-based parameters, while SysV
# only reserves space for the overflow ones.
my $stack_params_skip = $win64 ? scalar(@inp) : 0;
my $num_stack_params = $win64 ? $max_params : $max_params - scalar(@inp);

my ($func, $state, $argv, $argc, $unwind) = @inp;
my $code = <<____;
.text

# abi_test_trampoline loads callee-saved registers from |state|, calls |func|
# with |argv|, then saves the callee-saved registers into |state|. It returns
# the result of |func|. If |unwind| is non-zero, this function triggers unwind
# instrumentation.
# uint64_t abi_test_trampoline(void (*func)(...), CallerState *state,
#                              const uint64_t *argv, size_t argc,
#                              int unwind);
.type	abi_test_trampoline, \@abi-omnipotent
.globl	abi_test_trampoline
.align	16
abi_test_trampoline:
.cfi_startproc
.seh_startproc
	_CET_ENDBR
	# Stack layout:
	#   8 bytes - align
	#   $caller_state_size bytes - saved caller registers
	#   8 bytes - scratch space
	#   8 bytes - saved copy of \$unwind (SysV-only)
	#   8 bytes - saved copy of \$state
	#   8 bytes - saved copy of \$func
	#   8 bytes - if needed for stack alignment
	#   8*$num_stack_params bytes - parameters for \$func
____
my $stack_alloc_size = 8 + $caller_state_size + 8*3 + 8*$num_stack_params;
if (!$win64) {
  $stack_alloc_size += 8;
}
# SysV and Windows both require the stack to be 16-byte-aligned. The call
# instruction offsets it by 8, so stack allocations must be 8 mod 16.
if ($stack_alloc_size % 16 != 8) {
  $num_stack_params++;
  $stack_alloc_size += 8;
}
my $stack_params_offset = 8 * $stack_params_skip;
my $func_offset = 8 * $num_stack_params;
my $state_offset = $func_offset + 8;
# On Win64, unwind is already passed in memory. On SysV, it is passed in as
# register and we must reserve stack space for it.
my ($unwind_offset, $scratch_offset);
if ($win64) {
  $unwind_offset = $stack_alloc_size + 5*8;
  $scratch_offset = $state_offset + 8;
} else {
  $unwind_offset = $state_offset + 8;
  $scratch_offset = $unwind_offset + 8;
}
my $caller_state_offset = $scratch_offset + 8;
$code .= <<____;
	subq	\$$stack_alloc_size, %rsp
.cfi_adjust_cfa_offset	$stack_alloc_size
.seh_allocstack	$stack_alloc_size
____
$code .= <<____ if (!$win64);
	movq	$unwind, $unwind_offset(%rsp)
____
# Store our caller's state. This is needed because we modify it ourselves, and
# also to isolate the test infrastruction from the function under test failing
# to save some register.
$code .= store_caller_state($caller_state_offset, "%rsp", sub {
  my ($off, $reg) = @_;
  $reg = substr($reg, 1);
  # SEH records offsets relative to %rsp (when there is no frame pointer), while
  # CFI records them relative to the CFA, the value of the parent's stack
  # pointer just before the call.
  my $cfi_off = $off - $stack_alloc_size - 8;
  my $seh_dir = ".seh_savereg";
  $seh_dir = ".seh_savexmm128" if ($reg =~ /^xmm/);
  return <<____;
.cfi_offset	$reg, $cfi_off
$seh_dir	\%$reg, $off
____
});

$code .= load_caller_state(0, $state);
$code .= <<____;
	# Stash \$func and \$state, so they are available after the call returns.
	movq	$func, $func_offset(%rsp)
	movq	$state, $state_offset(%rsp)

	# Load parameters. Note this will clobber \$argv and \$argc, so we can
	# only use non-parameter volatile registers. There are three, and they
	# are the same between SysV and Win64: %rax, %r10, and %r11.
	movq	$argv, %r10
	movq	$argc, %r11
____
foreach (@inp) {
  $code .= <<____;
	dec	%r11
	js	.Largs_done
	movq	(%r10), $_
	addq	\$8, %r10
____
}
$code .= <<____;
	leaq	$stack_params_offset(%rsp), %rax
.Largs_loop:
	dec	%r11
	js	.Largs_done

	# This block should be:
	#    movq (%r10), %rtmp
	#    movq %rtmp, (%rax)
	# There are no spare registers available, so we spill into the scratch
	# space.
	movq	%r11, $scratch_offset(%rsp)
	movq	(%r10), %r11
	movq	%r11, (%rax)
	movq	$scratch_offset(%rsp), %r11

	addq	\$8, %r10
	addq	\$8, %rax
	jmp	.Largs_loop

.Largs_done:
	movq	$func_offset(%rsp), %rax
	movq	$unwind_offset(%rsp), %r10
	testq	%r10, %r10
	jz	.Lno_unwind

	# Set the trap flag.
	pushfq
	orq	\$0x100, 0(%rsp)
	popfq

	# Run an instruction to trigger a breakpoint immediately before the
	# call.
	nop
.globl	abi_test_unwind_start
abi_test_unwind_start:

	call	*%rax
.globl	abi_test_unwind_return
abi_test_unwind_return:

	# Clear the trap flag. Note this assumes the trap flag was clear on
	# entry. We do not support instrumenting an unwind-instrumented
	# |abi_test_trampoline|.
	pushfq
	andq	\$-0x101, 0(%rsp)	# -0x101 is ~0x100
	popfq
.globl	abi_test_unwind_stop
abi_test_unwind_stop:

	jmp	.Lcall_done

.Lno_unwind:
	call	*%rax

.Lcall_done:
	# Store what \$func did our state, so our caller can check.
	movq  $state_offset(%rsp), $state
____
$code .= store_caller_state(0, $state);

# Restore our caller's state.
$code .= load_caller_state($caller_state_offset, "%rsp", sub {
  my ($off, $reg) = @_;
  $reg = substr($reg, 1);
  return ".cfi_restore\t$reg\n";
});
$code .= <<____;
	addq	\$$stack_alloc_size, %rsp
.cfi_adjust_cfa_offset	-$stack_alloc_size

	# %rax already contains \$func's return value, unmodified.
	ret
.cfi_endproc
.seh_endproc
.size	abi_test_trampoline,.-abi_test_trampoline
____

# abi_test_clobber_* zeros the corresponding register. These are used to test
# the ABI-testing framework.
foreach ("ax", "bx", "cx", "dx", "di", "si", "bp", 8..15) {
  $code .= <<____;
.type	abi_test_clobber_r$_, \@abi-omnipotent
.globl	abi_test_clobber_r$_
.align	16
abi_test_clobber_r$_:
	_CET_ENDBR
	xorq	%r$_, %r$_
	ret
.size	abi_test_clobber_r$_,.-abi_test_clobber_r$_
____
}

foreach (0..15) {
  $code .= <<____;
.type	abi_test_clobber_xmm$_, \@abi-omnipotent
.globl	abi_test_clobber_xmm$_
.align	16
abi_test_clobber_xmm$_:
	_CET_ENDBR
	pxor	%xmm$_, %xmm$_
	ret
.size	abi_test_clobber_xmm$_,.-abi_test_clobber_xmm$_
____
}

$code .= <<____;
# abi_test_bad_unwind_wrong_register preserves the ABI, but annotates the wrong
# register in unwind metadata.
# void abi_test_bad_unwind_wrong_register(void);
.type	abi_test_bad_unwind_wrong_register, \@abi-omnipotent
.globl	abi_test_bad_unwind_wrong_register
.align	16
abi_test_bad_unwind_wrong_register:
.cfi_startproc
.seh_startproc
	_CET_ENDBR
	pushq	%r12
.cfi_push	%r13	# This should be %r13
.seh_pushreg	%r13	# This should be %r13
	# Windows evaluates epilogs directly in the unwinder, rather than using
	# unwind codes. Add a nop so there is one non-epilog point (immediately
	# before the nop) where the unwinder can observe the mistake.
	nop
	popq	%r12
.cfi_pop	%r12
	ret
.seh_endproc
.cfi_endproc
.size	abi_test_bad_unwind_wrong_register,.-abi_test_bad_unwind_wrong_register

# abi_test_bad_unwind_temporary preserves the ABI, but temporarily corrupts the
# storage space for a saved register, breaking unwind.
# void abi_test_bad_unwind_temporary(void);
.type	abi_test_bad_unwind_temporary, \@abi-omnipotent
.globl	abi_test_bad_unwind_temporary
.align	16
abi_test_bad_unwind_temporary:
.cfi_startproc
.seh_startproc
	_CET_ENDBR
	pushq	%r12
.cfi_push	%r12
.seh_pushreg	%r12

	movq	%r12, %rax
	inc	%rax
	movq	%rax, (%rsp)
	# Unwinding from here is incorrect. Although %r12 itself has not been
	# changed, the unwind codes say to look in (%rsp) instead.

	movq	%r12, (%rsp)
	# Unwinding is now fixed.

	popq	%r12
.cfi_pop	%r12
	ret
.cfi_endproc
.seh_endproc
.size	abi_test_bad_unwind_temporary,.-abi_test_bad_unwind_temporary

# abi_test_get_and_clear_direction_flag clears the direction flag. If the flag
# was previously set, it returns one. Otherwise, it returns zero.
# int abi_test_get_and_clear_direction_flag(void);
.type	abi_test_set_direction_flag, \@abi-omnipotent
.globl	abi_test_get_and_clear_direction_flag
abi_test_get_and_clear_direction_flag:
	_CET_ENDBR
	pushfq
	popq	%rax
	andq	\$0x400, %rax
	shrq	\$10, %rax
	cld
	ret
.size abi_test_get_and_clear_direction_flag,.-abi_test_get_and_clear_direction_flag

# abi_test_set_direction_flag sets the direction flag.
# void abi_test_set_direction_flag(void);
.type	abi_test_set_direction_flag, \@abi-omnipotent
.globl	abi_test_set_direction_flag
abi_test_set_direction_flag:
	_CET_ENDBR
	std
	ret
.size abi_test_set_direction_flag,.-abi_test_set_direction_flag
____

if ($win64) {
  $code .= <<____;
# abi_test_bad_unwind_epilog preserves the ABI, and correctly annotates the
# prolog, but the epilog does not match Win64's rules, breaking unwind during
# the epilog.
# void abi_test_bad_unwind_epilog(void);
.type	abi_test_bad_unwind_epilog, \@abi-omnipotent
.globl	abi_test_bad_unwind_epilog
.align	16
abi_test_bad_unwind_epilog:
.seh_startproc
	pushq	%r12
.seh_pushreg	%r12

	nop

	# The epilog should begin here, but the nop makes it invalid.
	popq	%r12
	nop
	ret
.seh_endproc
.size	abi_test_bad_unwind_epilog,.-abi_test_bad_unwind_epilog
____
}

print $code;
close STDOUT or die "error closing STDOUT: $!";
