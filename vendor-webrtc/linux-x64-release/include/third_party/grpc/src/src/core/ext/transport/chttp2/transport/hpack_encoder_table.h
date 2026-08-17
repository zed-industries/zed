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

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_HPACK_ENCODER_TABLE_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_HPACK_ENCODER_TABLE_H

#include <grpc/support/port_platform.h>
#include <stddef.h>
#include <stdint.h>

#include <limits>
#include <vector>

#include "src/core/ext/transport/chttp2/transport/hpack_constants.h"

namespace grpc_core {

// Tracks the values available in the remote HPACK header table, and their
// sizes.
class HPackEncoderTable {
 public:
  using EntrySize = uint16_t;

  HPackEncoderTable() : elem_size_(hpack_constants::kInitialTableEntries) {}

  static constexpr size_t MaxEntrySize() {
    return std::numeric_limits<EntrySize>::max();
  }

  // Reserve space in table for the new element, evict entries if needed.
  // Return the new index of the element. Return 0 to indicate not adding to
  // table.
  uint32_t AllocateIndex(size_t element_size);
  // Set the maximum table size. Return true if it changed.
  bool SetMaxSize(uint32_t max_table_size);
  // Get the current max table size
  uint32_t max_size() const { return max_table_size_; }
  // Get the current table size
  uint32_t test_only_table_size() const { return table_size_; }
  // Get the number of entries in the table
  uint32_t test_only_table_elems() const { return table_elems_; }

  // Convert an element index into a dynamic index
  uint32_t DynamicIndex(uint32_t index) const {
    return 1 + hpack_constants::kLastStaticEntry + tail_remote_index_ +
           table_elems_ - index;
  }
  // Check if an element index is convertible to a dynamic index
  // Note that 0 is always not convertible
  bool ConvertibleToDynamicIndex(uint32_t index) const {
    return index > tail_remote_index_;
  }

 private:
  void EvictOne();
  void Rebuild(uint32_t capacity);

  // one before the lowest usable table index
  uint32_t tail_remote_index_ = 0;
  uint32_t max_table_size_ = hpack_constants::kInitialTableSize;
  uint32_t table_elems_ = 0;
  uint32_t table_size_ = 0;
  // The size of each element in the HPACK table.
  std::vector<EntrySize> elem_size_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_HPACK_ENCODER_TABLE_H
