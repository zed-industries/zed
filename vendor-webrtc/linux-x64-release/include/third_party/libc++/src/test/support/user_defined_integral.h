//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_USER_DEFINED_INTEGRAL_H
#define SUPPORT_USER_DEFINED_INTEGRAL_H

#include "test_macros.h"

template <class T>
struct UserDefinedIntegral
{
    TEST_CONSTEXPR_CXX14 UserDefinedIntegral() : value(0) {}
    TEST_CONSTEXPR_CXX14 UserDefinedIntegral(T v) : value(v) {}
    TEST_CONSTEXPR_CXX14 operator T() const { return value; }
    T value;
};

// Poison the arithmetic and comparison operations
template <class T, class U>
void operator+(UserDefinedIntegral<T>, UserDefinedIntegral<U>);

template <class T, class U>
void operator-(UserDefinedIntegral<T>, UserDefinedIntegral<U>);

template <class T, class U>
void operator*(UserDefinedIntegral<T>, UserDefinedIntegral<U>);

template <class T, class U>
void operator/(UserDefinedIntegral<T>, UserDefinedIntegral<U>);

template <class T, class U>
void operator==(UserDefinedIntegral<T>, UserDefinedIntegral<U>);

template <class T, class U>
void operator!=(UserDefinedIntegral<T>, UserDefinedIntegral<U>);

template <class T, class U>
void operator<(UserDefinedIntegral<T>, UserDefinedIntegral<U>);

template <class T, class U>
void operator>(UserDefinedIntegral<T>, UserDefinedIntegral<U>);

template <class T, class U>
void operator<=(UserDefinedIntegral<T>, UserDefinedIntegral<U>);

template <class T, class U>
void operator>=(UserDefinedIntegral<T>, UserDefinedIntegral<U>);

#endif // SUPPORT_USER_DEFINED_INTEGRAL_H
