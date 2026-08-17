/*
 * Copyright (C) 2023 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACING_CORE_IN_PROCESS_SHARED_MEMORY_H_
#define SRC_TRACING_CORE_IN_PROCESS_SHARED_MEMORY_H_

#include <memory>

#include "perfetto/ext/base/paged_memory.h"
#include "perfetto/ext/tracing/core/shared_memory.h"

namespace perfetto {

// An implementation of the ShareMemory interface that allocates memory that can
// only be shared intra-process.
class InProcessSharedMemory : public SharedMemory {
 public:
  static constexpr size_t kDefaultSize = 128 * 1024;
  static constexpr size_t kShmemEmulationSize = 1024 * 1024;

  // Default ctor used for intra-process shmem between a producer and the
  // service.
  explicit InProcessSharedMemory(size_t size)
      : mem_(base::PagedMemory::Allocate(size)) {}
  ~InProcessSharedMemory() override;

  static std::unique_ptr<InProcessSharedMemory> Create(
      size_t size = kDefaultSize) {
    return std::make_unique<InProcessSharedMemory>(size);
  }

  // SharedMemory implementation.
  using SharedMemory::start;  // Equal priority to const and non-const versions
  const void* start() const override;
  size_t size() const override;

  class Factory : public SharedMemory::Factory {
   public:
    ~Factory() override;
    std::unique_ptr<SharedMemory> CreateSharedMemory(size_t size) override {
      return InProcessSharedMemory::Create(size);
    }
  };

 private:
  base::PagedMemory mem_;
};

}  // namespace perfetto

#endif  // SRC_TRACING_CORE_IN_PROCESS_SHARED_MEMORY_H_
