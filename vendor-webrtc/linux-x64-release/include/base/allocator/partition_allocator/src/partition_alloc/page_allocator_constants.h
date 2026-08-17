// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PAGE_ALLOCATOR_CONSTANTS_H_
#define PARTITION_ALLOC_PAGE_ALLOCATOR_CONSTANTS_H_

#include <cstddef>

#include "partition_alloc/build_config.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/component_export.h"

#if PA_BUILDFLAG(IS_APPLE) && PA_BUILDFLAG(PA_ARCH_CPU_64_BITS)

#include <mach/vm_page_size.h>

// Although page allocator constants are not constexpr, they are run-time
// constant. Because the underlying variables they access, such as vm_page_size,
// are not marked const, the compiler normally has no way to know that they
// don’t change and must obtain their values whenever it can't prove that they
// haven't been modified, even if they had already been obtained previously.
// Attaching __attribute__((const)) to these declarations allows these redundant
// accesses to be omitted under optimization such as common subexpression
// elimination.
#define PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR __attribute__((const))

#elif (PA_BUILDFLAG(IS_ANDROID) && PA_BUILDFLAG(PA_ARCH_CPU_64_BITS)) || \
    (PA_BUILDFLAG(IS_LINUX) && PA_BUILDFLAG(PA_ARCH_CPU_ARM64)) ||       \
    (PA_BUILDFLAG(IS_LINUX) && PA_BUILDFLAG(PA_ARCH_CPU_PPC64))
// This should work for all POSIX (if needed), but currently all other
// supported OS/architecture combinations use either hard-coded values
// (such as x86) or have means to determine these values without needing
// atomics (such as macOS on arm64).

#define PARTITION_ALLOCATOR_CONSTANTS_POSIX_NONCONST_PAGE_SIZE

// Page allocator constants are run-time constant
#define PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR __attribute__((const))

#include <unistd.h>

#include <atomic>

namespace partition_alloc::internal {

// Holds the current page size and shift, where size = 1 << shift
// Use PageAllocationGranularity(), PageAllocationGranularityShift()
// to initialize and retrieve these values safely.
struct PageCharacteristics {
  std::atomic<size_t> size;
  std::atomic<size_t> shift;
};
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
extern PageCharacteristics page_characteristics;

}  // namespace partition_alloc::internal

#else

// When defined, page size constants are fixed at compile time. When not
// defined, they may vary at run time.
#define PAGE_ALLOCATOR_CONSTANTS_ARE_CONSTEXPR 1

// Use this macro to declare a function as constexpr or not based on whether
// PAGE_ALLOCATOR_CONSTANTS_ARE_CONSTEXPR is defined.
#define PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR constexpr

#endif

// Ability to name anonymous VMAs is available on some, but not all Linux-based
// systems.
#if PA_BUILDFLAG(IS_ANDROID) || PA_BUILDFLAG(IS_LINUX) || \
    PA_BUILDFLAG(IS_CHROMEOS)
#include <sys/prctl.h>

#if (PA_BUILDFLAG(IS_LINUX) || PA_BUILDFLAG(IS_CHROMEOS)) && \
    !(defined(PR_SET_VMA) && defined(PR_SET_VMA_ANON_NAME))

// The PR_SET_VMA* symbols are originally from
// https://android.googlesource.com/platform/bionic/+/lollipop-release/libc/private/bionic_prctl.h
// and were subsequently added to mainline Linux in Jan 2022, see
// https://github.com/torvalds/linux/commit/9a10064f5625d5572c3626c1516e0bebc6c9fe9b.
//
// Define them to support compiling with older headers.
#if !defined(PR_SET_VMA)
#define PR_SET_VMA 0x53564d41
#endif

#if !defined(PR_SET_VMA_ANON_NAME)
#define PR_SET_VMA_ANON_NAME 0
#endif

#endif  // (PA_BUILDFLAG(IS_LINUX) || PA_BUILDFLAG(IS_CHROMEOS)) &&
        // !(defined(PR_SET_VMA) && defined(PR_SET_VMA_ANON_NAME))

#if defined(PR_SET_VMA) && defined(PR_SET_VMA_ANON_NAME)
#define LINUX_NAME_REGION 1
#endif

#endif  // PA_BUILDFLAG(IS_ANDROID) || PA_BUILDFLAG(IS_LINUX)

