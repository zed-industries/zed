//===-- Exhaustive test template for math functions -------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "src/__support/CPP/type_traits.h"
#include "src/__support/FPUtil/FPBits.h"
#include "src/__support/macros/properties/types.h"
#include "test/UnitTest/FPMatcher.h"
#include "test/UnitTest/Test.h"
#include "utils/MPFRWrapper/MPFRUtils.h"

#include <atomic>
#include <functional>
#include <iostream>
#include <mutex>
#include <sstream>
#include <thread>
#include <vector>

// To test exhaustively for inputs in the range [start, stop) in parallel:
// 1. Define a Checker class with:
//    - FloatType: define floating point type to be used.
//    - FPBits: fputil::FPBits<FloatType>.
//    - StorageType: define bit type for the corresponding floating point type.
//    - uint64_t check(start, stop, rounding_mode): a method to test in given
//          range for a given rounding mode, which returns the number of
//          failures.
// 2. Use LlvmLibcExhaustiveMathTest<Checker> class
// 3. Call: test_full_range(start, stop, nthreads, rounding)
//       or test_full_range_all_roundings(start, stop).
// * For single input single output math function, use the convenient template:
//   LlvmLibcUnaryOpExhaustiveMathTest<FloatType, Op, Func>.
namespace mpfr = LIBC_NAMESPACE::testing::mpfr;

template <typename OutType, typename InType = OutType>
using UnaryOp = OutType(InType);

template <typename OutType, typename InType, mpfr::Operation Op,
          UnaryOp<OutType, InType> Func>
struct UnaryOpChecker : public virtual LIBC_NAMESPACE::testing::Test {
  using FloatType = InType;
  using FPBits = LIBC_NAMESPACE::fputil::FPBits<FloatType>;
  using StorageType = typename FPBits::StorageType;

  // Check in a range, return the number of failures.
  uint64_t check(StorageType start, StorageType stop,
                 mpfr::RoundingMode rounding) {
    mpfr::ForceRoundingMode r(rounding);
    if (!r.success)
      return (stop > start);
    StorageType bits = start;
    uint64_t failed = 0;
    do {
      FPBits xbits(bits);
      FloatType x = xbits.get_val();
      bool correct =
          TEST_MPFR_MATCH_ROUNDING_SILENTLY(Op, x, Func(x), 0.5, rounding);
      failed += (!correct);
      // Uncomment to print out failed values.
      if (!correct) {
        EXPECT_MPFR_MATCH_ROUNDING(Op, x, Func(x), 0.5, rounding);
      }
    } while (bits++ < stop);
    return failed;
  }
};

template <typename OutType, typename InType = OutType>
using BinaryOp = OutType(InType, InType);

template <typename OutType, typename InType, mpfr::Operation Op,
          BinaryOp<OutType, InType> Func>
struct BinaryOpChecker : public virtual LIBC_NAMESPACE::testing::Test {
  using FloatType = InType;
  using FPBits = LIBC_NAMESPACE::fputil::FPBits<FloatType>;
  using StorageType = typename FPBits::StorageType;

  // Check in a range, return the number of failures.
  uint64_t check(StorageType x_start, StorageType x_stop, StorageType y_start,
                 StorageType y_stop, mpfr::RoundingMode rounding) {
    mpfr::ForceRoundingMode r(rounding);
    if (!r.success)
      return x_stop > x_start || y_stop > y_start;
    StorageType xbits = x_start;
    uint64_t failed = 0;
    do {
      FloatType x = FPBits(xbits).get_val();
      StorageType ybits = y_start;
      do {
        FloatType y = FPBits(ybits).get_val();
        mpfr::BinaryInput<FloatType> input{x, y};
        bool correct = TEST_MPFR_MATCH_ROUNDING_SILENTLY(Op, input, Func(x, y),
                                                         0.5, rounding);
        failed += (!correct);
        // Uncomment to print out failed values.
        if (!correct) {
          EXPECT_MPFR_MATCH_ROUNDING(Op, input, Func(x, y), 0.5, rounding);
        }
      } while (ybits++ < y_stop);
    } while (xbits++ < x_stop);
    return failed;
  }
};

