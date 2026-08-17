//===-- Utilities to log to standard output during tests --------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_BENCHMARKS_GPU_BENCHMARKLOGGER_H
#define LLVM_LIBC_BENCHMARKS_GPU_BENCHMARKLOGGER_H

#include "src/__support/macros/config.h"

namespace LIBC_NAMESPACE_DECL {
namespace benchmarks {

// A class to log to standard output in the context of hermetic tests.
struct BenchmarkLogger {
  constexpr BenchmarkLogger() = default;
  template <typename T> BenchmarkLogger &operator<<(T);
};

// A global TestLogger instance to be used in tests.
extern BenchmarkLogger log;

} // namespace benchmarks
} // namespace LIBC_NAMESPACE_DECL

#endif /* LLVM_LIBC_BENCHMARKS_GPU_BENCHMARKLOGGER_H */
