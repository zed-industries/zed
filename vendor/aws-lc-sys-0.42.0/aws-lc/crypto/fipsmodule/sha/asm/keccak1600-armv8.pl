#!/usr/bin/env perl
# Copyright 2017-2022 The OpenSSL Project Authors. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0
#
# ====================================================================
# Written by Andy Polyakov <appro@openssl.org> for the OpenSSL
# project.
# ====================================================================
#
# Keccak-1600 for ARMv8.
#
# June 2017.
#
# This is straightforward KECCAK_1X_ALT implementation. It makes no
# sense to attempt SIMD/NEON implementation for following reason.
# 64-bit lanes of vector registers can't be addressed as easily as in
# 32-bit mode. This means that 64-bit NEON is bound to be slower than
# 32-bit NEON, and this implementation is faster than 32-bit NEON on
# same processor. Even though it takes more scalar xor's and andn's,
# it gets compensated by availability of rotate. Not to forget that
# most processors achieve higher issue rate with scalar instructions.
#
# February 2018.
#
# Add hardware-assisted ARMv8.2 implementation. It's KECCAK_1X_ALT
# variant with register permutation/rotation twist that allows to
# eliminate copies to temporary registers. If you look closely you'll
# notice that it uses only one lane of vector registers. The new
# instructions effectively facilitate parallel hashing, which we don't
# support [yet?]. But lowest-level core procedure is prepared for it.
# The inner round is 67 [vector] instructions, so it's not actually
# obvious that it will provide performance improvement [in serial
# hash] as long as vector instructions issue rate is limited to 1 per
# cycle...
#
# July 2025
#
# Removed SHA3 variant, restricted assembly to core Keccak permutation.
#
######################################################################
# Numbers are cycles per processed byte.
#
#		r=1088(*)
#
# Cortex-A53	13
# Cortex-A57	12
# X-Gene	14
# Mongoose	10
# Kryo		12
# Denver	7.8
# Apple A7	7.2
# ThunderX2	9.7
#
# (*)	Corresponds to SHA3-256. No improvement coefficients are listed
#	because they vary too much from compiler to compiler. Newer
#	compiler does much better and improvement varies from 5% on
#	Cortex-A57 to 25% on Cortex-A53. While in comparison to older
#	compiler this code is at least 2x faster...

# File keccak1600-armv8.pl is imported from OpenSSL.
# https://github.com/openssl/openssl/blob/479b9adb88b9050186c1e9fc94879906f378b14b/crypto/sha/asm/keccak1600-armv8.pl

