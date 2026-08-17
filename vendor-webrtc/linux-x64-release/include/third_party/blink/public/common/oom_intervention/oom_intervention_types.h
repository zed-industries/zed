// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_OOM_INTERVENTION_OOM_INTERVENTION_TYPES_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_OOM_INTERVENTION_OOM_INTERVENTION_TYPES_H_

#include <stdint.h>

namespace blink {

// The struct with renderer metrics that are used to detect OOMs. This is stored
// in shared memory so that browser can read it even after the renderer dies.
// Use uint64_t for this struct in order to keep the memory layout exactly the
// same across architectures, since it is possible on Android to have browser in
// the arm64 and renderer in the arm32.

struct OomInterventionMetrics {
  uint64_t current_available_memory_kb = 0;
  uint64_t current_swap_free_kb = 0;

  // Indicates whether the crash was because of virtual address space OOM.
  // This holds only 0 or 1 as a value but because of the reason stated above,
  // uses uint64_t instead of boolean.
  uint64_t allocation_failed = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_OOM_INTERVENTION_OOM_INTERVENTION_TYPES_H_
