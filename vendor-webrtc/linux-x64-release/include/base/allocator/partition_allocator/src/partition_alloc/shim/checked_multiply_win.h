// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_SHIM_CHECKED_MULTIPLY_WIN_H_
#define PARTITION_ALLOC_SHIM_CHECKED_MULTIPLY_WIN_H_

#include "partition_alloc/partition_alloc_base/numerics/checked_math.h"

namespace allocator_shim::internal {

#if !defined(COMPONENT_BUILD)
PA_ALWAYS_INLINE
#endif
size_t CheckedMultiply(size_t multiplicand, size_t multiplier) {
  partition_alloc::internal::base::CheckedNumeric<size_t> result = multiplicand;
  result *= multiplier;
  return result.ValueOrDie();
}

}  // namespace allocator_shim::internal

#endif  // PARTITION_ALLOC_SHIM_CHECKED_MULTIPLY_WIN_H_
