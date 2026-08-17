/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_TEST_REGISTER_STATE_CHECK_H_
#define AOM_TEST_REGISTER_STATE_CHECK_H_

#include "gtest/gtest.h"

#include "config/aom_config.h"

#include "aom/aom_integer.h"

// API_REGISTER_STATE_CHECK(function)
//   Validates the environment pre & post function execution to ensure the
//   environment is in a consistent state. This should be used with API
//   function sand assembly functions which are not expected to fully restore
//   the system state.
//   See platform implementations of RegisterStateCheck and
//   RegisterStateCheckMMX for details.

#if defined(_WIN64) && AOM_ARCH_X86_64

#undef NOMINMAX
#define NOMINMAX
#undef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <winnt.h>

inline bool operator==(const M128A &lhs, const M128A &rhs) {
  return (lhs.Low == rhs.Low && lhs.High == rhs.High);
}

namespace libaom_test {

// Compares the state of xmm[6-15] at construction with their state at
// destruction. These registers should be preserved by the callee on
// Windows x64.
class RegisterStateCheck {
 public:
  RegisterStateCheck() { initialized_ = StoreRegisters(&pre_context_); }
  ~RegisterStateCheck() { Check(); }

 private:
  static bool StoreRegisters(CONTEXT *const context) {
    const HANDLE this_thread = GetCurrentThread();
    EXPECT_NE(this_thread, nullptr);
    context->ContextFlags = CONTEXT_FLOATING_POINT;
    const bool context_saved = GetThreadContext(this_thread, context) == TRUE;
    EXPECT_TRUE(context_saved) << "GetLastError: " << GetLastError();
    return context_saved;
  }

  // Compares the register state. Returns true if the states match.
  void Check() const {
    ASSERT_TRUE(initialized_);
    CONTEXT post_context;
    ASSERT_TRUE(StoreRegisters(&post_context));

    const M128A *xmm_pre = &pre_context_.Xmm6;
    const M128A *xmm_post = &post_context.Xmm6;
    for (int i = 6; i <= 15; ++i) {
      EXPECT_EQ(*xmm_pre, *xmm_post) << "xmm" << i << " has been modified!";
      ++xmm_pre;
      ++xmm_post;
    }
  }

  bool initialized_;
  CONTEXT pre_context_;
};
}  // namespace libaom_test

#else

namespace libaom_test {

class RegisterStateCheck {};
}  // namespace libaom_test

#endif  // _WIN64 && AOM_ARCH_X86_64

#if (AOM_ARCH_X86 || AOM_ARCH_X86_64) && defined(__GNUC__)
namespace libaom_test {

// Checks the FPU tag word pre/post execution to ensure emms has been called.
class RegisterStateCheckMMX {
 public:
  RegisterStateCheckMMX() {
    __asm__ volatile("fstenv %0" : "=rm"(pre_fpu_env_));
  }
  ~RegisterStateCheckMMX() { Check(); }

 private:
  // Checks the FPU tag word pre/post execution, returning false if not cleared
  // to 0xffff.
  void Check() const {
    EXPECT_EQ(0xffff, pre_fpu_env_[4])
        << "FPU was in an inconsistent state prior to call";

    uint16_t post_fpu_env[14];
    __asm__ volatile("fstenv %0" : "=rm"(post_fpu_env));
    EXPECT_EQ(0xffff, post_fpu_env[4])
        << "FPU was left in an inconsistent state after call";
  }

  uint16_t pre_fpu_env_[14];
};
}  // namespace libaom_test

#else
namespace libaom_test {

class RegisterStateCheckMMX {};
}  // namespace libaom_test

#endif  // (AOM_ARCH_X86 || AOM_ARCH_X86_64) && defined(__GNUC__)

#define API_REGISTER_STATE_CHECK(statement)           \
  do {                                                \
    libaom_test::RegisterStateCheck reg_check;        \
    libaom_test::RegisterStateCheckMMX reg_check_mmx; \
    statement;                                        \
    (void)reg_check_mmx;                              \
    (void)reg_check;                                  \
  } while (false)

#endif  // AOM_TEST_REGISTER_STATE_CHECK_H_
