// This file is generated from a similarly-named Perl script in the BoringSSL
// source tree. Do not edit by hand.

#include <openssl/asm_base.h>

#if !defined(OPENSSL_NO_ASM) && defined(OPENSSL_AARCH64) && defined(__APPLE__)
#include <openssl/arm_arch.h>


.text

# int CRYPTO_rndr_multiple8(uint8_t *out, const size_t len)
.globl	_CRYPTO_rndr_multiple8
.private_extern	_CRYPTO_rndr_multiple8

.align	4
_CRYPTO_rndr_multiple8:
	cbz	x1, Lrndr_multiple8_error  // len = 0 is not supported

Lrndr_multiple8_loop:
	mrs	x2, s3_3_c2_c4_0             // rndr instruction https://developer.arm.com/documentation/ddi0601/2024-09/Index-by-Encoding
	cbz	x2, Lrndr_multiple8_error   // Check if rndr failed

	str	x2, [x0], #8               // Copy 8 bytes to *out and increment pointer by 8
	sub	x1, x1, #8
	cbz	x1, Lrndr_multiple8_done       // If multiple of 8 this will be 0 eventually
	b	Lrndr_multiple8_loop

Lrndr_multiple8_done:
	mov	x0, #1                            // Return value success
	ret

Lrndr_multiple8_error:
	mov	x0, #0                            // Return value error
	ret

#endif  // !OPENSSL_NO_ASM && defined(OPENSSL_AARCH64) && defined(__APPLE__)
