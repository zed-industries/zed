// This file is generated from a similarly-named Perl script in the BoringSSL
// source tree. Do not edit by hand.

#include <openssl/asm_base.h>

#if !defined(OPENSSL_NO_ASM) && defined(__powerpc64__) && defined(__ELF__)
#include <openssl/boringssl_prefix_symbols_asm.h>
.machine	"any"
.abiversion	2
.text







.globl	abi_test_trampoline
.type	abi_test_trampoline,@function
.align	5
abi_test_trampoline:
.localentry	abi_test_trampoline,0


	mflr	0
	std	0, 16(1)



















	stdu	1, -528(1)

	mfcr	0
	std	0, 8(1)
	std	2, 24(1)
	std	4, 32(1)
	li	11, 48
	stvx	20, 11, 1
	li	11, 64
	stvx	21, 11, 1
	li	11, 80
	stvx	22, 11, 1
	li	11, 96
	stvx	23, 11, 1
	li	11, 112
	stvx	24, 11, 1
	li	11, 128
	stvx	25, 11, 1
	li	11, 144
	stvx	26, 11, 1
	li	11, 160
	stvx	27, 11, 1
	li	11, 176
	stvx	28, 11, 1
	li	11, 192
	stvx	29, 11, 1
	li	11, 208
	stvx	30, 11, 1
	li	11, 224
	stvx	31, 11, 1
	std	14, 240(1)
	std	15, 248(1)
	std	16, 256(1)
	std	17, 264(1)
	std	18, 272(1)
	std	19, 280(1)
	std	20, 288(1)
	std	21, 296(1)
	std	22, 304(1)
	std	23, 312(1)
	std	24, 320(1)
	std	25, 328(1)
	std	26, 336(1)
	std	27, 344(1)
	std	28, 352(1)
	std	29, 360(1)
	std	30, 368(1)
	std	31, 376(1)
	stfd	14, 384(1)
	stfd	15, 392(1)
	stfd	16, 400(1)
	stfd	17, 408(1)
	stfd	18, 416(1)
	stfd	19, 424(1)
	stfd	20, 432(1)
	stfd	21, 440(1)
	stfd	22, 448(1)
	stfd	23, 456(1)
	stfd	24, 464(1)
	stfd	25, 472(1)
	stfd	26, 480(1)
	stfd	27, 488(1)
	stfd	28, 496(1)
	stfd	29, 504(1)
	stfd	30, 512(1)
	stfd	31, 520(1)
	li	11, 0
	lvx	20, 11, 4
	li	11, 16
	lvx	21, 11, 4
	li	11, 32
	lvx	22, 11, 4
	li	11, 48
	lvx	23, 11, 4
	li	11, 64
	lvx	24, 11, 4
	li	11, 80
	lvx	25, 11, 4
	li	11, 96
	lvx	26, 11, 4
	li	11, 112
	lvx	27, 11, 4
	li	11, 128
	lvx	28, 11, 4
	li	11, 144
	lvx	29, 11, 4
	li	11, 160
	lvx	30, 11, 4
	li	11, 176
	lvx	31, 11, 4
	ld	14, 192(4)
	ld	15, 200(4)
	ld	16, 208(4)
	ld	17, 216(4)
	ld	18, 224(4)
	ld	19, 232(4)
	ld	20, 240(4)
	ld	21, 248(4)
	ld	22, 256(4)
	ld	23, 264(4)
	ld	24, 272(4)
	ld	25, 280(4)
	ld	26, 288(4)
	ld	27, 296(4)
	ld	28, 304(4)
	ld	29, 312(4)
	ld	30, 320(4)
	ld	31, 328(4)
	lfd	14, 336(4)
	lfd	15, 344(4)
	lfd	16, 352(4)
	lfd	17, 360(4)
	lfd	18, 368(4)
	lfd	19, 376(4)
	lfd	20, 384(4)
	lfd	21, 392(4)
	lfd	22, 400(4)
	lfd	23, 408(4)
	lfd	24, 416(4)
	lfd	25, 424(4)
	lfd	26, 432(4)
	lfd	27, 440(4)
	lfd	28, 448(4)
	lfd	29, 456(4)
	lfd	30, 464(4)
	lfd	31, 472(4)

	ld	0, 480(4)
	mtcr	0


	addi	11, 5, -8
	mr	12, 3


	cmpdi	6, 0
	beq	.Largs_done
	mtctr	6
	ldu	3, 8(11)
	bdz	.Largs_done
	ldu	4, 8(11)
	bdz	.Largs_done
	ldu	5, 8(11)
	bdz	.Largs_done
	ldu	6, 8(11)
	bdz	.Largs_done
	ldu	7, 8(11)
	bdz	.Largs_done
	ldu	8, 8(11)
	bdz	.Largs_done
	ldu	9, 8(11)
	bdz	.Largs_done
	ldu	10, 8(11)

