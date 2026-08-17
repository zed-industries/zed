// This file is generated from a similarly-named Perl script in the BoringSSL
// source tree. Do not edit by hand.

#include <openssl/asm_base.h>

#if !defined(OPENSSL_NO_ASM) && defined(OPENSSL_X86_64) && defined(__ELF__)
.text	








.type	abi_test_trampoline, @function
.globl	abi_test_trampoline
.hidden abi_test_trampoline
.align	16
abi_test_trampoline:
.cfi_startproc	

_CET_ENDBR









	subq	$120,%rsp
.cfi_adjust_cfa_offset	120

	movq	%r8,48(%rsp)
	movq	%rbx,64(%rsp)
.cfi_offset	rbx, -64

	movq	%rbp,72(%rsp)
.cfi_offset	rbp, -56

	movq	%r12,80(%rsp)
.cfi_offset	r12, -48

	movq	%r13,88(%rsp)
.cfi_offset	r13, -40

	movq	%r14,96(%rsp)
.cfi_offset	r14, -32

	movq	%r15,104(%rsp)
.cfi_offset	r15, -24

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
	js	.Largs_done
	movq	(%r10),%rdi
	addq	$8,%r10
	decq	%r11
	js	.Largs_done
	movq	(%r10),%rsi
	addq	$8,%r10
	decq	%r11
	js	.Largs_done
	movq	(%r10),%rdx
	addq	$8,%r10
	decq	%r11
	js	.Largs_done
	movq	(%r10),%rcx
	addq	$8,%r10
	decq	%r11
	js	.Largs_done
	movq	(%r10),%r8
	addq	$8,%r10
	decq	%r11
	js	.Largs_done
	movq	(%r10),%r9
	addq	$8,%r10
	leaq	0(%rsp),%rax
.Largs_loop:
	decq	%r11
	js	.Largs_done






	movq	%r11,56(%rsp)
	movq	(%r10),%r11
	movq	%r11,(%rax)
	movq	56(%rsp),%r11

	addq	$8,%r10
	addq	$8,%rax
	jmp	.Largs_loop

.Largs_done:
	movq	32(%rsp),%rax
	movq	48(%rsp),%r10
	testq	%r10,%r10
	jz	.Lno_unwind


	pushfq
	orq	$0x100,0(%rsp)
	popfq



	nop
.globl	abi_test_unwind_start
.hidden abi_test_unwind_start
abi_test_unwind_start:

	call	*%rax
.globl	abi_test_unwind_return
.hidden abi_test_unwind_return
abi_test_unwind_return:




	pushfq
	andq	$-0x101,0(%rsp)
	popfq
.globl	abi_test_unwind_stop
.hidden abi_test_unwind_stop
abi_test_unwind_stop:

	jmp	.Lcall_done

.Lno_unwind:
	call	*%rax

.Lcall_done:

	movq	40(%rsp),%rsi
	movq	%rbx,0(%rsi)
	movq	%rbp,8(%rsi)
	movq	%r12,16(%rsi)
	movq	%r13,24(%rsi)
	movq	%r14,32(%rsi)
	movq	%r15,40(%rsi)
	movq	64(%rsp),%rbx
.cfi_restore	rbx
	movq	72(%rsp),%rbp
.cfi_restore	rbp
	movq	80(%rsp),%r12
.cfi_restore	r12
	movq	88(%rsp),%r13
.cfi_restore	r13
	movq	96(%rsp),%r14
.cfi_restore	r14
	movq	104(%rsp),%r15
.cfi_restore	r15
	addq	$120,%rsp
.cfi_adjust_cfa_offset	-120


	.byte	0xf3,0xc3
.cfi_endproc	

.size	abi_test_trampoline,.-abi_test_trampoline
.type	abi_test_clobber_rax, @function
.globl	abi_test_clobber_rax
.hidden abi_test_clobber_rax
.align	16
abi_test_clobber_rax:
_CET_ENDBR
	xorq	%rax,%rax
	.byte	0xf3,0xc3
.size	abi_test_clobber_rax,.-abi_test_clobber_rax
.type	abi_test_clobber_rbx, @function
.globl	abi_test_clobber_rbx
.hidden abi_test_clobber_rbx
.align	16
abi_test_clobber_rbx:
_CET_ENDBR
	xorq	%rbx,%rbx
	.byte	0xf3,0xc3
