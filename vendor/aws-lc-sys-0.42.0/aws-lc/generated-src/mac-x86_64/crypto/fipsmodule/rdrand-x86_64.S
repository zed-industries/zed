// This file is generated from a similarly-named Perl script in the BoringSSL
// source tree. Do not edit by hand.

#include <openssl/asm_base.h>

#if !defined(OPENSSL_NO_ASM) && defined(OPENSSL_X86_64) && defined(__APPLE__)
.text	


.globl	_CRYPTO_rdrand_multiple8
.private_extern _CRYPTO_rdrand_multiple8

.p2align	4
_CRYPTO_rdrand_multiple8:

_CET_ENDBR
	testq	%rsi,%rsi
	jz	L$out
	movq	$8,%rdx
L$loop:
.byte	72,15,199,241
	jnc	L$err_multiple
	testq	%rcx,%rcx
	jz	L$err_multiple
	cmpq	$-1,%rcx
	je	L$err_multiple
	movq	%rcx,0(%rdi)
	addq	%rdx,%rdi
	subq	%rdx,%rsi
	jnz	L$loop
L$out:
	movq	$1,%rax
	.byte	0xf3,0xc3
L$err_multiple:
	xorq	%rax,%rax
	.byte	0xf3,0xc3


#endif
