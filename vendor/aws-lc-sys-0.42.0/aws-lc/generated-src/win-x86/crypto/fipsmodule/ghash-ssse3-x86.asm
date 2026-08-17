; This file is generated from a similarly-named Perl script in the BoringSSL
; source tree. Do not edit by hand.

%include "openssl/boringssl_prefix_symbols_nasm.inc"
%ifidn __OUTPUT_FORMAT__, win32
%ifidn __OUTPUT_FORMAT__,obj
section	code	use32 class=code align=64
%elifidn __OUTPUT_FORMAT__,win32
$@feat.00 equ 1
section	.text	code align=64
%else
section	.text	code
%endif
global	_gcm_gmult_ssse3
align	16
_gcm_gmult_ssse3:
L$_gcm_gmult_ssse3_begin:
	push	ebp
	push	ebx
	push	esi
	push	edi
	mov	edi,DWORD [20+esp]
	mov	esi,DWORD [24+esp]
	movdqu	xmm0,[edi]
	call	L$000pic_point
L$000pic_point:
	pop	eax
	movdqa	xmm7,[(L$reverse_bytes-L$000pic_point)+eax]
	movdqa	xmm2,[(L$low4_mask-L$000pic_point)+eax]
db	102,15,56,0,199
	movdqa	xmm1,xmm2
	pandn	xmm1,xmm0
	psrld	xmm1,4
	pand	xmm0,xmm2
	pxor	xmm2,xmm2
	pxor	xmm3,xmm3
	mov	eax,5
L$001loop_row_1:
	movdqa	xmm4,[esi]
	lea	esi,[16+esi]
	movdqa	xmm6,xmm2
db	102,15,58,15,243,1
	movdqa	xmm3,xmm6
	psrldq	xmm2,1
	movdqa	xmm5,xmm4
db	102,15,56,0,224
db	102,15,56,0,233
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
	sub	eax,1
	jnz	NEAR L$001loop_row_1
	pxor	xmm2,xmm3
	psrlq	xmm3,1
	pxor	xmm2,xmm3
	psrlq	xmm3,1
	pxor	xmm2,xmm3
	psrlq	xmm3,5
	pxor	xmm2,xmm3
	pxor	xmm3,xmm3
	mov	eax,5
L$002loop_row_2:
	movdqa	xmm4,[esi]
	lea	esi,[16+esi]
	movdqa	xmm6,xmm2
db	102,15,58,15,243,1
	movdqa	xmm3,xmm6
	psrldq	xmm2,1
	movdqa	xmm5,xmm4
db	102,15,56,0,224
db	102,15,56,0,233
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
	sub	eax,1
	jnz	NEAR L$002loop_row_2
	pxor	xmm2,xmm3
	psrlq	xmm3,1
	pxor	xmm2,xmm3
	psrlq	xmm3,1
	pxor	xmm2,xmm3
	psrlq	xmm3,5
	pxor	xmm2,xmm3
	pxor	xmm3,xmm3
	mov	eax,6
L$003loop_row_3:
	movdqa	xmm4,[esi]
	lea	esi,[16+esi]
	movdqa	xmm6,xmm2
db	102,15,58,15,243,1
	movdqa	xmm3,xmm6
	psrldq	xmm2,1
	movdqa	xmm5,xmm4
db	102,15,56,0,224
db	102,15,56,0,233
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
	sub	eax,1
	jnz	NEAR L$003loop_row_3
	pxor	xmm2,xmm3
	psrlq	xmm3,1
	pxor	xmm2,xmm3
	psrlq	xmm3,1
	pxor	xmm2,xmm3
	psrlq	xmm3,5
	pxor	xmm2,xmm3
	pxor	xmm3,xmm3
db	102,15,56,0,215
	movdqu	[edi],xmm2
	pxor	xmm0,xmm0
	pxor	xmm1,xmm1
	pxor	xmm2,xmm2
	pxor	xmm3,xmm3
	pxor	xmm4,xmm4
	pxor	xmm5,xmm5
	pxor	xmm6,xmm6
	pop	edi
	pop	esi
	pop	ebx
	pop	ebp
	ret
