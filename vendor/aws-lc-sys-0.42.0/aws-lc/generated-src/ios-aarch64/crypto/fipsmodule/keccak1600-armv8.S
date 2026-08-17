// This file is generated from a similarly-named Perl script in the BoringSSL
// source tree. Do not edit by hand.

#include <openssl/asm_base.h>

#if !defined(OPENSSL_NO_ASM) && defined(OPENSSL_AARCH64) && defined(__APPLE__)
#include <openssl/arm_arch.h>
.text
.align	8	// strategic alignment and padding that allows to use
		// address value as loop termination condition...
.quad	0,0,0,0,0,0,0,0

iotas_hw:
.quad	0x0000000000000001
.quad	0x0000000000008082
.quad	0x800000000000808a
.quad	0x8000000080008000
.quad	0x000000000000808b
.quad	0x0000000080000001
.quad	0x8000000080008081
.quad	0x8000000000008009
.quad	0x000000000000008a
.quad	0x0000000000000088
.quad	0x0000000080008009
.quad	0x000000008000000a
.quad	0x000000008000808b
.quad	0x800000000000008b
.quad	0x8000000000008089
.quad	0x8000000000008003
.quad	0x8000000000008002
.quad	0x8000000000000080
.quad	0x000000000000800a
.quad	0x800000008000000a
.quad	0x8000000080008081
.quad	0x8000000000008080
.quad	0x0000000080000001
.quad	0x8000000080008008


