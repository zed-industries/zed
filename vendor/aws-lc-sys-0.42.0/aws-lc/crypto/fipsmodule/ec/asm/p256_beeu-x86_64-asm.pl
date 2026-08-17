# Copyright (c) 2018, Amazon Inc.
#
# Written by Nir Drucker, and Shay Gueron
# AWS Cryptographic Algorithms Group
# (ndrucker@amazon.com, gueron@amazon.com)
# based on BN_mod_inverse_odd
#
# SPDX-License-Identifier: Apache-2.0 OR ISC

# The first two arguments should always be the flavour and output file path.
if ($#ARGV < 1) { die "Not enough arguments provided.
  Two arguments are necessary: the flavour and the output file path."; }

$flavour = shift;
$output  = shift;

$win64=0; $win64=1 if ($flavour =~ /[nm]asm|mingw64/ || $output =~ /\.asm$/);

$0 =~ m/(.*[\/\\])[^\/\\]+$/; $dir=$1;
( $xlate="${dir}x86_64-xlate.pl" and -f $xlate ) or
( $xlate="${dir}../../../perlasm/x86_64-xlate.pl" and -f $xlate) or
die "can't locate x86_64-xlate.pl";

$avx = 2;
for (@ARGV) { $avx = 0 if (/-DMY_ASSEMBLER_IS_TOO_OLD_FOR_AVX/); }

open OUT,"| \"$^X\" \"$xlate\" $flavour \"$output\"";
*STDOUT=*OUT;

#############################################################################
# extern int beeu_mod_inverse_vartime(BN_ULONG out[P256_LIMBS],
#                                     BN_ULONG a[P256_LIMBS],
#                                     BN_ULONG n[P256_LIMBS]);
#
# (Binary Extended Euclidean Algorithm.
#  See https://en.wikipedia.org/wiki/Binary_GCD_algorithm)
#
# Assumption 1: n is odd for the BEEU
# Assumption 2: 1 < a < n < 2^256

$out = "%rdi";
$a = "%rsi";
$n = "%rdx";

# X/Y will hold the inverse parameter
# Assumption: X,Y<2^(256)
$x0 = "%r8";
$x1 = "%r9";
$x2 = "%r10";
$x3 = "%r11";
# borrow from out (out is needed only at the end)
$x4 = "%rdi";
$y0 = "%r12";
$y1 = "%r13";
$y2 = "%r14";
$y3 = "%r15";
$y4 = "%rbp";
$shift = "%rcx";
$t0 = "%rax";
$t1 = "%rbx";
$t2 = "%rsi";
# borrow
$t3 = "%rcx";

$T0 = "%xmm0";
$T1 = "%xmm1";

# Offsets on the stack
$out_rsp = 0;
$shift_rsp = $out_rsp+0x8;
$a_rsp0 = $shift_rsp+0x8;
$a_rsp1 = $a_rsp0+0x8;
$a_rsp2 = $a_rsp1+0x8;
$a_rsp3 = $a_rsp2+0x8;
$b_rsp0 = $a_rsp3+0x8;
$b_rsp1 = $b_rsp0+0x8;
$b_rsp2 = $b_rsp1+0x8;
$b_rsp3 = $b_rsp2+0x8;

# Borrow when a_rsp/b_rsp are no longer needed.
$y_rsp0 = $a_rsp0;
$y_rsp1 = $y_rsp0+0x8;
$y_rsp2 = $y_rsp1+0x8;
$y_rsp3 = $y_rsp2+0x8;
$y_rsp4 = $y_rsp3+0x8;
$last_rsp_offset = $b_rsp3+0x8;

sub TEST_B_ZERO {
  return <<___;
    xorq $t1, $t1
    or $b_rsp0(%rsp), $t1
    or $b_rsp1(%rsp), $t1
    or $b_rsp2(%rsp), $t1
    or $b_rsp3(%rsp), $t1
    jz .Lbeeu_loop_end
___
}

$g_next_label = 0;

sub SHIFT1 {
  my ($var0, $var1, $var2, $var3, $var4) = @_;
  my $label = ".Lshift1_${g_next_label}";
  $g_next_label++;

  return <<___;
    # Ensure X is even and divide by two.
    movq \$1, $t1
    andq $var0, $t1
    jz $label
    add 0*8($n), $var0
    adc 1*8($n), $var1
    adc 2*8($n), $var2
    adc 3*8($n), $var3
    adc \$0, $var4

$label:
    shrdq \$1, $var1, $var0
    shrdq \$1, $var2, $var1
    shrdq \$1, $var3, $var2
    shrdq \$1, $var4, $var3
    shrq  \$1, $var4
___
}

sub SHIFT256 {
  my ($var) = @_;
  return <<___;
    # Copy shifted values.
    # Remember not to override t3=rcx
    movq 1*8+$var(%rsp), $t0
    movq 2*8+$var(%rsp), $t1
    movq 3*8+$var(%rsp), $t2

    shrdq %cl, $t0, 0*8+$var(%rsp)
    shrdq %cl, $t1, 1*8+$var(%rsp)
    shrdq %cl, $t2, 2*8+$var(%rsp)

    shrq  %cl, $t2
    mov $t2, 3*8+$var(%rsp)
___
}

$code.=<<___  if($avx);
.text

.type beeu_mod_inverse_vartime,\@function
.hidden beeu_mod_inverse_vartime
.globl  beeu_mod_inverse_vartime
.align 32
beeu_mod_inverse_vartime:
.cfi_startproc
    _CET_ENDBR
    push %rbp
.cfi_push rbp
    push %r12
.cfi_push r12
    push %r13
.cfi_push r13
    push %r14
.cfi_push r14
    push %r15
.cfi_push r15
    push %rbx
.cfi_push rbx
    push %rsi
.cfi_push rsi

    sub \$$last_rsp_offset, %rsp
.cfi_adjust_cfa_offset  $last_rsp_offset
    movq $out, $out_rsp(%rsp)

    # X=1, Y=0
    movq \$1, $x0
    xorq $x1, $x1
    xorq $x2, $x2
    xorq $x3, $x3
    xorq $x4, $x4

    xorq $y0, $y0
    xorq $y1, $y1
    xorq $y2, $y2
    xorq $y3, $y3
    xorq $y4, $y4

    # Copy a/n into B/A on the stack.
    vmovdqu 0*8($a), $T0
    vmovdqu 2*8($a), $T1
    vmovdqu $T0, $b_rsp0(%rsp)
    vmovdqu $T1, $b_rsp2(%rsp)

    vmovdqu 0*8($n), $T0
    vmovdqu 2*8($n), $T1
    vmovdqu $T0, $a_rsp0(%rsp)
    vmovdqu $T1, $a_rsp2(%rsp)

.Lbeeu_loop:
    ${\TEST_B_ZERO}

    # 0 < B < |n|,
    # 0 < A <= |n|,
    # (1)      X*a  ==  B   (mod |n|),
    # (2) (-1)*Y*a  ==  A   (mod |n|)

    # Now divide B by the maximum possible power of two in the
    # integers, and divide X by the same value mod |n|. When we're
    # done, (1) still holds.
    movq \$1, $shift

    # Note that B > 0
.Lbeeu_shift_loop_XB:
    movq $shift, $t1
    andq $b_rsp0(%rsp), $t1
    jnz .Lbeeu_shift_loop_end_XB

    ${\SHIFT1($x0, $x1, $x2, $x3, $x4)}
    shl \$1, $shift

    # Test wraparound of the shift parameter. The probability to have 32 zeroes
    # in a row is small Therefore having the value below equal \$0x8000000 or
    # \$0x8000 does not affect the performance. We choose 0x8000000 because it
    # is the maximal immediate value possible.
    cmp \$0x8000000, $shift
    jne .Lbeeu_shift_loop_XB

.Lbeeu_shift_loop_end_XB:
    bsf $shift, $shift
    test $shift, $shift
    jz .Lbeeu_no_shift_XB

    ${\SHIFT256($b_rsp0)}

.Lbeeu_no_shift_XB:
    # Same for A and Y.  Afterwards, (2) still holds.
    movq \$1, $shift

    # Note that A > 0
.Lbeeu_shift_loop_YA:
    movq $shift, $t1
    andq $a_rsp0(%rsp), $t1
    jnz .Lbeeu_shift_loop_end_YA

    ${\SHIFT1($y0, $y1, $y2, $y3, $y4)}
    shl \$1, $shift

    # Test wraparound of the shift parameter. The probability to have 32 zeroes
    # in a row is small therefore having the value below equal \$0x8000000 or
    # \$0x8000 Does not affect the performance. We choose 0x8000000 because it
    # is the maximal immediate value possible.
    cmp \$0x8000000, $shift
    jne .Lbeeu_shift_loop_YA

.Lbeeu_shift_loop_end_YA:
    bsf $shift, $shift
    test $shift, $shift
    jz .Lbeeu_no_shift_YA

    ${\SHIFT256($a_rsp0)}

.Lbeeu_no_shift_YA:
    # T = B-A (A,B < 2^256)
    mov $b_rsp0(%rsp), $t0
    mov $b_rsp1(%rsp), $t1
    mov $b_rsp2(%rsp), $t2
    mov $b_rsp3(%rsp), $t3
    sub $a_rsp0(%rsp), $t0
    sbb $a_rsp1(%rsp), $t1
    sbb $a_rsp2(%rsp), $t2
    sbb $a_rsp3(%rsp), $t3  # borrow from shift
    jnc .Lbeeu_B_bigger_than_A

    # A = A - B
    mov $a_rsp0(%rsp), $t0
    mov $a_rsp1(%rsp), $t1
    mov $a_rsp2(%rsp), $t2
    mov $a_rsp3(%rsp), $t3
    sub $b_rsp0(%rsp), $t0
    sbb $b_rsp1(%rsp), $t1
    sbb $b_rsp2(%rsp), $t2
    sbb $b_rsp3(%rsp), $t3
    mov $t0, $a_rsp0(%rsp)
    mov $t1, $a_rsp1(%rsp)
    mov $t2, $a_rsp2(%rsp)
    mov $t3, $a_rsp3(%rsp)

    # Y = Y + X
    add $x0, $y0
    adc $x1, $y1
    adc $x2, $y2
    adc $x3, $y3
    adc $x4, $y4
    jmp .Lbeeu_loop

.Lbeeu_B_bigger_than_A:
    # B = T = B - A
    mov $t0, $b_rsp0(%rsp)
    mov $t1, $b_rsp1(%rsp)
    mov $t2, $b_rsp2(%rsp)
    mov $t3, $b_rsp3(%rsp)

    # X = Y + X
    add $y0, $x0
    adc $y1, $x1
    adc $y2, $x2
    adc $y3, $x3
    adc $y4, $x4

    jmp .Lbeeu_loop

.Lbeeu_loop_end:
    # The Euclid's algorithm loop ends when A == beeu(a,n);
    # Therefore (-1)*Y*a == A (mod |n|), Y>0

    # Verify that A = 1 ==> (-1)*Y*a = A = 1  (mod |n|)
    mov $a_rsp0(%rsp), $t1
    sub \$1, $t1
    or $a_rsp1(%rsp), $t1
    or $a_rsp2(%rsp), $t1
    or $a_rsp3(%rsp), $t1
    # If not, fail.
    jnz .Lbeeu_err

    # From this point on, we no longer need X
    # Therefore we use it as a temporary storage.
    # X = n
    movq 0*8($n), $x0
    movq 1*8($n), $x1
    movq 2*8($n), $x2
    movq 3*8($n), $x3
    xorq $x4, $x4

.Lbeeu_reduction_loop:
    movq $y0, $y_rsp0(%rsp)
    movq $y1, $y_rsp1(%rsp)
    movq $y2, $y_rsp2(%rsp)
    movq $y3, $y_rsp3(%rsp)
    movq $y4, $y_rsp4(%rsp)

    # If Y>n ==> Y=Y-n
    sub $x0, $y0
    sbb $x1, $y1
    sbb $x2, $y2
    sbb $x3, $y3
    sbb \$0, $y4

    # Choose old Y or new Y
    cmovc $y_rsp0(%rsp), $y0
    cmovc $y_rsp1(%rsp), $y1
    cmovc $y_rsp2(%rsp), $y2
    cmovc $y_rsp3(%rsp), $y3
    jnc .Lbeeu_reduction_loop

    # X = n - Y (n, Y < 2^256), (Cancel the (-1))
    sub $y0, $x0
    sbb $y1, $x1
    sbb $y2, $x2
    sbb $y3, $x3

.Lbeeu_save:
    # Save the inverse(<2^256) to out.
    mov $out_rsp(%rsp), $out

    movq $x0, 0*8($out)
    movq $x1, 1*8($out)
    movq $x2, 2*8($out)
    movq $x3, 3*8($out)

    # Return 1.
    movq \$1, %rax
    jmp .Lbeeu_finish

.Lbeeu_err:
    # Return 0.
    xorq %rax, %rax

.Lbeeu_finish:
    add \$$last_rsp_offset, %rsp
.cfi_adjust_cfa_offset  -$last_rsp_offset
    pop %rsi
.cfi_pop rsi
    pop %rbx
.cfi_pop rbx
    pop %r15
.cfi_pop r15
    pop %r14
.cfi_pop r14
    pop %r13
.cfi_pop r13
    pop %r12
.cfi_pop r12
    pop %rbp
.cfi_pop rbp
    ret
.cfi_endproc

.size beeu_mod_inverse_vartime, .-beeu_mod_inverse_vartime
___

print $code;
close STDOUT or die "error closing STDOUT: $!";
