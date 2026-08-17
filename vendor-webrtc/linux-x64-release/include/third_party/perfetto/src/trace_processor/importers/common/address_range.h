/*
 * Copyright (C) 2024 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_ADDRESS_RANGE_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_ADDRESS_RANGE_H_

#include <algorithm>
#include <cstdint>
#include <iterator>
#include <map>
#include <set>
#include <tuple>
#include <utility>

#include "perfetto/base/logging.h"

namespace perfetto {
namespace trace_processor {

// A range in the form [start, end), i.e. start is inclusive and end is
// exclusive.
// Note: This means that you can not have a range containing int64_max
class AddressRange {
 public:
  struct CompareByEnd {
    // Allow heterogeneous lookups (https://abseil.io/tips/144)
    using is_transparent = void;
    // Keeps ranges sorted by end address
    bool operator()(const AddressRange& lhs, const AddressRange& rhs) const {
      return lhs.end() < rhs.end();
    }

    // Overload to implement PC lookup via upper_bound.
    bool operator()(const AddressRange& lhs, uint64_t pc) const {
      return lhs.end() < pc;
    }

    // Overload to implement PC lookup via upper_bound.
    bool operator()(uint64_t pc, const AddressRange& rhs) const {
      return pc < rhs.end();
    }
  };

  static constexpr AddressRange FromStartAndSize(uint64_t start,
                                                 uint64_t size) {
    return AddressRange(start, start + size);
  }
  constexpr AddressRange() : AddressRange(0, 0) {}

  constexpr AddressRange(uint64_t start, uint64_t end)
      : start_(start), end_(end) {
    PERFETTO_CHECK(start <= end);
  }

  // Checks whether the given `addr` lies withing this range.
  constexpr bool Contains(uint64_t addr) const {
    return start_ <= addr && addr < end_;
  }

  // Checks whether the given `other` range is fully contained in this range.
  constexpr bool Contains(const AddressRange& other) const {
    return start_ <= other.start_ && other.end_ <= end_;
  }

  // Computes the intersection of the two ranges, that is, it returns a range
  // with all the points in common between the two.
  constexpr AddressRange IntersectWith(const AddressRange& other) const {
    auto start = std::max(start_, other.start_);
    auto end = std::min(end_, other.end_);
    return start < end ? AddressRange(start, end) : AddressRange();
  }

  // Checks whether there is any overlap between the two ranges, that it, if
  // there exists a point such that Contains(point) would return true for both
  // ranges.
  constexpr bool Overlaps(const AddressRange& other) const {
    return !empty() && !other.empty() && start_ < other.end_ &&
           other.start_ < end_;
  }

  // Two ranges are the same is their respective limits are the same, that is A
  // contains A and B contains A
  constexpr bool operator==(const AddressRange& other) const {
    return start_ == other.start_ && end_ == other.end_;
  }
  constexpr bool operator!=(const AddressRange& other) const {
    return !(*this == other);
  }

  // Start of range, inclusive
  constexpr uint64_t start() const { return start_; }
  // Start of range, exclusive
  constexpr uint64_t end() const { return end_; }

  constexpr uint64_t length() const { return end_ - start_; }
  constexpr uint64_t size() const { return end_ - start_; }

  // Check whether the length is zero, that is no point will is contained by
  // this range.
  constexpr bool empty() const { return length() == 0; }

 private:
  uint64_t start_;
  uint64_t end_;
};

// Contains unique collection of addresses. These addresses are kept as
// sorted collection of non contiguous and non overlapping AddressRange
// instances. As addresses are added or removed these AddressRange might be
// merged or spliced as needed to keep the ranges non contiguous and non
// overlapping.
class AddressSet {
 public:
  // TODO(carlscab): Consider using base::FlatSet. As of now this class is used
  // so little that it does not really matter.
  using Impl = std::set<AddressRange, AddressRange::CompareByEnd>;

  using value_type = typename Impl::value_type;
  using const_iterator = typename Impl::const_iterator;
  using size_type = typename Impl::size_type;

  const_iterator begin() const { return ranges_.begin(); }
  const_iterator end() const { return ranges_.end(); }

  // Adds all the addresses in the given range to the set.
  void Add(AddressRange range) {
    if (range.empty()) {
      return;
    }
    uint64_t start = range.start();
    uint64_t end = range.end();
    // Note lower_bound here as we might need to merge with the range just
    // before.
    auto it = ranges_.lower_bound(start);

    PERFETTO_DCHECK(it == ranges_.end() || range.start() <= it->end());

    while (it != ranges_.end() && range.end() >= it->start()) {
      start = std::min(start, it->start());
      end = std::max(end, it->end());
      it = ranges_.erase(it);
    }
    ranges_.emplace_hint(it, AddressRange(start, end));
  }

  // Removes all the addresses in the given range from the set.
  void Remove(AddressRange range) {
    if (range.empty()) {
      return;
    }
    auto it = ranges_.upper_bound(range.start());
    PERFETTO_DCHECK(it == ranges_.end() || range.start() < it->end());

    while (it != ranges_.end() && range.end() > it->start()) {
      if (range.start() > it->start()) {
        // range.start() is contained in *it. Split *it at range.start() into
        // two ranges. Continue the loop at the second of them.
        PERFETTO_DCHECK(it->Contains(range.start()));
        auto old = *it;
        it = ranges_.erase(it);
        ranges_.emplace_hint(it, old.start(), range.start());
        it = ranges_.emplace_hint(it, range.start(), old.end());
      } else if (range.end() < it->end()) {
        // range.end() is contained in *it. Split *it at range.end() into two
        // ranges. The first of them needs to be deleted.
        PERFETTO_DCHECK(it->Contains(range.end()));
        auto old_end = it->end();
        it = ranges_.erase(it);
        ranges_.emplace_hint(it, range.end(), old_end);
      } else {
        // range fully contains *it, so it can be removed
        PERFETTO_DCHECK(range.Contains(*it));
        it = ranges_.erase(it);
      }
    }
  }

  bool operator==(const AddressSet& o) const { return ranges_ == o.ranges_; }
  bool operator!=(const AddressSet& o) const { return ranges_ != o.ranges_; }

 private:
  // Invariants:
  //   * There are no overlapping ranges.
  //   * There are no empty ranges.
  //   * There are no two ranges a, b where a.end == b.start, that is there are
  //     no contiguous mappings.
  //   * Ranges are sorted by end
  // Thus lookups are O(log N) and point lookups are trivial using upper_bound()
  Impl ranges_;
};

// Maps AddressRange instances to a given value. These AddressRange instances
// (basically the keys of the map)  will never overlap, as insertions of
// overlapping ranges will always fail.
template <typename Value>
class AddressRangeMap {
 public:
  using Impl = std::map<AddressRange, Value, AddressRange::CompareByEnd>;

  using value_type = typename Impl::value_type;
  using iterator = typename Impl::iterator;
  using const_iterator = typename Impl::const_iterator;
  using size_type = typename Impl::size_type;

  // Fails if the new range overlaps with any existing one or when inserting an
  // empty range (as there is effectively no key to map from).
  template <typename... Args>
  bool Emplace(AddressRange range, Args&&... args) {
    if (range.empty()) {
      return false;
    }
    auto it = ranges_.upper_bound(range.start());
    if (it != ranges_.end() && range.end() > it->first.start()) {
      return false;
    }
    ranges_.emplace_hint(it, std::piecewise_construct,
                         std::forward_as_tuple(range),
                         std::forward_as_tuple(std::forward<Args>(args)...));
    return true;
  }

  // Finds the map entry that fully contains the given `range` or `end()` if not
  // such entry can be found.
  // ATTENTION: `range` can not be empty. Strictly speaking any range contains
  // the empty range but that would mean we need to return all the ranges here.
  // So we chose to just ban that case.
  iterator FindRangeThatContains(AddressRange range) {
    PERFETTO_CHECK(!range.empty());
    auto it = Find(range.start());
    if (it != end() && it->first.end() >= range.end()) {
      return it;
    }
    return end();
  }

  // Finds the range that contains a given address.
  iterator Find(uint64_t address) {
    auto it = ranges_.upper_bound(address);
    if (it != ranges_.end() && address >= it->first.start()) {
      return it;
    }
    return end();
  }

  // Finds the range that contains a given address.
  const_iterator Find(uint64_t address) const {
    auto it = ranges_.upper_bound(address);
    if (it != ranges_.end() && address >= it->first.start()) {
      return it;
    }
    return end();
  }

  // std::map like methods

  bool empty() const { return ranges_.empty(); }
  bool size() const { return ranges_.size(); }
  iterator begin() { return ranges_.begin(); }
  const_iterator begin() const { return ranges_.begin(); }
  iterator end() { return ranges_.end(); }
  const_iterator end() const { return ranges_.end(); }
  iterator erase(const_iterator pos) { return ranges_.erase(pos); }

  // Emplaces a new value into the map by first trimming all overlapping
  // intervals, deleting them if the overlap is fully contained in the new
  // range, and splitting into two entries pointing to the same value if a
  // single entry fully contains the new range. Returns true on success, fails
  // if the new range is empty (as there is effectively no key to map from).
  template <typename... Args>
  bool TrimOverlapsAndEmplace(AddressRange range, Args&&... args) {
    if (range.empty()) {
      return false;
    }
    auto it = ranges_.upper_bound(range.start());
    PERFETTO_DCHECK(it == ranges_.end() || range.start() < it->first.end());

    // First check if we need to trim the first overlapping range, if any.
    if (it != ranges_.end() && it->first.start() < range.start()) {
      // Range starts after `it->first` starts, but before `it->first` ends, and
      // so overlaps it:
      //   it->first:   |-----------?
      //       range:        |------?
      PERFETTO_DCHECK(it->first.Overlaps(range));

      // Cache it->first since we'll be mutating it in TrimEntryRange.
      AddressRange existing_range = it->first;

      // Trim the first overlap to end at the start of the range.
      //   it->first:   |----|
      //       range:        |------?
      it = TrimEntryRange(it,
                          AddressRange(existing_range.start(), range.start()));

      if (range.end() < existing_range.end()) {
        // Range also ends before existing_range, thus strictly containing it.
        //   existing_range:   |-----------|    (previously it->first)
        //            range:        |----|
        PERFETTO_DCHECK(existing_range.Contains(range));

        // In this special case, we need to split existing_range into two
        // ranges, with the same value, and insert the new range between them:
        //        it->first:   |----|
        //            range:        |----|
        //             tail:             |-|
        // We've already trimmed existing_range (as it->first), so just add
        // the tail now.

        AddressRange tail(range.end(), existing_range.end());
        ranges_.emplace_hint(std::next(it, 1), tail, Value(it->second));
      }

      // After trimming, the current iterated range is now before the new
      // range. This means it no longer ends after the new range starts, and we
      // need to advance the iterator to the new upper_bound.
      ++it;
      PERFETTO_DCHECK(it == ranges_.upper_bound(range.start()));
    }

    // Now, check for any ranges which are _fully_ contained inside
    // the existing range.
    while (it != ranges_.end() && it->first.end() <= range.end()) {
      // Range fully contains `it->first`:
      //   it->first:     |----|
      //       range:   |-----------|
      //
      // We're testing for whether it ends after it->first, and we know it
      // starts before it->first (because we've already handled the first
      // overlap), so this existing range is fully contained inside the new
      // range
      PERFETTO_DCHECK(range.Contains(it->first));
      it = ranges_.erase(it);
    }

    // Finally, check if we need to trim the last range. We know that it
    // ends after the new range, but it might also start after the new
    // range, or we might have reached the end, so this is really a check for
    // overlap.
    if (it != ranges_.end() && it->first.start() < range.end()) {
      // Range overlaps with it->first, and ends before `it->first`:
      //   it->first:     |----------|
      //       range:   |-----|
      PERFETTO_DCHECK(range.Overlaps(it->first));

      // Trim this overlap to end after the end of the range, and insert it
      // after where the range will be inserted.
      //       range:   |-----|
      //   it->first:         |-----|
      it = TrimEntryRange(it, AddressRange(range.end(), it->first.end()));

      // `it` now points to the newly trimmed range, which is _after_ where
      // we want to insert the new range. This is what we want as an insertion
      // hint, so keep it as is.
    }

    ranges_.emplace_hint(it, std::piecewise_construct,
                         std::forward_as_tuple(range),
                         std::forward_as_tuple(std::forward<Args>(args)...));

    return true;
  }

  // Emplaces a new value into the map by first deleting all overlapping
  // intervals. It takes an optional (set to nullptr to ignore) callback `cb`
  // that will be called for each deleted map entry.
  // Returns true on success, fails if the new range is empty (as there is
  // effectively no key to map from).
  template <typename Callback, typename... Args>
  bool DeleteOverlapsAndEmplace(Callback cb,
                                AddressRange range,
                                Args&&... args) {
    if (range.empty()) {
      return false;
    }
    auto it = ranges_.upper_bound(range.start());
    PERFETTO_DCHECK(it == ranges_.end() || range.start() < it->first.end());

    while (it != ranges_.end() && range.end() > it->first.start()) {
      cb(*it);
      it = ranges_.erase(it);
    }

    ranges_.emplace_hint(it, std::piecewise_construct,
                         std::forward_as_tuple(range),
                         std::forward_as_tuple(std::forward<Args>(args)...));
    return true;
  }

  // Calls `cb` for each entry in the map that overlaps the given `range`. That
  // is, there is a point so for which `AddressRange::Contains` returns true for
  // both the entry and the given `range'
  template <typename Callback>
  void ForOverlaps(AddressRange range, Callback cb) {
    if (range.empty()) {
      return;
    }
    for (auto it = ranges_.upper_bound(range.start());
         it != ranges_.end() && range.end() > it->first.start(); ++it) {
      cb(*it);
    }
  }

 private:
  // Trim an entry's address range to a new value. The new value must be fully
  // contained inside the existing range's value, to guarantee that the ranges
  // stay in the same order. Returns a new iterator to the trimmed entry, since
  // the trimming process invalidates the iterator.
  typename Impl::iterator TrimEntryRange(typename Impl::iterator it,
                                         AddressRange new_range) {
    PERFETTO_DCHECK(it->first.Contains(new_range));
    PERFETTO_DCHECK(!new_range.empty());

    // Advance the iterator so that it stays valid -- it now also conveniently
    // points to the entry after the current entry, which is exactly the hint we
    // want for re-inserting in the same place.
    auto extracted = ranges_.extract(it++);
    extracted.key() = new_range;
    // Reinsert in the same place, using the advanced iterator as a hint.
    return ranges_.insert(it, std::move(extracted));
  }

  // Invariants:
  //   * There are no overlapping ranges.
  //   * There are no empty ranges.
  //   * Ranges are sorted by end
  // Thus lookups are O(log N) and point lookups are trivial using upper_bound()
  Impl ranges_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_ADDRESS_RANGE_H_
