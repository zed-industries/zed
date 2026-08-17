// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_CONTAINERS_ENUM_SET_H_
#define BASE_CONTAINERS_ENUM_SET_H_

#include <bitset>
#include <cstddef>
#include <initializer_list>
#include <optional>
#include <string>
#include <type_traits>
#include <utility>

#include "base/check.h"
#include "base/check_op.h"
#include "base/memory/raw_ptr.h"
#include "base/types/cxx23_to_underlying.h"
#include "build/build_config.h"

namespace base {

// Forward declarations needed for friend declarations.
template <typename E, E MinEnumValue, E MaxEnumValue>
class EnumSet;

template <typename E, E Min, E Max>
EnumSet<E, Min, Max> Union(EnumSet<E, Min, Max> set1,
                           EnumSet<E, Min, Max> set2);

template <typename E, E Min, E Max>
EnumSet<E, Min, Max> Intersection(EnumSet<E, Min, Max> set1,
                                  EnumSet<E, Min, Max> set2);

template <typename E, E Min, E Max>
EnumSet<E, Min, Max> Difference(EnumSet<E, Min, Max> set1,
                                EnumSet<E, Min, Max> set2);

// An EnumSet is a set that can hold enum values between a min and a
// max value (inclusive of both).  It's essentially a wrapper around
// std::bitset<> with stronger type enforcement, more descriptive
// member function names, and an iterator interface.
//
// If you're working with enums with a small number of possible values
// (say, fewer than 64), you can efficiently pass around an EnumSet
// for that enum around by value.

template <typename E, E MinEnumValue, E MaxEnumValue>
class EnumSet {
 private:
  static_assert(
      std::is_enum_v<E>,
      "First template parameter of EnumSet must be an enumeration type");

  static constexpr bool InRange(E value) {
    return (value >= MinEnumValue) && (value <= MaxEnumValue);
  }

 public:
  using EnumType = E;
  static const E kMinValue = MinEnumValue;
  static const E kMaxValue = MaxEnumValue;
  static const size_t kValueCount =
      to_underlying(kMaxValue) - to_underlying(kMinValue) + 1;

  static_assert(kMinValue <= kMaxValue,
                "min value must be no greater than max value");

  // Allow use with ::testing::ValuesIn, which expects a value_type defined.
  using value_type = EnumType;

 private:
  // Declaration needed by Iterator.
  using EnumBitSet = std::bitset<kValueCount>;

 public:
  // Iterator is a forward-only read-only iterator for EnumSet. It follows the
  // common STL input iterator interface (like std::unordered_set).
  //
  // Example usage, using a range-based for loop:
  //
  // EnumSet<SomeType> enums;
  // for (SomeType val : enums) {
  //   Process(val);
  // }
  //
  // Or using an explicit iterator (not recommended):
  //
  // for (EnumSet<...>::Iterator it = enums.begin(); it != enums.end(); it++) {
  //   Process(*it);
  // }
  //
  // The iterator must not be outlived by the set. In particular, the following
  // is an error:
  //
  // EnumSet<...> SomeFn() { ... }
  //
  // /* ERROR */
  // for (EnumSet<...>::Iterator it = SomeFun().begin(); ...
  //
  // Also, there are no guarantees as to what will happen if you
  // modify an EnumSet while traversing it with an iterator.
  class Iterator {
   public:
    using value_type = EnumType;
    using size_type = size_t;
    using difference_type = ptrdiff_t;
    using pointer = EnumType*;
    using reference = EnumType&;
    using iterator_category = std::forward_iterator_tag;

    Iterator() : enums_(nullptr), i_(kValueCount) {}
    ~Iterator() = default;

    Iterator(const Iterator&) = default;
    Iterator& operator=(const Iterator&) = default;

    Iterator(Iterator&&) = default;
    Iterator& operator=(Iterator&&) = default;

    friend bool operator==(const Iterator& lhs, const Iterator& rhs) {
      return lhs.i_ == rhs.i_;
    }

    value_type operator*() const {
      DCHECK(Good());
      return FromIndex(i_);
    }

    Iterator& operator++() {
      DCHECK(Good());
      // If there are no more set elements in the bitset, this will result in an
      // index equal to kValueCount, which is equivalent to EnumSet.end().
      i_ = FindNext(i_ + 1);

      return *this;
    }

    Iterator operator++(int) {
      DCHECK(Good());
      Iterator old(*this);

      // If there are no more set elements in the bitset, this will result in an
      // index equal to kValueCount, which is equivalent to EnumSet.end().
      i_ = FindNext(i_ + 1);

      return std::move(old);
    }

   private:
    friend Iterator EnumSet::begin() const;

    explicit Iterator(const EnumBitSet& enums)
        : enums_(&enums), i_(FindNext(0)) {}

