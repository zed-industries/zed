// This file is generated from a similarly-named Perl script in the BoringSSL
// source tree. Do not edit by hand.

#include <openssl/asm_base.h>

#if !defined(OPENSSL_NO_ASM) && defined(OPENSSL_X86) && defined(__ELF__)
.text
.globl	abi_test_trampoline
.hidden	abi_test_trampoline
.type	abi_test_trampoline,@function
.align	16
abi_test_trampoline:
.L_abi_test_trampoline_begin:
	pushl	%ebp
	pushl	%ebx
	pushl	%esi
	pushl	%edi
	movl	24(%esp),%ecx
	movl	(%ecx),%esi
	movl	4(%ecx),%edi
	movl	8(%ecx),%ebx
	movl	12(%ecx),%ebp
	subl	$44,%esp
	movl	72(%esp),%eax
	xorl	%ecx,%ecx
.L000loop:
	cmpl	76(%esp),%ecx
	jae	.L001loop_done
	movl	(%eax,%ecx,4),%edx
	movl	%edx,(%esp,%ecx,4)
	addl	$1,%ecx
	jmp	.L000loop
.L001loop_done:
	call	*64(%esp)
	addl	$44,%esp
	movl	24(%esp),%ecx
	movl	%esi,(%ecx)
	movl	%edi,4(%ecx)
	movl	%ebx,8(%ecx)
	movl	%ebp,12(%ecx)
	popl	%edi
	popl	%esi
	popl	%ebx
	popl	%ebp
	ret
.size	abi_test_trampoline,.-.L_abi_test_trampoline_begin
.globl	abi_test_get_and_clear_direction_flag
.hidden	abi_test_get_and_clear_direction_flag
.type	abi_test_get_and_clear_direction_flag,@function
.align	16
abi_test_get_and_clear_direction_flag:
.L_abi_test_get_and_clear_direction_flag_begin:
	pushfl
	popl	%eax
	andl	$1024,%eax
	shrl	$10,%eax
	cld
	ret
.size	abi_test_get_and_clear_direction_flag,.-.L_abi_test_get_and_clear_direction_flag_begin
.globl	abi_test_set_direction_flag
.hidden	abi_test_set_direction_flag
.type	abi_test_set_direction_flag,@function
.align	16
abi_test_set_direction_flag:
.L_abi_test_set_direction_flag_begin:
	std
	ret
.size	abi_test_set_direction_flag,.-.L_abi_test_set_direction_flag_begin
.globl	abi_test_clobber_eax
.hidden	abi_test_clobber_eax
.type	abi_test_clobber_eax,@function
.align	16
abi_test_clobber_eax:
.L_abi_test_clobber_eax_begin:
	xorl	%eax,%eax
	ret
.size	abi_test_clobber_eax,.-.L_abi_test_clobber_eax_begin
.globl	abi_test_clobber_ebx
.hidden	abi_test_clobber_ebx
.type	abi_test_clobber_ebx,@function
.align	16
abi_test_clobber_ebx:
.L_abi_test_clobber_ebx_begin:
	xorl	%ebx,%ebx
	ret
.size	abi_test_clobber_ebx,.-.L_abi_test_clobber_ebx_begin
.globl	abi_test_clobber_ecx
.hidden	abi_test_clobber_ecx
.type	abi_test_clobber_ecx,@function
.align	16
abi_test_clobber_ecx:
.L_abi_test_clobber_ecx_begin:
	xorl	%ecx,%ecx
	ret
.size	abi_test_clobber_ecx,.-.L_abi_test_clobber_ecx_begin
.globl	abi_test_clobber_edx
.hidden	abi_test_clobber_edx
.type	abi_test_clobber_edx,@function
.align	16
abi_test_clobber_edx:
.L_abi_test_clobber_edx_begin:
	xorl	%edx,%edx
	ret
.size	abi_test_clobber_edx,.-.L_abi_test_clobber_edx_begin
.globl	abi_test_clobber_edi
.hidden	abi_test_clobber_edi
.type	abi_test_clobber_edi,@function
.align	16
abi_test_clobber_edi:
.L_abi_test_clobber_edi_begin:
	xorl	%edi,%edi
	ret
