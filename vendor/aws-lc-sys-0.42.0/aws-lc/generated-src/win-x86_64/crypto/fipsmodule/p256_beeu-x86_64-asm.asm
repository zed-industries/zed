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




global	beeu_mod_inverse_vartime
ALIGN	32
beeu_mod_inverse_vartime:
	mov	QWORD[8+rsp],rdi	;WIN64 prologue
	mov	QWORD[16+rsp],rsi
	mov	rax,rsp
$L$SEH_begin_beeu_mod_inverse_vartime:
	mov	rdi,rcx
	mov	rsi,rdx
	mov	rdx,r8
	mov	rcx,r9
	mov	r8,QWORD[40+rsp]
	mov	r9,QWORD[48+rsp]



_CET_ENDBR
	push	rbp

	push	r12

	push	r13

	push	r14

	push	r15

	push	rbx

	push	rsi


	sub	rsp,80

	mov	QWORD[rsp],rdi


	mov	r8,1
	xor	r9,r9
	xor	r10,r10
	xor	r11,r11
	xor	rdi,rdi

	xor	r12,r12
	xor	r13,r13
	xor	r14,r14
	xor	r15,r15
	xor	rbp,rbp


	vmovdqu	xmm0,XMMWORD[rsi]
	vmovdqu	xmm1,XMMWORD[16+rsi]
	vmovdqu	XMMWORD[48+rsp],xmm0
	vmovdqu	XMMWORD[64+rsp],xmm1

	vmovdqu	xmm0,XMMWORD[rdx]
	vmovdqu	xmm1,XMMWORD[16+rdx]
	vmovdqu	XMMWORD[16+rsp],xmm0
	vmovdqu	XMMWORD[32+rsp],xmm1

$L$beeu_loop:
	xor	rbx,rbx
	or	rbx,QWORD[48+rsp]
	or	rbx,QWORD[56+rsp]
	or	rbx,QWORD[64+rsp]
	or	rbx,QWORD[72+rsp]
	jz	NEAR $L$beeu_loop_end










	mov	rcx,1


$L$beeu_shift_loop_XB:
	mov	rbx,rcx
	and	rbx,QWORD[48+rsp]
	jnz	NEAR $L$beeu_shift_loop_end_XB


	mov	rbx,1
	and	rbx,r8
	jz	NEAR $L$shift1_0
	add	r8,QWORD[rdx]
	adc	r9,QWORD[8+rdx]
	adc	r10,QWORD[16+rdx]
	adc	r11,QWORD[24+rdx]
	adc	rdi,0

$L$shift1_0:
	shrd	r8,r9,1
	shrd	r9,r10,1
	shrd	r10,r11,1
	shrd	r11,rdi,1
	shr	rdi,1

	shl	rcx,1





	cmp	rcx,0x8000000
	jne	NEAR $L$beeu_shift_loop_XB

$L$beeu_shift_loop_end_XB:
	bsf	rcx,rcx
	test	rcx,rcx
	jz	NEAR $L$beeu_no_shift_XB



	mov	rax,QWORD[((8+48))+rsp]
	mov	rbx,QWORD[((16+48))+rsp]
	mov	rsi,QWORD[((24+48))+rsp]

	shrd	QWORD[((0+48))+rsp],rax,cl
	shrd	QWORD[((8+48))+rsp],rbx,cl
	shrd	QWORD[((16+48))+rsp],rsi,cl

	shr	rsi,cl
	mov	QWORD[((24+48))+rsp],rsi


$L$beeu_no_shift_XB:

	mov	rcx,1


$L$beeu_shift_loop_YA:
	mov	rbx,rcx
	and	rbx,QWORD[16+rsp]
	jnz	NEAR $L$beeu_shift_loop_end_YA


	mov	rbx,1
	and	rbx,r12
	jz	NEAR $L$shift1_1
	add	r12,QWORD[rdx]
	adc	r13,QWORD[8+rdx]
	adc	r14,QWORD[16+rdx]
	adc	r15,QWORD[24+rdx]
	adc	rbp,0

$L$shift1_1:
	shrd	r12,r13,1
	shrd	r13,r14,1
	shrd	r14,r15,1
	shrd	r15,rbp,1
	shr	rbp,1

	shl	rcx,1





	cmp	rcx,0x8000000
	jne	NEAR $L$beeu_shift_loop_YA

