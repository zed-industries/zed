# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0 OR ISC
#
# This code is based on p256_beeu-x86_64-asm.pl (which is based on BN_mod_inverse_odd).

# The first two arguments should always be the flavour and output file path.
if ($#ARGV < 1) { die "Not enough arguments provided.
  Two arguments are necessary: the flavour and the output file path."; }

$flavour = shift;
$output  = shift;

$0 =~ m/(.*[\/\\])[^\/\\]+$/; $dir=$1;
( $xlate="${dir}arm-xlate.pl" and -f $xlate ) or
( $xlate="${dir}../../../perlasm/arm-xlate.pl" and -f $xlate) or
die "can't locate arm-xlate.pl";

open OUT, qq{| "$^X" "$xlate" $flavour "$output"};
*STDOUT=*OUT;
#############################################################################
# extern int beeu_mod_inverse_vartime(BN_ULONG out[P256_LIMBS],
#                                     BN_ULONG a[P256_LIMBS],
#                                     BN_ULONG n[P256_LIMBS]);
#
# (Binary Extended GCD (Euclidean) Algorithm.
#  See A. Menezes, P. vanOorschot, and S. Vanstone's Handbook of Applied Cryptography,
#  Chapter 14, Algorithm 14.61 and Note 14.64
#  http://cacr.uwaterloo.ca/hac/about/chap14.pdf)

# Assumption 1: n is odd for the BEEU
# Assumption 2: 1 < a < n < 2^256

# Details
# The inverse of x modulo y can be calculated using Alg. 14.61, where "a" would be that inverse.
# In other words,
# ax == 1 (mod y) (where the symbol “==“ denotes ”congruent“)
#  a == x^{-1} (mod y)
#
# It can be shown that throughout all the iterations of the algorithm, the following holds:
#    u = Ax + By
#    v = Cx + Dy
# The values B and D are not of interest in this case, so they need not be computed by the algorithm.
# This means the following congruences hold through the iterations of the algorithm.
#    Ax == u (mod y)
#    Cx == v (mod y)

# Now we will modify the notation to match that of BN_mod_inverse_odd()
# on which beeu_mod_inverse_vartime() in `p256_beeu-x86_64-asm` is based.
# In those functions:
#    x, y -> a, n
#    u, v -> B, A
#    A, C -> X, Y’, where Y’ = -Y
# Hence, the following holds throughout the algorithm iterations
#    Xa == B (mod n)
#   -Ya == A (mod n)
#
# Same algorithm in Python:
# def beeu(a, n):
#     X = 1
#     Y = 0
#     B = a
#     A = n
#     while (B != 0):
#         while (B % 2) == 0:
#             B >>= 1
#             if (X % 2) == 1:
#                 X = X + n
#             X >>= 1
#         while (A % 2) == 0:
#             A >>= 1
#             if (Y % 2) == 1:
#                 Y = Y + n
#             Y >>= 1
#         if (B >= A):
#             B = B - A
#             X = X + Y
#         else:
#             A = A - B
#             Y = Y + X
#     if (A != 1):
#         # error
#         return 0
#     else:
#         while (Y > n):
#             Y = Y - n
#         Y = n - Y
#         return Y


# For the internal variables,
# x0-x2, x30 are used to hold the modulus n. The input parameters passed in
# x1,x2 are copied first before corrupting them. x0 (out) is stored on the stack.
# x3-x7 are used for parameters, which is not the case in this function, so they are corruptible
# x8 is corruptible here
# (the function doesn't return a struct, hence x8 doesn't contain a passed-in address
#  for that struct).
# x9-x15 are corruptible registers
# x19-x28 are callee-saved registers

# X/Y will hold the inverse parameter
# Assumption: a,n,X,Y < 2^(256)
# Initially, X := 1, Y := 0
#            A := n, B := a

# Function parameters (as per the Procedure Call Standard)
my($out, $a_in, $n_in)=map("x$_",(0..2));
# Internal variables
my($n0, $n1, $n2, $n3)=map("x$_",(0..2,30));
my($x0, $x1, $x2, $x3, $x4)=map("x$_",(3..7));
my($y0, $y1, $y2, $y3, $y4)=map("x$_",(8..12));
my($shift)=("x13");
my($t0, $t1, $t2, $t3)=map("x$_",(14,15,19,20));
my($a0, $a1, $a2, $a3)=map("x$_",(21..24));
my($b0, $b1, $b2, $b3)=map("x$_",(25..28));

# if B == 0, jump to end of loop
sub TEST_B_ZERO {
  return <<___;
    orr     $t0, $b0, $b1
    orr     $t0, $t0, $b2

    // reverse the bit order of $b0. This is needed for clz after this macro
    rbit     $t1, $b0

    orr     $t0, $t0, $b3
    cbz     $t0,.Lbeeu_loop_end
___
}

# Shift right by 1 bit, adding the modulus first if the variable is odd
# if least_sig_bit(var0) == 0,
#     goto shift1_<ctr>
# else
#     add n and goto shift1_<ctr>
# Prerequisite: t0 = 0
$g_next_label = 0;
sub SHIFT1 {
  my ($var0, $var1, $var2, $var3, $var4) = @_;
  my $label = ".Lshift1_${g_next_label}";
  $g_next_label++;
  return <<___;
    tbz     $var0, #0, $label
    adds    $var0, $var0, $n0
    adcs    $var1, $var1, $n1
    adcs    $var2, $var2, $n2
    adcs    $var3, $var3, $n3
    adc     $var4, $var4, $t0
$label:
    // var0 := [var1|var0]<64..1>;
    // i.e. concatenate var1 and var0,
    //      extract bits <64..1> from the resulting 128-bit value
    //      and put them in var0
    extr    $var0, $var1, $var0, #1
    extr    $var1, $var2, $var1, #1
    extr    $var2, $var3, $var2, #1
    extr    $var3, $var4, $var3, #1
    lsr     $var4, $var4, #1
___
}

# compilation by clang 10.0.0 with -O2/-O3 of
#      a[0] = (a[0] >> count) | (a[1] << (64-count));
#      a[1] = (a[1] >> count) | (a[2] << (64-count));
#      a[2] = (a[2] >> count) | (a[3] << (64-count));
#      a[3] >>= count;
# Note: EXTR instruction used in SHIFT1 is similar to x86_64's SHRDQ
# except that the second source operand of EXTR is only immediate;
# that's why it cannot be used here where $shift is a variable
#
# In the following,
# t0 := 0 - shift
#
# then var0, for example, will be shifted right as follows:
# var0 := (var0 >> (uint(shift) mod 64)) | (var1 << (uint(t0) mod 64))
# "uint() mod 64" is from the definition of LSL and LSR instructions.
#
# What matters here is the order of instructions relative to certain other
# instructions, i.e.
# - lsr and lsl must precede orr of the corresponding registers.
# - lsl must preced the lsr of the same register afterwards.
# The chosen order of the instructions overall is to try and maximize
# the pipeline usage.
sub SHIFT256 {
  my ($var0, $var1, $var2, $var3) = @_;
  return <<___;
    neg $t0, $shift
    lsr $var0, $var0, $shift
    lsl $t1, $var1, $t0

    lsr $var1, $var1, $shift
    lsl $t2, $var2, $t0

    orr $var0, $var0, $t1

    lsr $var2, $var2, $shift
    lsl $t3, $var3, $t0

    orr $var1, $var1, $t2

    lsr $var3, $var3, $shift

    orr $var2, $var2, $t3
___
}

$code.=<<___;
#include "openssl/arm_arch.h"

.text
.globl  beeu_mod_inverse_vartime
.type   beeu_mod_inverse_vartime, %function
.align  4
beeu_mod_inverse_vartime:
    // Reserve enough space for 14 8-byte registers on the stack
    // in the first stp call for x29, x30.
    // Then store the remaining callee-saved registers.
    //
    //    | x29 | x30 | x19 | x20 | ... | x27 | x28 |  x0 |  x2 |
    //    ^                                                     ^
    //    sp  <------------------- 112 bytes ----------------> old sp
    //   x29 (FP)
    //
    AARCH64_SIGN_LINK_REGISTER
    stp     x29,x30,[sp,#-112]!
    add     x29,sp,#0
    stp     x19,x20,[sp,#16]
    stp     x21,x22,[sp,#32]
    stp     x23,x24,[sp,#48]
    stp     x25,x26,[sp,#64]
    stp     x27,x28,[sp,#80]
    stp     x0,x2,[sp,#96]

    // B = b3..b0 := a
    ldp     $b0,$b1,[$a_in]
    ldp     $b2,$b3,[$a_in,#16]

    // n3..n0 := n
    // Note: the value of input params are changed in the following.
    ldp     $n0,$n1,[$n_in]
    ldp     $n2,$n3,[$n_in,#16]

    // A = a3..a0 := n
    mov     $a0, $n0
    mov     $a1, $n1
    mov     $a2, $n2
    mov     $a3, $n3

    // X = x4..x0 := 1
    mov     $x0, #1
    eor     $x1, $x1, $x1
    eor     $x2, $x2, $x2
    eor     $x3, $x3, $x3
    eor     $x4, $x4, $x4

    // Y = y4..y0 := 0
    eor     $y0, $y0, $y0
    eor     $y1, $y1, $y1
    eor     $y2, $y2, $y2
    eor     $y3, $y3, $y3
    eor     $y4, $y4, $y4

.Lbeeu_loop:
    // if B == 0, jump to .Lbeeu_loop_end
    ${\TEST_B_ZERO}

    // 0 < B < |n|,
    // 0 < A <= |n|,
    // (1)      X*a  ==  B   (mod |n|),
    // (2) (-1)*Y*a  ==  A   (mod |n|)

    // Now divide B by the maximum possible power of two in the
    // integers, and divide X by the same value mod |n|.
    // When we're done, (1) still holds.

    // shift := number of trailing 0s in $b0
    // (      = number of leading 0s in $t1; see the "rbit" instruction in TEST_B_ZERO)
    clz     $shift, $t1

    // If there is no shift, goto shift_A_Y
    cbz     $shift, .Lbeeu_shift_A_Y

    // Shift B right by "$shift" bits
    ${\SHIFT256($b0, $b1, $b2, $b3)}

    // Shift X right by "$shift" bits, adding n whenever X becomes odd.
    // $shift--;
    // $t0 := 0; needed in the addition to the most significant word in SHIFT1
    eor     $t0, $t0, $t0
.Lbeeu_shift_loop_X:
    ${\SHIFT1($x0, $x1, $x2, $x3, $x4)}
    subs    $shift, $shift, #1
    bne     .Lbeeu_shift_loop_X

    // Note: the steps above perform the same sequence as in p256_beeu-x86_64-asm.pl
    // with the following differences:
    // - "$shift" is set directly to the number of trailing 0s in B
    //   (using rbit and clz instructions)
    // - The loop is only used to call SHIFT1(X)
    //   and $shift is decreased while executing the X loop.
    // - SHIFT256(B, $shift) is performed before right-shifting X; they are independent

.Lbeeu_shift_A_Y:
    // Same for A and Y.
    // Afterwards, (2) still holds.
    // Reverse the bit order of $a0
    // $shift := number of trailing 0s in $a0 (= number of leading 0s in $t1)
    rbit    $t1, $a0
    clz     $shift, $t1

    // If there is no shift, goto |B-A|, X+Y update
    cbz     $shift, .Lbeeu_update_B_X_or_A_Y

    // Shift A right by "$shift" bits
    ${\SHIFT256($a0, $a1, $a2, $a3)}

    // Shift Y right by "$shift" bits, adding n whenever Y becomes odd.
    // $shift--;
    // $t0 := 0; needed in the addition to the most significant word in SHIFT1
    eor     $t0, $t0, $t0
.Lbeeu_shift_loop_Y:
    ${\SHIFT1($y0, $y1, $y2, $y3, $y4)}
    subs    $shift, $shift, #1
    bne     .Lbeeu_shift_loop_Y

.Lbeeu_update_B_X_or_A_Y:
    // Try T := B - A; if cs, continue with B > A (cs: carry set = no borrow)
    // Note: this is a case of unsigned arithmetic, where T fits in 4 64-bit words
    //       without taking a sign bit if generated. The lack of a carry would
    //       indicate a negative result. See, for example,
    //       https://community.arm.com/developer/ip-products/processors/b/processors-ip-blog/posts/condition-codes-1-condition-flags-and-codes
    subs    $t0, $b0, $a0
    sbcs    $t1, $b1, $a1
    sbcs    $t2, $b2, $a2
    sbcs    $t3, $b3, $a3
    bcs     .Lbeeu_B_greater_than_A

    // Else A > B =>
    // A := A - B; Y := Y + X; goto beginning of the loop
    subs    $a0, $a0, $b0
    sbcs    $a1, $a1, $b1
    sbcs    $a2, $a2, $b2
    sbcs    $a3, $a3, $b3

    adds    $y0, $y0, $x0
    adcs    $y1, $y1, $x1
    adcs    $y2, $y2, $x2
    adcs    $y3, $y3, $x3
    adc     $y4, $y4, $x4
    b       .Lbeeu_loop

.Lbeeu_B_greater_than_A:
    // Continue with B > A =>
    // B := B - A; X := X + Y; goto beginning of the loop
    mov     $b0, $t0
    mov     $b1, $t1
    mov     $b2, $t2
    mov     $b3, $t3

    adds    $x0, $x0, $y0
    adcs    $x1, $x1, $y1
    adcs    $x2, $x2, $y2
    adcs    $x3, $x3, $y3
    adc     $x4, $x4, $y4
    b       .Lbeeu_loop

.Lbeeu_loop_end:
    // The Euclid's algorithm loop ends when A == gcd(a,n);
    // this would be 1, when a and n are co-prime (i.e. do not have a common factor).
    // Since (-1)*Y*a == A (mod |n|), Y>0
    // then out = -Y mod n

    // Verify that A = 1 ==> (-1)*Y*a = A = 1  (mod |n|)
    // Is A-1 == 0?
    // If not, fail.
    sub     $t0, $a0, #1
    orr     $t0, $t0, $a1
    orr     $t0, $t0, $a2
    orr     $t0, $t0, $a3
    cbnz    $t0, .Lbeeu_err

    // If Y>n ==> Y:=Y-n
.Lbeeu_reduction_loop:
    // x_i := y_i - n_i (X is no longer needed, use it as temp)
    // ($t0 = 0 from above)
    subs    $x0, $y0, $n0
    sbcs    $x1, $y1, $n1
    sbcs    $x2, $y2, $n2
    sbcs    $x3, $y3, $n3
    sbcs    $x4, $y4, $t0

    // If result is non-negative (i.e., cs = carry set = no borrow),
    // y_i := x_i; goto reduce again
    // else
    // y_i := y_i; continue
    csel    $y0, $x0, $y0, cs
    csel    $y1, $x1, $y1, cs
    csel    $y2, $x2, $y2, cs
    csel    $y3, $x3, $y3, cs
    csel    $y4, $x4, $y4, cs
    bcs     .Lbeeu_reduction_loop

    // Now Y < n (Y cannot be equal to n, since the inverse cannot be 0)
    // out = -Y = n-Y
    subs    $y0, $n0, $y0
    sbcs    $y1, $n1, $y1
    sbcs    $y2, $n2, $y2
    sbcs    $y3, $n3, $y3

    // Save Y in output (out (x0) was saved on the stack)
    ldr     x3, [sp,#96]
    stp     $y0, $y1, [x3]
    stp     $y2, $y3, [x3,#16]
    // return 1 (success)
    mov     x0, #1
    b       .Lbeeu_finish

.Lbeeu_err:
    // return 0 (error)
    eor     x0, x0, x0

.Lbeeu_finish:
    // Restore callee-saved registers, except x0, x2
    add     sp,x29,#0
    ldp     x19,x20,[sp,#16]
    ldp     x21,x22,[sp,#32]
    ldp     x23,x24,[sp,#48]
    ldp     x25,x26,[sp,#64]
    ldp     x27,x28,[sp,#80]
    ldp     x29,x30,[sp],#112

    AARCH64_VALIDATE_LINK_REGISTER
    ret
.size beeu_mod_inverse_vartime,.-beeu_mod_inverse_vartime
___


foreach (split("\n",$code)) {
    s/\`([^\`]*)\`/eval $1/ge;

    print $_,"\n";
}
close STDOUT or die "error closing STDOUT: $!"; # enforce flush
