//===-- ErrnoSetterMatcher.h ------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_ERRNOSETTERMATCHER_H
#define LLVM_LIBC_TEST_ERRNOSETTERMATCHER_H

#include "src/__support/FPUtil/FPBits.h"
#include "src/__support/FPUtil/fpbits_str.h"
#include "src/__support/StringUtil/error_to_string.h"
#include "src/__support/macros/config.h"
#include "src/__support/macros/properties/architectures.h"
#include "src/errno/libc_errno.h"
#include "test/UnitTest/Test.h"

namespace LIBC_NAMESPACE_DECL {
namespace testing {

namespace internal {

enum class CompareAction { EQ = 0, GE, GT, LE, LT, NE };

constexpr const char *CompareMessage[] = {
    "equal to",     "greater than or equal to",
    "greater than", "less than or equal to",
    "less than",    "not equal to"};

template <typename T> struct Comparator {
  CompareAction cmp;
  T expected;
  bool compare(T actual) {
    switch (cmp) {
    case CompareAction::EQ:
      return actual == expected;
    case CompareAction::NE:
      return actual != expected;
    case CompareAction::GE:
      return actual >= expected;
    case CompareAction::GT:
      return actual > expected;
    case CompareAction::LE:
      return actual <= expected;
    case CompareAction::LT:
      return actual < expected;
    }
    __builtin_unreachable();
  }

  // The NVPTX backend cannot handle circular dependencies on global variables.
  // We provide a constant dummy implementation to prevent this from occurring.
#ifdef LIBC_TARGET_ARCH_IS_NVPTX
  constexpr const char *str() { return ""; }
#else
  const char *str() { return CompareMessage[static_cast<int>(cmp)]; }
#endif
};

template <typename T> class ErrnoSetterMatcher : public Matcher<T> {
  Comparator<T> return_cmp;
  Comparator<int> errno_cmp;
  T actual_return;
  int actual_errno;

  // Even though this is a errno matcher primarily, it has to cater to platforms
  // which do not have an errno. This predicate checks if errno matching is to
  // be skipped.
  static constexpr bool ignore_errno() {
#ifdef LIBC_TARGET_ARCH_IS_GPU
    return true;
#else
    return false;
#endif
  }

public:
  ErrnoSetterMatcher(Comparator<T> rcmp) : return_cmp(rcmp) {}
  ErrnoSetterMatcher(Comparator<T> rcmp, Comparator<int> ecmp)
      : return_cmp(rcmp), errno_cmp(ecmp) {}

  ErrnoSetterMatcher<T> with_errno(Comparator<int> ecmp) {
    errno_cmp = ecmp;
    return *this;
  }

  void explainError() override {
    if (!return_cmp.compare(actual_return)) {
      if constexpr (cpp::is_floating_point_v<T>) {
        tlog << "Expected return value to be " << return_cmp.str() << ": "
             << str(fputil::FPBits<T>(return_cmp.expected)) << '\n'
             << "                    But got: "
             << str(fputil::FPBits<T>(actual_return)) << '\n';
      } else {
        tlog << "Expected return value to be " << return_cmp.str() << " "
             << return_cmp.expected << " but got " << actual_return << ".\n";
      }
    }

    if constexpr (!ignore_errno()) {
      if (!errno_cmp.compare(actual_errno)) {
        auto expected_str = try_get_errno_name(errno_cmp.expected);
        auto actual_str = try_get_errno_name(actual_errno);
        tlog << "Expected errno to be " << errno_cmp.str() << " "
             << (expected_str ? *expected_str : "<unknown>") << "("
             << errno_cmp.expected << ") but got "
             << (actual_str ? *actual_str : "<unknown>") << "(" << actual_errno
             << ").\n";
      }
    }
  }

  bool match(T got) {
    actual_return = got;
    actual_errno = LIBC_NAMESPACE::libc_errno;
    LIBC_NAMESPACE::libc_errno = 0;
    if constexpr (ignore_errno())
      return return_cmp.compare(actual_return);
    else
      return return_cmp.compare(actual_return) &&
             errno_cmp.compare(actual_errno);
  }
};

} // namespace internal

namespace ErrnoSetterMatcher {

template <typename T> internal::Comparator<T> LT(T val) {
  return internal::Comparator<T>{internal::CompareAction::LT, val};
}

template <typename T> internal::Comparator<T> LE(T val) {
  return internal::Comparator<T>{internal::CompareAction::LE, val};
}

template <typename T> internal::Comparator<T> GT(T val) {
  return internal::Comparator<T>{internal::CompareAction::GT, val};
}

template <typename T> internal::Comparator<T> GE(T val) {
  return internal::Comparator<T>{internal::CompareAction::GE, val};
}

template <typename T> internal::Comparator<T> EQ(T val) {
  return internal::Comparator<T>{internal::CompareAction::EQ, val};
}

template <typename T> internal::Comparator<T> NE(T val) {
  return internal::Comparator<T>{internal::CompareAction::NE, val};
}

template <typename RetT = int>
static internal::ErrnoSetterMatcher<RetT> Succeeds(RetT ExpectedReturn = 0,
                                                   int ExpectedErrno = 0) {
  return internal::ErrnoSetterMatcher<RetT>(EQ(ExpectedReturn),
                                            EQ(ExpectedErrno));
}

template <typename RetT = int>
static internal::ErrnoSetterMatcher<RetT> Fails(int ExpectedErrno,
                                                RetT ExpectedReturn = -1) {
  return internal::ErrnoSetterMatcher<RetT>(EQ(ExpectedReturn),
                                            EQ(ExpectedErrno));
}

template <typename RetT = int> class ErrnoSetterMatcherBuilder {
public:
  template <typename T> using Cmp = internal::Comparator<T>;
  ErrnoSetterMatcherBuilder(Cmp<RetT> cmp) : return_cmp(cmp) {}

  internal::ErrnoSetterMatcher<RetT> with_errno(Cmp<int> cmp) {
    return internal::ErrnoSetterMatcher<RetT>(return_cmp, cmp);
  }

private:
  Cmp<RetT> return_cmp;
};

template <typename RetT>
static ErrnoSetterMatcherBuilder<RetT> returns(internal::Comparator<RetT> cmp) {
  return ErrnoSetterMatcherBuilder<RetT>(cmp);
}

} // namespace ErrnoSetterMatcher

} // namespace testing
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_TEST_ERRNOSETTERMATCHER_H