$L$beeu_shift_loop_end_YA:
	bsf	rcx,rcx
	test	rcx,rcx
	jz	NEAR $L$beeu_no_shift_YA



	mov	rax,QWORD[((8+16))+rsp]
	mov	rbx,QWORD[((16+16))+rsp]
	mov	rsi,QWORD[((24+16))+rsp]

	shrd	QWORD[((0+16))+rsp],rax,cl
	shrd	QWORD[((8+16))+rsp],rbx,cl
	shrd	QWORD[((16+16))+rsp],rsi,cl

	shr	rsi,cl
	mov	QWORD[((24+16))+rsp],rsi


$L$beeu_no_shift_YA:

	mov	rax,QWORD[48+rsp]
	mov	rbx,QWORD[56+rsp]
	mov	rsi,QWORD[64+rsp]
	mov	rcx,QWORD[72+rsp]
	sub	rax,QWORD[16+rsp]
	sbb	rbx,QWORD[24+rsp]
	sbb	rsi,QWORD[32+rsp]
	sbb	rcx,QWORD[40+rsp]
	jnc	NEAR $L$beeu_B_bigger_than_A


	mov	rax,QWORD[16+rsp]
	mov	rbx,QWORD[24+rsp]
	mov	rsi,QWORD[32+rsp]
	mov	rcx,QWORD[40+rsp]
	sub	rax,QWORD[48+rsp]
	sbb	rbx,QWORD[56+rsp]
	sbb	rsi,QWORD[64+rsp]
	sbb	rcx,QWORD[72+rsp]
	mov	QWORD[16+rsp],rax
	mov	QWORD[24+rsp],rbx
	mov	QWORD[32+rsp],rsi
	mov	QWORD[40+rsp],rcx


	add	r12,r8
	adc	r13,r9
	adc	r14,r10
	adc	r15,r11
	adc	rbp,rdi
	jmp	NEAR $L$beeu_loop

$L$beeu_B_bigger_than_A:

	mov	QWORD[48+rsp],rax
	mov	QWORD[56+rsp],rbx
	mov	QWORD[64+rsp],rsi
	mov	QWORD[72+rsp],rcx


	add	r8,r12
	adc	r9,r13
	adc	r10,r14
	adc	r11,r15
	adc	rdi,rbp

	jmp	NEAR $L$beeu_loop

$L$beeu_loop_end:




	mov	rbx,QWORD[16+rsp]
	sub	rbx,1
	or	rbx,QWORD[24+rsp]
	or	rbx,QWORD[32+rsp]
	or	rbx,QWORD[40+rsp]

	jnz	NEAR $L$beeu_err




	mov	r8,QWORD[rdx]
	mov	r9,QWORD[8+rdx]
	mov	r10,QWORD[16+rdx]
	mov	r11,QWORD[24+rdx]
	xor	rdi,rdi

$L$beeu_reduction_loop:
	mov	QWORD[16+rsp],r12
	mov	QWORD[24+rsp],r13
	mov	QWORD[32+rsp],r14
	mov	QWORD[40+rsp],r15
	mov	QWORD[48+rsp],rbp


	sub	r12,r8
	sbb	r13,r9
	sbb	r14,r10
	sbb	r15,r11
	sbb	rbp,0


	cmovc	r12,QWORD[16+rsp]
	cmovc	r13,QWORD[24+rsp]
	cmovc	r14,QWORD[32+rsp]
	cmovc	r15,QWORD[40+rsp]
	jnc	NEAR $L$beeu_reduction_loop


	sub	r8,r12
	sbb	r9,r13
	sbb	r10,r14
	sbb	r11,r15

$L$beeu_save:

	mov	rdi,QWORD[rsp]

	mov	QWORD[rdi],r8
	mov	QWORD[8+rdi],r9
	mov	QWORD[16+rdi],r10
	mov	QWORD[24+rdi],r11


	mov	rax,1
	jmp	NEAR $L$beeu_finish

$L$beeu_err:

	xor	rax,rax

$L$beeu_finish:
	add	rsp,80

	pop	rsi

	pop	rbx

	pop	r15

	pop	r14

	pop	r13

	pop	r12

	pop	rbp

	mov	rdi,QWORD[8+rsp]	;WIN64 epilogue
	mov	rsi,QWORD[16+rsp]
	DB	0F3h,0C3h		;repret


$L$SEH_end_beeu_mod_inverse_vartime:
%else
; Work around https://bugzilla.nasm.us/show_bug.cgi?id=3392738
ret
%endif