.Largs_done:
	li	2, 0
	mtctr	12
	bctrl	
	ld	2, 24(1)

	ld	4, 32(1)
	li	11, 0
	stvx	20, 11, 4
	li	11, 16
	stvx	21, 11, 4
	li	11, 32
	stvx	22, 11, 4
	li	11, 48
	stvx	23, 11, 4
	li	11, 64
	stvx	24, 11, 4
	li	11, 80
	stvx	25, 11, 4
	li	11, 96
	stvx	26, 11, 4
	li	11, 112
	stvx	27, 11, 4
	li	11, 128
	stvx	28, 11, 4
	li	11, 144
	stvx	29, 11, 4
	li	11, 160
	stvx	30, 11, 4
	li	11, 176
	stvx	31, 11, 4
	std	14, 192(4)
	std	15, 200(4)
	std	16, 208(4)
	std	17, 216(4)
	std	18, 224(4)
	std	19, 232(4)
	std	20, 240(4)
	std	21, 248(4)
	std	22, 256(4)
	std	23, 264(4)
	std	24, 272(4)
	std	25, 280(4)
	std	26, 288(4)
	std	27, 296(4)
	std	28, 304(4)
	std	29, 312(4)
	std	30, 320(4)
	std	31, 328(4)
	stfd	14, 336(4)
	stfd	15, 344(4)
	stfd	16, 352(4)
	stfd	17, 360(4)
	stfd	18, 368(4)
	stfd	19, 376(4)
	stfd	20, 384(4)
	stfd	21, 392(4)
	stfd	22, 400(4)
	stfd	23, 408(4)
	stfd	24, 416(4)
	stfd	25, 424(4)
	stfd	26, 432(4)
	stfd	27, 440(4)
	stfd	28, 448(4)
	stfd	29, 456(4)
	stfd	30, 464(4)
	stfd	31, 472(4)
	li	11, 48
	lvx	20, 11, 1
	li	11, 64
	lvx	21, 11, 1
	li	11, 80
	lvx	22, 11, 1
	li	11, 96
	lvx	23, 11, 1
	li	11, 112
	lvx	24, 11, 1
	li	11, 128
	lvx	25, 11, 1
	li	11, 144
	lvx	26, 11, 1
	li	11, 160
	lvx	27, 11, 1
	li	11, 176
	lvx	28, 11, 1
	li	11, 192
	lvx	29, 11, 1
	li	11, 208
	lvx	30, 11, 1
	li	11, 224
	lvx	31, 11, 1
	ld	14, 240(1)
	ld	15, 248(1)
	ld	16, 256(1)
	ld	17, 264(1)
	ld	18, 272(1)
	ld	19, 280(1)
	ld	20, 288(1)
	ld	21, 296(1)
	ld	22, 304(1)
	ld	23, 312(1)
	ld	24, 320(1)
	ld	25, 328(1)
	ld	26, 336(1)
	ld	27, 344(1)
	ld	28, 352(1)
	ld	29, 360(1)
	ld	30, 368(1)
	ld	31, 376(1)
	lfd	14, 384(1)
	lfd	15, 392(1)
	lfd	16, 400(1)
	lfd	17, 408(1)
	lfd	18, 416(1)
	lfd	19, 424(1)
	lfd	20, 432(1)
	lfd	21, 440(1)
	lfd	22, 448(1)
	lfd	23, 456(1)
	lfd	24, 464(1)
	lfd	25, 472(1)
	lfd	26, 480(1)
	lfd	27, 488(1)
	lfd	28, 496(1)
	lfd	29, 504(1)
	lfd	30, 512(1)
	lfd	31, 520(1)
	mfcr	0
	std	0, 480(4)
	ld	0, 8(1)
	mtcrf	0b00111000, 0
	addi	1, 1, 528
	ld	0, 16(1)
	mtlr	0
	blr	
.size	abi_test_trampoline,.-abi_test_trampoline
.globl	abi_test_clobber_r0
.type	abi_test_clobber_r0,@function
.align	5
abi_test_clobber_r0:
.localentry	abi_test_clobber_r0,0

	li	0, 0
	blr	
.size	abi_test_clobber_r0,.-abi_test_clobber_r0
.globl	abi_test_clobber_r2
.type	abi_test_clobber_r2,@function
.align	5
abi_test_clobber_r2:
.localentry	abi_test_clobber_r2,0

	li	2, 0
	blr	
.size	abi_test_clobber_r2,.-abi_test_clobber_r2
.globl	abi_test_clobber_r3
.type	abi_test_clobber_r3,@function
.align	5
abi_test_clobber_r3:
.localentry	abi_test_clobber_r3,0

	li	3, 0
	blr	
.size	abi_test_clobber_r3,.-abi_test_clobber_r3
.globl	abi_test_clobber_r4
.type	abi_test_clobber_r4,@function
.align	5
abi_test_clobber_r4:
.localentry	abi_test_clobber_r4,0

	li	4, 0
	blr	
.size	abi_test_clobber_r4,.-abi_test_clobber_r4
.globl	abi_test_clobber_r5
.type	abi_test_clobber_r5,@function
.align	5
abi_test_clobber_r5:
.localentry	abi_test_clobber_r5,0

	li	5, 0
	blr	
.size	abi_test_clobber_r5,.-abi_test_clobber_r5
.globl	abi_test_clobber_r6
.type	abi_test_clobber_r6,@function
.align	5
abi_test_clobber_r6:
.localentry	abi_test_clobber_r6,0

	li	6, 0
	blr	
