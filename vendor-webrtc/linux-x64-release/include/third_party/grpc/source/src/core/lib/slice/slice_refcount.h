// Copyright 2021 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_SLICE_SLICE_REFCOUNT_H
#define GRPC_SRC_CORE_LIB_SLICE_SLICE_REFCOUNT_H

#include <grpc/support/port_platform.h>
#include <inttypes.h>
#include <stddef.h>

#include <atomic>

#include "src/core/lib/debug/trace.h"
#include "src/core/util/debug_location.h"

// grpc_slice_refcount : A reference count for grpc_slice.
struct grpc_slice_refcount {
 public:
  typedef void (*DestroyerFn)(grpc_slice_refcount*);

  static grpc_slice_refcount* NoopRefcount() {
    return reinterpret_cast<grpc_slice_refcount*>(1);
  }

  grpc_slice_refcount() = default;

  // Regular constructor for grpc_slice_refcount.
  //
  // Parameters:
  //  1. DestroyerFn destroyer_fn
  //  Called when the refcount goes to 0, with 'this' as parameter.
  explicit grpc_slice_refcount(DestroyerFn destroyer_fn)
      : destroyer_fn_(destroyer_fn) {}

  void Ref(grpc_core::DebugLocation location) {
    auto prev_refs = ref_.fetch_add(1, std::memory_order_relaxed);
    GRPC_TRACE_LOG(slice_refcount, INFO)
            .AtLocation(location.file(), location.line())
        << "REF " << this << " " << prev_refs << "->" << prev_refs + 1;
  }
  void Unref(grpc_core::DebugLocation location) {
    auto prev_refs = ref_.fetch_sub(1, std::memory_order_acq_rel);
    GRPC_TRACE_LOG(slice_refcount, INFO)
            .AtLocation(location.file(), location.line())
        << "UNREF " << this << " " << prev_refs << "->" << prev_refs - 1;
    if (prev_refs == 1) {
      destroyer_fn_(this);
    }
  }

  // Is this the only instance?
  // For this to be useful the caller needs to ensure that if this is the only
  // instance, no other instance could be created during this call.
  bool IsUnique() const { return ref_.load(std::memory_order_relaxed) == 1; }

 private:
  std::atomic<size_t> ref_{1};
  DestroyerFn destroyer_fn_ = nullptr;
};

#endif  // GRPC_SRC_CORE_LIB_SLICE_SLICE_REFCOUNT_H
