// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_INTERVAL_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_INTERVAL_MAP_H_

#include <algorithm>
#include <limits>
#include <map>

#include "base/check.h"
#include "base/memory/raw_ptr.h"
#include "base/not_fatal_until.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"

namespace blink {

// An IntervalMap<KeyType, ValueType> maps every value of KeyType to
// a ValueType, and incrementing, decrementing and setting ranges of values
// has been optimized. The default state is to map all values in
// KeyType to ValueType(). (Which is usually zero.)
//
// Set/Increment operations should generally take
// O(log(N)) + O(M) time where N is the number of intervals in the map and
// M is the number of modified intervals.
//
// Internally, IntervalMap<> uses an std::map, where the beginning of each
// interval is stored along with the value for that interval. Adjacent intervals
// which have the same value are automatically merged. For instance, if you did:
//
// IntervalMap<int, int> tmp;
// tmp.IncrementInterval(2, 5, 2);
// tmp.IncrementInterval(4, 6, 1);
//
// Then:
//  tmp[0] = 0
//  tmp[1] = 0
//  tmp[2] = 2
//  tmp[3] = 2
//  tmp[4] = 3
//  tmp[5] = 1
//  tmp[6] = 0
//
// If you iterate over tmp, you get the following intervals:
//  -maxint .. 2 => 0
//  2 .. 4 => 2
//  4 .. 5 => 3
//  5 .. 6 => 1
//  6 .. maxint => 0
//
// Internally, this would be stored in a map as:
//    -maxint:0, 2:2, 4:3, 5:1, 6:0
//
// TODO(hubbe): Consider consolidating with media::Ranges.

// Simple interval class.
// Interval ends are always non-inclusive.
// Please note that end <= begin is a valid (but empty) interval.
template <typename T>
struct Interval {
 public:
  Interval(const T& begin, const T& end) : begin(begin), end(end) {}

  // May return empty intervals (begin >= end).
  Interval Intersect(const Interval& other) const {
    return Interval(std::max(begin, other.begin), std::min(end, other.end));
  }

  bool Empty() const { return begin >= end; }

  T begin;
  T end;
};

// The IntervalMapConstIterator points to an interval in an
// IntervalMap where all values are the same. Calling ++/--
// goes to the next/previous interval, which is guaranteed to
// have values different from the current interval.
template <typename KeyType,
          typename ValueType,
          class Compare = std::less<KeyType>,
          class NumericLimits = std::numeric_limits<KeyType>>
class IntervalMapConstIterator {
 public:
  typedef std::map<KeyType, ValueType, Compare> MapType;
  IntervalMapConstIterator() {}
  IntervalMapConstIterator(const MapType* map,
                           typename MapType::const_iterator iter)
      : map_(map), iter_(iter) {}

  bool operator==(const IntervalMapConstIterator& other) const {
    return iter_ == other.iter_;
  }

  bool operator!=(const IntervalMapConstIterator& other) const {
    return iter_ != other.iter_;
  }

  // Returns the beginning of the current interval.
  KeyType interval_begin() const {
    CHECK(iter_ != map_->end(), base::NotFatalUntil::M130);
    return iter_->first;
  }

  // Returns the end of the current interval, non-inclusive.
  KeyType interval_end() const {
    CHECK(iter_ != map_->end(), base::NotFatalUntil::M130);
    typename MapType::const_iterator next = iter_;
    ++next;
    if (next == map_->end()) {
      return NumericLimits::max();
    } else {
      return next->first;
    }
  }

  // Returns the current interval.
  Interval<KeyType> interval() const {
    return Interval<KeyType>(interval_begin(), interval_end());
  }

  // Returns the value associated with the current interval.
  ValueType value() const {
    CHECK(iter_ != map_->end(), base::NotFatalUntil::M130);
    return iter_->second;
  }

  // Needed to make the following construct work:
  // for (const auto& interval_value_pair : interval_map)
  std::pair<Interval<KeyType>, ValueType> operator*() const {
    return std::make_pair(interval(), value());
  }

  // Go to the next interval.
  // The beginning of the next interval always matches the end of the current
  // interval. (But should always have a different value.)
  // Not allowed if we're already at map_->end().
  void operator++() {
    CHECK(iter_ != map_->end(), base::NotFatalUntil::M130);
    ++iter_;
  }

