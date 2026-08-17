// This file is generated from a similarly-named Perl script in the BoringSSL
// source tree. Do not edit by hand.

#include <openssl/asm_base.h>

#if !defined(OPENSSL_NO_ASM) && defined(OPENSSL_X86_64) && defined(__APPLE__)
.text	









.globl	_abi_test_trampoline
.private_extern _abi_test_trampoline
.p2align	4
_abi_test_trampoline:


_CET_ENDBR









	subq	$120,%rsp


	movq	%r8,48(%rsp)
	movq	%rbx,64(%rsp)


	movq	%rbp,72(%rsp)


	movq	%r12,80(%rsp)


	movq	%r13,88(%rsp)


	movq	%r14,96(%rsp)


	movq	%r15,104(%rsp)


	movq	0(%rsi),%rbx
	movq	8(%rsi),%rbp
	movq	16(%rsi),%r12
	movq	24(%rsi),%r13
	movq	32(%rsi),%r14
	movq	40(%rsi),%r15

	movq	%rdi,32(%rsp)
	movq	%rsi,40(%rsp)




	movq	%rdx,%r10
	movq	%rcx,%r11
	decq	%r11
	js	L$args_done
	movq	(%r10),%rdi
	addq	$8,%r10
	decq	%r11
	js	L$args_done
	movq	(%r10),%rsi
	addq	$8,%r10
	decq	%r11
	js	L$args_done
	movq	(%r10),%rdx
	addq	$8,%r10
	decq	%r11
	js	L$args_done
	movq	(%r10),%rcx
	addq	$8,%r10
	decq	%r11
	js	L$args_done
	movq	(%r10),%r8
	addq	$8,%r10
	decq	%r11
	js	L$args_done
	movq	(%r10),%r9
	addq	$8,%r10
	leaq	0(%rsp),%rax
L$args_loop:
	decq	%r11
	js	L$args_done






	movq	%r11,56(%rsp)
	movq	(%r10),%r11
	movq	%r11,(%rax)
	movq	56(%rsp),%r11

	addq	$8,%r10
	addq	$8,%rax
	jmp	L$args_loop

L$args_done:
	movq	32(%rsp),%rax
	movq	48(%rsp),%r10
	testq	%r10,%r10
	jz	L$no_unwind


	pushfq
	orq	$0x100,0(%rsp)
	popfq



	nop
.globl	_abi_test_unwind_start
.private_extern _abi_test_unwind_start
_abi_test_unwind_start:

	call	*%rax
.globl	_abi_test_unwind_return
.private_extern _abi_test_unwind_return
_abi_test_unwind_return:




	pushfq
	andq	$-0x101,0(%rsp)
	popfq
.globl	_abi_test_unwind_stop
.private_extern _abi_test_unwind_stop
_abi_test_unwind_stop:

	jmp	L$call_done

L$no_unwind:
	call	*%rax

L$call_done:

	movq	40(%rsp),%rsi
	movq	%rbx,0(%rsi)
	movq	%rbp,8(%rsi)
	movq	%r12,16(%rsi)
	movq	%r13,24(%rsi)
	movq	%r14,32(%rsi)
	movq	%r15,40(%rsi)
	movq	64(%rsp),%rbx

	movq	72(%rsp),%rbp

	movq	80(%rsp),%r12

	movq	88(%rsp),%r13

	movq	96(%rsp),%r14

	movq	104(%rsp),%r15

	addq	$120,%rsp



	.byte	0xf3,0xc3




