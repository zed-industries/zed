// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_DANGLING_RAW_PTR_CHECKS_H_
#define PARTITION_ALLOC_DANGLING_RAW_PTR_CHECKS_H_

#include <cstdint>

#include "partition_alloc/partition_alloc_base/component_export.h"

// When compiled with build flags `enable_dangling_raw_ptr_checks`, dangling
// raw_ptr are reported. Its behavior can be configured here.
//
// Purpose of this level of indirection:
// - Ease testing.
// - Keep partition_alloc/ independent from base/. In most cases, when a
//   dangling raw_ptr is detected/released, this involves recording a
//   base::debug::StackTrace, which isn't desirable inside partition_alloc/.
// - Be able (potentially) to turn this feature on/off at runtime based on
//   dependant's flags.
namespace partition_alloc {

// DanglingRawPtrDetected is called when there exists a `raw_ptr` referencing a
// memory region and the allocator is asked to release it.
//
// It won't be called again with the same `id`, up until (potentially) a call to
// DanglingRawPtrReleased(`id`) is made.
//
// This function is called from within the allocator, and is not allowed to
// allocate memory.
using DanglingRawPtrDetectedFn = void(uintptr_t /*id*/);
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
DanglingRawPtrDetectedFn* GetDanglingRawPtrDetectedFn();
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
void SetDanglingRawPtrDetectedFn(DanglingRawPtrDetectedFn);

PA_COMPONENT_EXPORT(PARTITION_ALLOC)
DanglingRawPtrDetectedFn* GetUnretainedDanglingRawPtrDetectedFn();
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
void SetUnretainedDanglingRawPtrDetectedFn(DanglingRawPtrDetectedFn*);
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
bool SetUnretainedDanglingRawPtrCheckEnabled(bool enabled);

// DanglingRawPtrReleased: Called after DanglingRawPtrDetected(id), once the
// last dangling raw_ptr stops referencing the memory region.
//
// This function is allowed to allocate memory.
using DanglingRawPtrReleasedFn = void(uintptr_t /*id*/);
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
DanglingRawPtrReleasedFn* GetDanglingRawPtrReleasedFn();
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
void SetDanglingRawPtrReleasedFn(DanglingRawPtrReleasedFn);

namespace internal {

PA_COMPONENT_EXPORT(PARTITION_ALLOC) void DanglingRawPtrDetected(uintptr_t id);
PA_COMPONENT_EXPORT(PARTITION_ALLOC) void DanglingRawPtrReleased(uintptr_t id);
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
void UnretainedDanglingRawPtrDetected(uintptr_t id);
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
bool IsUnretainedDanglingRawPtrCheckEnabled();

}  // namespace internal
}  // namespace partition_alloc

#endif  // PARTITION_ALLOC_DANGLING_RAW_PTR_CHECKS_H_
