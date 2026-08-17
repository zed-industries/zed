// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_DEFAULT_DISPATCH_TO_PARTITION_ALLOC_INTERNAL_H_
#define PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_DEFAULT_DISPATCH_TO_PARTITION_ALLOC_INTERNAL_H_

#include "partition_alloc/buildflags.h"

#if PA_BUILDFLAG(USE_ALLOCATOR_SHIM)
#include "partition_alloc/shim/allocator_shim_default_dispatch_to_partition_alloc.h"

namespace allocator_shim::internal {

// Only allocator_shim component must include this header, because
// PartitionMalloc, PartitionMallocUnchecked, ... are DLL-exported when
// is_component_build=true and is_win=true. In the case, the other component
// needs to import the symbols from allocator_shim.dll...so, not constexpr.
inline constexpr AllocatorDispatch kPartitionAllocDispatch =
    allocator_shim::internal::PartitionAllocFunctions::MakeDispatch();

}  // namespace allocator_shim::internal

#endif  // PA_BUILDFLAG(USE_ALLOCATOR_SHIM)

#endif  // PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_DEFAULT_DISPATCH_TO_PARTITION_ALLOC_INTERNAL_H_
