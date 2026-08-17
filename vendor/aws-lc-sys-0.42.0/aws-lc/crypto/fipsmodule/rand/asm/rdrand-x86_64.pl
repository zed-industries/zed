#!/usr/bin/env perl

# Copyright (c) 2015, Google Inc.
# SPDX-License-Identifier: ISC

use strict;

# The first two arguments should always be the flavour and output file path.
if ($#ARGV < 1) { die "Not enough arguments provided.
  Two arguments are necessary: the flavour and the output file path."; }

my $flavour = shift;
my $output  = shift;

my $win64 = 0;
$win64 = 1 if ($flavour =~ /[nm]asm|mingw64/ || $output =~ /\.asm$/);

$0 =~ m/(.*[\/\\])[^\/\\]+$/;
my $dir = $1;
my $xlate;
( $xlate="${dir}../../../perlasm/x86_64-xlate.pl" and -f $xlate) or
die "can't locate x86_64-xlate.pl";

open OUT,"| \"$^X\" \"$xlate\" $flavour \"$output\"";
*STDOUT=*OUT;

my ($out, $len, $tmp1, $tmp2) = $win64 ? ("%rcx", "%rdx", "%r8", "%r9")
                                       : ("%rdi", "%rsi", "%rdx", "%rcx");

print<<___;
.text

# int CRYPTO_rdrand_multiple8(uint8_t *buf, size_t len);
.globl CRYPTO_rdrand_multiple8
.type CRYPTO_rdrand_multiple8,\@abi-omnipotent
.align 16
CRYPTO_rdrand_multiple8:
.cfi_startproc
	_CET_ENDBR
	test $len, $len
	jz .Lout
	movq \$8, $tmp1
.Lloop:
	rdrand $tmp2
	jnc .Lerr_multiple
	test $tmp2, $tmp2 # OLD cpu's: can use all 0s in output as error signal
	jz .Lerr_multiple
	cmp \$-1, $tmp2 # AMD bug: check if all returned bits by RDRAND is stuck on 1
	je .Lerr_multiple
	movq $tmp2, 0($out)
	addq $tmp1, $out
	subq $tmp1, $len
	jnz .Lloop
.Lout:
	movq \$1, %rax
	retq
.Lerr_multiple:
	xorq %rax, %rax
	retq
.cfi_endproc
.size CRYPTO_rdrand_multiple8,.-CRYPTO_rdrand_multiple8
___

close STDOUT or die "error closing STDOUT: $!";	# flush
