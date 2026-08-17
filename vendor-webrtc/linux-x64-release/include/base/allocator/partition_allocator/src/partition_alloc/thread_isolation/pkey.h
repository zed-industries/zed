// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_THREAD_ISOLATION_PKEY_H_
#define PARTITION_ALLOC_THREAD_ISOLATION_PKEY_H_

#include "partition_alloc/buildflags.h"

#if PA_BUILDFLAG(ENABLE_PKEYS)

#include <cstddef>
#include <cstdint>

#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/thread_isolation/alignment.h"

namespace partition_alloc::internal {

constexpr int kDefaultPkey = 0;
constexpr int kInvalidPkey = -1;

// Check if the CPU supports pkeys.
bool CPUHasPkeySupport();

// A wrapper around the pkey_mprotect syscall.
[[nodiscard]] int PkeyMprotect(void* addr, size_t len, int prot, int pkey);

void TagMemoryWithPkey(int pkey, void* address, size_t size);

int PkeyAlloc(int access_rights);

void PkeyFree(int pkey);

// Read the pkru register (the current pkey state).
uint32_t Rdpkru();

// Write the pkru register (the current pkey state).
void Wrpkru(uint32_t pkru);

#if PA_BUILDFLAG(DCHECKS_ARE_ON) || \
    PA_BUILDFLAG(ENABLE_PARTITION_LOCK_REENTRANCY_CHECK)

class PA_COMPONENT_EXPORT(PARTITION_ALLOC) LiftPkeyRestrictionsScope {
 public:
  static constexpr uint32_t kDefaultPkeyValue = 0x55555554;
  static constexpr uint32_t kAllowAllPkeyValue = 0x0;

  LiftPkeyRestrictionsScope();
  ~LiftPkeyRestrictionsScope();

 private:
  uint32_t saved_pkey_value_;
};

#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON) ||
        // PA_BUILDFLAG(ENABLE_PARTITION_LOCK_REENTRANCY_CHECK)

}  // namespace partition_alloc::internal

#endif  // PA_BUILDFLAG(ENABLE_PKEYS)

#endif  // PARTITION_ALLOC_THREAD_ISOLATION_PKEY_H_
