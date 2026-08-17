// Copyright 2006 Google LLC
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

// range_map.h: Range maps.
//
// A range map associates a range of addresses with a specific object.  This
// is useful when certain objects of variable size are located within an
// address space.  The range map makes it simple to determine which object is
// associated with a specific address, which may be any address within the
// range associated with an object.
//
// Author: Mark Mentovai

#ifndef PROCESSOR_RANGE_MAP_H__
#define PROCESSOR_RANGE_MAP_H__


#include <stdint.h>
#include <map>


namespace google_breakpad {

// Forward declarations (for later friend declarations of specialized template).
template<class, class> class RangeMapSerializer;

// Determines what happens when two ranges overlap.
enum class MergeRangeStrategy {
  // When two ranges overlap, the new range fails to be inserted. The default
  // strategy.
  kExclusiveRanges,

  // The range with the lower base address will be truncated such that it's
  // high address is one less than the range above it.
  kTruncateLower,

  // The range with the greater high address has its range truncated such that
  // its base address is one higher than the range below it.
  kTruncateUpper
};

template<typename AddressType, typename EntryType>
class RangeMap {
 public:
  RangeMap() : merge_strategy_(MergeRangeStrategy::kExclusiveRanges), map_() {}

  void SetMergeStrategy(MergeRangeStrategy strat) { merge_strategy_ = strat; }

  MergeRangeStrategy GetMergeStrategy() const { return merge_strategy_; }

  // Inserts a range into the map.  Returns false for a parameter error,
  // or if the location of the range would conflict with a range already
  // stored in the map.  If enable_shrink_down is true and there is an overlap
  // between the current range and some other range (already in the map),
  // shrink down the range which ends at a higher address.
  bool StoreRange(const AddressType& base, const AddressType& size,
                  const EntryType& entry);

  // Locates the range encompassing the supplied address.  If there is no such
  // range, returns false.  entry_base, entry_delta, and entry_size, if
  // non-NULL, are set to the base, delta, and size of the entry's range.
  // A positive entry delta (> 0) indicates that there was an overlap and the
  // entry was shrunk down (original start address was increased by delta).
  bool RetrieveRange(const AddressType& address, EntryType* entry,
                     AddressType* entry_base, AddressType* entry_delta,
                     AddressType* entry_size) const;

  // Locates the range encompassing the supplied address, if one exists.
  // If no range encompasses the supplied address, locates the nearest range
  // to the supplied address that is lower than the address.  Returns false
  // if no range meets these criteria.  entry_base, entry_delta, and entry_size,
  // if non-NULL, are set to the base, delta, and size of the entry's range.
  // A positive entry delta (> 0) indicates that there was an overlap and the
  // entry was shrunk down (original start address was increased by delta).
  bool RetrieveNearestRange(const AddressType& address, EntryType* entry,
                            AddressType* entry_base, AddressType* entry_delta,
                            AddressType* entry_size) const;

  // Treating all ranges as a list ordered by the address spaces that they
  // occupy, locates the range at the index specified by index.  Returns
  // false if index is larger than the number of ranges stored.  entry_base,
  // entry_delta, and entry_size, if non-NULL, are set to the base, delta, and
  // size of the entry's range.
  // A positive entry delta (> 0) indicates that there was an overlap and the
  // entry was shrunk down (original start address was increased by delta).
  //
  // RetrieveRangeAtIndex is not optimized for speedy operation.
  bool RetrieveRangeAtIndex(int64_t index, EntryType* entry,
                            AddressType* entry_base, AddressType* entry_delta,
                            AddressType* entry_size) const;

  // Returns the number of ranges stored in the RangeMap.
  int64_t GetCount() const;

  // Empties the range map, restoring it to the state it was when it was
  // initially created.
  void Clear();

 private:
  // Friend declarations.
  friend class ModuleComparer;
  friend class RangeMapSerializer<AddressType, EntryType>;

  // Same a StoreRange() with the only exception that the |delta| can be
  // passed in.
  bool StoreRangeInternal(const AddressType& base, const AddressType& delta,
                          const AddressType& size, const EntryType& entry);

  class Range {
   public:
    Range(const AddressType& base, const AddressType& delta,
          const EntryType& entry)
        : base_(base), delta_(delta), entry_(entry) {}

    AddressType base() const { return base_; }
    AddressType delta() const { return delta_; }
    EntryType entry() const { return entry_; }

   private:
    // The base address of the range.  The high address does not need to
    // be stored, because RangeMap uses it as the key to the map.
    const AddressType base_;

    // The delta when the range is shrunk down.
    const AddressType delta_;

    // The entry corresponding to a range.
    const EntryType entry_;
  };

  // Convenience types.
  typedef std::map<AddressType, Range> AddressToRangeMap;
  typedef typename AddressToRangeMap::const_iterator MapConstIterator;
  typedef typename AddressToRangeMap::value_type MapValue;

  MergeRangeStrategy merge_strategy_;

  // Maps the high address of each range to a EntryType.
  AddressToRangeMap map_;
};


}  // namespace google_breakpad


#endif  // PROCESSOR_RANGE_MAP_H__
