// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_CONTAINERS_SMALL_MAP_H_
#define BASE_CONTAINERS_SMALL_MAP_H_

#include <stddef.h>

#include <array>
#include <limits>
#include <map>
#include <memory>
#include <new>
#include <type_traits>
#include <utility>

#include "base/check.h"
#include "base/check_op.h"
#include "base/containers/adapters.h"
#include "base/containers/span.h"
#include "base/memory/stack_allocated.h"
#include "base/numerics/safe_conversions.h"
#include "base/types/to_address.h"

inline constexpr size_t kUsingFullMapSentinel =
    std::numeric_limits<size_t>::max();

namespace base {

// small_map is a container with a std::map-like interface. It starts out backed
// by an unsorted array but switches to some other container type if it grows
// beyond this fixed size.
//
// Please see //base/containers/README.md for an overview of which container
// to select.
//
// PROS
//
//  - Good memory locality and low overhead for smaller maps.
//  - Handles large maps without the degenerate performance of flat_map.
//
// CONS
//
//  - Larger code size than the alternatives.
//
// IMPORTANT NOTES
//
//  - Iterators are invalidated across mutations.
//
// DETAILS
//
// base::small_map will pick up the comparator from the underlying map type. In
// std::map only a "less" operator is defined, which requires us to do two
// comparisons per element when doing the brute-force search in the simple
// array. std::unordered_map has a key_equal function which will be used.
//
// We define default overrides for the common map types to avoid this
// double-compare, but you should be aware of this if you use your own operator<
// for your map and supply your own version of == to the small_map. You can use
// regular operator== by just doing:
//
//   base::small_map<std::map<MyKey, MyValue>, 4, std::equal_to<KyKey>>
//
//
// USAGE
// -----
//
// NormalMap:  The map type to fall back to. This also defines the key and value
//             types for the small_map.
// kArraySize:  The size of the initial array of results. This will be allocated
//              with the small_map object rather than separately on the heap.
//              Once the map grows beyond this size, the map type will be used
//              instead.
// EqualKey:  A functor which tests two keys for equality. If the wrapped map
//            type has a "key_equal" member (unordered_map does), then that will
//            be used by default. If the wrapped map type has a strict weak
//            ordering "key_compare" (std::map does), that will be used to
//            implement equality by default.
// MapInit: A functor that takes a NormalMap* and uses it to initialize the map.
//          This functor will be called at most once per small_map, when the map
//          exceeds the threshold of kArraySize and we are about to copy values
//          from the array to the map. The functor *must* initialize the
//          NormalMap* argument with placement new, since after it runs we
//          assume that the NormalMap has been initialized.
//
// Example:
//   base::small_map<std::map<string, int>> days;
//   days["sunday"   ] = 0;
//   days["monday"   ] = 1;
//   days["tuesday"  ] = 2;
//   days["wednesday"] = 3;
//   days["thursday" ] = 4;
//   days["friday"   ] = 5;
//   days["saturday" ] = 6;

namespace internal {

template <typename NormalMap>
class small_map_default_init {
 public:
  void operator()(NormalMap* map) const { std::construct_at(map); }
};

// has_key_equal<M>::value is true iff there exists a type M::key_equal. This is
// used to dispatch to one of the select_equal_key<> metafunctions below.
template <typename M>
struct has_key_equal {
  typedef char sml;  // "small" is sometimes #defined so we use an abbreviation.
  typedef struct {
    char dummy[2];
  } big;
  // Two functions, one accepts types that have a key_equal member, and one that
  // accepts anything. They each return a value of a different size, so we can
  // determine at compile-time which function would have been called.
  template <typename U>
  static big test(typename U::key_equal*);
  template <typename>
  static sml test(...);
  // Determines if M::key_equal exists by looking at the size of the return
  // type of the compiler-chosen test() function.
  static const bool value = (sizeof(test<M>(0)) == sizeof(big));
};
template <typename M>
const bool has_key_equal<M>::value;

// Base template used for map types that do NOT have an M::key_equal member,
// e.g., std::map<>. These maps have a strict weak ordering comparator rather
// than an equality functor, so equality will be implemented in terms of that
// comparator.
//
// There's a partial specialization of this template below for map types that do
// have an M::key_equal member.
template <typename M, bool has_key_equal_value>
struct select_equal_key {
  struct equal_key {
    bool operator()(const typename M::key_type& left,
                    const typename M::key_type& right) {
      // Implements equality in terms of a strict weak ordering comparator.
      typename M::key_compare comp;
      return !comp(left, right) && !comp(right, left);
    }
  };
};

// Partial template specialization handles case where M::key_equal exists, e.g.,
// unordered_map<>.
template <typename M>
struct select_equal_key<M, true> {
  typedef typename M::key_equal equal_key;
};

}  // namespace internal

template <typename NormalMap,
          size_t kArraySize = 4,
          typename EqualKey = typename internal::select_equal_key<
              NormalMap,
              internal::has_key_equal<NormalMap>::value>::equal_key,
          typename MapInit = internal::small_map_default_init<NormalMap>>
class small_map {
  static_assert(kArraySize > 0, "Initial size must be greater than 0");
  static_assert(kArraySize != kUsingFullMapSentinel,
                "Initial size out of range");

