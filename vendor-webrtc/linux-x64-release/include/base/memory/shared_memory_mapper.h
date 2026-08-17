// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_SHARED_MEMORY_MAPPER_H_
#define BASE_MEMORY_SHARED_MEMORY_MAPPER_H_

#include <stdint.h>

#include <optional>

#include "base/base_export.h"
#include "base/containers/span.h"
#include "base/memory/platform_shared_memory_handle.h"

namespace base {

// Interface to implement mapping and unmapping of shared memory regions into
// the virtual address space. The default implementation,
// |PlatformSharedMemoryMapper| uses the platform-specific APIs to map the
// region anywhere in the address space. Other implementations can be used for
// example to always map the regions into an existing address space reservation.
// Implementations of this interface should generally be statically allocated
// as SharedMemoryMappings keep a reference to their mapper.
class BASE_EXPORT SharedMemoryMapper {
 public:
  // Returns the default shared memory mapper.
  static SharedMemoryMapper* GetDefaultInstance();

  // Maps the shared memory region identified through the provided platform
  // handle into the caller's address space.
  virtual std::optional<span<uint8_t>> Map(
      subtle::PlatformSharedMemoryHandle handle,
      bool write_allowed,
      uint64_t offset,
      size_t size) = 0;

  // Unmaps the specified region of shared memory from the caller's address
  // space.
  virtual void Unmap(span<uint8_t> mapping) = 0;
};

}  // namespace base

#endif  // BASE_MEMORY_SHARED_MEMORY_MAPPER_H_
