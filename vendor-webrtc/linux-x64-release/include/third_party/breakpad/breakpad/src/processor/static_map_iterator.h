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

// static_map_iterator.h: StaticMapIterator template class declaration.
//
// StaticMapIterator provides increment and decrement operators to iterate
// through a StaticMap map.  It does not provide *, -> operators, user should
// use GetKeyPtr(), GetKey(), GetValuePtr() interfaces to retrieve data or
// pointer to data.  StaticMapIterator is essentially a const_iterator.
//
// Author: Siyang Xie (lambxsy@google.com)


#ifndef PROCESSOR_STATIC_MAP_ITERATOR_H__
#define PROCESSOR_STATIC_MAP_ITERATOR_H__

#include "google_breakpad/common/breakpad_types.h"

namespace google_breakpad {

// Forward declaration.
template<typename Key, typename Value, typename Compare> class StaticMap;

// StaticMapIterator does not support operator*() or operator->(),
// User should use GetKey(), GetKeyPtr(), GetValuePtr() instead;
template<typename Key, typename Value, typename Compare>
class StaticMapIterator {
 public:
  // Constructors.
  StaticMapIterator(): index_(-1), base_(nullptr) { }

  // Increment & Decrement operators:
  StaticMapIterator& operator++();
  StaticMapIterator operator++(int post_fix_operator);

  StaticMapIterator& operator--();
  StaticMapIterator operator--(int post_fix_operator);

  // Interface for retrieving data / pointer to data.
  const Key* GetKeyPtr() const;

  // Run time error will occur if GetKey() is called on an invalid iterator.
  inline const Key GetKey() const { return *GetKeyPtr(); }

  // return a raw memory pointer that points to the start address of value.
  const char* GetValueRawPtr() const;

  // return a reinterpret-casted pointer to the value.
  inline const Value* GetValuePtr() const {
    return reinterpret_cast<const Value*>(GetValueRawPtr());
  }

  bool operator==(const StaticMapIterator& x) const;
  bool operator!=(const StaticMapIterator& x) const;

  // Check if this iterator is valid.
  // If iterator is invalid, user is forbidden to use ++/-- operator
  // or interfaces for retrieving data / pointer to data.
  bool IsValid() const;

 private:
  friend class StaticMap<Key, Value, Compare>;

  // Only StaticMap can call this constructor.
  explicit StaticMapIterator(const char* base, int64_t index);

  // Index of node that the iterator is pointing to.
  int64_t index_;

  // Beginning address of the serialized map data.
  const char* base_;

  // Number of nodes in the map.  Use it to identify end() iterator.
  int64_t num_nodes_;

  // offsets_ is an array of offset addresses of mapped values.
  // For example:
  // address_of_i-th_node_value = base_ + offsets_[i]
  const uint64_t* offsets_;

  // keys_[i] = key of i_th node.
  const Key* keys_;
};

}  // namespace google_breakpad

#endif  // PROCESSOR_STATIC_MAP_ITERATOR_H__