 public:
  using key_type = NormalMap::key_type;
  using data_type = NormalMap::mapped_type;
  using mapped_type = NormalMap::mapped_type;
  using value_type = NormalMap::value_type;
  using key_equal = EqualKey;

  constexpr small_map() : functor_(MapInit()) { InitEmpty(); }

  constexpr explicit small_map(const MapInit& functor) : functor_(functor) {
    InitEmpty();
  }

  // Allow copy-constructor and assignment, since STL allows them too.
  constexpr small_map(const small_map& src) {
    // size_ and functor_ are initted in InitFrom()
    InitFrom(src);
  }

  constexpr void operator=(const small_map& src) {
    if (&src == this) {
      return;
    }

    // This is not optimal. If src and dest are both using the small array, we
    // could skip the teardown and reconstruct. One problem to be resolved is
    // that the value_type itself is pair<const K, V>, and const K is not
    // assignable.
    Destroy();
    InitFrom(src);
  }

  ~small_map() { Destroy(); }

  // The elements in the inline array storage. They are held in a union so that
  // they can be constructed lazily as they are inserted, and can be destroyed
  // when they are erased.
  union ArrayElement {
    ArrayElement() {}
    ~ArrayElement() {}

    value_type value;
  };

  class const_iterator;

  class iterator {
    STACK_ALLOCATED();

    using map_iterator = NormalMap::iterator;
    using array_iterator = span<ArrayElement>::iterator;

   public:
    using iterator_category = map_iterator::iterator_category;
    using value_type = map_iterator::value_type;
    using difference_type = map_iterator::difference_type;
    using pointer = map_iterator::pointer;
    using reference = map_iterator::reference;

    iterator() = default;

    constexpr iterator& operator++() {
      if (has_array_iter()) {
        ++array_iter_;
      } else {
        ++map_iter_;
      }
      return *this;
    }

    constexpr iterator operator++(int /*unused*/) {
      iterator result(*this);
      ++(*this);
      return result;
    }

    constexpr value_type* operator->() const {
      return has_array_iter() ? std::addressof(array_iter_->value)
                              : std::addressof(*map_iter_);
    }

    constexpr value_type& operator*() const {
      return has_array_iter() ? array_iter_->value : *map_iter_;
    }

    constexpr bool operator==(const iterator& other) const {
      if (has_array_iter()) {
        return array_iter_ == other.array_iter_;
      } else {
        return !other.has_array_iter() && map_iter_ == other.map_iter_;
      }
    }

   private:
    friend class small_map;
    friend class const_iterator;
    constexpr explicit iterator(const array_iterator& init)
        : array_iter_(init) {}
    constexpr explicit iterator(const map_iterator& init) : map_iter_(init) {}

    constexpr bool has_array_iter() const {
      return base::to_address(array_iter_) != nullptr;
    }

    array_iterator array_iter_;
    map_iterator map_iter_;
  };

  class const_iterator {
    STACK_ALLOCATED();

    using map_iterator = NormalMap::const_iterator;
    using array_iterator = span<const ArrayElement>::iterator;

   public:
    using iterator_category = map_iterator::iterator_category;
    using value_type = map_iterator::value_type;
    using difference_type = map_iterator::difference_type;
    using pointer = map_iterator::pointer;
    using reference = map_iterator::reference;

    const_iterator() = default;

    // Non-explicit constructor lets us convert regular iterators to const
    // iterators.
    constexpr const_iterator(const iterator& other)
        : array_iter_(other.array_iter_), map_iter_(other.map_iter_) {}

    constexpr const_iterator& operator++() {
      if (has_array_iter()) {
        ++array_iter_;
      } else {
        ++map_iter_;
      }
      return *this;
    }

    constexpr const_iterator operator++(int /*unused*/) {
      const_iterator result(*this);
      ++(*this);
      return result;
    }

    constexpr const value_type* operator->() const {
      return has_array_iter() ? std::addressof(array_iter_->value)
                              : std::addressof(*map_iter_);
    }

    constexpr const value_type& operator*() const {
      return has_array_iter() ? array_iter_->value : *map_iter_;
    }

