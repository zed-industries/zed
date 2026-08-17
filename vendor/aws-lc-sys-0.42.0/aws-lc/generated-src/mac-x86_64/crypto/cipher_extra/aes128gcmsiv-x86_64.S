// This file is generated from a similarly-named Perl script in the BoringSSL
// source tree. Do not edit by hand.

#include <openssl/asm_base.h>

#if !defined(OPENSSL_NO_ASM) && defined(OPENSSL_X86_64) && defined(__APPLE__)
.section	__DATA,__const

.p2align	4
one:
.quad	1,0
two:
.quad	2,0
three:
.quad	3,0
four:
.quad	4,0
five:
.quad	5,0
six:
.quad	6,0
seven:
.quad	7,0
eight:
.quad	8,0

OR_MASK:
.long	0x00000000,0x00000000,0x00000000,0x80000000
poly:
.quad	0x1, 0xc200000000000000
mask:
.long	0x0c0f0e0d,0x0c0f0e0d,0x0c0f0e0d,0x0c0f0e0d
con1:
.long	1,1,1,1
con2:
.long	0x1b,0x1b,0x1b,0x1b
con3:
.byte	-1,-1,-1,-1,-1,-1,-1,-1,4,5,6,7,4,5,6,7
and_mask:
.long	0,0xffffffff, 0xffffffff, 0xffffffff
.text	

.p2align	4
GFMUL:

	vpclmulqdq	$0x00,%xmm1,%xmm0,%xmm2
	vpclmulqdq	$0x11,%xmm1,%xmm0,%xmm5
	vpclmulqdq	$0x10,%xmm1,%xmm0,%xmm3
	vpclmulqdq	$0x01,%xmm1,%xmm0,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpslldq	$8,%xmm3,%xmm4
	vpsrldq	$8,%xmm3,%xmm3
	vpxor	%xmm4,%xmm2,%xmm2
	vpxor	%xmm3,%xmm5,%xmm5

	vpclmulqdq	$0x10,poly(%rip),%xmm2,%xmm3
	vpshufd	$78,%xmm2,%xmm4
	vpxor	%xmm4,%xmm3,%xmm2

	vpclmulqdq	$0x10,poly(%rip),%xmm2,%xmm3
	vpshufd	$78,%xmm2,%xmm4
	vpxor	%xmm4,%xmm3,%xmm2

	vpxor	%xmm5,%xmm2,%xmm0
	.byte	0xf3,0xc3


.globl	_aesgcmsiv_htable_init
.private_extern _aesgcmsiv_htable_init

.p2align	4
_aesgcmsiv_htable_init:

_CET_ENDBR
	vmovdqa	(%rsi),%xmm0
	vmovdqa	%xmm0,%xmm1
	vmovdqa	%xmm0,(%rdi)
	call	GFMUL
	vmovdqa	%xmm0,16(%rdi)
	call	GFMUL
	vmovdqa	%xmm0,32(%rdi)
	call	GFMUL
	vmovdqa	%xmm0,48(%rdi)
	call	GFMUL
	vmovdqa	%xmm0,64(%rdi)
	call	GFMUL
	vmovdqa	%xmm0,80(%rdi)
	call	GFMUL
	vmovdqa	%xmm0,96(%rdi)
	call	GFMUL
	vmovdqa	%xmm0,112(%rdi)
	.byte	0xf3,0xc3


.globl	_aesgcmsiv_htable6_init
.private_extern _aesgcmsiv_htable6_init

.p2align	4
_aesgcmsiv_htable6_init:

_CET_ENDBR
	vmovdqa	(%rsi),%xmm0
	vmovdqa	%xmm0,%xmm1
	vmovdqa	%xmm0,(%rdi)
	call	GFMUL
	vmovdqa	%xmm0,16(%rdi)
	call	GFMUL
	vmovdqa	%xmm0,32(%rdi)
	call	GFMUL
	vmovdqa	%xmm0,48(%rdi)
	call	GFMUL
	vmovdqa	%xmm0,64(%rdi)
	call	GFMUL
	vmovdqa	%xmm0,80(%rdi)
	.byte	0xf3,0xc3


.globl	_aesgcmsiv_htable_polyval
.private_extern _aesgcmsiv_htable_polyval

.p2align	4
_aesgcmsiv_htable_polyval:

_CET_ENDBR
	testq	%rdx,%rdx
	jnz	L$htable_polyval_start
	.byte	0xf3,0xc3

L$htable_polyval_start:
	vzeroall



	movq	%rdx,%r11
	andq	$127,%r11

	jz	L$htable_polyval_no_prefix

	vpxor	%xmm9,%xmm9,%xmm9
	vmovdqa	(%rcx),%xmm1
	subq	%r11,%rdx

	subq	$16,%r11


	vmovdqu	(%rsi),%xmm0
	vpxor	%xmm1,%xmm0,%xmm0

	vpclmulqdq	$0x01,(%rdi,%r11,1),%xmm0,%xmm5
	vpclmulqdq	$0x00,(%rdi,%r11,1),%xmm0,%xmm3
	vpclmulqdq	$0x11,(%rdi,%r11,1),%xmm0,%xmm4
	vpclmulqdq	$0x10,(%rdi,%r11,1),%xmm0,%xmm6
	vpxor	%xmm6,%xmm5,%xmm5

	leaq	16(%rsi),%rsi
	testq	%r11,%r11
	jnz	L$htable_polyval_prefix_loop
	jmp	L$htable_polyval_prefix_complete


.p2align	6
L$htable_polyval_prefix_loop:
	subq	$16,%r11

	vmovdqu	(%rsi),%xmm0

	vpclmulqdq	$0x00,(%rdi,%r11,1),%xmm0,%xmm6
	vpxor	%xmm6,%xmm3,%xmm3
	vpclmulqdq	$0x11,(%rdi,%r11,1),%xmm0,%xmm6
	vpxor	%xmm6,%xmm4,%xmm4
	vpclmulqdq	$0x01,(%rdi,%r11,1),%xmm0,%xmm6
	vpxor	%xmm6,%xmm5,%xmm5
	vpclmulqdq	$0x10,(%rdi,%r11,1),%xmm0,%xmm6
	vpxor	%xmm6,%xmm5,%xmm5

	testq	%r11,%r11

	leaq	16(%rsi),%rsi

	jnz	L$htable_polyval_prefix_loop

L$htable_polyval_prefix_complete:
	vpsrldq	$8,%xmm5,%xmm6
	vpslldq	$8,%xmm5,%xmm5

	vpxor	%xmm6,%xmm4,%xmm9
	vpxor	%xmm5,%xmm3,%xmm1

	jmp	L$htable_polyval_main_loop

L$htable_polyval_no_prefix:




	vpxor	%xmm1,%xmm1,%xmm1
	vmovdqa	(%rcx),%xmm9

