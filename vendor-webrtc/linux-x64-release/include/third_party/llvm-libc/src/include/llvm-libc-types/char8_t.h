//===-- Definition of char8_t type ----------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TYPES_CHAR8_T_H
#define LLVM_LIBC_TYPES_CHAR8_T_H

#if !defined(__cplusplus) && defined(__STDC_VERSION__) &&                      \
    __STDC_VERSION__ >= 202311L
typedef unsigned char char8_t;
#endif

#endif // LLVM_LIBC_TYPES_CHAR8_T_H
