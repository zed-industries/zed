//===-- JSON serialization routines -----------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_UTILS_BENCHMARK_JSON_H
#define LLVM_LIBC_UTILS_BENCHMARK_JSON_H

#include "LibcBenchmark.h"
#include "LibcMemoryBenchmark.h"
#include "llvm/Support/JSON.h"

namespace llvm {
namespace libc_benchmarks {

// Parses a Study from a json string.
Expected<Study> parseJsonStudy(StringRef Content);

// Serialize a Study as json.
void serializeToJson(const Study &S, llvm::json::OStream &JOS);

} // namespace libc_benchmarks
} // namespace llvm

#endif // LLVM_LIBC_UTILS_BENCHMARK_JSON_H
