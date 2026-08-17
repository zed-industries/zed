; This file is generated from a similarly-named Perl script in the BoringSSL
; source tree. Do not edit by hand.

%ifidn __OUTPUT_FORMAT__, win64
default	rel
%define XMMWORD
%define YMMWORD
%define ZMMWORD
%define _CET_ENDBR

%include "openssl/boringssl_prefix_symbols_nasm.inc"
%ifndef MY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX
section	.text code align=64

global	aes_hw_xts_encrypt_avx512


ALIGN	32
aes_hw_xts_encrypt_avx512:

DB	243,15,30,250
	push	rbp
	mov	rbp,rsp
	sub	rsp,552
	and	rsp,0xffffffffffffffc0
	mov	QWORD[528+rsp],rbx
	mov	QWORD[((528 + 8))+rsp],rdi
	mov	QWORD[((528 + 16))+rsp],rsi
	vmovdqa	XMMWORD[(368 + 0)+rsp],xmm6
	vmovdqa	XMMWORD[(368 + 16)+rsp],xmm7
	vmovdqa	XMMWORD[(368 + 32)+rsp],xmm8
	vmovdqa	XMMWORD[(368 + 48)+rsp],xmm9
	vmovdqa	XMMWORD[(368 + 64)+rsp],xmm10
	vmovdqa	XMMWORD[(368 + 80)+rsp],xmm11
	vmovdqa	XMMWORD[(368 + 96)+rsp],xmm12
	vmovdqa	XMMWORD[(368 + 112)+rsp],xmm13
	vmovdqa	XMMWORD[(368 + 128)+rsp],xmm14
	vmovdqa	XMMWORD[(368 + 144)+rsp],xmm15
	mov	rdi,0x87
	vmovdqu	xmm1,XMMWORD[r11]
	vpxor	xmm4,xmm4,xmm4
	vmovdqu	xmm0,XMMWORD[r10]
	vpxor	xmm1,xmm1,xmm0

	vmovdqu	xmm2,XMMWORD[r9]
	vmovdqa	XMMWORD[128+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[16+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[16+r9]
	vmovdqa	XMMWORD[144+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[32+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[32+r9]
	vmovdqa	XMMWORD[160+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[48+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[48+r9]
	vmovdqa	XMMWORD[176+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[64+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[64+r9]
	vmovdqa	XMMWORD[192+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[80+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[80+r9]
	vmovdqa	XMMWORD[208+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[96+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[96+r9]
	vmovdqa	XMMWORD[224+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[112+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[112+r9]
	vmovdqa	XMMWORD[240+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[128+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[128+r9]
	vmovdqa	XMMWORD[256+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[144+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[144+r9]
	vmovdqa	XMMWORD[272+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[160+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[160+r9]
	vmovdqa	XMMWORD[288+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[176+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[176+r9]
	vmovdqa	XMMWORD[304+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[192+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[192+r9]
	vmovdqa	XMMWORD[320+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[208+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[208+r9]
	vmovdqa	XMMWORD[336+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[224+r10]
	DB	98,242,117,8,221,200

	vmovdqu	xmm2,XMMWORD[224+r9]
	vmovdqa	XMMWORD[352+rsp],xmm2

	vmovdqa	XMMWORD[rsp],xmm1
	mov	QWORD[((8 + 40))+rbp],rcx
	mov	QWORD[((8 + 48))+rbp],rdx

	cmp	r8,0x80
	jl	NEAR $L$_less_than_128_bytes_hEgxyDlCngwrfFe
	vpbroadcastq	zmm25,rdi
	cmp	r8,0x100
	jge	NEAR $L$_start_by16_hEgxyDlCngwrfFe
	cmp	r8,0x80
	jge	NEAR $L$_start_by8_hEgxyDlCngwrfFe

$L$_do_n_blocks_hEgxyDlCngwrfFe:
	cmp	r8,0x0
	je	NEAR $L$_ret_hEgxyDlCngwrfFe
	cmp	r8,0x70
	jge	NEAR $L$_remaining_num_blocks_is_7_hEgxyDlCngwrfFe
	cmp	r8,0x60
	jge	NEAR $L$_remaining_num_blocks_is_6_hEgxyDlCngwrfFe
	cmp	r8,0x50
	jge	NEAR $L$_remaining_num_blocks_is_5_hEgxyDlCngwrfFe
	cmp	r8,0x40
	jge	NEAR $L$_remaining_num_blocks_is_4_hEgxyDlCngwrfFe
	cmp	r8,0x30
	jge	NEAR $L$_remaining_num_blocks_is_3_hEgxyDlCngwrfFe
	cmp	r8,0x20
	jge	NEAR $L$_remaining_num_blocks_is_2_hEgxyDlCngwrfFe
	cmp	r8,0x10
	jge	NEAR $L$_remaining_num_blocks_is_1_hEgxyDlCngwrfFe
	vmovdqa	xmm8,xmm0
	vmovdqa	xmm0,xmm9
	jmp	NEAR $L$_steal_cipher_hEgxyDlCngwrfFe

$L$_remaining_num_blocks_is_7_hEgxyDlCngwrfFe:
	mov	r10,0xffffffffffffffff
	shr	r10,0x10
	kmovq	k1,r10
	vmovdqu8	zmm1,ZMMWORD[rcx]
	vmovdqu8	zmm2{k1},[64+rcx]
	add	rcx,0x70

	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vbroadcasti32x4	zmm0,ZMMWORD[128+rsp]
	vpxorq	zmm1,zmm1,zmm0
	vpxorq	zmm2,zmm2,zmm0
	vbroadcasti32x4	zmm0,ZMMWORD[144+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[160+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[176+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208

	vbroadcasti32x4	zmm0,ZMMWORD[192+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[208+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[224+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[240+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[256+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[272+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[288+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[304+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[320+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[336+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[352+rsp]
	DB	98,242,117,72,221,200
	DB	98,242,109,72,221,208


	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vmovdqa32	zmm9,zmm15
	vmovdqa32	zmm10,zmm16
	vmovdqu8	ZMMWORD[rdx],zmm1
	vmovdqu8	ZMMWORD[64+rdx]{k1},zmm2
	add	rdx,0x70
	vextracti32x4	xmm8,zmm2,0x2
	vextracti32x4	xmm0,zmm10,0x3
	and	r8,0xf
	je	NEAR $L$_ret_hEgxyDlCngwrfFe
	jmp	NEAR $L$_steal_cipher_hEgxyDlCngwrfFe

$L$_remaining_num_blocks_is_6_hEgxyDlCngwrfFe:
	vmovdqu8	zmm1,ZMMWORD[rcx]
	vmovdqu8	ymm2,YMMWORD[64+rcx]
	add	rcx,0x60

	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vbroadcasti32x4	zmm0,ZMMWORD[128+rsp]
	vpxorq	zmm1,zmm1,zmm0
	vpxorq	zmm2,zmm2,zmm0
	vbroadcasti32x4	zmm0,ZMMWORD[144+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[160+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[176+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208

	vbroadcasti32x4	zmm0,ZMMWORD[192+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[208+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[224+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[240+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[256+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[272+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[288+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[304+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[320+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[336+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[352+rsp]
	DB	98,242,117,72,221,200
	DB	98,242,109,72,221,208


	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vmovdqa32	zmm9,zmm15
	vmovdqa32	zmm10,zmm16
	vmovdqu8	ZMMWORD[rdx],zmm1
	vmovdqu8	YMMWORD[64+rdx],ymm2
	add	rdx,0x60
	vextracti32x4	xmm8,zmm2,0x1
	vextracti32x4	xmm0,zmm10,0x2
	and	r8,0xf
	je	NEAR $L$_ret_hEgxyDlCngwrfFe
	jmp	NEAR $L$_steal_cipher_hEgxyDlCngwrfFe

$L$_remaining_num_blocks_is_5_hEgxyDlCngwrfFe:
	vmovdqu8	zmm1,ZMMWORD[rcx]
	vmovdqu	xmm2,XMMWORD[64+rcx]
	add	rcx,0x50

	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vbroadcasti32x4	zmm0,ZMMWORD[128+rsp]
	vpxorq	zmm1,zmm1,zmm0
	vpxorq	zmm2,zmm2,zmm0
	vbroadcasti32x4	zmm0,ZMMWORD[144+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[160+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[176+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208

	vbroadcasti32x4	zmm0,ZMMWORD[192+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[208+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[224+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[240+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[256+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[272+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[288+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[304+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[320+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[336+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[352+rsp]
	DB	98,242,117,72,221,200
	DB	98,242,109,72,221,208


	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vmovdqa32	zmm9,zmm15
	vmovdqa32	zmm10,zmm16
	vmovdqu8	ZMMWORD[rdx],zmm1
	vmovdqu	XMMWORD[64+rdx],xmm2
	add	rdx,0x50
	vmovdqa	xmm8,xmm2
	vextracti32x4	xmm0,zmm10,0x1
	and	r8,0xf
	je	NEAR $L$_ret_hEgxyDlCngwrfFe
	jmp	NEAR $L$_steal_cipher_hEgxyDlCngwrfFe

$L$_remaining_num_blocks_is_4_hEgxyDlCngwrfFe:
	vmovdqu8	zmm1,ZMMWORD[rcx]
	add	rcx,0x40

	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vbroadcasti32x4	zmm0,ZMMWORD[128+rsp]
	vpxorq	zmm1,zmm1,zmm0
	vpxorq	zmm2,zmm2,zmm0
	vbroadcasti32x4	zmm0,ZMMWORD[144+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[160+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[176+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208

	vbroadcasti32x4	zmm0,ZMMWORD[192+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[208+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[224+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[240+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[256+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[272+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[288+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[304+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[320+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[336+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[352+rsp]
	DB	98,242,117,72,221,200
	DB	98,242,109,72,221,208


	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vmovdqa32	zmm9,zmm15
	vmovdqa32	zmm10,zmm16
	vmovdqu8	ZMMWORD[rdx],zmm1
	add	rdx,0x40
	vextracti32x4	xmm8,zmm1,0x3
	vextracti32x4	xmm0,zmm10,0x0
	and	r8,0xf
	je	NEAR $L$_ret_hEgxyDlCngwrfFe
	jmp	NEAR $L$_steal_cipher_hEgxyDlCngwrfFe
$L$_remaining_num_blocks_is_3_hEgxyDlCngwrfFe:
	vextracti32x4	xmm10,zmm9,0x1
	vextracti32x4	xmm11,zmm9,0x2
	vmovdqu	xmm1,XMMWORD[rcx]
	vmovdqu	xmm2,XMMWORD[16+rcx]
	vmovdqu	xmm3,XMMWORD[32+rcx]
	add	rcx,0x30
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vpxor	xmm2,xmm2,xmm0
	vpxor	xmm3,xmm3,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,221,200
	DB	98,242,109,8,221,208
	DB	98,242,101,8,221,216
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vmovdqu	XMMWORD[rdx],xmm1
	vmovdqu	XMMWORD[16+rdx],xmm2
	vmovdqu	XMMWORD[32+rdx],xmm3
	add	rdx,0x30
	vmovdqa	xmm8,xmm3
	vextracti32x4	xmm0,zmm9,0x3
	and	r8,0xf
	je	NEAR $L$_ret_hEgxyDlCngwrfFe
	jmp	NEAR $L$_steal_cipher_hEgxyDlCngwrfFe
$L$_remaining_num_blocks_is_2_hEgxyDlCngwrfFe:
	vextracti32x4	xmm10,zmm9,0x1
	vmovdqu	xmm1,XMMWORD[rcx]
	vmovdqu	xmm2,XMMWORD[16+rcx]
	add	rcx,0x20
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vpxor	xmm2,xmm2,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,221,200
	DB	98,242,109,8,221,208
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vmovdqu	XMMWORD[rdx],xmm1
	vmovdqu	XMMWORD[16+rdx],xmm2
	add	rdx,0x20
	vmovdqa	xmm8,xmm2
	vextracti32x4	xmm0,zmm9,0x2
	and	r8,0xf
	je	NEAR $L$_ret_hEgxyDlCngwrfFe
	jmp	NEAR $L$_steal_cipher_hEgxyDlCngwrfFe
$L$_remaining_num_blocks_is_1_hEgxyDlCngwrfFe:
	vmovdqu	xmm1,XMMWORD[rcx]
	add	rcx,0x10
	vpxor	xmm1,xmm1,xmm9
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,221,200
	vpxor	xmm1,xmm1,xmm9
	vmovdqu	XMMWORD[rdx],xmm1
	add	rdx,0x10
	vmovdqa	xmm8,xmm1
	vextracti32x4	xmm0,zmm9,0x1
	and	r8,0xf
	je	NEAR $L$_ret_hEgxyDlCngwrfFe
	jmp	NEAR $L$_steal_cipher_hEgxyDlCngwrfFe

$L$_start_by16_hEgxyDlCngwrfFe:
	vbroadcasti32x4	zmm0,ZMMWORD[rsp]
	vbroadcasti32x4	zmm8,ZMMWORD[shufb_15_7]
	mov	r10,0xaa
	kmovq	k2,r10
	vpshufb	zmm1,zmm0,zmm8
	vpsllvq	zmm4,zmm0,ZMMWORD[const_dq3210]
	vpsrlvq	zmm2,zmm1,ZMMWORD[const_dq5678]
	DB	98,147,109,72,68,217,0
	vpxorq	zmm4{k2},zmm4,zmm2
	vpxord	zmm9,zmm3,zmm4
	vpsllvq	zmm5,zmm0,ZMMWORD[const_dq7654]
	vpsrlvq	zmm6,zmm1,ZMMWORD[const_dq1234]
	DB	98,147,77,72,68,249,0
	vpxorq	zmm5{k2},zmm5,zmm6
	vpxord	zmm10,zmm7,zmm5
	vpsrldq	zmm13,zmm9,0xf
	DB	98,19,21,72,68,241,0
	vpslldq	zmm11,zmm9,0x1
	vpxord	zmm11,zmm11,zmm14
	vpsrldq	zmm15,zmm10,0xf
	DB	98,131,5,72,68,193,0
	vpslldq	zmm12,zmm10,0x1
	vpxord	zmm12,zmm12,zmm16

$L$_main_loop_run_16_hEgxyDlCngwrfFe:
	vmovdqu8	zmm1,ZMMWORD[rcx]
	vmovdqu8	zmm2,ZMMWORD[64+rcx]
	vmovdqu8	zmm3,ZMMWORD[128+rcx]
	vmovdqu8	zmm4,ZMMWORD[192+rcx]
	add	rcx,0x100
	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10
	vpxorq	zmm3,zmm3,zmm11
	vpxorq	zmm4,zmm4,zmm12
	vbroadcasti32x4	zmm0,ZMMWORD[128+rsp]
	vpxorq	zmm1,zmm1,zmm0
	vpxorq	zmm2,zmm2,zmm0
	vpxorq	zmm3,zmm3,zmm0
	vpxorq	zmm4,zmm4,zmm0
	vpsrldq	zmm13,zmm11,0xf
	DB	98,19,21,72,68,241,0
	vpslldq	zmm15,zmm11,0x1
	vpxord	zmm15,zmm15,zmm14
	vbroadcasti32x4	zmm0,ZMMWORD[144+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208
	DB	98,242,101,72,220,216
	DB	98,242,93,72,220,224
	vbroadcasti32x4	zmm0,ZMMWORD[160+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208
	DB	98,242,101,72,220,216
	DB	98,242,93,72,220,224
	vbroadcasti32x4	zmm0,ZMMWORD[176+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208
	DB	98,242,101,72,220,216
	DB	98,242,93,72,220,224
	vpsrldq	zmm13,zmm12,0xf
	DB	98,19,21,72,68,241,0
	vpslldq	zmm16,zmm12,0x1
	vpxord	zmm16,zmm16,zmm14
	vbroadcasti32x4	zmm0,ZMMWORD[192+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208
	DB	98,242,101,72,220,216
	DB	98,242,93,72,220,224
	vbroadcasti32x4	zmm0,ZMMWORD[208+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208
	DB	98,242,101,72,220,216
	DB	98,242,93,72,220,224
	vbroadcasti32x4	zmm0,ZMMWORD[224+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208
	DB	98,242,101,72,220,216
	DB	98,242,93,72,220,224
	vpsrldq	zmm13,zmm15,0xf
	DB	98,19,21,72,68,241,0
	vpslldq	zmm17,zmm15,0x1
	vpxord	zmm17,zmm17,zmm14
	vbroadcasti32x4	zmm0,ZMMWORD[240+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208
	DB	98,242,101,72,220,216
	DB	98,242,93,72,220,224
	vbroadcasti32x4	zmm0,ZMMWORD[256+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208
	DB	98,242,101,72,220,216
	DB	98,242,93,72,220,224
	vbroadcasti32x4	zmm0,ZMMWORD[272+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208
	DB	98,242,101,72,220,216
	DB	98,242,93,72,220,224
	vpsrldq	zmm13,zmm16,0xf
	DB	98,19,21,72,68,241,0
	vpslldq	zmm18,zmm16,0x1
	vpxord	zmm18,zmm18,zmm14
	vbroadcasti32x4	zmm0,ZMMWORD[288+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208
	DB	98,242,101,72,220,216
	DB	98,242,93,72,220,224
	vbroadcasti32x4	zmm0,ZMMWORD[304+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208
	DB	98,242,101,72,220,216
	DB	98,242,93,72,220,224
	vbroadcasti32x4	zmm0,ZMMWORD[320+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208
	DB	98,242,101,72,220,216
	DB	98,242,93,72,220,224
	vbroadcasti32x4	zmm0,ZMMWORD[336+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208
	DB	98,242,101,72,220,216
	DB	98,242,93,72,220,224
	vbroadcasti32x4	zmm0,ZMMWORD[352+rsp]
	DB	98,242,117,72,221,200
	DB	98,242,109,72,221,208
	DB	98,242,101,72,221,216
	DB	98,242,93,72,221,224
	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10
	vpxorq	zmm3,zmm3,zmm11
	vpxorq	zmm4,zmm4,zmm12

	vmovdqa32	zmm9,zmm15
	vmovdqa32	zmm10,zmm16
	vmovdqa32	zmm11,zmm17
	vmovdqa32	zmm12,zmm18
	vmovdqu8	ZMMWORD[rdx],zmm1
	vmovdqu8	ZMMWORD[64+rdx],zmm2
	vmovdqu8	ZMMWORD[128+rdx],zmm3
	vmovdqu8	ZMMWORD[192+rdx],zmm4
	add	rdx,0x100
	sub	r8,0x100
	cmp	r8,0x100
	jge	NEAR $L$_main_loop_run_16_hEgxyDlCngwrfFe
	cmp	r8,0x80
	jge	NEAR $L$_main_loop_run_8_hEgxyDlCngwrfFe
	vextracti32x4	xmm0,zmm4,0x3
	jmp	NEAR $L$_do_n_blocks_hEgxyDlCngwrfFe

$L$_start_by8_hEgxyDlCngwrfFe:
	vbroadcasti32x4	zmm0,ZMMWORD[rsp]
	vbroadcasti32x4	zmm8,ZMMWORD[shufb_15_7]
	mov	r10,0xaa
	kmovq	k2,r10
	vpshufb	zmm1,zmm0,zmm8
	vpsllvq	zmm4,zmm0,ZMMWORD[const_dq3210]
	vpsrlvq	zmm2,zmm1,ZMMWORD[const_dq5678]
	DB	98,147,109,72,68,217,0
	vpxorq	zmm4{k2},zmm4,zmm2
	vpxord	zmm9,zmm3,zmm4
	vpsllvq	zmm5,zmm0,ZMMWORD[const_dq7654]
	vpsrlvq	zmm6,zmm1,ZMMWORD[const_dq1234]
	DB	98,147,77,72,68,249,0
	vpxorq	zmm5{k2},zmm5,zmm6
	vpxord	zmm10,zmm7,zmm5

$L$_main_loop_run_8_hEgxyDlCngwrfFe:
	vmovdqu8	zmm1,ZMMWORD[rcx]
	vmovdqu8	zmm2,ZMMWORD[64+rcx]
	add	rcx,0x80

	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vbroadcasti32x4	zmm0,ZMMWORD[128+rsp]
	vpxorq	zmm1,zmm1,zmm0
	vpxorq	zmm2,zmm2,zmm0
	vpsrldq	zmm13,zmm9,0xf
	DB	98,19,21,72,68,241,0
	vpslldq	zmm15,zmm9,0x1
	vpxord	zmm15,zmm15,zmm14
	vbroadcasti32x4	zmm0,ZMMWORD[144+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[160+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[176+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208
	vpsrldq	zmm13,zmm10,0xf
	DB	98,19,21,72,68,241,0
	vpslldq	zmm16,zmm10,0x1
	vpxord	zmm16,zmm16,zmm14

	vbroadcasti32x4	zmm0,ZMMWORD[192+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[208+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[224+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[240+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[256+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[272+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[288+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[304+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[320+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[336+rsp]
	DB	98,242,117,72,220,200
	DB	98,242,109,72,220,208


	vbroadcasti32x4	zmm0,ZMMWORD[352+rsp]
	DB	98,242,117,72,221,200
	DB	98,242,109,72,221,208


	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vmovdqa32	zmm9,zmm15
	vmovdqa32	zmm10,zmm16
	vmovdqu8	ZMMWORD[rdx],zmm1
	vmovdqu8	ZMMWORD[64+rdx],zmm2
	add	rdx,0x80
	sub	r8,0x80
	cmp	r8,0x80
	jge	NEAR $L$_main_loop_run_8_hEgxyDlCngwrfFe
	vextracti32x4	xmm0,zmm2,0x3
	jmp	NEAR $L$_do_n_blocks_hEgxyDlCngwrfFe

$L$_steal_cipher_next_hEgxyDlCngwrfFe:
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[rsp],rax
	mov	QWORD[8+rsp],rbx
	vmovdqa	xmm0,XMMWORD[rsp]

$L$_steal_cipher_hEgxyDlCngwrfFe:
	vmovdqa	xmm2,xmm8
	lea	rax,[vpshufb_shf_table]
	vmovdqu	xmm10,XMMWORD[r8*1+rax]
	vpshufb	xmm8,xmm8,xmm10
	vmovdqu	xmm3,XMMWORD[((-16))+r8*1+rcx]
	vmovdqu	XMMWORD[(-16)+r8*1+rdx],xmm8
	lea	rax,[vpshufb_shf_table]
	add	rax,16
	sub	rax,r8
	vmovdqu	xmm10,XMMWORD[rax]
	vpxor	xmm10,xmm10,XMMWORD[mask1]
	vpshufb	xmm3,xmm3,xmm10
	vpblendvb	xmm3,xmm3,xmm2,xmm10
	vpxor	xmm8,xmm3,xmm0
	vpxor	xmm8,xmm8,XMMWORD[128+rsp]
	DB	98,114,61,8,220,132,36,144,0,0,0
	DB	98,114,61,8,220,132,36,160,0,0,0
	DB	98,114,61,8,220,132,36,176,0,0,0
	DB	98,114,61,8,220,132,36,192,0,0,0
	DB	98,114,61,8,220,132,36,208,0,0,0
	DB	98,114,61,8,220,132,36,224,0,0,0
	DB	98,114,61,8,220,132,36,240,0,0,0
	DB	98,114,61,8,220,132,36,0,1,0,0
	DB	98,114,61,8,220,132,36,16,1,0,0
	DB	98,114,61,8,220,132,36,32,1,0,0
	DB	98,114,61,8,220,132,36,48,1,0,0
	DB	98,114,61,8,220,132,36,64,1,0,0
	DB	98,114,61,8,220,132,36,80,1,0,0
	DB	98,114,61,8,221,132,36,96,1,0,0
	vpxor	xmm8,xmm8,xmm0
	vmovdqu	XMMWORD[(-16)+rdx],xmm8
$L$_ret_hEgxyDlCngwrfFe:
	mov	rbx,QWORD[528+rsp]
	xor	r10,r10
	mov	QWORD[528+rsp],r10

	vpxorq	zmm0,zmm0,zmm0
	mov	rdi,QWORD[((528 + 8))+rsp]
	mov	QWORD[((528 + 8))+rsp],r10
	mov	rsi,QWORD[((528 + 16))+rsp]
	mov	QWORD[((528 + 16))+rsp],r10

	vmovdqa	xmm6,XMMWORD[((368 + 0))+rsp]
	vmovdqa	xmm7,XMMWORD[((368 + 16))+rsp]
	vmovdqa	xmm8,XMMWORD[((368 + 32))+rsp]
	vmovdqa	xmm9,XMMWORD[((368 + 48))+rsp]


	vmovdqa64	ZMMWORD[368+rsp],zmm0

	vmovdqa	xmm10,XMMWORD[((368 + 64))+rsp]
	vmovdqa	xmm11,XMMWORD[((368 + 80))+rsp]
	vmovdqa	xmm12,XMMWORD[((368 + 96))+rsp]
	vmovdqa	xmm13,XMMWORD[((368 + 112))+rsp]


	vmovdqa64	ZMMWORD[(368 + 64)+rsp],zmm0

	vmovdqa	xmm14,XMMWORD[((368 + 128))+rsp]
	vmovdqa	xmm15,XMMWORD[((368 + 144))+rsp]



	vmovdqa	YMMWORD[(368 + 128)+rsp],ymm0

	vmovdqa64	ZMMWORD[128+rsp],zmm0
	vmovdqa64	ZMMWORD[192+rsp],zmm0
	vmovdqa64	ZMMWORD[256+rsp],zmm0



	mov	r10,0x3f
	kmovq	k2,r10
	vmovdqa64	ZMMWORD[320+rsp]{k2},zmm0

	mov	rsp,rbp
	pop	rbp
	vzeroupper
	DB	0F3h,0C3h		;repret

$L$_less_than_128_bytes_hEgxyDlCngwrfFe:
	cmp	r8,0x10
	jb	NEAR $L$_ret_hEgxyDlCngwrfFe
	mov	r10,r8
	and	r10,0x70
	cmp	r10,0x60
	je	NEAR $L$_num_blocks_is_6_hEgxyDlCngwrfFe
	cmp	r10,0x50
	je	NEAR $L$_num_blocks_is_5_hEgxyDlCngwrfFe
	cmp	r10,0x40
	je	NEAR $L$_num_blocks_is_4_hEgxyDlCngwrfFe
	cmp	r10,0x30
	je	NEAR $L$_num_blocks_is_3_hEgxyDlCngwrfFe
	cmp	r10,0x20
	je	NEAR $L$_num_blocks_is_2_hEgxyDlCngwrfFe
	cmp	r10,0x10
	je	NEAR $L$_num_blocks_is_1_hEgxyDlCngwrfFe

$L$_num_blocks_is_7_hEgxyDlCngwrfFe:
	vmovdqa	xmm9,XMMWORD[rsp]
	mov	rax,QWORD[rsp]
	mov	rbx,QWORD[8+rsp]
	vmovdqu	xmm1,XMMWORD[rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[16+rsp],rax
	mov	QWORD[24+rsp],rbx
	vmovdqa	xmm10,XMMWORD[16+rsp]
	vmovdqu	xmm2,XMMWORD[16+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[32+rsp],rax
	mov	QWORD[40+rsp],rbx
	vmovdqa	xmm11,XMMWORD[32+rsp]
	vmovdqu	xmm3,XMMWORD[32+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[48+rsp],rax
	mov	QWORD[56+rsp],rbx
	vmovdqa	xmm12,XMMWORD[48+rsp]
	vmovdqu	xmm4,XMMWORD[48+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[64+rsp],rax
	mov	QWORD[72+rsp],rbx
	vmovdqa	xmm13,XMMWORD[64+rsp]
	vmovdqu	xmm5,XMMWORD[64+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[80+rsp],rax
	mov	QWORD[88+rsp],rbx
	vmovdqa	xmm14,XMMWORD[80+rsp]
	vmovdqu	xmm6,XMMWORD[80+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[96+rsp],rax
	mov	QWORD[104+rsp],rbx
	vmovdqa	xmm15,XMMWORD[96+rsp]
	vmovdqu	xmm7,XMMWORD[96+rcx]
	add	rcx,0x70
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vpxor	xmm4,xmm4,xmm12
	vpxor	xmm5,xmm5,xmm13
	vpxor	xmm6,xmm6,xmm14
	vpxor	xmm7,xmm7,xmm15
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vpxor	xmm2,xmm2,xmm0
	vpxor	xmm3,xmm3,xmm0
	vpxor	xmm4,xmm4,xmm0
	vpxor	xmm5,xmm5,xmm0
	vpxor	xmm6,xmm6,xmm0
	vpxor	xmm7,xmm7,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	DB	98,242,69,8,220,248
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	DB	98,242,69,8,220,248
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	DB	98,242,69,8,220,248
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	DB	98,242,69,8,220,248
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	DB	98,242,69,8,220,248
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	DB	98,242,69,8,220,248
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	DB	98,242,69,8,220,248
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	DB	98,242,69,8,220,248
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	DB	98,242,69,8,220,248
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	DB	98,242,69,8,220,248
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	DB	98,242,69,8,220,248
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	DB	98,242,69,8,220,248
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	DB	98,242,69,8,220,248
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,221,200
	DB	98,242,109,8,221,208
	DB	98,242,101,8,221,216
	DB	98,242,93,8,221,224
	DB	98,242,85,8,221,232
	DB	98,242,77,8,221,240
	DB	98,242,69,8,221,248
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vpxor	xmm4,xmm4,xmm12
	vpxor	xmm5,xmm5,xmm13
	vpxor	xmm6,xmm6,xmm14
	vpxor	xmm7,xmm7,xmm15
	vmovdqu	XMMWORD[rdx],xmm1
	vmovdqu	XMMWORD[16+rdx],xmm2
	vmovdqu	XMMWORD[32+rdx],xmm3
	vmovdqu	XMMWORD[48+rdx],xmm4
	vmovdqu	XMMWORD[64+rdx],xmm5
	vmovdqu	XMMWORD[80+rdx],xmm6
	vmovdqu	XMMWORD[96+rdx],xmm7
	add	rdx,0x70
	vmovdqa	xmm8,xmm7
	and	r8,0xf
	je	NEAR $L$_ret_hEgxyDlCngwrfFe
	jmp	NEAR $L$_steal_cipher_next_hEgxyDlCngwrfFe

$L$_num_blocks_is_6_hEgxyDlCngwrfFe:
	vmovdqa	xmm9,XMMWORD[rsp]
	mov	rax,QWORD[rsp]
	mov	rbx,QWORD[8+rsp]
	vmovdqu	xmm1,XMMWORD[rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[16+rsp],rax
	mov	QWORD[24+rsp],rbx
	vmovdqa	xmm10,XMMWORD[16+rsp]
	vmovdqu	xmm2,XMMWORD[16+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[32+rsp],rax
	mov	QWORD[40+rsp],rbx
	vmovdqa	xmm11,XMMWORD[32+rsp]
	vmovdqu	xmm3,XMMWORD[32+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[48+rsp],rax
	mov	QWORD[56+rsp],rbx
	vmovdqa	xmm12,XMMWORD[48+rsp]
	vmovdqu	xmm4,XMMWORD[48+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[64+rsp],rax
	mov	QWORD[72+rsp],rbx
	vmovdqa	xmm13,XMMWORD[64+rsp]
	vmovdqu	xmm5,XMMWORD[64+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[80+rsp],rax
	mov	QWORD[88+rsp],rbx
	vmovdqa	xmm14,XMMWORD[80+rsp]
	vmovdqu	xmm6,XMMWORD[80+rcx]
	add	rcx,0x60
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vpxor	xmm4,xmm4,xmm12
	vpxor	xmm5,xmm5,xmm13
	vpxor	xmm6,xmm6,xmm14
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vpxor	xmm2,xmm2,xmm0
	vpxor	xmm3,xmm3,xmm0
	vpxor	xmm4,xmm4,xmm0
	vpxor	xmm5,xmm5,xmm0
	vpxor	xmm6,xmm6,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	DB	98,242,77,8,220,240
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,221,200
	DB	98,242,109,8,221,208
	DB	98,242,101,8,221,216
	DB	98,242,93,8,221,224
	DB	98,242,85,8,221,232
	DB	98,242,77,8,221,240
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vpxor	xmm4,xmm4,xmm12
	vpxor	xmm5,xmm5,xmm13
	vpxor	xmm6,xmm6,xmm14
	vmovdqu	XMMWORD[rdx],xmm1
	vmovdqu	XMMWORD[16+rdx],xmm2
	vmovdqu	XMMWORD[32+rdx],xmm3
	vmovdqu	XMMWORD[48+rdx],xmm4
	vmovdqu	XMMWORD[64+rdx],xmm5
	vmovdqu	XMMWORD[80+rdx],xmm6
	add	rdx,0x60
	vmovdqa	xmm8,xmm6
	and	r8,0xf
	je	NEAR $L$_ret_hEgxyDlCngwrfFe
	jmp	NEAR $L$_steal_cipher_next_hEgxyDlCngwrfFe

$L$_num_blocks_is_5_hEgxyDlCngwrfFe:
	vmovdqa	xmm9,XMMWORD[rsp]
	mov	rax,QWORD[rsp]
	mov	rbx,QWORD[8+rsp]
	vmovdqu	xmm1,XMMWORD[rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[16+rsp],rax
	mov	QWORD[24+rsp],rbx
	vmovdqa	xmm10,XMMWORD[16+rsp]
	vmovdqu	xmm2,XMMWORD[16+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[32+rsp],rax
	mov	QWORD[40+rsp],rbx
	vmovdqa	xmm11,XMMWORD[32+rsp]
	vmovdqu	xmm3,XMMWORD[32+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[48+rsp],rax
	mov	QWORD[56+rsp],rbx
	vmovdqa	xmm12,XMMWORD[48+rsp]
	vmovdqu	xmm4,XMMWORD[48+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[64+rsp],rax
	mov	QWORD[72+rsp],rbx
	vmovdqa	xmm13,XMMWORD[64+rsp]
	vmovdqu	xmm5,XMMWORD[64+rcx]
	add	rcx,0x50
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vpxor	xmm4,xmm4,xmm12
	vpxor	xmm5,xmm5,xmm13
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vpxor	xmm2,xmm2,xmm0
	vpxor	xmm3,xmm3,xmm0
	vpxor	xmm4,xmm4,xmm0
	vpxor	xmm5,xmm5,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	DB	98,242,85,8,220,232
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,221,200
	DB	98,242,109,8,221,208
	DB	98,242,101,8,221,216
	DB	98,242,93,8,221,224
	DB	98,242,85,8,221,232
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vpxor	xmm4,xmm4,xmm12
	vpxor	xmm5,xmm5,xmm13
	vmovdqu	XMMWORD[rdx],xmm1
	vmovdqu	XMMWORD[16+rdx],xmm2
	vmovdqu	XMMWORD[32+rdx],xmm3
	vmovdqu	XMMWORD[48+rdx],xmm4
	vmovdqu	XMMWORD[64+rdx],xmm5
	add	rdx,0x50
	vmovdqa	xmm8,xmm5
	and	r8,0xf
	je	NEAR $L$_ret_hEgxyDlCngwrfFe
	jmp	NEAR $L$_steal_cipher_next_hEgxyDlCngwrfFe

$L$_num_blocks_is_4_hEgxyDlCngwrfFe:
	vmovdqa	xmm9,XMMWORD[rsp]
	mov	rax,QWORD[rsp]
	mov	rbx,QWORD[8+rsp]
	vmovdqu	xmm1,XMMWORD[rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[16+rsp],rax
	mov	QWORD[24+rsp],rbx
	vmovdqa	xmm10,XMMWORD[16+rsp]
	vmovdqu	xmm2,XMMWORD[16+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[32+rsp],rax
	mov	QWORD[40+rsp],rbx
	vmovdqa	xmm11,XMMWORD[32+rsp]
	vmovdqu	xmm3,XMMWORD[32+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[48+rsp],rax
	mov	QWORD[56+rsp],rbx
	vmovdqa	xmm12,XMMWORD[48+rsp]
	vmovdqu	xmm4,XMMWORD[48+rcx]
	add	rcx,0x40
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vpxor	xmm4,xmm4,xmm12
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vpxor	xmm2,xmm2,xmm0
	vpxor	xmm3,xmm3,xmm0
	vpxor	xmm4,xmm4,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	DB	98,242,93,8,220,224
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,221,200
	DB	98,242,109,8,221,208
	DB	98,242,101,8,221,216
	DB	98,242,93,8,221,224
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vpxor	xmm4,xmm4,xmm12
	vmovdqu	XMMWORD[rdx],xmm1
	vmovdqu	XMMWORD[16+rdx],xmm2
	vmovdqu	XMMWORD[32+rdx],xmm3
	vmovdqu	XMMWORD[48+rdx],xmm4
	add	rdx,0x40
	vmovdqa	xmm8,xmm4
	and	r8,0xf
	je	NEAR $L$_ret_hEgxyDlCngwrfFe
	jmp	NEAR $L$_steal_cipher_next_hEgxyDlCngwrfFe

$L$_num_blocks_is_3_hEgxyDlCngwrfFe:
	vmovdqa	xmm9,XMMWORD[rsp]
	mov	rax,QWORD[rsp]
	mov	rbx,QWORD[8+rsp]
	vmovdqu	xmm1,XMMWORD[rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[16+rsp],rax
	mov	QWORD[24+rsp],rbx
	vmovdqa	xmm10,XMMWORD[16+rsp]
	vmovdqu	xmm2,XMMWORD[16+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[32+rsp],rax
	mov	QWORD[40+rsp],rbx
	vmovdqa	xmm11,XMMWORD[32+rsp]
	vmovdqu	xmm3,XMMWORD[32+rcx]
	add	rcx,0x30
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vpxor	xmm2,xmm2,xmm0
	vpxor	xmm3,xmm3,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	DB	98,242,101,8,220,216
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,221,200
	DB	98,242,109,8,221,208
	DB	98,242,101,8,221,216
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vmovdqu	XMMWORD[rdx],xmm1
	vmovdqu	XMMWORD[16+rdx],xmm2
	vmovdqu	XMMWORD[32+rdx],xmm3
	add	rdx,0x30
	vmovdqa	xmm8,xmm3
	and	r8,0xf
	je	NEAR $L$_ret_hEgxyDlCngwrfFe
	jmp	NEAR $L$_steal_cipher_next_hEgxyDlCngwrfFe

$L$_num_blocks_is_2_hEgxyDlCngwrfFe:
	vmovdqa	xmm9,XMMWORD[rsp]
	mov	rax,QWORD[rsp]
	mov	rbx,QWORD[8+rsp]
	vmovdqu	xmm1,XMMWORD[rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[16+rsp],rax
	mov	QWORD[24+rsp],rbx
	vmovdqa	xmm10,XMMWORD[16+rsp]
	vmovdqu	xmm2,XMMWORD[16+rcx]
	add	rcx,0x20
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vpxor	xmm2,xmm2,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,220,200
	DB	98,242,109,8,220,208
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,221,200
	DB	98,242,109,8,221,208
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vmovdqu	XMMWORD[rdx],xmm1
	vmovdqu	XMMWORD[16+rdx],xmm2
	add	rdx,0x20
	vmovdqa	xmm8,xmm2
	and	r8,0xf
	je	NEAR $L$_ret_hEgxyDlCngwrfFe
	jmp	NEAR $L$_steal_cipher_next_hEgxyDlCngwrfFe

$L$_num_blocks_is_1_hEgxyDlCngwrfFe:
	vmovdqa	xmm9,XMMWORD[rsp]
	mov	rax,QWORD[rsp]
	mov	rbx,QWORD[8+rsp]
	vmovdqu	xmm1,XMMWORD[rcx]
	add	rcx,0x10
	vpxor	xmm1,xmm1,xmm9
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,220,200
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,221,200
	vpxor	xmm1,xmm1,xmm9
	vmovdqu	XMMWORD[rdx],xmm1
	add	rdx,0x10
	vmovdqa	xmm8,xmm1
	and	r8,0xf
	je	NEAR $L$_ret_hEgxyDlCngwrfFe
	jmp	NEAR $L$_steal_cipher_next_hEgxyDlCngwrfFe

global	aes_hw_xts_decrypt_avx512


ALIGN	32
aes_hw_xts_decrypt_avx512:

DB	243,15,30,250
	push	rbp
	mov	rbp,rsp
	sub	rsp,552
	and	rsp,0xffffffffffffffc0
	mov	QWORD[528+rsp],rbx
	mov	QWORD[((528 + 8))+rsp],rdi
	mov	QWORD[((528 + 16))+rsp],rsi
	vmovdqa	XMMWORD[(368 + 0)+rsp],xmm6
	vmovdqa	XMMWORD[(368 + 16)+rsp],xmm7
	vmovdqa	XMMWORD[(368 + 32)+rsp],xmm8
	vmovdqa	XMMWORD[(368 + 48)+rsp],xmm9
	vmovdqa	XMMWORD[(368 + 64)+rsp],xmm10
	vmovdqa	XMMWORD[(368 + 80)+rsp],xmm11
	vmovdqa	XMMWORD[(368 + 96)+rsp],xmm12
	vmovdqa	XMMWORD[(368 + 112)+rsp],xmm13
	vmovdqa	XMMWORD[(368 + 128)+rsp],xmm14
	vmovdqa	XMMWORD[(368 + 144)+rsp],xmm15
	mov	rdi,0x87
	vmovdqu	xmm1,XMMWORD[r11]
	vpxor	xmm4,xmm4,xmm4
	vmovdqu	xmm0,XMMWORD[r10]
	vpxor	xmm1,xmm1,xmm0

	vmovdqu	xmm2,XMMWORD[224+r9]
	vmovdqa	XMMWORD[352+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[16+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[208+r9]
	vmovdqa	XMMWORD[336+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[32+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[192+r9]
	vmovdqa	XMMWORD[320+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[48+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[176+r9]
	vmovdqa	XMMWORD[304+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[64+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[160+r9]
	vmovdqa	XMMWORD[288+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[80+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[144+r9]
	vmovdqa	XMMWORD[272+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[96+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[128+r9]
	vmovdqa	XMMWORD[256+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[112+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[112+r9]
	vmovdqa	XMMWORD[240+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[128+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[96+r9]
	vmovdqa	XMMWORD[224+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[144+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[80+r9]
	vmovdqa	XMMWORD[208+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[160+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[64+r9]
	vmovdqa	XMMWORD[192+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[176+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[48+r9]
	vmovdqa	XMMWORD[176+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[192+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[32+r9]
	vmovdqa	XMMWORD[160+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[208+r10]
	DB	98,242,117,8,220,200

	vmovdqu	xmm2,XMMWORD[16+r9]
	vmovdqa	XMMWORD[144+rsp],xmm2

	vmovdqu	xmm0,XMMWORD[224+r10]
	DB	98,242,117,8,221,200

	vmovdqu	xmm2,XMMWORD[r9]
	vmovdqa	XMMWORD[128+rsp],xmm2

	vmovdqa	XMMWORD[rsp],xmm1
	mov	QWORD[((8 + 40))+rbp],rcx
	mov	QWORD[((8 + 48))+rbp],rdx

	cmp	r8,0x80
	jb	NEAR $L$_less_than_128_bytes_amivrujEyduiFoi
	vpbroadcastq	zmm25,rdi
	cmp	r8,0x100
	jge	NEAR $L$_start_by16_amivrujEyduiFoi
	jmp	NEAR $L$_start_by8_amivrujEyduiFoi

$L$_do_n_blocks_amivrujEyduiFoi:
	cmp	r8,0x0
	je	NEAR $L$_ret_amivrujEyduiFoi
	cmp	r8,0x70
	jge	NEAR $L$_remaining_num_blocks_is_7_amivrujEyduiFoi
	cmp	r8,0x60
	jge	NEAR $L$_remaining_num_blocks_is_6_amivrujEyduiFoi
	cmp	r8,0x50
	jge	NEAR $L$_remaining_num_blocks_is_5_amivrujEyduiFoi
	cmp	r8,0x40
	jge	NEAR $L$_remaining_num_blocks_is_4_amivrujEyduiFoi
	cmp	r8,0x30
	jge	NEAR $L$_remaining_num_blocks_is_3_amivrujEyduiFoi
	cmp	r8,0x20
	jge	NEAR $L$_remaining_num_blocks_is_2_amivrujEyduiFoi
	cmp	r8,0x10
	jge	NEAR $L$_remaining_num_blocks_is_1_amivrujEyduiFoi


	vmovdqu	xmm1,xmm5

	vpxor	xmm1,xmm1,xmm9
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,223,200
	vpxor	xmm1,xmm1,xmm9
	vmovdqu	XMMWORD[(-16)+rdx],xmm1
	vmovdqa	xmm8,xmm1


	mov	r10,0x1
	kmovq	k1,r10
	vpsllq	xmm13,xmm9,0x3f
	vpsraq	xmm14,xmm13,0x3f
	vpandq	xmm5,xmm14,xmm25
	vpxorq	xmm9{k1},xmm9,xmm5
	vpsrldq	xmm10,xmm9,0x8
	DB	98,211,181,8,115,194,1
	vpslldq	xmm13,xmm13,0x8
	vpxorq	xmm0,xmm0,xmm13
	jmp	NEAR $L$_steal_cipher_amivrujEyduiFoi

$L$_remaining_num_blocks_is_7_amivrujEyduiFoi:
	mov	r10,0xffffffffffffffff
	shr	r10,0x10
	kmovq	k1,r10
	vmovdqu8	zmm1,ZMMWORD[rcx]
	vmovdqu8	zmm2{k1},[64+rcx]
	add	rcx,0x70
	and	r8,0xf
	je	NEAR $L$_done_7_remain_amivrujEyduiFoi
	vextracti32x4	xmm12,zmm10,0x2
	vextracti32x4	xmm13,zmm10,0x3
	vinserti32x4	zmm10,zmm10,xmm13,0x2

	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vbroadcasti32x4	zmm0,ZMMWORD[128+rsp]
	vpxorq	zmm1,zmm1,zmm0
	vpxorq	zmm2,zmm2,zmm0
	vbroadcasti32x4	zmm0,ZMMWORD[144+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[160+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[176+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208

	vbroadcasti32x4	zmm0,ZMMWORD[192+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[208+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[224+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[240+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[256+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[272+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[288+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[304+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[320+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[336+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[352+rsp]
	DB	98,242,117,72,223,200
	DB	98,242,109,72,223,208


	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vmovdqa32	zmm9,zmm15
	vmovdqa32	zmm10,zmm16
	vmovdqu8	ZMMWORD[rdx],zmm1
	vmovdqu8	ZMMWORD[64+rdx]{k1},zmm2
	add	rdx,0x70
	vextracti32x4	xmm8,zmm2,0x2
	vmovdqa	xmm0,xmm12
	jmp	NEAR $L$_steal_cipher_amivrujEyduiFoi

$L$_done_7_remain_amivrujEyduiFoi:

	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vbroadcasti32x4	zmm0,ZMMWORD[128+rsp]
	vpxorq	zmm1,zmm1,zmm0
	vpxorq	zmm2,zmm2,zmm0
	vbroadcasti32x4	zmm0,ZMMWORD[144+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[160+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[176+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208

	vbroadcasti32x4	zmm0,ZMMWORD[192+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[208+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[224+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[240+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[256+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[272+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[288+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[304+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[320+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[336+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[352+rsp]
	DB	98,242,117,72,223,200
	DB	98,242,109,72,223,208


	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vmovdqa32	zmm9,zmm15
	vmovdqa32	zmm10,zmm16
	vmovdqu8	ZMMWORD[rdx],zmm1
	vmovdqu8	ZMMWORD[64+rdx]{k1},zmm2
	jmp	NEAR $L$_ret_amivrujEyduiFoi

$L$_remaining_num_blocks_is_6_amivrujEyduiFoi:
	vmovdqu8	zmm1,ZMMWORD[rcx]
	vmovdqu8	ymm2,YMMWORD[64+rcx]
	add	rcx,0x60
	and	r8,0xf
	je	NEAR $L$_done_6_remain_amivrujEyduiFoi
	vextracti32x4	xmm12,zmm10,0x1
	vextracti32x4	xmm13,zmm10,0x2
	vinserti32x4	zmm10,zmm10,xmm13,0x1

	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vbroadcasti32x4	zmm0,ZMMWORD[128+rsp]
	vpxorq	zmm1,zmm1,zmm0
	vpxorq	zmm2,zmm2,zmm0
	vbroadcasti32x4	zmm0,ZMMWORD[144+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[160+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[176+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208

	vbroadcasti32x4	zmm0,ZMMWORD[192+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[208+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[224+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[240+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[256+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[272+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[288+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[304+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[320+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[336+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[352+rsp]
	DB	98,242,117,72,223,200
	DB	98,242,109,72,223,208


	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vmovdqa32	zmm9,zmm15
	vmovdqa32	zmm10,zmm16
	vmovdqu8	ZMMWORD[rdx],zmm1
	vmovdqu8	YMMWORD[64+rdx],ymm2
	add	rdx,0x60
	vextracti32x4	xmm8,zmm2,0x1
	vmovdqa	xmm0,xmm12
	jmp	NEAR $L$_steal_cipher_amivrujEyduiFoi

$L$_done_6_remain_amivrujEyduiFoi:

	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vbroadcasti32x4	zmm0,ZMMWORD[128+rsp]
	vpxorq	zmm1,zmm1,zmm0
	vpxorq	zmm2,zmm2,zmm0
	vbroadcasti32x4	zmm0,ZMMWORD[144+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[160+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[176+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208

	vbroadcasti32x4	zmm0,ZMMWORD[192+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[208+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[224+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[240+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[256+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[272+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[288+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[304+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[320+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[336+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[352+rsp]
	DB	98,242,117,72,223,200
	DB	98,242,109,72,223,208


	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vmovdqa32	zmm9,zmm15
	vmovdqa32	zmm10,zmm16
	vmovdqu8	ZMMWORD[rdx],zmm1
	vmovdqu8	YMMWORD[64+rdx],ymm2
	jmp	NEAR $L$_ret_amivrujEyduiFoi

$L$_remaining_num_blocks_is_5_amivrujEyduiFoi:
	vmovdqu8	zmm1,ZMMWORD[rcx]
	vmovdqu	xmm2,XMMWORD[64+rcx]
	add	rcx,0x50
	and	r8,0xf
	je	NEAR $L$_done_5_remain_amivrujEyduiFoi
	vmovdqa	xmm12,xmm10
	vextracti32x4	xmm10,zmm10,0x1

	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vbroadcasti32x4	zmm0,ZMMWORD[128+rsp]
	vpxorq	zmm1,zmm1,zmm0
	vpxorq	zmm2,zmm2,zmm0
	vbroadcasti32x4	zmm0,ZMMWORD[144+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[160+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[176+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208

	vbroadcasti32x4	zmm0,ZMMWORD[192+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[208+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[224+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[240+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[256+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[272+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[288+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[304+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[320+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[336+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[352+rsp]
	DB	98,242,117,72,223,200
	DB	98,242,109,72,223,208


	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vmovdqa32	zmm9,zmm15
	vmovdqa32	zmm10,zmm16
	vmovdqu8	ZMMWORD[rdx],zmm1
	vmovdqu	XMMWORD[64+rdx],xmm2
	add	rdx,0x50
	vmovdqa	xmm8,xmm2
	vmovdqa	xmm0,xmm12
	jmp	NEAR $L$_steal_cipher_amivrujEyduiFoi

$L$_done_5_remain_amivrujEyduiFoi:

	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vbroadcasti32x4	zmm0,ZMMWORD[128+rsp]
	vpxorq	zmm1,zmm1,zmm0
	vpxorq	zmm2,zmm2,zmm0
	vbroadcasti32x4	zmm0,ZMMWORD[144+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[160+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[176+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208

	vbroadcasti32x4	zmm0,ZMMWORD[192+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[208+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[224+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[240+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[256+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[272+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[288+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[304+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[320+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[336+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[352+rsp]
	DB	98,242,117,72,223,200
	DB	98,242,109,72,223,208


	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vmovdqa32	zmm9,zmm15
	vmovdqa32	zmm10,zmm16
	vmovdqu8	ZMMWORD[rdx],zmm1
	vmovdqu8	XMMWORD[64+rdx],xmm2
	jmp	NEAR $L$_ret_amivrujEyduiFoi

$L$_remaining_num_blocks_is_4_amivrujEyduiFoi:
	vmovdqu8	zmm1,ZMMWORD[rcx]
	add	rcx,0x40
	and	r8,0xf
	je	NEAR $L$_done_4_remain_amivrujEyduiFoi
	vextracti32x4	xmm12,zmm9,0x3
	vinserti32x4	zmm9,zmm9,xmm10,0x3

	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vbroadcasti32x4	zmm0,ZMMWORD[128+rsp]
	vpxorq	zmm1,zmm1,zmm0
	vpxorq	zmm2,zmm2,zmm0
	vbroadcasti32x4	zmm0,ZMMWORD[144+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[160+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[176+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208

	vbroadcasti32x4	zmm0,ZMMWORD[192+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[208+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[224+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[240+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[256+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[272+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[288+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[304+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[320+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[336+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[352+rsp]
	DB	98,242,117,72,223,200
	DB	98,242,109,72,223,208


	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vmovdqa32	zmm9,zmm15
	vmovdqa32	zmm10,zmm16
	vmovdqu8	ZMMWORD[rdx],zmm1
	add	rdx,0x40
	vextracti32x4	xmm8,zmm1,0x3
	vmovdqa	xmm0,xmm12
	jmp	NEAR $L$_steal_cipher_amivrujEyduiFoi

$L$_done_4_remain_amivrujEyduiFoi:

	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vbroadcasti32x4	zmm0,ZMMWORD[128+rsp]
	vpxorq	zmm1,zmm1,zmm0
	vpxorq	zmm2,zmm2,zmm0
	vbroadcasti32x4	zmm0,ZMMWORD[144+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[160+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[176+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208

	vbroadcasti32x4	zmm0,ZMMWORD[192+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[208+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[224+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[240+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[256+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[272+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[288+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[304+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[320+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[336+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[352+rsp]
	DB	98,242,117,72,223,200
	DB	98,242,109,72,223,208


	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vmovdqa32	zmm9,zmm15
	vmovdqa32	zmm10,zmm16
	vmovdqu8	ZMMWORD[rdx],zmm1
	jmp	NEAR $L$_ret_amivrujEyduiFoi

$L$_remaining_num_blocks_is_3_amivrujEyduiFoi:
	vmovdqu	xmm1,XMMWORD[rcx]
	vmovdqu	xmm2,XMMWORD[16+rcx]
	vmovdqu	xmm3,XMMWORD[32+rcx]
	add	rcx,0x30
	and	r8,0xf
	je	NEAR $L$_done_3_remain_amivrujEyduiFoi
	vextracti32x4	xmm13,zmm9,0x2
	vextracti32x4	xmm10,zmm9,0x1
	vextracti32x4	xmm11,zmm9,0x3
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vpxor	xmm2,xmm2,xmm0
	vpxor	xmm3,xmm3,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,223,200
	DB	98,242,109,8,223,208
	DB	98,242,101,8,223,216
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vmovdqu	XMMWORD[rdx],xmm1
	vmovdqu	XMMWORD[16+rdx],xmm2
	vmovdqu	XMMWORD[32+rdx],xmm3
	add	rdx,0x30
	vmovdqa	xmm8,xmm3
	vmovdqa	xmm0,xmm13
	jmp	NEAR $L$_steal_cipher_amivrujEyduiFoi

$L$_done_3_remain_amivrujEyduiFoi:
	vextracti32x4	xmm10,zmm9,0x1
	vextracti32x4	xmm11,zmm9,0x2
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vpxor	xmm2,xmm2,xmm0
	vpxor	xmm3,xmm3,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,223,200
	DB	98,242,109,8,223,208
	DB	98,242,101,8,223,216
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vmovdqu	XMMWORD[rdx],xmm1
	vmovdqu	XMMWORD[16+rdx],xmm2
	vmovdqu	XMMWORD[32+rdx],xmm3
	jmp	NEAR $L$_ret_amivrujEyduiFoi

$L$_remaining_num_blocks_is_2_amivrujEyduiFoi:
	vmovdqu	xmm1,XMMWORD[rcx]
	vmovdqu	xmm2,XMMWORD[16+rcx]
	add	rcx,0x20
	and	r8,0xf
	je	NEAR $L$_done_2_remain_amivrujEyduiFoi
	vextracti32x4	xmm10,zmm9,0x2
	vextracti32x4	xmm12,zmm9,0x1
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vpxor	xmm2,xmm2,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,223,200
	DB	98,242,109,8,223,208
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vmovdqu	XMMWORD[rdx],xmm1
	vmovdqu	XMMWORD[16+rdx],xmm2
	add	rdx,0x20
	vmovdqa	xmm8,xmm2
	vmovdqa	xmm0,xmm12
	jmp	NEAR $L$_steal_cipher_amivrujEyduiFoi

$L$_done_2_remain_amivrujEyduiFoi:
	vextracti32x4	xmm10,zmm9,0x1
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vpxor	xmm2,xmm2,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,223,200
	DB	98,242,109,8,223,208
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vmovdqu	XMMWORD[rdx],xmm1
	vmovdqu	XMMWORD[16+rdx],xmm2
	jmp	NEAR $L$_ret_amivrujEyduiFoi

$L$_remaining_num_blocks_is_1_amivrujEyduiFoi:
	vmovdqu	xmm1,XMMWORD[rcx]
	add	rcx,0x10
	and	r8,0xf
	je	NEAR $L$_done_1_remain_amivrujEyduiFoi
	vextracti32x4	xmm11,zmm9,0x1
	vpxor	xmm1,xmm1,xmm11
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,223,200
	vpxor	xmm1,xmm1,xmm11
	vmovdqu	XMMWORD[rdx],xmm1
	add	rdx,0x10
	vmovdqa	xmm8,xmm1
	vmovdqa	xmm0,xmm9
	jmp	NEAR $L$_steal_cipher_amivrujEyduiFoi

$L$_done_1_remain_amivrujEyduiFoi:
	vpxor	xmm1,xmm1,xmm9
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,223,200
	vpxor	xmm1,xmm1,xmm9
	vmovdqu	XMMWORD[rdx],xmm1
	jmp	NEAR $L$_ret_amivrujEyduiFoi

$L$_start_by16_amivrujEyduiFoi:
	vbroadcasti32x4	zmm0,ZMMWORD[rsp]
	vbroadcasti32x4	zmm8,ZMMWORD[shufb_15_7]
	mov	r10,0xaa
	kmovq	k2,r10


	vpshufb	zmm1,zmm0,zmm8
	vpsllvq	zmm4,zmm0,ZMMWORD[const_dq3210]
	vpsrlvq	zmm2,zmm1,ZMMWORD[const_dq5678]
	DB	98,147,109,72,68,217,0
	vpxorq	zmm4{k2},zmm4,zmm2
	vpxord	zmm9,zmm3,zmm4


	vpsllvq	zmm5,zmm0,ZMMWORD[const_dq7654]
	vpsrlvq	zmm6,zmm1,ZMMWORD[const_dq1234]
	DB	98,147,77,72,68,249,0
	vpxorq	zmm5{k2},zmm5,zmm6
	vpxord	zmm10,zmm7,zmm5


	vpsrldq	zmm13,zmm9,0xf
	DB	98,19,21,72,68,241,0
	vpslldq	zmm11,zmm9,0x1
	vpxord	zmm11,zmm11,zmm14

	vpsrldq	zmm15,zmm10,0xf
	DB	98,131,5,72,68,193,0
	vpslldq	zmm12,zmm10,0x1
	vpxord	zmm12,zmm12,zmm16

$L$_main_loop_run_16_amivrujEyduiFoi:
	vmovdqu8	zmm1,ZMMWORD[rcx]
	vmovdqu8	zmm2,ZMMWORD[64+rcx]
	vmovdqu8	zmm3,ZMMWORD[128+rcx]
	vmovdqu8	zmm4,ZMMWORD[192+rcx]
	vmovdqu8	xmm5,XMMWORD[240+rcx]
	add	rcx,0x100
	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10
	vpxorq	zmm3,zmm3,zmm11
	vpxorq	zmm4,zmm4,zmm12
	vbroadcasti32x4	zmm0,ZMMWORD[128+rsp]
	vpxorq	zmm1,zmm1,zmm0
	vpxorq	zmm2,zmm2,zmm0
	vpxorq	zmm3,zmm3,zmm0
	vpxorq	zmm4,zmm4,zmm0
	vpsrldq	zmm13,zmm11,0xf
	DB	98,19,21,72,68,241,0
	vpslldq	zmm15,zmm11,0x1
	vpxord	zmm15,zmm15,zmm14
	vbroadcasti32x4	zmm0,ZMMWORD[144+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208
	DB	98,242,101,72,222,216
	DB	98,242,93,72,222,224
	vbroadcasti32x4	zmm0,ZMMWORD[160+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208
	DB	98,242,101,72,222,216
	DB	98,242,93,72,222,224
	vbroadcasti32x4	zmm0,ZMMWORD[176+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208
	DB	98,242,101,72,222,216
	DB	98,242,93,72,222,224
	vpsrldq	zmm13,zmm12,0xf
	DB	98,19,21,72,68,241,0
	vpslldq	zmm16,zmm12,0x1
	vpxord	zmm16,zmm16,zmm14
	vbroadcasti32x4	zmm0,ZMMWORD[192+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208
	DB	98,242,101,72,222,216
	DB	98,242,93,72,222,224
	vbroadcasti32x4	zmm0,ZMMWORD[208+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208
	DB	98,242,101,72,222,216
	DB	98,242,93,72,222,224
	vbroadcasti32x4	zmm0,ZMMWORD[224+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208
	DB	98,242,101,72,222,216
	DB	98,242,93,72,222,224
	vpsrldq	zmm13,zmm15,0xf
	DB	98,19,21,72,68,241,0
	vpslldq	zmm17,zmm15,0x1
	vpxord	zmm17,zmm17,zmm14
	vbroadcasti32x4	zmm0,ZMMWORD[240+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208
	DB	98,242,101,72,222,216
	DB	98,242,93,72,222,224
	vbroadcasti32x4	zmm0,ZMMWORD[256+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208
	DB	98,242,101,72,222,216
	DB	98,242,93,72,222,224
	vbroadcasti32x4	zmm0,ZMMWORD[272+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208
	DB	98,242,101,72,222,216
	DB	98,242,93,72,222,224
	vpsrldq	zmm13,zmm16,0xf
	DB	98,19,21,72,68,241,0
	vpslldq	zmm18,zmm16,0x1
	vpxord	zmm18,zmm18,zmm14
	vbroadcasti32x4	zmm0,ZMMWORD[288+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208
	DB	98,242,101,72,222,216
	DB	98,242,93,72,222,224
	vbroadcasti32x4	zmm0,ZMMWORD[304+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208
	DB	98,242,101,72,222,216
	DB	98,242,93,72,222,224
	vbroadcasti32x4	zmm0,ZMMWORD[320+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208
	DB	98,242,101,72,222,216
	DB	98,242,93,72,222,224
	vbroadcasti32x4	zmm0,ZMMWORD[336+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208
	DB	98,242,101,72,222,216
	DB	98,242,93,72,222,224
	vbroadcasti32x4	zmm0,ZMMWORD[352+rsp]
	DB	98,242,117,72,223,200
	DB	98,242,109,72,223,208
	DB	98,242,101,72,223,216
	DB	98,242,93,72,223,224
	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10
	vpxorq	zmm3,zmm3,zmm11
	vpxorq	zmm4,zmm4,zmm12

	vmovdqa32	zmm9,zmm15
	vmovdqa32	zmm10,zmm16
	vmovdqa32	zmm11,zmm17
	vmovdqa32	zmm12,zmm18
	vmovdqu8	ZMMWORD[rdx],zmm1
	vmovdqu8	ZMMWORD[64+rdx],zmm2
	vmovdqu8	ZMMWORD[128+rdx],zmm3
	vmovdqu8	ZMMWORD[192+rdx],zmm4
	add	rdx,0x100
	sub	r8,0x100
	cmp	r8,0x100
	jge	NEAR $L$_main_loop_run_16_amivrujEyduiFoi

	cmp	r8,0x80
	jge	NEAR $L$_main_loop_run_8_amivrujEyduiFoi
	jmp	NEAR $L$_do_n_blocks_amivrujEyduiFoi

$L$_start_by8_amivrujEyduiFoi:

	vbroadcasti32x4	zmm0,ZMMWORD[rsp]
	vbroadcasti32x4	zmm8,ZMMWORD[shufb_15_7]
	mov	r10,0xaa
	kmovq	k2,r10


	vpshufb	zmm1,zmm0,zmm8
	vpsllvq	zmm4,zmm0,ZMMWORD[const_dq3210]
	vpsrlvq	zmm2,zmm1,ZMMWORD[const_dq5678]
	DB	98,147,109,72,68,217,0
	vpxorq	zmm4{k2},zmm4,zmm2
	vpxord	zmm9,zmm3,zmm4


	vpsllvq	zmm5,zmm0,ZMMWORD[const_dq7654]
	vpsrlvq	zmm6,zmm1,ZMMWORD[const_dq1234]
	DB	98,147,77,72,68,249,0
	vpxorq	zmm5{k2},zmm5,zmm6
	vpxord	zmm10,zmm7,zmm5

$L$_main_loop_run_8_amivrujEyduiFoi:
	vmovdqu8	zmm1,ZMMWORD[rcx]
	vmovdqu8	zmm2,ZMMWORD[64+rcx]
	vmovdqu8	xmm5,XMMWORD[112+rcx]
	add	rcx,0x80

	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vbroadcasti32x4	zmm0,ZMMWORD[128+rsp]
	vpxorq	zmm1,zmm1,zmm0
	vpxorq	zmm2,zmm2,zmm0
	vpsrldq	zmm13,zmm9,0xf
	DB	98,19,21,72,68,241,0
	vpslldq	zmm15,zmm9,0x1
	vpxord	zmm15,zmm15,zmm14
	vbroadcasti32x4	zmm0,ZMMWORD[144+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[160+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[176+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208
	vpsrldq	zmm13,zmm10,0xf
	DB	98,19,21,72,68,241,0
	vpslldq	zmm16,zmm10,0x1
	vpxord	zmm16,zmm16,zmm14

	vbroadcasti32x4	zmm0,ZMMWORD[192+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[208+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[224+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[240+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[256+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[272+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[288+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[304+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[320+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[336+rsp]
	DB	98,242,117,72,222,200
	DB	98,242,109,72,222,208


	vbroadcasti32x4	zmm0,ZMMWORD[352+rsp]
	DB	98,242,117,72,223,200
	DB	98,242,109,72,223,208


	vpxorq	zmm1,zmm1,zmm9
	vpxorq	zmm2,zmm2,zmm10


	vmovdqa32	zmm9,zmm15
	vmovdqa32	zmm10,zmm16
	vmovdqu8	ZMMWORD[rdx],zmm1
	vmovdqu8	ZMMWORD[64+rdx],zmm2
	add	rdx,0x80
	sub	r8,0x80
	cmp	r8,0x80
	jge	NEAR $L$_main_loop_run_8_amivrujEyduiFoi
	jmp	NEAR $L$_do_n_blocks_amivrujEyduiFoi

$L$_steal_cipher_amivrujEyduiFoi:

	vmovdqa	xmm2,xmm8


	lea	rax,[vpshufb_shf_table]
	vmovdqu	xmm10,XMMWORD[r8*1+rax]
	vpshufb	xmm8,xmm8,xmm10


	vmovdqu	xmm3,XMMWORD[((-16))+r8*1+rcx]
	vmovdqu	XMMWORD[(-16)+r8*1+rdx],xmm8


	lea	rax,[vpshufb_shf_table]
	add	rax,16
	sub	rax,r8
	vmovdqu	xmm10,XMMWORD[rax]
	vpxor	xmm10,xmm10,XMMWORD[mask1]
	vpshufb	xmm3,xmm3,xmm10

	vpblendvb	xmm3,xmm3,xmm2,xmm10


	vpxor	xmm8,xmm3,xmm0


	vpxor	xmm8,xmm8,XMMWORD[128+rsp]
	DB	98,114,61,8,222,132,36,144,0,0,0
	DB	98,114,61,8,222,132,36,160,0,0,0
	DB	98,114,61,8,222,132,36,176,0,0,0
	DB	98,114,61,8,222,132,36,192,0,0,0
	DB	98,114,61,8,222,132,36,208,0,0,0
	DB	98,114,61,8,222,132,36,224,0,0,0
	DB	98,114,61,8,222,132,36,240,0,0,0
	DB	98,114,61,8,222,132,36,0,1,0,0
	DB	98,114,61,8,222,132,36,16,1,0,0
	DB	98,114,61,8,222,132,36,32,1,0,0
	DB	98,114,61,8,222,132,36,48,1,0,0
	DB	98,114,61,8,222,132,36,64,1,0,0
	DB	98,114,61,8,222,132,36,80,1,0,0
	DB	98,114,61,8,223,132,36,96,1,0,0


	vpxor	xmm8,xmm8,xmm0

$L$_done_amivrujEyduiFoi:

	vmovdqu	XMMWORD[(-16)+rdx],xmm8
$L$_ret_amivrujEyduiFoi:
	mov	rbx,QWORD[528+rsp]
	xor	r10,r10
	mov	QWORD[528+rsp],r10

	vpxorq	zmm0,zmm0,zmm0
	mov	rdi,QWORD[((528 + 8))+rsp]
	mov	QWORD[((528 + 8))+rsp],r10
	mov	rsi,QWORD[((528 + 16))+rsp]
	mov	QWORD[((528 + 16))+rsp],r10

	vmovdqa	xmm6,XMMWORD[((368 + 0))+rsp]
	vmovdqa	xmm7,XMMWORD[((368 + 16))+rsp]
	vmovdqa	xmm8,XMMWORD[((368 + 32))+rsp]
	vmovdqa	xmm9,XMMWORD[((368 + 48))+rsp]


	vmovdqa64	ZMMWORD[368+rsp],zmm0

	vmovdqa	xmm10,XMMWORD[((368 + 64))+rsp]
	vmovdqa	xmm11,XMMWORD[((368 + 80))+rsp]
	vmovdqa	xmm12,XMMWORD[((368 + 96))+rsp]
	vmovdqa	xmm13,XMMWORD[((368 + 112))+rsp]


	vmovdqa64	ZMMWORD[(368 + 64)+rsp],zmm0

	vmovdqa	xmm14,XMMWORD[((368 + 128))+rsp]
	vmovdqa	xmm15,XMMWORD[((368 + 144))+rsp]



	vmovdqa	YMMWORD[(368 + 128)+rsp],ymm0

	vmovdqa64	ZMMWORD[128+rsp],zmm0
	vmovdqa64	ZMMWORD[192+rsp],zmm0
	vmovdqa64	ZMMWORD[256+rsp],zmm0



	mov	r10,0x3f
	kmovq	k2,r10
	vmovdqa64	ZMMWORD[320+rsp]{k2},zmm0

	mov	rsp,rbp
	pop	rbp
	vzeroupper
	DB	0F3h,0C3h		;repret

$L$_less_than_128_bytes_amivrujEyduiFoi:
	cmp	r8,0x10
	jb	NEAR $L$_ret_amivrujEyduiFoi

	mov	r10,r8
	and	r10,0x70
	cmp	r10,0x60
	je	NEAR $L$_num_blocks_is_6_amivrujEyduiFoi
	cmp	r10,0x50
	je	NEAR $L$_num_blocks_is_5_amivrujEyduiFoi
	cmp	r10,0x40
	je	NEAR $L$_num_blocks_is_4_amivrujEyduiFoi
	cmp	r10,0x30
	je	NEAR $L$_num_blocks_is_3_amivrujEyduiFoi
	cmp	r10,0x20
	je	NEAR $L$_num_blocks_is_2_amivrujEyduiFoi
	cmp	r10,0x10
	je	NEAR $L$_num_blocks_is_1_amivrujEyduiFoi

$L$_num_blocks_is_7_amivrujEyduiFoi:
	vmovdqa	xmm9,XMMWORD[rsp]
	mov	rax,QWORD[rsp]
	mov	rbx,QWORD[8+rsp]
	vmovdqu	xmm1,XMMWORD[rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[16+rsp],rax
	mov	QWORD[24+rsp],rbx
	vmovdqa	xmm10,XMMWORD[16+rsp]
	vmovdqu	xmm2,XMMWORD[16+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[32+rsp],rax
	mov	QWORD[40+rsp],rbx
	vmovdqa	xmm11,XMMWORD[32+rsp]
	vmovdqu	xmm3,XMMWORD[32+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[48+rsp],rax
	mov	QWORD[56+rsp],rbx
	vmovdqa	xmm12,XMMWORD[48+rsp]
	vmovdqu	xmm4,XMMWORD[48+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[64+rsp],rax
	mov	QWORD[72+rsp],rbx
	vmovdqa	xmm13,XMMWORD[64+rsp]
	vmovdqu	xmm5,XMMWORD[64+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[80+rsp],rax
	mov	QWORD[88+rsp],rbx
	vmovdqa	xmm14,XMMWORD[80+rsp]
	vmovdqu	xmm6,XMMWORD[80+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[96+rsp],rax
	mov	QWORD[104+rsp],rbx
	vmovdqa	xmm15,XMMWORD[96+rsp]
	vmovdqu	xmm7,XMMWORD[96+rcx]
	add	rcx,0x70
	and	r8,0xf
	je	NEAR $L$_done_7_amivrujEyduiFoi

$L$_steal_cipher_7_amivrujEyduiFoi:
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[16+rsp],rax
	mov	QWORD[24+rsp],rbx
	vmovdqa64	xmm16,xmm15
	vmovdqa	xmm15,XMMWORD[16+rsp]
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vpxor	xmm4,xmm4,xmm12
	vpxor	xmm5,xmm5,xmm13
	vpxor	xmm6,xmm6,xmm14
	vpxor	xmm7,xmm7,xmm15
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vpxor	xmm2,xmm2,xmm0
	vpxor	xmm3,xmm3,xmm0
	vpxor	xmm4,xmm4,xmm0
	vpxor	xmm5,xmm5,xmm0
	vpxor	xmm6,xmm6,xmm0
	vpxor	xmm7,xmm7,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,223,200
	DB	98,242,109,8,223,208
	DB	98,242,101,8,223,216
	DB	98,242,93,8,223,224
	DB	98,242,85,8,223,232
	DB	98,242,77,8,223,240
	DB	98,242,69,8,223,248
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vpxor	xmm4,xmm4,xmm12
	vpxor	xmm5,xmm5,xmm13
	vpxor	xmm6,xmm6,xmm14
	vpxor	xmm7,xmm7,xmm15
	vmovdqu	XMMWORD[rdx],xmm1
	vmovdqu	XMMWORD[16+rdx],xmm2
	vmovdqu	XMMWORD[32+rdx],xmm3
	vmovdqu	XMMWORD[48+rdx],xmm4
	vmovdqu	XMMWORD[64+rdx],xmm5
	vmovdqu	XMMWORD[80+rdx],xmm6
	add	rdx,0x70
	vmovdqa64	xmm0,xmm16
	vmovdqa	xmm8,xmm7
	jmp	NEAR $L$_steal_cipher_amivrujEyduiFoi

$L$_done_7_amivrujEyduiFoi:
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vpxor	xmm4,xmm4,xmm12
	vpxor	xmm5,xmm5,xmm13
	vpxor	xmm6,xmm6,xmm14
	vpxor	xmm7,xmm7,xmm15
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vpxor	xmm2,xmm2,xmm0
	vpxor	xmm3,xmm3,xmm0
	vpxor	xmm4,xmm4,xmm0
	vpxor	xmm5,xmm5,xmm0
	vpxor	xmm6,xmm6,xmm0
	vpxor	xmm7,xmm7,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	DB	98,242,69,8,222,248
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,223,200
	DB	98,242,109,8,223,208
	DB	98,242,101,8,223,216
	DB	98,242,93,8,223,224
	DB	98,242,85,8,223,232
	DB	98,242,77,8,223,240
	DB	98,242,69,8,223,248
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vpxor	xmm4,xmm4,xmm12
	vpxor	xmm5,xmm5,xmm13
	vpxor	xmm6,xmm6,xmm14
	vpxor	xmm7,xmm7,xmm15
	vmovdqu	XMMWORD[rdx],xmm1
	vmovdqu	XMMWORD[16+rdx],xmm2
	vmovdqu	XMMWORD[32+rdx],xmm3
	vmovdqu	XMMWORD[48+rdx],xmm4
	vmovdqu	XMMWORD[64+rdx],xmm5
	vmovdqu	XMMWORD[80+rdx],xmm6
	add	rdx,0x70
	vmovdqa	xmm8,xmm7
	jmp	NEAR $L$_done_amivrujEyduiFoi

$L$_num_blocks_is_6_amivrujEyduiFoi:
	vmovdqa	xmm9,XMMWORD[rsp]
	mov	rax,QWORD[rsp]
	mov	rbx,QWORD[8+rsp]
	vmovdqu	xmm1,XMMWORD[rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[16+rsp],rax
	mov	QWORD[24+rsp],rbx
	vmovdqa	xmm10,XMMWORD[16+rsp]
	vmovdqu	xmm2,XMMWORD[16+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[32+rsp],rax
	mov	QWORD[40+rsp],rbx
	vmovdqa	xmm11,XMMWORD[32+rsp]
	vmovdqu	xmm3,XMMWORD[32+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[48+rsp],rax
	mov	QWORD[56+rsp],rbx
	vmovdqa	xmm12,XMMWORD[48+rsp]
	vmovdqu	xmm4,XMMWORD[48+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[64+rsp],rax
	mov	QWORD[72+rsp],rbx
	vmovdqa	xmm13,XMMWORD[64+rsp]
	vmovdqu	xmm5,XMMWORD[64+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[80+rsp],rax
	mov	QWORD[88+rsp],rbx
	vmovdqa	xmm14,XMMWORD[80+rsp]
	vmovdqu	xmm6,XMMWORD[80+rcx]
	add	rcx,0x60
	and	r8,0xf
	je	NEAR $L$_done_6_amivrujEyduiFoi

$L$_steal_cipher_6_amivrujEyduiFoi:
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[16+rsp],rax
	mov	QWORD[24+rsp],rbx
	vmovdqa64	xmm15,xmm14
	vmovdqa	xmm14,XMMWORD[16+rsp]
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vpxor	xmm4,xmm4,xmm12
	vpxor	xmm5,xmm5,xmm13
	vpxor	xmm6,xmm6,xmm14
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vpxor	xmm2,xmm2,xmm0
	vpxor	xmm3,xmm3,xmm0
	vpxor	xmm4,xmm4,xmm0
	vpxor	xmm5,xmm5,xmm0
	vpxor	xmm6,xmm6,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,223,200
	DB	98,242,109,8,223,208
	DB	98,242,101,8,223,216
	DB	98,242,93,8,223,224
	DB	98,242,85,8,223,232
	DB	98,242,77,8,223,240
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vpxor	xmm4,xmm4,xmm12
	vpxor	xmm5,xmm5,xmm13
	vpxor	xmm6,xmm6,xmm14
	vmovdqu	XMMWORD[rdx],xmm1
	vmovdqu	XMMWORD[16+rdx],xmm2
	vmovdqu	XMMWORD[32+rdx],xmm3
	vmovdqu	XMMWORD[48+rdx],xmm4
	vmovdqu	XMMWORD[64+rdx],xmm5
	add	rdx,0x60
	vmovdqa	xmm0,xmm15
	vmovdqa	xmm8,xmm6
	jmp	NEAR $L$_steal_cipher_amivrujEyduiFoi

$L$_done_6_amivrujEyduiFoi:
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vpxor	xmm4,xmm4,xmm12
	vpxor	xmm5,xmm5,xmm13
	vpxor	xmm6,xmm6,xmm14
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vpxor	xmm2,xmm2,xmm0
	vpxor	xmm3,xmm3,xmm0
	vpxor	xmm4,xmm4,xmm0
	vpxor	xmm5,xmm5,xmm0
	vpxor	xmm6,xmm6,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	DB	98,242,77,8,222,240
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,223,200
	DB	98,242,109,8,223,208
	DB	98,242,101,8,223,216
	DB	98,242,93,8,223,224
	DB	98,242,85,8,223,232
	DB	98,242,77,8,223,240
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vpxor	xmm4,xmm4,xmm12
	vpxor	xmm5,xmm5,xmm13
	vpxor	xmm6,xmm6,xmm14
	vmovdqu	XMMWORD[rdx],xmm1
	vmovdqu	XMMWORD[16+rdx],xmm2
	vmovdqu	XMMWORD[32+rdx],xmm3
	vmovdqu	XMMWORD[48+rdx],xmm4
	vmovdqu	XMMWORD[64+rdx],xmm5
	add	rdx,0x60
	vmovdqa	xmm8,xmm6
	jmp	NEAR $L$_done_amivrujEyduiFoi

$L$_num_blocks_is_5_amivrujEyduiFoi:
	vmovdqa	xmm9,XMMWORD[rsp]
	mov	rax,QWORD[rsp]
	mov	rbx,QWORD[8+rsp]
	vmovdqu	xmm1,XMMWORD[rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[16+rsp],rax
	mov	QWORD[24+rsp],rbx
	vmovdqa	xmm10,XMMWORD[16+rsp]
	vmovdqu	xmm2,XMMWORD[16+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[32+rsp],rax
	mov	QWORD[40+rsp],rbx
	vmovdqa	xmm11,XMMWORD[32+rsp]
	vmovdqu	xmm3,XMMWORD[32+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[48+rsp],rax
	mov	QWORD[56+rsp],rbx
	vmovdqa	xmm12,XMMWORD[48+rsp]
	vmovdqu	xmm4,XMMWORD[48+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[64+rsp],rax
	mov	QWORD[72+rsp],rbx
	vmovdqa	xmm13,XMMWORD[64+rsp]
	vmovdqu	xmm5,XMMWORD[64+rcx]
	add	rcx,0x50
	and	r8,0xf
	je	NEAR $L$_done_5_amivrujEyduiFoi

$L$_steal_cipher_5_amivrujEyduiFoi:
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[16+rsp],rax
	mov	QWORD[24+rsp],rbx
	vmovdqa64	xmm14,xmm13
	vmovdqa	xmm13,XMMWORD[16+rsp]
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vpxor	xmm4,xmm4,xmm12
	vpxor	xmm5,xmm5,xmm13
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vpxor	xmm2,xmm2,xmm0
	vpxor	xmm3,xmm3,xmm0
	vpxor	xmm4,xmm4,xmm0
	vpxor	xmm5,xmm5,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,223,200
	DB	98,242,109,8,223,208
	DB	98,242,101,8,223,216
	DB	98,242,93,8,223,224
	DB	98,242,85,8,223,232
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vpxor	xmm4,xmm4,xmm12
	vpxor	xmm5,xmm5,xmm13
	vmovdqu	XMMWORD[rdx],xmm1
	vmovdqu	XMMWORD[16+rdx],xmm2
	vmovdqu	XMMWORD[32+rdx],xmm3
	vmovdqu	XMMWORD[48+rdx],xmm4
	add	rdx,0x50
	vmovdqa	xmm0,xmm14
	vmovdqa	xmm8,xmm5
	jmp	NEAR $L$_steal_cipher_amivrujEyduiFoi

$L$_done_5_amivrujEyduiFoi:
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vpxor	xmm4,xmm4,xmm12
	vpxor	xmm5,xmm5,xmm13
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vpxor	xmm2,xmm2,xmm0
	vpxor	xmm3,xmm3,xmm0
	vpxor	xmm4,xmm4,xmm0
	vpxor	xmm5,xmm5,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	DB	98,242,85,8,222,232
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,223,200
	DB	98,242,109,8,223,208
	DB	98,242,101,8,223,216
	DB	98,242,93,8,223,224
	DB	98,242,85,8,223,232
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vpxor	xmm4,xmm4,xmm12
	vpxor	xmm5,xmm5,xmm13
	vmovdqu	XMMWORD[rdx],xmm1
	vmovdqu	XMMWORD[16+rdx],xmm2
	vmovdqu	XMMWORD[32+rdx],xmm3
	vmovdqu	XMMWORD[48+rdx],xmm4
	add	rdx,0x50
	vmovdqa	xmm8,xmm5
	jmp	NEAR $L$_done_amivrujEyduiFoi

$L$_num_blocks_is_4_amivrujEyduiFoi:
	vmovdqa	xmm9,XMMWORD[rsp]
	mov	rax,QWORD[rsp]
	mov	rbx,QWORD[8+rsp]
	vmovdqu	xmm1,XMMWORD[rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[16+rsp],rax
	mov	QWORD[24+rsp],rbx
	vmovdqa	xmm10,XMMWORD[16+rsp]
	vmovdqu	xmm2,XMMWORD[16+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[32+rsp],rax
	mov	QWORD[40+rsp],rbx
	vmovdqa	xmm11,XMMWORD[32+rsp]
	vmovdqu	xmm3,XMMWORD[32+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[48+rsp],rax
	mov	QWORD[56+rsp],rbx
	vmovdqa	xmm12,XMMWORD[48+rsp]
	vmovdqu	xmm4,XMMWORD[48+rcx]
	add	rcx,0x40
	and	r8,0xf
	je	NEAR $L$_done_4_amivrujEyduiFoi

$L$_steal_cipher_4_amivrujEyduiFoi:
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[16+rsp],rax
	mov	QWORD[24+rsp],rbx
	vmovdqa64	xmm13,xmm12
	vmovdqa	xmm12,XMMWORD[16+rsp]
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vpxor	xmm4,xmm4,xmm12
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vpxor	xmm2,xmm2,xmm0
	vpxor	xmm3,xmm3,xmm0
	vpxor	xmm4,xmm4,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,223,200
	DB	98,242,109,8,223,208
	DB	98,242,101,8,223,216
	DB	98,242,93,8,223,224
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vpxor	xmm4,xmm4,xmm12
	vmovdqu	XMMWORD[rdx],xmm1
	vmovdqu	XMMWORD[16+rdx],xmm2
	vmovdqu	XMMWORD[32+rdx],xmm3
	add	rdx,0x40
	vmovdqa	xmm0,xmm13
	vmovdqa	xmm8,xmm4
	jmp	NEAR $L$_steal_cipher_amivrujEyduiFoi

$L$_done_4_amivrujEyduiFoi:
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vpxor	xmm4,xmm4,xmm12
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vpxor	xmm2,xmm2,xmm0
	vpxor	xmm3,xmm3,xmm0
	vpxor	xmm4,xmm4,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	DB	98,242,93,8,222,224
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,223,200
	DB	98,242,109,8,223,208
	DB	98,242,101,8,223,216
	DB	98,242,93,8,223,224
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vpxor	xmm4,xmm4,xmm12
	vmovdqu	XMMWORD[rdx],xmm1
	vmovdqu	XMMWORD[16+rdx],xmm2
	vmovdqu	XMMWORD[32+rdx],xmm3
	add	rdx,0x40
	vmovdqa	xmm8,xmm4
	jmp	NEAR $L$_done_amivrujEyduiFoi

$L$_num_blocks_is_3_amivrujEyduiFoi:
	vmovdqa	xmm9,XMMWORD[rsp]
	mov	rax,QWORD[rsp]
	mov	rbx,QWORD[8+rsp]
	vmovdqu	xmm1,XMMWORD[rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[16+rsp],rax
	mov	QWORD[24+rsp],rbx
	vmovdqa	xmm10,XMMWORD[16+rsp]
	vmovdqu	xmm2,XMMWORD[16+rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[32+rsp],rax
	mov	QWORD[40+rsp],rbx
	vmovdqa	xmm11,XMMWORD[32+rsp]
	vmovdqu	xmm3,XMMWORD[32+rcx]
	add	rcx,0x30
	and	r8,0xf
	je	NEAR $L$_done_3_amivrujEyduiFoi

$L$_steal_cipher_3_amivrujEyduiFoi:
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[16+rsp],rax
	mov	QWORD[24+rsp],rbx
	vmovdqa64	xmm12,xmm11
	vmovdqa	xmm11,XMMWORD[16+rsp]
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vpxor	xmm2,xmm2,xmm0
	vpxor	xmm3,xmm3,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,223,200
	DB	98,242,109,8,223,208
	DB	98,242,101,8,223,216
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vmovdqu	XMMWORD[rdx],xmm1
	vmovdqu	XMMWORD[16+rdx],xmm2
	add	rdx,0x30
	vmovdqa	xmm0,xmm12
	vmovdqa	xmm8,xmm3
	jmp	NEAR $L$_steal_cipher_amivrujEyduiFoi

$L$_done_3_amivrujEyduiFoi:
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vpxor	xmm2,xmm2,xmm0
	vpxor	xmm3,xmm3,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	DB	98,242,101,8,222,216
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,223,200
	DB	98,242,109,8,223,208
	DB	98,242,101,8,223,216
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vpxor	xmm3,xmm3,xmm11
	vmovdqu	XMMWORD[rdx],xmm1
	vmovdqu	XMMWORD[16+rdx],xmm2
	add	rdx,0x30
	vmovdqa	xmm8,xmm3
	jmp	NEAR $L$_done_amivrujEyduiFoi

$L$_num_blocks_is_2_amivrujEyduiFoi:
	vmovdqa	xmm9,XMMWORD[rsp]
	mov	rax,QWORD[rsp]
	mov	rbx,QWORD[8+rsp]
	vmovdqu	xmm1,XMMWORD[rcx]
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[16+rsp],rax
	mov	QWORD[24+rsp],rbx
	vmovdqa	xmm10,XMMWORD[16+rsp]
	vmovdqu	xmm2,XMMWORD[16+rcx]
	add	rcx,0x20
	and	r8,0xf
	je	NEAR $L$_done_2_amivrujEyduiFoi

$L$_steal_cipher_2_amivrujEyduiFoi:
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[16+rsp],rax
	mov	QWORD[24+rsp],rbx
	vmovdqa64	xmm11,xmm10
	vmovdqa	xmm10,XMMWORD[16+rsp]
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vpxor	xmm2,xmm2,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,223,200
	DB	98,242,109,8,223,208
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vmovdqu	XMMWORD[rdx],xmm1
	add	rdx,0x20
	vmovdqa	xmm0,xmm11
	vmovdqa	xmm8,xmm2
	jmp	NEAR $L$_steal_cipher_amivrujEyduiFoi

$L$_done_2_amivrujEyduiFoi:
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vpxor	xmm2,xmm2,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,222,200
	DB	98,242,109,8,222,208
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,223,200
	DB	98,242,109,8,223,208
	vpxor	xmm1,xmm1,xmm9
	vpxor	xmm2,xmm2,xmm10
	vmovdqu	XMMWORD[rdx],xmm1
	add	rdx,0x20
	vmovdqa	xmm8,xmm2
	jmp	NEAR $L$_done_amivrujEyduiFoi

$L$_num_blocks_is_1_amivrujEyduiFoi:
	vmovdqa	xmm9,XMMWORD[rsp]
	mov	rax,QWORD[rsp]
	mov	rbx,QWORD[8+rsp]
	vmovdqu	xmm1,XMMWORD[rcx]
	add	rcx,0x10
	and	r8,0xf
	je	NEAR $L$_done_1_amivrujEyduiFoi

$L$_steal_cipher_1_amivrujEyduiFoi:
	xor	rsi,rsi
	shl	rax,1
	adc	rbx,rbx
	cmovc	rsi,rdi
	xor	rax,rsi
	mov	QWORD[16+rsp],rax
	mov	QWORD[24+rsp],rbx
	vmovdqa64	xmm10,xmm9
	vmovdqa	xmm9,XMMWORD[16+rsp]
	vpxor	xmm1,xmm1,xmm9
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,223,200
	vpxor	xmm1,xmm1,xmm9
	add	rdx,0x10
	vmovdqa	xmm0,xmm10
	vmovdqa	xmm8,xmm1
	jmp	NEAR $L$_steal_cipher_amivrujEyduiFoi

$L$_done_1_amivrujEyduiFoi:
	vpxor	xmm1,xmm1,xmm9
	vmovdqa	xmm0,XMMWORD[128+rsp]
	vpxor	xmm1,xmm1,xmm0
	vmovdqa	xmm0,XMMWORD[144+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[160+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[176+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[192+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[208+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[224+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[240+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[256+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[272+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[288+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[304+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[320+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[336+rsp]
	DB	98,242,117,8,222,200
	vmovdqa	xmm0,XMMWORD[352+rsp]
	DB	98,242,117,8,223,200
	vpxor	xmm1,xmm1,xmm9
	add	rdx,0x10
	vmovdqa	xmm8,xmm1
	jmp	NEAR $L$_done_amivrujEyduiFoi

section	.rdata rdata align=8
ALIGN	16

vpshufb_shf_table:
	DQ	0x8786858483828100,0x8f8e8d8c8b8a8988
	DQ	0x0706050403020100,0x000e0d0c0b0a0908

mask1:
	DQ	0x8080808080808080,0x8080808080808080

const_dq3210:
	DQ	0,0,1,1,2,2,3,3
const_dq5678:
	DQ	8,8,7,7,6,6,5,5
const_dq7654:
	DQ	4,4,5,5,6,6,7,7
const_dq1234:
	DQ	4,4,3,3,2,2,1,1

shufb_15_7:
	DB	15,0xff,0xff,0xff,0xff,0xff,0xff,0xff,7,0xff,0xff
	DB	0xff,0xff,0xff,0xff,0xff

section	.text

%endif
%else
; Work around https://bugzilla.nasm.us/show_bug.cgi?id=3392738
ret
%endif
