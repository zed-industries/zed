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

// static_contained_range_map.h: StaticContainedRangeMap.
//
// StaticContainedRangeMap is similar to ContainedRangeMap.  However,
// StaticContainedRangeMap wraps a StaticMap instead of std::map, and does not
// support dynamic operations like StoreRange(...).
// StaticContainedRangeMap provides same RetrieveRange(...) interfaces as
// ContainedRangeMap.
//
// Please see contained_range_map.h for more documentation.
//
// Author: Siyang Xie (lambxsy@google.com)

#ifndef PROCESSOR_STATIC_CONTAINED_RANGE_MAP_H__
#define PROCESSOR_STATIC_CONTAINED_RANGE_MAP_H__

#include <vector>
#include "processor/static_map-inl.h"

namespace google_breakpad {

template<typename AddressType, typename EntryType>
class StaticContainedRangeMap {
 public:
  StaticContainedRangeMap(): base_(), entry_size_(), entry_ptr_(), map_() { }
  explicit StaticContainedRangeMap(const char *base);

  // Retrieves the most specific (smallest) descendant range encompassing
  // the specified address.  This method will only return entries held by
  // child ranges, and not the entry contained by |this|.  This is necessary
  // to support a sparsely-populated root range.  If no descendant range
  // encompasses the address, returns false.
  bool RetrieveRange(const AddressType& address, const EntryType*& entry) const;

  // Retrieves the vector of entries encompassing the specified address from the
  // innermost entry to the outermost entry.
  bool RetrieveRanges(const AddressType& address,
                      std::vector<const EntryType*>& entry) const;

 private:
  friend class ModuleComparer;
  // AddressToRangeMap stores pointers.  This makes reparenting simpler in
  // StoreRange, because it doesn't need to copy entire objects.
  typedef StaticContainedRangeMap* SelfPtr;
  typedef
  StaticMap<AddressType, StaticContainedRangeMap> AddressToRangeMap;
  typedef typename AddressToRangeMap::const_iterator MapConstIterator;

  // The base address of this range.  The high address does not need to
  // be stored, because it is used as the key to an object in its parent's
  // map, and all ContainedRangeMaps except for the root range are contained
  // within maps.  The root range does not actually contain an entry, so its
  // base_ field is meaningless, and the fact that it has no parent and thus
  // no key is unimportant.  For this reason, the base_ field should only be
  // is accessed on child ContainedRangeMap objects, and never on |this|.
  AddressType base_;

  // The entry corresponding to this range.  The root range does not
  // actually contain an entry, so its entry_ field is meaningless.  For
  // this reason, the entry_ field should only be accessed on child
  // ContainedRangeMap objects, and never on |this|.
  uint64_t entry_size_;
  const EntryType *entry_ptr_;

  // The map containing child ranges, keyed by each child range's high
  // address.  This is a pointer to avoid allocating map structures for
  // leaf nodes, where they are not needed.
  AddressToRangeMap map_;
};

}  // namespace google_breakpad


#endif  // PROCESSOR_STATIC_CONTAINED_RANGE_MAP_H__
