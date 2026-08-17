//===-- BSD / GNU header <subdir/test.h> --===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===---------------------------------------------------------------------===//

#ifndef _LLVM_LIBC_SUBDIR_TEST_H
#define _LLVM_LIBC_SUBDIR_TEST_H

#include "../__llvm-libc-common.h"
#include "../llvm-libc-types/type_a.h"
#include "../llvm-libc-types/type_b.h"

__BEGIN_C_DECLS

type_a func(type_b) __NOEXCEPT;

void gnufunc(type_a) __NOEXCEPT;

int *ptrfunc(void) __NOEXCEPT;

__END_C_DECLS

#endif // _LLVM_LIBC_SUBDIR_TEST_H
