/*
 *  Copyright 2011 The LibYuv Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS. All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef UNIT_TEST_UNIT_TEST_H_  // NOLINT
#define UNIT_TEST_UNIT_TEST_H_

#include <stddef.h>  // For NULL
#ifdef _WIN32
#include <windows.h>
#else
#include <sys/time.h>
#endif

#include <gtest/gtest.h>

#include "libyuv/basic_types.h"

#ifndef SIMD_ALIGNED
#if defined(_MSC_VER) && !defined(__CLR_VER)
#define SIMD_ALIGNED(var) __declspec(align(16)) var
#elif defined(__GNUC__) && !defined(__pnacl__)
#define SIMD_ALIGNED(var) var __attribute__((aligned(16)))
#else
#define SIMD_ALIGNED(var) var
#endif
#endif

static __inline int Abs(int v) {
  return v >= 0 ? v : -v;
}

static __inline float FAbs(float v) {
  return v >= 0 ? v : -v;
}
#define OFFBY 0

// Scaling uses 16.16 fixed point to step thru the source image, so a
// maximum size of 32767.999 can be expressed.  32768 is valid because
// the step is 1 beyond the image but not used.
// Destination size is mainly constrained by valid scale step not the
// absolute size, so it may be possible to relax the destination size
// constraint.
// Source size is unconstrained for most specialized scalers.  e.g.
// An image of 65536 scaled to half size would be valid.  The test
// could be relaxed for special scale factors.
// If this test is removed, the scaling function should gracefully
// fail with a return code.  The test could be changed to know that
// libyuv failed in a controlled way.

static const int kMaxWidth = 32768;
static const int kMaxHeight = 32768;

static inline bool SizeValid(int src_width,
                             int src_height,
                             int dst_width,
                             int dst_height) {
  if (src_width > kMaxWidth || src_height > kMaxHeight ||
      dst_width > kMaxWidth || dst_height > kMaxHeight) {
    printf("Warning - size too large to test.  Skipping\n");
    return false;
  }
  return true;
}

#define align_buffer_page_end(var, size)                                \
  uint8_t* var = NULL;                                                  \
  uint8_t* var##_mem =                                                  \
      reinterpret_cast<uint8_t*>(malloc(((size) + 4095 + 63) & ~4095)); \
  if (var##_mem)                                                        \
  var = reinterpret_cast<uint8_t*>(                                     \
      (intptr_t)(var##_mem + (((size) + 4095 + 63) & ~4095) - (size)) & ~63)

#define free_aligned_buffer_page_end(var) \
  free(var##_mem);                        \
  var = NULL

#define align_buffer_page_end_16(var, size)                                 \
  uint16_t* var = NULL;                                                     \
  uint8_t* var##_mem =                                                      \
      reinterpret_cast<uint8_t*>(malloc(((size)*2 + 4095 + 63) & ~4095));   \
  if (var##_mem)                                                            \
  var = reinterpret_cast<uint16_t*>(                                        \
      (intptr_t)(var##_mem + (((size)*2 + 4095 + 63) & ~4095) - (size)*2) & \
      ~63)

#define free_aligned_buffer_page_end_16(var) \
  free(var##_mem);                           \
  var = NULL

#ifdef WIN32
static inline double get_time() {
  LARGE_INTEGER t, f;
  QueryPerformanceCounter(&t);
  QueryPerformanceFrequency(&f);
  return static_cast<double>(t.QuadPart) / static_cast<double>(f.QuadPart);
}
#else
static inline double get_time() {
  struct timeval t;
  struct timezone tzp;
  gettimeofday(&t, &tzp);
  return t.tv_sec + t.tv_usec * 1e-6;
}
#endif

#ifndef SIMD_ALIGNED
#if defined(_MSC_VER) && !defined(__CLR_VER)
#define SIMD_ALIGNED(var) __declspec(align(16)) var
#elif defined(__GNUC__) && !defined(__pnacl__)
#define SIMD_ALIGNED(var) var __attribute__((aligned(16)))
#else
#define SIMD_ALIGNED(var) var
#endif
#endif

extern unsigned int fastrand_seed;
inline int fastrand() {
  fastrand_seed = fastrand_seed * 214013u + 2531011u;
  return static_cast<int>((fastrand_seed >> 16) & 0xffff);
}

// ubsan fails if dst is unaligned unless we use uint8
static inline void MemRandomize(uint8_t* dst, int64_t len) {
  int64_t i;
  for (i = 0; i < len - 1; i += 2) {
    int r = fastrand();
    dst[0] = static_cast<uint8_t>(r);
    dst[1] = static_cast<uint8_t>(r >> 8);
    dst += 2;
  }
  for (; i < len; ++i) {
    *dst++ = fastrand();
  }
}

class LibYUVColorTest : public ::testing::Test {
 protected:
  LibYUVColorTest();

  int benchmark_iterations_;  // Default 1. Use 1000 for benchmarking.
  int benchmark_width_;       // Default 1280.  Use 640 for benchmarking VGA.
  int benchmark_height_;      // Default 720.  Use 360 for benchmarking VGA.
  int benchmark_pixels_div1280_;  // Total pixels to benchmark / 1280.
  int disable_cpu_flags_;         // Default 1.  Use -1 for benchmarking.
  int benchmark_cpu_info_;        // Default -1.  Use 1 to disable SIMD.
};

class LibYUVConvertTest : public ::testing::Test {
 protected:
  LibYUVConvertTest();

  int benchmark_iterations_;  // Default 1. Use 1000 for benchmarking.
  int benchmark_width_;       // Default 1280.  Use 640 for benchmarking VGA.
  int benchmark_height_;      // Default 720.  Use 360 for benchmarking VGA.
  int benchmark_pixels_div1280_;  // Total pixels to benchmark / 1280.
  int disable_cpu_flags_;         // Default 1.  Use -1 for benchmarking.
  int benchmark_cpu_info_;        // Default -1.  Use 1 to disable SIMD.
};

class LibYUVScaleTest : public ::testing::Test {
 protected:
  LibYUVScaleTest();

  int benchmark_iterations_;  // Default 1. Use 1000 for benchmarking.
  int benchmark_width_;       // Default 1280.  Use 640 for benchmarking VGA.
  int benchmark_height_;      // Default 720.  Use 360 for benchmarking VGA.
  int benchmark_pixels_div1280_;  // Total pixels to benchmark / 1280.
  int disable_cpu_flags_;         // Default 1.  Use -1 for benchmarking.
  int benchmark_cpu_info_;        // Default -1.  Use 1 to disable SIMD.
};

class LibYUVRotateTest : public ::testing::Test {
 protected:
  LibYUVRotateTest();

  int benchmark_iterations_;  // Default 1. Use 1000 for benchmarking.
  int benchmark_width_;       // Default 1280.  Use 640 for benchmarking VGA.
  int benchmark_height_;      // Default 720.  Use 360 for benchmarking VGA.
  int benchmark_pixels_div1280_;  // Total pixels to benchmark / 1280.
  int disable_cpu_flags_;         // Default 1.  Use -1 for benchmarking.
  int benchmark_cpu_info_;        // Default -1.  Use 1 to disable SIMD.
};

class LibYUVPlanarTest : public ::testing::Test {
 protected:
  LibYUVPlanarTest();

  int benchmark_iterations_;  // Default 1. Use 1000 for benchmarking.
  int benchmark_width_;       // Default 1280.  Use 640 for benchmarking VGA.
  int benchmark_height_;      // Default 720.  Use 360 for benchmarking VGA.
  int benchmark_pixels_div1280_;  // Total pixels to benchmark / 1280.
  int disable_cpu_flags_;         // Default 1.  Use -1 for benchmarking.
  int benchmark_cpu_info_;        // Default -1.  Use 1 to disable SIMD.
};

class LibYUVBaseTest : public ::testing::Test {
 protected:
  LibYUVBaseTest();

  int benchmark_iterations_;  // Default 1. Use 1000 for benchmarking.
  int benchmark_width_;       // Default 1280.  Use 640 for benchmarking VGA.
  int benchmark_height_;      // Default 720.  Use 360 for benchmarking VGA.
  int benchmark_pixels_div1280_;  // Total pixels to benchmark / 1280.
  int disable_cpu_flags_;         // Default 1.  Use -1 for benchmarking.
  int benchmark_cpu_info_;        // Default -1.  Use 1 to disable SIMD.
};

class LibYUVCompareTest : public ::testing::Test {
 protected:
  LibYUVCompareTest();

  int benchmark_iterations_;  // Default 1. Use 1000 for benchmarking.
  int benchmark_width_;       // Default 1280.  Use 640 for benchmarking VGA.
  int benchmark_height_;      // Default 720.  Use 360 for benchmarking VGA.
  int benchmark_pixels_div1280_;  // Total pixels to benchmark / 1280.
  int disable_cpu_flags_;         // Default 1.  Use -1 for benchmarking.
  int benchmark_cpu_info_;        // Default -1.  Use 1 to disable SIMD.
};

#endif  // UNIT_TEST_UNIT_TEST_H_  NOLINT
