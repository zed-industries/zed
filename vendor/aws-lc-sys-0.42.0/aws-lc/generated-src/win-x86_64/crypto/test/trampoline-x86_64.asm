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










global	abi_test_trampoline
ALIGN	16
abi_test_trampoline:

$L$SEH_begin_abi_test_trampoline_1:
_CET_ENDBR









	sub	rsp,344

$L$SEH_prolog_abi_test_trampoline_2:
	mov	QWORD[112+rsp],rbx

$L$SEH_prolog_abi_test_trampoline_3:
	mov	QWORD[120+rsp],rbp

$L$SEH_prolog_abi_test_trampoline_4:
	mov	QWORD[128+rsp],rdi

$L$SEH_prolog_abi_test_trampoline_5:
	mov	QWORD[136+rsp],rsi

$L$SEH_prolog_abi_test_trampoline_6:
	mov	QWORD[144+rsp],r12

$L$SEH_prolog_abi_test_trampoline_7:
	mov	QWORD[152+rsp],r13

$L$SEH_prolog_abi_test_trampoline_8:
	mov	QWORD[160+rsp],r14

$L$SEH_prolog_abi_test_trampoline_9:
	mov	QWORD[168+rsp],r15

$L$SEH_prolog_abi_test_trampoline_10:
	movdqa	XMMWORD[176+rsp],xmm6

$L$SEH_prolog_abi_test_trampoline_11:
	movdqa	XMMWORD[192+rsp],xmm7

$L$SEH_prolog_abi_test_trampoline_12:
	movdqa	XMMWORD[208+rsp],xmm8

$L$SEH_prolog_abi_test_trampoline_13:
	movdqa	XMMWORD[224+rsp],xmm9

$L$SEH_prolog_abi_test_trampoline_14:
	movdqa	XMMWORD[240+rsp],xmm10

$L$SEH_prolog_abi_test_trampoline_15:
	movdqa	XMMWORD[256+rsp],xmm11

$L$SEH_prolog_abi_test_trampoline_16:
	movdqa	XMMWORD[272+rsp],xmm12

$L$SEH_prolog_abi_test_trampoline_17:
	movdqa	XMMWORD[288+rsp],xmm13

$L$SEH_prolog_abi_test_trampoline_18:
	movdqa	XMMWORD[304+rsp],xmm14

$L$SEH_prolog_abi_test_trampoline_19:
	movdqa	XMMWORD[320+rsp],xmm15

$L$SEH_prolog_abi_test_trampoline_20:
	mov	rbx,QWORD[rdx]
	mov	rbp,QWORD[8+rdx]
	mov	rdi,QWORD[16+rdx]
	mov	rsi,QWORD[24+rdx]
	mov	r12,QWORD[32+rdx]
	mov	r13,QWORD[40+rdx]
	mov	r14,QWORD[48+rdx]
	mov	r15,QWORD[56+rdx]
	movdqa	xmm6,XMMWORD[64+rdx]
	movdqa	xmm7,XMMWORD[80+rdx]
	movdqa	xmm8,XMMWORD[96+rdx]
	movdqa	xmm9,XMMWORD[112+rdx]
	movdqa	xmm10,XMMWORD[128+rdx]
	movdqa	xmm11,XMMWORD[144+rdx]
	movdqa	xmm12,XMMWORD[160+rdx]
	movdqa	xmm13,XMMWORD[176+rdx]
	movdqa	xmm14,XMMWORD[192+rdx]
	movdqa	xmm15,XMMWORD[208+rdx]

	mov	QWORD[88+rsp],rcx
	mov	QWORD[96+rsp],rdx




	mov	r10,r8
	mov	r11,r9
	dec	r11
	js	NEAR $L$args_done
	mov	rcx,QWORD[r10]
	add	r10,8
	dec	r11
	js	NEAR $L$args_done
	mov	rdx,QWORD[r10]
	add	r10,8
	dec	r11
	js	NEAR $L$args_done
	mov	r8,QWORD[r10]
	add	r10,8
	dec	r11
	js	NEAR $L$args_done
	mov	r9,QWORD[r10]
	add	r10,8
	lea	rax,[32+rsp]
$L$args_loop:
	dec	r11
	js	NEAR $L$args_done






	mov	QWORD[104+rsp],r11
	mov	r11,QWORD[r10]
	mov	QWORD[rax],r11
	mov	r11,QWORD[104+rsp]

	add	r10,8
	add	rax,8
	jmp	NEAR $L$args_loop

$L$args_done:
	mov	rax,QWORD[88+rsp]
	mov	r10,QWORD[384+rsp]
	test	r10,r10
	jz	NEAR $L$no_unwind


	pushfq
	or	QWORD[rsp],0x100
	popfq



	nop
global	abi_test_unwind_start
abi_test_unwind_start:

	call	rax
