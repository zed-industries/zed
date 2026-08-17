// Copyright 2021 The gRPC Authors
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
#ifndef GRPC_EVENT_ENGINE_INTERNAL_MEMORY_ALLOCATOR_IMPL_H
#define GRPC_EVENT_ENGINE_INTERNAL_MEMORY_ALLOCATOR_IMPL_H

#include <grpc/event_engine/memory_request.h>
#include <grpc/slice.h>
#include <grpc/support/port_platform.h>

#include <algorithm>
#include <memory>
#include <type_traits>
#include <vector>

namespace grpc_event_engine {
namespace experimental {

namespace internal {

/// Underlying memory allocation interface.
/// This is an internal interface, not intended to be used by users.
/// Its interface is subject to change at any time.
class MemoryAllocatorImpl
    : public std::enable_shared_from_this<MemoryAllocatorImpl> {
 public:
  MemoryAllocatorImpl() {}
  virtual ~MemoryAllocatorImpl() {}

  MemoryAllocatorImpl(const MemoryAllocatorImpl&) = delete;
  MemoryAllocatorImpl& operator=(const MemoryAllocatorImpl&) = delete;

  /// Reserve bytes from the quota.
  /// If we enter overcommit, reclamation will begin concurrently.
  /// Returns the number of bytes reserved.
  /// If MemoryRequest is invalid, this function will abort.
  /// If MemoryRequest is valid, this function is infallible, and will always
  /// succeed at reserving the some number of bytes between request.min() and
  /// request.max() inclusively.
  virtual size_t Reserve(MemoryRequest request) = 0;

  /// Allocate a slice, using MemoryRequest to size the number of returned
  /// bytes. For a variable length request, check the returned slice length to
  /// verify how much memory was allocated. Takes care of reserving memory for
  /// any relevant control structures also.
  virtual grpc_slice MakeSlice(MemoryRequest request) = 0;

  /// Release some bytes that were previously reserved.
  /// If more bytes are released than were reserved, we will have undefined
  /// behavior.
  virtual void Release(size_t n) = 0;

  /// Shutdown this allocator.
  /// Further usage of Reserve() is undefined behavior.
  virtual void Shutdown() = 0;
};

}  // namespace internal

}  // namespace experimental
}  // namespace grpc_event_engine

#endif  // GRPC_EVENT_ENGINE_INTERNAL_MEMORY_ALLOCATOR_IMPL_H
