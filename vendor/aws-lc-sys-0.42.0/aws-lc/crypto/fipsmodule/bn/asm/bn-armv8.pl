#!/usr/bin/env perl
# Copyright (c) 2023, Google Inc.
# SPDX-License-Identifier: Apache-2.0

use strict;

my $flavour = shift;
my $output  = shift;
if ($flavour =~ /\./) { $output = $flavour; undef $flavour; }

$0 =~ m/(.*[\/\\])[^\/\\]+$/;
my $dir = $1;
my $xlate;
( $xlate="${dir}arm-xlate.pl" and -f $xlate ) or
( $xlate="${dir}../../../perlasm/arm-xlate.pl" and -f $xlate) or
die "can't locate arm-xlate.pl";

open OUT, "| \"$^X\" \"$xlate\" $flavour \"$output\"";
*STDOUT = *OUT;

my ($rp, $ap, $bp, $num) = ("x0", "x1", "x2", "x3");
my ($a0, $a1, $b0, $b1, $num_pairs) = ("x4", "x5", "x6", "x7", "x8");
my $code = <<____;
#include <openssl/arm_arch.h>

.text

// BN_ULONG bn_add_words(BN_ULONG *rp, const BN_ULONG *ap, const BN_ULONG *bp,
//                       size_t num);
.type	bn_add_words, %function
.globl	bn_add_words
.align	4
bn_add_words:
.cfi_startproc
	AARCH64_VALID_CALL_TARGET
	# Clear the carry flag.
	cmn	xzr, xzr

	# aarch64 can load two registers at a time, so we do two loop iterations at
	# at a time. Split $num = 2 * $num_pairs + $num. This allows loop
	# operations to use CBNZ without clobbering the carry flag.
	lsr	$num_pairs, $num, #1
	and	$num, $num, #1

	cbz	$num_pairs, .Ladd_tail
.Ladd_loop:
	ldp	$a0, $a1, [$ap], #16
	ldp	$b0, $b1, [$bp], #16
	sub	$num_pairs, $num_pairs, #1
	adcs	$a0, $a0, $b0
	adcs	$a1, $a1, $b1
	stp	$a0, $a1, [$rp], #16
	cbnz	$num_pairs, .Ladd_loop

.Ladd_tail:
	cbz	$num, .Ladd_exit
	ldr	$a0, [$ap], #8
	ldr	$b0, [$bp], #8
	adcs	$a0, $a0, $b0
	str	$a0, [$rp], #8

.Ladd_exit:
	cset	x0, cs
	ret
.cfi_endproc
.size	bn_add_words,.-bn_add_words

// BN_ULONG bn_sub_words(BN_ULONG *rp, const BN_ULONG *ap, const BN_ULONG *bp,
//                       size_t num);
.type	bn_sub_words, %function
.globl	bn_sub_words
.align	4
bn_sub_words:
.cfi_startproc
	AARCH64_VALID_CALL_TARGET
	# Set the carry flag. Arm's borrow bit is flipped from the carry flag,
	# so we want C = 1 here.
	cmp	xzr, xzr

	# aarch64 can load two registers at a time, so we do two loop iterations at
	# at a time. Split $num = 2 * $num_pairs + $num. This allows loop
	# operations to use CBNZ without clobbering the carry flag.
	lsr	$num_pairs, $num, #1
	and	$num, $num, #1

	cbz	$num_pairs, .Lsub_tail
.Lsub_loop:
	ldp	$a0, $a1, [$ap], #16
	ldp	$b0, $b1, [$bp], #16
	sub	$num_pairs, $num_pairs, #1
	sbcs	$a0, $a0, $b0
	sbcs	$a1, $a1, $b1
	stp	$a0, $a1, [$rp], #16
	cbnz	$num_pairs, .Lsub_loop

.Lsub_tail:
	cbz	$num, .Lsub_exit
	ldr	$a0, [$ap], #8
	ldr	$b0, [$bp], #8
	sbcs	$a0, $a0, $b0
	str	$a0, [$rp], #8

.Lsub_exit:
	cset x0, cc
	ret
.cfi_endproc
.size   bn_sub_words,.-bn_sub_words
____

print $code;
close STDOUT or die "error closing STDOUT: $!";
