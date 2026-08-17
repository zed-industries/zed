// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_COOKIE_H_
#define PARTITION_ALLOC_PARTITION_COOKIE_H_

#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_check.h"

#if PA_BUILDFLAG(SMALLER_PARTITION_COOKIE)
#include "partition_alloc/in_slot_metadata.h"
#endif  // PA_BUILDFLAG(SMALLER_PARTITION_COOKIE)

namespace partition_alloc::internal {

#if PA_BUILDFLAG(SMALLER_PARTITION_COOKIE)
#if PA_BUILDFLAG(ENABLE_BACKUP_REF_PTR_SUPPORT)
static constexpr size_t kCookieSize =
    AlignUpInSlotMetadataSizeForApple(sizeof(InSlotMetadata));
static_assert(kCookieSize == kInSlotMetadataSizeAdjustment);
#else
// Size of `InSlotMetadata` is unknown: using 4 bytes as an estimate.
static constexpr size_t kCookieSize = AlignUpInSlotMetadataSizeForApple(4);
static_assert(kCookieSize <= 16);
#endif  //  PA_BUILDFLAG(ENABLE_BACKUP_REF_PTR_SUPPORT)
#else
static constexpr size_t kCookieSize = 16;
#endif  // PA_BUILDFLAG(SMALLER_PARTITION_COOKIE)

#if PA_BUILDFLAG(USE_PARTITION_COOKIE)

inline constexpr unsigned char kCookieValue[] = {
    0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xD0, 0x0D,
    0x13, 0x37, 0xF0, 0x05, 0xBA, 0x11, 0xAB, 0x1E};

constexpr size_t kPartitionCookieSizeAdjustment = kCookieSize;

[[noreturn]] PA_NOINLINE PA_NOT_TAIL_CALLED
    PA_COMPONENT_EXPORT(PARTITION_ALLOC) void CookieCorruptionDetected(
        const unsigned char* cookie_ptr,
        size_t slot_usable_size);

PA_ALWAYS_INLINE void PartitionCookieCheckValue(const unsigned char* cookie_ptr,
                                                size_t slot_usable_size) {
  for (size_t i = 0; i < kCookieSize; ++i, ++cookie_ptr) {
    if (*cookie_ptr != kCookieValue[i]) {
      CookieCorruptionDetected(cookie_ptr, slot_usable_size);
    }
  }
}

PA_ALWAYS_INLINE void PartitionCookieWriteValue(unsigned char* cookie_ptr) {
  for (size_t i = 0; i < kCookieSize; ++i, ++cookie_ptr) {
    *cookie_ptr = kCookieValue[i];
  }
}

#else

constexpr size_t kPartitionCookieSizeAdjustment = 0;

PA_ALWAYS_INLINE void PartitionCookieCheckValue(const unsigned char* address,
                                                size_t slot_usable_size) {}

PA_ALWAYS_INLINE void PartitionCookieWriteValue(unsigned char* cookie_ptr) {}

#endif  // PA_BUILDFLAG(USE_PARTITION_COOKIE)

}  // namespace partition_alloc::internal

#endif  // PARTITION_ALLOC_PARTITION_COOKIE_H_
