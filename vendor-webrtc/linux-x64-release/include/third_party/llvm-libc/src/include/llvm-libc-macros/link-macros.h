//===-- Definition of macros to for extra dynamic linker functionality ----===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_MACROS_LINK_MACROS_H
#define LLVM_LIBC_MACROS_LINK_MACROS_H

#include "elf-macros.h"

#ifdef __LP64__
#define ElfW(type) Elf64_##type
#else
#define ElfW(type) Elf32_##type
#endif

#endif