.size	abi_test_clobber_rbx,.-abi_test_clobber_rbx
.type	abi_test_clobber_rcx, @function
.globl	abi_test_clobber_rcx
.hidden abi_test_clobber_rcx
.align	16
abi_test_clobber_rcx:
_CET_ENDBR
	xorq	%rcx,%rcx
	.byte	0xf3,0xc3
.size	abi_test_clobber_rcx,.-abi_test_clobber_rcx
.type	abi_test_clobber_rdx, @function
.globl	abi_test_clobber_rdx
.hidden abi_test_clobber_rdx
.align	16
abi_test_clobber_rdx:
_CET_ENDBR
	xorq	%rdx,%rdx
	.byte	0xf3,0xc3
.size	abi_test_clobber_rdx,.-abi_test_clobber_rdx
.type	abi_test_clobber_rdi, @function
.globl	abi_test_clobber_rdi
.hidden abi_test_clobber_rdi
.align	16
abi_test_clobber_rdi:
_CET_ENDBR
	xorq	%rdi,%rdi
	.byte	0xf3,0xc3
.size	abi_test_clobber_rdi,.-abi_test_clobber_rdi
.type	abi_test_clobber_rsi, @function
.globl	abi_test_clobber_rsi
.hidden abi_test_clobber_rsi
.align	16
abi_test_clobber_rsi:
_CET_ENDBR
	xorq	%rsi,%rsi
	.byte	0xf3,0xc3
.size	abi_test_clobber_rsi,.-abi_test_clobber_rsi
.type	abi_test_clobber_rbp, @function
.globl	abi_test_clobber_rbp
.hidden abi_test_clobber_rbp
.align	16
abi_test_clobber_rbp:
_CET_ENDBR
	xorq	%rbp,%rbp
	.byte	0xf3,0xc3
.size	abi_test_clobber_rbp,.-abi_test_clobber_rbp
.type	abi_test_clobber_r8, @function
.globl	abi_test_clobber_r8
.hidden abi_test_clobber_r8
.align	16
abi_test_clobber_r8:
_CET_ENDBR
	xorq	%r8,%r8
	.byte	0xf3,0xc3
.size	abi_test_clobber_r8,.-abi_test_clobber_r8
.type	abi_test_clobber_r9, @function
.globl	abi_test_clobber_r9
.hidden abi_test_clobber_r9
.align	16
abi_test_clobber_r9:
_CET_ENDBR
	xorq	%r9,%r9
	.byte	0xf3,0xc3
.size	abi_test_clobber_r9,.-abi_test_clobber_r9
.type	abi_test_clobber_r10, @function
.globl	abi_test_clobber_r10
.hidden abi_test_clobber_r10
.align	16
abi_test_clobber_r10:
_CET_ENDBR
	xorq	%r10,%r10
	.byte	0xf3,0xc3
.size	abi_test_clobber_r10,.-abi_test_clobber_r10
.type	abi_test_clobber_r11, @function
.globl	abi_test_clobber_r11
.hidden abi_test_clobber_r11
.align	16
abi_test_clobber_r11:
_CET_ENDBR
	xorq	%r11,%r11
	.byte	0xf3,0xc3
.size	abi_test_clobber_r11,.-abi_test_clobber_r11
.type	abi_test_clobber_r12, @function
.globl	abi_test_clobber_r12
.hidden abi_test_clobber_r12
.align	16
abi_test_clobber_r12:
_CET_ENDBR
	xorq	%r12,%r12
	.byte	0xf3,0xc3
.size	abi_test_clobber_r12,.-abi_test_clobber_r12
.type	abi_test_clobber_r13, @function
.globl	abi_test_clobber_r13
.hidden abi_test_clobber_r13
.align	16
abi_test_clobber_r13:
_CET_ENDBR
	xorq	%r13,%r13
	.byte	0xf3,0xc3
.size	abi_test_clobber_r13,.-abi_test_clobber_r13
.type	abi_test_clobber_r14, @function
.globl	abi_test_clobber_r14
.hidden abi_test_clobber_r14
.align	16
abi_test_clobber_r14:
_CET_ENDBR
	xorq	%r14,%r14
	.byte	0xf3,0xc3