.size	abi_test_clobber_r6,.-abi_test_clobber_r6
.globl	abi_test_clobber_r7
.type	abi_test_clobber_r7,@function
.align	5
abi_test_clobber_r7:
.localentry	abi_test_clobber_r7,0

	li	7, 0
	blr	
.size	abi_test_clobber_r7,.-abi_test_clobber_r7
.globl	abi_test_clobber_r8
.type	abi_test_clobber_r8,@function
.align	5
abi_test_clobber_r8:
.localentry	abi_test_clobber_r8,0

	li	8, 0
	blr	
.size	abi_test_clobber_r8,.-abi_test_clobber_r8
.globl	abi_test_clobber_r9
.type	abi_test_clobber_r9,@function
.align	5
abi_test_clobber_r9:
.localentry	abi_test_clobber_r9,0

	li	9, 0
	blr	
.size	abi_test_clobber_r9,.-abi_test_clobber_r9
.globl	abi_test_clobber_r10
.type	abi_test_clobber_r10,@function
.align	5
abi_test_clobber_r10:
.localentry	abi_test_clobber_r10,0

	li	10, 0
	blr	
.size	abi_test_clobber_r10,.-abi_test_clobber_r10
.globl	abi_test_clobber_r11
.type	abi_test_clobber_r11,@function
.align	5
abi_test_clobber_r11:
.localentry	abi_test_clobber_r11,0

	li	11, 0
	blr	
.size	abi_test_clobber_r11,.-abi_test_clobber_r11
.globl	abi_test_clobber_r12
.type	abi_test_clobber_r12,@function
.align	5
abi_test_clobber_r12:
.localentry	abi_test_clobber_r12,0

	li	12, 0
	blr	
.size	abi_test_clobber_r12,.-abi_test_clobber_r12
.globl	abi_test_clobber_r14
.type	abi_test_clobber_r14,@function
.align	5
abi_test_clobber_r14:
.localentry	abi_test_clobber_r14,0

	li	14, 0
	blr	
.size	abi_test_clobber_r14,.-abi_test_clobber_r14
.globl	abi_test_clobber_r15
.type	abi_test_clobber_r15,@function
.align	5
abi_test_clobber_r15:
.localentry	abi_test_clobber_r15,0

	li	15, 0
	blr	
.size	abi_test_clobber_r15,.-abi_test_clobber_r15
.globl	abi_test_clobber_r16
.type	abi_test_clobber_r16,@function
.align	5
abi_test_clobber_r16:
.localentry	abi_test_clobber_r16,0

	li	16, 0
	blr	
.size	abi_test_clobber_r16,.-abi_test_clobber_r16
.globl	abi_test_clobber_r17
.type	abi_test_clobber_r17,@function
.align	5
abi_test_clobber_r17:
.localentry	abi_test_clobber_r17,0

	li	17, 0
	blr	
.size	abi_test_clobber_r17,.-abi_test_clobber_r17
.globl	abi_test_clobber_r18
.type	abi_test_clobber_r18,@function
.align	5
abi_test_clobber_r18:
.localentry	abi_test_clobber_r18,0

	li	18, 0
	blr	
.size	abi_test_clobber_r18,.-abi_test_clobber_r18
.globl	abi_test_clobber_r19
.type	abi_test_clobber_r19,@function
.align	5
abi_test_clobber_r19:
.localentry	abi_test_clobber_r19,0

	li	19, 0
	blr	
.size	abi_test_clobber_r19,.-abi_test_clobber_r19
.globl	abi_test_clobber_r20
.type	abi_test_clobber_r20,@function
.align	5
abi_test_clobber_r20:
.localentry	abi_test_clobber_r20,0

	li	20, 0
	blr	
.size	abi_test_clobber_r20,.-abi_test_clobber_r20
.globl	abi_test_clobber_r21
.type	abi_test_clobber_r21,@function
.align	5
abi_test_clobber_r21:
.localentry	abi_test_clobber_r21,0

	li	21, 0
	blr	
.size	abi_test_clobber_r21,.-abi_test_clobber_r21
.globl	abi_test_clobber_r22
.type	abi_test_clobber_r22,@function
.align	5
abi_test_clobber_r22:
.localentry	abi_test_clobber_r22,0

	li	22, 0
	blr	
.size	abi_test_clobber_r22,.-abi_test_clobber_r22
.globl	abi_test_clobber_r23
.type	abi_test_clobber_r23,@function
.align	5
abi_test_clobber_r23:
.localentry	abi_test_clobber_r23,0

	li	23, 0
	blr	
.size	abi_test_clobber_r23,.-abi_test_clobber_r23
.globl	abi_test_clobber_r24
.type	abi_test_clobber_r24,@function
.align	5
abi_test_clobber_r24:
.localentry	abi_test_clobber_r24,0

	li	24, 0
	blr	
.size	abi_test_clobber_r24,.-abi_test_clobber_r24
.globl	abi_test_clobber_r25
.type	abi_test_clobber_r25,@function
.align	5
abi_test_clobber_r25:
.localentry	abi_test_clobber_r25,0

	li	25, 0
	blr	
