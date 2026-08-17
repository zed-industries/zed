//===-- Benchmark function --------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

// This file mainly defines a `Benchmark` function.
//
// The benchmarking process is as follows:
// - We start by measuring the time it takes to run the function
// `InitialIterations` times. This is called a Sample. From this we can derive
// the time it took to run a single iteration.
//
// - We repeat the previous step with a greater number of iterations to lower
// the impact of the measurement. We can derive a more precise estimation of the
// runtime for a single iteration.
//
// - Each sample gives a more accurate estimation of the runtime for a single
// iteration but also takes more time to run. We stop the process when:
//   * The measure stabilize under a certain precision (Epsilon),
//   * The overall benchmarking time is greater than MaxDuration,
//   * The overall sample count is greater than MaxSamples,
//   * The last sample used more than MaxIterations iterations.
//
// - We also makes sure that the benchmark doesn't run for a too short period of
// time by defining MinDuration and MinSamples.

#ifndef LLVM_LIBC_UTILS_BENCHMARK_BENCHMARK_H
#define LLVM_LIBC_UTILS_BENCHMARK_BENCHMARK_H

#include "benchmark/benchmark.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/SmallVector.h"
#include <array>
#include <chrono>
#include <cmath>
#include <cstdint>
#include <optional>

namespace llvm {
namespace libc_benchmarks {

using Duration = std::chrono::duration<double>;

enum class BenchmarkLog {
  None, // Don't keep the internal state of the benchmark.
  Last, // Keep only the last batch.
  Full  // Keep all iterations states, useful for testing or debugging.
};

// An object to configure the benchmark stopping conditions.
// See documentation at the beginning of the file for the overall algorithm and
// meaning of each field.
struct BenchmarkOptions {
  // The minimum time for which the benchmark is running.
  Duration MinDuration = std::chrono::seconds(0);
  // The maximum time for which the benchmark is running.
  Duration MaxDuration = std::chrono::seconds(10);
  // The number of iterations in the first sample.
  uint32_t InitialIterations = 1;
  // The maximum number of iterations for any given sample.
  uint32_t MaxIterations = 10000000;
  // The minimum number of samples.
  uint32_t MinSamples = 4;
  // The maximum number of samples.
  uint32_t MaxSamples = 1000;
  // The benchmark will stop if the relative difference between the current and
  // the last estimation is less than epsilon. This is 1% by default.
  double Epsilon = 0.01;
  // The number of iterations grows exponentially between each sample.
  // Must be greater or equal to 1.
  double ScalingFactor = 1.4;
  BenchmarkLog Log = BenchmarkLog::None;
};

// The state of a benchmark.
enum class BenchmarkStatus {
  Running,
  MaxDurationReached,
  MaxIterationsReached,
  MaxSamplesReached,
  PrecisionReached,
};

// The internal state of the benchmark, useful to debug, test or report
// statistics.
struct BenchmarkState {
  size_t LastSampleIterations;
  Duration LastBatchElapsed;
  BenchmarkStatus CurrentStatus;
  Duration CurrentBestGuess; // The time estimation for a single run of `foo`.
  double ChangeRatio; // The change in time estimation between previous and
                      // current samples.
};

// A lightweight result for a benchmark.
struct BenchmarkResult {
  BenchmarkStatus TerminationStatus = BenchmarkStatus::Running;
  Duration BestGuess = {};
  std::optional<llvm::SmallVector<BenchmarkState, 16>> MaybeBenchmarkLog;
};

// Stores information about a cache in the host memory system.
struct CacheInfo {
  std::string Type; //  e.g. "Instruction", "Data", "Unified".
  int Level;        // 0 is closest to processing unit.
  int Size;         // In bytes.
  int NumSharing;   // The number of processing units (Hyper-Threading Thread)
                    // with which this cache is shared.
};

// Stores information about the host.
struct HostState {
  std::string CpuName; // returns a string compatible with the -march option.
  double CpuFrequency; // in Hertz.
  std::vector<CacheInfo> Caches;