global	abi_test_unwind_return
abi_test_unwind_return:




	pushfq
	and	QWORD[rsp],-0x101
	popfq
global	abi_test_unwind_stop
abi_test_unwind_stop:

	jmp	NEAR $L$call_done

$L$no_unwind:
	call	rax

$L$call_done:

	mov	rdx,QWORD[96+rsp]
	mov	QWORD[rdx],rbx
	mov	QWORD[8+rdx],rbp
	mov	QWORD[16+rdx],rdi
	mov	QWORD[24+rdx],rsi
	mov	QWORD[32+rdx],r12
	mov	QWORD[40+rdx],r13
	mov	QWORD[48+rdx],r14
	mov	QWORD[56+rdx],r15
	movdqa	XMMWORD[64+rdx],xmm6
	movdqa	XMMWORD[80+rdx],xmm7
	movdqa	XMMWORD[96+rdx],xmm8
	movdqa	XMMWORD[112+rdx],xmm9
	movdqa	XMMWORD[128+rdx],xmm10
	movdqa	XMMWORD[144+rdx],xmm11
	movdqa	XMMWORD[160+rdx],xmm12
	movdqa	XMMWORD[176+rdx],xmm13
	movdqa	XMMWORD[192+rdx],xmm14
	movdqa	XMMWORD[208+rdx],xmm15
	mov	rbx,QWORD[112+rsp]

	mov	rbp,QWORD[120+rsp]

	mov	rdi,QWORD[128+rsp]

	mov	rsi,QWORD[136+rsp]

	mov	r12,QWORD[144+rsp]

	mov	r13,QWORD[152+rsp]

	mov	r14,QWORD[160+rsp]

	mov	r15,QWORD[168+rsp]

	movdqa	xmm6,XMMWORD[176+rsp]

	movdqa	xmm7,XMMWORD[192+rsp]

	movdqa	xmm8,XMMWORD[208+rsp]

	movdqa	xmm9,XMMWORD[224+rsp]

	movdqa	xmm10,XMMWORD[240+rsp]

	movdqa	xmm11,XMMWORD[256+rsp]

	movdqa	xmm12,XMMWORD[272+rsp]

	movdqa	xmm13,XMMWORD[288+rsp]

	movdqa	xmm14,XMMWORD[304+rsp]

	movdqa	xmm15,XMMWORD[320+rsp]

	add	rsp,344



	DB	0F3h,0C3h		;repret

$L$SEH_end_abi_test_trampoline_21:


global	abi_test_clobber_rax
ALIGN	16
abi_test_clobber_rax:
_CET_ENDBR
	xor	rax,rax
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_rbx
ALIGN	16
abi_test_clobber_rbx:
_CET_ENDBR
	xor	rbx,rbx
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_rcx
ALIGN	16
abi_test_clobber_rcx:
_CET_ENDBR
	xor	rcx,rcx
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_rdx
ALIGN	16
abi_test_clobber_rdx:
_CET_ENDBR
	xor	rdx,rdx
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_rdi
ALIGN	16
abi_test_clobber_rdi:
_CET_ENDBR
	xor	rdi,rdi
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_rsi
ALIGN	16
abi_test_clobber_rsi:
_CET_ENDBR
	xor	rsi,rsi
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_rbp
ALIGN	16
abi_test_clobber_rbp:
_CET_ENDBR
	xor	rbp,rbp
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_r8
ALIGN	16
abi_test_clobber_r8:
_CET_ENDBR
	xor	r8,r8
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_r9
ALIGN	16
abi_test_clobber_r9:
_CET_ENDBR
	xor	r9,r9
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_r10
ALIGN	16
abi_test_clobber_r10:
_CET_ENDBR
	xor	r10,r10
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_r11
ALIGN	16
abi_test_clobber_r11:
_CET_ENDBR
	xor	r11,r11
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_r12
ALIGN	16
abi_test_clobber_r12:
_CET_ENDBR
	xor	r12,r12
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_r13
ALIGN	16
abi_test_clobber_r13:
_CET_ENDBR
	xor	r13,r13
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_r14
ALIGN	16
abi_test_clobber_r14:
_CET_ENDBR
	xor	r14,r14
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_r15
ALIGN	16
abi_test_clobber_r15:
_CET_ENDBR
	xor	r15,r15
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_xmm0
ALIGN	16
abi_test_clobber_xmm0:
_CET_ENDBR
	pxor	xmm0,xmm0
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_xmm1
ALIGN	16
abi_test_clobber_xmm1:
_CET_ENDBR
	pxor	xmm1,xmm1
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_xmm2
ALIGN	16
abi_test_clobber_xmm2:
_CET_ENDBR
	pxor	xmm2,xmm2
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_xmm3
ALIGN	16
abi_test_clobber_xmm3:
_CET_ENDBR
	pxor	xmm3,xmm3
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_xmm4
ALIGN	16
abi_test_clobber_xmm4:
_CET_ENDBR
	pxor	xmm4,xmm4
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_xmm5
ALIGN	16
abi_test_clobber_xmm5:
_CET_ENDBR
	pxor	xmm5,xmm5
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_xmm6
ALIGN	16
abi_test_clobber_xmm6:
_CET_ENDBR
	pxor	xmm6,xmm6
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_xmm7
ALIGN	16
abi_test_clobber_xmm7:
_CET_ENDBR
	pxor	xmm7,xmm7
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_xmm8
ALIGN	16
abi_test_clobber_xmm8:
_CET_ENDBR
	pxor	xmm8,xmm8
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_xmm9
ALIGN	16
abi_test_clobber_xmm9:
_CET_ENDBR
	pxor	xmm9,xmm9
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_xmm10
ALIGN	16
abi_test_clobber_xmm10:
_CET_ENDBR
	pxor	xmm10,xmm10
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_xmm11
ALIGN	16
abi_test_clobber_xmm11:
_CET_ENDBR
	pxor	xmm11,xmm11
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_xmm12
ALIGN	16
abi_test_clobber_xmm12:
_CET_ENDBR
	pxor	xmm12,xmm12
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_xmm13
ALIGN	16
abi_test_clobber_xmm13:
_CET_ENDBR
	pxor	xmm13,xmm13
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_xmm14
ALIGN	16
abi_test_clobber_xmm14:
_CET_ENDBR
	pxor	xmm14,xmm14
	DB	0F3h,0C3h		;repret


