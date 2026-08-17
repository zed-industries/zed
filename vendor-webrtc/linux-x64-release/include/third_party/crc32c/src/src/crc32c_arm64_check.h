// Copyright 2017 The CRC32C Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file. See the AUTHORS file for names of contributors.

// ARM-specific code checking for the availability of CRC32C instructions.

#ifndef CRC32C_CRC32C_ARM_CHECK_H_
#define CRC32C_CRC32C_ARM_CHECK_H_

#include <cstddef>
#include <cstdint>

#include "crc32c/crc32c_config.h"

#if HAVE_ARM64_CRC32C

#ifdef __linux__
#if HAVE_STRONG_GETAUXVAL
#include <sys/auxv.h>
#elif HAVE_WEAK_GETAUXVAL
// getauxval() is not available on Android until API level 20. Link it as a weak
// symbol.
extern "C" unsigned long getauxval(unsigned long type) __attribute__((weak));

#define AT_HWCAP 16
#endif  // HAVE_STRONG_GETAUXVAL || HAVE_WEAK_GETAUXVAL
#endif  // defined (__linux__)

#ifdef __APPLE__
#include <sys/types.h>
#include <sys/sysctl.h>
#endif  // defined (__APPLE__)

namespace crc32c {

inline bool CanUseArm64Crc32() {
#if defined (__linux__) && (HAVE_STRONG_GETAUXVAL || HAVE_WEAK_GETAUXVAL)
  // From 'arch/arm64/include/uapi/asm/hwcap.h' in Linux kernel source code.
  constexpr unsigned long kHWCAP_PMULL = 1 << 4;
  constexpr unsigned long kHWCAP_CRC32 = 1 << 7;
  unsigned long hwcap =
#if HAVE_STRONG_GETAUXVAL
      // Some compilers warn on (&getauxval != nullptr) in the block below.
      getauxval(AT_HWCAP);
#elif HAVE_WEAK_GETAUXVAL
      (&getauxval != nullptr) ? getauxval(AT_HWCAP) : 0;
#else
#error This is supposed to be nested inside a check for HAVE_*_GETAUXVAL.
#endif  // HAVE_STRONG_GETAUXVAL
  return (hwcap & (kHWCAP_PMULL | kHWCAP_CRC32)) ==
         (kHWCAP_PMULL | kHWCAP_CRC32);
#elif defined(__APPLE__)
  int val = 0;
  size_t len = sizeof(val);
  return sysctlbyname("hw.optional.armv8_crc32", &val, &len, nullptr, 0) == 0
             && val != 0;
#else
  return false;
#endif  // HAVE_STRONG_GETAUXVAL || HAVE_WEAK_GETAUXVAL
}

}  // namespace crc32c

#endif  // HAVE_ARM64_CRC32C

#endif  // CRC32C_CRC32C_ARM_CHECK_H_
