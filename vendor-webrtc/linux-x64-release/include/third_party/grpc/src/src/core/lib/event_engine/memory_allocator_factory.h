// Copyright 2023 The gRPC Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_MEMORY_ALLOCATOR_FACTORY_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_MEMORY_ALLOCATOR_FACTORY_H
#include <grpc/event_engine/memory_allocator.h>
#include <grpc/support/port_platform.h>

#include <memory>
#include <utility>

#include "absl/strings/string_view.h"
#include "src/core/lib/resource_quota/memory_quota.h"

namespace grpc_event_engine::experimental {

class MemoryQuotaBasedMemoryAllocatorFactory : public MemoryAllocatorFactory {
 public:
  explicit MemoryQuotaBasedMemoryAllocatorFactory(
      grpc_core::MemoryQuotaRefPtr memory_quota)
      : memory_quota_(std::move(memory_quota)) {}

  MemoryAllocator CreateMemoryAllocator(absl::string_view name) override {
    return memory_quota_->CreateMemoryAllocator(name);
  }

 private:
  grpc_core::MemoryQuotaRefPtr memory_quota_;
};

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_MEMORY_ALLOCATOR_FACTORY_H
