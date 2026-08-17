; This file is generated from a similarly-named Perl script in the BoringSSL
; source tree. Do not edit by hand.

%ifidn __OUTPUT_FORMAT__, win64
default	rel
%define XMMWORD
%define YMMWORD
%define ZMMWORD
%define _CET_ENDBR

%include "openssl/boringssl_prefix_symbols_nasm.inc"
section	.text code align=64







global	gcm_gmult_ssse3
ALIGN	16
gcm_gmult_ssse3:

$L$SEH_begin_gcm_gmult_ssse3_1:
_CET_ENDBR
	sub	rsp,40
$L$SEH_prolog_gcm_gmult_ssse3_2:
	movdqa	XMMWORD[rsp],xmm6
$L$SEH_prolog_gcm_gmult_ssse3_3:
	movdqa	XMMWORD[16+rsp],xmm10
$L$SEH_prolog_gcm_gmult_ssse3_4:
	movdqu	xmm0,XMMWORD[rcx]
	movdqa	xmm10,XMMWORD[$L$reverse_bytes]
	movdqa	xmm2,XMMWORD[$L$low4_mask]


DB	102,65,15,56,0,194


	movdqa	xmm1,xmm2
	pandn	xmm1,xmm0
	psrld	xmm1,4
	pand	xmm0,xmm2




	pxor	xmm2,xmm2
	pxor	xmm3,xmm3
	mov	rax,5
$L$oop_row_1:
	movdqa	xmm4,XMMWORD[rdx]
	lea	rdx,[16+rdx]


	movdqa	xmm6,xmm2
DB	102,15,58,15,243,1
	movdqa	xmm3,xmm6
	psrldq	xmm2,1




	movdqa	xmm5,xmm4
DB	102,15,56,0,224
DB	102,15,56,0,233


	pxor	xmm2,xmm5



	movdqa	xmm5,xmm4
	psllq	xmm5,60
	movdqa	xmm6,xmm5
	pslldq	xmm6,8
	pxor	xmm3,xmm6


	psrldq	xmm5,8
	pxor	xmm2,xmm5
	psrlq	xmm4,4
	pxor	xmm2,xmm4

	sub	rax,1
	jnz	NEAR $L$oop_row_1



	pxor	xmm2,xmm3
	psrlq	xmm3,1
	pxor	xmm2,xmm3
	psrlq	xmm3,1
	pxor	xmm2,xmm3
	psrlq	xmm3,5
	pxor	xmm2,xmm3
	pxor	xmm3,xmm3
	mov	rax,5
$L$oop_row_2:
	movdqa	xmm4,XMMWORD[rdx]
	lea	rdx,[16+rdx]


	movdqa	xmm6,xmm2
DB	102,15,58,15,243,1
	movdqa	xmm3,xmm6
	psrldq	xmm2,1




	movdqa	xmm5,xmm4
DB	102,15,56,0,224
DB	102,15,56,0,233


	pxor	xmm2,xmm5



	movdqa	xmm5,xmm4
	psllq	xmm5,60
	movdqa	xmm6,xmm5
	pslldq	xmm6,8
	pxor	xmm3,xmm6


	psrldq	xmm5,8
	pxor	xmm2,xmm5
	psrlq	xmm4,4
	pxor	xmm2,xmm4

	sub	rax,1
	jnz	NEAR $L$oop_row_2



	pxor	xmm2,xmm3
	psrlq	xmm3,1
	pxor	xmm2,xmm3
	psrlq	xmm3,1
	pxor	xmm2,xmm3
	psrlq	xmm3,5
	pxor	xmm2,xmm3
	pxor	xmm3,xmm3
	mov	rax,6
$L$oop_row_3:
	movdqa	xmm4,XMMWORD[rdx]
	lea	rdx,[16+rdx]


	movdqa	xmm6,xmm2
DB	102,15,58,15,243,1
	movdqa	xmm3,xmm6
	psrldq	xmm2,1




	movdqa	xmm5,xmm4
DB	102,15,56,0,224
DB	102,15,56,0,233


	pxor	xmm2,xmm5



	movdqa	xmm5,xmm4
	psllq	xmm5,60
	movdqa	xmm6,xmm5
	pslldq	xmm6,8
	pxor	xmm3,xmm6


	psrldq	xmm5,8
	pxor	xmm2,xmm5
	psrlq	xmm4,4
	pxor	xmm2,xmm4

	sub	rax,1
	jnz	NEAR $L$oop_row_3



	pxor	xmm2,xmm3
	psrlq	xmm3,1
	pxor	xmm2,xmm3
	psrlq	xmm3,1
	pxor	xmm2,xmm3
	psrlq	xmm3,5
	pxor	xmm2,xmm3
	pxor	xmm3,xmm3

