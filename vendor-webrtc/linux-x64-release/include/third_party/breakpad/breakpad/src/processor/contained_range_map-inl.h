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

// contained_range_map-inl.h: Hierarchically-organized range map implementation.
//
// See contained_range_map.h for documentation.
//
// Author: Mark Mentovai

#ifndef PROCESSOR_CONTAINED_RANGE_MAP_INL_H__
#define PROCESSOR_CONTAINED_RANGE_MAP_INL_H__

#include "processor/contained_range_map.h"

#include <assert.h>

#include "processor/logging.h"


namespace google_breakpad {


template<typename AddressType, typename EntryType>
ContainedRangeMap<AddressType, EntryType>::~ContainedRangeMap() {
  // Clear frees the children pointed to by the map, and frees the map itself.
  Clear();
}


template<typename AddressType, typename EntryType>
bool ContainedRangeMap<AddressType, EntryType>::StoreRange(
    const AddressType& base, const AddressType& size, const EntryType& entry) {
  AddressType high = base + size - 1;

  // Check for undersize or overflow.
  if (size <= 0 || high < base) {
    //TODO(nealsid) We are commenting this out in order to prevent
    // excessive logging.  We plan to move to better logging as this
    // failure happens quite often and is expected(see comment in
    // basic_source_line_resolver.cc:671).
    // BPLOG(INFO) << "StoreRange failed, " << HexString(base) << "+"
    // << HexString(size) << ", " << HexString(high);
    return false;
  }

  if (!map_)
    map_ = new AddressToRangeMap();

  MapIterator iterator_base = map_->lower_bound(base);
  MapIterator iterator_high = map_->lower_bound(high);
  MapIterator iterator_end = map_->end();

  if (iterator_base == iterator_high && iterator_base != iterator_end &&
      base >= iterator_base->second->base_) {
    // The new range is entirely within an existing child range.

    // If the new range's geometry is exactly equal to an existing child
    // range's, it violates the containment rules, and an attempt to store
    // it must fail.  iterator_base->first contains the key, which was the
    // containing child's high address.
    if (!allow_equal_range_ && iterator_base->second->base_ == base &&
        iterator_base->first == high) {
      // TODO(nealsid): See the TODO above on why this is commented out.
      //       BPLOG(INFO) << "StoreRange failed, identical range is already "
      //                      "present: " << HexString(base) << "+" <<
      //                      HexString(size);
      return false;
    }

    // Pass the new range on to the child to attempt to store.
    return iterator_base->second->StoreRange(base, size, entry);
  }

  // iterator_high might refer to an irrelevant range: one whose base address
  // is higher than the new range's high address.  Set contains_high to true
  // only if iterator_high refers to a range that is at least partially
  // within the new range.
  bool contains_high = iterator_high != iterator_end &&
                       high >= iterator_high->second->base_;

  // If the new range encompasses any existing child ranges, it must do so
  // fully.  Partial containment isn't allowed.
  if ((iterator_base != iterator_end && base > iterator_base->second->base_) ||
      (contains_high && high < iterator_high->first)) {
    // TODO(mmentovai): Some symbol files will trip this check frequently
    // on STACK lines.  Too many messages will be produced.  These are more
    // suitable for a DEBUG channel than an INFO channel.
    // BPLOG(INFO) << "StoreRange failed, new range partially contains "
    //               "existing range: " << HexString(base) << "+" <<
    //               HexString(size);
    return false;
  }

  // When copying and erasing contained ranges, the "end" iterator needs to
  // point one past the last item of the range to copy.  If contains_high is
  // false, the iterator's already in the right place; the increment is safe
  // because contains_high can't be true if iterator_high == iterator_end.
  if (contains_high)
    ++iterator_high;

  // Optimization: if the iterators are equal, no child ranges would be
  // moved.  Create the new child range with a NULL map to conserve space
  // in leaf nodes, of which there will be many.
  AddressToRangeMap* child_map = nullptr;

  if (iterator_base != iterator_high) {
    // The children of this range that are contained by the new range must
    // be transferred over to the new range.  Create the new child range map
    // and copy the pointers to range maps it should contain into it.
    child_map = new AddressToRangeMap(iterator_base, iterator_high);

    // Remove the copied child pointers from this range's map of children.
    map_->erase(iterator_base, iterator_high);
  }

  // Store the new range in the map by its high address.  Any children that
  // the new child range contains were formerly children of this range but
  // are now this range's grandchildren.  Ownership of these is transferred
  // to the new child range.
  ContainedRangeMap* new_child =
      new ContainedRangeMap(base, entry, child_map, allow_equal_range_);

  map_->insert(MapValue(high, new_child));
  return true;
}


template<typename AddressType, typename EntryType>
bool ContainedRangeMap<AddressType, EntryType>::RetrieveRange(
    const AddressType& address, EntryType* entry) const {
  BPLOG_IF(ERROR, !entry) << "ContainedRangeMap::RetrieveRange requires "
                             "|entry|";
  assert(entry);

  // If nothing was ever stored, then there's nothing to retrieve.
  if (!map_)
    return false;

  // Get an iterator to the child range whose high address is equal to or
  // greater than the supplied address.  If the supplied address is higher
  // than all of the high addresses in the range, then this range does not
  // contain a child at address, so return false.  If the supplied address
  // is lower than the base address of the child range, then it is not within
  // the child range, so return false.
  MapConstIterator iterator = map_->lower_bound(address);
  if (iterator == map_->end() || address < iterator->second->base_)
    return false;

  // The child in iterator->second contains the specified address.  Find out
  // if it has a more-specific descendant that also contains it.  If it does,
  // it will set |entry| appropriately.  If not, set |entry| to the child.
  if (!iterator->second->RetrieveRange(address, entry))
    *entry = iterator->second->entry_;

  return true;
}

template <typename AddressType, typename EntryType>
bool ContainedRangeMap<AddressType, EntryType>::RetrieveRanges(
    const AddressType& address,
    std::vector<const EntryType*>& entries) const {
  // If nothing was ever stored, then there's nothing to retrieve.
  if (!map_)
    return false;
  MapIterator iterator = map_->lower_bound(address);
  if (iterator == map_->end() || address < iterator->second->base_)
    return false;
  iterator->second->RetrieveRanges(address, entries);
  entries.push_back(&iterator->second->entry_);
  return true;
}

template<typename AddressType, typename EntryType>
void ContainedRangeMap<AddressType, EntryType>::Clear() {
  if (map_) {
    MapConstIterator end = map_->end();
    for (MapConstIterator child = map_->begin(); child != end; ++child)
      delete child->second;

    delete map_;
    map_ = nullptr;
  }
}


}  // namespace google_breakpad


#endif  // PROCESSOR_CONTAINED_RANGE_MAP_INL_H__