.globl	_abi_test_clobber_rax
.private_extern _abi_test_clobber_rax
.p2align	4
_abi_test_clobber_rax:
_CET_ENDBR
	xorq	%rax,%rax
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_rbx
.private_extern _abi_test_clobber_rbx
.p2align	4
_abi_test_clobber_rbx:
_CET_ENDBR
	xorq	%rbx,%rbx
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_rcx
.private_extern _abi_test_clobber_rcx
.p2align	4
_abi_test_clobber_rcx:
_CET_ENDBR
	xorq	%rcx,%rcx
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_rdx
.private_extern _abi_test_clobber_rdx
.p2align	4
_abi_test_clobber_rdx:
_CET_ENDBR
	xorq	%rdx,%rdx
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_rdi
.private_extern _abi_test_clobber_rdi
.p2align	4
_abi_test_clobber_rdi:
_CET_ENDBR
	xorq	%rdi,%rdi
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_rsi
.private_extern _abi_test_clobber_rsi
.p2align	4
_abi_test_clobber_rsi:
_CET_ENDBR
	xorq	%rsi,%rsi
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_rbp
.private_extern _abi_test_clobber_rbp
.p2align	4
_abi_test_clobber_rbp:
_CET_ENDBR
	xorq	%rbp,%rbp
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_r8
.private_extern _abi_test_clobber_r8
.p2align	4
_abi_test_clobber_r8:
_CET_ENDBR
	xorq	%r8,%r8
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_r9
.private_extern _abi_test_clobber_r9
.p2align	4
_abi_test_clobber_r9:
_CET_ENDBR
	xorq	%r9,%r9
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_r10
.private_extern _abi_test_clobber_r10
.p2align	4
_abi_test_clobber_r10:
_CET_ENDBR
	xorq	%r10,%r10
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_r11
.private_extern _abi_test_clobber_r11
.p2align	4
_abi_test_clobber_r11:
_CET_ENDBR
	xorq	%r11,%r11
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_r12
.private_extern _abi_test_clobber_r12
.p2align	4
_abi_test_clobber_r12:
_CET_ENDBR
	xorq	%r12,%r12
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_r13
.private_extern _abi_test_clobber_r13
.p2align	4
_abi_test_clobber_r13:
_CET_ENDBR
	xorq	%r13,%r13
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_r14
.private_extern _abi_test_clobber_r14
.p2align	4
_abi_test_clobber_r14:
_CET_ENDBR
	xorq	%r14,%r14
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_r15
.private_extern _abi_test_clobber_r15
.p2align	4
_abi_test_clobber_r15:
_CET_ENDBR
	xorq	%r15,%r15
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_xmm0
.private_extern _abi_test_clobber_xmm0
.p2align	4
_abi_test_clobber_xmm0:
_CET_ENDBR
	pxor	%xmm0,%xmm0
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_xmm1
.private_extern _abi_test_clobber_xmm1
.p2align	4
_abi_test_clobber_xmm1:
_CET_ENDBR
	pxor	%xmm1,%xmm1
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_xmm2
.private_extern _abi_test_clobber_xmm2
.p2align	4
_abi_test_clobber_xmm2:
_CET_ENDBR
	pxor	%xmm2,%xmm2
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_xmm3
.private_extern _abi_test_clobber_xmm3
.p2align	4
_abi_test_clobber_xmm3:
_CET_ENDBR
	pxor	%xmm3,%xmm3
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_xmm4
.private_extern _abi_test_clobber_xmm4
.p2align	4
_abi_test_clobber_xmm4:
_CET_ENDBR
	pxor	%xmm4,%xmm4
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_xmm5
.private_extern _abi_test_clobber_xmm5
.p2align	4
_abi_test_clobber_xmm5:
_CET_ENDBR
	pxor	%xmm5,%xmm5
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_xmm6
.private_extern _abi_test_clobber_xmm6
.p2align	4
_abi_test_clobber_xmm6:
_CET_ENDBR
	pxor	%xmm6,%xmm6
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_xmm7
.private_extern _abi_test_clobber_xmm7
.p2align	4
_abi_test_clobber_xmm7:
_CET_ENDBR
	pxor	%xmm7,%xmm7
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_xmm8
.private_extern _abi_test_clobber_xmm8
.p2align	4
_abi_test_clobber_xmm8:
_CET_ENDBR
	pxor	%xmm8,%xmm8
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_xmm9
.private_extern _abi_test_clobber_xmm9
.p2align	4
_abi_test_clobber_xmm9:
_CET_ENDBR
	pxor	%xmm9,%xmm9
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_xmm10
.private_extern _abi_test_clobber_xmm10
.p2align	4
_abi_test_clobber_xmm10:
_CET_ENDBR
	pxor	%xmm10,%xmm10
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_xmm11
.private_extern _abi_test_clobber_xmm11
.p2align	4
_abi_test_clobber_xmm11:
_CET_ENDBR
	pxor	%xmm11,%xmm11
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_xmm12
.private_extern _abi_test_clobber_xmm12
.p2align	4
_abi_test_clobber_xmm12:
_CET_ENDBR
	pxor	%xmm12,%xmm12
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_xmm13
.private_extern _abi_test_clobber_xmm13
.p2align	4
_abi_test_clobber_xmm13:
_CET_ENDBR
	pxor	%xmm13,%xmm13
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_xmm14
.private_extern _abi_test_clobber_xmm14
.p2align	4
_abi_test_clobber_xmm14:
_CET_ENDBR
	pxor	%xmm14,%xmm14
	.byte	0xf3,0xc3


.globl	_abi_test_clobber_xmm15
.private_extern _abi_test_clobber_xmm15
.p2align	4
_abi_test_clobber_xmm15:
_CET_ENDBR
	pxor	%xmm15,%xmm15
	.byte	0xf3,0xc3





.globl	_abi_test_bad_unwind_wrong_register
.private_extern _abi_test_bad_unwind_wrong_register
.p2align	4
_abi_test_bad_unwind_wrong_register:


_CET_ENDBR
	pushq	%r12





	nop
	popq	%r12

	.byte	0xf3,0xc3








.globl	_abi_test_bad_unwind_temporary
.private_extern _abi_test_bad_unwind_temporary
.p2align	4
_abi_test_bad_unwind_temporary:


_CET_ENDBR
	pushq	%r12



	movq	%r12,%rax
	incq	%rax
	movq	%rax,(%rsp)



	movq	%r12,(%rsp)


	popq	%r12

	.byte	0xf3,0xc3








.globl	_abi_test_get_and_clear_direction_flag
.private_extern _abi_test_get_and_clear_direction_flag
_abi_test_get_and_clear_direction_flag:
_CET_ENDBR
	pushfq
	popq	%rax
	andq	$0x400,%rax
	shrq	$10,%rax
	cld
	.byte	0xf3,0xc3





.globl	_abi_test_set_direction_flag
.private_extern _abi_test_set_direction_flag
_abi_test_set_direction_flag:
_CET_ENDBR
	std
	.byte	0xf3,0xc3

#endif
