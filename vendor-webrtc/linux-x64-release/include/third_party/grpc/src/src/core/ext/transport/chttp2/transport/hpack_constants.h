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

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_HPACK_CONSTANTS_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_HPACK_CONSTANTS_H

#include <grpc/support/port_platform.h>

#include <cstddef>
#include <cstdint>

namespace grpc_core {
namespace hpack_constants {
// Per entry overhead bytes as per the spec
static constexpr uint32_t kEntryOverhead = 32;
// Initial table size as per the spec
static constexpr uint32_t kInitialTableSize = 4096;

// last index in the static table
static constexpr uint32_t kLastStaticEntry = 61;

static constexpr uint32_t EntriesForBytes(uint32_t bytes) noexcept {
  return (bytes + kEntryOverhead - 1) / kEntryOverhead;
}

static constexpr size_t SizeForEntry(size_t key_length,
                                     size_t value_length) noexcept {
  return key_length + value_length + kEntryOverhead;
}

static constexpr uint32_t kInitialTableEntries =
    EntriesForBytes(kInitialTableSize);
}  // namespace hpack_constants
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_HPACK_CONSTANTS_H
