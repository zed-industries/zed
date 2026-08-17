// Copyright 2017 The CRC32C Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file. See the AUTHORS file for names of contributors.

#ifndef CRC32C_CRC32C_SSE42_H_
#define CRC32C_CRC32C_SSE42_H_

// X86-specific code.

#include <cstddef>
#include <cstdint>

#include "crc32c/crc32c_config.h"

// The hardware-accelerated implementation is only enabled for 64-bit builds,
// because a straightforward 32-bit implementation actually runs slower than the
// portable version. Most X86 machines are 64-bit nowadays, so it doesn't make
// much sense to spend time building an optimized hardware-accelerated
// implementation.
#if HAVE_SSE42 && (defined(_M_X64) || defined(__x86_64__))

namespace crc32c {

// SSE4.2-accelerated implementation in crc32c_sse42.cc
uint32_t ExtendSse42(uint32_t crc, const uint8_t* data, size_t count);

}  // namespace crc32c

#endif  // HAVE_SSE42 && (defined(_M_X64) || defined(__x86_64__))

#endif  // CRC32C_CRC32C_SSE42_H_