  static HostState get();
};

namespace internal {

struct Measurement {
  size_t Iterations = 0;
  Duration Elapsed = {};
};

// Updates the estimation of the elapsed time for a single iteration.
class RefinableRuntimeEstimation {
  Duration TotalTime = {};
  size_t TotalIterations = 0;

public:
  Duration update(const Measurement &M) {
    assert(M.Iterations > 0);
    // Duration is encoded as a double (see definition).
    // `TotalTime` and `M.Elapsed` are of the same magnitude so we don't expect
    // loss of precision due to radically different scales.
    TotalTime += M.Elapsed;
    TotalIterations += M.Iterations;
    return TotalTime / TotalIterations;
  }
};

// This class tracks the progression of the runtime estimation.
class RuntimeEstimationProgression {
  RefinableRuntimeEstimation RRE;

public:
  Duration CurrentEstimation = {};

  // Returns the change ratio between our best guess so far and the one from the
  // new measurement.
  double computeImprovement(const Measurement &M) {
    const Duration NewEstimation = RRE.update(M);
    const double Ratio = fabs(((CurrentEstimation / NewEstimation) - 1.0));
    CurrentEstimation = NewEstimation;
    return Ratio;
  }
};

} // namespace internal

// Measures the runtime of `foo` until conditions defined by `Options` are met.
//
// To avoid measurement's imprecisions we measure batches of `foo`.
// The batch size is growing by `ScalingFactor` to minimize the effect of
// measuring.
//
// Note: The benchmark is not responsible for serializing the executions of
// `foo`. It is not suitable for measuring, very small & side effect free
// functions, as the processor is free to execute several executions in
// parallel.
//
// - Options: A set of parameters controlling the stopping conditions for the
//     benchmark.
// - foo: The function under test. It takes one value and returns one value.
//     The input value is used to randomize the execution of `foo` as part of a
//     batch to mitigate the effect of the branch predictor. Signature:
//     `ProductType foo(ParameterProvider::value_type value);`
//     The output value is a product of the execution of `foo` and prevents the
//     compiler from optimizing out foo's body.
// - ParameterProvider: An object responsible for providing a range of
//     `Iterations` values to use as input for `foo`. The `value_type` of the
//     returned container has to be compatible with `foo` argument.
//     Must implement one of:
//     `Container<ParameterType> generateBatch(size_t Iterations);`
//     `const Container<ParameterType>& generateBatch(size_t Iterations);`
// - Clock: An object providing the current time. Must implement:
//     `std::chrono::time_point now();`
template <typename Function, typename ParameterProvider,
          typename BenchmarkClock = const std::chrono::high_resolution_clock>
BenchmarkResult benchmark(const BenchmarkOptions &Options,
                          ParameterProvider &PP, Function foo,
                          BenchmarkClock &Clock = BenchmarkClock()) {
  BenchmarkResult Result;
  internal::RuntimeEstimationProgression REP;
  Duration TotalBenchmarkDuration = {};
  size_t Iterations = std::max(Options.InitialIterations, uint32_t(1));
  size_t Samples = 0;
  if (Options.ScalingFactor < 1.0)
    report_fatal_error("ScalingFactor should be >= 1");
  if (Options.Log != BenchmarkLog::None)
    Result.MaybeBenchmarkLog.emplace();
  for (;;) {
    // Request a new Batch of size `Iterations`.
    const auto &Batch = PP.generateBatch(Iterations);

    // Measuring this Batch.
    const auto StartTime = Clock.now();
    for (const auto Parameter : Batch) {
      auto Production = foo(Parameter);
      benchmark::DoNotOptimize(Production);
    }
    const auto EndTime = Clock.now();
    const Duration Elapsed = EndTime - StartTime;

    // Updating statistics.
    ++Samples;
    TotalBenchmarkDuration += Elapsed;
    const double ChangeRatio = REP.computeImprovement({Iterations, Elapsed});
    Result.BestGuess = REP.CurrentEstimation;

    // Stopping condition.
    if (TotalBenchmarkDuration >= Options.MinDuration &&
        Samples >= Options.MinSamples && ChangeRatio < Options.Epsilon)
      Result.TerminationStatus = BenchmarkStatus::PrecisionReached;
    else if (Samples >= Options.MaxSamples)
      Result.TerminationStatus = BenchmarkStatus::MaxSamplesReached;
    else if (TotalBenchmarkDuration >= Options.MaxDuration)
      Result.TerminationStatus = BenchmarkStatus::MaxDurationReached;
    else if (Iterations >= Options.MaxIterations)
      Result.TerminationStatus = BenchmarkStatus::MaxIterationsReached;

    if (Result.MaybeBenchmarkLog) {
      auto &BenchmarkLog = *Result.MaybeBenchmarkLog;
      if (Options.Log == BenchmarkLog::Last && !BenchmarkLog.empty())
        BenchmarkLog.pop_back();
      BenchmarkState BS;
      BS.LastSampleIterations = Iterations;
      BS.LastBatchElapsed = Elapsed;
      BS.CurrentStatus = Result.TerminationStatus;
      BS.CurrentBestGuess = Result.BestGuess;
      BS.ChangeRatio = ChangeRatio;
      BenchmarkLog.push_back(BS);
    }

    if (Result.TerminationStatus != BenchmarkStatus::Running)
      return Result;

    if (Options.ScalingFactor > 1 &&
        Iterations * Options.ScalingFactor == Iterations)
      report_fatal_error(
          "`Iterations *= ScalingFactor` is idempotent, increase ScalingFactor "
          "or InitialIterations.");

    Iterations *= Options.ScalingFactor;
  }
}

// Interprets `Array` as a circular buffer of `Size` elements.
template <typename T> class CircularArrayRef {
  llvm::ArrayRef<T> Array;
  size_t Size;

public:
  using value_type = T;
  using reference = T &;
  using const_reference = const T &;
  using difference_type = ssize_t;
  using size_type = size_t;

  class const_iterator {
    using iterator_category = std::input_iterator_tag;
    llvm::ArrayRef<T> Array;
    size_t Index;
    size_t Offset;

  public:
    explicit const_iterator(llvm::ArrayRef<T> Array, size_t Index = 0)
        : Array(Array), Index(Index), Offset(Index % Array.size()) {}
    const_iterator &operator++() {
      ++Index;
      ++Offset;
      if (Offset == Array.size())
        Offset = 0;
      return *this;
    }
    bool operator==(const_iterator Other) const { return Index == Other.Index; }
    bool operator!=(const_iterator Other) const { return !(*this == Other); }
    const T &operator*() const { return Array[Offset]; }
  };

  CircularArrayRef(llvm::ArrayRef<T> Array, size_t Size)
      : Array(Array), Size(Size) {
    assert(Array.size() > 0);
  }

  const_iterator begin() const { return const_iterator(Array); }
  const_iterator end() const { return const_iterator(Array, Size); }
};

// A convenient helper to produce a CircularArrayRef from an ArrayRef.
template <typename T>
CircularArrayRef<T> cycle(llvm::ArrayRef<T> Array, size_t Size) {
  return {Array, Size};
}

// Creates an std::array which storage size is constrained under `Bytes`.
template <typename T, size_t Bytes>
using ByteConstrainedArray = std::array<T, Bytes / sizeof(T)>;

// A convenient helper to produce a CircularArrayRef from a
// ByteConstrainedArray.
template <typename T, size_t N>
CircularArrayRef<T> cycle(const std::array<T, N> &Container, size_t Size) {
  return {llvm::ArrayRef<T>(Container.cbegin(), Container.cend()), Size};
}

// Makes sure the binary was compiled in release mode and that frequency
// governor is set on performance.
void checkRequirements();

} // namespace libc_benchmarks
} // namespace llvm

#endif // LLVM_LIBC_UTILS_BENCHMARK_BENCHMARK_H
