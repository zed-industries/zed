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


global	gcm_init_avx512
global	gcm_ghash_avx512
global	gcm_gmult_avx512
global	gcm_setiv_avx512
global	aes_gcm_encrypt_avx512
global	aes_gcm_decrypt_avx512


gcm_init_avx512:
gcm_ghash_avx512:
gcm_gmult_avx512:
gcm_setiv_avx512:
aes_gcm_encrypt_avx512:
aes_gcm_decrypt_avx512:
	DB	0x0f,0x0b
	DB	0F3h,0C3h		;repret

%else
; Work around https://bugzilla.nasm.us/show_bug.cgi?id=3392738
ret
%endif
