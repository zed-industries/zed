//===-- Benchmark memory specific tools -------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

// This file complements the `benchmark` header with memory specific tools and
// benchmarking facilities.

#ifndef LLVM_LIBC_UTILS_BENCHMARK_MEMORY_BENCHMARK_H
#define LLVM_LIBC_UTILS_BENCHMARK_MEMORY_BENCHMARK_H

#include "LibcBenchmark.h"
#include "LibcFunctionPrototypes.h"
#include "MemorySizeDistributions.h"
#include "llvm/ADT/SmallVector.h"
#include "llvm/Support/Alignment.h"
#include "llvm/Support/MathExtras.h"
#include <cstdint>
#include <optional>
#include <random>

namespace llvm {
namespace libc_benchmarks {

//--------------
// Configuration
//--------------

struct StudyConfiguration {
  // One of 'memcpy', 'memset', 'memcmp'.
  // The underlying implementation is always the llvm libc one.
  // e.g. 'memcpy' will test 'LIBC_NAMESPACE::memcpy'
  std::string Function;

  // The number of trials to run for this benchmark.
  // If in SweepMode, each individual sizes are measured 'NumTrials' time.
  // i.e 'NumTrials' measurements for 0, 'NumTrials' measurements for 1 ...
  uint32_t NumTrials = 1;

  // Toggles between Sweep Mode and Distribution Mode (default).
  // See 'SweepModeMaxSize' and 'SizeDistributionName' below.
  bool IsSweepMode = false;

  // Maximum size to use when measuring a ramp of size values (SweepMode).
  // The benchmark measures all sizes from 0 to SweepModeMaxSize.
  // Note: in sweep mode the same size is sampled several times in a row this
  // will allow the processor to learn it and optimize the branching pattern.
  // The resulting measurement is likely to be idealized.
  uint32_t SweepModeMaxSize = 0; // inclusive

  // The name of the distribution to be used to randomize the size parameter.
  // This is used when SweepMode is false (default).
  std::string SizeDistributionName;

  // This parameter allows to control how the buffers are accessed during
  // benchmark:
  // None : Use a fixed address that is at least cache line aligned,
  //    1 : Use random address,
  //   >1 : Use random address aligned to value.
  MaybeAlign AccessAlignment = std::nullopt;

  // When Function == 'memcmp', this is the buffers mismatch position.
  //  0 : Buffers always compare equal,
  // >0 : Buffers compare different at byte N-1.
  uint32_t MemcmpMismatchAt = 0;
};

struct Runtime {
  // Details about the Host (cpu name, cpu frequency, cache hierarchy).
  HostState Host;

  // The framework will populate this value so all data accessed during the
  // benchmark will stay in L1 data cache. This includes bookkeeping data.
  uint32_t BufferSize = 0;

  // This is the number of distinct parameters used in a single batch.
  // The framework always tests a batch of randomized parameter to prevent the
  // cpu from learning branching patterns.
  uint32_t BatchParameterCount = 0;

  // The benchmark options that were used to perform the measurement.
  // This is decided by the framework.
  BenchmarkOptions BenchmarkOptions;
};

//--------
// Results
//--------

// The root object containing all the data (configuration and measurements).
struct Study {
  std::string StudyName;
  Runtime Runtime;
  StudyConfiguration Configuration;
  std::vector<Duration> Measurements;
};

//------
// Utils
//------

// Provides an aligned, dynamically allocated buffer.
class AlignedBuffer {
  char *const Buffer = nullptr;
  size_t Size = 0;

public:
  static constexpr size_t Alignment = 512;

  explicit AlignedBuffer(size_t Size)
      : Buffer(static_cast<char *>(
            aligned_alloc(Alignment, alignTo(Size, Alignment)))),
        Size(Size) {}
  ~AlignedBuffer() { free(Buffer); }

  inline char *operator+(size_t Index) { return Buffer + Index; }
  inline const char *operator+(size_t Index) const { return Buffer + Index; }
  inline char &operator[](size_t Index) { return Buffer[Index]; }
  inline const char &operator[](size_t Index) const { return Buffer[Index]; }
  inline char *begin() { return Buffer; }
  inline char *end() { return Buffer + Size; }
};

// Helper to generate random buffer offsets that satisfy the configuration
// constraints.
class OffsetDistribution {
  std::uniform_int_distribution<uint32_t> Distribution;
  uint32_t Factor;

public:
  explicit OffsetDistribution(size_t BufferSize, size_t MaxSizeValue,
                              MaybeAlign AccessAlignment);

