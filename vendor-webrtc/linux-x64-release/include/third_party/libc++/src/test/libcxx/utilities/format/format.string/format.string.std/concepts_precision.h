//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LIBCXX_TEST_STD_UTILITIES_FORMAT_FORMAT_STRING_FORMAT_STRING_STD_CONCEPTS_PRECISION_H
#define LIBCXX_TEST_STD_UTILITIES_FORMAT_FORMAT_STRING_FORMAT_STRING_STD_CONCEPTS_PRECISION_H

template <class T>
concept has_precision = requires(T parser) {
  parser.__precision;
};

template <class T>
concept has_precision_as_arg = requires(T parser) {
  parser.__precision_as_arg;
};

#endif // LIBCXX_TEST_STD_UTILITIES_FORMAT_FORMAT_STRING_FORMAT_STRING_STD_CONCEPTS_PRECISION_H
