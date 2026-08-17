//===-- Common utility class for differential analysis --------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "src/__support/CPP/algorithm.h"
#include "src/__support/FPUtil/FPBits.h"
#include "src/__support/macros/config.h"
#include "test/src/math/performance_testing/Timer.h"

#include <cstddef>
#include <fstream>

namespace LIBC_NAMESPACE_DECL {
namespace testing {
template <typename OutputType, typename InputType> class PerfTest {
  using FPBits = fputil::FPBits<OutputType>;
  using StorageType = typename FPBits::StorageType;
  static constexpr StorageType U_INT_MAX =
      cpp::numeric_limits<StorageType>::max();

public:
  using BinaryFuncPtr = OutputType (*)(InputType, InputType);
  using UnaryFuncPtr = OutputType (*)(InputType);

  template <bool binary, typename Func>
  static void run_perf_in_range(Func FuncA, Func FuncB, StorageType startingBit,
                                StorageType endingBit, size_t N, size_t rounds,
                                const char *name_a, const char *name_b,
                                std::ofstream &log) {
    if (sizeof(StorageType) <= sizeof(size_t))
      N = cpp::min(N, static_cast<size_t>(endingBit - startingBit));

    auto runner = [=](Func func) {
      [[maybe_unused]] volatile OutputType result;
      if (endingBit < startingBit) {
        return;
      }

      StorageType step = (endingBit - startingBit) / N;
      if (step == 0)
        step = 1;
      for (size_t i = 0; i < rounds; i++) {
        for (StorageType bits_x = startingBit, bits_y = endingBit;;
             bits_x += step, bits_y -= step) {
          InputType x = FPBits(bits_x).get_val();
          if constexpr (binary) {
            InputType y = FPBits(bits_y).get_val();
            result = func(x, y);
          } else {
            result = func(x);
          }
          if (endingBit - bits_x < step) {
            break;
          }
        }
      }
    };

    Timer timer;
    timer.start();
    runner(FuncA);
    timer.stop();

    double a_average = static_cast<double>(timer.nanoseconds()) / N / rounds;
    log << "-- Function A: " << name_a << " --\n";
    log << "     Total time      : " << timer.nanoseconds() << " ns \n";
    log << "     Average runtime : " << a_average << " ns/op \n";
    log << "     Ops per second  : "
        << static_cast<uint64_t>(1'000'000'000.0 / a_average) << " op/s \n";

    timer.start();
    runner(FuncB);
    timer.stop();

    double b_average = static_cast<double>(timer.nanoseconds()) / N / rounds;
    log << "-- Function B: " << name_b << " --\n";
    log << "     Total time      : " << timer.nanoseconds() << " ns \n";
    log << "     Average runtime : " << b_average << " ns/op \n";
    log << "     Ops per second  : "
        << static_cast<uint64_t>(1'000'000'000.0 / b_average) << " op/s \n";

    log << "-- Average ops per second ratio --\n";
    log << "     A / B  : " << b_average / a_average << " \n";
  }

  template <bool binary, typename Func>
  static void run_perf(Func FuncA, Func FuncB, int rounds, const char *name_a,
                       const char *name_b, const char *logFile) {
    std::ofstream log(logFile);
    log << " Performance tests with inputs in denormal range:\n";
    run_perf_in_range<binary>(
        FuncA, FuncB, /* startingBit= */ StorageType(0),
        /* endingBit= */ FPBits::max_subnormal().uintval(), 1'000'001, rounds,
        name_a, name_b, log);
    log << "\n Performance tests with inputs in normal range:\n";
    run_perf_in_range<binary>(FuncA, FuncB,
                              /* startingBit= */ FPBits::min_normal().uintval(),
                              /* endingBit= */ FPBits::max_normal().uintval(),
                              1'000'001, rounds, name_a, name_b, log);
    log << "\n Performance tests with inputs in normal range with exponents "
           "close to each other:\n";
    run_perf_in_range<binary>(
        FuncA, FuncB,
        /* startingBit= */ FPBits(OutputType(0x1.0p-10)).uintval(),
        /* endingBit= */ FPBits(OutputType(0x1.0p+10)).uintval(), 1'000'001,
        rounds, name_a, name_b, log);
  }
};

} // namespace testing
} // namespace LIBC_NAMESPACE_DECL

#define BINARY_INPUT_SINGLE_OUTPUT_PERF(OutputType, InputType, FuncA, FuncB,   \
                                        filename)                              \
  {                                                                            \
    using TargetFuncPtr =                                                      \
        typename LIBC_NAMESPACE::testing::PerfTest<OutputType,                 \
                                                   InputType>::BinaryFuncPtr;  \
    LIBC_NAMESPACE::testing::PerfTest<OutputType, InputType>::run_perf<true>(  \
        static_cast<TargetFuncPtr>(&FuncA),                                    \
        static_cast<TargetFuncPtr>(&FuncB), 1, #FuncA, #FuncB, filename);      \
    return 0;                                                                  \
  }

#define BINARY_INPUT_SINGLE_OUTPUT_PERF_EX(OutputType, InputType, FuncA,       \
                                           FuncB, rounds, filename)            \
  {                                                                            \
    using TargetFuncPtr =                                                      \
        typename LIBC_NAMESPACE::testing::PerfTest<OutputType,                 \
                                                   InputType>::BinaryFuncPtr;  \
    LIBC_NAMESPACE::testing::PerfTest<OutputType, InputType>::run_perf<true>(  \
        static_cast<TargetFuncPtr>(&FuncA),                                    \
        static_cast<TargetFuncPtr>(&FuncB), rounds, #FuncA, #FuncB, filename); \
    return 0;                                                                  \
  }

#define SINGLE_INPUT_SINGLE_OUTPUT_PERF(T, FuncA, FuncB, filename)             \
  {                                                                            \
    using TargetFuncPtr =                                                      \
        typename LIBC_NAMESPACE::testing::PerfTest<T, T>::UnaryFuncPtr;        \
    LIBC_NAMESPACE::testing::PerfTest<T, T>::run_perf<false>(                  \
        static_cast<TargetFuncPtr>(&FuncA),                                    \
        static_cast<TargetFuncPtr>(&FuncB), 1, #FuncA, #FuncB, filename);      \
    return 0;                                                                  \
  }

#define SINGLE_INPUT_SINGLE_OUTPUT_PERF_EX(T, FuncA, FuncB, rounds, filename)  \
  {                                                                            \
    using TargetFuncPtr =                                                      \
        typename LIBC_NAMESPACE::testing::PerfTest<T, T>::UnaryFuncPtr;        \
    LIBC_NAMESPACE::testing::PerfTest<T, T>::run_perf<false>(                  \
        static_cast<TargetFuncPtr>(&FuncA),                                    \
        static_cast<TargetFuncPtr>(&FuncB), rounds, #FuncA, #FuncB, filename); \
    return 0;                                                                  \
  }