global	abi_test_clobber_xmm15
ALIGN	16
abi_test_clobber_xmm15:
_CET_ENDBR
	pxor	xmm15,xmm15
	DB	0F3h,0C3h		;repret





global	abi_test_bad_unwind_wrong_register
ALIGN	16
abi_test_bad_unwind_wrong_register:

$L$SEH_begin_abi_test_bad_unwind_wrong_register_1:
_CET_ENDBR
	push	r12

$L$SEH_prolog_abi_test_bad_unwind_wrong_register_2:



	nop
	pop	r12

	DB	0F3h,0C3h		;repret
$L$SEH_end_abi_test_bad_unwind_wrong_register_3:







global	abi_test_bad_unwind_temporary
ALIGN	16
abi_test_bad_unwind_temporary:

$L$SEH_begin_abi_test_bad_unwind_temporary_1:
_CET_ENDBR
	push	r12

$L$SEH_prolog_abi_test_bad_unwind_temporary_2:

	mov	rax,r12
	inc	rax
	mov	QWORD[rsp],rax



	mov	QWORD[rsp],r12


	pop	r12

	DB	0F3h,0C3h		;repret

$L$SEH_end_abi_test_bad_unwind_temporary_3:






global	abi_test_get_and_clear_direction_flag
abi_test_get_and_clear_direction_flag:
_CET_ENDBR
	pushfq
	pop	rax
	and	rax,0x400
	shr	rax,10
	cld
	DB	0F3h,0C3h		;repret





global	abi_test_set_direction_flag
abi_test_set_direction_flag:
_CET_ENDBR
	std
	DB	0F3h,0C3h		;repret






global	abi_test_bad_unwind_epilog
ALIGN	16
abi_test_bad_unwind_epilog:
$L$SEH_begin_abi_test_bad_unwind_epilog_1:
	push	r12
$L$SEH_prolog_abi_test_bad_unwind_epilog_2:

	nop


	pop	r12
	nop
	DB	0F3h,0C3h		;repret
$L$SEH_end_abi_test_bad_unwind_epilog_3:

section	.pdata rdata align=4
ALIGN	4
	DD	$L$SEH_begin_abi_test_trampoline_1 wrt ..imagebase
	DD	$L$SEH_end_abi_test_trampoline_21 wrt ..imagebase
	DD	$L$SEH_info_abi_test_trampoline_0 wrt ..imagebase

	DD	$L$SEH_begin_abi_test_bad_unwind_wrong_register_1 wrt ..imagebase
	DD	$L$SEH_end_abi_test_bad_unwind_wrong_register_3 wrt ..imagebase
	DD	$L$SEH_info_abi_test_bad_unwind_wrong_register_0 wrt ..imagebase

	DD	$L$SEH_begin_abi_test_bad_unwind_temporary_1 wrt ..imagebase
	DD	$L$SEH_end_abi_test_bad_unwind_temporary_3 wrt ..imagebase
	DD	$L$SEH_info_abi_test_bad_unwind_temporary_0 wrt ..imagebase

	DD	$L$SEH_begin_abi_test_bad_unwind_epilog_1 wrt ..imagebase
	DD	$L$SEH_end_abi_test_bad_unwind_epilog_3 wrt ..imagebase
	DD	$L$SEH_info_abi_test_bad_unwind_epilog_0 wrt ..imagebase


