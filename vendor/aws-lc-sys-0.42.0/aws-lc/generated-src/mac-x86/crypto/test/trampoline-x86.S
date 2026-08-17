// This file is generated from a similarly-named Perl script in the BoringSSL
// source tree. Do not edit by hand.

#include <openssl/asm_base.h>

#if !defined(OPENSSL_NO_ASM) && defined(OPENSSL_X86) && defined(__APPLE__)
.text
.globl	_abi_test_trampoline
.private_extern	_abi_test_trampoline
.align	4
_abi_test_trampoline:
L_abi_test_trampoline_begin:
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
L000loop:
	cmpl	76(%esp),%ecx
	jae	L001loop_done
	movl	(%eax,%ecx,4),%edx
	movl	%edx,(%esp,%ecx,4)
	addl	$1,%ecx
	jmp	L000loop
L001loop_done:
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
.globl	_abi_test_get_and_clear_direction_flag
.private_extern	_abi_test_get_and_clear_direction_flag
.align	4
_abi_test_get_and_clear_direction_flag:
L_abi_test_get_and_clear_direction_flag_begin:
	pushfl
	popl	%eax
	andl	$1024,%eax
	shrl	$10,%eax
	cld
	ret
.globl	_abi_test_set_direction_flag
.private_extern	_abi_test_set_direction_flag
.align	4
_abi_test_set_direction_flag:
L_abi_test_set_direction_flag_begin:
	std
	ret
.globl	_abi_test_clobber_eax
.private_extern	_abi_test_clobber_eax
.align	4
_abi_test_clobber_eax:
L_abi_test_clobber_eax_begin:
	xorl	%eax,%eax
	ret
.globl	_abi_test_clobber_ebx
.private_extern	_abi_test_clobber_ebx
.align	4
_abi_test_clobber_ebx:
L_abi_test_clobber_ebx_begin:
	xorl	%ebx,%ebx
	ret
.globl	_abi_test_clobber_ecx
.private_extern	_abi_test_clobber_ecx
.align	4
_abi_test_clobber_ecx:
L_abi_test_clobber_ecx_begin:
	xorl	%ecx,%ecx
	ret
.globl	_abi_test_clobber_edx
.private_extern	_abi_test_clobber_edx
.align	4
_abi_test_clobber_edx:
L_abi_test_clobber_edx_begin:
	xorl	%edx,%edx
	ret
.globl	_abi_test_clobber_edi
.private_extern	_abi_test_clobber_edi
.align	4
_abi_test_clobber_edi:
L_abi_test_clobber_edi_begin:
	xorl	%edi,%edi
	ret
.globl	_abi_test_clobber_esi
.private_extern	_abi_test_clobber_esi
.align	4
_abi_test_clobber_esi:
L_abi_test_clobber_esi_begin:
	xorl	%esi,%esi
	ret
.globl	_abi_test_clobber_ebp
.private_extern	_abi_test_clobber_ebp
.align	4
_abi_test_clobber_ebp:
L_abi_test_clobber_ebp_begin:
	xorl	%ebp,%ebp
	ret
.globl	_abi_test_clobber_xmm0
.private_extern	_abi_test_clobber_xmm0
.align	4
_abi_test_clobber_xmm0:
L_abi_test_clobber_xmm0_begin:
	pxor	%xmm0,%xmm0
	ret
.globl	_abi_test_clobber_xmm1
.private_extern	_abi_test_clobber_xmm1
.align	4
_abi_test_clobber_xmm1:
L_abi_test_clobber_xmm1_begin:
	pxor	%xmm1,%xmm1
	ret
.globl	_abi_test_clobber_xmm2
.private_extern	_abi_test_clobber_xmm2
.align	4
_abi_test_clobber_xmm2:
L_abi_test_clobber_xmm2_begin:
	pxor	%xmm2,%xmm2
	ret
.globl	_abi_test_clobber_xmm3
.private_extern	_abi_test_clobber_xmm3
.align	4
_abi_test_clobber_xmm3:
L_abi_test_clobber_xmm3_begin:
	pxor	%xmm3,%xmm3
	ret
.globl	_abi_test_clobber_xmm4
.private_extern	_abi_test_clobber_xmm4
.align	4
_abi_test_clobber_xmm4:
L_abi_test_clobber_xmm4_begin:
	pxor	%xmm4,%xmm4
	ret
.globl	_abi_test_clobber_xmm5
.private_extern	_abi_test_clobber_xmm5
.align	4
_abi_test_clobber_xmm5:
L_abi_test_clobber_xmm5_begin:
	pxor	%xmm5,%xmm5
	ret
.globl	_abi_test_clobber_xmm6
.private_extern	_abi_test_clobber_xmm6
.align	4
_abi_test_clobber_xmm6:
L_abi_test_clobber_xmm6_begin:
	pxor	%xmm6,%xmm6
	ret
.globl	_abi_test_clobber_xmm7
.private_extern	_abi_test_clobber_xmm7
.align	4
_abi_test_clobber_xmm7:
L_abi_test_clobber_xmm7_begin:
	pxor	%xmm7,%xmm7
	ret
#endif  // !defined(OPENSSL_NO_ASM) && defined(OPENSSL_X86) && defined(__APPLE__)
