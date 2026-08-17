// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PAGE_ALLOCATOR_INTERNALS_POSIX_H_
#define PARTITION_ALLOC_PAGE_ALLOCATOR_INTERNALS_POSIX_H_

#include <sys/mman.h>
#include <sys/syscall.h>

#include <algorithm>
#include <atomic>
#include <cerrno>
#include <cstdint>
#include <cstring>

#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"
#include "partition_alloc/oom.h"
#include "partition_alloc/page_allocator.h"
#include "partition_alloc/page_allocator_constants.h"
#include "partition_alloc/partition_alloc_base/posix/eintr_wrapper.h"
#include "partition_alloc/partition_alloc_check.h"
#include "partition_alloc/thread_isolation/thread_isolation.h"

#if PA_BUILDFLAG(IS_ANDROID) || PA_BUILDFLAG(IS_LINUX)
#include <sys/prctl.h>
#endif

#if PA_BUILDFLAG(IS_LINUX) || PA_BUILDFLAG(IS_CHROMEOS)
#include <sys/resource.h>
#endif

#ifndef MAP_ANONYMOUS
#define MAP_ANONYMOUS MAP_ANON
#endif

namespace partition_alloc::internal {

#if defined(LINUX_NAME_REGION)
void NameRegion(void* start, size_t length, PageTag page_tag);
#endif  // defined(LINUX_NAME_REGION)

#if PA_BUILDFLAG(IS_APPLE)
// Tests whether the version of macOS supports the MAP_JIT flag and if the
// current process is signed with the hardened runtime and the allow-jit
// entitlement, returning whether MAP_JIT should be used to allocate regions
// that will contain JIT-compiled executable code.
bool UseMapJit();
#endif  // PA_BUILDFLAG(IS_IOS)

// |mmap| uses a nearby address if the hint address is blocked.
constexpr bool kHintIsAdvisory = true;
std::atomic<int32_t> s_allocPageErrorCode{0};

int GetAccessFlags(PageAccessibilityConfiguration accessibility);

uintptr_t SystemAllocPagesInternal(uintptr_t hint,
                                   size_t length,
                                   PageAccessibilityConfiguration accessibility,
                                   PageTag page_tag,
                                   int file_descriptor_for_shared_alloc) {
#if PA_BUILDFLAG(IS_APPLE)
  // Use a custom tag to make it easier to distinguish PartitionAlloc regions
  // in vmmap(1). Tags between 240-255 are supported.
  int fd = file_descriptor_for_shared_alloc == -1
               ? VM_MAKE_TAG(static_cast<int>(page_tag))
               : file_descriptor_for_shared_alloc;
#else
  int fd = file_descriptor_for_shared_alloc;
#endif

  int access_flag = GetAccessFlags(accessibility);
  int map_flags = MAP_ANONYMOUS | MAP_PRIVATE;

#if PA_BUILDFLAG(IS_APPLE)
  // On macOS, executables that are code signed with the "runtime" option cannot
  // execute writable memory by default. They can opt into this capability by
  // specifying the "com.apple.security.cs.allow-jit" code signing entitlement
  // and allocating the region with the MAP_JIT flag.
  static const bool kUseMapJit = UseMapJit();
  if (accessibility.permissions ==
          PageAccessibilityConfiguration::kInaccessibleWillJitLater &&
      kUseMapJit) {
    map_flags |= MAP_JIT;
    // iOS devices do not support toggling the page permissions after a MAP_JIT
    // call, they must be set initially. iOS has per-thread W^X state that
    // takes precedence over the mapping's permissions for MAP_JIT regions.
    // See https://developer.apple.com/forums/thread/672804
#if PA_BUILDFLAG(IS_IOS)
    access_flag = PROT_READ | PROT_WRITE | PROT_EXEC;
#endif
  }
#endif

  void* ret = mmap(reinterpret_cast<void*>(hint), length, access_flag,
                   map_flags, fd, 0);
  if (ret == MAP_FAILED) {
    s_allocPageErrorCode = errno;
    ret = nullptr;
  }

#if defined(LINUX_NAME_REGION)
  if (ret) {
    NameRegion(ret, length, page_tag);
  }
#endif

  return reinterpret_cast<uintptr_t>(ret);
}

bool TrySetSystemPagesAccessInternal(
    uintptr_t address,
    size_t length,
    PageAccessibilityConfiguration accessibility) {
#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
  if (accessibility.thread_isolation.enabled) {
    return 0 == MprotectWithThreadIsolation(reinterpret_cast<void*>(address),
                                            length,
                                            GetAccessFlags(accessibility),
                                            accessibility.thread_isolation);
  }
#endif  // PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
  return 0 == WrapEINTR(mprotect)(reinterpret_cast<void*>(address), length,
                                  GetAccessFlags(accessibility));
}

void SetSystemPagesAccessInternal(
    uintptr_t address,
    size_t length,
    PageAccessibilityConfiguration accessibility) {
  int access_flags = GetAccessFlags(accessibility);
  int ret;
#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
  if (accessibility.thread_isolation.enabled) {
    ret = MprotectWithThreadIsolation(reinterpret_cast<void*>(address), length,
                                      GetAccessFlags(accessibility),
                                      accessibility.thread_isolation);
  } else
#endif  // PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
  {
    ret = WrapEINTR(mprotect)(reinterpret_cast<void*>(address), length,
                              GetAccessFlags(accessibility));
  }

  // On Linux, man mprotect(2) states that ENOMEM is returned when (1) internal
  // kernel data structures cannot be allocated, (2) the address range is
  // invalid, or (3) this would split an existing mapping in a way that would
  // exceed the maximum number of allowed mappings.
  //
  // Neither are very likely, but we still get a lot of crashes here. This is
  // because setrlimit(RLIMIT_DATA)'s limit is checked and enforced here, if the
  // access flags match a "data" mapping, which in our case would be MAP_PRIVATE
  // | MAP_ANONYMOUS, and PROT_WRITE. see the call to may_expand_vm() in
  // mm/mprotect.c in the kernel for details.
  //
  // In this case, we are almost certainly bumping into the sandbox limit, mark
  // the crash as OOM. See SandboxLinux::LimitAddressSpace() for details.
  if (ret == -1 && errno == ENOMEM) {
    // Likely setrlimit(2) related.
    if (access_flags & PROT_WRITE) {
      OOM_CRASH(length);
    } else {
      // Linux-based systems have a low-ish default limit of ~65k VMAs per
      // process, and we've seen crashes triggered by that. Single out this
      // error so that we can track it in crashes.
      OnErrnoNoMem();
    }
  }

  PA_PCHECK(0 == ret);
}

void FreePagesInternal(uintptr_t address, size_t length) {
  PA_PCHECK(0 == munmap(reinterpret_cast<void*>(address), length));
}

uintptr_t TrimMappingInternal(uintptr_t base_address,
                              size_t base_length,
                              size_t trim_length,
                              PageAccessibilityConfiguration accessibility,
                              size_t pre_slack,
                              size_t post_slack) {
  uintptr_t ret = base_address;
  // We can resize the allocation run. Release unneeded memory before and after
  // the aligned range.
  if (pre_slack) {
    FreePages(base_address, pre_slack);
    ret = base_address + pre_slack;
  }
  if (post_slack) {
    FreePages(ret + trim_length, post_slack);
  }
  return ret;
}

void DecommitSystemPagesInternal(
    uintptr_t address,
    size_t length,
    PageAccessibilityDisposition accessibility_disposition) {
  // In POSIX, there is no decommit concept. Discarding is an effective way of
  // implementing the Windows semantics where the OS is allowed to not swap the
  // pages in the region.
  DiscardSystemPages(address, length);

  bool change_permissions =
      accessibility_disposition == PageAccessibilityDisposition::kRequireUpdate;
#if PA_BUILDFLAG(DCHECKS_ARE_ON)
  // This is not guaranteed, show that we're serious.
  //
  // More specifically, several callers have had issues with assuming that
  // memory is zeroed, this would hopefully make these bugs more visible.  We
  // don't memset() everything, because ranges can be very large, and doing it
  // over the entire range could make Chrome unusable with
  // PA_BUILDFLAG(DCHECKS_ARE_ON).
  //
  // Only do it when we are about to change the permissions, since we don't know
  // the previous permissions, and cannot restore them.
  if (!DecommittedMemoryIsAlwaysZeroed() && change_permissions) {
    // Memory may not be writable.
    size_t size = std::min(length, 2 * SystemPageSize());
    void* ptr = reinterpret_cast<void*>(address);
    PA_CHECK(mprotect(ptr, size, PROT_WRITE) == 0);
    memset(ptr, 0xcc, size);
  }
#endif

  // Make pages inaccessible, unless the caller requested to keep permissions.
  //
  // Note, there is a small window between these calls when the pages can be
  // incorrectly touched and brought back to memory. Not ideal, but doing those
  // operations in the opposite order resulted in PMF regression on Mac (see
  // crbug.com/1153021).
  if (change_permissions) {
    SetSystemPagesAccess(address, length,
                         PageAccessibilityConfiguration(
                             PageAccessibilityConfiguration::kInaccessible));
  }
}

bool DecommitAndZeroSystemPagesInternal(uintptr_t address,
                                        size_t length,
                                        PageTag page_tag) {
  int fd = -1;
#if PA_BUILDFLAG(IS_APPLE)
  fd = VM_MAKE_TAG(static_cast<int>(page_tag));
#endif

  // https://pubs.opengroup.org/onlinepubs/9699919799/functions/mmap.html: "If
  // a MAP_FIXED request is successful, then any previous mappings [...] for
  // those whole pages containing any part of the address range [pa,pa+len)
  // shall be removed, as if by an appropriate call to munmap(), before the
  // new mapping is established." As a consequence, the memory will be
  // zero-initialized on next access.
  void* ptr = reinterpret_cast<void*>(address);
  void* ret = mmap(ptr, length, PROT_NONE,
                   MAP_FIXED | MAP_ANONYMOUS | MAP_PRIVATE, fd, 0);
  if (ret == MAP_FAILED) {
    // Decomitting may create additional VMAs (e.g. if we're decommitting pages
    // in the middle of a larger mapping) and so it can fail with ENOMEM if the
    // limit of VMAs is exceeded.
    PA_CHECK(errno == ENOMEM);
    return false;
  }
  PA_CHECK(ret == ptr);
  // Since we just remapped the region, need to set is name again.
#if defined(LINUX_NAME_REGION)
  NameRegion(ret, length, page_tag);
#endif
  return true;
}

void RecommitSystemPagesInternal(
    uintptr_t address,
    size_t length,
    PageAccessibilityConfiguration accessibility,
    PageAccessibilityDisposition accessibility_disposition) {
  // On POSIX systems, the caller needs to simply read the memory to recommit
  // it. However, if decommit changed the permissions, recommit has to change
  // them back.
  if (accessibility_disposition ==
      PageAccessibilityDisposition::kRequireUpdate) {
    SetSystemPagesAccess(address, length, accessibility);
  }

#if PA_BUILDFLAG(IS_APPLE)
  // On macOS, to update accounting, we need to make another syscall. For more
  // details, see https://crbug.com/823915.
  madvise(reinterpret_cast<void*>(address), length, MADV_FREE_REUSE);
#endif
}

bool TryRecommitSystemPagesInternal(
    uintptr_t address,
    size_t length,
    PageAccessibilityConfiguration accessibility,
    PageAccessibilityDisposition accessibility_disposition) {
  // On POSIX systems, the caller needs to simply read the memory to recommit
  // it. However, if decommit changed the permissions, recommit has to change
  // them back.
  if (accessibility_disposition ==
      PageAccessibilityDisposition::kRequireUpdate) {
    bool ok = TrySetSystemPagesAccess(address, length, accessibility);
    if (!ok) {
      return false;
    }
  }

#if PA_BUILDFLAG(IS_APPLE)
  // On macOS, to update accounting, we need to make another syscall. For more
  // details, see https://crbug.com/823915.
  madvise(reinterpret_cast<void*>(address), length, MADV_FREE_REUSE);
#endif

  return true;
}

void DiscardSystemPagesInternal(uintptr_t address, size_t length) {
  void* ptr = reinterpret_cast<void*>(address);
#if PA_BUILDFLAG(IS_APPLE)
  int ret = madvise(ptr, length, MADV_FREE_REUSABLE);
  if (ret) {
    // MADV_FREE_REUSABLE sometimes fails, so fall back to MADV_DONTNEED.
    ret = madvise(ptr, length, MADV_DONTNEED);
  }
  PA_PCHECK(ret == 0);
#else   // PA_BUILDFLAG(IS_APPLE)
  // We have experimented with other flags, but with suboptimal results.
  //
  // MADV_FREE (Linux): Makes our memory measurements less predictable;
  // performance benefits unclear.
  //
  // Therefore, we just do the simple thing: MADV_DONTNEED.
  PA_PCHECK(0 == madvise(ptr, length, MADV_DONTNEED));
#endif  // PA_BUILDFLAG(IS_APPLE)
}

bool SealSystemPagesInternal(uintptr_t address, size_t length) {
  // TODO(sroettger): we either need to ensure that __NR_mseal is defined in the
  // headers used by builders or define it ourselves.
#if PA_BUILDFLAG(IS_LINUX) && defined(__NR_mseal)
  long ret = syscall(__NR_mseal, address, length, 0);
  return ret == 0;
#else
  return false;
#endif
}

}  // namespace partition_alloc::internal

#endif  // PARTITION_ALLOC_PAGE_ALLOCATOR_INTERNALS_POSIX_H_
