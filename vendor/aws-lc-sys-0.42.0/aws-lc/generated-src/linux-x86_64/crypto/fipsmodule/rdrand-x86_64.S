// This file is generated from a similarly-named Perl script in the BoringSSL
// source tree. Do not edit by hand.

#include <openssl/asm_base.h>

#if !defined(OPENSSL_NO_ASM) && defined(OPENSSL_X86_64) && defined(__ELF__)
.text	


.globl	CRYPTO_rdrand_multiple8
.hidden CRYPTO_rdrand_multiple8
.type	CRYPTO_rdrand_multiple8,@function
.align	16
CRYPTO_rdrand_multiple8:
.cfi_startproc	
_CET_ENDBR
	testq	%rsi,%rsi
	jz	.Lout
	movq	$8,%rdx
.Lloop:
.byte	72,15,199,241
	jnc	.Lerr_multiple
	testq	%rcx,%rcx
	jz	.Lerr_multiple
	cmpq	$-1,%rcx
	je	.Lerr_multiple
	movq	%rcx,0(%rdi)
	addq	%rdx,%rdi
	subq	%rdx,%rsi
	jnz	.Lloop
.Lout:
	movq	$1,%rax
	.byte	0xf3,0xc3
.Lerr_multiple:
	xorq	%rax,%rax
	.byte	0xf3,0xc3
.cfi_endproc	
.size	CRYPTO_rdrand_multiple8,.-CRYPTO_rdrand_multiple8
#endif