.size	abi_test_clobber_edi,.-.L_abi_test_clobber_edi_begin
.globl	abi_test_clobber_esi
.hidden	abi_test_clobber_esi
.type	abi_test_clobber_esi,@function
.align	16
abi_test_clobber_esi:
.L_abi_test_clobber_esi_begin:
	xorl	%esi,%esi
	ret
.size	abi_test_clobber_esi,.-.L_abi_test_clobber_esi_begin
.globl	abi_test_clobber_ebp
.hidden	abi_test_clobber_ebp
.type	abi_test_clobber_ebp,@function
.align	16
abi_test_clobber_ebp:
.L_abi_test_clobber_ebp_begin:
	xorl	%ebp,%ebp
	ret
.size	abi_test_clobber_ebp,.-.L_abi_test_clobber_ebp_begin
.globl	abi_test_clobber_xmm0
.hidden	abi_test_clobber_xmm0
.type	abi_test_clobber_xmm0,@function
.align	16
abi_test_clobber_xmm0:
.L_abi_test_clobber_xmm0_begin:
	pxor	%xmm0,%xmm0
	ret
.size	abi_test_clobber_xmm0,.-.L_abi_test_clobber_xmm0_begin
.globl	abi_test_clobber_xmm1
.hidden	abi_test_clobber_xmm1
.type	abi_test_clobber_xmm1,@function
.align	16
abi_test_clobber_xmm1:
.L_abi_test_clobber_xmm1_begin:
	pxor	%xmm1,%xmm1
	ret
.size	abi_test_clobber_xmm1,.-.L_abi_test_clobber_xmm1_begin
.globl	abi_test_clobber_xmm2
.hidden	abi_test_clobber_xmm2
.type	abi_test_clobber_xmm2,@function
.align	16
abi_test_clobber_xmm2:
.L_abi_test_clobber_xmm2_begin:
	pxor	%xmm2,%xmm2
	ret
.size	abi_test_clobber_xmm2,.-.L_abi_test_clobber_xmm2_begin
.globl	abi_test_clobber_xmm3
.hidden	abi_test_clobber_xmm3
.type	abi_test_clobber_xmm3,@function
.align	16
abi_test_clobber_xmm3:
.L_abi_test_clobber_xmm3_begin:
	pxor	%xmm3,%xmm3
	ret
.size	abi_test_clobber_xmm3,.-.L_abi_test_clobber_xmm3_begin
.globl	abi_test_clobber_xmm4
.hidden	abi_test_clobber_xmm4
.type	abi_test_clobber_xmm4,@function
.align	16
abi_test_clobber_xmm4:
.L_abi_test_clobber_xmm4_begin:
	pxor	%xmm4,%xmm4
	ret
.size	abi_test_clobber_xmm4,.-.L_abi_test_clobber_xmm4_begin
.globl	abi_test_clobber_xmm5
.hidden	abi_test_clobber_xmm5
.type	abi_test_clobber_xmm5,@function
.align	16
abi_test_clobber_xmm5:
.L_abi_test_clobber_xmm5_begin:
	pxor	%xmm5,%xmm5
	ret
.size	abi_test_clobber_xmm5,.-.L_abi_test_clobber_xmm5_begin
.globl	abi_test_clobber_xmm6
.hidden	abi_test_clobber_xmm6
.type	abi_test_clobber_xmm6,@function
.align	16
abi_test_clobber_xmm6:
.L_abi_test_clobber_xmm6_begin:
	pxor	%xmm6,%xmm6
	ret
.size	abi_test_clobber_xmm6,.-.L_abi_test_clobber_xmm6_begin
.globl	abi_test_clobber_xmm7
.hidden	abi_test_clobber_xmm7
.type	abi_test_clobber_xmm7,@function
.align	16
abi_test_clobber_xmm7:
.L_abi_test_clobber_xmm7_begin:
	pxor	%xmm7,%xmm7
	ret
.size	abi_test_clobber_xmm7,.-.L_abi_test_clobber_xmm7_begin
#endif  // !defined(OPENSSL_NO_ASM) && defined(OPENSSL_X86) && defined(__ELF__)