global	_gcm_ghash_ssse3
align	16
_gcm_ghash_ssse3:
L$_gcm_ghash_ssse3_begin:
	push	ebp
	push	ebx
	push	esi
	push	edi
	mov	edi,DWORD [20+esp]
	mov	esi,DWORD [24+esp]
	mov	edx,DWORD [28+esp]
	mov	ecx,DWORD [32+esp]
	movdqu	xmm0,[edi]
	call	L$004pic_point
L$004pic_point:
	pop	ebx
	movdqa	xmm7,[(L$reverse_bytes-L$004pic_point)+ebx]
	and	ecx,-16
db	102,15,56,0,199
	pxor	xmm3,xmm3
L$005loop_ghash:
	movdqa	xmm2,[(L$low4_mask-L$004pic_point)+ebx]
	movdqu	xmm1,[edx]
db	102,15,56,0,207
	pxor	xmm0,xmm1
	movdqa	xmm1,xmm2
	pandn	xmm1,xmm0
	psrld	xmm1,4
	pand	xmm0,xmm2
	pxor	xmm2,xmm2
	mov	eax,5
L$006loop_row_4:
	movdqa	xmm4,[esi]
	lea	esi,[16+esi]
	movdqa	xmm6,xmm2
db	102,15,58,15,243,1
	movdqa	xmm3,xmm6
	psrldq	xmm2,1
	movdqa	xmm5,xmm4
db	102,15,56,0,224
db	102,15,56,0,233
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
	sub	eax,1
	jnz	NEAR L$006loop_row_4
	pxor	xmm2,xmm3
	psrlq	xmm3,1
	pxor	xmm2,xmm3
	psrlq	xmm3,1
	pxor	xmm2,xmm3
	psrlq	xmm3,5
	pxor	xmm2,xmm3
	pxor	xmm3,xmm3
	mov	eax,5
L$007loop_row_5:
	movdqa	xmm4,[esi]
	lea	esi,[16+esi]
	movdqa	xmm6,xmm2
db	102,15,58,15,243,1
	movdqa	xmm3,xmm6
	psrldq	xmm2,1
	movdqa	xmm5,xmm4
db	102,15,56,0,224
db	102,15,56,0,233
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
	sub	eax,1
	jnz	NEAR L$007loop_row_5
	pxor	xmm2,xmm3
	psrlq	xmm3,1
	pxor	xmm2,xmm3
	psrlq	xmm3,1
	pxor	xmm2,xmm3
	psrlq	xmm3,5
	pxor	xmm2,xmm3
	pxor	xmm3,xmm3
	mov	eax,6
L$008loop_row_6:
	movdqa	xmm4,[esi]
	lea	esi,[16+esi]
	movdqa	xmm6,xmm2
db	102,15,58,15,243,1
	movdqa	xmm3,xmm6
	psrldq	xmm2,1
	movdqa	xmm5,xmm4
db	102,15,56,0,224
db	102,15,56,0,233
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
	sub	eax,1
	jnz	NEAR L$008loop_row_6
	pxor	xmm2,xmm3
	psrlq	xmm3,1
	pxor	xmm2,xmm3
	psrlq	xmm3,1
	pxor	xmm2,xmm3
	psrlq	xmm3,5
	pxor	xmm2,xmm3
	pxor	xmm3,xmm3
	movdqa	xmm0,xmm2
	lea	esi,[esi-256]
	lea	edx,[16+edx]
	sub	ecx,16
	jnz	NEAR L$005loop_ghash
db	102,15,56,0,199
	movdqu	[edi],xmm0
	pxor	xmm0,xmm0
	pxor	xmm1,xmm1
	pxor	xmm2,xmm2
	pxor	xmm3,xmm3
	pxor	xmm4,xmm4
	pxor	xmm5,xmm5
	pxor	xmm6,xmm6
	pop	edi
	pop	esi
	pop	ebx
	pop	ebp
	ret
align	16
L$reverse_bytes:
db	15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0
align	16
L$low4_mask:
dd	252645135,252645135,252645135,252645135
%else
; Work around https://bugzilla.nasm.us/show_bug.cgi?id=3392738
ret
%endif