    // Returns true iff the iterator points to an EnumSet and it
    // hasn't yet traversed the EnumSet entirely.
    bool Good() const { return enums_ && i_ < kValueCount && enums_->test(i_); }

    size_t FindNext(size_t i) {
      while ((i < kValueCount) && !enums_->test(i)) {
        ++i;
      }
      return i;
    }

    raw_ptr<const EnumBitSet> enums_;
    size_t i_;
  };

  EnumSet() = default;

  ~EnumSet() = default;

  constexpr EnumSet(std::initializer_list<E> values) {
    if (std::is_constant_evaluated()) {
      enums_ = bitstring(values);
    } else {
      for (E value : values) {
        Put(value);
      }
    }
  }

  // Returns an EnumSet with all values between kMinValue and kMaxValue, which
  // also contains undefined enum values if the enum in question has gaps
  // between kMinValue and kMaxValue.
  static constexpr EnumSet All() {
    if (std::is_constant_evaluated()) {
      if (kValueCount == 0) {
        return EnumSet();
      }
      // Since `1 << kValueCount` may trigger shift-count-overflow warning if
      // the `kValueCount` is 64, instead of returning `(1 << kValueCount) - 1`,
      // the bitmask will be constructed from two parts: the most significant
      // bits and the remaining.
      uint64_t mask = 1ULL << (kValueCount - 1);
      return EnumSet(EnumBitSet(mask - 1 + mask));
    } else {
      // When `kValueCount` is greater than 64, we can't use the constexpr path,
      // and we will build an `EnumSet` value by value.
      EnumSet enum_set;
      for (size_t value = 0; value < kValueCount; ++value) {
        enum_set.Put(FromIndex(value));
      }
      return enum_set;
    }
  }

  // Returns an EnumSet with all the values from start to end, inclusive.
  static constexpr EnumSet FromRange(E start, E end) {
    CHECK_LE(start, end);
    return EnumSet(EnumBitSet(
        ((single_val_bitstring(end)) - (single_val_bitstring(start))) |
        (single_val_bitstring(end))));
  }

  // Copy constructor and assignment welcome.

  // Bitmask operations.
  //
  // This bitmask is 0-based and the value of the Nth bit depends on whether
  // the set contains an enum element of integer value N.
  //
  // These may only be used if Min >= 0 and Max < 64.

  // Returns an EnumSet constructed from |bitmask|.
  static constexpr EnumSet FromEnumBitmask(const uint64_t bitmask) {
    static_assert(to_underlying(kMaxValue) < 64,
                  "The highest enum value must be < 64 for FromEnumBitmask ");
    static_assert(to_underlying(kMinValue) >= 0,
                  "The lowest enum value must be >= 0 for FromEnumBitmask ");
    return EnumSet(EnumBitSet(bitmask >> to_underlying(kMinValue)));
  }
  // Returns a bitmask for the EnumSet.
  uint64_t ToEnumBitmask() const {
    static_assert(to_underlying(kMaxValue) < 64,
                  "The highest enum value must be < 64 for ToEnumBitmask ");
    static_assert(to_underlying(kMinValue) >= 0,
                  "The lowest enum value must be >= 0 for FromEnumBitmask ");
    return enums_.to_ullong() << to_underlying(kMinValue);
  }

  // Returns a uint64_t bit mask representing the values within the range
  // [64*n, 64*n + 63] of the EnumSet.
  std::optional<uint64_t> GetNth64bitWordBitmask(size_t n) const {
    // If the EnumSet contains less than n 64-bit masks, return std::nullopt.
    if (to_underlying(kMaxValue) / 64 < n) {
      return std::nullopt;
    }

    std::bitset<kValueCount> mask = ~uint64_t{0};
    std::bitset<kValueCount> bits = enums_;
    if (to_underlying(kMinValue) < n * 64) {
      bits >>= n * 64 - to_underlying(kMinValue);
    }
    uint64_t result = (bits & mask).to_ullong();
    if (to_underlying(kMinValue) > n * 64) {
      result <<= to_underlying(kMinValue) - n * 64;
    }
    return result;
  }

  // Set operations.  Put, Retain, and Remove are basically
  // self-mutating versions of Union, Intersection, and Difference
  // (defined below).

  // Adds the given value (which must be in range) to our set.
  void Put(E value) { enums_.set(ToIndex(value)); }

  // Adds all values in the given set to our set.
  void PutAll(EnumSet other) { enums_ |= other.enums_; }

  // Adds all values in the given range to our set, inclusive.
  void PutRange(E start, E end) {
    CHECK_LE(start, end);
    size_t endIndexInclusive = ToIndex(end);
    for (size_t current = ToIndex(start); current <= endIndexInclusive;
         ++current) {
      enums_.set(current);
    }
  }

  // There's no real need for a Retain(E) member function.

  // Removes all values not in the given set from our set.
  void RetainAll(EnumSet other) { enums_ &= other.enums_; }

