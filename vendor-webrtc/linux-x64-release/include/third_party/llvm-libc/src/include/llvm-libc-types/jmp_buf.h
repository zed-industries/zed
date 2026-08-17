//===-- Definition of type jmp_buf ----------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TYPES_JMP_BUF_H
#define LLVM_LIBC_TYPES_JMP_BUF_H

typedef struct {
#ifdef __x86_64__
  __UINT64_TYPE__ rbx;
  __UINT64_TYPE__ rbp;
  __UINT64_TYPE__ r12;
  __UINT64_TYPE__ r13;
  __UINT64_TYPE__ r14;
  __UINT64_TYPE__ r15;
  __UINTPTR_TYPE__ rsp;
  __UINTPTR_TYPE__ rip;
#elif defined(__i386__)
  long ebx;
  long esi;
  long edi;
  long ebp;
  long esp;
  long eip;
#elif defined(__riscv)
  /* Program counter.  */
  long int __pc;
  /* Callee-saved registers.  */
  long int __regs[12];
  /* Stack pointer.  */
  long int __sp;
  /* Callee-saved floating point registers.  */
#if __riscv_float_abi_double
  double __fpregs[12];
#elif defined(__riscv_float_abi_single)
#error "__jmp_buf not available for your target architecture."
#endif
#elif defined(__arm__)
  // r4, r5, r6, r7, r8, r9, r10, r11, r12, lr
  long opaque[10];
#elif defined(__aarch64__)
  long opaque[14]; // x19-x29, lr, sp, optional x18
#if __ARM_FP
  long fopaque[8]; // d8-d15
#endif
#else
#error "__jmp_buf not available for your target architecture."
#endif
} __jmp_buf;

typedef __jmp_buf jmp_buf[1];

#endif // LLVM_LIBC_TYPES_JMP_BUF_H