.size	abi_test_clobber_r14,.-abi_test_clobber_r14
.type	abi_test_clobber_r15, @function
.globl	abi_test_clobber_r15
.hidden abi_test_clobber_r15
.align	16
abi_test_clobber_r15:
_CET_ENDBR
	xorq	%r15,%r15
	.byte	0xf3,0xc3
.size	abi_test_clobber_r15,.-abi_test_clobber_r15
.type	abi_test_clobber_xmm0, @function
.globl	abi_test_clobber_xmm0
.hidden abi_test_clobber_xmm0
.align	16
abi_test_clobber_xmm0:
_CET_ENDBR
	pxor	%xmm0,%xmm0
	.byte	0xf3,0xc3
.size	abi_test_clobber_xmm0,.-abi_test_clobber_xmm0
.type	abi_test_clobber_xmm1, @function
.globl	abi_test_clobber_xmm1
.hidden abi_test_clobber_xmm1
.align	16
abi_test_clobber_xmm1:
_CET_ENDBR
	pxor	%xmm1,%xmm1
	.byte	0xf3,0xc3
.size	abi_test_clobber_xmm1,.-abi_test_clobber_xmm1
.type	abi_test_clobber_xmm2, @function
.globl	abi_test_clobber_xmm2
.hidden abi_test_clobber_xmm2
.align	16
abi_test_clobber_xmm2:
_CET_ENDBR
	pxor	%xmm2,%xmm2
	.byte	0xf3,0xc3
.size	abi_test_clobber_xmm2,.-abi_test_clobber_xmm2
.type	abi_test_clobber_xmm3, @function
.globl	abi_test_clobber_xmm3
.hidden abi_test_clobber_xmm3
.align	16
abi_test_clobber_xmm3:
_CET_ENDBR
	pxor	%xmm3,%xmm3
	.byte	0xf3,0xc3
.size	abi_test_clobber_xmm3,.-abi_test_clobber_xmm3
.type	abi_test_clobber_xmm4, @function
.globl	abi_test_clobber_xmm4
.hidden abi_test_clobber_xmm4
.align	16
abi_test_clobber_xmm4:
_CET_ENDBR
	pxor	%xmm4,%xmm4
	.byte	0xf3,0xc3
.size	abi_test_clobber_xmm4,.-abi_test_clobber_xmm4
.type	abi_test_clobber_xmm5, @function
.globl	abi_test_clobber_xmm5
.hidden abi_test_clobber_xmm5
.align	16
abi_test_clobber_xmm5:
_CET_ENDBR
	pxor	%xmm5,%xmm5
	.byte	0xf3,0xc3
.size	abi_test_clobber_xmm5,.-abi_test_clobber_xmm5
.type	abi_test_clobber_xmm6, @function
.globl	abi_test_clobber_xmm6
.hidden abi_test_clobber_xmm6
.align	16
abi_test_clobber_xmm6:
_CET_ENDBR
	pxor	%xmm6,%xmm6
	.byte	0xf3,0xc3
.size	abi_test_clobber_xmm6,.-abi_test_clobber_xmm6
.type	abi_test_clobber_xmm7, @function
.globl	abi_test_clobber_xmm7
.hidden abi_test_clobber_xmm7
.align	16
abi_test_clobber_xmm7:
_CET_ENDBR
	pxor	%xmm7,%xmm7
	.byte	0xf3,0xc3
.size	abi_test_clobber_xmm7,.-abi_test_clobber_xmm7
.type	abi_test_clobber_xmm8, @function
.globl	abi_test_clobber_xmm8
.hidden abi_test_clobber_xmm8
.align	16
abi_test_clobber_xmm8:
_CET_ENDBR
	pxor	%xmm8,%xmm8
	.byte	0xf3,0xc3
.size	abi_test_clobber_xmm8,.-abi_test_clobber_xmm8
.type	abi_test_clobber_xmm9, @function
.globl	abi_test_clobber_xmm9
.hidden abi_test_clobber_xmm9
.align	16
abi_test_clobber_xmm9:
_CET_ENDBR
	pxor	%xmm9,%xmm9
	.byte	0xf3,0xc3
.size	abi_test_clobber_xmm9,.-abi_test_clobber_xmm9
.type	abi_test_clobber_xmm10, @function
.globl	abi_test_clobber_xmm10
.hidden abi_test_clobber_xmm10
.align	16
abi_test_clobber_xmm10:
_CET_ENDBR
	pxor	%xmm10,%xmm10
	.byte	0xf3,0xc3
