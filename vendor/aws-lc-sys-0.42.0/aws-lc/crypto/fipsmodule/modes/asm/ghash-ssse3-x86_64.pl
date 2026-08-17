#!/usr/bin/env perl
# Copyright (c) 2019, Google Inc.
#
# SPDX-License-Identifier: ISC

# ghash-ssse3-x86_64.pl is a constant-time variant of the traditional 4-bit
# table-based GHASH implementation. It requires SSSE3 instructions.
#
# For background, the table-based strategy is a 4-bit windowed multiplication.
# It precomputes all 4-bit multiples of H (this is 16 128-bit rows), then loops
# over 4-bit windows of the input and indexes them up into the table. Visually,
# it multiplies as in the schoolbook multiplication diagram below, but with
# more terms. (Each term is 4 bits, so there are 32 terms in each row.) First
# it incorporates the terms labeled '1' by indexing the most significant term
# of X into the table. Then it shifts and repeats for '2' and so on.
#
#        hhhhhh
#  *     xxxxxx
#  ============
#        666666
#       555555
#      444444
#     333333
#    222222
#   111111
#
# This implementation changes the order. We treat the table as a 16×16 matrix
# and transpose it. The first row is then the first byte of each multiple of H,
# and so on. We then reorder terms as below. Observe that the terms labeled '1'
# and '2' are all lookups into the first row, etc. This maps well to the SSSE3
# pshufb instruction, using alternating terms of X in parallel as indices. This
# alternation is needed because pshufb maps 4 bits to 8 bits. Then we shift and
# repeat for each row.
#
#        hhhhhh
#  *     xxxxxx
#  ============
#        224466
#       113355
#      224466
#     113355
#    224466
#   113355
#
# Next we account for GCM's confusing bit order. The "first" bit is the least
# significant coefficient, but GCM treats the most sigificant bit within a byte
# as first. Bytes are little-endian, and bits are big-endian. We reverse the
# bytes in XMM registers for a consistent bit and byte ordering, but this means
# the least significant bit is the most significant coefficient and vice versa.
#
# For consistency, "low", "high", "left-shift", and "right-shift" refer to the
# bit ordering within the XMM register, rather than the reversed coefficient
# ordering. Low bits are less significant bits and more significant
# coefficients. Right-shifts move from MSB to the LSB and correspond to
# increasing the power of each coefficient.
#
# Note this bit reversal enters into the table's column indices. H*1 is stored
# in column 0b1000 and H*x^3 is stored in column 0b0001. It also means earlier
# table rows contain more significant coefficients, so we iterate forwards.

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
( $xlate="${dir}x86_64-xlate.pl" and -f $xlate ) or
( $xlate="${dir}../../../perlasm/x86_64-xlate.pl" and -f $xlate) or
die "can't locate x86_64-xlate.pl";

open OUT, "| \"$^X\" \"$xlate\" $flavour \"$output\"";
*STDOUT = *OUT;

my ($Xi, $Htable, $in, $len) = $win64 ? ("%rcx", "%rdx", "%r8", "%r9") :
                                        ("%rdi", "%rsi", "%rdx", "%rcx");


my $code = <<____;
.text

# gcm_gmult_ssse3 multiplies |Xi| by |Htable| and writes the result to |Xi|.
# |Xi| is represented in GHASH's serialized byte representation. |Htable| is
# formatted as described above.
# void gcm_gmult_ssse3(uint64_t Xi[2], const u128 Htable[16]);
.type	gcm_gmult_ssse3, \@abi-omnipotent
.globl	gcm_gmult_ssse3
.align	16
gcm_gmult_ssse3:
.cfi_startproc
.seh_startproc
	_CET_ENDBR
____
$code .= <<____ if ($win64);
	subq	\$40, %rsp
.seh_allocstack	40
	movdqa	%xmm6, (%rsp)
.seh_savexmm128	%xmm6, 0
	movdqa	%xmm10, 16(%rsp)