  template <class Generator> uint32_t operator()(Generator &G) {
    return Distribution(G) * Factor;
  }
};

// Helper to generate random buffer offsets that satisfy the configuration
// constraints. It is specifically designed to benchmark `memcmp` functions
// where we may want the Nth byte to differ.
class MismatchOffsetDistribution {
  std::uniform_int_distribution<size_t> MismatchIndexSelector;
  llvm::SmallVector<uint32_t, 16> MismatchIndices;
  const uint32_t MismatchAt;

public:
  explicit MismatchOffsetDistribution(size_t BufferSize, size_t MaxSizeValue,
                                      size_t MismatchAt);

  explicit operator bool() const { return !MismatchIndices.empty(); }

  const llvm::SmallVectorImpl<uint32_t> &getMismatchIndices() const {
    return MismatchIndices;
  }

  template <class Generator> uint32_t operator()(Generator &G, uint32_t Size) {
    const uint32_t MismatchIndex = MismatchIndices[MismatchIndexSelector(G)];
    // We need to position the offset so that a mismatch occurs at MismatchAt.
    if (Size >= MismatchAt)
      return MismatchIndex - MismatchAt;
    // Size is too small to trigger the mismatch.
    return MismatchIndex - Size - 1;
  }
};

/// This structure holds a vector of ParameterType.
/// It makes sure that BufferCount x BufferSize Bytes and the vector of
/// ParameterType can all fit in the L1 cache.
struct ParameterBatch {
  struct ParameterType {
    unsigned OffsetBytes : 16; // max : 16 KiB - 1
    unsigned SizeBytes : 16;   // max : 16 KiB - 1
  };

  ParameterBatch(size_t BufferCount);

  /// Verifies that memory accessed through this parameter is valid.
  void checkValid(const ParameterType &) const;

  /// Computes the number of bytes processed during within this batch.
  size_t getBatchBytes() const;

  const size_t BufferSize;
  const size_t BatchSize;
  std::vector<ParameterType> Parameters;
};

/// Provides source and destination buffers for the Copy operation as well as
/// the associated size distributions.
struct CopySetup : public ParameterBatch {
  CopySetup();

  inline static const ArrayRef<MemorySizeDistribution> getDistributions() {
    return getMemcpySizeDistributions();
  }

  inline void *Call(ParameterType Parameter, MemcpyFunction Memcpy) {
    return Memcpy(DstBuffer + Parameter.OffsetBytes,
                  SrcBuffer + Parameter.OffsetBytes, Parameter.SizeBytes);
  }

private:
  AlignedBuffer SrcBuffer;
  AlignedBuffer DstBuffer;
};

/// Provides source and destination buffers for the Move operation as well as
/// the associated size distributions.
struct MoveSetup : public ParameterBatch {
  MoveSetup();

  inline static const ArrayRef<MemorySizeDistribution> getDistributions() {
    return getMemmoveSizeDistributions();
  }

  inline void *Call(ParameterType Parameter, MemmoveFunction Memmove) {
    return Memmove(Buffer + ParameterBatch::BufferSize / 3,
                   Buffer + Parameter.OffsetBytes, Parameter.SizeBytes);
  }

private:
  AlignedBuffer Buffer;
};

/// Provides destination buffer for the Set operation as well as the associated
/// size distributions.
struct SetSetup : public ParameterBatch {
  SetSetup();

  inline static const ArrayRef<MemorySizeDistribution> getDistributions() {
    return getMemsetSizeDistributions();
  }

  inline void *Call(ParameterType Parameter, MemsetFunction Memset) {
    return Memset(DstBuffer + Parameter.OffsetBytes,
                  Parameter.OffsetBytes % 0xFF, Parameter.SizeBytes);
  }

  inline void *Call(ParameterType Parameter, BzeroFunction Bzero) {
    Bzero(DstBuffer + Parameter.OffsetBytes, Parameter.SizeBytes);
    return DstBuffer.begin();
  }

private:
  AlignedBuffer DstBuffer;
};

/// Provides left and right buffers for the Comparison operation as well as the
/// associated size distributions.
struct ComparisonSetup : public ParameterBatch {
  ComparisonSetup();

  inline static const ArrayRef<MemorySizeDistribution> getDistributions() {
    return getMemcmpSizeDistributions();
  }

  inline int Call(ParameterType Parameter, MemcmpOrBcmpFunction MemcmpOrBcmp) {
    return MemcmpOrBcmp(LhsBuffer + Parameter.OffsetBytes,
                        RhsBuffer + Parameter.OffsetBytes, Parameter.SizeBytes);
  }

private:
  AlignedBuffer LhsBuffer;
  AlignedBuffer RhsBuffer;
};

} // namespace libc_benchmarks
} // namespace llvm

#endif // LLVM_LIBC_UTILS_BENCHMARK_MEMORY_BENCHMARK_H
