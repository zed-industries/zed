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

// static_map_iterator-inl.h: StaticMapIterator implementation.
//
// See static_map_iterator.h for documentation.
//
// Author: Siyang Xie (lambxsy@google.com)

#ifndef PROCESSOR_STATIC_MAP_ITERATOR_INL_H__
#define PROCESSOR_STATIC_MAP_ITERATOR_INL_H__

#include "processor/static_map_iterator.h"

#include "processor/logging.h"

namespace google_breakpad {

template<typename Key, typename Value, typename Compare>
StaticMapIterator<Key, Value, Compare>::StaticMapIterator(const char* base,
                                                          int64_t index):
      index_(index), base_(base) {
  // See static_map.h for documentation on
  // bytes format of serialized StaticMap data.
  num_nodes_ = *(reinterpret_cast<const int64_t*>(base_));
  offsets_ = reinterpret_cast<const uint64_t*>(base_ + sizeof(num_nodes_));
  keys_ = reinterpret_cast<const Key*>(
      base_ + (1 + num_nodes_) * sizeof(num_nodes_));
}

// Increment & Decrement operators:
template<typename Key, typename Value, typename Compare>
StaticMapIterator<Key, Value, Compare>&
StaticMapIterator<Key, Value, Compare>::operator++() {
  if (!IsValid()) {
    BPLOG(ERROR) << "operator++ on invalid iterator";
    return *this;
  }
  if (++index_ > num_nodes_) index_ = num_nodes_;
  return *this;
}

template<typename Key, typename Value, typename Compare>
StaticMapIterator<Key, Value, Compare>
StaticMapIterator<Key, Value, Compare>::operator++(int postfix_operator) {
  if (!IsValid()) {
    BPLOG(ERROR) << "operator++ on invalid iterator";
    return *this;
  }
  StaticMapIterator<Key, Value, Compare> tmp = *this;
  if (++index_ > num_nodes_) index_ = num_nodes_;
  return tmp;
}

template<typename Key, typename Value, typename Compare>
StaticMapIterator<Key, Value, Compare>&
StaticMapIterator<Key, Value, Compare>::operator--() {
  if (!IsValid()) {
    BPLOG(ERROR) << "operator++ on invalid iterator";
    return *this;
  }

  if (--index_ < 0) index_ = 0;
  return *this;
}

template<typename Key, typename Value, typename Compare>
StaticMapIterator<Key, Value, Compare>
StaticMapIterator<Key, Value, Compare>::operator--(int postfix_operator) {
  if (!IsValid()) {
    BPLOG(ERROR) << "operator++ on invalid iterator";
    return *this;
  }
  StaticMapIterator<Key, Value, Compare> tmp = *this;

  if (--index_ < 0) index_ = 0;
  return tmp;
}

template<typename Key, typename Value, typename Compare>
const Key* StaticMapIterator<Key, Value, Compare>::GetKeyPtr() const {
  if (!IsValid()) {
    BPLOG(ERROR) << "call GetKeyPtr() on invalid iterator";
    return nullptr;
  }
  return &(keys_[index_]);
}

template<typename Key, typename Value, typename Compare>
const char* StaticMapIterator<Key, Value, Compare>::GetValueRawPtr() const {
  if (!IsValid()) {
    BPLOG(ERROR) << "call GetValuePtr() on invalid iterator";
    return nullptr;
  }
  return base_ + offsets_[index_];
}

template<typename Key, typename Value, typename Compare>
bool StaticMapIterator<Key, Value, Compare>::operator==(
    const StaticMapIterator<Key, Value, Compare>& x) const {
  return base_ == x.base_ && index_ == x.index_;
}

template<typename Key, typename Value, typename Compare>
bool StaticMapIterator<Key, Value, Compare>::operator!=(
    const StaticMapIterator<Key, Value, Compare>& x) const {
  // Only need to compare base_ and index_.
  // Other data members are auxiliary.
  return base_ != x.base_ || index_ != x.index_;
}

template<typename Key, typename Value, typename Compare>
bool StaticMapIterator<Key, Value, Compare>::IsValid() const {
  if (!base_ || index_ < 0 || index_ > num_nodes_)
    return false;

  return true;
}

}  // namespace google_breakpad

#endif  // PROCESSOR_STATIC_MAP_ITERATOR_INL_H__
