// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_RANDOM_H_
#define PARTITION_ALLOC_RANDOM_H_

#include <cstdint>

#include "partition_alloc/partition_alloc_base/component_export.h"

namespace partition_alloc {

namespace internal {

// Returns a random value. The generator's internal state is initialized with
// `base::RandUint64` which is very unpredictable, but which is expensive due to
// the need to call into the kernel. Therefore this generator uses a fast,
// entirely user-space function after initialization.
PA_COMPONENT_EXPORT(PARTITION_ALLOC) uint32_t RandomValue();

}  // namespace internal

// Sets the seed for the random number generator to a known value, to cause the
// RNG to generate a predictable sequence of outputs. May be called multiple
// times.
PA_COMPONENT_EXPORT(PARTITION_ALLOC) void SetMmapSeedForTesting(uint64_t seed);

}  // namespace partition_alloc

#endif  // PARTITION_ALLOC_RANDOM_H_