.size	abi_test_clobber_r25,.-abi_test_clobber_r25
.globl	abi_test_clobber_r26
.type	abi_test_clobber_r26,@function
.align	5
abi_test_clobber_r26:
.localentry	abi_test_clobber_r26,0

	li	26, 0
	blr	
.size	abi_test_clobber_r26,.-abi_test_clobber_r26
.globl	abi_test_clobber_r27
.type	abi_test_clobber_r27,@function
.align	5
abi_test_clobber_r27:
.localentry	abi_test_clobber_r27,0

	li	27, 0
	blr	
.size	abi_test_clobber_r27,.-abi_test_clobber_r27
.globl	abi_test_clobber_r28
.type	abi_test_clobber_r28,@function
.align	5
abi_test_clobber_r28:
.localentry	abi_test_clobber_r28,0

	li	28, 0
	blr	
.size	abi_test_clobber_r28,.-abi_test_clobber_r28
.globl	abi_test_clobber_r29
.type	abi_test_clobber_r29,@function
.align	5
abi_test_clobber_r29:
.localentry	abi_test_clobber_r29,0

	li	29, 0
	blr	
.size	abi_test_clobber_r29,.-abi_test_clobber_r29
.globl	abi_test_clobber_r30
.type	abi_test_clobber_r30,@function
.align	5
abi_test_clobber_r30:
.localentry	abi_test_clobber_r30,0

	li	30, 0
	blr	
.size	abi_test_clobber_r30,.-abi_test_clobber_r30
.globl	abi_test_clobber_r31
.type	abi_test_clobber_r31,@function
.align	5
abi_test_clobber_r31:
.localentry	abi_test_clobber_r31,0

	li	31, 0
	blr	
.size	abi_test_clobber_r31,.-abi_test_clobber_r31
.globl	abi_test_clobber_f0
.type	abi_test_clobber_f0,@function
.align	4
abi_test_clobber_f0:
.localentry	abi_test_clobber_f0,0

	li	0, 0

	std	0, -8(1)
	lfd	0, -8(1)
	blr	
.size	abi_test_clobber_f0,.-abi_test_clobber_f0
.globl	abi_test_clobber_f1
.type	abi_test_clobber_f1,@function
.align	4
abi_test_clobber_f1:
.localentry	abi_test_clobber_f1,0

	li	0, 0

	std	0, -8(1)
	lfd	1, -8(1)
	blr	
.size	abi_test_clobber_f1,.-abi_test_clobber_f1
.globl	abi_test_clobber_f2
.type	abi_test_clobber_f2,@function
.align	4
abi_test_clobber_f2:
.localentry	abi_test_clobber_f2,0

	li	0, 0

	std	0, -8(1)
	lfd	2, -8(1)
	blr	
.size	abi_test_clobber_f2,.-abi_test_clobber_f2
.globl	abi_test_clobber_f3
.type	abi_test_clobber_f3,@function
.align	4
abi_test_clobber_f3:
.localentry	abi_test_clobber_f3,0

	li	0, 0

	std	0, -8(1)
	lfd	3, -8(1)
	blr	
.size	abi_test_clobber_f3,.-abi_test_clobber_f3
.globl	abi_test_clobber_f4
.type	abi_test_clobber_f4,@function
.align	4
abi_test_clobber_f4:
.localentry	abi_test_clobber_f4,0

	li	0, 0

	std	0, -8(1)
	lfd	4, -8(1)
	blr	
.size	abi_test_clobber_f4,.-abi_test_clobber_f4
.globl	abi_test_clobber_f5
.type	abi_test_clobber_f5,@function
.align	4
abi_test_clobber_f5:
.localentry	abi_test_clobber_f5,0

	li	0, 0

	std	0, -8(1)
	lfd	5, -8(1)
	blr	
.size	abi_test_clobber_f5,.-abi_test_clobber_f5
.globl	abi_test_clobber_f6
.type	abi_test_clobber_f6,@function
.align	4
abi_test_clobber_f6:
.localentry	abi_test_clobber_f6,0

	li	0, 0

	std	0, -8(1)
	lfd	6, -8(1)
	blr	
.size	abi_test_clobber_f6,.-abi_test_clobber_f6
.globl	abi_test_clobber_f7
.type	abi_test_clobber_f7,@function
.align	4
abi_test_clobber_f7:
.localentry	abi_test_clobber_f7,0

	li	0, 0

	std	0, -8(1)
	lfd	7, -8(1)
	blr	
.size	abi_test_clobber_f7,.-abi_test_clobber_f7
.globl	abi_test_clobber_f8
.type	abi_test_clobber_f8,@function
.align	4
abi_test_clobber_f8:
.localentry	abi_test_clobber_f8,0

	li	0, 0

	std	0, -8(1)
	lfd	8, -8(1)
	blr	
.size	abi_test_clobber_f8,.-abi_test_clobber_f8
.globl	abi_test_clobber_f9
.type	abi_test_clobber_f9,@function
.align	4
abi_test_clobber_f9:
.localentry	abi_test_clobber_f9,0

	li	0, 0

	std	0, -8(1)
	lfd	9, -8(1)
	blr	
