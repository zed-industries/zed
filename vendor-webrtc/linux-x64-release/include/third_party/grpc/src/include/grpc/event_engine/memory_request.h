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
#ifndef GRPC_EVENT_ENGINE_MEMORY_REQUEST_H
#define GRPC_EVENT_ENGINE_MEMORY_REQUEST_H

#include <grpc/support/port_platform.h>
#include <stddef.h>

#include "absl/strings/string_view.h"

namespace grpc_event_engine {
namespace experimental {

/// Reservation request - how much memory do we want to allocate?
class MemoryRequest {
 public:
  /// Request a fixed amount of memory.
  // NOLINTNEXTLINE(google-explicit-constructor)
  MemoryRequest(size_t n) : min_(n), max_(n) {}
  /// Request a range of memory.
  /// Requires: \a min <= \a max.
  /// Requires: \a max <= max_size()
  MemoryRequest(size_t min, size_t max) : min_(min), max_(max) {}

  /// Maximum allowable request size - hard coded to 1GB.
  static constexpr size_t max_allowed_size() { return 1024 * 1024 * 1024; }

  /// Increase the size by \a amount.
  /// Undefined behavior if min() + amount or max() + amount overflows.
  MemoryRequest Increase(size_t amount) const {
    return MemoryRequest(min_ + amount, max_ + amount);
  }

  size_t min() const { return min_; }
  size_t max() const { return max_; }

  bool operator==(const MemoryRequest& other) const {
    return min_ == other.min_ && max_ == other.max_;
  }
  bool operator!=(const MemoryRequest& other) const {
    return !(*this == other);
  }

  template <typename Sink>
  friend void AbslStringify(Sink& s, const MemoryRequest& r) {
    if (r.min_ == r.max_) {
      s.Append(std::to_string(r.min_));
    } else {
      s.Append(std::to_string(r.min_));
      s.Append("..");
      s.Append(std::to_string(r.max_));
    }
  }

 private:
  size_t min_;
  size_t max_;
};

}  // namespace experimental
}  // namespace grpc_event_engine

#endif  // GRPC_EVENT_ENGINE_MEMORY_REQUEST_H
