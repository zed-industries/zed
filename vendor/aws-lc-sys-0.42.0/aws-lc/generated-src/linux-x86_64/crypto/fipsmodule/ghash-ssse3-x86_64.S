// This file is generated from a similarly-named Perl script in the BoringSSL
// source tree. Do not edit by hand.

#include <openssl/asm_base.h>

#if !defined(OPENSSL_NO_ASM) && defined(OPENSSL_X86_64) && defined(__ELF__)
.text	





.type	gcm_gmult_ssse3, @function
.globl	gcm_gmult_ssse3
.hidden gcm_gmult_ssse3
.align	16
gcm_gmult_ssse3:
.cfi_startproc	

_CET_ENDBR
	movdqu	(%rdi),%xmm0
	movdqa	.Lreverse_bytes(%rip),%xmm10
	movdqa	.Llow4_mask(%rip),%xmm2


.byte	102,65,15,56,0,194


	movdqa	%xmm2,%xmm1
	pandn	%xmm0,%xmm1
	psrld	$4,%xmm1
	pand	%xmm2,%xmm0




	pxor	%xmm2,%xmm2
	pxor	%xmm3,%xmm3
	movq	$5,%rax
.Loop_row_1:
	movdqa	(%rsi),%xmm4
	leaq	16(%rsi),%rsi


	movdqa	%xmm2,%xmm6
.byte	102,15,58,15,243,1
	movdqa	%xmm6,%xmm3
	psrldq	$1,%xmm2




	movdqa	%xmm4,%xmm5
.byte	102,15,56,0,224
.byte	102,15,56,0,233


	pxor	%xmm5,%xmm2



	movdqa	%xmm4,%xmm5
	psllq	$60,%xmm5
	movdqa	%xmm5,%xmm6
	pslldq	$8,%xmm6
	pxor	%xmm6,%xmm3


	psrldq	$8,%xmm5
	pxor	%xmm5,%xmm2
	psrlq	$4,%xmm4
	pxor	%xmm4,%xmm2

	subq	$1,%rax
	jnz	.Loop_row_1



	pxor	%xmm3,%xmm2
	psrlq	$1,%xmm3
	pxor	%xmm3,%xmm2
	psrlq	$1,%xmm3
	pxor	%xmm3,%xmm2
	psrlq	$5,%xmm3
	pxor	%xmm3,%xmm2
	pxor	%xmm3,%xmm3
	movq	$5,%rax
.Loop_row_2:
	movdqa	(%rsi),%xmm4
	leaq	16(%rsi),%rsi


	movdqa	%xmm2,%xmm6
.byte	102,15,58,15,243,1
	movdqa	%xmm6,%xmm3
	psrldq	$1,%xmm2




	movdqa	%xmm4,%xmm5
.byte	102,15,56,0,224
.byte	102,15,56,0,233


	pxor	%xmm5,%xmm2



	movdqa	%xmm4,%xmm5
	psllq	$60,%xmm5
	movdqa	%xmm5,%xmm6
	pslldq	$8,%xmm6
	pxor	%xmm6,%xmm3


	psrldq	$8,%xmm5
	pxor	%xmm5,%xmm2
	psrlq	$4,%xmm4
	pxor	%xmm4,%xmm2

	subq	$1,%rax
	jnz	.Loop_row_2



	pxor	%xmm3,%xmm2
	psrlq	$1,%xmm3
	pxor	%xmm3,%xmm2
	psrlq	$1,%xmm3
	pxor	%xmm3,%xmm2
	psrlq	$5,%xmm3
	pxor	%xmm3,%xmm2
	pxor	%xmm3,%xmm3
	movq	$6,%rax
.Loop_row_3:
	movdqa	(%rsi),%xmm4
	leaq	16(%rsi),%rsi


	movdqa	%xmm2,%xmm6
.byte	102,15,58,15,243,1
	movdqa	%xmm6,%xmm3
	psrldq	$1,%xmm2




	movdqa	%xmm4,%xmm5
.byte	102,15,56,0,224
.byte	102,15,56,0,233


	pxor	%xmm5,%xmm2



	movdqa	%xmm4,%xmm5
	psllq	$60,%xmm5
	movdqa	%xmm5,%xmm6
	pslldq	$8,%xmm6
	pxor	%xmm6,%xmm3


	psrldq	$8,%xmm5
	pxor	%xmm5,%xmm2
	psrlq	$4,%xmm4
	pxor	%xmm4,%xmm2

	subq	$1,%rax
	jnz	.Loop_row_3



	pxor	%xmm3,%xmm2
	psrlq	$1,%xmm3
	pxor	%xmm3,%xmm2
	psrlq	$1,%xmm3
	pxor	%xmm3,%xmm2
	psrlq	$5,%xmm3
	pxor	%xmm3,%xmm2
	pxor	%xmm3,%xmm3

.byte	102,65,15,56,0,210
	movdqu	%xmm2,(%rdi)


	pxor	%xmm0,%xmm0
	pxor	%xmm1,%xmm1
	pxor	%xmm2,%xmm2
	pxor	%xmm3,%xmm3
	pxor	%xmm4,%xmm4
	pxor	%xmm5,%xmm5
	pxor	%xmm6,%xmm6
	.byte	0xf3,0xc3
.cfi_endproc	

.size	gcm_gmult_ssse3,.-gcm_gmult_ssse3