DB	102,65,15,56,0,210
	movdqu	XMMWORD[rcx],xmm2


	pxor	xmm0,xmm0
	pxor	xmm1,xmm1
	pxor	xmm2,xmm2
	pxor	xmm3,xmm3
	pxor	xmm4,xmm4
	pxor	xmm5,xmm5
	pxor	xmm6,xmm6
	movdqa	xmm6,XMMWORD[rsp]
	movdqa	xmm10,XMMWORD[16+rsp]
	add	rsp,40
	DB	0F3h,0C3h		;repret

$L$SEH_end_gcm_gmult_ssse3_5:







global	gcm_ghash_ssse3
ALIGN	16
gcm_ghash_ssse3:

$L$SEH_begin_gcm_ghash_ssse3_1:
_CET_ENDBR
	sub	rsp,56
$L$SEH_prolog_gcm_ghash_ssse3_2:
	movdqa	XMMWORD[rsp],xmm6
$L$SEH_prolog_gcm_ghash_ssse3_3:
	movdqa	XMMWORD[16+rsp],xmm10
$L$SEH_prolog_gcm_ghash_ssse3_4:
	movdqa	XMMWORD[32+rsp],xmm11
$L$SEH_prolog_gcm_ghash_ssse3_5:
	movdqu	xmm0,XMMWORD[rcx]
	movdqa	xmm10,XMMWORD[$L$reverse_bytes]
	movdqa	xmm11,XMMWORD[$L$low4_mask]


	and	r9,-16



DB	102,65,15,56,0,194


	pxor	xmm3,xmm3
$L$oop_ghash:

	movdqu	xmm1,XMMWORD[r8]
DB	102,65,15,56,0,202
	pxor	xmm0,xmm1


	movdqa	xmm1,xmm11
	pandn	xmm1,xmm0
	psrld	xmm1,4
	pand	xmm0,xmm11




	pxor	xmm2,xmm2

	mov	rax,5
$L$oop_row_4:
	movdqa	xmm4,XMMWORD[rdx]
	lea	rdx,[16+rdx]


	movdqa	xmm6,xmm2
DB	102,15,58,15,243,1
	movdqa	xmm3,xmm6
	psrldq	xmm2,1




	movdqa	xmm5,xmm4
DB	102,15,56,0,224
DB	102,15,56,0,233


	pxor	xmm2,xmm5



	movdqa	xmm5,xmm4
	psllq	xmm5,60
	movdqa	xmm6,xmm5
	pslldq	xmm6,8
	pxor	xmm3,xmm6


	psrldq	xmm5,8
	pxor	xmm2,xmm5
	psrlq	xmm4,4
	pxor	xmm2,xmm4

	sub	rax,1
	jnz	NEAR $L$oop_row_4



	pxor	xmm2,xmm3
	psrlq	xmm3,1
	pxor	xmm2,xmm3
	psrlq	xmm3,1
	pxor	xmm2,xmm3
	psrlq	xmm3,5
	pxor	xmm2,xmm3
	pxor	xmm3,xmm3
	mov	rax,5
$L$oop_row_5:
	movdqa	xmm4,XMMWORD[rdx]
	lea	rdx,[16+rdx]


	movdqa	xmm6,xmm2
DB	102,15,58,15,243,1
	movdqa	xmm3,xmm6
	psrldq	xmm2,1




	movdqa	xmm5,xmm4
DB	102,15,56,0,224
DB	102,15,56,0,233


	pxor	xmm2,xmm5



	movdqa	xmm5,xmm4
	psllq	xmm5,60
	movdqa	xmm6,xmm5
	pslldq	xmm6,8
	pxor	xmm3,xmm6


	psrldq	xmm5,8
	pxor	xmm2,xmm5
	psrlq	xmm4,4
	pxor	xmm2,xmm4

	sub	rax,1
	jnz	NEAR $L$oop_row_5



	pxor	xmm2,xmm3
	psrlq	xmm3,1
	pxor	xmm2,xmm3
	psrlq	xmm3,1
	pxor	xmm2,xmm3
	psrlq	xmm3,5
	pxor	xmm2,xmm3
	pxor	xmm3,xmm3
	mov	rax,6
$L$oop_row_6:
	movdqa	xmm4,XMMWORD[rdx]
	lea	rdx,[16+rdx]


	movdqa	xmm6,xmm2