.align	5
KeccakF1600_int:
	AARCH64_SIGN_LINK_REGISTER
	adr	x28,iotas_hw
	stp	x28,x30,[sp,#16]		// 32 bytes on top are mine
	b	Loop
.align	4
Loop:
	////////////////////////////////////////// Theta
	eor	x26,x0,x5
	stp	x4,x9,[sp,#0]	// offload pair...
	eor	x27,x1,x6
	eor	x28,x2,x7
	eor	x30,x3,x8
	eor	x4,x4,x9
	eor	x26,x26,x10
	eor	x27,x27,x11
	eor	x28,x28,x12
	eor	x30,x30,x13
	eor	x4,x4,x14
	eor	x26,x26,x15
	eor	x27,x27,x16
	eor	x28,x28,x17
	eor	x30,x30,x25
	eor	x4,x4,x19
	eor	x26,x26,x20
	eor	x28,x28,x22
	eor	x27,x27,x21
	eor	x30,x30,x23
	eor	x4,x4,x24
	eor	x9,x26,x28,ror#63
	eor	x1,x1,x9
	eor	x6,x6,x9
	eor	x11,x11,x9
	eor	x16,x16,x9
	eor	x21,x21,x9
	eor	x9,x27,x30,ror#63
	eor	x28,x28,x4,ror#63
	eor	x30,x30,x26,ror#63
	eor	x4,x4,x27,ror#63
	eor	x27,   x2,x9		// mov	x27,x2
	eor	x7,x7,x9
	eor	x12,x12,x9
	eor	x17,x17,x9
	eor	x22,x22,x9
	eor	x0,x0,x4
	eor	x5,x5,x4
	eor	x10,x10,x4
	eor	x15,x15,x4
	eor	x20,x20,x4
	ldp	x4,x9,[sp,#0]	// re-load offloaded data
	eor	x26,   x3,x28		// mov	x26,x3
	eor	x8,x8,x28
	eor	x13,x13,x28
	eor	x25,x25,x28
	eor	x23,x23,x28
	eor	x28,   x4,x30		// mov	x28,x4
	eor	x9,x9,x30
	eor	x14,x14,x30
	eor	x19,x19,x30
	eor	x24,x24,x30
	////////////////////////////////////////// Rho+Pi
	mov	x30,x1
	ror	x1,x6,#20
	//mov	x27,x2
	ror	x2,x12,#21
	//mov	x26,x3
	ror	x3,x25,#43
	//mov	x28,x4
	ror	x4,x24,#50
	ror	x6,x9,#44
	ror	x12,x13,#39
	ror	x25,x17,#49
	ror	x24,x21,#62
	ror	x9,x22,#3
	ror	x13,x19,#56
	ror	x17,x11,#54
	ror	x21,x8,#9
	ror	x22,x14,#25
	ror	x19,x23,#8
	ror	x11,x7,#58
	ror	x8,x16,#19
	ror	x14,x20,#46
	ror	x23,x15,#23
	ror	x7,x10,#61
	ror	x16,x5,#28
	ror	x5,x26,#36
	ror	x10,x30,#63
	ror	x15,x28,#37
	ror	x20,x27,#2
	////////////////////////////////////////// Chi+Iota
	bic	x26,x2,x1
	bic	x27,x3,x2
	bic	x28,x0,x4
	bic	x30,x1,x0
	eor	x0,x0,x26
	bic	x26,x4,x3
	eor	x1,x1,x27
	ldr	x27,[sp,#16]
	eor	x3,x3,x28
	eor	x4,x4,x30
	eor	x2,x2,x26
	ldr	x30,[x27],#8		// Iota[i++]
	bic	x26,x7,x6
	tst	x27,#255			// are we done?
	str	x27,[sp,#16]
	bic	x27,x8,x7
	bic	x28,x5,x9
	eor	x0,x0,x30		// A[0][0] ^= Iota
	bic	x30,x6,x5
	eor	x5,x5,x26
	bic	x26,x9,x8
	eor	x6,x6,x27
	eor	x8,x8,x28
	eor	x9,x9,x30
	eor	x7,x7,x26
	bic	x26,x12,x11
	bic	x27,x13,x12
	bic	x28,x10,x14
	bic	x30,x11,x10
	eor	x10,x10,x26
	bic	x26,x14,x13
	eor	x11,x11,x27
	eor	x13,x13,x28
	eor	x14,x14,x30
	eor	x12,x12,x26
	bic	x26,x17,x16
	bic	x27,x25,x17
	bic	x28,x15,x19
	bic	x30,x16,x15
	eor	x15,x15,x26
	bic	x26,x19,x25
	eor	x16,x16,x27
	eor	x25,x25,x28
	eor	x19,x19,x30
	eor	x17,x17,x26
	bic	x26,x22,x21
	bic	x27,x23,x22
	bic	x28,x20,x24
	bic	x30,x21,x20
	eor	x20,x20,x26
	bic	x26,x24,x23
	eor	x21,x21,x27
	eor	x23,x23,x28
	eor	x24,x24,x30
	eor	x22,x22,x26
	bne	Loop
	ldr	x30,[sp,#24]
	AARCH64_VALIDATE_LINK_REGISTER
	ret

.globl	_KeccakF1600_hw
.private_extern	_KeccakF1600_hw

.align	5
_KeccakF1600_hw:
	AARCH64_SIGN_LINK_REGISTER
	stp	x29,x30,[sp,#-128]!
	add	x29,sp,#0
	stp	x19,x20,[sp,#16]
	stp	x21,x22,[sp,#32]
	stp	x23,x24,[sp,#48]
	stp	x25,x26,[sp,#64]
	stp	x27,x28,[sp,#80]
	sub	sp,sp,#48
	str	x0,[sp,#32]			// offload argument
	mov	x26,x0
	ldp	x0,x1,[x0,#16*0]
	ldp	x2,x3,[x26,#16*1]
	ldp	x4,x5,[x26,#16*2]
	ldp	x6,x7,[x26,#16*3]
	ldp	x8,x9,[x26,#16*4]
	ldp	x10,x11,[x26,#16*5]
	ldp	x12,x13,[x26,#16*6]
	ldp	x14,x15,[x26,#16*7]
	ldp	x16,x17,[x26,#16*8]
	ldp	x25,x19,[x26,#16*9]
	ldp	x20,x21,[x26,#16*10]
	ldp	x22,x23,[x26,#16*11]
	ldr	x24,[x26,#16*12]
	bl	KeccakF1600_int
	ldr	x26,[sp,#32]
	stp	x0,x1,[x26,#16*0]
	stp	x2,x3,[x26,#16*1]
	stp	x4,x5,[x26,#16*2]
	stp	x6,x7,[x26,#16*3]
	stp	x8,x9,[x26,#16*4]
	stp	x10,x11,[x26,#16*5]
	stp	x12,x13,[x26,#16*6]
	stp	x14,x15,[x26,#16*7]
	stp	x16,x17,[x26,#16*8]
	stp	x25,x19,[x26,#16*9]
	stp	x20,x21,[x26,#16*10]
	stp	x22,x23,[x26,#16*11]
	str	x24,[x26,#16*12]
	ldp	x19,x20,[x29,#16]
	add	sp,sp,#48
	ldp	x21,x22,[x29,#32]
	ldp	x23,x24,[x29,#48]
	ldp	x25,x26,[x29,#64]
	ldp	x27,x28,[x29,#80]
	ldp	x29,x30,[sp],#128
	AARCH64_VALIDATE_LINK_REGISTER
	ret

.byte	75,101,99,99,97,107,45,49,54,48,48,32,112,101,114,109,117,116,97,116,105,111,110,32,102,111,114,32,65,82,77,118,56,44,32,67,82,89,80,84,79,71,65,77,83,32,98,121,32,60,97,112,112,114,111,64,111,112,101,110,115,115,108,46,111,114,103,62,0
.align	2
#endif  // !OPENSSL_NO_ASM && defined(OPENSSL_AARCH64) && defined(__APPLE__)
