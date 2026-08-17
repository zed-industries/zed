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
global	_abi_test_trampoline
align	16
_abi_test_trampoline:
L$_abi_test_trampoline_begin:
	push	ebp
	push	ebx
	push	esi
	push	edi
	mov	ecx,DWORD [24+esp]
	mov	esi,DWORD [ecx]
	mov	edi,DWORD [4+ecx]
	mov	ebx,DWORD [8+ecx]
	mov	ebp,DWORD [12+ecx]
	sub	esp,44
	mov	eax,DWORD [72+esp]
	xor	ecx,ecx
L$000loop:
	cmp	ecx,DWORD [76+esp]
	jae	NEAR L$001loop_done
	mov	edx,DWORD [ecx*4+eax]
	mov	DWORD [ecx*4+esp],edx
	add	ecx,1
	jmp	NEAR L$000loop
L$001loop_done:
	call	DWORD [64+esp]
	add	esp,44
	mov	ecx,DWORD [24+esp]
	mov	DWORD [ecx],esi
	mov	DWORD [4+ecx],edi
	mov	DWORD [8+ecx],ebx
	mov	DWORD [12+ecx],ebp
	pop	edi
	pop	esi
	pop	ebx
	pop	ebp
	ret
global	_abi_test_get_and_clear_direction_flag
align	16
_abi_test_get_and_clear_direction_flag:
L$_abi_test_get_and_clear_direction_flag_begin:
	pushfd
	pop	eax
	and	eax,1024
	shr	eax,10
	cld
	ret
global	_abi_test_set_direction_flag
align	16
_abi_test_set_direction_flag:
L$_abi_test_set_direction_flag_begin:
	std
	ret
global	_abi_test_clobber_eax
align	16
_abi_test_clobber_eax:
L$_abi_test_clobber_eax_begin:
	xor	eax,eax
	ret
global	_abi_test_clobber_ebx
align	16
_abi_test_clobber_ebx:
L$_abi_test_clobber_ebx_begin:
	xor	ebx,ebx
	ret
global	_abi_test_clobber_ecx
align	16
_abi_test_clobber_ecx:
L$_abi_test_clobber_ecx_begin:
	xor	ecx,ecx
	ret
global	_abi_test_clobber_edx
align	16
_abi_test_clobber_edx:
L$_abi_test_clobber_edx_begin:
	xor	edx,edx
	ret
global	_abi_test_clobber_edi
align	16
_abi_test_clobber_edi:
L$_abi_test_clobber_edi_begin:
	xor	edi,edi
	ret
global	_abi_test_clobber_esi
align	16
_abi_test_clobber_esi:
L$_abi_test_clobber_esi_begin:
	xor	esi,esi
	ret
global	_abi_test_clobber_ebp
align	16
_abi_test_clobber_ebp:
L$_abi_test_clobber_ebp_begin:
	xor	ebp,ebp
	ret
global	_abi_test_clobber_xmm0
align	16
_abi_test_clobber_xmm0:
L$_abi_test_clobber_xmm0_begin:
	pxor	xmm0,xmm0
	ret
global	_abi_test_clobber_xmm1
align	16
_abi_test_clobber_xmm1:
L$_abi_test_clobber_xmm1_begin:
	pxor	xmm1,xmm1
	ret
global	_abi_test_clobber_xmm2
align	16
_abi_test_clobber_xmm2:
L$_abi_test_clobber_xmm2_begin:
	pxor	xmm2,xmm2
	ret
global	_abi_test_clobber_xmm3
align	16
_abi_test_clobber_xmm3:
L$_abi_test_clobber_xmm3_begin:
	pxor	xmm3,xmm3
	ret
global	_abi_test_clobber_xmm4
align	16
_abi_test_clobber_xmm4:
L$_abi_test_clobber_xmm4_begin:
	pxor	xmm4,xmm4
	ret
global	_abi_test_clobber_xmm5
align	16
_abi_test_clobber_xmm5:
L$_abi_test_clobber_xmm5_begin:
	pxor	xmm5,xmm5
	ret
global	_abi_test_clobber_xmm6
align	16
_abi_test_clobber_xmm6:
L$_abi_test_clobber_xmm6_begin:
	pxor	xmm6,xmm6
	ret
global	_abi_test_clobber_xmm7
align	16
_abi_test_clobber_xmm7:
L$_abi_test_clobber_xmm7_begin:
	pxor	xmm7,xmm7
	ret
%else
; Work around https://bugzilla.nasm.us/show_bug.cgi?id=3392738
ret
%endif
