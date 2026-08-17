//===-- Definition of macros to be used with complex functions ------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef __LLVM_LIBC_MACROS_COMPLEX_MACROS_H
#define __LLVM_LIBC_MACROS_COMPLEX_MACROS_H

#ifndef __STDC_NO_COMPLEX__

#define __STDC_VERSION_COMPLEX_H__ 202311L

#define complex _Complex
#define _Complex_I ((_Complex float)1.0fi)
#define I _Complex_I

// TODO: Add imaginary macros once GCC or Clang support _Imaginary builtin-type.

#endif

#endif // __LLVM_LIBC_MACROS_COMPLEX_MACROS_H
