/*
 * Copyright (C) 2020 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_EXT_TRACE_PROCESSOR_IMPORTERS_MEMORY_TRACKER_MEMORY_ALLOCATOR_NODE_ID_H_
#define INCLUDE_PERFETTO_EXT_TRACE_PROCESSOR_IMPORTERS_MEMORY_TRACKER_MEMORY_ALLOCATOR_NODE_ID_H_

#include <stdint.h>

#include <string>

#include "perfetto/base/export.h"

namespace perfetto {
namespace trace_processor {

class PERFETTO_EXPORT_COMPONENT MemoryAllocatorNodeId {
 public:
  MemoryAllocatorNodeId();
  explicit MemoryAllocatorNodeId(uint64_t id);

  uint64_t ToUint64() const { return id_; }

  // Returns a (hex-encoded) string representation of the id.
  std::string ToString() const;

  bool empty() const { return id_ == 0u; }

  bool operator==(const MemoryAllocatorNodeId& other) const {
    return id_ == other.id_;
  }

  bool operator!=(const MemoryAllocatorNodeId& other) const {
    return !(*this == other);
  }

  bool operator<(const MemoryAllocatorNodeId& other) const {
    return id_ < other.id_;
  }

 private:
  uint64_t id_;

  // Deliberately copy-able.
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_TRACE_PROCESSOR_IMPORTERS_MEMORY_TRACKER_MEMORY_ALLOCATOR_NODE_ID_H_