.size	abi_test_clobber_f9,.-abi_test_clobber_f9
.globl	abi_test_clobber_f10
.type	abi_test_clobber_f10,@function
.align	4
abi_test_clobber_f10:
.localentry	abi_test_clobber_f10,0

	li	0, 0

	std	0, -8(1)
	lfd	10, -8(1)
	blr	
.size	abi_test_clobber_f10,.-abi_test_clobber_f10
.globl	abi_test_clobber_f11
.type	abi_test_clobber_f11,@function
.align	4
abi_test_clobber_f11:
.localentry	abi_test_clobber_f11,0

	li	0, 0

	std	0, -8(1)
	lfd	11, -8(1)
	blr	
.size	abi_test_clobber_f11,.-abi_test_clobber_f11
.globl	abi_test_clobber_f12
.type	abi_test_clobber_f12,@function
.align	4
abi_test_clobber_f12:
.localentry	abi_test_clobber_f12,0

	li	0, 0

	std	0, -8(1)
	lfd	12, -8(1)
	blr	
.size	abi_test_clobber_f12,.-abi_test_clobber_f12
.globl	abi_test_clobber_f13
.type	abi_test_clobber_f13,@function
.align	4
abi_test_clobber_f13:
.localentry	abi_test_clobber_f13,0

	li	0, 0

	std	0, -8(1)
	lfd	13, -8(1)
	blr	
.size	abi_test_clobber_f13,.-abi_test_clobber_f13
.globl	abi_test_clobber_f14
.type	abi_test_clobber_f14,@function
.align	4
abi_test_clobber_f14:
.localentry	abi_test_clobber_f14,0

	li	0, 0

	std	0, -8(1)
	lfd	14, -8(1)
	blr	
.size	abi_test_clobber_f14,.-abi_test_clobber_f14
.globl	abi_test_clobber_f15
.type	abi_test_clobber_f15,@function
.align	4
abi_test_clobber_f15:
.localentry	abi_test_clobber_f15,0

	li	0, 0

	std	0, -8(1)
	lfd	15, -8(1)
	blr	
.size	abi_test_clobber_f15,.-abi_test_clobber_f15
.globl	abi_test_clobber_f16
.type	abi_test_clobber_f16,@function
.align	4
abi_test_clobber_f16:
.localentry	abi_test_clobber_f16,0

	li	0, 0

	std	0, -8(1)
	lfd	16, -8(1)
	blr	
.size	abi_test_clobber_f16,.-abi_test_clobber_f16
.globl	abi_test_clobber_f17
.type	abi_test_clobber_f17,@function
.align	4
abi_test_clobber_f17:
.localentry	abi_test_clobber_f17,0

	li	0, 0

	std	0, -8(1)
	lfd	17, -8(1)
	blr	
.size	abi_test_clobber_f17,.-abi_test_clobber_f17
.globl	abi_test_clobber_f18
.type	abi_test_clobber_f18,@function
.align	4
abi_test_clobber_f18:
.localentry	abi_test_clobber_f18,0

	li	0, 0

	std	0, -8(1)
	lfd	18, -8(1)
	blr	
.size	abi_test_clobber_f18,.-abi_test_clobber_f18
.globl	abi_test_clobber_f19
.type	abi_test_clobber_f19,@function
.align	4
abi_test_clobber_f19:
.localentry	abi_test_clobber_f19,0

	li	0, 0

	std	0, -8(1)
	lfd	19, -8(1)
	blr	
.size	abi_test_clobber_f19,.-abi_test_clobber_f19
.globl	abi_test_clobber_f20
.type	abi_test_clobber_f20,@function
.align	4
abi_test_clobber_f20:
.localentry	abi_test_clobber_f20,0

	li	0, 0

	std	0, -8(1)
	lfd	20, -8(1)
	blr	
.size	abi_test_clobber_f20,.-abi_test_clobber_f20
.globl	abi_test_clobber_f21
.type	abi_test_clobber_f21,@function
.align	4
abi_test_clobber_f21:
.localentry	abi_test_clobber_f21,0

	li	0, 0

	std	0, -8(1)
	lfd	21, -8(1)
	blr	
.size	abi_test_clobber_f21,.-abi_test_clobber_f21
.globl	abi_test_clobber_f22
.type	abi_test_clobber_f22,@function
.align	4
abi_test_clobber_f22:
.localentry	abi_test_clobber_f22,0

	li	0, 0

	std	0, -8(1)
	lfd	22, -8(1)
	blr	
.size	abi_test_clobber_f22,.-abi_test_clobber_f22
.globl	abi_test_clobber_f23
.type	abi_test_clobber_f23,@function
.align	4
abi_test_clobber_f23:
.localentry	abi_test_clobber_f23,0

	li	0, 0

	std	0, -8(1)
	lfd	23, -8(1)
	blr	
.size	abi_test_clobber_f23,.-abi_test_clobber_f23
.globl	abi_test_clobber_f24
.type	abi_test_clobber_f24,@function
.align	4
abi_test_clobber_f24:
.localentry	abi_test_clobber_f24,0

	li	0, 0

	std	0, -8(1)
	lfd	24, -8(1)
	blr	
