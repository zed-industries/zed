// This file is generated from a similarly-named Perl script in the BoringSSL
// source tree. Do not edit by hand.

#include <openssl/asm_base.h>

#if !defined(OPENSSL_NO_ASM) && defined(OPENSSL_AARCH64) && defined(_WIN32)
#include "openssl/arm_arch.h"

.text
.globl	beeu_mod_inverse_vartime


.align	4
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
	stp	x29,x30,[sp,#-112]!
	add	x29,sp,#0
	stp	x19,x20,[sp,#16]
	stp	x21,x22,[sp,#32]
	stp	x23,x24,[sp,#48]
	stp	x25,x26,[sp,#64]
	stp	x27,x28,[sp,#80]
	stp	x0,x2,[sp,#96]

    // B = b3..b0 := a
	ldp	x25,x26,[x1]
	ldp	x27,x28,[x1,#16]

    // n3..n0 := n
    // Note: the value of input params are changed in the following.
	ldp	x0,x1,[x2]
	ldp	x2,x30,[x2,#16]

    // A = a3..a0 := n
	mov	x21, x0
	mov	x22, x1
	mov	x23, x2
	mov	x24, x30

    // X = x4..x0 := 1
	mov	x3, #1
	eor	x4, x4, x4
	eor	x5, x5, x5
	eor	x6, x6, x6
	eor	x7, x7, x7

    // Y = y4..y0 := 0
	eor	x8, x8, x8
	eor	x9, x9, x9
	eor	x10, x10, x10
	eor	x11, x11, x11
	eor	x12, x12, x12

Lbeeu_loop:
    // if B == 0, jump to .Lbeeu_loop_end
	orr	x14, x25, x26
	orr	x14, x14, x27

    // reverse the bit order of x25. This is needed for clz after this macro
	rbit	x15, x25

	orr	x14, x14, x28
	cbz	x14,Lbeeu_loop_end


    // 0 < B < |n|,
    // 0 < A <= |n|,
    // (1)      X*a  ==  B   (mod |n|),
    // (2) (-1)*Y*a  ==  A   (mod |n|)

    // Now divide B by the maximum possible power of two in the
    // integers, and divide X by the same value mod |n|.
    // When we're done, (1) still holds.

    // shift := number of trailing 0s in x25
    // (      = number of leading 0s in x15; see the "rbit" instruction in TEST_B_ZERO)
	clz	x13, x15

    // If there is no shift, goto shift_A_Y
	cbz	x13, Lbeeu_shift_A_Y

    // Shift B right by "x13" bits
	neg	x14, x13
	lsr	x25, x25, x13
	lsl	x15, x26, x14

	lsr	x26, x26, x13
	lsl	x19, x27, x14

	orr	x25, x25, x15

	lsr	x27, x27, x13
	lsl	x20, x28, x14

	orr	x26, x26, x19

	lsr	x28, x28, x13

	orr	x27, x27, x20


    // Shift X right by "x13" bits, adding n whenever X becomes odd.
    // x13--;
    // x14 := 0; needed in the addition to the most significant word in SHIFT1
	eor	x14, x14, x14
Lbeeu_shift_loop_X:
	tbz	x3, #0, Lshift1_0
	adds	x3, x3, x0
	adcs	x4, x4, x1
	adcs	x5, x5, x2
	adcs	x6, x6, x30
	adc	x7, x7, x14
Lshift1_0:
    // var0 := [var1|var0]<64..1>;
    // i.e. concatenate var1 and var0,
    //      extract bits <64..1> from the resulting 128-bit value
    //      and put them in var0
	extr	x3, x4, x3, #1
	extr	x4, x5, x4, #1
	extr	x5, x6, x5, #1
	extr	x6, x7, x6, #1
	lsr	x7, x7, #1

	subs	x13, x13, #1
	bne	Lbeeu_shift_loop_X

    // Note: the steps above perform the same sequence as in p256_beeu-x86_64-asm.pl
    // with the following differences:
    // - "x13" is set directly to the number of trailing 0s in B
    //   (using rbit and clz instructions)
    // - The loop is only used to call SHIFT1(X)
    //   and x13 is decreased while executing the X loop.
    // - SHIFT256(B, x13) is performed before right-shifting X; they are independent

Lbeeu_shift_A_Y:
    // Same for A and Y.
    // Afterwards, (2) still holds.
    // Reverse the bit order of x21
    // x13 := number of trailing 0s in x21 (= number of leading 0s in x15)
	rbit	x15, x21
	clz	x13, x15

    // If there is no shift, goto |B-A|, X+Y update
	cbz	x13, Lbeeu_update_B_X_or_A_Y

    // Shift A right by "x13" bits
	neg	x14, x13
	lsr	x21, x21, x13
	lsl	x15, x22, x14

	lsr	x22, x22, x13
	lsl	x19, x23, x14

	orr	x21, x21, x15

	lsr	x23, x23, x13
	lsl	x20, x24, x14

	orr	x22, x22, x19

	lsr	x24, x24, x13

	orr	x23, x23, x20


    // Shift Y right by "x13" bits, adding n whenever Y becomes odd.
    // x13--;
    // x14 := 0; needed in the addition to the most significant word in SHIFT1
	eor	x14, x14, x14
Lbeeu_shift_loop_Y:
	tbz	x8, #0, Lshift1_1
	adds	x8, x8, x0
	adcs	x9, x9, x1
	adcs	x10, x10, x2
	adcs	x11, x11, x30
	adc	x12, x12, x14
Lshift1_1:
    // var0 := [var1|var0]<64..1>;
    // i.e. concatenate var1 and var0,
    //      extract bits <64..1> from the resulting 128-bit value
    //      and put them in var0
	extr	x8, x9, x8, #1
	extr	x9, x10, x9, #1
	extr	x10, x11, x10, #1
	extr	x11, x12, x11, #1
	lsr	x12, x12, #1

	subs	x13, x13, #1
	bne	Lbeeu_shift_loop_Y

Lbeeu_update_B_X_or_A_Y:
    // Try T := B - A; if cs, continue with B > A (cs: carry set = no borrow)
    // Note: this is a case of unsigned arithmetic, where T fits in 4 64-bit words
    //       without taking a sign bit if generated. The lack of a carry would
    //       indicate a negative result. See, for example,
    //       https://community.arm.com/developer/ip-products/processors/b/processors-ip-blog/posts/condition-codes-1-condition-flags-and-codes
	subs	x14, x25, x21
	sbcs	x15, x26, x22
	sbcs	x19, x27, x23
	sbcs	x20, x28, x24
	bcs	Lbeeu_B_greater_than_A

    // Else A > B =>
    // A := A - B; Y := Y + X; goto beginning of the loop
	subs	x21, x21, x25
	sbcs	x22, x22, x26
	sbcs	x23, x23, x27
	sbcs	x24, x24, x28

	adds	x8, x8, x3
	adcs	x9, x9, x4
	adcs	x10, x10, x5
	adcs	x11, x11, x6
	adc	x12, x12, x7
	b	Lbeeu_loop

Lbeeu_B_greater_than_A:
    // Continue with B > A =>
    // B := B - A; X := X + Y; goto beginning of the loop
	mov	x25, x14
	mov	x26, x15
	mov	x27, x19
	mov	x28, x20

	adds	x3, x3, x8
	adcs	x4, x4, x9
	adcs	x5, x5, x10
	adcs	x6, x6, x11
	adc	x7, x7, x12
	b	Lbeeu_loop

Lbeeu_loop_end:
    // The Euclid's algorithm loop ends when A == gcd(a,n);
    // this would be 1, when a and n are co-prime (i.e. do not have a common factor).
    // Since (-1)*Y*a == A (mod |n|), Y>0
    // then out = -Y mod n

    // Verify that A = 1 ==> (-1)*Y*a = A = 1  (mod |n|)
    // Is A-1 == 0?
    // If not, fail.
	sub	x14, x21, #1
	orr	x14, x14, x22
	orr	x14, x14, x23
	orr	x14, x14, x24
	cbnz	x14, Lbeeu_err

    // If Y>n ==> Y:=Y-n
Lbeeu_reduction_loop:
    // x_i := y_i - n_i (X is no longer needed, use it as temp)
    // (x14 = 0 from above)
	subs	x3, x8, x0
	sbcs	x4, x9, x1
	sbcs	x5, x10, x2
	sbcs	x6, x11, x30
	sbcs	x7, x12, x14

    // If result is non-negative (i.e., cs = carry set = no borrow),
    // y_i := x_i; goto reduce again
    // else
    // y_i := y_i; continue
	csel	x8, x3, x8, cs
	csel	x9, x4, x9, cs
	csel	x10, x5, x10, cs
	csel	x11, x6, x11, cs
	csel	x12, x7, x12, cs
	bcs	Lbeeu_reduction_loop

    // Now Y < n (Y cannot be equal to n, since the inverse cannot be 0)
    // out = -Y = n-Y
	subs	x8, x0, x8
	sbcs	x9, x1, x9
	sbcs	x10, x2, x10
	sbcs	x11, x30, x11

    // Save Y in output (out (x0) was saved on the stack)
	ldr	x3, [sp,#96]
	stp	x8, x9, [x3]
	stp	x10, x11, [x3,#16]
    // return 1 (success)
	mov	x0, #1
	b	Lbeeu_finish

Lbeeu_err:
    // return 0 (error)
	eor	x0, x0, x0

Lbeeu_finish:
    // Restore callee-saved registers, except x0, x2
	add	sp,x29,#0
	ldp	x19,x20,[sp,#16]
	ldp	x21,x22,[sp,#32]
	ldp	x23,x24,[sp,#48]
	ldp	x25,x26,[sp,#64]
	ldp	x27,x28,[sp,#80]
	ldp	x29,x30,[sp],#112

	AARCH64_VALIDATE_LINK_REGISTER
	ret

#endif  // !OPENSSL_NO_ASM && defined(OPENSSL_AARCH64) && defined(_WIN32)
