// Copyright 2010 Google LLC
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//     * Neither the name of Google LLC nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

// static_map-inl.h: StaticMap implementation.
//
// See static_map.h for documentation.
//
// Author: Siyang Xie (lambxsy@google.com)


#ifndef PROCESSOR_STATIC_MAP_INL_H__
#define PROCESSOR_STATIC_MAP_INL_H__

#include "processor/static_map.h"
#include "processor/static_map_iterator-inl.h"
#include "processor/logging.h"

namespace google_breakpad {

template<typename Key, typename Value, typename Compare>
StaticMap<Key, Value, Compare>::StaticMap(const char* raw_data)
    : raw_data_(raw_data),
      compare_() {
  // First 4 Bytes store the number of nodes.
  num_nodes_ = *(reinterpret_cast<const uint64_t*>(raw_data_));

  offsets_ = reinterpret_cast<const uint64_t*>(
      raw_data_ + sizeof(num_nodes_));

  keys_ = reinterpret_cast<const Key*>(
      raw_data_ + (1 + num_nodes_) * sizeof(uint64_t));
}

// find(), lower_bound() and upper_bound() implement binary search algorithm.
template<typename Key, typename Value, typename Compare>
StaticMapIterator<Key, Value, Compare>
StaticMap<Key, Value, Compare>::find(const Key& key) const {
  int64_t begin = 0;
  int64_t end = num_nodes_;
  int64_t middle;
  int64_t compare_result;
  while (begin < end) {
    middle = begin + (end - begin) / 2;
    compare_result = compare_(key, GetKeyAtIndex(middle));
    if (compare_result == 0)
      return IteratorAtIndex(middle);
    if (compare_result < 0) {
      end = middle;
    } else {
      begin = middle + 1;
    }
  }
  return this->end();
}

template<typename Key, typename Value, typename Compare>
StaticMapIterator<Key, Value, Compare>
StaticMap<Key, Value, Compare>::lower_bound(const Key& key) const {
  int64_t begin = 0;
  int64_t end = num_nodes_;
  int64_t middle;
  int64_t comp_result;
  while (begin < end) {
    middle = begin + (end - begin) / 2;
    comp_result = compare_(key, GetKeyAtIndex(middle));
    if (comp_result == 0)
      return IteratorAtIndex(middle);
    if (comp_result < 0) {
      end = middle;
    } else {
      begin = middle + 1;
    }
  }
  return IteratorAtIndex(begin);
}

template<typename Key, typename Value, typename Compare>
StaticMapIterator<Key, Value, Compare>
StaticMap<Key, Value, Compare>::upper_bound(const Key& key) const {
  int64_t begin = 0;
  int64_t end = num_nodes_;
  int64_t middle;
  int64_t compare_result;
  while (begin < end) {
    middle = begin + (end - begin) / 2;
    compare_result = compare_(key, GetKeyAtIndex(middle));
    if (compare_result == 0)
      return IteratorAtIndex(middle + 1);
    if (compare_result < 0) {
      end = middle;
    } else {
      begin = middle + 1;
    }
  }
  return IteratorAtIndex(begin);
}

template<typename Key, typename Value, typename Compare>
bool StaticMap<Key, Value, Compare>::ValidateInMemoryStructure() const {
  // check the number of nodes is non-negative:
  if (!raw_data_) return false;
  int64_t num_nodes = *(reinterpret_cast<const int64_t*>(raw_data_));
  if (num_nodes < 0) {
    BPLOG(INFO) << "StaticMap check failed: negative number of nodes";
    return false;
  }

  int64_t node_index = 0;
  if (num_nodes_) {
    uint64_t first_offset = sizeof(int64_t) * (num_nodes_ + 1)
                           + sizeof(Key) * num_nodes_;
    // Num_nodes_ is too large.
    if (first_offset > 0xffffffffUL) {
      BPLOG(INFO) << "StaticMap check failed: size exceeds limit";
      return false;
    }
    if (offsets_[node_index] != static_cast<uint64_t>(first_offset)) {
      BPLOG(INFO) << "StaticMap check failed: first node offset is incorrect";
      return false;
    }
  }

  for (node_index = 1; node_index < num_nodes_; ++node_index) {
    // Check offsets[i] is strictly increasing:
    if (offsets_[node_index] <= offsets_[node_index - 1]) {
      BPLOG(INFO) << "StaticMap check failed: node offsets non-increasing";
      return false;
    }
    // Check Key[i] is strictly increasing as no duplicate keys are allowed.
    if (compare_(GetKeyAtIndex(node_index),
                 GetKeyAtIndex(node_index - 1)) <= 0) {
      BPLOG(INFO) << "StaticMap check failed: node keys non-increasing";
      return false;
    }
  }
  return true;
}

template<typename Key, typename Value, typename Compare>
const Key StaticMap<Key, Value, Compare>::GetKeyAtIndex(int64_t index) const {
  if (index < 0 || index >= num_nodes_) {
    BPLOG(ERROR) << "Key index out of range error";
    // Key type is required to be primitive type.  Return 0 if index is invalid.
    return 0;
  }
  return keys_[index];
}

}  // namespace google_breakpad

#endif  // PROCESSOR_STATIC_MAP_INL_H__
