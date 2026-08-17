// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_PLATFORM_SHARED_MEMORY_MAPPER_H_
#define BASE_MEMORY_PLATFORM_SHARED_MEMORY_MAPPER_H_

#include "base/base_export.h"
#include "base/memory/shared_memory_mapper.h"

namespace base {

// Default implementation of the SharedMemoryMapper interface. Implements the
// platform-specific logic for mapping shared memory regions into the virtual
// address space of the process.
class PlatformSharedMemoryMapper : public SharedMemoryMapper {
 public:
  std::optional<span<uint8_t>> Map(subtle::PlatformSharedMemoryHandle handle,
                                   bool write_allowed,
                                   uint64_t offset,
                                   size_t size) override;

  void Unmap(span<uint8_t> mapping) override;
};

}  // namespace base

#endif  // BASE_MEMORY_PLATFORM_SHARED_MEMORY_MAPPER_H_
