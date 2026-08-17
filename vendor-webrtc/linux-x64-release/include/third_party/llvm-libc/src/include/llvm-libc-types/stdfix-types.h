//===-- Definition of stdfix integer types --------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TYPES_STDFIX_TYPES_H
#define LLVM_LIBC_TYPES_STDFIX_TYPES_H

typedef signed char int_hr_t;
typedef signed short int int_r_t;
typedef signed int int_lr_t;
typedef signed short int_hk_t;
typedef signed int int_k_t;
typedef signed long long int_lk_t;
typedef unsigned char uint_uhr_t;
typedef unsigned short int uint_ur_t;
typedef unsigned int uint_ulr_t;
typedef unsigned short int uint_uhk_t;
typedef unsigned int uint_uk_t;
typedef unsigned long long uint_ulk_t;

#endif // LLVM_LIBC_TYPES_STDFIX_TYPES_H