    constexpr bool operator==(const const_iterator& other) const {
      if (has_array_iter()) {
        return array_iter_ == other.array_iter_;
      }
      return !other.has_array_iter() && map_iter_ == other.map_iter_;
    }

   private:
    friend class small_map;
    constexpr explicit const_iterator(const array_iterator& init)
        : array_iter_(init) {}
    constexpr explicit const_iterator(const map_iterator& init)
        : map_iter_(init) {}

    constexpr bool has_array_iter() const {
      return base::to_address(array_iter_) != nullptr;
    }

    array_iterator array_iter_;
    map_iterator map_iter_;
  };

  constexpr iterator find(const key_type& key) {
    key_equal compare;

    if (UsingFullMap()) {
      return iterator(map()->find(key));
    }

    span<ArrayElement> r = sized_array_span();
    auto it = r.begin();
    for (; it != r.end(); ++it) {
      if (compare(it->value.first, key)) {
        return iterator(it);
      }
    }
    return iterator(it);
  }

  constexpr const_iterator find(const key_type& key) const {
    key_equal compare;

    if (UsingFullMap()) {
      return const_iterator(map()->find(key));
    }

    span<const ArrayElement> r = sized_array_span();
    auto it = r.begin();
    for (; it != r.end(); ++it) {
      if (compare(it->value.first, key)) {
        return const_iterator(it);
      }
    }
    return const_iterator(it);
  }

  // Invalidates iterators.
  constexpr data_type& operator[](const key_type& key)
    requires(std::is_default_constructible_v<data_type>)
  {
    key_equal compare;

    if (UsingFullMap()) {
      return map_[key];
    }

    // Search backwards to favor recently-added elements.
    span<ArrayElement> r = sized_array_span();
    for (ArrayElement& e : Reversed(r)) {
      if (compare(e.value.first, key)) {
        return e.value.second;
      }
    }

    if (size_ == kArraySize) {
      ConvertToRealMap();
      return map_[key];
    }

    ArrayElement& e = array_[size_++];
    std::construct_at(std::addressof(e.value), key, data_type());
    return e.value.second;
  }

  // Invalidates iterators.
  constexpr std::pair<iterator, bool> insert(const value_type& x) {
    key_equal compare;

    if (UsingFullMap()) {
      auto [map_iter, inserted] = map_.insert(x);
      return std::make_pair(iterator(map_iter), inserted);
    }

    span<ArrayElement> r = sized_array_span();
    for (auto it = r.begin(); it != r.end(); ++it) {
      if (compare(it->value.first, x.first)) {
        return std::make_pair(iterator(it), false);
      }
    }

    if (size_ == kArraySize) {
      ConvertToRealMap();  // Invalidates all iterators!
      auto [map_iter, inserted] = map_.insert(x);
      return std::make_pair(iterator(map_iter), inserted);
    }

    ArrayElement& e = array_[size_++];
    std::construct_at(std::addressof(e.value), x);
    return std::make_pair(iterator(sized_array_span().end() - 1u), true);
  }

  // Invalidates iterators.
  template <class InputIterator>
  constexpr void insert(InputIterator f, InputIterator l) {
    while (f != l) {
      insert(*f);
      ++f;
    }
  }

  // Invalidates iterators.
  template <typename... Args>
  constexpr std::pair<iterator, bool> emplace(Args&&... args) {
    key_equal compare;

    if (UsingFullMap()) {
      auto [map_iter, inserted] = map_.emplace(std::forward<Args>(args)...);
      return std::make_pair(iterator(map_iter), inserted);
    }

    value_type x(std::forward<Args>(args)...);
    span<ArrayElement> r = sized_array_span();
    for (auto it = r.begin(); it != r.end(); ++it) {
      if (compare(it->value.first, x.first)) {
        return std::make_pair(iterator(it), false);
      }
    }

    if (size_ == kArraySize) {
      ConvertToRealMap();  // Invalidates all iterators!
      auto [map_iter, inserted] = map_.emplace(std::move(x));
      return std::make_pair(iterator(map_iter), inserted);
    }

    ArrayElement& p = array_[size_++];
    std::construct_at(std::addressof(p.value), std::move(x));
    return std::make_pair(iterator(sized_array_span().end() - 1u), true);
  }

  constexpr iterator begin() {
    return UsingFullMap() ? iterator(map_.begin())
                          : iterator(sized_array_span().begin());
  }

  constexpr const_iterator begin() const {
    return UsingFullMap() ? const_iterator(map_.begin())
                          : const_iterator(sized_array_span().begin());
  }

  constexpr iterator end() {
    return UsingFullMap() ? iterator(map_.end())
                          : iterator(sized_array_span().end());
  }

