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
//
// static_range_map.h: StaticRangeMap.
//
// StaticRangeMap is similar as RangeMap.  However, StaticRangeMap wraps a
// StaticMap instead of std::map, and does not support dynamic operations like
// StoreRange(...).  StaticRangeMap provides same Retrieve*() interfaces as
// RangeMap.  Please see range_map.h for more documentation.
//
// Author: Siyang Xie (lambxsy@google.com)

#ifndef PROCESSOR_STATIC_RANGE_MAP_H__
#define PROCESSOR_STATIC_RANGE_MAP_H__


#include "processor/static_map-inl.h"

namespace google_breakpad {

// AddressType is basic type, e.g.: integer types, pointers etc
// EntryType could be a complex type, so we retrieve its pointer instead.
template<typename AddressType, typename EntryType>
class StaticRangeMap {
 public:
  StaticRangeMap(): map_() { }
  explicit StaticRangeMap(const char* memory): map_(memory) { }

  // Locates the range encompassing the supplied address.  If there is
  // no such range, returns false.  entry_base and entry_size, if non-NULL,
  // are set to the base and size of the entry's range.
  bool RetrieveRange(const AddressType& address, const EntryType*& entry,
                     AddressType* entry_base, AddressType* entry_size) const;

  // Locates the range encompassing the supplied address, if one exists.
  // If no range encompasses the supplied address, locates the nearest range
  // to the supplied address that is lower than the address.  Returns false
  // if no range meets these criteria.  entry_base and entry_size, if
  // non-NULL, are set to the base and size of the entry's range.
  bool RetrieveNearestRange(const AddressType& address, const EntryType*& entry,
                            AddressType* entry_base, AddressType* entry_size)
                            const;

  // Treating all ranges as a list ordered by the address spaces that they
  // occupy, locates the range at the index specified by index.  Returns
  // false if index is larger than the number of ranges stored.  entry_base
  // and entry_size, if non-NULL, are set to the base and size of the entry's
  // range.
  //
  // RetrieveRangeAtIndex is not optimized for speedy operation.
  bool RetrieveRangeAtIndex(int64_t index, const EntryType*& entry,
                            AddressType* entry_base, AddressType* entry_size)
                            const;

  // Returns the number of ranges stored in the RangeMap.
  inline int64_t GetCount() const { return map_.size(); }

 private:
  friend class ModuleComparer;
  class Range {
   public:
    AddressType base() const {
      return *(reinterpret_cast<const AddressType*>(this));
    }
    const EntryType* entryptr() const {
      return reinterpret_cast<const EntryType*>(this + sizeof(AddressType));
    }
  };

  // Convenience types.
  typedef StaticRangeMap* SelfPtr;
  typedef StaticMap<AddressType, Range> AddressToRangeMap;
  typedef typename AddressToRangeMap::const_iterator MapConstIterator;

  AddressToRangeMap map_;
};

}  // namespace google_breakpad

#endif  // PROCESSOR_STATIC_RANGE_MAP_H__