.seh_savexmm128	%xmm10, 16
____
$code .= <<____;
	movdqu	($Xi), %xmm0
	movdqa	.Lreverse_bytes(%rip), %xmm10
	movdqa	.Llow4_mask(%rip), %xmm2

	# Reverse input bytes to deserialize.
	pshufb	%xmm10, %xmm0

	# Split each byte into low (%xmm0) and high (%xmm1) halves.
	movdqa	%xmm2, %xmm1
	pandn	%xmm0, %xmm1
	psrld	\$4, %xmm1
	pand	%xmm2, %xmm0

	# Maintain the result in %xmm2 (the value) and %xmm3 (carry bits). Note
	# that, due to bit reversal, %xmm3 contains bits that fall off when
	# right-shifting, not left-shifting.
	pxor	%xmm2, %xmm2
	pxor	%xmm3, %xmm3
____

my $call_counter = 0;
# process_rows returns assembly code to process $rows rows of the table. On
# input, $Htable stores the pointer to the next row. %xmm0 and %xmm1 store the
# low and high halves of the input. The result so far is passed in %xmm2. %xmm3
# must be zero. On output, $Htable is advanced to the next row and %xmm2 is
# updated. %xmm3 remains zero. It clobbers %rax, %xmm4, %xmm5, and %xmm6.
sub process_rows {
    my ($rows) = @_;
    $call_counter++;

    # Shifting whole XMM registers by bits is complex. psrldq shifts by bytes,
    # and psrlq shifts the two 64-bit halves separately. Each row produces 8
    # bits of carry, and the reduction needs an additional 7-bit shift. This
    # must fit in 64 bits so reduction can use psrlq. This allows up to 7 rows
    # at a time.
    die "Carry register would overflow 64 bits." if ($rows*8 + 7 > 64);

    return <<____;
	movq	\$$rows, %rax
.Loop_row_$call_counter:
	movdqa	($Htable), %xmm4
	leaq	16($Htable), $Htable

	# Right-shift %xmm2 and %xmm3 by 8 bytes.
	movdqa	%xmm2, %xmm6
	palignr	\$1, %xmm3, %xmm6
	movdqa	%xmm6, %xmm3
	psrldq	\$1, %xmm2

	# Load the next table row and index the low and high bits of the input.
	# Note the low (respectively, high) half corresponds to more
	# (respectively, less) significant coefficients.
	movdqa	%xmm4, %xmm5
	pshufb	%xmm0, %xmm4
	pshufb	%xmm1, %xmm5

	# Add the high half (%xmm5) without shifting.
	pxor	%xmm5, %xmm2

	# Add the low half (%xmm4). This must be right-shifted by 4 bits. First,
	# add into the carry register (%xmm3).
	movdqa	%xmm4, %xmm5
	psllq	\$60, %xmm5
	movdqa	%xmm5, %xmm6
	pslldq	\$8, %xmm6
	pxor	%xmm6, %xmm3

	# Next, add into %xmm2.
	psrldq	\$8, %xmm5
	pxor	%xmm5, %xmm2
	psrlq	\$4, %xmm4
	pxor	%xmm4, %xmm2

	subq	\$1, %rax
	jnz	.Loop_row_$call_counter

	# Reduce the carry register. The reduction polynomial is 1 + x + x^2 +
	# x^7, so we shift and XOR four times.
	pxor	%xmm3, %xmm2	# x^0 = 0
	psrlq	\$1, %xmm3
	pxor	%xmm3, %xmm2	# x^1 = x
	psrlq	\$1, %xmm3
	pxor	%xmm3, %xmm2	# x^(1+1) = x^2
	psrlq	\$5, %xmm3
	pxor	%xmm3, %xmm2	# x^(1+1+5) = x^7
	pxor	%xmm3, %xmm3
____
}

# We must reduce at least once every 7 rows, so divide into three chunks.
$code .= process_rows(5);
$code .= process_rows(5);
$code .= process_rows(6);

$code .= <<____;
	# Store the result. Reverse bytes to serialize.
	pshufb	%xmm10, %xmm2
	movdqu	%xmm2, ($Xi)

	# Zero any registers which contain secrets.
	pxor	%xmm0, %xmm0
	pxor	%xmm1, %xmm1
	pxor	%xmm2, %xmm2
	pxor	%xmm3, %xmm3
	pxor	%xmm4, %xmm4
	pxor	%xmm5, %xmm5
	pxor	%xmm6, %xmm6
____
$code .= <<____ if ($win64);
	movdqa	(%rsp), %xmm6
	movdqa	16(%rsp), %xmm10
	addq	\$40, %rsp
