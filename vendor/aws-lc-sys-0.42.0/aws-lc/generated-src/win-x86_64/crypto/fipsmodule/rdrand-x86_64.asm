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



global	CRYPTO_rdrand_multiple8

ALIGN	16
CRYPTO_rdrand_multiple8:

_CET_ENDBR
	test	rdx,rdx
	jz	NEAR $L$out
	mov	r8,8
$L$loop:
DB	73,15,199,241
	jnc	NEAR $L$err_multiple
	test	r9,r9
	jz	NEAR $L$err_multiple
	cmp	r9,-1
	je	NEAR $L$err_multiple
	mov	QWORD[rcx],r9
	add	rcx,r8
	sub	rdx,r8
	jnz	NEAR $L$loop
$L$out:
	mov	rax,1
	DB	0F3h,0C3h		;repret
$L$err_multiple:
	xor	rax,rax
	DB	0F3h,0C3h		;repret


%else
; Work around https://bugzilla.nasm.us/show_bug.cgi?id=3392738
ret
%endif
