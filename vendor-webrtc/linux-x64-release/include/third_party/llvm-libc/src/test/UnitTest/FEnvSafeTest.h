//===-- FEnvSafeTest.h -----------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===---------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_UNITTEST_FPENVSAFE_H
#define LLVM_LIBC_TEST_UNITTEST_FPENVSAFE_H

#include "hdr/types/fenv_t.h"
#include "src/__support/CPP/utility.h"
#include "src/__support/macros/config.h"
#include "test/UnitTest/Test.h"

namespace LIBC_NAMESPACE_DECL {
namespace testing {

// This provides a test fixture (or base class for other test fixtures) that
// asserts that each test does not leave the FPU state represented by `fenv_t`
// (aka `FPState`) perturbed from its initial state.
class FEnvSafeTest : public Test {
public:
  void TearDown() override;

protected:
  // This is an RAII type where `PreserveFEnv preserve{this};` will sample the
  // `fenv_t` state and restore it when `preserve` goes out of scope.
  class PreserveFEnv {
    fenv_t before;
    FEnvSafeTest &test;

  public:
    explicit PreserveFEnv(FEnvSafeTest *self) : test{*self} {
      test.get_fenv(before);
    }

    // Cause test expectation failures if the current state doesn't match what
    // was captured in the constructor.
    void check();

    // Restore the state captured in the constructor.
    void restore() { test.set_fenv(before); }

    ~PreserveFEnv() { restore(); }
  };

  // This is an RAII type where `CheckFEnv check{this};` will sample the
  // `fenv_t` state and require it be the same when `check` goes out of scope.
  struct CheckFEnv : public PreserveFEnv {
    using PreserveFEnv::PreserveFEnv;

    ~CheckFEnv() { check(); }
  };

  // This calls callable() and returns its value, but has EXPECT_* failures if
  // the `fenv_t` state is not preserved by the call.
  template <typename T> decltype(auto) check_fenv_preserved(T &&callable) {
    CheckFEnv check{this};
    return cpp::forward<T>(callable)();
  }

  // This calls callable() and returns its value, but saves and restores the
  // `fenv_t` state around the call.
  template <typename T>
  auto with_fenv_preserved(T &&callable)
      -> decltype(cpp::forward<decltype(callable)>(callable)()) {
    PreserveFEnv preserve{this};
    return cpp::forward<T>(callable)();
  }

  // A test can call these to indicate it will or won't change `fenv_t` state.
  void will_change_fenv() { should_be_unchanged = false; }
  void will_not_change_fenv() { should_be_unchanged = true; }

  // This explicitly resets back to the "before" state captured in SetUp().
  // TearDown() always does this, but should_be_unchanged controls whether
  // it also causes test failures if a test fails to restore it.
  void restore_fenv() { check.restore(); }

private:
  void get_fenv(fenv_t &fenv);
  void set_fenv(const fenv_t &fenv);
  void expect_fenv_eq(const fenv_t &before_fenv, const fenv_t &after_fenv);

  CheckFEnv check{this};

  // TODO: Many tests fail if this is true. It needs to be figured out whether
  // the state should be preserved by each library function under test, and
  // separately whether each test itself should preserve the state.  It
  // probably isn't important that tests be explicitly written to preserve the
  // state, as the fixture can (and does) reset it--the next test can rely on
  // getting "normal" ambient state initially.  For library functions that
  // should preserve the state, that should be checked after each call, not
  // just after the whole test.  So they can use check_fenv_preserved or
  // with_fenv_preserved as appropriate.
  bool should_be_unchanged = false;
};

} // namespace testing
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_TEST_UNITTEST_FPENVSAFE_H
