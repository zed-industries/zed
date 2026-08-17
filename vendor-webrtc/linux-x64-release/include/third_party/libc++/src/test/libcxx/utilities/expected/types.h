//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_LIBCXX_UTILITIES_EXPECTED_TYPES_H
#define TEST_LIBCXX_UTILITIES_EXPECTED_TYPES_H

#include <initializer_list>

struct DefaultMayThrow {
  DefaultMayThrow();
};

struct CopyMayThrow {
  CopyMayThrow(const CopyMayThrow&);
};

struct ConvertFromCopyIntMayThrow {
  ConvertFromCopyIntMayThrow(const int&);
  ConvertFromCopyIntMayThrow(int&&) noexcept;
};

struct ConvertFromMoveIntMayThrow {
  ConvertFromMoveIntMayThrow(const int&) noexcept;
  ConvertFromMoveIntMayThrow(int&&);
};

struct ConvertFromInitializerListNoexcept {
  ConvertFromInitializerListNoexcept(std::initializer_list<int>) noexcept;
};

struct ConvertFromInitializerListMayThrow {
  ConvertFromInitializerListMayThrow(std::initializer_list<int>);
};

struct CopyConstructMayThrow {
  CopyConstructMayThrow(const CopyConstructMayThrow&);
  CopyConstructMayThrow& operator=(CopyConstructMayThrow const&) noexcept;
};

struct CopyAssignMayThrow {
  CopyAssignMayThrow(const CopyAssignMayThrow&) noexcept;
  CopyAssignMayThrow& operator=(CopyAssignMayThrow const&);
};


#endif // TEST_LIBCXX_UTILITIES_EXPECTED_TYPES_H