  // If the given value is in range, removes it from our set.
  void Remove(E value) {
    if (InRange(value)) {
      enums_.reset(ToIndex(value));
    }
  }

  // Removes all values in the given set from our set.
  void RemoveAll(EnumSet other) { enums_ &= ~other.enums_; }

  // Removes all values from our set.
  void Clear() { enums_.reset(); }

  // Conditionally puts or removes `value`, based on `should_be_present`.
  void PutOrRemove(E value, bool should_be_present) {
    if (should_be_present) {
      Put(value);
    } else {
      Remove(value);
    }
  }

  // Returns true iff the given value is in range and a member of our set.
  constexpr bool Has(E value) const {
    return InRange(value) && enums_[ToIndex(value)];
  }

  // Returns true iff the given set is a subset of our set.
  bool HasAll(EnumSet other) const {
    return (enums_ & other.enums_) == other.enums_;
  }

  // Returns true if the given set contains any value of our set.
  bool HasAny(EnumSet other) const {
    return (enums_ & other.enums_).count() > 0;
  }

  // Returns true iff our set is empty.
  bool empty() const { return !enums_.any(); }

  // Returns how many values our set has.
  size_t size() const { return enums_.count(); }

  // Returns an iterator pointing to the first element (if any).
  Iterator begin() const { return Iterator(enums_); }

  // Returns an iterator that does not point to any element, but to the position
  // that follows the last element in the set.
  Iterator end() const { return Iterator(); }

  // Returns true iff our set and the given set contain exactly the same values.
  friend bool operator==(const EnumSet&, const EnumSet&) = default;

  std::string ToString() const { return enums_.to_string(); }

 private:
  friend EnumSet Union<E, MinEnumValue, MaxEnumValue>(EnumSet set1,
                                                      EnumSet set2);
  friend EnumSet Intersection<E, MinEnumValue, MaxEnumValue>(EnumSet set1,
                                                             EnumSet set2);
  friend EnumSet Difference<E, MinEnumValue, MaxEnumValue>(EnumSet set1,
                                                           EnumSet set2);

  static constexpr uint64_t bitstring(const std::initializer_list<E>& values) {
    uint64_t result = 0;
    for (E value : values) {
      result |= single_val_bitstring(value);
    }
    return result;
  }

  static constexpr uint64_t single_val_bitstring(E val) {
    const uint64_t bitstring = 1;
    const size_t shift_amount = ToIndex(val);
    CHECK_LT(shift_amount, sizeof(bitstring) * 8);
    return bitstring << shift_amount;
  }

  // A bitset can't be constexpr constructed if it has size > 64, since the
  // constexpr constructor uses a uint64_t. If your EnumSet has > 64 values, you
  // can safely remove the constepxr qualifiers from this file, at the cost of
  // some minor optimizations.
  explicit constexpr EnumSet(EnumBitSet enums) : enums_(enums) {
    if (std::is_constant_evaluated()) {
      CHECK(kValueCount <= 64)
          << "Max number of enum values is 64 for constexpr constructor";
    }
  }

  // Converts a value to/from an index into |enums_|.
  static constexpr size_t ToIndex(E value) {
    CHECK(InRange(value));
    return static_cast<size_t>(to_underlying(value)) -
           static_cast<size_t>(to_underlying(MinEnumValue));
  }

  static E FromIndex(size_t i) {
    DCHECK_LT(i, kValueCount);
    return static_cast<E>(to_underlying(MinEnumValue) + i);
  }

  EnumBitSet enums_;
};

template <typename E, E MinEnumValue, E MaxEnumValue>
const E EnumSet<E, MinEnumValue, MaxEnumValue>::kMinValue;

template <typename E, E MinEnumValue, E MaxEnumValue>
const E EnumSet<E, MinEnumValue, MaxEnumValue>::kMaxValue;

template <typename E, E MinEnumValue, E MaxEnumValue>
const size_t EnumSet<E, MinEnumValue, MaxEnumValue>::kValueCount;

// The usual set operations.

template <typename E, E Min, E Max>
EnumSet<E, Min, Max> Union(EnumSet<E, Min, Max> set1,
                           EnumSet<E, Min, Max> set2) {
  return EnumSet<E, Min, Max>(set1.enums_ | set2.enums_);
}

template <typename E, E Min, E Max>
EnumSet<E, Min, Max> Intersection(EnumSet<E, Min, Max> set1,
                                  EnumSet<E, Min, Max> set2) {
  return EnumSet<E, Min, Max>(set1.enums_ & set2.enums_);
}

template <typename E, E Min, E Max>
EnumSet<E, Min, Max> Difference(EnumSet<E, Min, Max> set1,
                                EnumSet<E, Min, Max> set2) {
  return EnumSet<E, Min, Max>(set1.enums_ & ~set2.enums_);
}

}  // namespace base

#endif  // BASE_CONTAINERS_ENUM_SET_H_