section	.xdata rdata align=4
ALIGN	4
$L$SEH_info_abi_test_trampoline_0:
	DB	1
	DB	$L$SEH_prolog_abi_test_trampoline_20-$L$SEH_begin_abi_test_trampoline_1
	DB	38
	DB	0
	DB	$L$SEH_prolog_abi_test_trampoline_20-$L$SEH_begin_abi_test_trampoline_1
	DB	248
	DW	20
	DB	$L$SEH_prolog_abi_test_trampoline_19-$L$SEH_begin_abi_test_trampoline_1
	DB	232
	DW	19
	DB	$L$SEH_prolog_abi_test_trampoline_18-$L$SEH_begin_abi_test_trampoline_1
	DB	216
	DW	18
	DB	$L$SEH_prolog_abi_test_trampoline_17-$L$SEH_begin_abi_test_trampoline_1
	DB	200
	DW	17
	DB	$L$SEH_prolog_abi_test_trampoline_16-$L$SEH_begin_abi_test_trampoline_1
	DB	184
	DW	16
	DB	$L$SEH_prolog_abi_test_trampoline_15-$L$SEH_begin_abi_test_trampoline_1
	DB	168
	DW	15
	DB	$L$SEH_prolog_abi_test_trampoline_14-$L$SEH_begin_abi_test_trampoline_1
	DB	152
	DW	14
	DB	$L$SEH_prolog_abi_test_trampoline_13-$L$SEH_begin_abi_test_trampoline_1
	DB	136
	DW	13
	DB	$L$SEH_prolog_abi_test_trampoline_12-$L$SEH_begin_abi_test_trampoline_1
	DB	120
	DW	12
	DB	$L$SEH_prolog_abi_test_trampoline_11-$L$SEH_begin_abi_test_trampoline_1
	DB	104
	DW	11
	DB	$L$SEH_prolog_abi_test_trampoline_10-$L$SEH_begin_abi_test_trampoline_1
	DB	244
	DW	21
	DB	$L$SEH_prolog_abi_test_trampoline_9-$L$SEH_begin_abi_test_trampoline_1
	DB	228
	DW	20
	DB	$L$SEH_prolog_abi_test_trampoline_8-$L$SEH_begin_abi_test_trampoline_1
	DB	212
	DW	19
	DB	$L$SEH_prolog_abi_test_trampoline_7-$L$SEH_begin_abi_test_trampoline_1
	DB	196
	DW	18
	DB	$L$SEH_prolog_abi_test_trampoline_6-$L$SEH_begin_abi_test_trampoline_1
	DB	100
	DW	17
	DB	$L$SEH_prolog_abi_test_trampoline_5-$L$SEH_begin_abi_test_trampoline_1
	DB	116
	DW	16
	DB	$L$SEH_prolog_abi_test_trampoline_4-$L$SEH_begin_abi_test_trampoline_1
	DB	84
	DW	15
	DB	$L$SEH_prolog_abi_test_trampoline_3-$L$SEH_begin_abi_test_trampoline_1
	DB	52
	DW	14
	DB	$L$SEH_prolog_abi_test_trampoline_2-$L$SEH_begin_abi_test_trampoline_1
	DB	1
	DW	43

ALIGN	4
$L$SEH_info_abi_test_bad_unwind_wrong_register_0:
	DB	1
	DB	$L$SEH_prolog_abi_test_bad_unwind_wrong_register_2-$L$SEH_begin_abi_test_bad_unwind_wrong_register_1
	DB	1
	DB	0
	DB	$L$SEH_prolog_abi_test_bad_unwind_wrong_register_2-$L$SEH_begin_abi_test_bad_unwind_wrong_register_1
	DB	208

ALIGN	4
$L$SEH_info_abi_test_bad_unwind_temporary_0:
	DB	1
	DB	$L$SEH_prolog_abi_test_bad_unwind_temporary_2-$L$SEH_begin_abi_test_bad_unwind_temporary_1
	DB	1
	DB	0
	DB	$L$SEH_prolog_abi_test_bad_unwind_temporary_2-$L$SEH_begin_abi_test_bad_unwind_temporary_1
	DB	192

ALIGN	4
$L$SEH_info_abi_test_bad_unwind_epilog_0:
	DB	1
	DB	$L$SEH_prolog_abi_test_bad_unwind_epilog_2-$L$SEH_begin_abi_test_bad_unwind_epilog_1
	DB	1
	DB	0
	DB	$L$SEH_prolog_abi_test_bad_unwind_epilog_2-$L$SEH_begin_abi_test_bad_unwind_epilog_1
	DB	192
%else
; Work around https://bugzilla.nasm.us/show_bug.cgi?id=3392738
ret
%endif
