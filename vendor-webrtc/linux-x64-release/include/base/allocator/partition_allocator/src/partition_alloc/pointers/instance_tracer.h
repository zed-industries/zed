// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_POINTERS_INSTANCE_TRACER_H_
#define PARTITION_ALLOC_POINTERS_INSTANCE_TRACER_H_

#include <array>
#include <atomic>
#include <cstdint>
#include <utility>
#include <vector>

#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_base/cxx20_is_constant_evaluated.h"

namespace base::internal {

#if !PA_BUILDFLAG(ENABLE_BACKUP_REF_PTR_INSTANCE_TRACER)

// When the buildflag is disabled, use a minimal no-state implementation so
// sizeof(raw_ptr<T>) == sizeof(T*).
class InstanceTracer {
 public:
  constexpr uint64_t owner_id() const { return 0; }

  constexpr static void Trace([[maybe_unused]] uint64_t owner_id,
                              [[maybe_unused]] uintptr_t address) {}
  constexpr static void Untrace([[maybe_unused]] uint64_t owner_id) {}
};

#else

class PA_TRIVIAL_ABI InstanceTracer {
 public:
  constexpr InstanceTracer() : owner_id_(CreateOwnerId()) {}

  // Copy constructing InstanceTracer should not inherit the owner ID; the new
  // InstanceTracer needs a new ID to be separately tracked.
  constexpr InstanceTracer(const InstanceTracer&) : InstanceTracer() {}
  // Similarly, copy assigning an InstanceTracer should not take the owner ID
  // from the right-hand side; it should preserve the owner ID.
  constexpr InstanceTracer& operator=(const InstanceTracer&) { return *this; }

  constexpr InstanceTracer(InstanceTracer&&) : InstanceTracer() {}
  constexpr InstanceTracer& operator=(InstanceTracer&&) { return *this; }

  constexpr uint64_t owner_id() const { return owner_id_; }

  constexpr static void Trace(uint64_t owner_id,
                              bool may_dangle,
                              uintptr_t address) {
    if (partition_alloc::internal::base::is_constant_evaluated() ||
        owner_id == 0) {
      return;
    }
    TraceImpl(owner_id, may_dangle, address);
  }
  constexpr static void Untrace(uint64_t owner_id) {
    if (partition_alloc::internal::base::is_constant_evaluated() ||
        owner_id == 0) {
      return;
    }
    UntraceImpl(owner_id);
  }

  PA_COMPONENT_EXPORT(RAW_PTR)
  static std::vector<std::array<const void*, 32>> GetStackTracesForDanglingRefs(
      uintptr_t allocation);

  PA_COMPONENT_EXPORT(RAW_PTR)
  static std::vector<std::array<const void*, 32>>
  GetStackTracesForAddressForTest(const void* address);

 private:
  PA_COMPONENT_EXPORT(RAW_PTR)
  static void TraceImpl(uint64_t owner_id, bool may_dangle, uintptr_t address);
  PA_COMPONENT_EXPORT(RAW_PTR) static void UntraceImpl(uint64_t owner_id);

  constexpr uint64_t CreateOwnerId() {
    if (partition_alloc::internal::base::is_constant_evaluated()) {
      return 0;
    }
    return ++counter_;
  }

  PA_COMPONENT_EXPORT(RAW_PTR) static std::atomic<uint64_t> counter_;

  // 0 is treated as 'ownerless'. It is used as a sentinel for constexpr
  // raw_ptrs or other places where owner tracking doesn't make sense.
  uint64_t owner_id_ = 0;
};

#endif

}  // namespace base::internal

#endif  // PARTITION_ALLOC_POINTERS_INSTANCE_TRACER_H_
