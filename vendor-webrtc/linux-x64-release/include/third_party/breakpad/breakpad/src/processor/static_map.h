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

// static_map.h: StaticMap.
//
// StaticMap provides lookup interfaces and iterators similar as stl::map's.
// These lookup operations are purely Read-Only, thus memory
// allocation & deallocation is mostly avoided (intentionally).
//
// The chunk of memory should contain data with pre-defined pattern:
// **************** header ***************
// int64  (8 bytes): number of nodes
// uint64 (8 bytes): address offset of node1's mapped_value
// uint64 (8 bytes): address offset of node2's mapped_value
// ...
// uint64 (8 bytes): address offset of nodeN's mapped_value
//
// ************* Key array ************
// (X bytes): node1's key
// (X bytes): node2's key
// ...
// (X bytes): nodeN's key
//
// ************* Value array **********
// (? bytes): node1's mapped_value
// (? bytes): node2's mapped_value
// ...
// (? bytes): nodeN's mapped_value
//
// REQUIREMENT: Key type MUST be primitive type or pointers so that:
// X = sizeof(typename Key);

// Author: Siyang Xie (lambxsy@google.com)


#ifndef PROCESSOR_STATIC_MAP_H__
#define PROCESSOR_STATIC_MAP_H__

#include "processor/static_map_iterator-inl.h"

namespace google_breakpad {

// Default functor to compare keys.
template<typename Key>
class DefaultCompare {
 public:
  int64_t operator()(const Key& k1, const Key& k2) const {
    if (k1 < k2) return -1;
    if (k1 == k2) return 0;
    return 1;
  }
};

template<typename Key, typename Value, typename Compare = DefaultCompare<Key> >
class StaticMap {
 public:
  typedef StaticMapIterator<Key, Value, Compare> iterator;
  typedef StaticMapIterator<Key, Value, Compare> const_iterator;

  StaticMap() : raw_data_(0),
                num_nodes_(0),
                offsets_(0),
                compare_() { }

  explicit StaticMap(const char* raw_data);

  inline bool empty() const { return num_nodes_ == 0; }
  inline uint64_t size() const { return num_nodes_; }

  // Return iterators.
  inline iterator begin() const { return IteratorAtIndex(0); }
  inline iterator last() const { return IteratorAtIndex(num_nodes_ - 1); }
  inline iterator end() const { return IteratorAtIndex(num_nodes_); }
  inline iterator IteratorAtIndex(int64_t index) const {
    return iterator(raw_data_, index);
  }

  // Lookup operations.
  iterator find(const Key& k) const;

  // lower_bound(k) searches in a sorted range for the first element that has a
  // key not less than the argument k.
  iterator lower_bound(const Key& k) const;

  // upper_bound(k) searches in a sorted range for the first element that has a
  // key greater than the argument k.
  iterator upper_bound(const Key& k) const;

  // Checks if the underlying memory data conforms to the predefined pattern:
  // first check the number of nodes is non-negative,
  // then check both offsets and keys are strictly increasing (sorted).
  bool ValidateInMemoryStructure() const;

 private:
  const Key GetKeyAtIndex(int64_t i) const;

  // Start address of a raw memory chunk with serialized data.
  const char* raw_data_;

  // Number of nodes in the static map.
  int64_t num_nodes_;

  // Array of offset addresses for stored values.
  // For example:
  // address_of_i-th_node_value = raw_data_ + offsets_[i]
  const uint64_t* offsets_;

  // keys_[i] = key of i_th node
  const Key* keys_;

  Compare compare_;
};

}  // namespace google_breakpad

#endif  // PROCESSOR_STATIC_MAP_H__