// Checker class needs inherit from LIBC_NAMESPACE::testing::Test and provide
//   StorageType and check method.
template <typename Checker, size_t Increment = 1 << 20>
struct LlvmLibcExhaustiveMathTest
    : public virtual LIBC_NAMESPACE::testing::Test,
      public Checker {
  using FloatType = typename Checker::FloatType;
  using FPBits = typename Checker::FPBits;
  using StorageType = typename Checker::StorageType;

  void explain_failed_range(std::stringstream &msg, StorageType x_begin,
                            StorageType x_end) {
#ifdef LIBC_TYPES_HAS_FLOAT16
    using T = LIBC_NAMESPACE::cpp::conditional_t<
        LIBC_NAMESPACE::cpp::is_same_v<FloatType, float16>, float, FloatType>;
#else
    using T = FloatType;
#endif

    msg << x_begin << " to " << x_end << " [0x" << std::hex << x_begin << ", 0x"
        << x_end << "), [" << std::hexfloat
        << static_cast<T>(FPBits(x_begin).get_val()) << ", "
        << static_cast<T>(FPBits(x_end).get_val()) << ")";
  }

  void explain_failed_range(std::stringstream &msg, StorageType x_begin,
                            StorageType x_end, StorageType y_begin,
                            StorageType y_end) {
    msg << "x ";
    explain_failed_range(msg, x_begin, x_end);
    msg << ", y ";
    explain_failed_range(msg, y_begin, y_end);
  }

  // Break [start, stop) into `nthreads` subintervals and apply *check to each
  // subinterval in parallel.
  template <typename... T>
  void test_full_range(mpfr::RoundingMode rounding, StorageType start,
                       StorageType stop, T... extra_range_bounds) {
    int n_threads = std::thread::hardware_concurrency();
    std::vector<std::thread> thread_list;
    std::mutex mx_cur_val;
    int current_percent = -1;
    StorageType current_value = start;
    std::atomic<uint64_t> failed(0);

    for (int i = 0; i < n_threads; ++i) {
      thread_list.emplace_back([&, this]() {
        while (true) {
          StorageType range_begin, range_end;
          int new_percent = -1;
          {
            std::lock_guard<std::mutex> lock(mx_cur_val);
            if (current_value == stop)
              return;

            range_begin = current_value;
            if (stop >= Increment && stop - Increment >= current_value) {
              range_end = current_value + Increment;
            } else {
              range_end = stop;
            }
            current_value = range_end;
            int pc =
                static_cast<int>(100.0 * (range_end - start) / (stop - start));
            if (current_percent != pc) {
              new_percent = pc;
              current_percent = pc;
            }
          }
          if (new_percent >= 0) {
            std::stringstream msg;
            msg << new_percent << "% is in process     \r";
            std::cout << msg.str() << std::flush;
          }

          uint64_t failed_in_range = Checker::check(
              range_begin, range_end, extra_range_bounds..., rounding);
          if (failed_in_range > 0) {
            std::stringstream msg;
            msg << "Test failed for " << std::dec << failed_in_range
                << " inputs in range: ";
            explain_failed_range(msg, range_begin, range_end,
                                 extra_range_bounds...);
            msg << "\n";
            std::cerr << msg.str() << std::flush;

            failed.fetch_add(failed_in_range);
          }
        }
      });
    }

    for (auto &thread : thread_list) {
      if (thread.joinable()) {
        thread.join();
      }
    }

    std::cout << std::endl;
    std::cout << "Test " << ((failed > 0) ? "FAILED" : "PASSED") << std::endl;
    ASSERT_EQ(failed.load(), uint64_t(0));
  }

  void test_full_range_all_roundings(StorageType start, StorageType stop) {
    std::cout << "-- Testing for FE_TONEAREST in range [0x" << std::hex << start
              << ", 0x" << stop << ") --" << std::dec << std::endl;
    test_full_range(mpfr::RoundingMode::Nearest, start, stop);

    std::cout << "-- Testing for FE_UPWARD in range [0x" << std::hex << start
              << ", 0x" << stop << ") --" << std::dec << std::endl;
    test_full_range(mpfr::RoundingMode::Upward, start, stop);

    std::cout << "-- Testing for FE_DOWNWARD in range [0x" << std::hex << start
              << ", 0x" << stop << ") --" << std::dec << std::endl;
    test_full_range(mpfr::RoundingMode::Downward, start, stop);

    std::cout << "-- Testing for FE_TOWARDZERO in range [0x" << std::hex
              << start << ", 0x" << stop << ") --" << std::dec << std::endl;
    test_full_range(mpfr::RoundingMode::TowardZero, start, stop);
  }

  void test_full_range_all_roundings(StorageType x_start, StorageType x_stop,
                                     StorageType y_start, StorageType y_stop) {
    std::cout << "-- Testing for FE_TONEAREST in x range [0x" << std::hex
              << x_start << ", 0x" << x_stop << "), y range [0x" << y_start
              << ", 0x" << y_stop << ") --" << std::dec << std::endl;
    test_full_range(mpfr::RoundingMode::Nearest, x_start, x_stop, y_start,
                    y_stop);

    std::cout << "-- Testing for FE_UPWARD in x range [0x" << std::hex
              << x_start << ", 0x" << x_stop << "), y range [0x" << y_start
              << ", 0x" << y_stop << ") --" << std::dec << std::endl;
    test_full_range(mpfr::RoundingMode::Upward, x_start, x_stop, y_start,
                    y_stop);

    std::cout << "-- Testing for FE_DOWNWARD in x range [0x" << std::hex
              << x_start << ", 0x" << x_stop << "), y range [0x" << y_start
              << ", 0x" << y_stop << ") --" << std::dec << std::endl;
    test_full_range(mpfr::RoundingMode::Downward, x_start, x_stop, y_start,
                    y_stop);

    std::cout << "-- Testing for FE_TOWARDZERO in x range [0x" << std::hex
              << x_start << ", 0x" << x_stop << "), y range [0x" << y_start
              << ", 0x" << y_stop << ") --" << std::dec << std::endl;
    test_full_range(mpfr::RoundingMode::TowardZero, x_start, x_stop, y_start,
                    y_stop);
  }
};

template <typename FloatType, mpfr::Operation Op, UnaryOp<FloatType> Func>
using LlvmLibcUnaryOpExhaustiveMathTest =
    LlvmLibcExhaustiveMathTest<UnaryOpChecker<FloatType, FloatType, Op, Func>>;

template <typename OutType, typename InType, mpfr::Operation Op,
          UnaryOp<OutType, InType> Func>
using LlvmLibcUnaryNarrowingOpExhaustiveMathTest =
    LlvmLibcExhaustiveMathTest<UnaryOpChecker<OutType, InType, Op, Func>>;

template <typename FloatType, mpfr::Operation Op, BinaryOp<FloatType> Func>
using LlvmLibcBinaryOpExhaustiveMathTest =
    LlvmLibcExhaustiveMathTest<BinaryOpChecker<FloatType, FloatType, Op, Func>,
                               1 << 2>;
