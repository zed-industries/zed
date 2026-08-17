// This file is generated from a similarly-named Perl script in the BoringSSL
// source tree. Do not edit by hand.

#include <openssl/asm_base.h>

#if !defined(OPENSSL_NO_ASM) && defined(OPENSSL_X86_64) && defined(__APPLE__)
#ifndef MY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX
.text	
.globl	_aes_hw_xts_encrypt_avx512
.private_extern _aes_hw_xts_encrypt_avx512
.private_extern	_aes_hw_xts_encrypt_avx512

.p2align	5
_aes_hw_xts_encrypt_avx512:

.byte	243,15,30,250
	pushq	%rbp
	movq	%rsp,%rbp
	subq	$376,%rsp
	andq	$0xffffffffffffffc0,%rsp
	movq	%rbx,368(%rsp)
	movq	$0x87,%r10
	vmovdqu	(%r9),%xmm1
	vpxor	%xmm4,%xmm4,%xmm4
	vmovdqu	(%r8),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1

	vmovdqu	(%rcx),%xmm2
	vmovdqa	%xmm2,128(%rsp)

	vmovdqu	16(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	16(%rcx),%xmm2
	vmovdqa	%xmm2,144(%rsp)

	vmovdqu	32(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	32(%rcx),%xmm2
	vmovdqa	%xmm2,160(%rsp)

	vmovdqu	48(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	48(%rcx),%xmm2
	vmovdqa	%xmm2,176(%rsp)

	vmovdqu	64(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	64(%rcx),%xmm2
	vmovdqa	%xmm2,192(%rsp)

	vmovdqu	80(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	80(%rcx),%xmm2
	vmovdqa	%xmm2,208(%rsp)

	vmovdqu	96(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	96(%rcx),%xmm2
	vmovdqa	%xmm2,224(%rsp)

	vmovdqu	112(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	112(%rcx),%xmm2
	vmovdqa	%xmm2,240(%rsp)

	vmovdqu	128(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	128(%rcx),%xmm2
	vmovdqa	%xmm2,256(%rsp)

	vmovdqu	144(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	144(%rcx),%xmm2
	vmovdqa	%xmm2,272(%rsp)

	vmovdqu	160(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	160(%rcx),%xmm2
	vmovdqa	%xmm2,288(%rsp)

	vmovdqu	176(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	176(%rcx),%xmm2
	vmovdqa	%xmm2,304(%rsp)

	vmovdqu	192(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	192(%rcx),%xmm2
	vmovdqa	%xmm2,320(%rsp)

	vmovdqu	208(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	208(%rcx),%xmm2
	vmovdqa	%xmm2,336(%rsp)

	vmovdqu	224(%r8),%xmm0
.byte	98,242,117,8,221,200

	vmovdqu	224(%rcx),%xmm2
	vmovdqa	%xmm2,352(%rsp)

	vmovdqa	%xmm1,(%rsp)

	cmpq	$0x80,%rdx
	jl	L$_less_than_128_bytes_hEgxyDlCngwrfFe
	vpbroadcastq	%r10,%zmm25
	cmpq	$0x100,%rdx
	jge	L$_start_by16_hEgxyDlCngwrfFe
	cmpq	$0x80,%rdx
	jge	L$_start_by8_hEgxyDlCngwrfFe

L$_do_n_blocks_hEgxyDlCngwrfFe:
	cmpq	$0x0,%rdx
	je	L$_ret_hEgxyDlCngwrfFe
	cmpq	$0x70,%rdx
	jge	L$_remaining_num_blocks_is_7_hEgxyDlCngwrfFe
	cmpq	$0x60,%rdx
	jge	L$_remaining_num_blocks_is_6_hEgxyDlCngwrfFe
	cmpq	$0x50,%rdx
	jge	L$_remaining_num_blocks_is_5_hEgxyDlCngwrfFe
	cmpq	$0x40,%rdx
	jge	L$_remaining_num_blocks_is_4_hEgxyDlCngwrfFe
	cmpq	$0x30,%rdx
	jge	L$_remaining_num_blocks_is_3_hEgxyDlCngwrfFe
	cmpq	$0x20,%rdx
	jge	L$_remaining_num_blocks_is_2_hEgxyDlCngwrfFe
	cmpq	$0x10,%rdx
	jge	L$_remaining_num_blocks_is_1_hEgxyDlCngwrfFe
	vmovdqa	%xmm0,%xmm8
	vmovdqa	%xmm9,%xmm0
	jmp	L$_steal_cipher_hEgxyDlCngwrfFe

L$_remaining_num_blocks_is_7_hEgxyDlCngwrfFe:
	movq	$0xffffffffffffffff,%r8
	shrq	$0x10,%r8
	kmovq	%r8,%k1
	vmovdqu8	(%rdi),%zmm1
	vmovdqu8	64(%rdi),%zmm2{%k1}
	addq	$0x70,%rdi

	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vbroadcasti32x4	128(%rsp),%zmm0
	vpxorq	%zmm0,%zmm1,%zmm1
	vpxorq	%zmm0,%zmm2,%zmm2
	vbroadcasti32x4	144(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	160(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	176(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208

	vbroadcasti32x4	192(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	208(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	224(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	240(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	256(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	272(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	288(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	304(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	320(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	336(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	352(%rsp),%zmm0
.byte	98,242,117,72,221,200
.byte	98,242,109,72,221,208


	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vmovdqa32	%zmm15,%zmm9
	vmovdqa32	%zmm16,%zmm10
	vmovdqu8	%zmm1,(%rsi)
	vmovdqu8	%zmm2,64(%rsi){%k1}
	addq	$0x70,%rsi
	vextracti32x4	$0x2,%zmm2,%xmm8
	vextracti32x4	$0x3,%zmm10,%xmm0
	andq	$0xf,%rdx
	je	L$_ret_hEgxyDlCngwrfFe
	jmp	L$_steal_cipher_hEgxyDlCngwrfFe

L$_remaining_num_blocks_is_6_hEgxyDlCngwrfFe:
	vmovdqu8	(%rdi),%zmm1
	vmovdqu8	64(%rdi),%ymm2
	addq	$0x60,%rdi

	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vbroadcasti32x4	128(%rsp),%zmm0
	vpxorq	%zmm0,%zmm1,%zmm1
	vpxorq	%zmm0,%zmm2,%zmm2
	vbroadcasti32x4	144(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	160(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	176(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208

	vbroadcasti32x4	192(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	208(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	224(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	240(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	256(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	272(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	288(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	304(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	320(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	336(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	352(%rsp),%zmm0
.byte	98,242,117,72,221,200
.byte	98,242,109,72,221,208


	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vmovdqa32	%zmm15,%zmm9
	vmovdqa32	%zmm16,%zmm10
	vmovdqu8	%zmm1,(%rsi)
	vmovdqu8	%ymm2,64(%rsi)
	addq	$0x60,%rsi
	vextracti32x4	$0x1,%zmm2,%xmm8
	vextracti32x4	$0x2,%zmm10,%xmm0
	andq	$0xf,%rdx
	je	L$_ret_hEgxyDlCngwrfFe
	jmp	L$_steal_cipher_hEgxyDlCngwrfFe

L$_remaining_num_blocks_is_5_hEgxyDlCngwrfFe:
	vmovdqu8	(%rdi),%zmm1
	vmovdqu	64(%rdi),%xmm2
	addq	$0x50,%rdi

	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vbroadcasti32x4	128(%rsp),%zmm0
	vpxorq	%zmm0,%zmm1,%zmm1
	vpxorq	%zmm0,%zmm2,%zmm2
	vbroadcasti32x4	144(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	160(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	176(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208

	vbroadcasti32x4	192(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	208(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	224(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	240(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	256(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	272(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	288(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	304(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	320(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	336(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	352(%rsp),%zmm0
.byte	98,242,117,72,221,200
.byte	98,242,109,72,221,208


	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vmovdqa32	%zmm15,%zmm9
	vmovdqa32	%zmm16,%zmm10
	vmovdqu8	%zmm1,(%rsi)
	vmovdqu	%xmm2,64(%rsi)
	addq	$0x50,%rsi
	vmovdqa	%xmm2,%xmm8
	vextracti32x4	$0x1,%zmm10,%xmm0
	andq	$0xf,%rdx
	je	L$_ret_hEgxyDlCngwrfFe
	jmp	L$_steal_cipher_hEgxyDlCngwrfFe

L$_remaining_num_blocks_is_4_hEgxyDlCngwrfFe:
	vmovdqu8	(%rdi),%zmm1
	addq	$0x40,%rdi

	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vbroadcasti32x4	128(%rsp),%zmm0
	vpxorq	%zmm0,%zmm1,%zmm1
	vpxorq	%zmm0,%zmm2,%zmm2
	vbroadcasti32x4	144(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	160(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	176(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208

	vbroadcasti32x4	192(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	208(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	224(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	240(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	256(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	272(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	288(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	304(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	320(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	336(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	352(%rsp),%zmm0
.byte	98,242,117,72,221,200
.byte	98,242,109,72,221,208


	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vmovdqa32	%zmm15,%zmm9
	vmovdqa32	%zmm16,%zmm10
	vmovdqu8	%zmm1,(%rsi)
	addq	$0x40,%rsi
	vextracti32x4	$0x3,%zmm1,%xmm8
	vextracti32x4	$0x0,%zmm10,%xmm0
	andq	$0xf,%rdx
	je	L$_ret_hEgxyDlCngwrfFe
	jmp	L$_steal_cipher_hEgxyDlCngwrfFe
L$_remaining_num_blocks_is_3_hEgxyDlCngwrfFe:
	vextracti32x4	$0x1,%zmm9,%xmm10
	vextracti32x4	$0x2,%zmm9,%xmm11
	vmovdqu	(%rdi),%xmm1
	vmovdqu	16(%rdi),%xmm2
	vmovdqu	32(%rdi),%xmm3
	addq	$0x30,%rdi
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vpxor	%xmm0,%xmm2,%xmm2
	vpxor	%xmm0,%xmm3,%xmm3
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,221,200
.byte	98,242,109,8,221,208
.byte	98,242,101,8,221,216
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vmovdqu	%xmm1,(%rsi)
	vmovdqu	%xmm2,16(%rsi)
	vmovdqu	%xmm3,32(%rsi)
	addq	$0x30,%rsi
	vmovdqa	%xmm3,%xmm8
	vextracti32x4	$0x3,%zmm9,%xmm0
	andq	$0xf,%rdx
	je	L$_ret_hEgxyDlCngwrfFe
	jmp	L$_steal_cipher_hEgxyDlCngwrfFe
L$_remaining_num_blocks_is_2_hEgxyDlCngwrfFe:
	vextracti32x4	$0x1,%zmm9,%xmm10
	vmovdqu	(%rdi),%xmm1
	vmovdqu	16(%rdi),%xmm2
	addq	$0x20,%rdi
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vpxor	%xmm0,%xmm2,%xmm2
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,221,200
.byte	98,242,109,8,221,208
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vmovdqu	%xmm1,(%rsi)
	vmovdqu	%xmm2,16(%rsi)
	addq	$0x20,%rsi
	vmovdqa	%xmm2,%xmm8
	vextracti32x4	$0x2,%zmm9,%xmm0
	andq	$0xf,%rdx
	je	L$_ret_hEgxyDlCngwrfFe
	jmp	L$_steal_cipher_hEgxyDlCngwrfFe
L$_remaining_num_blocks_is_1_hEgxyDlCngwrfFe:
	vmovdqu	(%rdi),%xmm1
	addq	$0x10,%rdi
	vpxor	%xmm9,%xmm1,%xmm1
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,221,200
	vpxor	%xmm9,%xmm1,%xmm1
	vmovdqu	%xmm1,(%rsi)
	addq	$0x10,%rsi
	vmovdqa	%xmm1,%xmm8
	vextracti32x4	$0x1,%zmm9,%xmm0
	andq	$0xf,%rdx
	je	L$_ret_hEgxyDlCngwrfFe
	jmp	L$_steal_cipher_hEgxyDlCngwrfFe

L$_start_by16_hEgxyDlCngwrfFe:
	vbroadcasti32x4	(%rsp),%zmm0
	vbroadcasti32x4	shufb_15_7(%rip),%zmm8
	movq	$0xaa,%r8
	kmovq	%r8,%k2
	vpshufb	%zmm8,%zmm0,%zmm1
	vpsllvq	const_dq3210(%rip),%zmm0,%zmm4
	vpsrlvq	const_dq5678(%rip),%zmm1,%zmm2
.byte	98,147,109,72,68,217,0
	vpxorq	%zmm2,%zmm4,%zmm4{%k2}
	vpxord	%zmm4,%zmm3,%zmm9
	vpsllvq	const_dq7654(%rip),%zmm0,%zmm5
	vpsrlvq	const_dq1234(%rip),%zmm1,%zmm6
.byte	98,147,77,72,68,249,0
	vpxorq	%zmm6,%zmm5,%zmm5{%k2}
	vpxord	%zmm5,%zmm7,%zmm10
	vpsrldq	$0xf,%zmm9,%zmm13
.byte	98,19,21,72,68,241,0
	vpslldq	$0x1,%zmm9,%zmm11
	vpxord	%zmm14,%zmm11,%zmm11
	vpsrldq	$0xf,%zmm10,%zmm15
.byte	98,131,5,72,68,193,0
	vpslldq	$0x1,%zmm10,%zmm12
	vpxord	%zmm16,%zmm12,%zmm12

L$_main_loop_run_16_hEgxyDlCngwrfFe:
	vmovdqu8	(%rdi),%zmm1
	vmovdqu8	64(%rdi),%zmm2
	vmovdqu8	128(%rdi),%zmm3
	vmovdqu8	192(%rdi),%zmm4
	addq	$0x100,%rdi
	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2
	vpxorq	%zmm11,%zmm3,%zmm3
	vpxorq	%zmm12,%zmm4,%zmm4
	vbroadcasti32x4	128(%rsp),%zmm0
	vpxorq	%zmm0,%zmm1,%zmm1
	vpxorq	%zmm0,%zmm2,%zmm2
	vpxorq	%zmm0,%zmm3,%zmm3
	vpxorq	%zmm0,%zmm4,%zmm4
	vpsrldq	$0xf,%zmm11,%zmm13
.byte	98,19,21,72,68,241,0
	vpslldq	$0x1,%zmm11,%zmm15
	vpxord	%zmm14,%zmm15,%zmm15
	vbroadcasti32x4	144(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208
.byte	98,242,101,72,220,216
.byte	98,242,93,72,220,224
	vbroadcasti32x4	160(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208
.byte	98,242,101,72,220,216
.byte	98,242,93,72,220,224
	vbroadcasti32x4	176(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208
.byte	98,242,101,72,220,216
.byte	98,242,93,72,220,224
	vpsrldq	$0xf,%zmm12,%zmm13
.byte	98,19,21,72,68,241,0
	vpslldq	$0x1,%zmm12,%zmm16
	vpxord	%zmm14,%zmm16,%zmm16
	vbroadcasti32x4	192(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208
.byte	98,242,101,72,220,216
.byte	98,242,93,72,220,224
	vbroadcasti32x4	208(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208
.byte	98,242,101,72,220,216
.byte	98,242,93,72,220,224
	vbroadcasti32x4	224(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208
.byte	98,242,101,72,220,216
.byte	98,242,93,72,220,224
	vpsrldq	$0xf,%zmm15,%zmm13
.byte	98,19,21,72,68,241,0
	vpslldq	$0x1,%zmm15,%zmm17
	vpxord	%zmm14,%zmm17,%zmm17
	vbroadcasti32x4	240(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208
.byte	98,242,101,72,220,216
.byte	98,242,93,72,220,224
	vbroadcasti32x4	256(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208
.byte	98,242,101,72,220,216
.byte	98,242,93,72,220,224
	vbroadcasti32x4	272(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208
.byte	98,242,101,72,220,216
.byte	98,242,93,72,220,224
	vpsrldq	$0xf,%zmm16,%zmm13
.byte	98,19,21,72,68,241,0
	vpslldq	$0x1,%zmm16,%zmm18
	vpxord	%zmm14,%zmm18,%zmm18
	vbroadcasti32x4	288(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208
.byte	98,242,101,72,220,216
.byte	98,242,93,72,220,224
	vbroadcasti32x4	304(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208
.byte	98,242,101,72,220,216
.byte	98,242,93,72,220,224
	vbroadcasti32x4	320(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208
.byte	98,242,101,72,220,216
.byte	98,242,93,72,220,224
	vbroadcasti32x4	336(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208
.byte	98,242,101,72,220,216
.byte	98,242,93,72,220,224
	vbroadcasti32x4	352(%rsp),%zmm0
.byte	98,242,117,72,221,200
.byte	98,242,109,72,221,208
.byte	98,242,101,72,221,216
.byte	98,242,93,72,221,224
	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2
	vpxorq	%zmm11,%zmm3,%zmm3
	vpxorq	%zmm12,%zmm4,%zmm4

	vmovdqa32	%zmm15,%zmm9
	vmovdqa32	%zmm16,%zmm10
	vmovdqa32	%zmm17,%zmm11
	vmovdqa32	%zmm18,%zmm12
	vmovdqu8	%zmm1,(%rsi)
	vmovdqu8	%zmm2,64(%rsi)
	vmovdqu8	%zmm3,128(%rsi)
	vmovdqu8	%zmm4,192(%rsi)
	addq	$0x100,%rsi
	subq	$0x100,%rdx
	cmpq	$0x100,%rdx
	jge	L$_main_loop_run_16_hEgxyDlCngwrfFe
	cmpq	$0x80,%rdx
	jge	L$_main_loop_run_8_hEgxyDlCngwrfFe
	vextracti32x4	$0x3,%zmm4,%xmm0
	jmp	L$_do_n_blocks_hEgxyDlCngwrfFe

L$_start_by8_hEgxyDlCngwrfFe:
	vbroadcasti32x4	(%rsp),%zmm0
	vbroadcasti32x4	shufb_15_7(%rip),%zmm8
	movq	$0xaa,%r8
	kmovq	%r8,%k2
	vpshufb	%zmm8,%zmm0,%zmm1
	vpsllvq	const_dq3210(%rip),%zmm0,%zmm4
	vpsrlvq	const_dq5678(%rip),%zmm1,%zmm2
.byte	98,147,109,72,68,217,0
	vpxorq	%zmm2,%zmm4,%zmm4{%k2}
	vpxord	%zmm4,%zmm3,%zmm9
	vpsllvq	const_dq7654(%rip),%zmm0,%zmm5
	vpsrlvq	const_dq1234(%rip),%zmm1,%zmm6
.byte	98,147,77,72,68,249,0
	vpxorq	%zmm6,%zmm5,%zmm5{%k2}
	vpxord	%zmm5,%zmm7,%zmm10

L$_main_loop_run_8_hEgxyDlCngwrfFe:
	vmovdqu8	(%rdi),%zmm1
	vmovdqu8	64(%rdi),%zmm2
	addq	$0x80,%rdi

	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vbroadcasti32x4	128(%rsp),%zmm0
	vpxorq	%zmm0,%zmm1,%zmm1
	vpxorq	%zmm0,%zmm2,%zmm2
	vpsrldq	$0xf,%zmm9,%zmm13
.byte	98,19,21,72,68,241,0
	vpslldq	$0x1,%zmm9,%zmm15
	vpxord	%zmm14,%zmm15,%zmm15
	vbroadcasti32x4	144(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	160(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	176(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208
	vpsrldq	$0xf,%zmm10,%zmm13
.byte	98,19,21,72,68,241,0
	vpslldq	$0x1,%zmm10,%zmm16
	vpxord	%zmm14,%zmm16,%zmm16

	vbroadcasti32x4	192(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	208(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	224(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	240(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	256(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	272(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	288(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	304(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	320(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	336(%rsp),%zmm0
.byte	98,242,117,72,220,200
.byte	98,242,109,72,220,208


	vbroadcasti32x4	352(%rsp),%zmm0
.byte	98,242,117,72,221,200
.byte	98,242,109,72,221,208


	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vmovdqa32	%zmm15,%zmm9
	vmovdqa32	%zmm16,%zmm10
	vmovdqu8	%zmm1,(%rsi)
	vmovdqu8	%zmm2,64(%rsi)
	addq	$0x80,%rsi
	subq	$0x80,%rdx
	cmpq	$0x80,%rdx
	jge	L$_main_loop_run_8_hEgxyDlCngwrfFe
	vextracti32x4	$0x3,%zmm2,%xmm0
	jmp	L$_do_n_blocks_hEgxyDlCngwrfFe

L$_steal_cipher_next_hEgxyDlCngwrfFe:
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,(%rsp)
	movq	%rbx,8(%rsp)
	vmovdqa	(%rsp),%xmm0

L$_steal_cipher_hEgxyDlCngwrfFe:
	vmovdqa	%xmm8,%xmm2
	leaq	vpshufb_shf_table(%rip),%rax
	vmovdqu	(%rax,%rdx,1),%xmm10
	vpshufb	%xmm10,%xmm8,%xmm8
	vmovdqu	-16(%rdi,%rdx,1),%xmm3
	vmovdqu	%xmm8,-16(%rsi,%rdx,1)
	leaq	vpshufb_shf_table(%rip),%rax
	addq	$16,%rax
	subq	%rdx,%rax
	vmovdqu	(%rax),%xmm10
	vpxor	mask1(%rip),%xmm10,%xmm10
	vpshufb	%xmm10,%xmm3,%xmm3
	vpblendvb	%xmm10,%xmm2,%xmm3,%xmm3
	vpxor	%xmm0,%xmm3,%xmm8
	vpxor	128(%rsp),%xmm8,%xmm8
.byte	98,114,61,8,220,132,36,144,0,0,0
.byte	98,114,61,8,220,132,36,160,0,0,0
.byte	98,114,61,8,220,132,36,176,0,0,0
.byte	98,114,61,8,220,132,36,192,0,0,0
.byte	98,114,61,8,220,132,36,208,0,0,0
.byte	98,114,61,8,220,132,36,224,0,0,0
.byte	98,114,61,8,220,132,36,240,0,0,0
.byte	98,114,61,8,220,132,36,0,1,0,0
.byte	98,114,61,8,220,132,36,16,1,0,0
.byte	98,114,61,8,220,132,36,32,1,0,0
.byte	98,114,61,8,220,132,36,48,1,0,0
.byte	98,114,61,8,220,132,36,64,1,0,0
.byte	98,114,61,8,220,132,36,80,1,0,0
.byte	98,114,61,8,221,132,36,96,1,0,0
	vpxor	%xmm0,%xmm8,%xmm8
	vmovdqu	%xmm8,-16(%rsi)
L$_ret_hEgxyDlCngwrfFe:
	movq	368(%rsp),%rbx
	xorq	%r8,%r8
	movq	%r8,368(%rsp)

	vpxorq	%zmm0,%zmm0,%zmm0

	vmovdqa64	%zmm0,128(%rsp)
	vmovdqa64	%zmm0,192(%rsp)
	vmovdqa64	%zmm0,256(%rsp)



	movq	$0x3f,%r8
	kmovq	%r8,%k2
	vmovdqa64	%zmm0,320(%rsp){%k2}

	movq	%rbp,%rsp
	popq	%rbp
	vzeroupper
	.byte	0xf3,0xc3

L$_less_than_128_bytes_hEgxyDlCngwrfFe:
	cmpq	$0x10,%rdx
	jb	L$_ret_hEgxyDlCngwrfFe
	movq	%rdx,%r8
	andq	$0x70,%r8
	cmpq	$0x60,%r8
	je	L$_num_blocks_is_6_hEgxyDlCngwrfFe
	cmpq	$0x50,%r8
	je	L$_num_blocks_is_5_hEgxyDlCngwrfFe
	cmpq	$0x40,%r8
	je	L$_num_blocks_is_4_hEgxyDlCngwrfFe
	cmpq	$0x30,%r8
	je	L$_num_blocks_is_3_hEgxyDlCngwrfFe
	cmpq	$0x20,%r8
	je	L$_num_blocks_is_2_hEgxyDlCngwrfFe
	cmpq	$0x10,%r8
	je	L$_num_blocks_is_1_hEgxyDlCngwrfFe

L$_num_blocks_is_7_hEgxyDlCngwrfFe:
	vmovdqa	0(%rsp),%xmm9
	movq	0(%rsp),%rax
	movq	8(%rsp),%rbx
	vmovdqu	0(%rdi),%xmm1
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,16(%rsp)
	movq	%rbx,24(%rsp)
	vmovdqa	16(%rsp),%xmm10
	vmovdqu	16(%rdi),%xmm2
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,32(%rsp)
	movq	%rbx,40(%rsp)
	vmovdqa	32(%rsp),%xmm11
	vmovdqu	32(%rdi),%xmm3
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,48(%rsp)
	movq	%rbx,56(%rsp)
	vmovdqa	48(%rsp),%xmm12
	vmovdqu	48(%rdi),%xmm4
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,64(%rsp)
	movq	%rbx,72(%rsp)
	vmovdqa	64(%rsp),%xmm13
	vmovdqu	64(%rdi),%xmm5
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,80(%rsp)
	movq	%rbx,88(%rsp)
	vmovdqa	80(%rsp),%xmm14
	vmovdqu	80(%rdi),%xmm6
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,96(%rsp)
	movq	%rbx,104(%rsp)
	vmovdqa	96(%rsp),%xmm15
	vmovdqu	96(%rdi),%xmm7
	addq	$0x70,%rdi
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vpxor	%xmm12,%xmm4,%xmm4
	vpxor	%xmm13,%xmm5,%xmm5
	vpxor	%xmm14,%xmm6,%xmm6
	vpxor	%xmm15,%xmm7,%xmm7
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vpxor	%xmm0,%xmm2,%xmm2
	vpxor	%xmm0,%xmm3,%xmm3
	vpxor	%xmm0,%xmm4,%xmm4
	vpxor	%xmm0,%xmm5,%xmm5
	vpxor	%xmm0,%xmm6,%xmm6
	vpxor	%xmm0,%xmm7,%xmm7
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
.byte	98,242,69,8,220,248
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
.byte	98,242,69,8,220,248
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
.byte	98,242,69,8,220,248
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
.byte	98,242,69,8,220,248
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
.byte	98,242,69,8,220,248
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
.byte	98,242,69,8,220,248
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
.byte	98,242,69,8,220,248
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
.byte	98,242,69,8,220,248
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
.byte	98,242,69,8,220,248
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
.byte	98,242,69,8,220,248
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
.byte	98,242,69,8,220,248
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
.byte	98,242,69,8,220,248
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
.byte	98,242,69,8,220,248
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,221,200
.byte	98,242,109,8,221,208
.byte	98,242,101,8,221,216
.byte	98,242,93,8,221,224
.byte	98,242,85,8,221,232
.byte	98,242,77,8,221,240
.byte	98,242,69,8,221,248
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vpxor	%xmm12,%xmm4,%xmm4
	vpxor	%xmm13,%xmm5,%xmm5
	vpxor	%xmm14,%xmm6,%xmm6
	vpxor	%xmm15,%xmm7,%xmm7
	vmovdqu	%xmm1,(%rsi)
	vmovdqu	%xmm2,16(%rsi)
	vmovdqu	%xmm3,32(%rsi)
	vmovdqu	%xmm4,48(%rsi)
	vmovdqu	%xmm5,64(%rsi)
	vmovdqu	%xmm6,80(%rsi)
	vmovdqu	%xmm7,96(%rsi)
	addq	$0x70,%rsi
	vmovdqa	%xmm7,%xmm8
	andq	$0xf,%rdx
	je	L$_ret_hEgxyDlCngwrfFe
	jmp	L$_steal_cipher_next_hEgxyDlCngwrfFe

L$_num_blocks_is_6_hEgxyDlCngwrfFe:
	vmovdqa	0(%rsp),%xmm9
	movq	0(%rsp),%rax
	movq	8(%rsp),%rbx
	vmovdqu	0(%rdi),%xmm1
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,16(%rsp)
	movq	%rbx,24(%rsp)
	vmovdqa	16(%rsp),%xmm10
	vmovdqu	16(%rdi),%xmm2
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,32(%rsp)
	movq	%rbx,40(%rsp)
	vmovdqa	32(%rsp),%xmm11
	vmovdqu	32(%rdi),%xmm3
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,48(%rsp)
	movq	%rbx,56(%rsp)
	vmovdqa	48(%rsp),%xmm12
	vmovdqu	48(%rdi),%xmm4
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,64(%rsp)
	movq	%rbx,72(%rsp)
	vmovdqa	64(%rsp),%xmm13
	vmovdqu	64(%rdi),%xmm5
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,80(%rsp)
	movq	%rbx,88(%rsp)
	vmovdqa	80(%rsp),%xmm14
	vmovdqu	80(%rdi),%xmm6
	addq	$0x60,%rdi
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vpxor	%xmm12,%xmm4,%xmm4
	vpxor	%xmm13,%xmm5,%xmm5
	vpxor	%xmm14,%xmm6,%xmm6
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vpxor	%xmm0,%xmm2,%xmm2
	vpxor	%xmm0,%xmm3,%xmm3
	vpxor	%xmm0,%xmm4,%xmm4
	vpxor	%xmm0,%xmm5,%xmm5
	vpxor	%xmm0,%xmm6,%xmm6
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
.byte	98,242,77,8,220,240
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,221,200
.byte	98,242,109,8,221,208
.byte	98,242,101,8,221,216
.byte	98,242,93,8,221,224
.byte	98,242,85,8,221,232
.byte	98,242,77,8,221,240
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vpxor	%xmm12,%xmm4,%xmm4
	vpxor	%xmm13,%xmm5,%xmm5
	vpxor	%xmm14,%xmm6,%xmm6
	vmovdqu	%xmm1,(%rsi)
	vmovdqu	%xmm2,16(%rsi)
	vmovdqu	%xmm3,32(%rsi)
	vmovdqu	%xmm4,48(%rsi)
	vmovdqu	%xmm5,64(%rsi)
	vmovdqu	%xmm6,80(%rsi)
	addq	$0x60,%rsi
	vmovdqa	%xmm6,%xmm8
	andq	$0xf,%rdx
	je	L$_ret_hEgxyDlCngwrfFe
	jmp	L$_steal_cipher_next_hEgxyDlCngwrfFe

L$_num_blocks_is_5_hEgxyDlCngwrfFe:
	vmovdqa	0(%rsp),%xmm9
	movq	0(%rsp),%rax
	movq	8(%rsp),%rbx
	vmovdqu	0(%rdi),%xmm1
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,16(%rsp)
	movq	%rbx,24(%rsp)
	vmovdqa	16(%rsp),%xmm10
	vmovdqu	16(%rdi),%xmm2
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,32(%rsp)
	movq	%rbx,40(%rsp)
	vmovdqa	32(%rsp),%xmm11
	vmovdqu	32(%rdi),%xmm3
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,48(%rsp)
	movq	%rbx,56(%rsp)
	vmovdqa	48(%rsp),%xmm12
	vmovdqu	48(%rdi),%xmm4
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,64(%rsp)
	movq	%rbx,72(%rsp)
	vmovdqa	64(%rsp),%xmm13
	vmovdqu	64(%rdi),%xmm5
	addq	$0x50,%rdi
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vpxor	%xmm12,%xmm4,%xmm4
	vpxor	%xmm13,%xmm5,%xmm5
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vpxor	%xmm0,%xmm2,%xmm2
	vpxor	%xmm0,%xmm3,%xmm3
	vpxor	%xmm0,%xmm4,%xmm4
	vpxor	%xmm0,%xmm5,%xmm5
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
.byte	98,242,85,8,220,232
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,221,200
.byte	98,242,109,8,221,208
.byte	98,242,101,8,221,216
.byte	98,242,93,8,221,224
.byte	98,242,85,8,221,232
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vpxor	%xmm12,%xmm4,%xmm4
	vpxor	%xmm13,%xmm5,%xmm5
	vmovdqu	%xmm1,(%rsi)
	vmovdqu	%xmm2,16(%rsi)
	vmovdqu	%xmm3,32(%rsi)
	vmovdqu	%xmm4,48(%rsi)
	vmovdqu	%xmm5,64(%rsi)
	addq	$0x50,%rsi
	vmovdqa	%xmm5,%xmm8
	andq	$0xf,%rdx
	je	L$_ret_hEgxyDlCngwrfFe
	jmp	L$_steal_cipher_next_hEgxyDlCngwrfFe

L$_num_blocks_is_4_hEgxyDlCngwrfFe:
	vmovdqa	0(%rsp),%xmm9
	movq	0(%rsp),%rax
	movq	8(%rsp),%rbx
	vmovdqu	0(%rdi),%xmm1
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,16(%rsp)
	movq	%rbx,24(%rsp)
	vmovdqa	16(%rsp),%xmm10
	vmovdqu	16(%rdi),%xmm2
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,32(%rsp)
	movq	%rbx,40(%rsp)
	vmovdqa	32(%rsp),%xmm11
	vmovdqu	32(%rdi),%xmm3
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,48(%rsp)
	movq	%rbx,56(%rsp)
	vmovdqa	48(%rsp),%xmm12
	vmovdqu	48(%rdi),%xmm4
	addq	$0x40,%rdi
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vpxor	%xmm12,%xmm4,%xmm4
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vpxor	%xmm0,%xmm2,%xmm2
	vpxor	%xmm0,%xmm3,%xmm3
	vpxor	%xmm0,%xmm4,%xmm4
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
.byte	98,242,93,8,220,224
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,221,200
.byte	98,242,109,8,221,208
.byte	98,242,101,8,221,216
.byte	98,242,93,8,221,224
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vpxor	%xmm12,%xmm4,%xmm4
	vmovdqu	%xmm1,(%rsi)
	vmovdqu	%xmm2,16(%rsi)
	vmovdqu	%xmm3,32(%rsi)
	vmovdqu	%xmm4,48(%rsi)
	addq	$0x40,%rsi
	vmovdqa	%xmm4,%xmm8
	andq	$0xf,%rdx
	je	L$_ret_hEgxyDlCngwrfFe
	jmp	L$_steal_cipher_next_hEgxyDlCngwrfFe

L$_num_blocks_is_3_hEgxyDlCngwrfFe:
	vmovdqa	0(%rsp),%xmm9
	movq	0(%rsp),%rax
	movq	8(%rsp),%rbx
	vmovdqu	0(%rdi),%xmm1
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,16(%rsp)
	movq	%rbx,24(%rsp)
	vmovdqa	16(%rsp),%xmm10
	vmovdqu	16(%rdi),%xmm2
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,32(%rsp)
	movq	%rbx,40(%rsp)
	vmovdqa	32(%rsp),%xmm11
	vmovdqu	32(%rdi),%xmm3
	addq	$0x30,%rdi
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vpxor	%xmm0,%xmm2,%xmm2
	vpxor	%xmm0,%xmm3,%xmm3
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
.byte	98,242,101,8,220,216
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,221,200
.byte	98,242,109,8,221,208
.byte	98,242,101,8,221,216
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vmovdqu	%xmm1,(%rsi)
	vmovdqu	%xmm2,16(%rsi)
	vmovdqu	%xmm3,32(%rsi)
	addq	$0x30,%rsi
	vmovdqa	%xmm3,%xmm8
	andq	$0xf,%rdx
	je	L$_ret_hEgxyDlCngwrfFe
	jmp	L$_steal_cipher_next_hEgxyDlCngwrfFe

L$_num_blocks_is_2_hEgxyDlCngwrfFe:
	vmovdqa	0(%rsp),%xmm9
	movq	0(%rsp),%rax
	movq	8(%rsp),%rbx
	vmovdqu	0(%rdi),%xmm1
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,16(%rsp)
	movq	%rbx,24(%rsp)
	vmovdqa	16(%rsp),%xmm10
	vmovdqu	16(%rdi),%xmm2
	addq	$0x20,%rdi
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vpxor	%xmm0,%xmm2,%xmm2
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,220,200
.byte	98,242,109,8,220,208
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,221,200
.byte	98,242,109,8,221,208
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vmovdqu	%xmm1,(%rsi)
	vmovdqu	%xmm2,16(%rsi)
	addq	$0x20,%rsi
	vmovdqa	%xmm2,%xmm8
	andq	$0xf,%rdx
	je	L$_ret_hEgxyDlCngwrfFe
	jmp	L$_steal_cipher_next_hEgxyDlCngwrfFe

L$_num_blocks_is_1_hEgxyDlCngwrfFe:
	vmovdqa	0(%rsp),%xmm9
	movq	0(%rsp),%rax
	movq	8(%rsp),%rbx
	vmovdqu	0(%rdi),%xmm1
	addq	$0x10,%rdi
	vpxor	%xmm9,%xmm1,%xmm1
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,220,200
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,221,200
	vpxor	%xmm9,%xmm1,%xmm1
	vmovdqu	%xmm1,(%rsi)
	addq	$0x10,%rsi
	vmovdqa	%xmm1,%xmm8
	andq	$0xf,%rdx
	je	L$_ret_hEgxyDlCngwrfFe
	jmp	L$_steal_cipher_next_hEgxyDlCngwrfFe

.globl	_aes_hw_xts_decrypt_avx512
.private_extern _aes_hw_xts_decrypt_avx512
.private_extern	_aes_hw_xts_decrypt_avx512

.p2align	5
_aes_hw_xts_decrypt_avx512:

.byte	243,15,30,250
	pushq	%rbp
	movq	%rsp,%rbp
	subq	$376,%rsp
	andq	$0xffffffffffffffc0,%rsp
	movq	%rbx,368(%rsp)
	movq	$0x87,%r10
	vmovdqu	(%r9),%xmm1
	vpxor	%xmm4,%xmm4,%xmm4
	vmovdqu	(%r8),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1

	vmovdqu	224(%rcx),%xmm2
	vmovdqa	%xmm2,352(%rsp)

	vmovdqu	16(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	208(%rcx),%xmm2
	vmovdqa	%xmm2,336(%rsp)

	vmovdqu	32(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	192(%rcx),%xmm2
	vmovdqa	%xmm2,320(%rsp)

	vmovdqu	48(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	176(%rcx),%xmm2
	vmovdqa	%xmm2,304(%rsp)

	vmovdqu	64(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	160(%rcx),%xmm2
	vmovdqa	%xmm2,288(%rsp)

	vmovdqu	80(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	144(%rcx),%xmm2
	vmovdqa	%xmm2,272(%rsp)

	vmovdqu	96(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	128(%rcx),%xmm2
	vmovdqa	%xmm2,256(%rsp)

	vmovdqu	112(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	112(%rcx),%xmm2
	vmovdqa	%xmm2,240(%rsp)

	vmovdqu	128(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	96(%rcx),%xmm2
	vmovdqa	%xmm2,224(%rsp)

	vmovdqu	144(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	80(%rcx),%xmm2
	vmovdqa	%xmm2,208(%rsp)

	vmovdqu	160(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	64(%rcx),%xmm2
	vmovdqa	%xmm2,192(%rsp)

	vmovdqu	176(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	48(%rcx),%xmm2
	vmovdqa	%xmm2,176(%rsp)

	vmovdqu	192(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	32(%rcx),%xmm2
	vmovdqa	%xmm2,160(%rsp)

	vmovdqu	208(%r8),%xmm0
.byte	98,242,117,8,220,200

	vmovdqu	16(%rcx),%xmm2
	vmovdqa	%xmm2,144(%rsp)

	vmovdqu	224(%r8),%xmm0
.byte	98,242,117,8,221,200

	vmovdqu	(%rcx),%xmm2
	vmovdqa	%xmm2,128(%rsp)

	vmovdqa	%xmm1,(%rsp)

	cmpq	$0x80,%rdx
	jb	L$_less_than_128_bytes_amivrujEyduiFoi
	vpbroadcastq	%r10,%zmm25
	cmpq	$0x100,%rdx
	jge	L$_start_by16_amivrujEyduiFoi
	jmp	L$_start_by8_amivrujEyduiFoi

L$_do_n_blocks_amivrujEyduiFoi:
	cmpq	$0x0,%rdx
	je	L$_ret_amivrujEyduiFoi
	cmpq	$0x70,%rdx
	jge	L$_remaining_num_blocks_is_7_amivrujEyduiFoi
	cmpq	$0x60,%rdx
	jge	L$_remaining_num_blocks_is_6_amivrujEyduiFoi
	cmpq	$0x50,%rdx
	jge	L$_remaining_num_blocks_is_5_amivrujEyduiFoi
	cmpq	$0x40,%rdx
	jge	L$_remaining_num_blocks_is_4_amivrujEyduiFoi
	cmpq	$0x30,%rdx
	jge	L$_remaining_num_blocks_is_3_amivrujEyduiFoi
	cmpq	$0x20,%rdx
	jge	L$_remaining_num_blocks_is_2_amivrujEyduiFoi
	cmpq	$0x10,%rdx
	jge	L$_remaining_num_blocks_is_1_amivrujEyduiFoi


	vmovdqu	%xmm5,%xmm1

	vpxor	%xmm9,%xmm1,%xmm1
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,223,200
	vpxor	%xmm9,%xmm1,%xmm1
	vmovdqu	%xmm1,-16(%rsi)
	vmovdqa	%xmm1,%xmm8


	movq	$0x1,%r8
	kmovq	%r8,%k1
	vpsllq	$0x3f,%xmm9,%xmm13
	vpsraq	$0x3f,%xmm13,%xmm14
	vpandq	%xmm25,%xmm14,%xmm5
	vpxorq	%xmm5,%xmm9,%xmm9{%k1}
	vpsrldq	$0x8,%xmm9,%xmm10
.byte	98, 211, 181, 8, 115, 194, 1
	vpslldq	$0x8,%xmm13,%xmm13
	vpxorq	%xmm13,%xmm0,%xmm0
	jmp	L$_steal_cipher_amivrujEyduiFoi

L$_remaining_num_blocks_is_7_amivrujEyduiFoi:
	movq	$0xffffffffffffffff,%r8
	shrq	$0x10,%r8
	kmovq	%r8,%k1
	vmovdqu8	(%rdi),%zmm1
	vmovdqu8	64(%rdi),%zmm2{%k1}
	addq	$0x70,%rdi
	andq	$0xf,%rdx
	je	L$_done_7_remain_amivrujEyduiFoi
	vextracti32x4	$0x2,%zmm10,%xmm12
	vextracti32x4	$0x3,%zmm10,%xmm13
	vinserti32x4	$0x2,%xmm13,%zmm10,%zmm10

	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vbroadcasti32x4	128(%rsp),%zmm0
	vpxorq	%zmm0,%zmm1,%zmm1
	vpxorq	%zmm0,%zmm2,%zmm2
	vbroadcasti32x4	144(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	160(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	176(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208

	vbroadcasti32x4	192(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	208(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	224(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	240(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	256(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	272(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	288(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	304(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	320(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	336(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	352(%rsp),%zmm0
.byte	98,242,117,72,223,200
.byte	98,242,109,72,223,208


	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vmovdqa32	%zmm15,%zmm9
	vmovdqa32	%zmm16,%zmm10
	vmovdqu8	%zmm1,(%rsi)
	vmovdqu8	%zmm2,64(%rsi){%k1}
	addq	$0x70,%rsi
	vextracti32x4	$0x2,%zmm2,%xmm8
	vmovdqa	%xmm12,%xmm0
	jmp	L$_steal_cipher_amivrujEyduiFoi

L$_done_7_remain_amivrujEyduiFoi:

	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vbroadcasti32x4	128(%rsp),%zmm0
	vpxorq	%zmm0,%zmm1,%zmm1
	vpxorq	%zmm0,%zmm2,%zmm2
	vbroadcasti32x4	144(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	160(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	176(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208

	vbroadcasti32x4	192(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	208(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	224(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	240(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	256(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	272(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	288(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	304(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	320(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	336(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	352(%rsp),%zmm0
.byte	98,242,117,72,223,200
.byte	98,242,109,72,223,208


	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vmovdqa32	%zmm15,%zmm9
	vmovdqa32	%zmm16,%zmm10
	vmovdqu8	%zmm1,(%rsi)
	vmovdqu8	%zmm2,64(%rsi){%k1}
	jmp	L$_ret_amivrujEyduiFoi

L$_remaining_num_blocks_is_6_amivrujEyduiFoi:
	vmovdqu8	(%rdi),%zmm1
	vmovdqu8	64(%rdi),%ymm2
	addq	$0x60,%rdi
	andq	$0xf,%rdx
	je	L$_done_6_remain_amivrujEyduiFoi
	vextracti32x4	$0x1,%zmm10,%xmm12
	vextracti32x4	$0x2,%zmm10,%xmm13
	vinserti32x4	$0x1,%xmm13,%zmm10,%zmm10

	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vbroadcasti32x4	128(%rsp),%zmm0
	vpxorq	%zmm0,%zmm1,%zmm1
	vpxorq	%zmm0,%zmm2,%zmm2
	vbroadcasti32x4	144(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	160(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	176(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208

	vbroadcasti32x4	192(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	208(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	224(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	240(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	256(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	272(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	288(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	304(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	320(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	336(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	352(%rsp),%zmm0
.byte	98,242,117,72,223,200
.byte	98,242,109,72,223,208


	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vmovdqa32	%zmm15,%zmm9
	vmovdqa32	%zmm16,%zmm10
	vmovdqu8	%zmm1,(%rsi)
	vmovdqu8	%ymm2,64(%rsi)
	addq	$0x60,%rsi
	vextracti32x4	$0x1,%zmm2,%xmm8
	vmovdqa	%xmm12,%xmm0
	jmp	L$_steal_cipher_amivrujEyduiFoi

L$_done_6_remain_amivrujEyduiFoi:

	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vbroadcasti32x4	128(%rsp),%zmm0
	vpxorq	%zmm0,%zmm1,%zmm1
	vpxorq	%zmm0,%zmm2,%zmm2
	vbroadcasti32x4	144(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	160(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	176(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208

	vbroadcasti32x4	192(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	208(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	224(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	240(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	256(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	272(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	288(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	304(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	320(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	336(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	352(%rsp),%zmm0
.byte	98,242,117,72,223,200
.byte	98,242,109,72,223,208


	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vmovdqa32	%zmm15,%zmm9
	vmovdqa32	%zmm16,%zmm10
	vmovdqu8	%zmm1,(%rsi)
	vmovdqu8	%ymm2,64(%rsi)
	jmp	L$_ret_amivrujEyduiFoi

L$_remaining_num_blocks_is_5_amivrujEyduiFoi:
	vmovdqu8	(%rdi),%zmm1
	vmovdqu	64(%rdi),%xmm2
	addq	$0x50,%rdi
	andq	$0xf,%rdx
	je	L$_done_5_remain_amivrujEyduiFoi
	vmovdqa	%xmm10,%xmm12
	vextracti32x4	$0x1,%zmm10,%xmm10

	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vbroadcasti32x4	128(%rsp),%zmm0
	vpxorq	%zmm0,%zmm1,%zmm1
	vpxorq	%zmm0,%zmm2,%zmm2
	vbroadcasti32x4	144(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	160(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	176(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208

	vbroadcasti32x4	192(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	208(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	224(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	240(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	256(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	272(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	288(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	304(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	320(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	336(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	352(%rsp),%zmm0
.byte	98,242,117,72,223,200
.byte	98,242,109,72,223,208


	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vmovdqa32	%zmm15,%zmm9
	vmovdqa32	%zmm16,%zmm10
	vmovdqu8	%zmm1,(%rsi)
	vmovdqu	%xmm2,64(%rsi)
	addq	$0x50,%rsi
	vmovdqa	%xmm2,%xmm8
	vmovdqa	%xmm12,%xmm0
	jmp	L$_steal_cipher_amivrujEyduiFoi

L$_done_5_remain_amivrujEyduiFoi:

	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vbroadcasti32x4	128(%rsp),%zmm0
	vpxorq	%zmm0,%zmm1,%zmm1
	vpxorq	%zmm0,%zmm2,%zmm2
	vbroadcasti32x4	144(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	160(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	176(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208

	vbroadcasti32x4	192(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	208(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	224(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	240(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	256(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	272(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	288(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	304(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	320(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	336(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	352(%rsp),%zmm0
.byte	98,242,117,72,223,200
.byte	98,242,109,72,223,208


	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vmovdqa32	%zmm15,%zmm9
	vmovdqa32	%zmm16,%zmm10
	vmovdqu8	%zmm1,(%rsi)
	vmovdqu8	%xmm2,64(%rsi)
	jmp	L$_ret_amivrujEyduiFoi

L$_remaining_num_blocks_is_4_amivrujEyduiFoi:
	vmovdqu8	(%rdi),%zmm1
	addq	$0x40,%rdi
	andq	$0xf,%rdx
	je	L$_done_4_remain_amivrujEyduiFoi
	vextracti32x4	$0x3,%zmm9,%xmm12
	vinserti32x4	$0x3,%xmm10,%zmm9,%zmm9

	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vbroadcasti32x4	128(%rsp),%zmm0
	vpxorq	%zmm0,%zmm1,%zmm1
	vpxorq	%zmm0,%zmm2,%zmm2
	vbroadcasti32x4	144(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	160(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	176(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208

	vbroadcasti32x4	192(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	208(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	224(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	240(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	256(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	272(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	288(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	304(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	320(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	336(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	352(%rsp),%zmm0
.byte	98,242,117,72,223,200
.byte	98,242,109,72,223,208


	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vmovdqa32	%zmm15,%zmm9
	vmovdqa32	%zmm16,%zmm10
	vmovdqu8	%zmm1,(%rsi)
	addq	$0x40,%rsi
	vextracti32x4	$0x3,%zmm1,%xmm8
	vmovdqa	%xmm12,%xmm0
	jmp	L$_steal_cipher_amivrujEyduiFoi

L$_done_4_remain_amivrujEyduiFoi:

	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vbroadcasti32x4	128(%rsp),%zmm0
	vpxorq	%zmm0,%zmm1,%zmm1
	vpxorq	%zmm0,%zmm2,%zmm2
	vbroadcasti32x4	144(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	160(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	176(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208

	vbroadcasti32x4	192(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	208(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	224(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	240(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	256(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	272(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	288(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	304(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	320(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	336(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	352(%rsp),%zmm0
.byte	98,242,117,72,223,200
.byte	98,242,109,72,223,208


	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vmovdqa32	%zmm15,%zmm9
	vmovdqa32	%zmm16,%zmm10
	vmovdqu8	%zmm1,(%rsi)
	jmp	L$_ret_amivrujEyduiFoi

L$_remaining_num_blocks_is_3_amivrujEyduiFoi:
	vmovdqu	(%rdi),%xmm1
	vmovdqu	16(%rdi),%xmm2
	vmovdqu	32(%rdi),%xmm3
	addq	$0x30,%rdi
	andq	$0xf,%rdx
	je	L$_done_3_remain_amivrujEyduiFoi
	vextracti32x4	$0x2,%zmm9,%xmm13
	vextracti32x4	$0x1,%zmm9,%xmm10
	vextracti32x4	$0x3,%zmm9,%xmm11
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vpxor	%xmm0,%xmm2,%xmm2
	vpxor	%xmm0,%xmm3,%xmm3
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,223,200
.byte	98,242,109,8,223,208
.byte	98,242,101,8,223,216
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vmovdqu	%xmm1,(%rsi)
	vmovdqu	%xmm2,16(%rsi)
	vmovdqu	%xmm3,32(%rsi)
	addq	$0x30,%rsi
	vmovdqa	%xmm3,%xmm8
	vmovdqa	%xmm13,%xmm0
	jmp	L$_steal_cipher_amivrujEyduiFoi

L$_done_3_remain_amivrujEyduiFoi:
	vextracti32x4	$0x1,%zmm9,%xmm10
	vextracti32x4	$0x2,%zmm9,%xmm11
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vpxor	%xmm0,%xmm2,%xmm2
	vpxor	%xmm0,%xmm3,%xmm3
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,223,200
.byte	98,242,109,8,223,208
.byte	98,242,101,8,223,216
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vmovdqu	%xmm1,(%rsi)
	vmovdqu	%xmm2,16(%rsi)
	vmovdqu	%xmm3,32(%rsi)
	jmp	L$_ret_amivrujEyduiFoi

L$_remaining_num_blocks_is_2_amivrujEyduiFoi:
	vmovdqu	(%rdi),%xmm1
	vmovdqu	16(%rdi),%xmm2
	addq	$0x20,%rdi
	andq	$0xf,%rdx
	je	L$_done_2_remain_amivrujEyduiFoi
	vextracti32x4	$0x2,%zmm9,%xmm10
	vextracti32x4	$0x1,%zmm9,%xmm12
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vpxor	%xmm0,%xmm2,%xmm2
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,223,200
.byte	98,242,109,8,223,208
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vmovdqu	%xmm1,(%rsi)
	vmovdqu	%xmm2,16(%rsi)
	addq	$0x20,%rsi
	vmovdqa	%xmm2,%xmm8
	vmovdqa	%xmm12,%xmm0
	jmp	L$_steal_cipher_amivrujEyduiFoi

L$_done_2_remain_amivrujEyduiFoi:
	vextracti32x4	$0x1,%zmm9,%xmm10
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vpxor	%xmm0,%xmm2,%xmm2
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,223,200
.byte	98,242,109,8,223,208
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vmovdqu	%xmm1,(%rsi)
	vmovdqu	%xmm2,16(%rsi)
	jmp	L$_ret_amivrujEyduiFoi

L$_remaining_num_blocks_is_1_amivrujEyduiFoi:
	vmovdqu	(%rdi),%xmm1
	addq	$0x10,%rdi
	andq	$0xf,%rdx
	je	L$_done_1_remain_amivrujEyduiFoi
	vextracti32x4	$0x1,%zmm9,%xmm11
	vpxor	%xmm11,%xmm1,%xmm1
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,223,200
	vpxor	%xmm11,%xmm1,%xmm1
	vmovdqu	%xmm1,(%rsi)
	addq	$0x10,%rsi
	vmovdqa	%xmm1,%xmm8
	vmovdqa	%xmm9,%xmm0
	jmp	L$_steal_cipher_amivrujEyduiFoi

L$_done_1_remain_amivrujEyduiFoi:
	vpxor	%xmm9,%xmm1,%xmm1
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,223,200
	vpxor	%xmm9,%xmm1,%xmm1
	vmovdqu	%xmm1,(%rsi)
	jmp	L$_ret_amivrujEyduiFoi

L$_start_by16_amivrujEyduiFoi:
	vbroadcasti32x4	(%rsp),%zmm0
	vbroadcasti32x4	shufb_15_7(%rip),%zmm8
	movq	$0xaa,%r8
	kmovq	%r8,%k2


	vpshufb	%zmm8,%zmm0,%zmm1
	vpsllvq	const_dq3210(%rip),%zmm0,%zmm4
	vpsrlvq	const_dq5678(%rip),%zmm1,%zmm2
.byte	98,147,109,72,68,217,0
	vpxorq	%zmm2,%zmm4,%zmm4{%k2}
	vpxord	%zmm4,%zmm3,%zmm9


	vpsllvq	const_dq7654(%rip),%zmm0,%zmm5
	vpsrlvq	const_dq1234(%rip),%zmm1,%zmm6
.byte	98,147,77,72,68,249,0
	vpxorq	%zmm6,%zmm5,%zmm5{%k2}
	vpxord	%zmm5,%zmm7,%zmm10


	vpsrldq	$0xf,%zmm9,%zmm13
.byte	98,19,21,72,68,241,0
	vpslldq	$0x1,%zmm9,%zmm11
	vpxord	%zmm14,%zmm11,%zmm11

	vpsrldq	$0xf,%zmm10,%zmm15
.byte	98,131,5,72,68,193,0
	vpslldq	$0x1,%zmm10,%zmm12
	vpxord	%zmm16,%zmm12,%zmm12

L$_main_loop_run_16_amivrujEyduiFoi:
	vmovdqu8	(%rdi),%zmm1
	vmovdqu8	64(%rdi),%zmm2
	vmovdqu8	128(%rdi),%zmm3
	vmovdqu8	192(%rdi),%zmm4
	vmovdqu8	240(%rdi),%xmm5
	addq	$0x100,%rdi
	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2
	vpxorq	%zmm11,%zmm3,%zmm3
	vpxorq	%zmm12,%zmm4,%zmm4
	vbroadcasti32x4	128(%rsp),%zmm0
	vpxorq	%zmm0,%zmm1,%zmm1
	vpxorq	%zmm0,%zmm2,%zmm2
	vpxorq	%zmm0,%zmm3,%zmm3
	vpxorq	%zmm0,%zmm4,%zmm4
	vpsrldq	$0xf,%zmm11,%zmm13
.byte	98,19,21,72,68,241,0
	vpslldq	$0x1,%zmm11,%zmm15
	vpxord	%zmm14,%zmm15,%zmm15
	vbroadcasti32x4	144(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208
.byte	98,242,101,72,222,216
.byte	98,242,93,72,222,224
	vbroadcasti32x4	160(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208
.byte	98,242,101,72,222,216
.byte	98,242,93,72,222,224
	vbroadcasti32x4	176(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208
.byte	98,242,101,72,222,216
.byte	98,242,93,72,222,224
	vpsrldq	$0xf,%zmm12,%zmm13
.byte	98,19,21,72,68,241,0
	vpslldq	$0x1,%zmm12,%zmm16
	vpxord	%zmm14,%zmm16,%zmm16
	vbroadcasti32x4	192(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208
.byte	98,242,101,72,222,216
.byte	98,242,93,72,222,224
	vbroadcasti32x4	208(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208
.byte	98,242,101,72,222,216
.byte	98,242,93,72,222,224
	vbroadcasti32x4	224(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208
.byte	98,242,101,72,222,216
.byte	98,242,93,72,222,224
	vpsrldq	$0xf,%zmm15,%zmm13
.byte	98,19,21,72,68,241,0
	vpslldq	$0x1,%zmm15,%zmm17
	vpxord	%zmm14,%zmm17,%zmm17
	vbroadcasti32x4	240(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208
.byte	98,242,101,72,222,216
.byte	98,242,93,72,222,224
	vbroadcasti32x4	256(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208
.byte	98,242,101,72,222,216
.byte	98,242,93,72,222,224
	vbroadcasti32x4	272(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208
.byte	98,242,101,72,222,216
.byte	98,242,93,72,222,224
	vpsrldq	$0xf,%zmm16,%zmm13
.byte	98,19,21,72,68,241,0
	vpslldq	$0x1,%zmm16,%zmm18
	vpxord	%zmm14,%zmm18,%zmm18
	vbroadcasti32x4	288(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208
.byte	98,242,101,72,222,216
.byte	98,242,93,72,222,224
	vbroadcasti32x4	304(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208
.byte	98,242,101,72,222,216
.byte	98,242,93,72,222,224
	vbroadcasti32x4	320(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208
.byte	98,242,101,72,222,216
.byte	98,242,93,72,222,224
	vbroadcasti32x4	336(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208
.byte	98,242,101,72,222,216
.byte	98,242,93,72,222,224
	vbroadcasti32x4	352(%rsp),%zmm0
.byte	98,242,117,72,223,200
.byte	98,242,109,72,223,208
.byte	98,242,101,72,223,216
.byte	98,242,93,72,223,224
	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2
	vpxorq	%zmm11,%zmm3,%zmm3
	vpxorq	%zmm12,%zmm4,%zmm4

	vmovdqa32	%zmm15,%zmm9
	vmovdqa32	%zmm16,%zmm10
	vmovdqa32	%zmm17,%zmm11
	vmovdqa32	%zmm18,%zmm12
	vmovdqu8	%zmm1,(%rsi)
	vmovdqu8	%zmm2,64(%rsi)
	vmovdqu8	%zmm3,128(%rsi)
	vmovdqu8	%zmm4,192(%rsi)
	addq	$0x100,%rsi
	subq	$0x100,%rdx
	cmpq	$0x100,%rdx
	jge	L$_main_loop_run_16_amivrujEyduiFoi

	cmpq	$0x80,%rdx
	jge	L$_main_loop_run_8_amivrujEyduiFoi
	jmp	L$_do_n_blocks_amivrujEyduiFoi

L$_start_by8_amivrujEyduiFoi:

	vbroadcasti32x4	(%rsp),%zmm0
	vbroadcasti32x4	shufb_15_7(%rip),%zmm8
	movq	$0xaa,%r8
	kmovq	%r8,%k2


	vpshufb	%zmm8,%zmm0,%zmm1
	vpsllvq	const_dq3210(%rip),%zmm0,%zmm4
	vpsrlvq	const_dq5678(%rip),%zmm1,%zmm2
.byte	98,147,109,72,68,217,0
	vpxorq	%zmm2,%zmm4,%zmm4{%k2}
	vpxord	%zmm4,%zmm3,%zmm9


	vpsllvq	const_dq7654(%rip),%zmm0,%zmm5
	vpsrlvq	const_dq1234(%rip),%zmm1,%zmm6
.byte	98,147,77,72,68,249,0
	vpxorq	%zmm6,%zmm5,%zmm5{%k2}
	vpxord	%zmm5,%zmm7,%zmm10

L$_main_loop_run_8_amivrujEyduiFoi:
	vmovdqu8	(%rdi),%zmm1
	vmovdqu8	64(%rdi),%zmm2
	vmovdqu8	112(%rdi),%xmm5
	addq	$0x80,%rdi

	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vbroadcasti32x4	128(%rsp),%zmm0
	vpxorq	%zmm0,%zmm1,%zmm1
	vpxorq	%zmm0,%zmm2,%zmm2
	vpsrldq	$0xf,%zmm9,%zmm13
.byte	98,19,21,72,68,241,0
	vpslldq	$0x1,%zmm9,%zmm15
	vpxord	%zmm14,%zmm15,%zmm15
	vbroadcasti32x4	144(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	160(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	176(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208
	vpsrldq	$0xf,%zmm10,%zmm13
.byte	98,19,21,72,68,241,0
	vpslldq	$0x1,%zmm10,%zmm16
	vpxord	%zmm14,%zmm16,%zmm16

	vbroadcasti32x4	192(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	208(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	224(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	240(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	256(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	272(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	288(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	304(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	320(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	336(%rsp),%zmm0
.byte	98,242,117,72,222,200
.byte	98,242,109,72,222,208


	vbroadcasti32x4	352(%rsp),%zmm0
.byte	98,242,117,72,223,200
.byte	98,242,109,72,223,208


	vpxorq	%zmm9,%zmm1,%zmm1
	vpxorq	%zmm10,%zmm2,%zmm2


	vmovdqa32	%zmm15,%zmm9
	vmovdqa32	%zmm16,%zmm10
	vmovdqu8	%zmm1,(%rsi)
	vmovdqu8	%zmm2,64(%rsi)
	addq	$0x80,%rsi
	subq	$0x80,%rdx
	cmpq	$0x80,%rdx
	jge	L$_main_loop_run_8_amivrujEyduiFoi
	jmp	L$_do_n_blocks_amivrujEyduiFoi

L$_steal_cipher_amivrujEyduiFoi:

	vmovdqa	%xmm8,%xmm2


	leaq	vpshufb_shf_table(%rip),%rax
	vmovdqu	(%rax,%rdx,1),%xmm10
	vpshufb	%xmm10,%xmm8,%xmm8


	vmovdqu	-16(%rdi,%rdx,1),%xmm3
	vmovdqu	%xmm8,-16(%rsi,%rdx,1)


	leaq	vpshufb_shf_table(%rip),%rax
	addq	$16,%rax
	subq	%rdx,%rax
	vmovdqu	(%rax),%xmm10
	vpxor	mask1(%rip),%xmm10,%xmm10
	vpshufb	%xmm10,%xmm3,%xmm3

	vpblendvb	%xmm10,%xmm2,%xmm3,%xmm3


	vpxor	%xmm0,%xmm3,%xmm8


	vpxor	128(%rsp),%xmm8,%xmm8
.byte	98,114,61,8,222,132,36,144,0,0,0
.byte	98,114,61,8,222,132,36,160,0,0,0
.byte	98,114,61,8,222,132,36,176,0,0,0
.byte	98,114,61,8,222,132,36,192,0,0,0
.byte	98,114,61,8,222,132,36,208,0,0,0
.byte	98,114,61,8,222,132,36,224,0,0,0
.byte	98,114,61,8,222,132,36,240,0,0,0
.byte	98,114,61,8,222,132,36,0,1,0,0
.byte	98,114,61,8,222,132,36,16,1,0,0
.byte	98,114,61,8,222,132,36,32,1,0,0
.byte	98,114,61,8,222,132,36,48,1,0,0
.byte	98,114,61,8,222,132,36,64,1,0,0
.byte	98,114,61,8,222,132,36,80,1,0,0
.byte	98,114,61,8,223,132,36,96,1,0,0


	vpxor	%xmm0,%xmm8,%xmm8

L$_done_amivrujEyduiFoi:

	vmovdqu	%xmm8,-16(%rsi)
L$_ret_amivrujEyduiFoi:
	movq	368(%rsp),%rbx
	xorq	%r8,%r8
	movq	%r8,368(%rsp)

	vpxorq	%zmm0,%zmm0,%zmm0

	vmovdqa64	%zmm0,128(%rsp)
	vmovdqa64	%zmm0,192(%rsp)
	vmovdqa64	%zmm0,256(%rsp)



	movq	$0x3f,%r8
	kmovq	%r8,%k2
	vmovdqa64	%zmm0,320(%rsp){%k2}

	movq	%rbp,%rsp
	popq	%rbp
	vzeroupper
	.byte	0xf3,0xc3

L$_less_than_128_bytes_amivrujEyduiFoi:
	cmpq	$0x10,%rdx
	jb	L$_ret_amivrujEyduiFoi

	movq	%rdx,%r8
	andq	$0x70,%r8
	cmpq	$0x60,%r8
	je	L$_num_blocks_is_6_amivrujEyduiFoi
	cmpq	$0x50,%r8
	je	L$_num_blocks_is_5_amivrujEyduiFoi
	cmpq	$0x40,%r8
	je	L$_num_blocks_is_4_amivrujEyduiFoi
	cmpq	$0x30,%r8
	je	L$_num_blocks_is_3_amivrujEyduiFoi
	cmpq	$0x20,%r8
	je	L$_num_blocks_is_2_amivrujEyduiFoi
	cmpq	$0x10,%r8
	je	L$_num_blocks_is_1_amivrujEyduiFoi

L$_num_blocks_is_7_amivrujEyduiFoi:
	vmovdqa	0(%rsp),%xmm9
	movq	0(%rsp),%rax
	movq	8(%rsp),%rbx
	vmovdqu	0(%rdi),%xmm1
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,16(%rsp)
	movq	%rbx,24(%rsp)
	vmovdqa	16(%rsp),%xmm10
	vmovdqu	16(%rdi),%xmm2
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,32(%rsp)
	movq	%rbx,40(%rsp)
	vmovdqa	32(%rsp),%xmm11
	vmovdqu	32(%rdi),%xmm3
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,48(%rsp)
	movq	%rbx,56(%rsp)
	vmovdqa	48(%rsp),%xmm12
	vmovdqu	48(%rdi),%xmm4
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,64(%rsp)
	movq	%rbx,72(%rsp)
	vmovdqa	64(%rsp),%xmm13
	vmovdqu	64(%rdi),%xmm5
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,80(%rsp)
	movq	%rbx,88(%rsp)
	vmovdqa	80(%rsp),%xmm14
	vmovdqu	80(%rdi),%xmm6
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,96(%rsp)
	movq	%rbx,104(%rsp)
	vmovdqa	96(%rsp),%xmm15
	vmovdqu	96(%rdi),%xmm7
	addq	$0x70,%rdi
	andq	$0xf,%rdx
	je	L$_done_7_amivrujEyduiFoi

L$_steal_cipher_7_amivrujEyduiFoi:
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,16(%rsp)
	movq	%rbx,24(%rsp)
	vmovdqa64	%xmm15,%xmm16
	vmovdqa	16(%rsp),%xmm15
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vpxor	%xmm12,%xmm4,%xmm4
	vpxor	%xmm13,%xmm5,%xmm5
	vpxor	%xmm14,%xmm6,%xmm6
	vpxor	%xmm15,%xmm7,%xmm7
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vpxor	%xmm0,%xmm2,%xmm2
	vpxor	%xmm0,%xmm3,%xmm3
	vpxor	%xmm0,%xmm4,%xmm4
	vpxor	%xmm0,%xmm5,%xmm5
	vpxor	%xmm0,%xmm6,%xmm6
	vpxor	%xmm0,%xmm7,%xmm7
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,223,200
.byte	98,242,109,8,223,208
.byte	98,242,101,8,223,216
.byte	98,242,93,8,223,224
.byte	98,242,85,8,223,232
.byte	98,242,77,8,223,240
.byte	98,242,69,8,223,248
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vpxor	%xmm12,%xmm4,%xmm4
	vpxor	%xmm13,%xmm5,%xmm5
	vpxor	%xmm14,%xmm6,%xmm6
	vpxor	%xmm15,%xmm7,%xmm7
	vmovdqu	%xmm1,(%rsi)
	vmovdqu	%xmm2,16(%rsi)
	vmovdqu	%xmm3,32(%rsi)
	vmovdqu	%xmm4,48(%rsi)
	vmovdqu	%xmm5,64(%rsi)
	vmovdqu	%xmm6,80(%rsi)
	addq	$0x70,%rsi
	vmovdqa64	%xmm16,%xmm0
	vmovdqa	%xmm7,%xmm8
	jmp	L$_steal_cipher_amivrujEyduiFoi

L$_done_7_amivrujEyduiFoi:
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vpxor	%xmm12,%xmm4,%xmm4
	vpxor	%xmm13,%xmm5,%xmm5
	vpxor	%xmm14,%xmm6,%xmm6
	vpxor	%xmm15,%xmm7,%xmm7
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vpxor	%xmm0,%xmm2,%xmm2
	vpxor	%xmm0,%xmm3,%xmm3
	vpxor	%xmm0,%xmm4,%xmm4
	vpxor	%xmm0,%xmm5,%xmm5
	vpxor	%xmm0,%xmm6,%xmm6
	vpxor	%xmm0,%xmm7,%xmm7
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
.byte	98,242,69,8,222,248
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,223,200
.byte	98,242,109,8,223,208
.byte	98,242,101,8,223,216
.byte	98,242,93,8,223,224
.byte	98,242,85,8,223,232
.byte	98,242,77,8,223,240
.byte	98,242,69,8,223,248
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vpxor	%xmm12,%xmm4,%xmm4
	vpxor	%xmm13,%xmm5,%xmm5
	vpxor	%xmm14,%xmm6,%xmm6
	vpxor	%xmm15,%xmm7,%xmm7
	vmovdqu	%xmm1,(%rsi)
	vmovdqu	%xmm2,16(%rsi)
	vmovdqu	%xmm3,32(%rsi)
	vmovdqu	%xmm4,48(%rsi)
	vmovdqu	%xmm5,64(%rsi)
	vmovdqu	%xmm6,80(%rsi)
	addq	$0x70,%rsi
	vmovdqa	%xmm7,%xmm8
	jmp	L$_done_amivrujEyduiFoi

L$_num_blocks_is_6_amivrujEyduiFoi:
	vmovdqa	0(%rsp),%xmm9
	movq	0(%rsp),%rax
	movq	8(%rsp),%rbx
	vmovdqu	0(%rdi),%xmm1
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,16(%rsp)
	movq	%rbx,24(%rsp)
	vmovdqa	16(%rsp),%xmm10
	vmovdqu	16(%rdi),%xmm2
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,32(%rsp)
	movq	%rbx,40(%rsp)
	vmovdqa	32(%rsp),%xmm11
	vmovdqu	32(%rdi),%xmm3
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,48(%rsp)
	movq	%rbx,56(%rsp)
	vmovdqa	48(%rsp),%xmm12
	vmovdqu	48(%rdi),%xmm4
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,64(%rsp)
	movq	%rbx,72(%rsp)
	vmovdqa	64(%rsp),%xmm13
	vmovdqu	64(%rdi),%xmm5
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,80(%rsp)
	movq	%rbx,88(%rsp)
	vmovdqa	80(%rsp),%xmm14
	vmovdqu	80(%rdi),%xmm6
	addq	$0x60,%rdi
	andq	$0xf,%rdx
	je	L$_done_6_amivrujEyduiFoi

L$_steal_cipher_6_amivrujEyduiFoi:
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,16(%rsp)
	movq	%rbx,24(%rsp)
	vmovdqa64	%xmm14,%xmm15
	vmovdqa	16(%rsp),%xmm14
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vpxor	%xmm12,%xmm4,%xmm4
	vpxor	%xmm13,%xmm5,%xmm5
	vpxor	%xmm14,%xmm6,%xmm6
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vpxor	%xmm0,%xmm2,%xmm2
	vpxor	%xmm0,%xmm3,%xmm3
	vpxor	%xmm0,%xmm4,%xmm4
	vpxor	%xmm0,%xmm5,%xmm5
	vpxor	%xmm0,%xmm6,%xmm6
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,223,200
.byte	98,242,109,8,223,208
.byte	98,242,101,8,223,216
.byte	98,242,93,8,223,224
.byte	98,242,85,8,223,232
.byte	98,242,77,8,223,240
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vpxor	%xmm12,%xmm4,%xmm4
	vpxor	%xmm13,%xmm5,%xmm5
	vpxor	%xmm14,%xmm6,%xmm6
	vmovdqu	%xmm1,(%rsi)
	vmovdqu	%xmm2,16(%rsi)
	vmovdqu	%xmm3,32(%rsi)
	vmovdqu	%xmm4,48(%rsi)
	vmovdqu	%xmm5,64(%rsi)
	addq	$0x60,%rsi
	vmovdqa	%xmm15,%xmm0
	vmovdqa	%xmm6,%xmm8
	jmp	L$_steal_cipher_amivrujEyduiFoi

L$_done_6_amivrujEyduiFoi:
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vpxor	%xmm12,%xmm4,%xmm4
	vpxor	%xmm13,%xmm5,%xmm5
	vpxor	%xmm14,%xmm6,%xmm6
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vpxor	%xmm0,%xmm2,%xmm2
	vpxor	%xmm0,%xmm3,%xmm3
	vpxor	%xmm0,%xmm4,%xmm4
	vpxor	%xmm0,%xmm5,%xmm5
	vpxor	%xmm0,%xmm6,%xmm6
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
.byte	98,242,77,8,222,240
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,223,200
.byte	98,242,109,8,223,208
.byte	98,242,101,8,223,216
.byte	98,242,93,8,223,224
.byte	98,242,85,8,223,232
.byte	98,242,77,8,223,240
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vpxor	%xmm12,%xmm4,%xmm4
	vpxor	%xmm13,%xmm5,%xmm5
	vpxor	%xmm14,%xmm6,%xmm6
	vmovdqu	%xmm1,(%rsi)
	vmovdqu	%xmm2,16(%rsi)
	vmovdqu	%xmm3,32(%rsi)
	vmovdqu	%xmm4,48(%rsi)
	vmovdqu	%xmm5,64(%rsi)
	addq	$0x60,%rsi
	vmovdqa	%xmm6,%xmm8
	jmp	L$_done_amivrujEyduiFoi

L$_num_blocks_is_5_amivrujEyduiFoi:
	vmovdqa	0(%rsp),%xmm9
	movq	0(%rsp),%rax
	movq	8(%rsp),%rbx
	vmovdqu	0(%rdi),%xmm1
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,16(%rsp)
	movq	%rbx,24(%rsp)
	vmovdqa	16(%rsp),%xmm10
	vmovdqu	16(%rdi),%xmm2
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,32(%rsp)
	movq	%rbx,40(%rsp)
	vmovdqa	32(%rsp),%xmm11
	vmovdqu	32(%rdi),%xmm3
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,48(%rsp)
	movq	%rbx,56(%rsp)
	vmovdqa	48(%rsp),%xmm12
	vmovdqu	48(%rdi),%xmm4
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,64(%rsp)
	movq	%rbx,72(%rsp)
	vmovdqa	64(%rsp),%xmm13
	vmovdqu	64(%rdi),%xmm5
	addq	$0x50,%rdi
	andq	$0xf,%rdx
	je	L$_done_5_amivrujEyduiFoi

L$_steal_cipher_5_amivrujEyduiFoi:
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,16(%rsp)
	movq	%rbx,24(%rsp)
	vmovdqa64	%xmm13,%xmm14
	vmovdqa	16(%rsp),%xmm13
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vpxor	%xmm12,%xmm4,%xmm4
	vpxor	%xmm13,%xmm5,%xmm5
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vpxor	%xmm0,%xmm2,%xmm2
	vpxor	%xmm0,%xmm3,%xmm3
	vpxor	%xmm0,%xmm4,%xmm4
	vpxor	%xmm0,%xmm5,%xmm5
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,223,200
.byte	98,242,109,8,223,208
.byte	98,242,101,8,223,216
.byte	98,242,93,8,223,224
.byte	98,242,85,8,223,232
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vpxor	%xmm12,%xmm4,%xmm4
	vpxor	%xmm13,%xmm5,%xmm5
	vmovdqu	%xmm1,(%rsi)
	vmovdqu	%xmm2,16(%rsi)
	vmovdqu	%xmm3,32(%rsi)
	vmovdqu	%xmm4,48(%rsi)
	addq	$0x50,%rsi
	vmovdqa	%xmm14,%xmm0
	vmovdqa	%xmm5,%xmm8
	jmp	L$_steal_cipher_amivrujEyduiFoi

L$_done_5_amivrujEyduiFoi:
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vpxor	%xmm12,%xmm4,%xmm4
	vpxor	%xmm13,%xmm5,%xmm5
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vpxor	%xmm0,%xmm2,%xmm2
	vpxor	%xmm0,%xmm3,%xmm3
	vpxor	%xmm0,%xmm4,%xmm4
	vpxor	%xmm0,%xmm5,%xmm5
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
.byte	98,242,85,8,222,232
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,223,200
.byte	98,242,109,8,223,208
.byte	98,242,101,8,223,216
.byte	98,242,93,8,223,224
.byte	98,242,85,8,223,232
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vpxor	%xmm12,%xmm4,%xmm4
	vpxor	%xmm13,%xmm5,%xmm5
	vmovdqu	%xmm1,(%rsi)
	vmovdqu	%xmm2,16(%rsi)
	vmovdqu	%xmm3,32(%rsi)
	vmovdqu	%xmm4,48(%rsi)
	addq	$0x50,%rsi
	vmovdqa	%xmm5,%xmm8
	jmp	L$_done_amivrujEyduiFoi

L$_num_blocks_is_4_amivrujEyduiFoi:
	vmovdqa	0(%rsp),%xmm9
	movq	0(%rsp),%rax
	movq	8(%rsp),%rbx
	vmovdqu	0(%rdi),%xmm1
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,16(%rsp)
	movq	%rbx,24(%rsp)
	vmovdqa	16(%rsp),%xmm10
	vmovdqu	16(%rdi),%xmm2
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,32(%rsp)
	movq	%rbx,40(%rsp)
	vmovdqa	32(%rsp),%xmm11
	vmovdqu	32(%rdi),%xmm3
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,48(%rsp)
	movq	%rbx,56(%rsp)
	vmovdqa	48(%rsp),%xmm12
	vmovdqu	48(%rdi),%xmm4
	addq	$0x40,%rdi
	andq	$0xf,%rdx
	je	L$_done_4_amivrujEyduiFoi

L$_steal_cipher_4_amivrujEyduiFoi:
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,16(%rsp)
	movq	%rbx,24(%rsp)
	vmovdqa64	%xmm12,%xmm13
	vmovdqa	16(%rsp),%xmm12
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vpxor	%xmm12,%xmm4,%xmm4
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vpxor	%xmm0,%xmm2,%xmm2
	vpxor	%xmm0,%xmm3,%xmm3
	vpxor	%xmm0,%xmm4,%xmm4
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,223,200
.byte	98,242,109,8,223,208
.byte	98,242,101,8,223,216
.byte	98,242,93,8,223,224
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vpxor	%xmm12,%xmm4,%xmm4
	vmovdqu	%xmm1,(%rsi)
	vmovdqu	%xmm2,16(%rsi)
	vmovdqu	%xmm3,32(%rsi)
	addq	$0x40,%rsi
	vmovdqa	%xmm13,%xmm0
	vmovdqa	%xmm4,%xmm8
	jmp	L$_steal_cipher_amivrujEyduiFoi

L$_done_4_amivrujEyduiFoi:
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vpxor	%xmm12,%xmm4,%xmm4
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vpxor	%xmm0,%xmm2,%xmm2
	vpxor	%xmm0,%xmm3,%xmm3
	vpxor	%xmm0,%xmm4,%xmm4
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
.byte	98,242,93,8,222,224
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,223,200
.byte	98,242,109,8,223,208
.byte	98,242,101,8,223,216
.byte	98,242,93,8,223,224
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vpxor	%xmm12,%xmm4,%xmm4
	vmovdqu	%xmm1,(%rsi)
	vmovdqu	%xmm2,16(%rsi)
	vmovdqu	%xmm3,32(%rsi)
	addq	$0x40,%rsi
	vmovdqa	%xmm4,%xmm8
	jmp	L$_done_amivrujEyduiFoi

L$_num_blocks_is_3_amivrujEyduiFoi:
	vmovdqa	0(%rsp),%xmm9
	movq	0(%rsp),%rax
	movq	8(%rsp),%rbx
	vmovdqu	0(%rdi),%xmm1
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,16(%rsp)
	movq	%rbx,24(%rsp)
	vmovdqa	16(%rsp),%xmm10
	vmovdqu	16(%rdi),%xmm2
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,32(%rsp)
	movq	%rbx,40(%rsp)
	vmovdqa	32(%rsp),%xmm11
	vmovdqu	32(%rdi),%xmm3
	addq	$0x30,%rdi
	andq	$0xf,%rdx
	je	L$_done_3_amivrujEyduiFoi

L$_steal_cipher_3_amivrujEyduiFoi:
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,16(%rsp)
	movq	%rbx,24(%rsp)
	vmovdqa64	%xmm11,%xmm12
	vmovdqa	16(%rsp),%xmm11
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vpxor	%xmm0,%xmm2,%xmm2
	vpxor	%xmm0,%xmm3,%xmm3
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,223,200
.byte	98,242,109,8,223,208
.byte	98,242,101,8,223,216
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vmovdqu	%xmm1,(%rsi)
	vmovdqu	%xmm2,16(%rsi)
	addq	$0x30,%rsi
	vmovdqa	%xmm12,%xmm0
	vmovdqa	%xmm3,%xmm8
	jmp	L$_steal_cipher_amivrujEyduiFoi

L$_done_3_amivrujEyduiFoi:
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vpxor	%xmm0,%xmm2,%xmm2
	vpxor	%xmm0,%xmm3,%xmm3
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
.byte	98,242,101,8,222,216
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,223,200
.byte	98,242,109,8,223,208
.byte	98,242,101,8,223,216
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vpxor	%xmm11,%xmm3,%xmm3
	vmovdqu	%xmm1,(%rsi)
	vmovdqu	%xmm2,16(%rsi)
	addq	$0x30,%rsi
	vmovdqa	%xmm3,%xmm8
	jmp	L$_done_amivrujEyduiFoi

L$_num_blocks_is_2_amivrujEyduiFoi:
	vmovdqa	0(%rsp),%xmm9
	movq	0(%rsp),%rax
	movq	8(%rsp),%rbx
	vmovdqu	0(%rdi),%xmm1
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,16(%rsp)
	movq	%rbx,24(%rsp)
	vmovdqa	16(%rsp),%xmm10
	vmovdqu	16(%rdi),%xmm2
	addq	$0x20,%rdi
	andq	$0xf,%rdx
	je	L$_done_2_amivrujEyduiFoi

L$_steal_cipher_2_amivrujEyduiFoi:
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,16(%rsp)
	movq	%rbx,24(%rsp)
	vmovdqa64	%xmm10,%xmm11
	vmovdqa	16(%rsp),%xmm10
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vpxor	%xmm0,%xmm2,%xmm2
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,223,200
.byte	98,242,109,8,223,208
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vmovdqu	%xmm1,(%rsi)
	addq	$0x20,%rsi
	vmovdqa	%xmm11,%xmm0
	vmovdqa	%xmm2,%xmm8
	jmp	L$_steal_cipher_amivrujEyduiFoi

L$_done_2_amivrujEyduiFoi:
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vpxor	%xmm0,%xmm2,%xmm2
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,222,200
.byte	98,242,109,8,222,208
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,223,200
.byte	98,242,109,8,223,208
	vpxor	%xmm9,%xmm1,%xmm1
	vpxor	%xmm10,%xmm2,%xmm2
	vmovdqu	%xmm1,(%rsi)
	addq	$0x20,%rsi
	vmovdqa	%xmm2,%xmm8
	jmp	L$_done_amivrujEyduiFoi

L$_num_blocks_is_1_amivrujEyduiFoi:
	vmovdqa	0(%rsp),%xmm9
	movq	0(%rsp),%rax
	movq	8(%rsp),%rbx
	vmovdqu	0(%rdi),%xmm1
	addq	$0x10,%rdi
	andq	$0xf,%rdx
	je	L$_done_1_amivrujEyduiFoi

L$_steal_cipher_1_amivrujEyduiFoi:
	xorq	%r11,%r11
	shlq	$1,%rax
	adcq	%rbx,%rbx
	cmovcq	%r10,%r11
	xorq	%r11,%rax
	movq	%rax,16(%rsp)
	movq	%rbx,24(%rsp)
	vmovdqa64	%xmm9,%xmm10
	vmovdqa	16(%rsp),%xmm9
	vpxor	%xmm9,%xmm1,%xmm1
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,223,200
	vpxor	%xmm9,%xmm1,%xmm1
	addq	$0x10,%rsi
	vmovdqa	%xmm10,%xmm0
	vmovdqa	%xmm1,%xmm8
	jmp	L$_steal_cipher_amivrujEyduiFoi

L$_done_1_amivrujEyduiFoi:
	vpxor	%xmm9,%xmm1,%xmm1
	vmovdqa	128(%rsp),%xmm0
	vpxor	%xmm0,%xmm1,%xmm1
	vmovdqa	144(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	160(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	176(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	192(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	208(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	224(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	240(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	256(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	272(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	288(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	304(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	320(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	336(%rsp),%xmm0
.byte	98,242,117,8,222,200
	vmovdqa	352(%rsp),%xmm0
.byte	98,242,117,8,223,200
	vpxor	%xmm9,%xmm1,%xmm1
	addq	$0x10,%rsi
	vmovdqa	%xmm1,%xmm8
	jmp	L$_done_amivrujEyduiFoi

.section	__DATA,__const
.p2align	4

vpshufb_shf_table:
.quad	0x8786858483828100, 0x8f8e8d8c8b8a8988
.quad	0x0706050403020100, 0x000e0d0c0b0a0908

mask1:
.quad	0x8080808080808080, 0x8080808080808080

const_dq3210:
.quad	0, 0, 1, 1, 2, 2, 3, 3
const_dq5678:
.quad	8, 8, 7, 7, 6, 6, 5, 5
const_dq7654:
.quad	4, 4, 5, 5, 6, 6, 7, 7
const_dq1234:
.quad	4, 4, 3, 3, 2, 2, 1, 1

shufb_15_7:
.byte	15, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 7, 0xff, 0xff
.byte	0xff, 0xff, 0xff, 0xff, 0xff

.text	
#endif
#endif
