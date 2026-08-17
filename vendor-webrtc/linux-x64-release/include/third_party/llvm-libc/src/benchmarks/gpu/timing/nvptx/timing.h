//===------------- NVPTX implementation of timing utils ---------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_UTILS_GPU_TIMING_NVPTX
#define LLVM_LIBC_UTILS_GPU_TIMING_NVPTX

#include "src/__support/CPP/array.h"
#include "src/__support/CPP/type_traits.h"
#include "src/__support/GPU/utils.h"
#include "src/__support/common.h"
#include "src/__support/macros/attributes.h"
#include "src/__support/macros/config.h"

#include <stdint.h>

namespace LIBC_NAMESPACE_DECL {

// Returns the overhead associated with calling the profiling region. This
// allows us to substract the constant-time overhead from the latency to
// obtain a true result. This can vary with system load.
[[gnu::noinline]] static uint64_t overhead() {
  volatile uint32_t x = 1;
  uint32_t y = x;
  uint64_t start = gpu::processor_clock();
  asm("" ::"llr"(start));
  uint32_t result = y;
  asm("or.b32 %[v_reg], %[v_reg], 0;" ::[v_reg] "r"(result));
  uint64_t stop = gpu::processor_clock();
  volatile auto storage = result;
  return stop - start;
}

// Stimulate a simple function and obtain its latency in clock cycles on the
// system. This function cannot be inlined or else it will disturb the very
// delicate balance of hard-coded dependencies.
template <typename F, typename T>
[[gnu::noinline]] static LIBC_INLINE uint64_t latency(F f, T t) {
  // We need to store the input somewhere to guarantee that the compiler will
  // not constant propagate it and remove the profiling region.
  volatile T storage = t;
  T arg = storage;

  // Get the current timestamp from the clock.
  gpu::memory_fence();
  uint64_t start = gpu::processor_clock();

  // This forces the compiler to load the input argument and run the clock cycle
  // counter before the profiling region.
  asm("" ::"llr"(start));

  // Run the function under test and return its value.
  auto result = f(arg);

  // This inline assembly performs a no-op which forces the result to both be
  // used and prevents us from exiting this region before it's complete.
  asm("or.b32 %[v_reg], %[v_reg], 0;" ::[v_reg] "r"(result));

  // Obtain the current timestamp after running the calculation and force
  // ordering.
  uint64_t stop = gpu::processor_clock();
  gpu::memory_fence();
  asm("" ::"r"(stop));
  volatile T output = result;

  // Return the time elapsed.
  return stop - start;
}

template <typename F, typename T1, typename T2>
static LIBC_INLINE uint64_t latency(F f, T1 t1, T2 t2) {
  volatile T1 storage = t1;
  volatile T2 storage2 = t2;
  T1 arg = storage;
  T2 arg2 = storage2;

  gpu::memory_fence();
  uint64_t start = gpu::processor_clock();

  asm("" ::"llr"(start));

  auto result = f(arg, arg2);

  asm("or.b32 %[v_reg], %[v_reg], 0;" ::[v_reg] "r"(result));

  uint64_t stop = gpu::processor_clock();
  gpu::memory_fence();
  asm("" ::"r"(stop));
  volatile auto output = result;

  return stop - start;
}

// Provides throughput benchmarking.
template <typename F, typename T, size_t N>
[[gnu::noinline]] static LIBC_INLINE uint64_t
throughput(F f, const cpp::array<T, N> &inputs) {
  asm("" ::"r"(&inputs));

  gpu::memory_fence();
  uint64_t start = gpu::processor_clock();

  asm("" ::"llr"(start));

  uint64_t result;
  for (auto input : inputs) {
    asm("" ::"r"(input));
    result = f(input);
    asm("" ::"r"(result));
  }

  uint64_t stop = gpu::processor_clock();
  gpu::memory_fence();
  asm("" ::"r"(stop));
  volatile auto output = result;

  // Return the time elapsed.
  return stop - start;
}

// Provides throughput benchmarking for 2 arguments (e.g. atan2())
template <typename F, typename T, size_t N>
[[gnu::noinline]] static LIBC_INLINE uint64_t throughput(
    F f, const cpp::array<T, N> &inputs1, const cpp::array<T, N> &inputs2) {
  asm("" ::"r"(&inputs1), "r"(&inputs2));

  gpu::memory_fence();
  uint64_t start = gpu::processor_clock();

  asm("" ::"llr"(start));

  uint64_t result;
  for (size_t i = 0; i < inputs1.size(); i++) {
    result = f(inputs1[i], inputs2[i]);
    asm("" ::"r"(result));
  }

  uint64_t stop = gpu::processor_clock();
  gpu::memory_fence();
  asm("" ::"r"(stop));
  volatile auto output = result;

  // Return the time elapsed.
  return stop - start;
}
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_UTILS_GPU_TIMING_NVPTX
