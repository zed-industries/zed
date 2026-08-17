// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_THREAD_ISOLATION_THREAD_ISOLATION_H_
#define PARTITION_ALLOC_THREAD_ISOLATION_THREAD_ISOLATION_H_

#include "partition_alloc/buildflags.h"

#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)

#include <cstddef>
#include <cstdint>

#include "partition_alloc/partition_alloc_base/component_export.h"

#if PA_BUILDFLAG(ENABLE_PKEYS)
#include "partition_alloc/thread_isolation/pkey.h"
#endif

#if !PA_BUILDFLAG(HAS_64_BIT_POINTERS)
#error "thread isolation support requires 64 bit pointers"
#endif

namespace partition_alloc {

struct ThreadIsolationOption {
  constexpr ThreadIsolationOption() = default;
  explicit ThreadIsolationOption(bool enabled) : enabled(enabled) {}

#if PA_BUILDFLAG(ENABLE_PKEYS)
  explicit ThreadIsolationOption(int pkey) : pkey(pkey) {
    enabled = pkey != internal::kInvalidPkey;
  }
  int pkey = -1;
#endif  // PA_BUILDFLAG(ENABLE_PKEYS)

  bool enabled = false;

  bool operator==(const ThreadIsolationOption& other) const {
#if PA_BUILDFLAG(ENABLE_PKEYS)
    if (pkey != other.pkey) {
      return false;
    }
#endif  // PA_BUILDFLAG(ENABLE_PKEYS)
    return enabled == other.enabled;
  }
};

}  // namespace partition_alloc

namespace partition_alloc::internal {

#if PA_BUILDFLAG(DCHECKS_ARE_ON) || \
    PA_BUILDFLAG(ENABLE_PARTITION_LOCK_REENTRANCY_CHECK)

struct PA_THREAD_ISOLATED_ALIGN ThreadIsolationSettings {
  bool enabled = false;
  PA_CONSTINIT static ThreadIsolationSettings settings;
};

#if PA_BUILDFLAG(ENABLE_PKEYS)

using LiftThreadIsolationScope = LiftPkeyRestrictionsScope;

#endif  // PA_BUILDFLAG(ENABLE_PKEYS)
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON) ||
        // PA_BUILDFLAG(ENABLE_PARTITION_LOCK_REENTRANCY_CHECK)

void WriteProtectThreadIsolatedGlobals(ThreadIsolationOption thread_isolation);
void UnprotectThreadIsolatedGlobals();
[[nodiscard]] int MprotectWithThreadIsolation(
    void* addr,
    size_t len,
    int prot,
    ThreadIsolationOption thread_isolation);

}  // namespace partition_alloc::internal

#endif  // PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)

#endif  // PARTITION_ALLOC_THREAD_ISOLATION_THREAD_ISOLATION_H_