# $output is the last argument if it looks like a file (it has an extension)
# $flavour is the first argument if it doesn't look like a file
if ($#ARGV < 1) { die "Not enough arguments provided.
  Two arguments are necessary: the flavour and the output file path."; }

$flavour = shift;
$output = shift;

if ($flavour && $flavour ne "void") {
    $0 =~ m/(.*[\/\\])[^\/\\]+$/; $dir=$1;
    ( $xlate="${dir}arm-xlate.pl" and -f $xlate ) or
    ( $xlate="${dir}../../../perlasm/arm-xlate.pl" and -f $xlate) or
    die "can't locate arm-xlate.pl";

    open OUT,"| \"$^X\" \"$xlate\" $flavour \"$output\"";
    *STDOUT=*OUT;
} else {
    open OUT,">$output";
    *STDOUT=*OUT;
}

my @subrhotates = ([  64,  63, 2, 36, 37 ],
                [ 28, 20,  58, 9, 44 ],
                [  61, 54, 21, 39, 25 ],
                [ 23, 19, 49, 43,  56 ],
                [ 46,  62, 3, 8, 50 ]);

$code.=<<___;
#include <openssl/arm_arch.h>
.text
.align 8	// strategic alignment and padding that allows to use
		// address value as loop termination condition...
	.quad	0,0,0,0,0,0,0,0
.type	iotas_hw,%object
iotas_hw:
	.quad	0x0000000000000001
	.quad	0x0000000000008082
	.quad	0x800000000000808a
	.quad	0x8000000080008000
	.quad	0x000000000000808b
	.quad	0x0000000080000001
	.quad	0x8000000080008081
	.quad	0x8000000000008009
	.quad	0x000000000000008a
	.quad	0x0000000000000088
	.quad	0x0000000080008009
	.quad	0x000000008000000a
	.quad	0x000000008000808b
	.quad	0x800000000000008b
	.quad	0x8000000000008089
	.quad	0x8000000000008003
	.quad	0x8000000000008002
	.quad	0x8000000000000080
	.quad	0x000000000000800a
	.quad	0x800000008000000a
	.quad	0x8000000080008081
	.quad	0x8000000000008080
	.quad	0x0000000080000001
	.quad	0x8000000080008008
.size	iotas_hw,.-iotas_hw
___
								{{{
my @A = map([ "x$_", "x".($_+1), "x".($_+2), "x".($_+3), "x".($_+4) ],
            (0, 5, 10, 15, 20));
   $A[3][3] = "x25"; # x18 is reserved

my @C = map("x$_", (26,27,28,30));

$code.=<<___;
.type	KeccakF1600_int,%function
.align	5
KeccakF1600_int:
	AARCH64_SIGN_LINK_REGISTER
	adr	$C[2],iotas_hw
	stp	$C[2],x30,[sp,#16]		// 32 bytes on top are mine
	b	.Loop
.align	4
.Loop:
	////////////////////////////////////////// Theta
	eor	$C[0],$A[0][0],$A[1][0]
	stp	$A[0][4],$A[1][4],[sp,#0]	// offload pair...
	eor	$C[1],$A[0][1],$A[1][1]
	eor	$C[2],$A[0][2],$A[1][2]
	eor	$C[3],$A[0][3],$A[1][3]
___
	$C[4]=$A[0][4];
	$C[5]=$A[1][4];
$code.=<<___;
	eor	$C[4],$A[0][4],$A[1][4]
	eor	$C[0],$C[0],$A[2][0]
	eor	$C[1],$C[1],$A[2][1]
	eor	$C[2],$C[2],$A[2][2]
	eor	$C[3],$C[3],$A[2][3]
	eor	$C[4],$C[4],$A[2][4]
	eor	$C[0],$C[0],$A[3][0]
	eor	$C[1],$C[1],$A[3][1]
	eor	$C[2],$C[2],$A[3][2]
	eor	$C[3],$C[3],$A[3][3]
	eor	$C[4],$C[4],$A[3][4]
	eor	$C[0],$C[0],$A[4][0]
	eor	$C[2],$C[2],$A[4][2]
	eor	$C[1],$C[1],$A[4][1]
	eor	$C[3],$C[3],$A[4][3]
	eor	$C[4],$C[4],$A[4][4]
	eor	$C[5],$C[0],$C[2],ror#63
	eor	$A[0][1],$A[0][1],$C[5]
	eor	$A[1][1],$A[1][1],$C[5]
	eor	$A[2][1],$A[2][1],$C[5]
	eor	$A[3][1],$A[3][1],$C[5]
	eor	$A[4][1],$A[4][1],$C[5]
	eor	$C[5],$C[1],$C[3],ror#63
	eor	$C[2],$C[2],$C[4],ror#63
	eor	$C[3],$C[3],$C[0],ror#63
	eor	$C[4],$C[4],$C[1],ror#63
	eor	$C[1],   $A[0][2],$C[5]		// mov	$C[1],$A[0][2]
	eor	$A[1][2],$A[1][2],$C[5]
	eor	$A[2][2],$A[2][2],$C[5]
	eor	$A[3][2],$A[3][2],$C[5]
	eor	$A[4][2],$A[4][2],$C[5]
	eor	$A[0][0],$A[0][0],$C[4]
	eor	$A[1][0],$A[1][0],$C[4]
	eor	$A[2][0],$A[2][0],$C[4]
	eor	$A[3][0],$A[3][0],$C[4]
	eor	$A[4][0],$A[4][0],$C[4]
___
	$C[4]=undef;
	$C[5]=undef;
$code.=<<___;
	ldp	$A[0][4],$A[1][4],[sp,#0]	// re-load offloaded data
	eor	$C[0],   $A[0][3],$C[2]		// mov	$C[0],$A[0][3]
	eor	$A[1][3],$A[1][3],$C[2]
	eor	$A[2][3],$A[2][3],$C[2]
	eor	$A[3][3],$A[3][3],$C[2]
	eor	$A[4][3],$A[4][3],$C[2]
	eor	$C[2],   $A[0][4],$C[3]		// mov	$C[2],$A[0][4]
	eor	$A[1][4],$A[1][4],$C[3]
	eor	$A[2][4],$A[2][4],$C[3]
	eor	$A[3][4],$A[3][4],$C[3]
	eor	$A[4][4],$A[4][4],$C[3]
	////////////////////////////////////////// Rho+Pi
	mov	$C[3],$A[0][1]
	ror	$A[0][1],$A[1][1],#$subrhotates[1][1]
	//mov	$C[1],$A[0][2]
	ror	$A[0][2],$A[2][2],#$subrhotates[2][2]
	//mov	$C[0],$A[0][3]
	ror	$A[0][3],$A[3][3],#$subrhotates[3][3]
	//mov	$C[2],$A[0][4]
	ror	$A[0][4],$A[4][4],#$subrhotates[4][4]
	ror	$A[1][1],$A[1][4],#$subrhotates[1][4]
	ror	$A[2][2],$A[2][3],#$subrhotates[2][3]
	ror	$A[3][3],$A[3][2],#$subrhotates[3][2]
	ror	$A[4][4],$A[4][1],#$subrhotates[4][1]
	ror	$A[1][4],$A[4][2],#$subrhotates[4][2]
	ror	$A[2][3],$A[3][4],#$subrhotates[3][4]
	ror	$A[3][2],$A[2][1],#$subrhotates[2][1]
	ror	$A[4][1],$A[1][3],#$subrhotates[1][3]
	ror	$A[4][2],$A[2][4],#$subrhotates[2][4]
	ror	$A[3][4],$A[4][3],#$subrhotates[4][3]
	ror	$A[2][1],$A[1][2],#$subrhotates[1][2]
	ror	$A[1][3],$A[3][1],#$subrhotates[3][1]
	ror	$A[2][4],$A[4][0],#$subrhotates[4][0]
	ror	$A[4][3],$A[3][0],#$subrhotates[3][0]
	ror	$A[1][2],$A[2][0],#$subrhotates[2][0]
	ror	$A[3][1],$A[1][0],#$subrhotates[1][0]
	ror	$A[1][0],$C[0],#$subrhotates[0][3]
	ror	$A[2][0],$C[3],#$subrhotates[0][1]
	ror	$A[3][0],$C[2],#$subrhotates[0][4]
	ror	$A[4][0],$C[1],#$subrhotates[0][2]
	////////////////////////////////////////// Chi+Iota
	bic	$C[0],$A[0][2],$A[0][1]
	bic	$C[1],$A[0][3],$A[0][2]
	bic	$C[2],$A[0][0],$A[0][4]
	bic	$C[3],$A[0][1],$A[0][0]
	eor	$A[0][0],$A[0][0],$C[0]
	bic	$C[0],$A[0][4],$A[0][3]
	eor	$A[0][1],$A[0][1],$C[1]
	 ldr	$C[1],[sp,#16]
	eor	$A[0][3],$A[0][3],$C[2]
	eor	$A[0][4],$A[0][4],$C[3]
	eor	$A[0][2],$A[0][2],$C[0]
	 ldr	$C[3],[$C[1]],#8		// Iota[i++]
	bic	$C[0],$A[1][2],$A[1][1]
	 tst	$C[1],#255			// are we done?
	 str	$C[1],[sp,#16]
	bic	$C[1],$A[1][3],$A[1][2]
	bic	$C[2],$A[1][0],$A[1][4]
	 eor	$A[0][0],$A[0][0],$C[3]		// A[0][0] ^= Iota
	bic	$C[3],$A[1][1],$A[1][0]
	eor	$A[1][0],$A[1][0],$C[0]
	bic	$C[0],$A[1][4],$A[1][3]
	eor	$A[1][1],$A[1][1],$C[1]
	eor	$A[1][3],$A[1][3],$C[2]
	eor	$A[1][4],$A[1][4],$C[3]
	eor	$A[1][2],$A[1][2],$C[0]
	bic	$C[0],$A[2][2],$A[2][1]
	bic	$C[1],$A[2][3],$A[2][2]
	bic	$C[2],$A[2][0],$A[2][4]
	bic	$C[3],$A[2][1],$A[2][0]
	eor	$A[2][0],$A[2][0],$C[0]
	bic	$C[0],$A[2][4],$A[2][3]
	eor	$A[2][1],$A[2][1],$C[1]
	eor	$A[2][3],$A[2][3],$C[2]
	eor	$A[2][4],$A[2][4],$C[3]
	eor	$A[2][2],$A[2][2],$C[0]
	bic	$C[0],$A[3][2],$A[3][1]
	bic	$C[1],$A[3][3],$A[3][2]
	bic	$C[2],$A[3][0],$A[3][4]
	bic	$C[3],$A[3][1],$A[3][0]
	eor	$A[3][0],$A[3][0],$C[0]
	bic	$C[0],$A[3][4],$A[3][3]
	eor	$A[3][1],$A[3][1],$C[1]
	eor	$A[3][3],$A[3][3],$C[2]
	eor	$A[3][4],$A[3][4],$C[3]
	eor	$A[3][2],$A[3][2],$C[0]
	bic	$C[0],$A[4][2],$A[4][1]
	bic	$C[1],$A[4][3],$A[4][2]
	bic	$C[2],$A[4][0],$A[4][4]
	bic	$C[3],$A[4][1],$A[4][0]
	eor	$A[4][0],$A[4][0],$C[0]
	bic	$C[0],$A[4][4],$A[4][3]
	eor	$A[4][1],$A[4][1],$C[1]
	eor	$A[4][3],$A[4][3],$C[2]
	eor	$A[4][4],$A[4][4],$C[3]
	eor	$A[4][2],$A[4][2],$C[0]
	bne	.Loop
	ldr	x30,[sp,#24]
	AARCH64_VALIDATE_LINK_REGISTER
	ret
.size	KeccakF1600_int,.-KeccakF1600_int
.globl	KeccakF1600_hw
.type	KeccakF1600_hw,%function
.align	5
KeccakF1600_hw:
	AARCH64_SIGN_LINK_REGISTER
	stp	x29,x30,[sp,#-128]!
	add	x29,sp,#0
	stp	x19,x20,[sp,#16]
	stp	x21,x22,[sp,#32]
	stp	x23,x24,[sp,#48]
	stp	x25,x26,[sp,#64]
	stp	x27,x28,[sp,#80]
	sub	sp,sp,#48
	str	x0,[sp,#32]			// offload argument
	mov	$C[0],x0
	ldp	$A[0][0],$A[0][1],[x0,#16*0]
	ldp	$A[0][2],$A[0][3],[$C[0],#16*1]
	ldp	$A[0][4],$A[1][0],[$C[0],#16*2]
	ldp	$A[1][1],$A[1][2],[$C[0],#16*3]
	ldp	$A[1][3],$A[1][4],[$C[0],#16*4]
	ldp	$A[2][0],$A[2][1],[$C[0],#16*5]
	ldp	$A[2][2],$A[2][3],[$C[0],#16*6]
	ldp	$A[2][4],$A[3][0],[$C[0],#16*7]
	ldp	$A[3][1],$A[3][2],[$C[0],#16*8]
	ldp	$A[3][3],$A[3][4],[$C[0],#16*9]
	ldp	$A[4][0],$A[4][1],[$C[0],#16*10]
	ldp	$A[4][2],$A[4][3],[$C[0],#16*11]
	ldr	$A[4][4],[$C[0],#16*12]
	bl	KeccakF1600_int
	ldr	$C[0],[sp,#32]
	stp	$A[0][0],$A[0][1],[$C[0],#16*0]
	stp	$A[0][2],$A[0][3],[$C[0],#16*1]
	stp	$A[0][4],$A[1][0],[$C[0],#16*2]
	stp	$A[1][1],$A[1][2],[$C[0],#16*3]
	stp	$A[1][3],$A[1][4],[$C[0],#16*4]
	stp	$A[2][0],$A[2][1],[$C[0],#16*5]
	stp	$A[2][2],$A[2][3],[$C[0],#16*6]
	stp	$A[2][4],$A[3][0],[$C[0],#16*7]
	stp	$A[3][1],$A[3][2],[$C[0],#16*8]
	stp	$A[3][3],$A[3][4],[$C[0],#16*9]
	stp	$A[4][0],$A[4][1],[$C[0],#16*10]
	stp	$A[4][2],$A[4][3],[$C[0],#16*11]
	str	$A[4][4],[$C[0],#16*12]
	ldp	x19,x20,[x29,#16]
	add	sp,sp,#48
	ldp	x21,x22,[x29,#32]
	ldp	x23,x24,[x29,#48]
	ldp	x25,x26,[x29,#64]
	ldp	x27,x28,[x29,#80]
	ldp	x29,x30,[sp],#128
	AARCH64_VALIDATE_LINK_REGISTER
	ret
.size	KeccakF1600_hw,.-KeccakF1600_hw
___
								}}}
$code.=<<___;
.asciz	"Keccak-1600 permutation for ARMv8, CRYPTOGAMS by <appro\@openssl.org>"
___

foreach(split("\n",$code)) {

	s/\`([^\`]*)\`/eval($1)/ge;

	print $_,"\n";
}

close STDOUT or die "error closing STDOUT: $!";