DB	102,15,58,15,243,1
	movdqa	xmm3,xmm6
	psrldq	xmm2,1




	movdqa	xmm5,xmm4
DB	102,15,56,0,224
DB	102,15,56,0,233


	pxor	xmm2,xmm5



	movdqa	xmm5,xmm4
	psllq	xmm5,60
	movdqa	xmm6,xmm5
	pslldq	xmm6,8
	pxor	xmm3,xmm6


	psrldq	xmm5,8
	pxor	xmm2,xmm5
	psrlq	xmm4,4
	pxor	xmm2,xmm4

	sub	rax,1
	jnz	NEAR $L$oop_row_6



	pxor	xmm2,xmm3
	psrlq	xmm3,1
	pxor	xmm2,xmm3
	psrlq	xmm3,1
	pxor	xmm2,xmm3
	psrlq	xmm3,5
	pxor	xmm2,xmm3
	pxor	xmm3,xmm3
	movdqa	xmm0,xmm2


	lea	rdx,[((-256))+rdx]


	lea	r8,[16+r8]
	sub	r9,16
	jnz	NEAR $L$oop_ghash


DB	102,65,15,56,0,194
	movdqu	XMMWORD[rcx],xmm0


	pxor	xmm0,xmm0
	pxor	xmm1,xmm1
	pxor	xmm2,xmm2
	pxor	xmm3,xmm3
	pxor	xmm4,xmm4
	pxor	xmm5,xmm5
	pxor	xmm6,xmm6
	movdqa	xmm6,XMMWORD[rsp]
	movdqa	xmm10,XMMWORD[16+rsp]
	movdqa	xmm11,XMMWORD[32+rsp]
	add	rsp,56
	DB	0F3h,0C3h		;repret

$L$SEH_end_gcm_ghash_ssse3_6:


section	.rdata rdata align=8
ALIGN	16


$L$reverse_bytes:
	DB	15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0

$L$low4_mask:
	DQ	0x0f0f0f0f0f0f0f0f,0x0f0f0f0f0f0f0f0f
section	.text

section	.pdata rdata align=4
ALIGN	4
	DD	$L$SEH_begin_gcm_gmult_ssse3_1 wrt ..imagebase
	DD	$L$SEH_end_gcm_gmult_ssse3_5 wrt ..imagebase
	DD	$L$SEH_info_gcm_gmult_ssse3_0 wrt ..imagebase

	DD	$L$SEH_begin_gcm_ghash_ssse3_1 wrt ..imagebase
	DD	$L$SEH_end_gcm_ghash_ssse3_6 wrt ..imagebase
	DD	$L$SEH_info_gcm_ghash_ssse3_0 wrt ..imagebase


section	.xdata rdata align=4
ALIGN	4
$L$SEH_info_gcm_gmult_ssse3_0:
	DB	1
	DB	$L$SEH_prolog_gcm_gmult_ssse3_4-$L$SEH_begin_gcm_gmult_ssse3_1
	DB	5
	DB	0
	DB	$L$SEH_prolog_gcm_gmult_ssse3_4-$L$SEH_begin_gcm_gmult_ssse3_1
	DB	168
	DW	1
	DB	$L$SEH_prolog_gcm_gmult_ssse3_3-$L$SEH_begin_gcm_gmult_ssse3_1
	DB	104
	DW	0
	DB	$L$SEH_prolog_gcm_gmult_ssse3_2-$L$SEH_begin_gcm_gmult_ssse3_1
	DB	66

ALIGN	4
$L$SEH_info_gcm_ghash_ssse3_0:
	DB	1
	DB	$L$SEH_prolog_gcm_ghash_ssse3_5-$L$SEH_begin_gcm_ghash_ssse3_1
	DB	7
	DB	0
	DB	$L$SEH_prolog_gcm_ghash_ssse3_5-$L$SEH_begin_gcm_ghash_ssse3_1
	DB	184
	DW	2
	DB	$L$SEH_prolog_gcm_ghash_ssse3_4-$L$SEH_begin_gcm_ghash_ssse3_1
	DB	168
	DW	1
	DB	$L$SEH_prolog_gcm_ghash_ssse3_3-$L$SEH_begin_gcm_ghash_ssse3_1
	DB	104
	DW	0
	DB	$L$SEH_prolog_gcm_ghash_ssse3_2-$L$SEH_begin_gcm_ghash_ssse3_1
	DB	98
%else
; Work around https://bugzilla.nasm.us/show_bug.cgi?id=3392738
ret
%endif
