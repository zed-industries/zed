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

// static_contained_range_map-inl.h: Hierarchically-organized range map,
// i.e., StaticContainedRangeMap implementation.
//
// See static_contained_range_map.h for documentation.
//
// Author: Siyang Xie (lambxsy@google.com)

#ifndef PROCESSOR_STATIC_CONTAINED_RANGE_MAP_INL_H__
#define PROCESSOR_STATIC_CONTAINED_RANGE_MAP_INL_H__

#include "processor/static_contained_range_map.h"
#include "processor/logging.h"

namespace google_breakpad {

template<typename AddressType, typename EntryType>
StaticContainedRangeMap<AddressType, EntryType>::StaticContainedRangeMap(
    const char *base)
    : base_(*(reinterpret_cast<const AddressType*>(base))),
      entry_size_(*(reinterpret_cast<const uint64_t*>(base + sizeof(base_)))),
      entry_ptr_(reinterpret_cast<const EntryType*>(
          base + sizeof(base_) + sizeof(entry_size_))),
      map_(base + sizeof(base_) + sizeof(entry_size_) + entry_size_) {
  if (entry_size_ == 0)
    entry_ptr_ = nullptr;
}


template<typename AddressType, typename EntryType>
bool StaticContainedRangeMap<AddressType, EntryType>::RetrieveRange(
    const AddressType& address, const EntryType*& entry) const {

  // Get an iterator to the child range whose high address is equal to or
  // greater than the supplied address.  If the supplied address is higher
  // than all of the high addresses in the range, then this range does not
  // contain a child at address, so return false.  If the supplied address
  // is lower than the base address of the child range, then it is not within
  // the child range, so return false.
  MapConstIterator iterator = map_.lower_bound(address);

  if (iterator == map_.end())
    return false;

  const char *memory_child =
      reinterpret_cast<const char*>(iterator.GetValuePtr());

  StaticContainedRangeMap child_map(memory_child);

  if (address < child_map.base_)
    return false;

  // The child in iterator->second contains the specified address.  Find out
  // if it has a more-specific descendant that also contains it.  If it does,
  // it will set |entry| appropriately.  If not, set |entry| to the child.
  if (!child_map.RetrieveRange(address, entry))
    entry = child_map.entry_ptr_;

  return true;
}

template <typename AddressType, typename EntryType>
bool StaticContainedRangeMap<AddressType, EntryType>::RetrieveRanges(
    const AddressType& address,
    std::vector<const EntryType*>& entries) const {
  MapConstIterator iterator = map_.lower_bound(address);
  if (iterator == map_.end())
    return false;
  const char* memory_child =
      reinterpret_cast<const char*>(iterator.GetValuePtr());
  StaticContainedRangeMap child_map(memory_child);
  if (address < child_map.base_)
    return false;
  child_map.RetrieveRanges(address, entries);
  entries.push_back(child_map.entry_ptr_);
  return true;
}

}  // namespace google_breakpad

#endif  // PROCESSOR_STATIC_CONTAINED_RANGE_MAP_INL_H__
