//===-- MemorySizeDistributions ---------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// Memory functions operate on buffers of certain sizes. This file provides
// probability distributions observed at runtime for a set of applications.
// These distributions are used to benchmark and compare memory functions
// implementations.
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_BENCHMARKS_MEMORYSIZEDISTRIBUTIONS_H
#define LLVM_LIBC_BENCHMARKS_MEMORYSIZEDISTRIBUTIONS_H

#include <llvm/ADT/ArrayRef.h>
#include <llvm/ADT/StringRef.h>

namespace llvm {
namespace libc_benchmarks {

/// A simple POD exposing caracteristics of a memory function size
/// distributions. The underlying data is immutable.
struct MemorySizeDistribution {
  StringRef Name;                 // The name of the distribution.
  ArrayRef<double> Probabilities; // Size indexed array of probabilities.
};

/// Returns a list of memmove size distributions.
ArrayRef<MemorySizeDistribution> getMemmoveSizeDistributions();

/// Returns a list of memcpy size distributions.
ArrayRef<MemorySizeDistribution> getMemcpySizeDistributions();

/// Returns a list of memset size distributions.
ArrayRef<MemorySizeDistribution> getMemsetSizeDistributions();

/// Returns a list of memcmp size distributions.
ArrayRef<MemorySizeDistribution> getMemcmpSizeDistributions();

/// Returns the first MemorySizeDistribution from Distributions with the
/// specified Name.
MemorySizeDistribution
getDistributionOrDie(ArrayRef<MemorySizeDistribution> Distributions,
                     StringRef Name);

} // namespace libc_benchmarks
} // namespace llvm

#endif // LLVM_LIBC_BENCHMARKS_MEMORYSIZEDISTRIBUTIONS_H