.type	gcm_ghash_ssse3, @function
.globl	gcm_ghash_ssse3
.hidden gcm_ghash_ssse3
.align	16
gcm_ghash_ssse3:
.cfi_startproc	

_CET_ENDBR
	movdqu	(%rdi),%xmm0
	movdqa	.Lreverse_bytes(%rip),%xmm10
	movdqa	.Llow4_mask(%rip),%xmm11


	andq	$-16,%rcx



.byte	102,65,15,56,0,194


	pxor	%xmm3,%xmm3
.Loop_ghash:

	movdqu	(%rdx),%xmm1
.byte	102,65,15,56,0,202
	pxor	%xmm1,%xmm0


	movdqa	%xmm11,%xmm1
	pandn	%xmm0,%xmm1
	psrld	$4,%xmm1
	pand	%xmm11,%xmm0




	pxor	%xmm2,%xmm2

	movq	$5,%rax
.Loop_row_4:
	movdqa	(%rsi),%xmm4
	leaq	16(%rsi),%rsi


	movdqa	%xmm2,%xmm6
.byte	102,15,58,15,243,1
	movdqa	%xmm6,%xmm3
	psrldq	$1,%xmm2




	movdqa	%xmm4,%xmm5
.byte	102,15,56,0,224
.byte	102,15,56,0,233


	pxor	%xmm5,%xmm2



	movdqa	%xmm4,%xmm5
	psllq	$60,%xmm5
	movdqa	%xmm5,%xmm6
	pslldq	$8,%xmm6
	pxor	%xmm6,%xmm3


	psrldq	$8,%xmm5
	pxor	%xmm5,%xmm2
	psrlq	$4,%xmm4
	pxor	%xmm4,%xmm2

	subq	$1,%rax
	jnz	.Loop_row_4



	pxor	%xmm3,%xmm2
	psrlq	$1,%xmm3
	pxor	%xmm3,%xmm2
	psrlq	$1,%xmm3
	pxor	%xmm3,%xmm2
	psrlq	$5,%xmm3
	pxor	%xmm3,%xmm2
	pxor	%xmm3,%xmm3
	movq	$5,%rax
.Loop_row_5:
	movdqa	(%rsi),%xmm4
	leaq	16(%rsi),%rsi


	movdqa	%xmm2,%xmm6
.byte	102,15,58,15,243,1
	movdqa	%xmm6,%xmm3
	psrldq	$1,%xmm2




	movdqa	%xmm4,%xmm5
.byte	102,15,56,0,224
.byte	102,15,56,0,233


	pxor	%xmm5,%xmm2



	movdqa	%xmm4,%xmm5
	psllq	$60,%xmm5
	movdqa	%xmm5,%xmm6
	pslldq	$8,%xmm6
	pxor	%xmm6,%xmm3


	psrldq	$8,%xmm5
	pxor	%xmm5,%xmm2
	psrlq	$4,%xmm4
	pxor	%xmm4,%xmm2

	subq	$1,%rax
	jnz	.Loop_row_5



	pxor	%xmm3,%xmm2
	psrlq	$1,%xmm3
	pxor	%xmm3,%xmm2
	psrlq	$1,%xmm3
	pxor	%xmm3,%xmm2
	psrlq	$5,%xmm3
	pxor	%xmm3,%xmm2
	pxor	%xmm3,%xmm3
	movq	$6,%rax
.Loop_row_6:
	movdqa	(%rsi),%xmm4
	leaq	16(%rsi),%rsi


	movdqa	%xmm2,%xmm6
.byte	102,15,58,15,243,1
	movdqa	%xmm6,%xmm3
	psrldq	$1,%xmm2




	movdqa	%xmm4,%xmm5
.byte	102,15,56,0,224
.byte	102,15,56,0,233


	pxor	%xmm5,%xmm2



	movdqa	%xmm4,%xmm5
	psllq	$60,%xmm5
	movdqa	%xmm5,%xmm6
	pslldq	$8,%xmm6
	pxor	%xmm6,%xmm3


	psrldq	$8,%xmm5
	pxor	%xmm5,%xmm2
	psrlq	$4,%xmm4
	pxor	%xmm4,%xmm2

	subq	$1,%rax
	jnz	.Loop_row_6



	pxor	%xmm3,%xmm2
	psrlq	$1,%xmm3
	pxor	%xmm3,%xmm2
	psrlq	$1,%xmm3
	pxor	%xmm3,%xmm2
	psrlq	$5,%xmm3
	pxor	%xmm3,%xmm2
	pxor	%xmm3,%xmm3
	movdqa	%xmm2,%xmm0


	leaq	-256(%rsi),%rsi


	leaq	16(%rdx),%rdx
	subq	$16,%rcx
	jnz	.Loop_ghash


.byte	102,65,15,56,0,194
	movdqu	%xmm0,(%rdi)


	pxor	%xmm0,%xmm0
	pxor	%xmm1,%xmm1
	pxor	%xmm2,%xmm2
	pxor	%xmm3,%xmm3
	pxor	%xmm4,%xmm4
	pxor	%xmm5,%xmm5
	pxor	%xmm6,%xmm6
	.byte	0xf3,0xc3
.cfi_endproc	

.size	gcm_ghash_ssse3,.-gcm_ghash_ssse3

.section	.rodata
.align	16


.Lreverse_bytes:
.byte	15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0

.Llow4_mask:
.quad	0x0f0f0f0f0f0f0f0f, 0x0f0f0f0f0f0f0f0f
.text	
#endif