____
$code .= <<____;
	ret
.cfi_endproc
.seh_endproc
.size	gcm_gmult_ssse3,.-gcm_gmult_ssse3
____

$code .= <<____;
# gcm_ghash_ssse3 incorporates |len| bytes from |in| to |Xi|, using |Htable| as
# the key. It writes the result back to |Xi|. |Xi| is represented in GHASH's
# serialized byte representation. |Htable| is formatted as described above.
# void gcm_ghash_ssse3(uint64_t Xi[2], const u128 Htable[16], const uint8_t *in,
#                      size_t len);
.type	gcm_ghash_ssse3, \@abi-omnipotent
.globl	gcm_ghash_ssse3
.align	16
gcm_ghash_ssse3:
.cfi_startproc
.seh_startproc
	_CET_ENDBR
____
$code .= <<____ if ($win64);
	subq	\$56, %rsp
.seh_allocstack	56
	movdqa	%xmm6, (%rsp)
.seh_savexmm128	%xmm6, 0
	movdqa	%xmm10, 16(%rsp)
.seh_savexmm128	%xmm10, 16
	movdqa	%xmm11, 32(%rsp)
.seh_savexmm128	%xmm11, 32
____
$code .= <<____;
	movdqu	($Xi), %xmm0
	movdqa	.Lreverse_bytes(%rip), %xmm10
	movdqa	.Llow4_mask(%rip), %xmm11

	# This function only processes whole blocks.
	andq	\$-16, $len

	# Reverse input bytes to deserialize. We maintain the running
	# total in %xmm0.
	pshufb	%xmm10, %xmm0

	# Iterate over each block. On entry to each iteration, %xmm3 is zero.
	pxor	%xmm3, %xmm3
.Loop_ghash:
	# Incorporate the next block of input.
	movdqu	($in), %xmm1
	pshufb	%xmm10, %xmm1	# Reverse bytes.
	pxor	%xmm1, %xmm0

	# Split each byte into low (%xmm0) and high (%xmm1) halves.
	movdqa	%xmm11, %xmm1
	pandn	%xmm0, %xmm1
	psrld	\$4, %xmm1
	pand	%xmm11, %xmm0

	# Maintain the result in %xmm2 (the value) and %xmm3 (carry bits). Note
	# that, due to bit reversal, %xmm3 contains bits that fall off when
	# right-shifting, not left-shifting.
	pxor	%xmm2, %xmm2
	# %xmm3 is already zero at this point.
____

# We must reduce at least once every 7 rows, so divide into three chunks.
$code .= process_rows(5);
$code .= process_rows(5);
$code .= process_rows(6);

$code .= <<____;
	movdqa	%xmm2, %xmm0

	# Rewind $Htable for the next iteration.
	leaq	-256($Htable), $Htable

	# Advance input and continue.
	leaq	16($in), $in
	subq	\$16, $len
	jnz	.Loop_ghash

	# Reverse bytes and store the result.
	pshufb	%xmm10, %xmm0
	movdqu	%xmm0, ($Xi)

	# Zero any registers which contain secrets.
	pxor	%xmm0, %xmm0
	pxor	%xmm1, %xmm1
	pxor	%xmm2, %xmm2
	pxor	%xmm3, %xmm3
	pxor	%xmm4, %xmm4
	pxor	%xmm5, %xmm5
	pxor	%xmm6, %xmm6
____
$code .= <<____ if ($win64);
	movdqa	(%rsp), %xmm6
	movdqa	16(%rsp), %xmm10
	movdqa	32(%rsp), %xmm11
	addq	\$56, %rsp
____
$code .= <<____;
	ret
.cfi_endproc
.seh_endproc
.size	gcm_ghash_ssse3,.-gcm_ghash_ssse3

.section .rodata
.align	16
# .Lreverse_bytes is a permutation which, if applied with pshufb, reverses the
# bytes in an XMM register.
.Lreverse_bytes:
.byte	15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0
# .Llow4_mask is an XMM mask which selects the low four bits of each byte.
.Llow4_mask:
.quad	0x0f0f0f0f0f0f0f0f, 0x0f0f0f0f0f0f0f0f
.text
____

print $code;
close STDOUT or die "error closing STDOUT: $!";