.size	abi_test_clobber_xmm10,.-abi_test_clobber_xmm10
.type	abi_test_clobber_xmm11, @function
.globl	abi_test_clobber_xmm11
.hidden abi_test_clobber_xmm11
.align	16
abi_test_clobber_xmm11:
_CET_ENDBR
	pxor	%xmm11,%xmm11
	.byte	0xf3,0xc3
.size	abi_test_clobber_xmm11,.-abi_test_clobber_xmm11
.type	abi_test_clobber_xmm12, @function
.globl	abi_test_clobber_xmm12
.hidden abi_test_clobber_xmm12
.align	16
abi_test_clobber_xmm12:
_CET_ENDBR
	pxor	%xmm12,%xmm12
	.byte	0xf3,0xc3
.size	abi_test_clobber_xmm12,.-abi_test_clobber_xmm12
.type	abi_test_clobber_xmm13, @function
.globl	abi_test_clobber_xmm13
.hidden abi_test_clobber_xmm13
.align	16
abi_test_clobber_xmm13:
_CET_ENDBR
	pxor	%xmm13,%xmm13
	.byte	0xf3,0xc3
.size	abi_test_clobber_xmm13,.-abi_test_clobber_xmm13
.type	abi_test_clobber_xmm14, @function
.globl	abi_test_clobber_xmm14
.hidden abi_test_clobber_xmm14
.align	16
abi_test_clobber_xmm14:
_CET_ENDBR
	pxor	%xmm14,%xmm14
	.byte	0xf3,0xc3
.size	abi_test_clobber_xmm14,.-abi_test_clobber_xmm14
.type	abi_test_clobber_xmm15, @function
.globl	abi_test_clobber_xmm15
.hidden abi_test_clobber_xmm15
.align	16
abi_test_clobber_xmm15:
_CET_ENDBR
	pxor	%xmm15,%xmm15
	.byte	0xf3,0xc3
.size	abi_test_clobber_xmm15,.-abi_test_clobber_xmm15



.type	abi_test_bad_unwind_wrong_register, @function
.globl	abi_test_bad_unwind_wrong_register
.hidden abi_test_bad_unwind_wrong_register
.align	16
abi_test_bad_unwind_wrong_register:
.cfi_startproc	

_CET_ENDBR
	pushq	%r12
.cfi_adjust_cfa_offset	8
.cfi_offset	%r13,-16




	nop
	popq	%r12
.cfi_adjust_cfa_offset	-8
.cfi_restore	%r12
	.byte	0xf3,0xc3

.cfi_endproc	
.size	abi_test_bad_unwind_wrong_register,.-abi_test_bad_unwind_wrong_register




.type	abi_test_bad_unwind_temporary, @function
.globl	abi_test_bad_unwind_temporary
.hidden abi_test_bad_unwind_temporary
.align	16
abi_test_bad_unwind_temporary:
.cfi_startproc	

_CET_ENDBR
	pushq	%r12
.cfi_adjust_cfa_offset	8
.cfi_offset	%r12,-16


	movq	%r12,%rax
	incq	%rax
	movq	%rax,(%rsp)



	movq	%r12,(%rsp)


	popq	%r12
.cfi_adjust_cfa_offset	-8
.cfi_restore	%r12
	.byte	0xf3,0xc3
.cfi_endproc	

.size	abi_test_bad_unwind_temporary,.-abi_test_bad_unwind_temporary




.type	abi_test_set_direction_flag, @function
.globl	abi_test_get_and_clear_direction_flag
.hidden abi_test_get_and_clear_direction_flag
abi_test_get_and_clear_direction_flag:
_CET_ENDBR
	pushfq
	popq	%rax
	andq	$0x400,%rax
	shrq	$10,%rax
	cld
	.byte	0xf3,0xc3
.size	abi_test_get_and_clear_direction_flag,.-abi_test_get_and_clear_direction_flag



.type	abi_test_set_direction_flag, @function
.globl	abi_test_set_direction_flag
.hidden abi_test_set_direction_flag
abi_test_set_direction_flag:
_CET_ENDBR
	std
	.byte	0xf3,0xc3
.size	abi_test_set_direction_flag,.-abi_test_set_direction_flag
#endif