.p2align	6
L$htable_polyval_main_loop:
	subq	$0x80,%rdx
	jb	L$htable_polyval_out

	vmovdqu	112(%rsi),%xmm0

	vpclmulqdq	$0x01,(%rdi),%xmm0,%xmm5
	vpclmulqdq	$0x00,(%rdi),%xmm0,%xmm3
	vpclmulqdq	$0x11,(%rdi),%xmm0,%xmm4
	vpclmulqdq	$0x10,(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm5,%xmm5


	vmovdqu	96(%rsi),%xmm0
	vpclmulqdq	$0x01,16(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm5,%xmm5
	vpclmulqdq	$0x00,16(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm3,%xmm3
	vpclmulqdq	$0x11,16(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm4,%xmm4
	vpclmulqdq	$0x10,16(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm5,%xmm5



	vmovdqu	80(%rsi),%xmm0

	vpclmulqdq	$0x10,poly(%rip),%xmm1,%xmm7
	vpalignr	$8,%xmm1,%xmm1,%xmm1

	vpclmulqdq	$0x01,32(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm5,%xmm5
	vpclmulqdq	$0x00,32(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm3,%xmm3
	vpclmulqdq	$0x11,32(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm4,%xmm4
	vpclmulqdq	$0x10,32(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm5,%xmm5


	vpxor	%xmm7,%xmm1,%xmm1

	vmovdqu	64(%rsi),%xmm0

	vpclmulqdq	$0x01,48(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm5,%xmm5
	vpclmulqdq	$0x00,48(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm3,%xmm3
	vpclmulqdq	$0x11,48(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm4,%xmm4
	vpclmulqdq	$0x10,48(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm5,%xmm5


	vmovdqu	48(%rsi),%xmm0

	vpclmulqdq	$0x10,poly(%rip),%xmm1,%xmm7
	vpalignr	$8,%xmm1,%xmm1,%xmm1

	vpclmulqdq	$0x01,64(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm5,%xmm5
	vpclmulqdq	$0x00,64(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm3,%xmm3
	vpclmulqdq	$0x11,64(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm4,%xmm4
	vpclmulqdq	$0x10,64(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm5,%xmm5


	vpxor	%xmm7,%xmm1,%xmm1

	vmovdqu	32(%rsi),%xmm0

	vpclmulqdq	$0x01,80(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm5,%xmm5
	vpclmulqdq	$0x00,80(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm3,%xmm3
	vpclmulqdq	$0x11,80(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm4,%xmm4
	vpclmulqdq	$0x10,80(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm5,%xmm5


	vpxor	%xmm9,%xmm1,%xmm1

	vmovdqu	16(%rsi),%xmm0

	vpclmulqdq	$0x01,96(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm5,%xmm5
	vpclmulqdq	$0x00,96(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm3,%xmm3
	vpclmulqdq	$0x11,96(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm4,%xmm4
	vpclmulqdq	$0x10,96(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm5,%xmm5


	vmovdqu	0(%rsi),%xmm0
	vpxor	%xmm1,%xmm0,%xmm0

	vpclmulqdq	$0x01,112(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm5,%xmm5
	vpclmulqdq	$0x00,112(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm3,%xmm3
	vpclmulqdq	$0x11,112(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm4,%xmm4
	vpclmulqdq	$0x10,112(%rdi),%xmm0,%xmm6
	vpxor	%xmm6,%xmm5,%xmm5


	vpsrldq	$8,%xmm5,%xmm6
	vpslldq	$8,%xmm5,%xmm5

	vpxor	%xmm6,%xmm4,%xmm9
	vpxor	%xmm5,%xmm3,%xmm1

	leaq	128(%rsi),%rsi
	jmp	L$htable_polyval_main_loop



L$htable_polyval_out:
	vpclmulqdq	$0x10,poly(%rip),%xmm1,%xmm6
	vpalignr	$8,%xmm1,%xmm1,%xmm1
	vpxor	%xmm6,%xmm1,%xmm1

	vpclmulqdq	$0x10,poly(%rip),%xmm1,%xmm6
	vpalignr	$8,%xmm1,%xmm1,%xmm1
	vpxor	%xmm6,%xmm1,%xmm1
	vpxor	%xmm9,%xmm1,%xmm1

	vmovdqu	%xmm1,(%rcx)
	vzeroupper
	.byte	0xf3,0xc3


.globl	_aesgcmsiv_polyval_horner
.private_extern _aesgcmsiv_polyval_horner

.p2align	4
_aesgcmsiv_polyval_horner:

_CET_ENDBR
	testq	%rcx,%rcx
	jnz	L$polyval_horner_start
	.byte	0xf3,0xc3

L$polyval_horner_start:



	xorq	%r10,%r10
	shlq	$4,%rcx

	vmovdqa	(%rsi),%xmm1
	vmovdqa	(%rdi),%xmm0

L$polyval_horner_loop:
	vpxor	(%rdx,%r10,1),%xmm0,%xmm0
	call	GFMUL

	addq	$16,%r10
	cmpq	%r10,%rcx
	jne	L$polyval_horner_loop


	vmovdqa	%xmm0,(%rdi)
	.byte	0xf3,0xc3


.globl	_aes128gcmsiv_aes_ks
.private_extern _aes128gcmsiv_aes_ks

.p2align	4
_aes128gcmsiv_aes_ks:

_CET_ENDBR
	vmovdqu	(%rdi),%xmm1
	vmovdqa	%xmm1,(%rsi)

	vmovdqa	con1(%rip),%xmm0
	vmovdqa	mask(%rip),%xmm15

	movq	$8,%rax

L$ks128_loop:
	addq	$16,%rsi
	subq	$1,%rax
	vpshufb	%xmm15,%xmm1,%xmm2
	vaesenclast	%xmm0,%xmm2,%xmm2
	vpslld	$1,%xmm0,%xmm0
	vpslldq	$4,%xmm1,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpslldq	$4,%xmm3,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpslldq	$4,%xmm3,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpxor	%xmm2,%xmm1,%xmm1
	vmovdqa	%xmm1,(%rsi)
	jne	L$ks128_loop

	vmovdqa	con2(%rip),%xmm0
	vpshufb	%xmm15,%xmm1,%xmm2
	vaesenclast	%xmm0,%xmm2,%xmm2
	vpslld	$1,%xmm0,%xmm0
	vpslldq	$4,%xmm1,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpslldq	$4,%xmm3,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpslldq	$4,%xmm3,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpxor	%xmm2,%xmm1,%xmm1
	vmovdqa	%xmm1,16(%rsi)

	vpshufb	%xmm15,%xmm1,%xmm2
	vaesenclast	%xmm0,%xmm2,%xmm2
	vpslldq	$4,%xmm1,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpslldq	$4,%xmm3,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpslldq	$4,%xmm3,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpxor	%xmm2,%xmm1,%xmm1
	vmovdqa	%xmm1,32(%rsi)
	.byte	0xf3,0xc3


.globl	_aes256gcmsiv_aes_ks
.private_extern _aes256gcmsiv_aes_ks

.p2align	4
_aes256gcmsiv_aes_ks:

_CET_ENDBR
	vmovdqu	(%rdi),%xmm1
	vmovdqu	16(%rdi),%xmm3
	vmovdqa	%xmm1,(%rsi)
	vmovdqa	%xmm3,16(%rsi)
	vmovdqa	con1(%rip),%xmm0
	vmovdqa	mask(%rip),%xmm15
	vpxor	%xmm14,%xmm14,%xmm14
	movq	$6,%rax

L$ks256_loop:
	addq	$32,%rsi
	subq	$1,%rax
	vpshufb	%xmm15,%xmm3,%xmm2
	vaesenclast	%xmm0,%xmm2,%xmm2
	vpslld	$1,%xmm0,%xmm0
	vpsllq	$32,%xmm1,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpshufb	con3(%rip),%xmm1,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpxor	%xmm2,%xmm1,%xmm1
	vmovdqa	%xmm1,(%rsi)
	vpshufd	$0xff,%xmm1,%xmm2
	vaesenclast	%xmm14,%xmm2,%xmm2
	vpsllq	$32,%xmm3,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpshufb	con3(%rip),%xmm3,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpxor	%xmm2,%xmm3,%xmm3
	vmovdqa	%xmm3,16(%rsi)
	jne	L$ks256_loop

	vpshufb	%xmm15,%xmm3,%xmm2
	vaesenclast	%xmm0,%xmm2,%xmm2
	vpsllq	$32,%xmm1,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpshufb	con3(%rip),%xmm1,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpxor	%xmm2,%xmm1,%xmm1
	vmovdqa	%xmm1,32(%rsi)
	.byte	0xf3,0xc3

.globl	_aes128gcmsiv_aes_ks_enc_x1
.private_extern _aes128gcmsiv_aes_ks_enc_x1

.p2align	4
_aes128gcmsiv_aes_ks_enc_x1:

_CET_ENDBR
	vmovdqa	(%rcx),%xmm1
	vmovdqa	0(%rdi),%xmm4

	vmovdqa	%xmm1,(%rdx)
	vpxor	%xmm1,%xmm4,%xmm4

	vmovdqa	con1(%rip),%xmm0
	vmovdqa	mask(%rip),%xmm15

	vpshufb	%xmm15,%xmm1,%xmm2
	vaesenclast	%xmm0,%xmm2,%xmm2
	vpslld	$1,%xmm0,%xmm0
	vpsllq	$32,%xmm1,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpshufb	con3(%rip),%xmm1,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpxor	%xmm2,%xmm1,%xmm1

	vaesenc	%xmm1,%xmm4,%xmm4
	vmovdqa	%xmm1,16(%rdx)

	vpshufb	%xmm15,%xmm1,%xmm2
	vaesenclast	%xmm0,%xmm2,%xmm2
	vpslld	$1,%xmm0,%xmm0
	vpsllq	$32,%xmm1,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpshufb	con3(%rip),%xmm1,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpxor	%xmm2,%xmm1,%xmm1

	vaesenc	%xmm1,%xmm4,%xmm4
	vmovdqa	%xmm1,32(%rdx)

	vpshufb	%xmm15,%xmm1,%xmm2
	vaesenclast	%xmm0,%xmm2,%xmm2
	vpslld	$1,%xmm0,%xmm0
	vpsllq	$32,%xmm1,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpshufb	con3(%rip),%xmm1,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpxor	%xmm2,%xmm1,%xmm1

	vaesenc	%xmm1,%xmm4,%xmm4
	vmovdqa	%xmm1,48(%rdx)

	vpshufb	%xmm15,%xmm1,%xmm2
	vaesenclast	%xmm0,%xmm2,%xmm2
	vpslld	$1,%xmm0,%xmm0
	vpsllq	$32,%xmm1,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpshufb	con3(%rip),%xmm1,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpxor	%xmm2,%xmm1,%xmm1

	vaesenc	%xmm1,%xmm4,%xmm4
	vmovdqa	%xmm1,64(%rdx)

	vpshufb	%xmm15,%xmm1,%xmm2
	vaesenclast	%xmm0,%xmm2,%xmm2
	vpslld	$1,%xmm0,%xmm0
	vpsllq	$32,%xmm1,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpshufb	con3(%rip),%xmm1,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpxor	%xmm2,%xmm1,%xmm1

	vaesenc	%xmm1,%xmm4,%xmm4
	vmovdqa	%xmm1,80(%rdx)

	vpshufb	%xmm15,%xmm1,%xmm2
	vaesenclast	%xmm0,%xmm2,%xmm2
	vpslld	$1,%xmm0,%xmm0
	vpsllq	$32,%xmm1,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpshufb	con3(%rip),%xmm1,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpxor	%xmm2,%xmm1,%xmm1

	vaesenc	%xmm1,%xmm4,%xmm4
	vmovdqa	%xmm1,96(%rdx)

	vpshufb	%xmm15,%xmm1,%xmm2
	vaesenclast	%xmm0,%xmm2,%xmm2
	vpslld	$1,%xmm0,%xmm0
	vpsllq	$32,%xmm1,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpshufb	con3(%rip),%xmm1,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpxor	%xmm2,%xmm1,%xmm1

	vaesenc	%xmm1,%xmm4,%xmm4
	vmovdqa	%xmm1,112(%rdx)

	vpshufb	%xmm15,%xmm1,%xmm2
	vaesenclast	%xmm0,%xmm2,%xmm2
	vpslld	$1,%xmm0,%xmm0
	vpsllq	$32,%xmm1,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpshufb	con3(%rip),%xmm1,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpxor	%xmm2,%xmm1,%xmm1

	vaesenc	%xmm1,%xmm4,%xmm4
	vmovdqa	%xmm1,128(%rdx)


	vmovdqa	con2(%rip),%xmm0

	vpshufb	%xmm15,%xmm1,%xmm2
	vaesenclast	%xmm0,%xmm2,%xmm2
	vpslld	$1,%xmm0,%xmm0
	vpsllq	$32,%xmm1,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpshufb	con3(%rip),%xmm1,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpxor	%xmm2,%xmm1,%xmm1

	vaesenc	%xmm1,%xmm4,%xmm4
	vmovdqa	%xmm1,144(%rdx)

	vpshufb	%xmm15,%xmm1,%xmm2
	vaesenclast	%xmm0,%xmm2,%xmm2
	vpsllq	$32,%xmm1,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpshufb	con3(%rip),%xmm1,%xmm3
	vpxor	%xmm3,%xmm1,%xmm1
	vpxor	%xmm2,%xmm1,%xmm1

	vaesenclast	%xmm1,%xmm4,%xmm4
	vmovdqa	%xmm1,160(%rdx)


	vmovdqa	%xmm4,0(%rsi)
	.byte	0xf3,0xc3


.globl	_aes128gcmsiv_kdf
.private_extern _aes128gcmsiv_kdf

.p2align	4
_aes128gcmsiv_kdf:

_CET_ENDBR




	vmovdqa	(%rdx),%xmm1
	vmovdqa	0(%rdi),%xmm9
	vmovdqa	and_mask(%rip),%xmm12
	vmovdqa	one(%rip),%xmm13
	vpshufd	$0x90,%xmm9,%xmm9
	vpand	%xmm12,%xmm9,%xmm9
	vpaddd	%xmm13,%xmm9,%xmm10
	vpaddd	%xmm13,%xmm10,%xmm11
	vpaddd	%xmm13,%xmm11,%xmm12

	vpxor	%xmm1,%xmm9,%xmm9
	vpxor	%xmm1,%xmm10,%xmm10
	vpxor	%xmm1,%xmm11,%xmm11
	vpxor	%xmm1,%xmm12,%xmm12

	vmovdqa	16(%rdx),%xmm1
	vaesenc	%xmm1,%xmm9,%xmm9
	vaesenc	%xmm1,%xmm10,%xmm10
	vaesenc	%xmm1,%xmm11,%xmm11
	vaesenc	%xmm1,%xmm12,%xmm12

	vmovdqa	32(%rdx),%xmm2
	vaesenc	%xmm2,%xmm9,%xmm9
	vaesenc	%xmm2,%xmm10,%xmm10
	vaesenc	%xmm2,%xmm11,%xmm11
	vaesenc	%xmm2,%xmm12,%xmm12

	vmovdqa	48(%rdx),%xmm1
	vaesenc	%xmm1,%xmm9,%xmm9
	vaesenc	%xmm1,%xmm10,%xmm10
	vaesenc	%xmm1,%xmm11,%xmm11
	vaesenc	%xmm1,%xmm12,%xmm12

	vmovdqa	64(%rdx),%xmm2
	vaesenc	%xmm2,%xmm9,%xmm9
	vaesenc	%xmm2,%xmm10,%xmm10
	vaesenc	%xmm2,%xmm11,%xmm11
	vaesenc	%xmm2,%xmm12,%xmm12

	vmovdqa	80(%rdx),%xmm1
	vaesenc	%xmm1,%xmm9,%xmm9
	vaesenc	%xmm1,%xmm10,%xmm10
	vaesenc	%xmm1,%xmm11,%xmm11
	vaesenc	%xmm1,%xmm12,%xmm12

	vmovdqa	96(%rdx),%xmm2
	vaesenc	%xmm2,%xmm9,%xmm9
	vaesenc	%xmm2,%xmm10,%xmm10
	vaesenc	%xmm2,%xmm11,%xmm11
	vaesenc	%xmm2,%xmm12,%xmm12

	vmovdqa	112(%rdx),%xmm1
	vaesenc	%xmm1,%xmm9,%xmm9
	vaesenc	%xmm1,%xmm10,%xmm10
	vaesenc	%xmm1,%xmm11,%xmm11
	vaesenc	%xmm1,%xmm12,%xmm12

	vmovdqa	128(%rdx),%xmm2
	vaesenc	%xmm2,%xmm9,%xmm9
	vaesenc	%xmm2,%xmm10,%xmm10
	vaesenc	%xmm2,%xmm11,%xmm11
	vaesenc	%xmm2,%xmm12,%xmm12

	vmovdqa	144(%rdx),%xmm1
	vaesenc	%xmm1,%xmm9,%xmm9
	vaesenc	%xmm1,%xmm10,%xmm10
	vaesenc	%xmm1,%xmm11,%xmm11
	vaesenc	%xmm1,%xmm12,%xmm12

	vmovdqa	160(%rdx),%xmm2
	vaesenclast	%xmm2,%xmm9,%xmm9
	vaesenclast	%xmm2,%xmm10,%xmm10
	vaesenclast	%xmm2,%xmm11,%xmm11
	vaesenclast	%xmm2,%xmm12,%xmm12


	vmovdqa	%xmm9,0(%rsi)
	vmovdqa	%xmm10,16(%rsi)
	vmovdqa	%xmm11,32(%rsi)
	vmovdqa	%xmm12,48(%rsi)
	.byte	0xf3,0xc3


.globl	_aes128gcmsiv_enc_msg_x4
.private_extern _aes128gcmsiv_enc_msg_x4

.p2align	4
_aes128gcmsiv_enc_msg_x4:

_CET_ENDBR
	testq	%r8,%r8
	jnz	L$128_enc_msg_x4_start
	.byte	0xf3,0xc3

L$128_enc_msg_x4_start:
	pushq	%r12

	pushq	%r13


	shrq	$4,%r8
	movq	%r8,%r10
	shlq	$62,%r10
	shrq	$62,%r10


	vmovdqa	(%rdx),%xmm15
	vpor	OR_MASK(%rip),%xmm15,%xmm15

	vmovdqu	four(%rip),%xmm4
	vmovdqa	%xmm15,%xmm0
	vpaddd	one(%rip),%xmm15,%xmm1
	vpaddd	two(%rip),%xmm15,%xmm2
	vpaddd	three(%rip),%xmm15,%xmm3

	shrq	$2,%r8
	je	L$128_enc_msg_x4_check_remainder

	subq	$64,%rsi
	subq	$64,%rdi

L$128_enc_msg_x4_loop1:
	addq	$64,%rsi
	addq	$64,%rdi

	vmovdqa	%xmm0,%xmm5
	vmovdqa	%xmm1,%xmm6
	vmovdqa	%xmm2,%xmm7
	vmovdqa	%xmm3,%xmm8

	vpxor	(%rcx),%xmm5,%xmm5
	vpxor	(%rcx),%xmm6,%xmm6
	vpxor	(%rcx),%xmm7,%xmm7
	vpxor	(%rcx),%xmm8,%xmm8

	vmovdqu	16(%rcx),%xmm12
	vaesenc	%xmm12,%xmm5,%xmm5
	vaesenc	%xmm12,%xmm6,%xmm6
	vaesenc	%xmm12,%xmm7,%xmm7
	vaesenc	%xmm12,%xmm8,%xmm8

	vpaddd	%xmm4,%xmm0,%xmm0
	vmovdqu	32(%rcx),%xmm12
	vaesenc	%xmm12,%xmm5,%xmm5
	vaesenc	%xmm12,%xmm6,%xmm6
	vaesenc	%xmm12,%xmm7,%xmm7
	vaesenc	%xmm12,%xmm8,%xmm8

	vpaddd	%xmm4,%xmm1,%xmm1
	vmovdqu	48(%rcx),%xmm12
	vaesenc	%xmm12,%xmm5,%xmm5
	vaesenc	%xmm12,%xmm6,%xmm6
	vaesenc	%xmm12,%xmm7,%xmm7
	vaesenc	%xmm12,%xmm8,%xmm8

	vpaddd	%xmm4,%xmm2,%xmm2
	vmovdqu	64(%rcx),%xmm12
	vaesenc	%xmm12,%xmm5,%xmm5
	vaesenc	%xmm12,%xmm6,%xmm6
	vaesenc	%xmm12,%xmm7,%xmm7
	vaesenc	%xmm12,%xmm8,%xmm8

	vpaddd	%xmm4,%xmm3,%xmm3

	vmovdqu	80(%rcx),%xmm12
	vaesenc	%xmm12,%xmm5,%xmm5
	vaesenc	%xmm12,%xmm6,%xmm6
	vaesenc	%xmm12,%xmm7,%xmm7
	vaesenc	%xmm12,%xmm8,%xmm8

	vmovdqu	96(%rcx),%xmm12
	vaesenc	%xmm12,%xmm5,%xmm5
	vaesenc	%xmm12,%xmm6,%xmm6
	vaesenc	%xmm12,%xmm7,%xmm7
	vaesenc	%xmm12,%xmm8,%xmm8

	vmovdqu	112(%rcx),%xmm12
	vaesenc	%xmm12,%xmm5,%xmm5
	vaesenc	%xmm12,%xmm6,%xmm6
	vaesenc	%xmm12,%xmm7,%xmm7
	vaesenc	%xmm12,%xmm8,%xmm8

	vmovdqu	128(%rcx),%xmm12
	vaesenc	%xmm12,%xmm5,%xmm5
	vaesenc	%xmm12,%xmm6,%xmm6
	vaesenc	%xmm12,%xmm7,%xmm7
	vaesenc	%xmm12,%xmm8,%xmm8

	vmovdqu	144(%rcx),%xmm12
	vaesenc	%xmm12,%xmm5,%xmm5
	vaesenc	%xmm12,%xmm6,%xmm6
	vaesenc	%xmm12,%xmm7,%xmm7
	vaesenc	%xmm12,%xmm8,%xmm8

	vmovdqu	160(%rcx),%xmm12
	vaesenclast	%xmm12,%xmm5,%xmm5
	vaesenclast	%xmm12,%xmm6,%xmm6
	vaesenclast	%xmm12,%xmm7,%xmm7
	vaesenclast	%xmm12,%xmm8,%xmm8



	vpxor	0(%rdi),%xmm5,%xmm5
	vpxor	16(%rdi),%xmm6,%xmm6
	vpxor	32(%rdi),%xmm7,%xmm7
	vpxor	48(%rdi),%xmm8,%xmm8

	subq	$1,%r8

	vmovdqu	%xmm5,0(%rsi)
	vmovdqu	%xmm6,16(%rsi)
	vmovdqu	%xmm7,32(%rsi)
	vmovdqu	%xmm8,48(%rsi)

	jne	L$128_enc_msg_x4_loop1

	addq	$64,%rsi
	addq	$64,%rdi

L$128_enc_msg_x4_check_remainder:
	cmpq	$0,%r10
	je	L$128_enc_msg_x4_out

L$128_enc_msg_x4_loop2:


	vmovdqa	%xmm0,%xmm5
	vpaddd	one(%rip),%xmm0,%xmm0

	vpxor	(%rcx),%xmm5,%xmm5
	vaesenc	16(%rcx),%xmm5,%xmm5
	vaesenc	32(%rcx),%xmm5,%xmm5
	vaesenc	48(%rcx),%xmm5,%xmm5
	vaesenc	64(%rcx),%xmm5,%xmm5
	vaesenc	80(%rcx),%xmm5,%xmm5
	vaesenc	96(%rcx),%xmm5,%xmm5
	vaesenc	112(%rcx),%xmm5,%xmm5
	vaesenc	128(%rcx),%xmm5,%xmm5
	vaesenc	144(%rcx),%xmm5,%xmm5
	vaesenclast	160(%rcx),%xmm5,%xmm5


	vpxor	(%rdi),%xmm5,%xmm5
	vmovdqu	%xmm5,(%rsi)

	addq	$16,%rdi
	addq	$16,%rsi

	subq	$1,%r10
	jne	L$128_enc_msg_x4_loop2

L$128_enc_msg_x4_out:
	popq	%r13

	popq	%r12

	.byte	0xf3,0xc3


.globl	_aes128gcmsiv_enc_msg_x8
.private_extern _aes128gcmsiv_enc_msg_x8

.p2align	4
_aes128gcmsiv_enc_msg_x8:

_CET_ENDBR
	testq	%r8,%r8
	jnz	L$128_enc_msg_x8_start
	.byte	0xf3,0xc3

L$128_enc_msg_x8_start:
	pushq	%r12

	pushq	%r13

	pushq	%rbp

	movq	%rsp,%rbp



	subq	$128,%rsp
	andq	$-64,%rsp

	shrq	$4,%r8
	movq	%r8,%r10
	shlq	$61,%r10
	shrq	$61,%r10


	vmovdqu	(%rdx),%xmm1
	vpor	OR_MASK(%rip),%xmm1,%xmm1


	vpaddd	seven(%rip),%xmm1,%xmm0
	vmovdqu	%xmm0,(%rsp)
	vpaddd	one(%rip),%xmm1,%xmm9
	vpaddd	two(%rip),%xmm1,%xmm10
	vpaddd	three(%rip),%xmm1,%xmm11
	vpaddd	four(%rip),%xmm1,%xmm12
	vpaddd	five(%rip),%xmm1,%xmm13
	vpaddd	six(%rip),%xmm1,%xmm14
	vmovdqa	%xmm1,%xmm0

	shrq	$3,%r8
	je	L$128_enc_msg_x8_check_remainder

	subq	$128,%rsi
	subq	$128,%rdi

L$128_enc_msg_x8_loop1:
	addq	$128,%rsi
	addq	$128,%rdi

	vmovdqa	%xmm0,%xmm1
	vmovdqa	%xmm9,%xmm2
	vmovdqa	%xmm10,%xmm3
	vmovdqa	%xmm11,%xmm4
	vmovdqa	%xmm12,%xmm5
	vmovdqa	%xmm13,%xmm6
	vmovdqa	%xmm14,%xmm7

	vmovdqu	(%rsp),%xmm8

	vpxor	(%rcx),%xmm1,%xmm1
	vpxor	(%rcx),%xmm2,%xmm2
	vpxor	(%rcx),%xmm3,%xmm3
	vpxor	(%rcx),%xmm4,%xmm4
	vpxor	(%rcx),%xmm5,%xmm5
	vpxor	(%rcx),%xmm6,%xmm6
	vpxor	(%rcx),%xmm7,%xmm7
	vpxor	(%rcx),%xmm8,%xmm8

	vmovdqu	16(%rcx),%xmm15
	vaesenc	%xmm15,%xmm1,%xmm1
	vaesenc	%xmm15,%xmm2,%xmm2
	vaesenc	%xmm15,%xmm3,%xmm3
	vaesenc	%xmm15,%xmm4,%xmm4
	vaesenc	%xmm15,%xmm5,%xmm5
	vaesenc	%xmm15,%xmm6,%xmm6
	vaesenc	%xmm15,%xmm7,%xmm7
	vaesenc	%xmm15,%xmm8,%xmm8

	vmovdqu	(%rsp),%xmm14
	vpaddd	eight(%rip),%xmm14,%xmm14
	vmovdqu	%xmm14,(%rsp)
	vmovdqu	32(%rcx),%xmm15
	vaesenc	%xmm15,%xmm1,%xmm1
	vaesenc	%xmm15,%xmm2,%xmm2
	vaesenc	%xmm15,%xmm3,%xmm3
	vaesenc	%xmm15,%xmm4,%xmm4
	vaesenc	%xmm15,%xmm5,%xmm5
	vaesenc	%xmm15,%xmm6,%xmm6
	vaesenc	%xmm15,%xmm7,%xmm7
	vaesenc	%xmm15,%xmm8,%xmm8

	vpsubd	one(%rip),%xmm14,%xmm14
	vmovdqu	48(%rcx),%xmm15
	vaesenc	%xmm15,%xmm1,%xmm1
	vaesenc	%xmm15,%xmm2,%xmm2
	vaesenc	%xmm15,%xmm3,%xmm3
	vaesenc	%xmm15,%xmm4,%xmm4
	vaesenc	%xmm15,%xmm5,%xmm5
	vaesenc	%xmm15,%xmm6,%xmm6
	vaesenc	%xmm15,%xmm7,%xmm7
	vaesenc	%xmm15,%xmm8,%xmm8

	vpaddd	eight(%rip),%xmm0,%xmm0
	vmovdqu	64(%rcx),%xmm15
	vaesenc	%xmm15,%xmm1,%xmm1
	vaesenc	%xmm15,%xmm2,%xmm2
	vaesenc	%xmm15,%xmm3,%xmm3
	vaesenc	%xmm15,%xmm4,%xmm4
	vaesenc	%xmm15,%xmm5,%xmm5
	vaesenc	%xmm15,%xmm6,%xmm6
	vaesenc	%xmm15,%xmm7,%xmm7
	vaesenc	%xmm15,%xmm8,%xmm8

	vpaddd	eight(%rip),%xmm9,%xmm9
	vmovdqu	80(%rcx),%xmm15
	vaesenc	%xmm15,%xmm1,%xmm1
	vaesenc	%xmm15,%xmm2,%xmm2
	vaesenc	%xmm15,%xmm3,%xmm3
	vaesenc	%xmm15,%xmm4,%xmm4
	vaesenc	%xmm15,%xmm5,%xmm5
	vaesenc	%xmm15,%xmm6,%xmm6
	vaesenc	%xmm15,%xmm7,%xmm7
	vaesenc	%xmm15,%xmm8,%xmm8

	vpaddd	eight(%rip),%xmm10,%xmm10
	vmovdqu	96(%rcx),%xmm15
	vaesenc	%xmm15,%xmm1,%xmm1
	vaesenc	%xmm15,%xmm2,%xmm2
	vaesenc	%xmm15,%xmm3,%xmm3
	vaesenc	%xmm15,%xmm4,%xmm4
	vaesenc	%xmm15,%xmm5,%xmm5
	vaesenc	%xmm15,%xmm6,%xmm6
	vaesenc	%xmm15,%xmm7,%xmm7
	vaesenc	%xmm15,%xmm8,%xmm8

	vpaddd	eight(%rip),%xmm11,%xmm11
	vmovdqu	112(%rcx),%xmm15
	vaesenc	%xmm15,%xmm1,%xmm1
	vaesenc	%xmm15,%xmm2,%xmm2
	vaesenc	%xmm15,%xmm3,%xmm3
	vaesenc	%xmm15,%xmm4,%xmm4
	vaesenc	%xmm15,%xmm5,%xmm5
	vaesenc	%xmm15,%xmm6,%xmm6
	vaesenc	%xmm15,%xmm7,%xmm7
	vaesenc	%xmm15,%xmm8,%xmm8

	vpaddd	eight(%rip),%xmm12,%xmm12
	vmovdqu	128(%rcx),%xmm15
	vaesenc	%xmm15,%xmm1,%xmm1
	vaesenc	%xmm15,%xmm2,%xmm2
	vaesenc	%xmm15,%xmm3,%xmm3
	vaesenc	%xmm15,%xmm4,%xmm4
	vaesenc	%xmm15,%xmm5,%xmm5
	vaesenc	%xmm15,%xmm6,%xmm6
	vaesenc	%xmm15,%xmm7,%xmm7
	vaesenc	%xmm15,%xmm8,%xmm8

	vpaddd	eight(%rip),%xmm13,%xmm13
	vmovdqu	144(%rcx),%xmm15
	vaesenc	%xmm15,%xmm1,%xmm1
	vaesenc	%xmm15,%xmm2,%xmm2
	vaesenc	%xmm15,%xmm3,%xmm3
	vaesenc	%xmm15,%xmm4,%xmm4
	vaesenc	%xmm15,%xmm5,%xmm5
	vaesenc	%xmm15,%xmm6,%xmm6
	vaesenc	%xmm15,%xmm7,%xmm7
	vaesenc	%xmm15,%xmm8,%xmm8

	vmovdqu	160(%rcx),%xmm15
	vaesenclast	%xmm15,%xmm1,%xmm1
	vaesenclast	%xmm15,%xmm2,%xmm2
	vaesenclast	%xmm15,%xmm3,%xmm3
	vaesenclast	%xmm15,%xmm4,%xmm4
	vaesenclast	%xmm15,%xmm5,%xmm5
	vaesenclast	%xmm15,%xmm6,%xmm6
	vaesenclast	%xmm15,%xmm7,%xmm7
	vaesenclast	%xmm15,%xmm8,%xmm8



	vpxor	0(%rdi),%xmm1,%xmm1
	vpxor	16(%rdi),%xmm2,%xmm2
	vpxor	32(%rdi),%xmm3,%xmm3
	vpxor	48(%rdi),%xmm4,%xmm4
	vpxor	64(%rdi),%xmm5,%xmm5
	vpxor	80(%rdi),%xmm6,%xmm6
	vpxor	96(%rdi),%xmm7,%xmm7
	vpxor	112(%rdi),%xmm8,%xmm8

	decq	%r8

	vmovdqu	%xmm1,0(%rsi)
	vmovdqu	%xmm2,16(%rsi)
	vmovdqu	%xmm3,32(%rsi)
	vmovdqu	%xmm4,48(%rsi)
	vmovdqu	%xmm5,64(%rsi)
	vmovdqu	%xmm6,80(%rsi)
	vmovdqu	%xmm7,96(%rsi)
	vmovdqu	%xmm8,112(%rsi)

	jne	L$128_enc_msg_x8_loop1

	addq	$128,%rsi
	addq	$128,%rdi

L$128_enc_msg_x8_check_remainder:
	cmpq	$0,%r10
	je	L$128_enc_msg_x8_out

L$128_enc_msg_x8_loop2:


	vmovdqa	%xmm0,%xmm1
	vpaddd	one(%rip),%xmm0,%xmm0

	vpxor	(%rcx),%xmm1,%xmm1
	vaesenc	16(%rcx),%xmm1,%xmm1
	vaesenc	32(%rcx),%xmm1,%xmm1
	vaesenc	48(%rcx),%xmm1,%xmm1
	vaesenc	64(%rcx),%xmm1,%xmm1
	vaesenc	80(%rcx),%xmm1,%xmm1
	vaesenc	96(%rcx),%xmm1,%xmm1
	vaesenc	112(%rcx),%xmm1,%xmm1
	vaesenc	128(%rcx),%xmm1,%xmm1
	vaesenc	144(%rcx),%xmm1,%xmm1
	vaesenclast	160(%rcx),%xmm1,%xmm1


	vpxor	(%rdi),%xmm1,%xmm1

	vmovdqu	%xmm1,(%rsi)

	addq	$16,%rdi
	addq	$16,%rsi

	decq	%r10
	jne	L$128_enc_msg_x8_loop2

L$128_enc_msg_x8_out:
	movq	%rbp,%rsp

	popq	%rbp

	popq	%r13

	popq	%r12

	.byte	0xf3,0xc3


.globl	_aes128gcmsiv_dec
.private_extern _aes128gcmsiv_dec

.p2align	4
_aes128gcmsiv_dec:

_CET_ENDBR
	testq	$~15,%r9
	jnz	L$128_dec_start
	.byte	0xf3,0xc3

L$128_dec_start:
	vzeroupper
	vmovdqa	(%rdx),%xmm0


	vmovdqu	16(%rdx),%xmm15
	vpor	OR_MASK(%rip),%xmm15,%xmm15
	movq	%rdx,%rax

	leaq	32(%rax),%rax
	leaq	32(%rcx),%rcx

	andq	$~15,%r9


	cmpq	$96,%r9
	jb	L$128_dec_loop2


	subq	$96,%r9
	vmovdqa	%xmm15,%xmm7
	vpaddd	one(%rip),%xmm7,%xmm8
	vpaddd	two(%rip),%xmm7,%xmm9
	vpaddd	one(%rip),%xmm9,%xmm10
	vpaddd	two(%rip),%xmm9,%xmm11
	vpaddd	one(%rip),%xmm11,%xmm12
	vpaddd	two(%rip),%xmm11,%xmm15

	vpxor	(%r8),%xmm7,%xmm7
	vpxor	(%r8),%xmm8,%xmm8
	vpxor	(%r8),%xmm9,%xmm9
	vpxor	(%r8),%xmm10,%xmm10
	vpxor	(%r8),%xmm11,%xmm11
	vpxor	(%r8),%xmm12,%xmm12

	vmovdqu	16(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	32(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	48(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	64(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	80(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	96(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	112(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	128(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	144(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	160(%r8),%xmm4
	vaesenclast	%xmm4,%xmm7,%xmm7
	vaesenclast	%xmm4,%xmm8,%xmm8
	vaesenclast	%xmm4,%xmm9,%xmm9
	vaesenclast	%xmm4,%xmm10,%xmm10
	vaesenclast	%xmm4,%xmm11,%xmm11
	vaesenclast	%xmm4,%xmm12,%xmm12


	vpxor	0(%rdi),%xmm7,%xmm7
	vpxor	16(%rdi),%xmm8,%xmm8
	vpxor	32(%rdi),%xmm9,%xmm9
	vpxor	48(%rdi),%xmm10,%xmm10
	vpxor	64(%rdi),%xmm11,%xmm11
	vpxor	80(%rdi),%xmm12,%xmm12

	vmovdqu	%xmm7,0(%rsi)
	vmovdqu	%xmm8,16(%rsi)
	vmovdqu	%xmm9,32(%rsi)
	vmovdqu	%xmm10,48(%rsi)
	vmovdqu	%xmm11,64(%rsi)
	vmovdqu	%xmm12,80(%rsi)

	addq	$96,%rdi
	addq	$96,%rsi
	jmp	L$128_dec_loop1


.p2align	6
L$128_dec_loop1:
	cmpq	$96,%r9
	jb	L$128_dec_finish_96
	subq	$96,%r9

	vmovdqa	%xmm12,%xmm6
	vmovdqa	%xmm11,16-32(%rax)
	vmovdqa	%xmm10,32-32(%rax)
	vmovdqa	%xmm9,48-32(%rax)
	vmovdqa	%xmm8,64-32(%rax)
	vmovdqa	%xmm7,80-32(%rax)

	vmovdqa	%xmm15,%xmm7
	vpaddd	one(%rip),%xmm7,%xmm8
	vpaddd	two(%rip),%xmm7,%xmm9
	vpaddd	one(%rip),%xmm9,%xmm10
	vpaddd	two(%rip),%xmm9,%xmm11
	vpaddd	one(%rip),%xmm11,%xmm12
	vpaddd	two(%rip),%xmm11,%xmm15

	vmovdqa	(%r8),%xmm4
	vpxor	%xmm4,%xmm7,%xmm7
	vpxor	%xmm4,%xmm8,%xmm8
	vpxor	%xmm4,%xmm9,%xmm9
	vpxor	%xmm4,%xmm10,%xmm10
	vpxor	%xmm4,%xmm11,%xmm11
	vpxor	%xmm4,%xmm12,%xmm12

	vmovdqu	0-32(%rcx),%xmm4
	vpclmulqdq	$0x11,%xmm4,%xmm6,%xmm2
	vpclmulqdq	$0x00,%xmm4,%xmm6,%xmm3
	vpclmulqdq	$0x01,%xmm4,%xmm6,%xmm1
	vpclmulqdq	$0x10,%xmm4,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1

	vmovdqu	16(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	-16(%rax),%xmm6
	vmovdqu	-16(%rcx),%xmm13

	vpclmulqdq	$0x10,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpclmulqdq	$0x11,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm2,%xmm2
	vpclmulqdq	$0x00,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpclmulqdq	$0x01,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1


	vmovdqu	32(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	0(%rax),%xmm6
	vmovdqu	0(%rcx),%xmm13

	vpclmulqdq	$0x10,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpclmulqdq	$0x11,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm2,%xmm2
	vpclmulqdq	$0x00,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpclmulqdq	$0x01,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1


	vmovdqu	48(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	16(%rax),%xmm6
	vmovdqu	16(%rcx),%xmm13

	vpclmulqdq	$0x10,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpclmulqdq	$0x11,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm2,%xmm2
	vpclmulqdq	$0x00,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpclmulqdq	$0x01,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1


	vmovdqu	64(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	32(%rax),%xmm6
	vmovdqu	32(%rcx),%xmm13

	vpclmulqdq	$0x10,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpclmulqdq	$0x11,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm2,%xmm2
	vpclmulqdq	$0x00,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpclmulqdq	$0x01,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1


	vmovdqu	80(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	96(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	112(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12


	vmovdqa	80-32(%rax),%xmm6
	vpxor	%xmm0,%xmm6,%xmm6
	vmovdqu	80-32(%rcx),%xmm5

	vpclmulqdq	$0x01,%xmm5,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpclmulqdq	$0x11,%xmm5,%xmm6,%xmm4
	vpxor	%xmm4,%xmm2,%xmm2
	vpclmulqdq	$0x00,%xmm5,%xmm6,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpclmulqdq	$0x10,%xmm5,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1

	vmovdqu	128(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12


	vpsrldq	$8,%xmm1,%xmm4
	vpxor	%xmm4,%xmm2,%xmm5
	vpslldq	$8,%xmm1,%xmm4
	vpxor	%xmm4,%xmm3,%xmm0

	vmovdqa	poly(%rip),%xmm3

	vmovdqu	144(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	160(%r8),%xmm6
	vpalignr	$8,%xmm0,%xmm0,%xmm2
	vpclmulqdq	$0x10,%xmm3,%xmm0,%xmm0
	vpxor	%xmm0,%xmm2,%xmm0

	vpxor	0(%rdi),%xmm6,%xmm4
	vaesenclast	%xmm4,%xmm7,%xmm7
	vpxor	16(%rdi),%xmm6,%xmm4
	vaesenclast	%xmm4,%xmm8,%xmm8
	vpxor	32(%rdi),%xmm6,%xmm4
	vaesenclast	%xmm4,%xmm9,%xmm9
	vpxor	48(%rdi),%xmm6,%xmm4
	vaesenclast	%xmm4,%xmm10,%xmm10
	vpxor	64(%rdi),%xmm6,%xmm4
	vaesenclast	%xmm4,%xmm11,%xmm11
	vpxor	80(%rdi),%xmm6,%xmm4
	vaesenclast	%xmm4,%xmm12,%xmm12

	vpalignr	$8,%xmm0,%xmm0,%xmm2
	vpclmulqdq	$0x10,%xmm3,%xmm0,%xmm0
	vpxor	%xmm0,%xmm2,%xmm0

	vmovdqu	%xmm7,0(%rsi)
	vmovdqu	%xmm8,16(%rsi)
	vmovdqu	%xmm9,32(%rsi)
	vmovdqu	%xmm10,48(%rsi)
	vmovdqu	%xmm11,64(%rsi)
	vmovdqu	%xmm12,80(%rsi)

	vpxor	%xmm5,%xmm0,%xmm0

	leaq	96(%rdi),%rdi
	leaq	96(%rsi),%rsi
	jmp	L$128_dec_loop1

L$128_dec_finish_96:
	vmovdqa	%xmm12,%xmm6
	vmovdqa	%xmm11,16-32(%rax)
	vmovdqa	%xmm10,32-32(%rax)
	vmovdqa	%xmm9,48-32(%rax)
	vmovdqa	%xmm8,64-32(%rax)
	vmovdqa	%xmm7,80-32(%rax)

	vmovdqu	0-32(%rcx),%xmm4
	vpclmulqdq	$0x10,%xmm4,%xmm6,%xmm1
	vpclmulqdq	$0x11,%xmm4,%xmm6,%xmm2
	vpclmulqdq	$0x00,%xmm4,%xmm6,%xmm3
	vpclmulqdq	$0x01,%xmm4,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1

	vmovdqu	-16(%rax),%xmm6
	vmovdqu	-16(%rcx),%xmm13

	vpclmulqdq	$0x10,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpclmulqdq	$0x11,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm2,%xmm2
	vpclmulqdq	$0x00,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpclmulqdq	$0x01,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1

	vmovdqu	0(%rax),%xmm6
	vmovdqu	0(%rcx),%xmm13

	vpclmulqdq	$0x10,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpclmulqdq	$0x11,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm2,%xmm2
	vpclmulqdq	$0x00,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpclmulqdq	$0x01,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1

	vmovdqu	16(%rax),%xmm6
	vmovdqu	16(%rcx),%xmm13

	vpclmulqdq	$0x10,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpclmulqdq	$0x11,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm2,%xmm2
	vpclmulqdq	$0x00,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpclmulqdq	$0x01,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1

	vmovdqu	32(%rax),%xmm6
	vmovdqu	32(%rcx),%xmm13

	vpclmulqdq	$0x10,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpclmulqdq	$0x11,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm2,%xmm2
	vpclmulqdq	$0x00,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpclmulqdq	$0x01,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1


	vmovdqu	80-32(%rax),%xmm6
	vpxor	%xmm0,%xmm6,%xmm6
	vmovdqu	80-32(%rcx),%xmm5
	vpclmulqdq	$0x11,%xmm5,%xmm6,%xmm4
	vpxor	%xmm4,%xmm2,%xmm2
	vpclmulqdq	$0x00,%xmm5,%xmm6,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpclmulqdq	$0x10,%xmm5,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpclmulqdq	$0x01,%xmm5,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1

	vpsrldq	$8,%xmm1,%xmm4
	vpxor	%xmm4,%xmm2,%xmm5
	vpslldq	$8,%xmm1,%xmm4
	vpxor	%xmm4,%xmm3,%xmm0

	vmovdqa	poly(%rip),%xmm3

	vpalignr	$8,%xmm0,%xmm0,%xmm2
	vpclmulqdq	$0x10,%xmm3,%xmm0,%xmm0
	vpxor	%xmm0,%xmm2,%xmm0

	vpalignr	$8,%xmm0,%xmm0,%xmm2
	vpclmulqdq	$0x10,%xmm3,%xmm0,%xmm0
	vpxor	%xmm0,%xmm2,%xmm0

	vpxor	%xmm5,%xmm0,%xmm0

L$128_dec_loop2:



	cmpq	$16,%r9
	jb	L$128_dec_out
	subq	$16,%r9

	vmovdqa	%xmm15,%xmm2
	vpaddd	one(%rip),%xmm15,%xmm15

	vpxor	0(%r8),%xmm2,%xmm2
	vaesenc	16(%r8),%xmm2,%xmm2
	vaesenc	32(%r8),%xmm2,%xmm2
	vaesenc	48(%r8),%xmm2,%xmm2
	vaesenc	64(%r8),%xmm2,%xmm2
	vaesenc	80(%r8),%xmm2,%xmm2
	vaesenc	96(%r8),%xmm2,%xmm2
	vaesenc	112(%r8),%xmm2,%xmm2
	vaesenc	128(%r8),%xmm2,%xmm2
	vaesenc	144(%r8),%xmm2,%xmm2
	vaesenclast	160(%r8),%xmm2,%xmm2
	vpxor	(%rdi),%xmm2,%xmm2
	vmovdqu	%xmm2,(%rsi)
	addq	$16,%rdi
	addq	$16,%rsi

	vpxor	%xmm2,%xmm0,%xmm0
	vmovdqa	-32(%rcx),%xmm1
	call	GFMUL

	jmp	L$128_dec_loop2

L$128_dec_out:
	vmovdqu	%xmm0,(%rdx)
	.byte	0xf3,0xc3


.globl	_aes128gcmsiv_ecb_enc_block
.private_extern _aes128gcmsiv_ecb_enc_block

.p2align	4
_aes128gcmsiv_ecb_enc_block:

_CET_ENDBR
	vmovdqa	(%rdi),%xmm1

	vpxor	(%rdx),%xmm1,%xmm1
	vaesenc	16(%rdx),%xmm1,%xmm1
	vaesenc	32(%rdx),%xmm1,%xmm1
	vaesenc	48(%rdx),%xmm1,%xmm1
	vaesenc	64(%rdx),%xmm1,%xmm1
	vaesenc	80(%rdx),%xmm1,%xmm1
	vaesenc	96(%rdx),%xmm1,%xmm1
	vaesenc	112(%rdx),%xmm1,%xmm1
	vaesenc	128(%rdx),%xmm1,%xmm1
	vaesenc	144(%rdx),%xmm1,%xmm1
	vaesenclast	160(%rdx),%xmm1,%xmm1

	vmovdqa	%xmm1,(%rsi)

	.byte	0xf3,0xc3


.globl	_aes256gcmsiv_aes_ks_enc_x1
.private_extern _aes256gcmsiv_aes_ks_enc_x1

.p2align	4
_aes256gcmsiv_aes_ks_enc_x1:

_CET_ENDBR
	vmovdqa	con1(%rip),%xmm0
	vmovdqa	mask(%rip),%xmm15
	vmovdqa	(%rdi),%xmm8
	vmovdqa	(%rcx),%xmm1
	vmovdqa	16(%rcx),%xmm3
	vpxor	%xmm1,%xmm8,%xmm8
	vaesenc	%xmm3,%xmm8,%xmm8
	vmovdqu	%xmm1,(%rdx)
	vmovdqu	%xmm3,16(%rdx)
	vpxor	%xmm14,%xmm14,%xmm14

	vpshufb	%xmm15,%xmm3,%xmm2
	vaesenclast	%xmm0,%xmm2,%xmm2
	vpslld	$1,%xmm0,%xmm0
	vpslldq	$4,%xmm1,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpxor	%xmm2,%xmm1,%xmm1
	vaesenc	%xmm1,%xmm8,%xmm8
	vmovdqu	%xmm1,32(%rdx)

	vpshufd	$0xff,%xmm1,%xmm2
	vaesenclast	%xmm14,%xmm2,%xmm2
	vpslldq	$4,%xmm3,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpxor	%xmm2,%xmm3,%xmm3
	vaesenc	%xmm3,%xmm8,%xmm8
	vmovdqu	%xmm3,48(%rdx)

	vpshufb	%xmm15,%xmm3,%xmm2
	vaesenclast	%xmm0,%xmm2,%xmm2
	vpslld	$1,%xmm0,%xmm0
	vpslldq	$4,%xmm1,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpxor	%xmm2,%xmm1,%xmm1
	vaesenc	%xmm1,%xmm8,%xmm8
	vmovdqu	%xmm1,64(%rdx)

	vpshufd	$0xff,%xmm1,%xmm2
	vaesenclast	%xmm14,%xmm2,%xmm2
	vpslldq	$4,%xmm3,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpxor	%xmm2,%xmm3,%xmm3
	vaesenc	%xmm3,%xmm8,%xmm8
	vmovdqu	%xmm3,80(%rdx)

	vpshufb	%xmm15,%xmm3,%xmm2
	vaesenclast	%xmm0,%xmm2,%xmm2
	vpslld	$1,%xmm0,%xmm0
	vpslldq	$4,%xmm1,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpxor	%xmm2,%xmm1,%xmm1
	vaesenc	%xmm1,%xmm8,%xmm8
	vmovdqu	%xmm1,96(%rdx)

	vpshufd	$0xff,%xmm1,%xmm2
	vaesenclast	%xmm14,%xmm2,%xmm2
	vpslldq	$4,%xmm3,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpxor	%xmm2,%xmm3,%xmm3
	vaesenc	%xmm3,%xmm8,%xmm8
	vmovdqu	%xmm3,112(%rdx)

	vpshufb	%xmm15,%xmm3,%xmm2
	vaesenclast	%xmm0,%xmm2,%xmm2
	vpslld	$1,%xmm0,%xmm0
	vpslldq	$4,%xmm1,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpxor	%xmm2,%xmm1,%xmm1
	vaesenc	%xmm1,%xmm8,%xmm8
	vmovdqu	%xmm1,128(%rdx)

	vpshufd	$0xff,%xmm1,%xmm2
	vaesenclast	%xmm14,%xmm2,%xmm2
	vpslldq	$4,%xmm3,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpxor	%xmm2,%xmm3,%xmm3
	vaesenc	%xmm3,%xmm8,%xmm8
	vmovdqu	%xmm3,144(%rdx)

	vpshufb	%xmm15,%xmm3,%xmm2
	vaesenclast	%xmm0,%xmm2,%xmm2
	vpslld	$1,%xmm0,%xmm0
	vpslldq	$4,%xmm1,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpxor	%xmm2,%xmm1,%xmm1
	vaesenc	%xmm1,%xmm8,%xmm8
	vmovdqu	%xmm1,160(%rdx)

	vpshufd	$0xff,%xmm1,%xmm2
	vaesenclast	%xmm14,%xmm2,%xmm2
	vpslldq	$4,%xmm3,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpxor	%xmm2,%xmm3,%xmm3
	vaesenc	%xmm3,%xmm8,%xmm8
	vmovdqu	%xmm3,176(%rdx)

	vpshufb	%xmm15,%xmm3,%xmm2
	vaesenclast	%xmm0,%xmm2,%xmm2
	vpslld	$1,%xmm0,%xmm0
	vpslldq	$4,%xmm1,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpxor	%xmm2,%xmm1,%xmm1
	vaesenc	%xmm1,%xmm8,%xmm8
	vmovdqu	%xmm1,192(%rdx)

	vpshufd	$0xff,%xmm1,%xmm2
	vaesenclast	%xmm14,%xmm2,%xmm2
	vpslldq	$4,%xmm3,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpxor	%xmm2,%xmm3,%xmm3
	vaesenc	%xmm3,%xmm8,%xmm8
	vmovdqu	%xmm3,208(%rdx)

	vpshufb	%xmm15,%xmm3,%xmm2
	vaesenclast	%xmm0,%xmm2,%xmm2
	vpslldq	$4,%xmm1,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpslldq	$4,%xmm4,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpxor	%xmm2,%xmm1,%xmm1
	vaesenclast	%xmm1,%xmm8,%xmm8
	vmovdqu	%xmm1,224(%rdx)

	vmovdqa	%xmm8,(%rsi)
	.byte	0xf3,0xc3


.globl	_aes256gcmsiv_ecb_enc_block
.private_extern _aes256gcmsiv_ecb_enc_block

.p2align	4
_aes256gcmsiv_ecb_enc_block:

_CET_ENDBR
	vmovdqa	(%rdi),%xmm1
	vpxor	(%rdx),%xmm1,%xmm1
	vaesenc	16(%rdx),%xmm1,%xmm1
	vaesenc	32(%rdx),%xmm1,%xmm1
	vaesenc	48(%rdx),%xmm1,%xmm1
	vaesenc	64(%rdx),%xmm1,%xmm1
	vaesenc	80(%rdx),%xmm1,%xmm1
	vaesenc	96(%rdx),%xmm1,%xmm1
	vaesenc	112(%rdx),%xmm1,%xmm1
	vaesenc	128(%rdx),%xmm1,%xmm1
	vaesenc	144(%rdx),%xmm1,%xmm1
	vaesenc	160(%rdx),%xmm1,%xmm1
	vaesenc	176(%rdx),%xmm1,%xmm1
	vaesenc	192(%rdx),%xmm1,%xmm1
	vaesenc	208(%rdx),%xmm1,%xmm1
	vaesenclast	224(%rdx),%xmm1,%xmm1
	vmovdqa	%xmm1,(%rsi)
	.byte	0xf3,0xc3


.globl	_aes256gcmsiv_enc_msg_x4
.private_extern _aes256gcmsiv_enc_msg_x4

.p2align	4
_aes256gcmsiv_enc_msg_x4:

_CET_ENDBR
	testq	%r8,%r8
	jnz	L$256_enc_msg_x4_start
	.byte	0xf3,0xc3

L$256_enc_msg_x4_start:
	movq	%r8,%r10
	shrq	$4,%r8
	shlq	$60,%r10
	jz	L$256_enc_msg_x4_start2
	addq	$1,%r8

L$256_enc_msg_x4_start2:
	movq	%r8,%r10
	shlq	$62,%r10
	shrq	$62,%r10


	vmovdqa	(%rdx),%xmm15
	vpor	OR_MASK(%rip),%xmm15,%xmm15

	vmovdqa	four(%rip),%xmm4
	vmovdqa	%xmm15,%xmm0
	vpaddd	one(%rip),%xmm15,%xmm1
	vpaddd	two(%rip),%xmm15,%xmm2
	vpaddd	three(%rip),%xmm15,%xmm3

	shrq	$2,%r8
	je	L$256_enc_msg_x4_check_remainder

	subq	$64,%rsi
	subq	$64,%rdi

L$256_enc_msg_x4_loop1:
	addq	$64,%rsi
	addq	$64,%rdi

	vmovdqa	%xmm0,%xmm5
	vmovdqa	%xmm1,%xmm6
	vmovdqa	%xmm2,%xmm7
	vmovdqa	%xmm3,%xmm8

	vpxor	(%rcx),%xmm5,%xmm5
	vpxor	(%rcx),%xmm6,%xmm6
	vpxor	(%rcx),%xmm7,%xmm7
	vpxor	(%rcx),%xmm8,%xmm8

	vmovdqu	16(%rcx),%xmm12
	vaesenc	%xmm12,%xmm5,%xmm5
	vaesenc	%xmm12,%xmm6,%xmm6
	vaesenc	%xmm12,%xmm7,%xmm7
	vaesenc	%xmm12,%xmm8,%xmm8

	vpaddd	%xmm4,%xmm0,%xmm0
	vmovdqu	32(%rcx),%xmm12
	vaesenc	%xmm12,%xmm5,%xmm5
	vaesenc	%xmm12,%xmm6,%xmm6
	vaesenc	%xmm12,%xmm7,%xmm7
	vaesenc	%xmm12,%xmm8,%xmm8

	vpaddd	%xmm4,%xmm1,%xmm1
	vmovdqu	48(%rcx),%xmm12
	vaesenc	%xmm12,%xmm5,%xmm5
	vaesenc	%xmm12,%xmm6,%xmm6
	vaesenc	%xmm12,%xmm7,%xmm7
	vaesenc	%xmm12,%xmm8,%xmm8

	vpaddd	%xmm4,%xmm2,%xmm2
	vmovdqu	64(%rcx),%xmm12
	vaesenc	%xmm12,%xmm5,%xmm5
	vaesenc	%xmm12,%xmm6,%xmm6
	vaesenc	%xmm12,%xmm7,%xmm7
	vaesenc	%xmm12,%xmm8,%xmm8

	vpaddd	%xmm4,%xmm3,%xmm3

	vmovdqu	80(%rcx),%xmm12
	vaesenc	%xmm12,%xmm5,%xmm5
	vaesenc	%xmm12,%xmm6,%xmm6
	vaesenc	%xmm12,%xmm7,%xmm7
	vaesenc	%xmm12,%xmm8,%xmm8

	vmovdqu	96(%rcx),%xmm12
	vaesenc	%xmm12,%xmm5,%xmm5
	vaesenc	%xmm12,%xmm6,%xmm6
	vaesenc	%xmm12,%xmm7,%xmm7
	vaesenc	%xmm12,%xmm8,%xmm8

	vmovdqu	112(%rcx),%xmm12
	vaesenc	%xmm12,%xmm5,%xmm5
	vaesenc	%xmm12,%xmm6,%xmm6
	vaesenc	%xmm12,%xmm7,%xmm7
	vaesenc	%xmm12,%xmm8,%xmm8

	vmovdqu	128(%rcx),%xmm12
	vaesenc	%xmm12,%xmm5,%xmm5
	vaesenc	%xmm12,%xmm6,%xmm6
	vaesenc	%xmm12,%xmm7,%xmm7
	vaesenc	%xmm12,%xmm8,%xmm8

	vmovdqu	144(%rcx),%xmm12
	vaesenc	%xmm12,%xmm5,%xmm5
	vaesenc	%xmm12,%xmm6,%xmm6
	vaesenc	%xmm12,%xmm7,%xmm7
	vaesenc	%xmm12,%xmm8,%xmm8

	vmovdqu	160(%rcx),%xmm12
	vaesenc	%xmm12,%xmm5,%xmm5
	vaesenc	%xmm12,%xmm6,%xmm6
	vaesenc	%xmm12,%xmm7,%xmm7
	vaesenc	%xmm12,%xmm8,%xmm8

	vmovdqu	176(%rcx),%xmm12
	vaesenc	%xmm12,%xmm5,%xmm5
	vaesenc	%xmm12,%xmm6,%xmm6
	vaesenc	%xmm12,%xmm7,%xmm7
	vaesenc	%xmm12,%xmm8,%xmm8

	vmovdqu	192(%rcx),%xmm12
	vaesenc	%xmm12,%xmm5,%xmm5
	vaesenc	%xmm12,%xmm6,%xmm6
	vaesenc	%xmm12,%xmm7,%xmm7
	vaesenc	%xmm12,%xmm8,%xmm8

	vmovdqu	208(%rcx),%xmm12
	vaesenc	%xmm12,%xmm5,%xmm5
	vaesenc	%xmm12,%xmm6,%xmm6
	vaesenc	%xmm12,%xmm7,%xmm7
	vaesenc	%xmm12,%xmm8,%xmm8

	vmovdqu	224(%rcx),%xmm12
	vaesenclast	%xmm12,%xmm5,%xmm5
	vaesenclast	%xmm12,%xmm6,%xmm6
	vaesenclast	%xmm12,%xmm7,%xmm7
	vaesenclast	%xmm12,%xmm8,%xmm8



	vpxor	0(%rdi),%xmm5,%xmm5
	vpxor	16(%rdi),%xmm6,%xmm6
	vpxor	32(%rdi),%xmm7,%xmm7
	vpxor	48(%rdi),%xmm8,%xmm8

	subq	$1,%r8

	vmovdqu	%xmm5,0(%rsi)
	vmovdqu	%xmm6,16(%rsi)
	vmovdqu	%xmm7,32(%rsi)
	vmovdqu	%xmm8,48(%rsi)

	jne	L$256_enc_msg_x4_loop1

	addq	$64,%rsi
	addq	$64,%rdi

L$256_enc_msg_x4_check_remainder:
	cmpq	$0,%r10
	je	L$256_enc_msg_x4_out

L$256_enc_msg_x4_loop2:



	vmovdqa	%xmm0,%xmm5
	vpaddd	one(%rip),%xmm0,%xmm0
	vpxor	(%rcx),%xmm5,%xmm5
	vaesenc	16(%rcx),%xmm5,%xmm5
	vaesenc	32(%rcx),%xmm5,%xmm5
	vaesenc	48(%rcx),%xmm5,%xmm5
	vaesenc	64(%rcx),%xmm5,%xmm5
	vaesenc	80(%rcx),%xmm5,%xmm5
	vaesenc	96(%rcx),%xmm5,%xmm5
	vaesenc	112(%rcx),%xmm5,%xmm5
	vaesenc	128(%rcx),%xmm5,%xmm5
	vaesenc	144(%rcx),%xmm5,%xmm5
	vaesenc	160(%rcx),%xmm5,%xmm5
	vaesenc	176(%rcx),%xmm5,%xmm5
	vaesenc	192(%rcx),%xmm5,%xmm5
	vaesenc	208(%rcx),%xmm5,%xmm5
	vaesenclast	224(%rcx),%xmm5,%xmm5


	vpxor	(%rdi),%xmm5,%xmm5

	vmovdqu	%xmm5,(%rsi)

	addq	$16,%rdi
	addq	$16,%rsi

	subq	$1,%r10
	jne	L$256_enc_msg_x4_loop2

L$256_enc_msg_x4_out:
	.byte	0xf3,0xc3


.globl	_aes256gcmsiv_enc_msg_x8
.private_extern _aes256gcmsiv_enc_msg_x8

.p2align	4
_aes256gcmsiv_enc_msg_x8:

_CET_ENDBR
	testq	%r8,%r8
	jnz	L$256_enc_msg_x8_start
	.byte	0xf3,0xc3

L$256_enc_msg_x8_start:

	movq	%rsp,%r11
	subq	$16,%r11
	andq	$-64,%r11

	movq	%r8,%r10
	shrq	$4,%r8
	shlq	$60,%r10
	jz	L$256_enc_msg_x8_start2
	addq	$1,%r8

L$256_enc_msg_x8_start2:
	movq	%r8,%r10
	shlq	$61,%r10
	shrq	$61,%r10


	vmovdqa	(%rdx),%xmm1
	vpor	OR_MASK(%rip),%xmm1,%xmm1


	vpaddd	seven(%rip),%xmm1,%xmm0
	vmovdqa	%xmm0,(%r11)
	vpaddd	one(%rip),%xmm1,%xmm9
	vpaddd	two(%rip),%xmm1,%xmm10
	vpaddd	three(%rip),%xmm1,%xmm11
	vpaddd	four(%rip),%xmm1,%xmm12
	vpaddd	five(%rip),%xmm1,%xmm13
	vpaddd	six(%rip),%xmm1,%xmm14
	vmovdqa	%xmm1,%xmm0

	shrq	$3,%r8
	jz	L$256_enc_msg_x8_check_remainder

	subq	$128,%rsi
	subq	$128,%rdi

L$256_enc_msg_x8_loop1:
	addq	$128,%rsi
	addq	$128,%rdi

	vmovdqa	%xmm0,%xmm1
	vmovdqa	%xmm9,%xmm2
	vmovdqa	%xmm10,%xmm3
	vmovdqa	%xmm11,%xmm4
	vmovdqa	%xmm12,%xmm5
	vmovdqa	%xmm13,%xmm6
	vmovdqa	%xmm14,%xmm7

	vmovdqa	(%r11),%xmm8

	vpxor	(%rcx),%xmm1,%xmm1
	vpxor	(%rcx),%xmm2,%xmm2
	vpxor	(%rcx),%xmm3,%xmm3
	vpxor	(%rcx),%xmm4,%xmm4
	vpxor	(%rcx),%xmm5,%xmm5
	vpxor	(%rcx),%xmm6,%xmm6
	vpxor	(%rcx),%xmm7,%xmm7
	vpxor	(%rcx),%xmm8,%xmm8

	vmovdqu	16(%rcx),%xmm15
	vaesenc	%xmm15,%xmm1,%xmm1
	vaesenc	%xmm15,%xmm2,%xmm2
	vaesenc	%xmm15,%xmm3,%xmm3
	vaesenc	%xmm15,%xmm4,%xmm4
	vaesenc	%xmm15,%xmm5,%xmm5
	vaesenc	%xmm15,%xmm6,%xmm6
	vaesenc	%xmm15,%xmm7,%xmm7
	vaesenc	%xmm15,%xmm8,%xmm8

	vmovdqa	(%r11),%xmm14
	vpaddd	eight(%rip),%xmm14,%xmm14
	vmovdqa	%xmm14,(%r11)
	vmovdqu	32(%rcx),%xmm15
	vaesenc	%xmm15,%xmm1,%xmm1
	vaesenc	%xmm15,%xmm2,%xmm2
	vaesenc	%xmm15,%xmm3,%xmm3
	vaesenc	%xmm15,%xmm4,%xmm4
	vaesenc	%xmm15,%xmm5,%xmm5
	vaesenc	%xmm15,%xmm6,%xmm6
	vaesenc	%xmm15,%xmm7,%xmm7
	vaesenc	%xmm15,%xmm8,%xmm8

	vpsubd	one(%rip),%xmm14,%xmm14
	vmovdqu	48(%rcx),%xmm15
	vaesenc	%xmm15,%xmm1,%xmm1
	vaesenc	%xmm15,%xmm2,%xmm2
	vaesenc	%xmm15,%xmm3,%xmm3
	vaesenc	%xmm15,%xmm4,%xmm4
	vaesenc	%xmm15,%xmm5,%xmm5
	vaesenc	%xmm15,%xmm6,%xmm6
	vaesenc	%xmm15,%xmm7,%xmm7
	vaesenc	%xmm15,%xmm8,%xmm8

	vpaddd	eight(%rip),%xmm0,%xmm0
	vmovdqu	64(%rcx),%xmm15
	vaesenc	%xmm15,%xmm1,%xmm1
	vaesenc	%xmm15,%xmm2,%xmm2
	vaesenc	%xmm15,%xmm3,%xmm3
	vaesenc	%xmm15,%xmm4,%xmm4
	vaesenc	%xmm15,%xmm5,%xmm5
	vaesenc	%xmm15,%xmm6,%xmm6
	vaesenc	%xmm15,%xmm7,%xmm7
	vaesenc	%xmm15,%xmm8,%xmm8

	vpaddd	eight(%rip),%xmm9,%xmm9
	vmovdqu	80(%rcx),%xmm15
	vaesenc	%xmm15,%xmm1,%xmm1
	vaesenc	%xmm15,%xmm2,%xmm2
	vaesenc	%xmm15,%xmm3,%xmm3
	vaesenc	%xmm15,%xmm4,%xmm4
	vaesenc	%xmm15,%xmm5,%xmm5
	vaesenc	%xmm15,%xmm6,%xmm6
	vaesenc	%xmm15,%xmm7,%xmm7
	vaesenc	%xmm15,%xmm8,%xmm8

	vpaddd	eight(%rip),%xmm10,%xmm10
	vmovdqu	96(%rcx),%xmm15
	vaesenc	%xmm15,%xmm1,%xmm1
	vaesenc	%xmm15,%xmm2,%xmm2
	vaesenc	%xmm15,%xmm3,%xmm3
	vaesenc	%xmm15,%xmm4,%xmm4
	vaesenc	%xmm15,%xmm5,%xmm5
	vaesenc	%xmm15,%xmm6,%xmm6
	vaesenc	%xmm15,%xmm7,%xmm7
	vaesenc	%xmm15,%xmm8,%xmm8

	vpaddd	eight(%rip),%xmm11,%xmm11
	vmovdqu	112(%rcx),%xmm15
	vaesenc	%xmm15,%xmm1,%xmm1
	vaesenc	%xmm15,%xmm2,%xmm2
	vaesenc	%xmm15,%xmm3,%xmm3
	vaesenc	%xmm15,%xmm4,%xmm4
	vaesenc	%xmm15,%xmm5,%xmm5
	vaesenc	%xmm15,%xmm6,%xmm6
	vaesenc	%xmm15,%xmm7,%xmm7
	vaesenc	%xmm15,%xmm8,%xmm8

	vpaddd	eight(%rip),%xmm12,%xmm12
	vmovdqu	128(%rcx),%xmm15
	vaesenc	%xmm15,%xmm1,%xmm1
	vaesenc	%xmm15,%xmm2,%xmm2
	vaesenc	%xmm15,%xmm3,%xmm3
	vaesenc	%xmm15,%xmm4,%xmm4
	vaesenc	%xmm15,%xmm5,%xmm5
	vaesenc	%xmm15,%xmm6,%xmm6
	vaesenc	%xmm15,%xmm7,%xmm7
	vaesenc	%xmm15,%xmm8,%xmm8

	vpaddd	eight(%rip),%xmm13,%xmm13
	vmovdqu	144(%rcx),%xmm15
	vaesenc	%xmm15,%xmm1,%xmm1
	vaesenc	%xmm15,%xmm2,%xmm2
	vaesenc	%xmm15,%xmm3,%xmm3
	vaesenc	%xmm15,%xmm4,%xmm4
	vaesenc	%xmm15,%xmm5,%xmm5
	vaesenc	%xmm15,%xmm6,%xmm6
	vaesenc	%xmm15,%xmm7,%xmm7
	vaesenc	%xmm15,%xmm8,%xmm8

	vmovdqu	160(%rcx),%xmm15
	vaesenc	%xmm15,%xmm1,%xmm1
	vaesenc	%xmm15,%xmm2,%xmm2
	vaesenc	%xmm15,%xmm3,%xmm3
	vaesenc	%xmm15,%xmm4,%xmm4
	vaesenc	%xmm15,%xmm5,%xmm5
	vaesenc	%xmm15,%xmm6,%xmm6
	vaesenc	%xmm15,%xmm7,%xmm7
	vaesenc	%xmm15,%xmm8,%xmm8

	vmovdqu	176(%rcx),%xmm15
	vaesenc	%xmm15,%xmm1,%xmm1
	vaesenc	%xmm15,%xmm2,%xmm2
	vaesenc	%xmm15,%xmm3,%xmm3
	vaesenc	%xmm15,%xmm4,%xmm4
	vaesenc	%xmm15,%xmm5,%xmm5
	vaesenc	%xmm15,%xmm6,%xmm6
	vaesenc	%xmm15,%xmm7,%xmm7
	vaesenc	%xmm15,%xmm8,%xmm8

	vmovdqu	192(%rcx),%xmm15
	vaesenc	%xmm15,%xmm1,%xmm1
	vaesenc	%xmm15,%xmm2,%xmm2
	vaesenc	%xmm15,%xmm3,%xmm3
	vaesenc	%xmm15,%xmm4,%xmm4
	vaesenc	%xmm15,%xmm5,%xmm5
	vaesenc	%xmm15,%xmm6,%xmm6
	vaesenc	%xmm15,%xmm7,%xmm7
	vaesenc	%xmm15,%xmm8,%xmm8

	vmovdqu	208(%rcx),%xmm15
	vaesenc	%xmm15,%xmm1,%xmm1
	vaesenc	%xmm15,%xmm2,%xmm2
	vaesenc	%xmm15,%xmm3,%xmm3
	vaesenc	%xmm15,%xmm4,%xmm4
	vaesenc	%xmm15,%xmm5,%xmm5
	vaesenc	%xmm15,%xmm6,%xmm6
	vaesenc	%xmm15,%xmm7,%xmm7
	vaesenc	%xmm15,%xmm8,%xmm8

	vmovdqu	224(%rcx),%xmm15
	vaesenclast	%xmm15,%xmm1,%xmm1
	vaesenclast	%xmm15,%xmm2,%xmm2
	vaesenclast	%xmm15,%xmm3,%xmm3
	vaesenclast	%xmm15,%xmm4,%xmm4
	vaesenclast	%xmm15,%xmm5,%xmm5
	vaesenclast	%xmm15,%xmm6,%xmm6
	vaesenclast	%xmm15,%xmm7,%xmm7
	vaesenclast	%xmm15,%xmm8,%xmm8



	vpxor	0(%rdi),%xmm1,%xmm1
	vpxor	16(%rdi),%xmm2,%xmm2
	vpxor	32(%rdi),%xmm3,%xmm3
	vpxor	48(%rdi),%xmm4,%xmm4
	vpxor	64(%rdi),%xmm5,%xmm5
	vpxor	80(%rdi),%xmm6,%xmm6
	vpxor	96(%rdi),%xmm7,%xmm7
	vpxor	112(%rdi),%xmm8,%xmm8

	subq	$1,%r8

	vmovdqu	%xmm1,0(%rsi)
	vmovdqu	%xmm2,16(%rsi)
	vmovdqu	%xmm3,32(%rsi)
	vmovdqu	%xmm4,48(%rsi)
	vmovdqu	%xmm5,64(%rsi)
	vmovdqu	%xmm6,80(%rsi)
	vmovdqu	%xmm7,96(%rsi)
	vmovdqu	%xmm8,112(%rsi)

	jne	L$256_enc_msg_x8_loop1

	addq	$128,%rsi
	addq	$128,%rdi

L$256_enc_msg_x8_check_remainder:
	cmpq	$0,%r10
	je	L$256_enc_msg_x8_out

L$256_enc_msg_x8_loop2:


	vmovdqa	%xmm0,%xmm1
	vpaddd	one(%rip),%xmm0,%xmm0

	vpxor	(%rcx),%xmm1,%xmm1
	vaesenc	16(%rcx),%xmm1,%xmm1
	vaesenc	32(%rcx),%xmm1,%xmm1
	vaesenc	48(%rcx),%xmm1,%xmm1
	vaesenc	64(%rcx),%xmm1,%xmm1
	vaesenc	80(%rcx),%xmm1,%xmm1
	vaesenc	96(%rcx),%xmm1,%xmm1
	vaesenc	112(%rcx),%xmm1,%xmm1
	vaesenc	128(%rcx),%xmm1,%xmm1
	vaesenc	144(%rcx),%xmm1,%xmm1
	vaesenc	160(%rcx),%xmm1,%xmm1
	vaesenc	176(%rcx),%xmm1,%xmm1
	vaesenc	192(%rcx),%xmm1,%xmm1
	vaesenc	208(%rcx),%xmm1,%xmm1
	vaesenclast	224(%rcx),%xmm1,%xmm1


	vpxor	(%rdi),%xmm1,%xmm1

	vmovdqu	%xmm1,(%rsi)

	addq	$16,%rdi
	addq	$16,%rsi
	subq	$1,%r10
	jnz	L$256_enc_msg_x8_loop2

L$256_enc_msg_x8_out:
	.byte	0xf3,0xc3



.globl	_aes256gcmsiv_dec
.private_extern _aes256gcmsiv_dec

.p2align	4
_aes256gcmsiv_dec:

_CET_ENDBR
	testq	$~15,%r9
	jnz	L$256_dec_start
	.byte	0xf3,0xc3

L$256_dec_start:
	vzeroupper
	vmovdqa	(%rdx),%xmm0


	vmovdqu	16(%rdx),%xmm15
	vpor	OR_MASK(%rip),%xmm15,%xmm15
	movq	%rdx,%rax

	leaq	32(%rax),%rax
	leaq	32(%rcx),%rcx

	andq	$~15,%r9


	cmpq	$96,%r9
	jb	L$256_dec_loop2


	subq	$96,%r9
	vmovdqa	%xmm15,%xmm7
	vpaddd	one(%rip),%xmm7,%xmm8
	vpaddd	two(%rip),%xmm7,%xmm9
	vpaddd	one(%rip),%xmm9,%xmm10
	vpaddd	two(%rip),%xmm9,%xmm11
	vpaddd	one(%rip),%xmm11,%xmm12
	vpaddd	two(%rip),%xmm11,%xmm15

	vpxor	(%r8),%xmm7,%xmm7
	vpxor	(%r8),%xmm8,%xmm8
	vpxor	(%r8),%xmm9,%xmm9
	vpxor	(%r8),%xmm10,%xmm10
	vpxor	(%r8),%xmm11,%xmm11
	vpxor	(%r8),%xmm12,%xmm12

	vmovdqu	16(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	32(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	48(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	64(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	80(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	96(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	112(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	128(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	144(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	160(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	176(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	192(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	208(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	224(%r8),%xmm4
	vaesenclast	%xmm4,%xmm7,%xmm7
	vaesenclast	%xmm4,%xmm8,%xmm8
	vaesenclast	%xmm4,%xmm9,%xmm9
	vaesenclast	%xmm4,%xmm10,%xmm10
	vaesenclast	%xmm4,%xmm11,%xmm11
	vaesenclast	%xmm4,%xmm12,%xmm12


	vpxor	0(%rdi),%xmm7,%xmm7
	vpxor	16(%rdi),%xmm8,%xmm8
	vpxor	32(%rdi),%xmm9,%xmm9
	vpxor	48(%rdi),%xmm10,%xmm10
	vpxor	64(%rdi),%xmm11,%xmm11
	vpxor	80(%rdi),%xmm12,%xmm12

	vmovdqu	%xmm7,0(%rsi)
	vmovdqu	%xmm8,16(%rsi)
	vmovdqu	%xmm9,32(%rsi)
	vmovdqu	%xmm10,48(%rsi)
	vmovdqu	%xmm11,64(%rsi)
	vmovdqu	%xmm12,80(%rsi)

	addq	$96,%rdi
	addq	$96,%rsi
	jmp	L$256_dec_loop1


.p2align	6
L$256_dec_loop1:
	cmpq	$96,%r9
	jb	L$256_dec_finish_96
	subq	$96,%r9

	vmovdqa	%xmm12,%xmm6
	vmovdqa	%xmm11,16-32(%rax)
	vmovdqa	%xmm10,32-32(%rax)
	vmovdqa	%xmm9,48-32(%rax)
	vmovdqa	%xmm8,64-32(%rax)
	vmovdqa	%xmm7,80-32(%rax)

	vmovdqa	%xmm15,%xmm7
	vpaddd	one(%rip),%xmm7,%xmm8
	vpaddd	two(%rip),%xmm7,%xmm9
	vpaddd	one(%rip),%xmm9,%xmm10
	vpaddd	two(%rip),%xmm9,%xmm11
	vpaddd	one(%rip),%xmm11,%xmm12
	vpaddd	two(%rip),%xmm11,%xmm15

	vmovdqa	(%r8),%xmm4
	vpxor	%xmm4,%xmm7,%xmm7
	vpxor	%xmm4,%xmm8,%xmm8
	vpxor	%xmm4,%xmm9,%xmm9
	vpxor	%xmm4,%xmm10,%xmm10
	vpxor	%xmm4,%xmm11,%xmm11
	vpxor	%xmm4,%xmm12,%xmm12

	vmovdqu	0-32(%rcx),%xmm4
	vpclmulqdq	$0x11,%xmm4,%xmm6,%xmm2
	vpclmulqdq	$0x00,%xmm4,%xmm6,%xmm3
	vpclmulqdq	$0x01,%xmm4,%xmm6,%xmm1
	vpclmulqdq	$0x10,%xmm4,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1

	vmovdqu	16(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	-16(%rax),%xmm6
	vmovdqu	-16(%rcx),%xmm13

	vpclmulqdq	$0x10,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpclmulqdq	$0x11,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm2,%xmm2
	vpclmulqdq	$0x00,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpclmulqdq	$0x01,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1


	vmovdqu	32(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	0(%rax),%xmm6
	vmovdqu	0(%rcx),%xmm13

	vpclmulqdq	$0x10,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpclmulqdq	$0x11,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm2,%xmm2
	vpclmulqdq	$0x00,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpclmulqdq	$0x01,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1


	vmovdqu	48(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	16(%rax),%xmm6
	vmovdqu	16(%rcx),%xmm13

	vpclmulqdq	$0x10,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpclmulqdq	$0x11,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm2,%xmm2
	vpclmulqdq	$0x00,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpclmulqdq	$0x01,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1


	vmovdqu	64(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	32(%rax),%xmm6
	vmovdqu	32(%rcx),%xmm13

	vpclmulqdq	$0x10,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpclmulqdq	$0x11,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm2,%xmm2
	vpclmulqdq	$0x00,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpclmulqdq	$0x01,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1


	vmovdqu	80(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	96(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	112(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12


	vmovdqa	80-32(%rax),%xmm6
	vpxor	%xmm0,%xmm6,%xmm6
	vmovdqu	80-32(%rcx),%xmm5

	vpclmulqdq	$0x01,%xmm5,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpclmulqdq	$0x11,%xmm5,%xmm6,%xmm4
	vpxor	%xmm4,%xmm2,%xmm2
	vpclmulqdq	$0x00,%xmm5,%xmm6,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpclmulqdq	$0x10,%xmm5,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1

	vmovdqu	128(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12


	vpsrldq	$8,%xmm1,%xmm4
	vpxor	%xmm4,%xmm2,%xmm5
	vpslldq	$8,%xmm1,%xmm4
	vpxor	%xmm4,%xmm3,%xmm0

	vmovdqa	poly(%rip),%xmm3

	vmovdqu	144(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	160(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	176(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	192(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	208(%r8),%xmm4
	vaesenc	%xmm4,%xmm7,%xmm7
	vaesenc	%xmm4,%xmm8,%xmm8
	vaesenc	%xmm4,%xmm9,%xmm9
	vaesenc	%xmm4,%xmm10,%xmm10
	vaesenc	%xmm4,%xmm11,%xmm11
	vaesenc	%xmm4,%xmm12,%xmm12

	vmovdqu	224(%r8),%xmm6
	vpalignr	$8,%xmm0,%xmm0,%xmm2
	vpclmulqdq	$0x10,%xmm3,%xmm0,%xmm0
	vpxor	%xmm0,%xmm2,%xmm0

	vpxor	0(%rdi),%xmm6,%xmm4
	vaesenclast	%xmm4,%xmm7,%xmm7
	vpxor	16(%rdi),%xmm6,%xmm4
	vaesenclast	%xmm4,%xmm8,%xmm8
	vpxor	32(%rdi),%xmm6,%xmm4
	vaesenclast	%xmm4,%xmm9,%xmm9
	vpxor	48(%rdi),%xmm6,%xmm4
	vaesenclast	%xmm4,%xmm10,%xmm10
	vpxor	64(%rdi),%xmm6,%xmm4
	vaesenclast	%xmm4,%xmm11,%xmm11
	vpxor	80(%rdi),%xmm6,%xmm4
	vaesenclast	%xmm4,%xmm12,%xmm12

	vpalignr	$8,%xmm0,%xmm0,%xmm2
	vpclmulqdq	$0x10,%xmm3,%xmm0,%xmm0
	vpxor	%xmm0,%xmm2,%xmm0

	vmovdqu	%xmm7,0(%rsi)
	vmovdqu	%xmm8,16(%rsi)
	vmovdqu	%xmm9,32(%rsi)
	vmovdqu	%xmm10,48(%rsi)
	vmovdqu	%xmm11,64(%rsi)
	vmovdqu	%xmm12,80(%rsi)

	vpxor	%xmm5,%xmm0,%xmm0

	leaq	96(%rdi),%rdi
	leaq	96(%rsi),%rsi
	jmp	L$256_dec_loop1

L$256_dec_finish_96:
	vmovdqa	%xmm12,%xmm6
	vmovdqa	%xmm11,16-32(%rax)
	vmovdqa	%xmm10,32-32(%rax)
	vmovdqa	%xmm9,48-32(%rax)
	vmovdqa	%xmm8,64-32(%rax)
	vmovdqa	%xmm7,80-32(%rax)

	vmovdqu	0-32(%rcx),%xmm4
	vpclmulqdq	$0x10,%xmm4,%xmm6,%xmm1
	vpclmulqdq	$0x11,%xmm4,%xmm6,%xmm2
	vpclmulqdq	$0x00,%xmm4,%xmm6,%xmm3
	vpclmulqdq	$0x01,%xmm4,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1

	vmovdqu	-16(%rax),%xmm6
	vmovdqu	-16(%rcx),%xmm13

	vpclmulqdq	$0x10,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpclmulqdq	$0x11,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm2,%xmm2
	vpclmulqdq	$0x00,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpclmulqdq	$0x01,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1

	vmovdqu	0(%rax),%xmm6
	vmovdqu	0(%rcx),%xmm13

	vpclmulqdq	$0x10,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpclmulqdq	$0x11,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm2,%xmm2
	vpclmulqdq	$0x00,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpclmulqdq	$0x01,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1

	vmovdqu	16(%rax),%xmm6
	vmovdqu	16(%rcx),%xmm13

	vpclmulqdq	$0x10,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpclmulqdq	$0x11,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm2,%xmm2
	vpclmulqdq	$0x00,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpclmulqdq	$0x01,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1

	vmovdqu	32(%rax),%xmm6
	vmovdqu	32(%rcx),%xmm13

	vpclmulqdq	$0x10,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpclmulqdq	$0x11,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm2,%xmm2
	vpclmulqdq	$0x00,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpclmulqdq	$0x01,%xmm13,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1


	vmovdqu	80-32(%rax),%xmm6
	vpxor	%xmm0,%xmm6,%xmm6
	vmovdqu	80-32(%rcx),%xmm5
	vpclmulqdq	$0x11,%xmm5,%xmm6,%xmm4
	vpxor	%xmm4,%xmm2,%xmm2
	vpclmulqdq	$0x00,%xmm5,%xmm6,%xmm4
	vpxor	%xmm4,%xmm3,%xmm3
	vpclmulqdq	$0x10,%xmm5,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1
	vpclmulqdq	$0x01,%xmm5,%xmm6,%xmm4
	vpxor	%xmm4,%xmm1,%xmm1

	vpsrldq	$8,%xmm1,%xmm4
	vpxor	%xmm4,%xmm2,%xmm5
	vpslldq	$8,%xmm1,%xmm4
	vpxor	%xmm4,%xmm3,%xmm0

	vmovdqa	poly(%rip),%xmm3

	vpalignr	$8,%xmm0,%xmm0,%xmm2
	vpclmulqdq	$0x10,%xmm3,%xmm0,%xmm0
	vpxor	%xmm0,%xmm2,%xmm0

	vpalignr	$8,%xmm0,%xmm0,%xmm2
	vpclmulqdq	$0x10,%xmm3,%xmm0,%xmm0
	vpxor	%xmm0,%xmm2,%xmm0

	vpxor	%xmm5,%xmm0,%xmm0

L$256_dec_loop2:



	cmpq	$16,%r9
	jb	L$256_dec_out
	subq	$16,%r9

	vmovdqa	%xmm15,%xmm2
	vpaddd	one(%rip),%xmm15,%xmm15

	vpxor	0(%r8),%xmm2,%xmm2
	vaesenc	16(%r8),%xmm2,%xmm2
	vaesenc	32(%r8),%xmm2,%xmm2
	vaesenc	48(%r8),%xmm2,%xmm2
	vaesenc	64(%r8),%xmm2,%xmm2
	vaesenc	80(%r8),%xmm2,%xmm2
	vaesenc	96(%r8),%xmm2,%xmm2
	vaesenc	112(%r8),%xmm2,%xmm2
	vaesenc	128(%r8),%xmm2,%xmm2
	vaesenc	144(%r8),%xmm2,%xmm2
	vaesenc	160(%r8),%xmm2,%xmm2
	vaesenc	176(%r8),%xmm2,%xmm2
	vaesenc	192(%r8),%xmm2,%xmm2
	vaesenc	208(%r8),%xmm2,%xmm2
	vaesenclast	224(%r8),%xmm2,%xmm2
	vpxor	(%rdi),%xmm2,%xmm2
	vmovdqu	%xmm2,(%rsi)
	addq	$16,%rdi
	addq	$16,%rsi

	vpxor	%xmm2,%xmm0,%xmm0
	vmovdqa	-32(%rcx),%xmm1
	call	GFMUL

	jmp	L$256_dec_loop2

L$256_dec_out:
	vmovdqu	%xmm0,(%rdx)
	.byte	0xf3,0xc3


.globl	_aes256gcmsiv_kdf
.private_extern _aes256gcmsiv_kdf

.p2align	4
_aes256gcmsiv_kdf:

_CET_ENDBR




	vmovdqa	(%rdx),%xmm1
	vmovdqa	0(%rdi),%xmm4
	vmovdqa	and_mask(%rip),%xmm11
	vmovdqa	one(%rip),%xmm8
	vpshufd	$0x90,%xmm4,%xmm4
	vpand	%xmm11,%xmm4,%xmm4
	vpaddd	%xmm8,%xmm4,%xmm6
	vpaddd	%xmm8,%xmm6,%xmm7
	vpaddd	%xmm8,%xmm7,%xmm11
	vpaddd	%xmm8,%xmm11,%xmm12
	vpaddd	%xmm8,%xmm12,%xmm13

	vpxor	%xmm1,%xmm4,%xmm4
	vpxor	%xmm1,%xmm6,%xmm6
	vpxor	%xmm1,%xmm7,%xmm7
	vpxor	%xmm1,%xmm11,%xmm11
	vpxor	%xmm1,%xmm12,%xmm12
	vpxor	%xmm1,%xmm13,%xmm13

	vmovdqa	16(%rdx),%xmm1
	vaesenc	%xmm1,%xmm4,%xmm4
	vaesenc	%xmm1,%xmm6,%xmm6
	vaesenc	%xmm1,%xmm7,%xmm7
	vaesenc	%xmm1,%xmm11,%xmm11
	vaesenc	%xmm1,%xmm12,%xmm12
	vaesenc	%xmm1,%xmm13,%xmm13

	vmovdqa	32(%rdx),%xmm2
	vaesenc	%xmm2,%xmm4,%xmm4
	vaesenc	%xmm2,%xmm6,%xmm6
	vaesenc	%xmm2,%xmm7,%xmm7
	vaesenc	%xmm2,%xmm11,%xmm11
	vaesenc	%xmm2,%xmm12,%xmm12
	vaesenc	%xmm2,%xmm13,%xmm13

	vmovdqa	48(%rdx),%xmm1
	vaesenc	%xmm1,%xmm4,%xmm4
	vaesenc	%xmm1,%xmm6,%xmm6
	vaesenc	%xmm1,%xmm7,%xmm7
	vaesenc	%xmm1,%xmm11,%xmm11
	vaesenc	%xmm1,%xmm12,%xmm12
	vaesenc	%xmm1,%xmm13,%xmm13

	vmovdqa	64(%rdx),%xmm2
	vaesenc	%xmm2,%xmm4,%xmm4
	vaesenc	%xmm2,%xmm6,%xmm6
	vaesenc	%xmm2,%xmm7,%xmm7
	vaesenc	%xmm2,%xmm11,%xmm11
	vaesenc	%xmm2,%xmm12,%xmm12
	vaesenc	%xmm2,%xmm13,%xmm13

	vmovdqa	80(%rdx),%xmm1
	vaesenc	%xmm1,%xmm4,%xmm4
	vaesenc	%xmm1,%xmm6,%xmm6
	vaesenc	%xmm1,%xmm7,%xmm7
	vaesenc	%xmm1,%xmm11,%xmm11
	vaesenc	%xmm1,%xmm12,%xmm12
	vaesenc	%xmm1,%xmm13,%xmm13

	vmovdqa	96(%rdx),%xmm2
	vaesenc	%xmm2,%xmm4,%xmm4
	vaesenc	%xmm2,%xmm6,%xmm6
	vaesenc	%xmm2,%xmm7,%xmm7
	vaesenc	%xmm2,%xmm11,%xmm11
	vaesenc	%xmm2,%xmm12,%xmm12
	vaesenc	%xmm2,%xmm13,%xmm13

	vmovdqa	112(%rdx),%xmm1
	vaesenc	%xmm1,%xmm4,%xmm4
	vaesenc	%xmm1,%xmm6,%xmm6
	vaesenc	%xmm1,%xmm7,%xmm7
	vaesenc	%xmm1,%xmm11,%xmm11
	vaesenc	%xmm1,%xmm12,%xmm12
	vaesenc	%xmm1,%xmm13,%xmm13

	vmovdqa	128(%rdx),%xmm2
	vaesenc	%xmm2,%xmm4,%xmm4
	vaesenc	%xmm2,%xmm6,%xmm6
	vaesenc	%xmm2,%xmm7,%xmm7
	vaesenc	%xmm2,%xmm11,%xmm11
	vaesenc	%xmm2,%xmm12,%xmm12
	vaesenc	%xmm2,%xmm13,%xmm13

	vmovdqa	144(%rdx),%xmm1
	vaesenc	%xmm1,%xmm4,%xmm4
	vaesenc	%xmm1,%xmm6,%xmm6
	vaesenc	%xmm1,%xmm7,%xmm7
	vaesenc	%xmm1,%xmm11,%xmm11
	vaesenc	%xmm1,%xmm12,%xmm12
	vaesenc	%xmm1,%xmm13,%xmm13

	vmovdqa	160(%rdx),%xmm2
	vaesenc	%xmm2,%xmm4,%xmm4
	vaesenc	%xmm2,%xmm6,%xmm6
	vaesenc	%xmm2,%xmm7,%xmm7
	vaesenc	%xmm2,%xmm11,%xmm11
	vaesenc	%xmm2,%xmm12,%xmm12
	vaesenc	%xmm2,%xmm13,%xmm13

	vmovdqa	176(%rdx),%xmm1
	vaesenc	%xmm1,%xmm4,%xmm4
	vaesenc	%xmm1,%xmm6,%xmm6
	vaesenc	%xmm1,%xmm7,%xmm7
	vaesenc	%xmm1,%xmm11,%xmm11
	vaesenc	%xmm1,%xmm12,%xmm12
	vaesenc	%xmm1,%xmm13,%xmm13

	vmovdqa	192(%rdx),%xmm2
	vaesenc	%xmm2,%xmm4,%xmm4
	vaesenc	%xmm2,%xmm6,%xmm6
	vaesenc	%xmm2,%xmm7,%xmm7
	vaesenc	%xmm2,%xmm11,%xmm11
	vaesenc	%xmm2,%xmm12,%xmm12
	vaesenc	%xmm2,%xmm13,%xmm13

	vmovdqa	208(%rdx),%xmm1
	vaesenc	%xmm1,%xmm4,%xmm4
	vaesenc	%xmm1,%xmm6,%xmm6
	vaesenc	%xmm1,%xmm7,%xmm7
	vaesenc	%xmm1,%xmm11,%xmm11
	vaesenc	%xmm1,%xmm12,%xmm12
	vaesenc	%xmm1,%xmm13,%xmm13

	vmovdqa	224(%rdx),%xmm2
	vaesenclast	%xmm2,%xmm4,%xmm4
	vaesenclast	%xmm2,%xmm6,%xmm6
	vaesenclast	%xmm2,%xmm7,%xmm7
	vaesenclast	%xmm2,%xmm11,%xmm11
	vaesenclast	%xmm2,%xmm12,%xmm12
	vaesenclast	%xmm2,%xmm13,%xmm13


	vmovdqa	%xmm4,0(%rsi)
	vmovdqa	%xmm6,16(%rsi)
	vmovdqa	%xmm7,32(%rsi)
	vmovdqa	%xmm11,48(%rsi)
	vmovdqa	%xmm12,64(%rsi)
	vmovdqa	%xmm13,80(%rsi)
	.byte	0xf3,0xc3


#endif
