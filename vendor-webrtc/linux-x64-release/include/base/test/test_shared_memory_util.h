// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_TEST_SHARED_MEMORY_UTIL_H_
#define BASE_TEST_TEST_SHARED_MEMORY_UTIL_H_

#include "base/memory/platform_shared_memory_region.h"
#include "base/memory/read_only_shared_memory_region.h"
#include "base/memory/shared_memory_mapping.h"
#include "testing/gtest/include/gtest/gtest.h"

namespace base {

// Check that the shared memory |region| cannot be used to perform a writable
// mapping with low-level system APIs like mmap(). Return true in case of
// success (i.e. writable mappings are _not_ allowed), or false otherwise.
bool CheckReadOnlyPlatformSharedMemoryRegionForTesting(
    subtle::PlatformSharedMemoryRegion region);

// Creates a scoped mapping from a PlatformSharedMemoryRegion. It's useful for
// PlatformSharedMemoryRegion testing to not leak mapped memory.
// WritableSharedMemoryMapping is used for wrapping because it has max
// capabilities but the actual permission depends on the |region|'s mode.
// This must not be used in production where PlatformSharedMemoryRegion should
// be wrapped with {Writable,Unsafe,ReadOnly}SharedMemoryRegion.
WritableSharedMemoryMapping MapAtForTesting(
    subtle::PlatformSharedMemoryRegion* region,
    uint64_t offset,
    size_t size);

WritableSharedMemoryMapping MapForTesting(
    subtle::PlatformSharedMemoryRegion* region);

template <typename SharedMemoryRegionType>
std::pair<SharedMemoryRegionType, WritableSharedMemoryMapping>
CreateMappedRegion(size_t size) {
  SharedMemoryRegionType region = SharedMemoryRegionType::Create(size);
  WritableSharedMemoryMapping mapping = region.Map();
  return {std::move(region), std::move(mapping)};
}

// Template specialization of CreateMappedRegion<>() for
// the ReadOnlySharedMemoryRegion. We need this because
// ReadOnlySharedMemoryRegion::Create() has a different return type.
template <>
std::pair<ReadOnlySharedMemoryRegion, WritableSharedMemoryMapping>
CreateMappedRegion(size_t size);

}  // namespace base

#endif  // BASE_TEST_TEST_SHARED_MEMORY_UTIL_H_