.size	abi_test_clobber_f24,.-abi_test_clobber_f24
.globl	abi_test_clobber_f25
.type	abi_test_clobber_f25,@function
.align	4
abi_test_clobber_f25:
.localentry	abi_test_clobber_f25,0

	li	0, 0

	std	0, -8(1)
	lfd	25, -8(1)
	blr	
.size	abi_test_clobber_f25,.-abi_test_clobber_f25
.globl	abi_test_clobber_f26
.type	abi_test_clobber_f26,@function
.align	4
abi_test_clobber_f26:
.localentry	abi_test_clobber_f26,0

	li	0, 0

	std	0, -8(1)
	lfd	26, -8(1)
	blr	
.size	abi_test_clobber_f26,.-abi_test_clobber_f26
.globl	abi_test_clobber_f27
.type	abi_test_clobber_f27,@function
.align	4
abi_test_clobber_f27:
.localentry	abi_test_clobber_f27,0

	li	0, 0

	std	0, -8(1)
	lfd	27, -8(1)
	blr	
.size	abi_test_clobber_f27,.-abi_test_clobber_f27
.globl	abi_test_clobber_f28
.type	abi_test_clobber_f28,@function
.align	4
abi_test_clobber_f28:
.localentry	abi_test_clobber_f28,0

	li	0, 0

	std	0, -8(1)
	lfd	28, -8(1)
	blr	
.size	abi_test_clobber_f28,.-abi_test_clobber_f28
.globl	abi_test_clobber_f29
.type	abi_test_clobber_f29,@function
.align	4
abi_test_clobber_f29:
.localentry	abi_test_clobber_f29,0

	li	0, 0

	std	0, -8(1)
	lfd	29, -8(1)
	blr	
.size	abi_test_clobber_f29,.-abi_test_clobber_f29
.globl	abi_test_clobber_f30
.type	abi_test_clobber_f30,@function
.align	4
abi_test_clobber_f30:
.localentry	abi_test_clobber_f30,0

	li	0, 0

	std	0, -8(1)
	lfd	30, -8(1)
	blr	
.size	abi_test_clobber_f30,.-abi_test_clobber_f30
.globl	abi_test_clobber_f31
.type	abi_test_clobber_f31,@function
.align	4
abi_test_clobber_f31:
.localentry	abi_test_clobber_f31,0

	li	0, 0

	std	0, -8(1)
	lfd	31, -8(1)
	blr	
.size	abi_test_clobber_f31,.-abi_test_clobber_f31
.globl	abi_test_clobber_v0
.type	abi_test_clobber_v0,@function
.align	4
abi_test_clobber_v0:
.localentry	abi_test_clobber_v0,0

	vxor	0, 0, 0
	blr	
.size	abi_test_clobber_v0,.-abi_test_clobber_v0
.globl	abi_test_clobber_v1
.type	abi_test_clobber_v1,@function
.align	4
abi_test_clobber_v1:
.localentry	abi_test_clobber_v1,0

	vxor	1, 1, 1
	blr	
.size	abi_test_clobber_v1,.-abi_test_clobber_v1
.globl	abi_test_clobber_v2
.type	abi_test_clobber_v2,@function
.align	4
abi_test_clobber_v2:
.localentry	abi_test_clobber_v2,0

	vxor	2, 2, 2
	blr	
.size	abi_test_clobber_v2,.-abi_test_clobber_v2
.globl	abi_test_clobber_v3
.type	abi_test_clobber_v3,@function
.align	4
abi_test_clobber_v3:
.localentry	abi_test_clobber_v3,0

	vxor	3, 3, 3
	blr	
.size	abi_test_clobber_v3,.-abi_test_clobber_v3
.globl	abi_test_clobber_v4
.type	abi_test_clobber_v4,@function
.align	4
abi_test_clobber_v4:
.localentry	abi_test_clobber_v4,0

	vxor	4, 4, 4
	blr	
.size	abi_test_clobber_v4,.-abi_test_clobber_v4
.globl	abi_test_clobber_v5
.type	abi_test_clobber_v5,@function
.align	4
abi_test_clobber_v5:
.localentry	abi_test_clobber_v5,0

	vxor	5, 5, 5
	blr	
.size	abi_test_clobber_v5,.-abi_test_clobber_v5
.globl	abi_test_clobber_v6
.type	abi_test_clobber_v6,@function
.align	4
abi_test_clobber_v6:
.localentry	abi_test_clobber_v6,0

	vxor	6, 6, 6
	blr	
.size	abi_test_clobber_v6,.-abi_test_clobber_v6
.globl	abi_test_clobber_v7
.type	abi_test_clobber_v7,@function
.align	4
abi_test_clobber_v7:
.localentry	abi_test_clobber_v7,0

	vxor	7, 7, 7
	blr	
.size	abi_test_clobber_v7,.-abi_test_clobber_v7
.globl	abi_test_clobber_v8
.type	abi_test_clobber_v8,@function
.align	4
abi_test_clobber_v8:
.localentry	abi_test_clobber_v8,0

	vxor	8, 8, 8
	blr	
.size	abi_test_clobber_v8,.-abi_test_clobber_v8
.globl	abi_test_clobber_v9
.type	abi_test_clobber_v9,@function
.align	4
abi_test_clobber_v9:
.localentry	abi_test_clobber_v9,0

	vxor	9, 9, 9
	blr	