namespace partition_alloc {
namespace internal {

// Forward declaration, implementation below
PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR size_t
PageAllocationGranularity();

PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR size_t
PageAllocationGranularityShift() {
#if defined(PARTITION_ALLOCATOR_CONSTANTS_POSIX_NONCONST_PAGE_SIZE)
  // arm64 supports 4kb (shift = 12), 16kb (shift = 14), and 64kb (shift = 16)
  // page sizes. Retrieve from or initialize cache.
  size_t shift = page_characteristics.shift.load(std::memory_order_relaxed);
  if (shift == 0) [[unlikely]] {
    shift = static_cast<size_t>(
        __builtin_ctz((unsigned int)PageAllocationGranularity()));
    page_characteristics.shift.store(shift, std::memory_order_relaxed);
  }
  return shift;
#elif PA_BUILDFLAG(IS_WIN) || PA_BUILDFLAG(PA_ARCH_CPU_PPC64)
  // Modern ppc64 systems support 4kB (shift = 12) and 64kB (shift = 16) page
  // sizes.  Since 64kB is the de facto standard on the platform and binaries
  // compiled for 64kB are likely to work on 4kB systems, 64kB is a good choice
  // here.
  return 16;  // 64kB
#elif defined(_MIPS_ARCH_LOONGSON) || PA_BUILDFLAG(PA_ARCH_CPU_LOONGARCH64)
  return 14;  // 16kB
#elif PA_BUILDFLAG(IS_APPLE) && PA_BUILDFLAG(PA_ARCH_CPU_64_BITS)
  return static_cast<size_t>(vm_page_shift);
#elif PA_BUILDFLAG(IS_WIN) || PA_BUILDFLAG(PA_ARCH_CPU_PPC64)
  // Modern ppc64 systems support 4kB (shift = 12) and 64kB (shift = 16) page
  // sizes.  Since 64kB is the de facto standard on the platform and binaries
  // compiled for 64kB are likely to work on 4kB systems, 64kB is a good choice
  // here.
  return 16;  // 64kB
#elif defined(_MIPS_ARCH_LOONGSON) || PA_BUILDFLAG(PA_ARCH_CPU_LOONGARCH64)
  return 14;  // 16kB
#elif PA_BUILDFLAG(IS_APPLE) && PA_BUILDFLAG(PA_ARCH_CPU_64_BITS)
  return static_cast<size_t>(vm_page_shift);
#else
  return 12;  // 4kB
#endif
}

PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR size_t
PageAllocationGranularity() {
#if PA_BUILDFLAG(IS_APPLE) && PA_BUILDFLAG(PA_ARCH_CPU_64_BITS)
  // This is literally equivalent to |1 << PageAllocationGranularityShift()|
  // below, but was separated out for IS_APPLE to avoid << on a non-constexpr.
  return vm_page_size;
#elif defined(PARTITION_ALLOCATOR_CONSTANTS_POSIX_NONCONST_PAGE_SIZE)
  // arm64 supports 4kb, 16kb, and 64kb page sizes. Retrieve from or
  // initialize cache.
  size_t size = page_characteristics.size.load(std::memory_order_relaxed);
  if (size == 0) [[unlikely]] {
    size = static_cast<size_t>(getpagesize());
    page_characteristics.size.store(size, std::memory_order_relaxed);
  }
  return size;
#else
  return 1 << PageAllocationGranularityShift();
#endif
}

PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR size_t
PageAllocationGranularityOffsetMask() {
  return PageAllocationGranularity() - 1;
}

PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR size_t
PageAllocationGranularityBaseMask() {
  return ~PageAllocationGranularityOffsetMask();
}

PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR size_t
SystemPageShift() {
  // On Windows allocation granularity is higher than the page size. This comes
  // into play when reserving address space range (allocation granularity),
  // compared to committing pages into memory (system page granularity).
#if PA_BUILDFLAG(IS_WIN)
  return 12;  // 4096=1<<12
#else
  return PageAllocationGranularityShift();
#endif
}

PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR size_t
SystemPageSize() {
#if (PA_BUILDFLAG(IS_APPLE) && PA_BUILDFLAG(PA_ARCH_CPU_64_BITS)) || \
    defined(PARTITION_ALLOCATOR_CONSTANTS_POSIX_NONCONST_PAGE_SIZE)
  // This is literally equivalent to |1 << SystemPageShift()| below, but was
  // separated out for 64-bit IS_APPLE and arm64 on Android/Linux to avoid <<
  // on a non-constexpr.
  return PageAllocationGranularity();
#else
  return 1 << SystemPageShift();
#endif
}

PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR size_t
SystemPageOffsetMask() {
  return SystemPageSize() - 1;
}

PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR size_t
SystemPageBaseMask() {
  return ~SystemPageOffsetMask();
}

constexpr size_t kPageMetadataShift = 5;  // 32 bytes per partition page.
constexpr size_t kPageMetadataSize = 1 << kPageMetadataShift;

}  // namespace internal

PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR size_t
SystemPageSize() {
  return internal::SystemPageSize();
}

}  // namespace partition_alloc

#endif  // PARTITION_ALLOC_PAGE_ALLOCATOR_CONSTANTS_H_
