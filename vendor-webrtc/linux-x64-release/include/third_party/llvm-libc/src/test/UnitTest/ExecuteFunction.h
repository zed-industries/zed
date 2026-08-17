//===-- ExecuteFunction.h ---------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_UNITTEST_EXECUTEFUNCTION_H
#define LLVM_LIBC_TEST_UNITTEST_EXECUTEFUNCTION_H

#include "src/__support/CPP/limits.h"
#include "src/__support/macros/config.h"
#include <stdint.h>

namespace LIBC_NAMESPACE_DECL {
namespace testutils {

class FunctionCaller {
public:
  virtual ~FunctionCaller() {}
  virtual void operator()() = 0;
};

struct ProcessStatus {
  int platform_defined;
  const char *failure = nullptr;

  static constexpr int TIMEOUT = cpp::numeric_limits<int>::max();

  static ProcessStatus error(const char *error) { return {0, error}; }
  static ProcessStatus timed_out_ps() {
    return {0, reinterpret_cast<const char *>(TIMEOUT)};
  }

  bool timed_out() const {
    return failure == reinterpret_cast<const char *>(TIMEOUT);
  }
  const char *get_error() const { return timed_out() ? nullptr : failure; }
  bool exited_normally();
  int get_exit_code();
  int get_fatal_signal();
};

ProcessStatus invoke_in_subprocess(FunctionCaller *func,
                                   int timeout_ms = ProcessStatus::TIMEOUT);

const char *signal_as_string(int signum);

} // namespace testutils
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_TEST_UNITTEST_EXECUTEFUNCTION_H