.size	abi_test_clobber_v9,.-abi_test_clobber_v9
.globl	abi_test_clobber_v10
.type	abi_test_clobber_v10,@function
.align	4
abi_test_clobber_v10:
.localentry	abi_test_clobber_v10,0

	vxor	10, 10, 10
	blr	
.size	abi_test_clobber_v10,.-abi_test_clobber_v10
.globl	abi_test_clobber_v11
.type	abi_test_clobber_v11,@function
.align	4
abi_test_clobber_v11:
.localentry	abi_test_clobber_v11,0

	vxor	11, 11, 11
	blr	
.size	abi_test_clobber_v11,.-abi_test_clobber_v11
.globl	abi_test_clobber_v12
.type	abi_test_clobber_v12,@function
.align	4
abi_test_clobber_v12:
.localentry	abi_test_clobber_v12,0

	vxor	12, 12, 12
	blr	
.size	abi_test_clobber_v12,.-abi_test_clobber_v12
.globl	abi_test_clobber_v13
.type	abi_test_clobber_v13,@function
.align	4
abi_test_clobber_v13:
.localentry	abi_test_clobber_v13,0

	vxor	13, 13, 13
	blr	
.size	abi_test_clobber_v13,.-abi_test_clobber_v13
.globl	abi_test_clobber_v14
.type	abi_test_clobber_v14,@function
.align	4
abi_test_clobber_v14:
.localentry	abi_test_clobber_v14,0

	vxor	14, 14, 14
	blr	
.size	abi_test_clobber_v14,.-abi_test_clobber_v14
.globl	abi_test_clobber_v15
.type	abi_test_clobber_v15,@function
.align	4
abi_test_clobber_v15:
.localentry	abi_test_clobber_v15,0

	vxor	15, 15, 15
	blr	
.size	abi_test_clobber_v15,.-abi_test_clobber_v15
.globl	abi_test_clobber_v16
.type	abi_test_clobber_v16,@function
.align	4
abi_test_clobber_v16:
.localentry	abi_test_clobber_v16,0

	vxor	16, 16, 16
	blr	
.size	abi_test_clobber_v16,.-abi_test_clobber_v16
.globl	abi_test_clobber_v17
.type	abi_test_clobber_v17,@function
.align	4
abi_test_clobber_v17:
.localentry	abi_test_clobber_v17,0

	vxor	17, 17, 17
	blr	
.size	abi_test_clobber_v17,.-abi_test_clobber_v17
.globl	abi_test_clobber_v18
.type	abi_test_clobber_v18,@function
.align	4
abi_test_clobber_v18:
.localentry	abi_test_clobber_v18,0

	vxor	18, 18, 18
	blr	
.size	abi_test_clobber_v18,.-abi_test_clobber_v18
.globl	abi_test_clobber_v19
.type	abi_test_clobber_v19,@function
.align	4
abi_test_clobber_v19:
.localentry	abi_test_clobber_v19,0

	vxor	19, 19, 19
	blr	
.size	abi_test_clobber_v19,.-abi_test_clobber_v19
.globl	abi_test_clobber_v20
.type	abi_test_clobber_v20,@function
.align	4
abi_test_clobber_v20:
.localentry	abi_test_clobber_v20,0

	vxor	20, 20, 20
	blr	
.size	abi_test_clobber_v20,.-abi_test_clobber_v20
.globl	abi_test_clobber_v21
.type	abi_test_clobber_v21,@function
.align	4
abi_test_clobber_v21:
.localentry	abi_test_clobber_v21,0

	vxor	21, 21, 21
	blr	
.size	abi_test_clobber_v21,.-abi_test_clobber_v21
.globl	abi_test_clobber_v22
.type	abi_test_clobber_v22,@function
.align	4
abi_test_clobber_v22:
.localentry	abi_test_clobber_v22,0

	vxor	22, 22, 22
	blr	
.size	abi_test_clobber_v22,.-abi_test_clobber_v22
.globl	abi_test_clobber_v23
.type	abi_test_clobber_v23,@function
.align	4
abi_test_clobber_v23:
.localentry	abi_test_clobber_v23,0

	vxor	23, 23, 23
	blr	
.size	abi_test_clobber_v23,.-abi_test_clobber_v23
.globl	abi_test_clobber_v24
.type	abi_test_clobber_v24,@function
.align	4
abi_test_clobber_v24:
.localentry	abi_test_clobber_v24,0

	vxor	24, 24, 24
	blr	
.size	abi_test_clobber_v24,.-abi_test_clobber_v24
.globl	abi_test_clobber_v25
.type	abi_test_clobber_v25,@function
.align	4
abi_test_clobber_v25:
.localentry	abi_test_clobber_v25,0

	vxor	25, 25, 25
	blr	
.size	abi_test_clobber_v25,.-abi_test_clobber_v25
.globl	abi_test_clobber_v26
.type	abi_test_clobber_v26,@function
.align	4
abi_test_clobber_v26:
.localentry	abi_test_clobber_v26,0

	vxor	26, 26, 26
	blr	
