// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_TAGGING_H_
#define PARTITION_ALLOC_TAGGING_H_

// This file contains method definitions to support Armv8.5-A's memory tagging
// extension.

#include <cstddef>
#include <cstdint>

#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_config.h"

#if PA_BUILDFLAG(HAS_MEMORY_TAGGING) && PA_BUILDFLAG(IS_ANDROID)
#include <csignal>
#endif

namespace partition_alloc {

// Enum configures Arm's MTE extension to operate in different modes
enum class TagViolationReportingMode {
  // Default settings
  kUndefined,
  // MTE explicitly disabled.
  kDisabled,
  // Precise tag violation reports, higher overhead. Good for unittests
  // and security critical threads.
  kSynchronous,
  // Imprecise tag violation reports (async mode). Lower overhead.
  kAsynchronous,
};

// Changes the memory tagging mode for the calling thread.
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
void ChangeMemoryTaggingModeForCurrentThread(TagViolationReportingMode);

namespace internal {

inline constexpr uint64_t kMemTagGranuleSize = 16u;
#if PA_BUILDFLAG(HAS_MEMORY_TAGGING)
inline constexpr uint64_t kPtrTagMask = 0xff00000000000000uLL;
inline constexpr size_t kPtrTagShift = 56;
static_assert(kPtrTagMask == (0xffULL << kPtrTagShift),
              "kPtrTagMask and kPtrTagShift must be consistent");
#else
inline constexpr uint64_t kPtrTagMask = 0;
inline constexpr size_t kPtrTagShift = 0;
#endif  // PA_BUILDFLAG(HAS_MEMORY_TAGGING)
inline constexpr uint64_t kPtrUntagMask = ~kPtrTagMask;

#if PA_BUILDFLAG(IS_ANDROID)
// Changes the memory tagging mode for all threads in the current process.
// Returns true on success. Most likely reason for failure is because heap
// tagging may not be re-enabled after being disabled.
// https://android.googlesource.com/platform/bionic/+/446b4dde724ee64a336a78188c3c9a15aebca87c/libc/include/malloc.h#235
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
bool ChangeMemoryTaggingModeForAllThreadsPerProcess(TagViolationReportingMode);
#endif

// Gets the memory tagging mode for the calling thread. Returns kUndefined if
// MTE support is not available.
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
TagViolationReportingMode GetMemoryTaggingModeForCurrentThread();

// These forward-defined functions do not really exist in tagging.cc, they're
// resolved by the dynamic linker to MTE-capable versions on the right hardware.
#if PA_BUILDFLAG(HAS_MEMORY_TAGGING)
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
void* TagMemoryRangeIncrementInternal(void* ptr, size_t size);
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
void* TagMemoryRangeRandomlyInternal(void* ptr, size_t size, uint64_t mask);
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
void* RemaskPointerInternal(void* ptr);
#endif  // PA_BUILDFLAG(HAS_MEMORY_TAGGING)

// Increments the tag of the memory range ptr. Useful for provable revocations
// (e.g. free). Returns the pointer with the new tag. Ensures that the entire
// range is set to the same tag.
PA_ALWAYS_INLINE void* TagMemoryRangeIncrement(void* ptr, size_t size) {
#if PA_BUILDFLAG(HAS_MEMORY_TAGGING)
  return TagMemoryRangeIncrementInternal(ptr, size);
#else
  return ptr;
#endif  // PA_BUILDFLAG(HAS_MEMORY_TAGGING)
}

PA_ALWAYS_INLINE void* TagMemoryRangeIncrement(uintptr_t address, size_t size) {
  return TagMemoryRangeIncrement(reinterpret_cast<void*>(address), size);
}

// Randomly changes the tag of the ptr memory range. Useful for initial random
// initialization. Returns the pointer with the new tag. Ensures that the entire
// range is set to the same tag.
PA_ALWAYS_INLINE void* TagMemoryRangeRandomly(void* ptr,
                                              size_t size,
                                              uint64_t mask = 0u) {
#if PA_BUILDFLAG(HAS_MEMORY_TAGGING)
  return reinterpret_cast<void*>(
      TagMemoryRangeRandomlyInternal(ptr, size, mask));
#else
  return ptr;
#endif  // PA_BUILDFLAG(HAS_MEMORY_TAGGING)
}

PA_ALWAYS_INLINE void* TagMemoryRangeRandomly(uintptr_t address,
                                              size_t size,
                                              uint64_t mask = 0u) {
  return TagMemoryRangeRandomly(reinterpret_cast<void*>(address), size, mask);
}

// Gets a version of ptr that's safe to dereference.
template <typename T>
PA_ALWAYS_INLINE T* TagPtr(T* ptr) {
#if PA_BUILDFLAG(HAS_MEMORY_TAGGING)
  return reinterpret_cast<T*>(RemaskPointerInternal(ptr));
#else
  return ptr;
#endif  // PA_BUILDFLAG(HAS_MEMORY_TAGGING)
}

// Gets a version of |address| that's safe to dereference, and casts to a
// pointer.
PA_ALWAYS_INLINE void* TagAddr(uintptr_t address) {
  return TagPtr(reinterpret_cast<void*>(address));
}

// Strips the tag bits off |address|.
PA_ALWAYS_INLINE uintptr_t UntagAddr(uintptr_t address) {
#if PA_BUILDFLAG(HAS_MEMORY_TAGGING)
  return address & internal::kPtrUntagMask;
#else
  return address;
#endif  // PA_BUILDFLAG(HAS_MEMORY_TAGGING)
}

#if PA_BUILDFLAG(HAS_MEMORY_TAGGING)
template <typename T>
inline uint8_t ExtractTagFromPtr(T* ptr) {
  return (reinterpret_cast<uintptr_t>(ptr) >> kPtrTagShift) & 0xf;
}
#endif  // PA_BUILDFLAG(HAS_MEMORY_TAGGING)

}  // namespace internal

// Strips the tag bits off |ptr|.
template <typename T>
PA_ALWAYS_INLINE uintptr_t UntagPtr(T* ptr) {
  return internal::UntagAddr(reinterpret_cast<uintptr_t>(ptr));
}

#if PA_BUILDFLAG(HAS_MEMORY_TAGGING) && PA_BUILDFLAG(IS_ANDROID)
class PA_COMPONENT_EXPORT(PARTITION_ALLOC) PermissiveMte {
 public:
  static void SetEnabled(bool enabled);
  static bool HandleCrash(int signo, siginfo_t* siginfo, ucontext_t* context);

 private:
  static bool enabled_;
};
#endif  // PA_BUILDFLAG(HAS_MEMORY_TAGGING)

// Stops MTE tag checking for the current thread while this is alive. This does
// not affect the return value for GetMemoryTaggingModeForCurrentThread().
class PA_COMPONENT_EXPORT(PARTITION_ALLOC) SuspendTagCheckingScope final {
 public:
  SuspendTagCheckingScope() noexcept;
  ~SuspendTagCheckingScope();

 private:
#if PA_BUILDFLAG(HAS_MEMORY_TAGGING)
  // Stores the previous value of the Tag Check Override (TCO) register.
  uint64_t previous_tco_;
#endif
};

}  // namespace partition_alloc

#endif  // PARTITION_ALLOC_TAGGING_H_