  // Go to the previous interval.
  // The end of the previous interval always matches the beginning of the
  // current interval. (But should always have a different value.)
  // Not allowed if we're already at map_->begin().
  void operator--() {
    DCHECK(iter_ != map_->begin());
    --iter_;
  }

 private:
  raw_ptr<const MapType> map_;

  // Pointer to the entry in the IntervalMap that specifies the
  // beginning of the current interval.
  typename MapType::const_iterator iter_;
};

template <typename KeyType,
          typename ValueType,
          class Compare = std::less<KeyType>,
          class NumericLimits = std::numeric_limits<KeyType>>
class IntervalMap {
 public:
  typedef std::map<KeyType, ValueType, Compare> MapType;
  typedef IntervalMapConstIterator<KeyType, ValueType, Compare, NumericLimits>
      const_iterator;
  IntervalMap() {
    // Adding an explicit entry for the default interval is not strictly needed,
    // but simplifies the code a lot.
    map_[NumericLimits::min()] = ValueType();
  }

  // Returns the value at a particular point.
  // Defaults to ValueType().
  ValueType operator[](const KeyType& k) const {
    typename MapType::const_iterator i = map_.upper_bound(k);
    DCHECK(i != map_.begin());
    --i;
    return i->second;
  }

  // Increase [from..to) by |how_much|.
  void IncrementInterval(KeyType from, KeyType to, ValueType how_much) {
    if (to <= from || how_much == 0)
      return;
    typename MapType::iterator a = MakeEntry(from);
    typename MapType::iterator b = MakeEntry(to);
    for (typename MapType::iterator i = a; i != b; ++i) {
      i->second += how_much;
    }
    RemoveDuplicates(a);
    // b may be invalid
    RemoveDuplicates(map_.lower_bound(to));
  }

  // Set [from..to) to |how_much|.
  void SetInterval(KeyType from, KeyType to, ValueType how_much) {
    if (to <= from)
      return;
    typename MapType::iterator a = MakeEntry(from);
    typename MapType::iterator b = MakeEntry(to);
    a->second = how_much;
    while (true) {
      typename MapType::iterator c = a;
      ++c;
      if (c == b) {
        break;
      } else {
        map_.erase(c);
      }
    }
    RemoveDuplicates(a);
    // b may be invalid
    RemoveDuplicates(map_.lower_bound(to));
  }

  // Returns an iterator to the first interval.
  // Note, there is always at least one interval.
  const_iterator begin() const { return const_iterator(&map(), map_.begin()); }

  // Returns an end marker iterator.
  const_iterator end() const { return const_iterator(&map(), map_.end()); }

  // Returns an iterator to the interval containing |k|.
  // Always returns a valid iterator.
  const_iterator find(KeyType k) const {
    typename MapType::const_iterator iter = map_.upper_bound(k);
    DCHECK(iter != map_.begin());
    --iter;
    return const_iterator(&map(), iter);
  }

  bool empty() const { return map().size() == 1; }
  void clear() {
    map_.clear();
    map_[NumericLimits::min()] = ValueType();
  }

 private:
  const MapType& map() const { return map_; }

  // Make an entry in map_ with the key |k| and return it's iterator.
  // If such an entry already exists, just re-use it.
  // If a new entry is created, it's value will be set to the same
  // as the preceeding entry, or ValueType() if no preceeding entry exists.
  // After calling this function, we'll need to call RemoveDuplicates()
  // to clean up any duplicates that we made.
  typename MapType::iterator MakeEntry(KeyType k) {
    typename MapType::value_type tmp(k, 0);
    std::pair<typename MapType::iterator, bool> insert_result;
    insert_result = map_.insert(tmp);
    if (insert_result.second) {
      if (insert_result.first != map_.begin()) {
        typename MapType::iterator i = insert_result.first;
        --i;
        insert_result.first->second = i->second;
      }
    }
    return insert_result.first;
  }

  // Remove duplicates before and after |i|.
  void RemoveDuplicates(typename MapType::iterator i) {
    if (i == map_.end())
      return;

    typename MapType::iterator first = i;
    typename MapType::iterator second = i;
    if (i != map_.begin()) {
      --first;
      if (first->second == second->second) {
        map_.erase(second);
        second = first;
      } else {
        first = second;
      }
    }
    ++second;
    if (second != map_.end() && first->second == second->second) {
      map_.erase(second);
    }
  }

  MapType map_ ALLOW_DISCOURAGED_TYPE("HashMap lacks key sorting.");
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_INTERVAL_MAP_H_