.size	abi_test_clobber_v26,.-abi_test_clobber_v26
.globl	abi_test_clobber_v27
.type	abi_test_clobber_v27,@function
.align	4
abi_test_clobber_v27:
.localentry	abi_test_clobber_v27,0

	vxor	27, 27, 27
	blr	
.size	abi_test_clobber_v27,.-abi_test_clobber_v27
.globl	abi_test_clobber_v28
.type	abi_test_clobber_v28,@function
.align	4
abi_test_clobber_v28:
.localentry	abi_test_clobber_v28,0

	vxor	28, 28, 28
	blr	
.size	abi_test_clobber_v28,.-abi_test_clobber_v28
.globl	abi_test_clobber_v29
.type	abi_test_clobber_v29,@function
.align	4
abi_test_clobber_v29:
.localentry	abi_test_clobber_v29,0

	vxor	29, 29, 29
	blr	
.size	abi_test_clobber_v29,.-abi_test_clobber_v29
.globl	abi_test_clobber_v30
.type	abi_test_clobber_v30,@function
.align	4
abi_test_clobber_v30:
.localentry	abi_test_clobber_v30,0

	vxor	30, 30, 30
	blr	
.size	abi_test_clobber_v30,.-abi_test_clobber_v30
.globl	abi_test_clobber_v31
.type	abi_test_clobber_v31,@function
.align	4
abi_test_clobber_v31:
.localentry	abi_test_clobber_v31,0

	vxor	31, 31, 31
	blr	
.size	abi_test_clobber_v31,.-abi_test_clobber_v31
.globl	abi_test_clobber_cr0
.type	abi_test_clobber_cr0,@function
.align	4
abi_test_clobber_cr0:
.localentry	abi_test_clobber_cr0,0



	mfcr	0
	not	0, 0
	mtcrf	128, 0
	blr	
.size	abi_test_clobber_cr0,.-abi_test_clobber_cr0
.globl	abi_test_clobber_cr1
.type	abi_test_clobber_cr1,@function
.align	4
abi_test_clobber_cr1:
.localentry	abi_test_clobber_cr1,0



	mfcr	0
	not	0, 0
	mtcrf	64, 0
	blr	
.size	abi_test_clobber_cr1,.-abi_test_clobber_cr1
.globl	abi_test_clobber_cr2
.type	abi_test_clobber_cr2,@function
.align	4
abi_test_clobber_cr2:
.localentry	abi_test_clobber_cr2,0



	mfcr	0
	not	0, 0
	mtcrf	32, 0
	blr	
.size	abi_test_clobber_cr2,.-abi_test_clobber_cr2
.globl	abi_test_clobber_cr3
.type	abi_test_clobber_cr3,@function
.align	4
abi_test_clobber_cr3:
.localentry	abi_test_clobber_cr3,0



	mfcr	0
	not	0, 0
	mtcrf	16, 0
	blr	
.size	abi_test_clobber_cr3,.-abi_test_clobber_cr3
.globl	abi_test_clobber_cr4
.type	abi_test_clobber_cr4,@function
.align	4
abi_test_clobber_cr4:
.localentry	abi_test_clobber_cr4,0



	mfcr	0
	not	0, 0
	mtcrf	8, 0
	blr	
.size	abi_test_clobber_cr4,.-abi_test_clobber_cr4
.globl	abi_test_clobber_cr5
.type	abi_test_clobber_cr5,@function
.align	4
abi_test_clobber_cr5:
.localentry	abi_test_clobber_cr5,0



	mfcr	0
	not	0, 0
	mtcrf	4, 0
	blr	
.size	abi_test_clobber_cr5,.-abi_test_clobber_cr5
.globl	abi_test_clobber_cr6
.type	abi_test_clobber_cr6,@function
.align	4
abi_test_clobber_cr6:
.localentry	abi_test_clobber_cr6,0



	mfcr	0
	not	0, 0
	mtcrf	2, 0
	blr	
.size	abi_test_clobber_cr6,.-abi_test_clobber_cr6
.globl	abi_test_clobber_cr7
.type	abi_test_clobber_cr7,@function
.align	4
abi_test_clobber_cr7:
.localentry	abi_test_clobber_cr7,0



	mfcr	0
	not	0, 0
	mtcrf	1, 0
	blr	
.size	abi_test_clobber_cr7,.-abi_test_clobber_cr7
.globl	abi_test_clobber_ctr
.type	abi_test_clobber_ctr,@function
.align	4
abi_test_clobber_ctr:
.localentry	abi_test_clobber_ctr,0

	li	0, 0
	mtctr	0
	blr	
.size	abi_test_clobber_ctr,.-abi_test_clobber_ctr

.globl	abi_test_clobber_lr
.type	abi_test_clobber_lr,@function
.align	4
abi_test_clobber_lr:
.localentry	abi_test_clobber_lr,0

	mflr	0
	mtctr	0
	li	0, 0
	mtlr	0
	bctr	
.size	abi_test_clobber_lr,.-abi_test_clobber_lr

#endif  // !OPENSSL_NO_ASM && __powerpc64__ && __ELF__
#if defined(__ELF__)
// See https://www.airs.com/blog/archives/518.
.section .note.GNU-stack,"",%progbits
#endif
