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

// range_map-inl.h: Range map implementation.
//
// See range_map.h for documentation.
//
// Author: Mark Mentovai

#ifndef PROCESSOR_RANGE_MAP_INL_H__
#define PROCESSOR_RANGE_MAP_INL_H__


#include <assert.h>

#include "common/safe_math.h"
#include "processor/range_map.h"
#include "processor/linked_ptr.h"
#include "processor/logging.h"


namespace google_breakpad {

template<typename AddressType, typename EntryType>
bool RangeMap<AddressType, EntryType>::StoreRange(const AddressType& base,
                                                  const AddressType& size,
                                                  const EntryType& entry) {
  return StoreRangeInternal(base, 0 /* delta */, size, entry);
}

template<typename AddressType, typename EntryType>
bool RangeMap<AddressType, EntryType>::StoreRangeInternal(
    const AddressType& base, const AddressType& delta,
    const AddressType& size, const EntryType& entry) {
  AddressType high = AddressType();
  bool high_ok = false;
  if (size > 0) {
    std::pair<AddressType, bool> result = AddWithOverflowCheck(base, size - 1);
    high = result.first;
    high_ok = !result.second;
  }
  // Check for undersize or overflow.
  if (!high_ok) {
    // The processor will hit this case too frequently with common symbol
    // files in the size == 0 case, which is more suited to a DEBUG channel.
    // Filter those out since there's no DEBUG channel at the moment.
    BPLOG_IF(INFO, size != 0) << "StoreRangeInternal failed, "
                              << HexString(base) << "+" << HexString(size)
                              << ", " << HexString(high)
                              << ", delta: " << HexString(delta);
    return false;
  }

  // Ensure that this range does not overlap with another one already in the
  // map.
  MapConstIterator iterator_base = map_.lower_bound(base);
  MapConstIterator iterator_high = map_.lower_bound(high);

  if (iterator_base != iterator_high) {
    // Some other range ends in the space used by this range.  It may be
    // contained within the space used by this range, or it may extend lower.
    if (merge_strategy_ == MergeRangeStrategy::kTruncateLower) {
      // kTruncate the range with the lower base address.
      AddressType other_base = iterator_base->second.base();
      if (base < other_base) {
        return StoreRangeInternal(base, delta, other_base - base, entry);
      } else if (other_base < base) {
        EntryType other_entry;
        AddressType other_high, other_size, other_delta;
        other_high = iterator_base->first;
        RetrieveRange(other_high, &other_entry, &other_base, &other_delta,
                      &other_size);
        map_.erase(iterator_base);
        map_.insert(
            MapValue(base - 1, Range(other_base, other_delta, other_entry)));
        return StoreRangeInternal(base, delta, size, entry);
      } else {
        return false;
      }
    } else if (merge_strategy_ == MergeRangeStrategy::kTruncateUpper) {
      // Truncate the lower portion of this range.
      AddressType additional_delta = iterator_base->first - base + 1;
      return StoreRangeInternal(base + additional_delta,
                                delta + additional_delta,
                                size - additional_delta, entry);
    } else {
      // The processor hits this case too frequently with common symbol files. 
      // This is most appropriate for a DEBUG channel, but since none exists
      // now simply comment out this logging.
      // AddressType other_base = iterator_base->second.base();
      // AddressType other_size = iterator_base->first - other_base + 1;
      // BPLOG(INFO) << "StoreRangeInternal failed, an existing range is "
      //             << "overlapping with the new range: new "
      //             << HexString(base) << "+" << HexString(size)
      //             << ", existing " << HexString(other_base) << "+"
      //             << HexString(other_size);
      return false;
    }
  }

  if (iterator_high != map_.end() && iterator_high->second.base() <= high) {
    // The range above this one overlaps with this one.  It may fully
    // contain this range, or it may begin within this range and extend
    // higher.
    if (merge_strategy_ == MergeRangeStrategy::kTruncateLower) {
      AddressType other_base = iterator_high->second.base();
      if (base < other_base) {
        return StoreRangeInternal(base, delta, other_base - base, entry);
      } else if (other_base < base) {
        EntryType other_entry;
        AddressType other_high, other_size, other_delta;
        other_high = iterator_high->first;
        RetrieveRange(other_high, &other_entry, &other_base, &other_delta,
                      &other_size);
        map_.erase(iterator_high);
        map_.insert(
            MapValue(base - 1, Range(other_base, other_delta, other_entry)));
        return StoreRangeInternal(base, delta, size, entry);
      } else {
        return false;
      }
    } else if (merge_strategy_ == MergeRangeStrategy::kTruncateUpper &&
               iterator_high->first > high) {
      // Shrink the other range down.
      AddressType other_high = iterator_high->first;
      AddressType additional_delta = high - iterator_high->second.base() + 1;
      EntryType other_entry;
      AddressType other_base = AddressType();
      AddressType other_size = AddressType();
      AddressType other_delta = AddressType();
      RetrieveRange(other_high, &other_entry, &other_base, &other_delta,
                    &other_size);
      map_.erase(iterator_high);
      map_.insert(MapValue(other_high,
                           Range(other_base + additional_delta,
                                 other_delta + additional_delta, other_entry)));
      // Retry to store this range.
      return StoreRangeInternal(base, delta, size, entry);
    } else {
      // The processor hits this case too frequently with common symbol files.
      // This is most appropriate for a DEBUG channel, but since none exists
      // now simply comment out this logging.
      //
      // AddressType other_base = iterator_high->second.base();
      // AddressType other_size = iterator_high->first - other_base + 1;
      // BPLOG(INFO) << "StoreRangeInternal failed, an existing range "
      //             << "contains or extends higher than the new range: new "
      //             << HexString(base) << "+" << HexString(size)
      //             << ", existing " << HexString(other_base) << "+"
      //             << HexString(other_size);
      return false;
    }
  }

  // Store the range in the map by its high address, so that lower_bound can
  // be used to quickly locate a range by address.
  map_.insert(MapValue(high, Range(base, delta, entry)));
  return true;
}


template<typename AddressType, typename EntryType>
bool RangeMap<AddressType, EntryType>::RetrieveRange(
    const AddressType& address, EntryType* entry, AddressType* entry_base,
    AddressType* entry_delta, AddressType* entry_size) const {
  BPLOG_IF(ERROR, !entry) << "RangeMap::RetrieveRange requires |entry|";
  assert(entry);

  MapConstIterator iterator = map_.lower_bound(address);
  if (iterator == map_.end())
    return false;

  // The map is keyed by the high address of each range, so |address| is
  // guaranteed to be lower than the range's high address.  If |range| is
  // not directly preceded by another range, it's possible for address to
  // be below the range's low address, though.  When that happens, address
  // references something not within any range, so return false.
  if (address < iterator->second.base())
    return false;

  *entry = iterator->second.entry();
  if (entry_base)
    *entry_base = iterator->second.base();
  if (entry_delta)
    *entry_delta = iterator->second.delta();
  if (entry_size)
    *entry_size = iterator->first - iterator->second.base() + 1;

  return true;
}


template<typename AddressType, typename EntryType>
bool RangeMap<AddressType, EntryType>::RetrieveNearestRange(
    const AddressType& address, EntryType* entry, AddressType* entry_base,
    AddressType* entry_delta, AddressType* entry_size) const {
  BPLOG_IF(ERROR, !entry) << "RangeMap::RetrieveNearestRange requires |entry|";
  assert(entry);

  // If address is within a range, RetrieveRange can handle it.
  if (RetrieveRange(address, entry, entry_base, entry_delta, entry_size))
    return true;

  // upper_bound gives the first element whose key is greater than address,
  // but we want the first element whose key is less than or equal to address.
  // Decrement the iterator to get there, but not if the upper_bound already
  // points to the beginning of the map - in that case, address is lower than
  // the lowest stored key, so return false.
  MapConstIterator iterator = map_.upper_bound(address);
  if (iterator == map_.begin())
    return false;
  --iterator;

  *entry = iterator->second.entry();
  if (entry_base)
    *entry_base = iterator->second.base();
  if (entry_delta)
    *entry_delta = iterator->second.delta();
  if (entry_size)
    *entry_size = iterator->first - iterator->second.base() + 1;

  return true;
}


template<typename AddressType, typename EntryType>
bool RangeMap<AddressType, EntryType>::RetrieveRangeAtIndex(
    int64_t index, EntryType* entry, AddressType* entry_base,
    AddressType* entry_delta, AddressType* entry_size) const {
  BPLOG_IF(ERROR, !entry) << "RangeMap::RetrieveRangeAtIndex requires |entry|";
  assert(entry);

  if (index >= GetCount()) {
    BPLOG(ERROR) << "Index out of range: " << index << "/" << GetCount();
    return false;
  }

  // Walk through the map.  Although it's ordered, it's not a vector, so it
  // can't be addressed directly by index.
  MapConstIterator iterator = map_.begin();
  for (int64_t this_index = 0; this_index < index; ++this_index)
    ++iterator;

  *entry = iterator->second.entry();
  if (entry_base)
    *entry_base = iterator->second.base();
  if (entry_delta)
    *entry_delta = iterator->second.delta();
  if (entry_size)
    *entry_size = iterator->first - iterator->second.base() + 1;

  return true;
}


template<typename AddressType, typename EntryType>
int64_t RangeMap<AddressType, EntryType>::GetCount() const {
  return static_cast<int64_t>(map_.size());
}


template<typename AddressType, typename EntryType>
void RangeMap<AddressType, EntryType>::Clear() {
  map_.clear();
}


}  // namespace google_breakpad


#endif  // PROCESSOR_RANGE_MAP_INL_H__
