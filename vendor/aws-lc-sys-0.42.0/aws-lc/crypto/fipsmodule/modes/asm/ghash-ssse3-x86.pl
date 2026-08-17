#!/usr/bin/env perl
# Copyright (c) 2019, Google Inc.
# SPDX-License-Identifier: ISC

# ghash-ssse3-x86.pl is a constant-time variant of the traditional 4-bit
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

# The first two arguments should always be the flavour and output file path.
if ($#ARGV < 1) { die "Not enough arguments provided.
  Two arguments are necessary: the flavour and the output file path."; }

$0 =~ m/(.*[\/\\])[^\/\\]+$/; $dir=$1;
push(@INC,"${dir}","${dir}../../../perlasm");
require "x86asm.pl";

$output = $ARGV[1];
open STDOUT, ">$output";

&asm_init($ARGV[0]);

my ($Xi, $Htable, $in, $len) = ("edi", "esi", "edx", "ecx");
&static_label("reverse_bytes");
&static_label("low4_mask");

my $call_counter = 0;
# process_rows emits assembly code to process $rows rows of the table. On
# input, $Htable stores the pointer to the next row. xmm0 and xmm1 store the
# low and high halves of the input. The result so far is passed in xmm2. xmm3
# must be zero. On output, $Htable is advanced to the next row and xmm2 is
# updated. xmm3 remains zero. It clobbers eax, xmm4, xmm5, and xmm6.
sub process_rows {
	my ($rows) = @_;
	$call_counter++;

	# Shifting whole XMM registers by bits is complex. psrldq shifts by
	# bytes, and psrlq shifts the two 64-bit halves separately. Each row
	# produces 8 bits of carry, and the reduction needs an additional 7-bit
	# shift. This must fit in 64 bits so reduction can use psrlq. This
	# allows up to 7 rows at a time.
	die "Carry register would overflow 64 bits." if ($rows*8 + 7 > 64);

	&mov("eax", $rows);
&set_label("loop_row_$call_counter");
	&movdqa("xmm4", &QWP(0, $Htable));
	&lea($Htable, &DWP(16, $Htable));

	# Right-shift xmm2 and xmm3 by 8 bytes.
	&movdqa("xmm6", "xmm2");
	&palignr("xmm6", "xmm3", 1);
	&movdqa("xmm3", "xmm6");
	&psrldq("xmm2", 1);

	# Load the next table row and index the low and high bits of the input.
	# Note the low (respectively, high) half corresponds to more
	# (respectively, less) significant coefficients.
	&movdqa("xmm5", "xmm4");
	&pshufb("xmm4", "xmm0");
	&pshufb("xmm5", "xmm1");

	# Add the high half (xmm5) without shifting.
	&pxor("xmm2", "xmm5");

	# Add the low half (xmm4). This must be right-shifted by 4 bits. First,
	# add into the carry register (xmm3).
	&movdqa("xmm5", "xmm4");
	&psllq("xmm5", 60);
	&movdqa("xmm6", "xmm5");
	&pslldq("xmm6", 8);
	&pxor("xmm3", "xmm6");

	# Next, add into xmm2.
	&psrldq("xmm5", 8);
	&pxor("xmm2", "xmm5");
	&psrlq("xmm4", 4);
	&pxor("xmm2", "xmm4");

	&sub("eax", 1);
	&jnz(&label("loop_row_$call_counter"));

	# Reduce the carry register. The reduction polynomial is 1 + x + x^2 +
	# x^7, so we shift and XOR four times.
	&pxor("xmm2", "xmm3");	# x^0 = 0
	&psrlq("xmm3", 1);
	&pxor("xmm2", "xmm3");	# x^1 = x
	&psrlq("xmm3", 1);
	&pxor("xmm2", "xmm3");	# x^(1+1) = x^2
	&psrlq("xmm3", 5);
	&pxor("xmm2", "xmm3");	# x^(1+1+5) = x^7
	&pxor("xmm3", "xmm3");
____
}

# gcm_gmult_ssse3 multiplies |Xi| by |Htable| and writes the result to |Xi|.
# |Xi| is represented in GHASH's serialized byte representation. |Htable| is
# formatted as described above.
# void gcm_gmult_ssse3(uint64_t Xi[2], const u128 Htable[16]);
&function_begin("gcm_gmult_ssse3");
	&mov($Xi, &wparam(0));
	&mov($Htable, &wparam(1));

	&movdqu("xmm0", &QWP(0, $Xi));
	&call(&label("pic_point"));
&set_label("pic_point");
	&blindpop("eax");
	&movdqa("xmm7", &QWP(&label("reverse_bytes")."-".&label("pic_point"), "eax"));
	&movdqa("xmm2", &QWP(&label("low4_mask")."-".&label("pic_point"), "eax"));

	# Reverse input bytes to deserialize.
	&pshufb("xmm0", "xmm7");

	# Split each byte into low (xmm0) and high (xmm1) halves.
	&movdqa("xmm1", "xmm2");
	&pandn("xmm1", "xmm0");
	&psrld("xmm1", 4);
	&pand("xmm0", "xmm2");

	# Maintain the result in xmm2 (the value) and xmm3 (carry bits). Note
	# that, due to bit reversal, xmm3 contains bits that fall off when
	# right-shifting, not left-shifting.
	&pxor("xmm2", "xmm2");
	&pxor("xmm3", "xmm3");

	# We must reduce at least once every 7 rows, so divide into three
	# chunks.
	&process_rows(5);
	&process_rows(5);
	&process_rows(6);

	# Store the result. Reverse bytes to serialize.
	&pshufb("xmm2", "xmm7");
	&movdqu(&QWP(0, $Xi), "xmm2");

	# Zero any registers which contain secrets.
	&pxor("xmm0", "xmm0");
	&pxor("xmm1", "xmm1");
	&pxor("xmm2", "xmm2");
	&pxor("xmm3", "xmm3");
	&pxor("xmm4", "xmm4");
	&pxor("xmm5", "xmm5");
	&pxor("xmm6", "xmm6");
&function_end("gcm_gmult_ssse3");

# gcm_ghash_ssse3 incorporates |len| bytes from |in| to |Xi|, using |Htable| as
# the key. It writes the result back to |Xi|. |Xi| is represented in GHASH's
# serialized byte representation. |Htable| is formatted as described above.
# void gcm_ghash_ssse3(uint64_t Xi[2], const u128 Htable[16], const uint8_t *in,
#                      size_t len);
&function_begin("gcm_ghash_ssse3");
	&mov($Xi, &wparam(0));
	&mov($Htable, &wparam(1));
	&mov($in, &wparam(2));
	&mov($len, &wparam(3));

	&movdqu("xmm0", &QWP(0, $Xi));
	&call(&label("pic_point"));
&set_label("pic_point");
	&blindpop("ebx");
	&movdqa("xmm7", &QWP(&label("reverse_bytes")."-".&label("pic_point"), "ebx"));

	# This function only processes whole blocks.
	&and($len, -16);

	# Reverse input bytes to deserialize. We maintain the running
	# total in xmm0.
	&pshufb("xmm0", "xmm7");

	# Iterate over each block. On entry to each iteration, xmm3 is zero.
	&pxor("xmm3", "xmm3");
&set_label("loop_ghash");
	&movdqa("xmm2", &QWP(&label("low4_mask")."-".&label("pic_point"), "ebx"));

	# Incorporate the next block of input.
	&movdqu("xmm1", &QWP(0, $in));
	&pshufb("xmm1", "xmm7");	# Reverse bytes.
	&pxor("xmm0", "xmm1");

	# Split each byte into low (xmm0) and high (xmm1) halves.
	&movdqa("xmm1", "xmm2");
	&pandn("xmm1", "xmm0");
	&psrld("xmm1", 4);
	&pand("xmm0", "xmm2");

	# Maintain the result in xmm2 (the value) and xmm3 (carry bits). Note
	# that, due to bit reversal, xmm3 contains bits that fall off when
	# right-shifting, not left-shifting.
	&pxor("xmm2", "xmm2");
	# xmm3 is already zero at this point.

	# We must reduce at least once every 7 rows, so divide into three
	# chunks.
	&process_rows(5);
	&process_rows(5);
	&process_rows(6);

	&movdqa("xmm0", "xmm2");

	# Rewind $Htable for the next iteration.
	&lea($Htable, &DWP(-256, $Htable));

	# Advance input and continue.
	&lea($in, &DWP(16, $in));
	&sub($len, 16);
	&jnz(&label("loop_ghash"));

	# Reverse bytes and store the result.
	&pshufb("xmm0", "xmm7");
	&movdqu(&QWP(0, $Xi), "xmm0");

	# Zero any registers which contain secrets.
	&pxor("xmm0", "xmm0");
	&pxor("xmm1", "xmm1");
	&pxor("xmm2", "xmm2");
	&pxor("xmm3", "xmm3");
	&pxor("xmm4", "xmm4");
	&pxor("xmm5", "xmm5");
	&pxor("xmm6", "xmm6");
&function_end("gcm_ghash_ssse3");

# reverse_bytes is a permutation which, if applied with pshufb, reverses the
# bytes in an XMM register.
&set_label("reverse_bytes", 16);
&data_byte(15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0);
# low4_mask is an XMM mask which selects the low four bits of each byte.
&set_label("low4_mask", 16);
&data_word(0x0f0f0f0f, 0x0f0f0f0f, 0x0f0f0f0f, 0x0f0f0f0f);

&asm_finish();

close STDOUT or die "error closing STDOUT: $!";
