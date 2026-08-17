// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_DEFAULT_DISPATCH_TO_PARTITION_ALLOC_WITH_ADVANCED_CHECKS_H_
#define PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_DEFAULT_DISPATCH_TO_PARTITION_ALLOC_WITH_ADVANCED_CHECKS_H_

#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/shim/allocator_dispatch.h"

#if !PA_BUILDFLAG( \
    ENABLE_ALLOCATOR_SHIM_PARTITION_ALLOC_DISPATCH_WITH_ADVANCED_CHECKS_SUPPORT)
#error PartitionAlloc with Advanced Checks is not available in this build \
       configuration.
#endif

// PartitionAlloc with Advanced Checks is a feature to install extra safety
// checks into PartitionAlloc, on opt-in at runtime basis.
// `InsertAllocatorDispatch()` API is not capable of this feature as it always
// inserts the new dispatch at beginning of the chain. As Dispatch here captures
// 100% requests, it will result in all other sampling-based feature nullified.
// Instead, this feature replaces a default dispatch at compile-time, and
// forwards all requests to `allocator_shim::(anonymous
// namespace)::g_delegate_dispatch`. `g_delegate_dispatch` can be either normal
// PA or PA with Advanced Checks. There will be very slight but non-zero cost
// for this one extra trampoline call. To minimize the cost, only following
// functions are delegated.
//
// - `AllocatorDispatch::free_function`
// - `AllocatorDispatch::realloc_function`

namespace allocator_shim {

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
void InstallCustomDispatchForPartitionAllocWithAdvancedChecks();

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
void InstallCustomDispatchForTesting(AllocatorDispatch* dispatch);

PA_COMPONENT_EXPORT(ALLOCATOR_SHIM)
void UninstallCustomDispatch();

}  // namespace allocator_shim

#endif  // PARTITION_ALLOC_SHIM_ALLOCATOR_SHIM_DEFAULT_DISPATCH_TO_PARTITION_ALLOC_WITH_ADVANCED_CHECKS_H_