  constexpr const_iterator end() const {
    return UsingFullMap() ? const_iterator(map_.end())
                          : const_iterator(sized_array_span().end());
  }

  constexpr void clear() {
    if (UsingFullMap()) {
      // Make the array active in the union.
      map_.~NormalMap();
      std::construct_at(&array_);
    } else {
      for (ArrayElement& e : sized_array_span()) {
        e.value.~value_type();
      }
    }
    size_ = 0u;
  }

  // Invalidates iterators. Returns iterator following the last removed element.
  constexpr iterator erase(const iterator& position) {
    if (UsingFullMap()) {
      return iterator(map_.erase(position.map_iter_));
    }

    auto erase_pos = position.array_iter_;
    auto last_pos = sized_array_span().end() - 1u;

    if (erase_pos == last_pos) {
      erase_pos->value.~value_type();
      --size_;
      return end();
    } else {
      ptrdiff_t index = std::ranges::distance(begin().array_iter_, erase_pos);
      erase_pos->value.~value_type();
      std::construct_at(std::addressof(erase_pos->value),
                        std::move(last_pos->value));
      last_pos->value.~value_type();
      --size_;
      return iterator(sized_array_span().begin() + index);
    }
  }

  constexpr size_t erase(const key_type& key) {
    iterator iter = find(key);
    if (iter == end()) {
      return 0u;
    }
    erase(iter);
    return 1u;
  }

  constexpr size_t count(const key_type& key) const {
    return (find(key) == end()) ? 0u : 1u;
  }

  constexpr size_t size() const { return UsingFullMap() ? map_.size() : size_; }

  constexpr bool empty() const {
    return UsingFullMap() ? map_.empty() : size_ == 0u;
  }

  // Returns true if we have fallen back to using the underlying map
  // representation.
  constexpr bool UsingFullMap() const { return size_ == kUsingFullMapSentinel; }

  constexpr NormalMap* map() {
    CHECK(UsingFullMap());
    return &map_;
  }

  constexpr const NormalMap* map() const {
    CHECK(UsingFullMap());
    return &map_;
  }

 private:
  // When `size_ == kUsingFullMapSentinel`, we have switched storage strategies
  // from `array_[kArraySize] to `NormalMap map_`. See ConvertToRealMap and
  // UsingFullMap.
  size_t size_ = 0u;

  MapInit functor_;

  // We want to call constructors and destructors manually, but we don't want
  // to allocate and deallocate the memory used for them separately. Since
  // array_ and map_ are mutually exclusive, we'll put them in a union.
  using ArrayMap = std::array<ArrayElement, kArraySize>;
  union {
    ArrayMap array_;
    NormalMap map_;
  };

  constexpr span<ArrayElement> sized_array_span() {
    CHECK(!UsingFullMap());
    return span(array_).first(size_);
  }
  constexpr span<const ArrayElement> sized_array_span() const {
    CHECK(!UsingFullMap());
    return span(array_).first(size_);
  }

  constexpr void ConvertToRealMap() {
    CHECK_EQ(size_, kArraySize);

    std::array<ArrayElement, kArraySize> temp_array;

    // Move the current elements into a temporary array.
    for (size_t i = 0u; i < kArraySize; ++i) {
      ArrayElement& e = temp_array[i];
      std::construct_at(std::addressof(e.value), std::move(array_[i].value));
      array_[i].value.~value_type();
    }

    // Make the map active in the union.
    size_ = kUsingFullMapSentinel;
    array_.~ArrayMap();
    functor_(&map_);

    // Insert elements into it.
    for (ArrayElement& e : temp_array) {
      map_.insert(std::move(e.value));
      e.value.~value_type();
    }
  }

  // Helpers for constructors and destructors.
  constexpr void InitEmpty() {
    // Make the array active in the union.
    std::construct_at(&array_);
  }
  constexpr void InitFrom(const small_map& src) {
    functor_ = src.functor_;
    size_ = src.size_;
    if (src.UsingFullMap()) {
      // Make the map active in the union.
      functor_(&map_);
      map_ = src.map_;
    } else {
      // Make the array active in the union.
      std::construct_at(&array_);
      for (size_t i = 0u; i < size_; ++i) {
        ArrayElement& e = array_[i];
        std::construct_at(std::addressof(e.value), src.array_[i].value);
      }
    }
  }

  constexpr void Destroy() {
    if (UsingFullMap()) {
      map_.~NormalMap();
    } else {
      for (size_t i = 0u; i < size_; ++i) {
        array_[i].value.~value_type();
      }
      array_.~ArrayMap();
    }
  }
};

}  // namespace base

#endif  // BASE_CONTAINERS_SMALL_MAP_H_
