//===-- Definition of macros from elf.h -----------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_MACROS_ELF_MACROS_H
#define LLVM_LIBC_MACROS_ELF_MACROS_H

#if __has_include(<linux/elf.h>)
#include <linux/elf.h>
#else
#error "cannot use <sys/elf.h> without proper system headers."
#endif

#endif // LLVM_LIBC_MACROS_ELF_MACROS_H
