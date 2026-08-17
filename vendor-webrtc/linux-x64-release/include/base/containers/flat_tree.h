// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_CONTAINERS_FLAT_TREE_H_
#define BASE_CONTAINERS_FLAT_TREE_H_

#include <algorithm>
#include <array>
#include <compare>
#include <concepts>
#include <functional>
#include <initializer_list>
#include <iterator>
#include <ranges>
#include <type_traits>
#include <utility>

#include "base/check.h"
#include "base/compiler_specific.h"
#include "base/containers/span.h"
#include "base/memory/raw_ptr_exclusion.h"

namespace base {

// Tag type that allows skipping the sort_and_unique step when constructing a
// flat_tree in case the underlying container is already sorted and has no
// duplicate elements.
struct sorted_unique_t {
  constexpr explicit sorted_unique_t() = default;
};
inline constexpr sorted_unique_t sorted_unique;

namespace internal {

// Helper functions used in DCHECKs below to make sure that inputs tagged with
// sorted_unique are indeed sorted and unique.
template <typename Range, typename Comp>
constexpr bool is_sorted_and_unique(const Range& range, Comp comp) {
  // Being unique implies that there are no adjacent elements that
  // compare equal. So this checks that each element is strictly less
  // than the element after it.
  return std::ranges::adjacent_find(range, std::not_fn(comp)) ==
         std::ranges::end(range);
}

// Helper inspired by C++20's std::to_array to convert a C-style array to a
// std::array. As opposed to the C++20 version this implementation does not
// provide an overload for rvalues and does not strip cv qualifers from the
// returned std::array::value_type. The returned value_type needs to be
// specified explicitly, allowing the construction of std::arrays with const
// elements.
//
// Reference: https://en.cppreference.com/w/cpp/container/array/to_array
template <typename U, typename T, size_t N>
  requires(std::constructible_from<U, T>)
constexpr std::array<U, N> ToArray(const T (&data)[N]) {
  auto impl = [&]<size_t... I>(std::index_sequence<I...>) {
    // SAFETY: `impl` is called with `make_index_sequence<N>`, so the largest
    // `I` will be `N - 1`.
    return std::array<U, N>({UNSAFE_BUFFERS(data[I])...});
  };
  return impl(std::make_index_sequence<N>());
}

// Helper that calls `container.reserve(std::size(source))`.
template <typename T, typename U>
void ReserveIfSupported(T& container, const U& source) {
  if constexpr (requires { container.reserve(std::size(source)); }) {
    container.reserve(std::size(source));
  }
}

// Implementation -------------------------------------------------------------

// Implementation for the sorted associative flat_set and flat_map using a
// sorted vector as the backing store. Do not use directly.
//
// The use of "value" in this is like std::map uses, meaning it's the thing
// contained (in the case of map it's a <Key, Mapped> pair). The Key is how
// things are looked up. In the case of a set, Key == Value. In the case of
// a map, the Key is a component of a Value.
//
// The helper class GetKeyFromValue provides the means to extract a key from a
// value for comparison purposes. It should implement:
//   const Key& operator()(const Value&).
template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
class flat_tree {
 public:
  // --------------------------------------------------------------------------
  // Types.
  //
  using key_type = Key;
  using key_compare = KeyCompare;
  using value_type = typename Container::value_type;

  // Wraps the templated key comparison to compare values.
  struct value_compare {
    constexpr bool operator()(const value_type& left,
                              const value_type& right) const {
      GetKeyFromValue extractor;
      return comp(extractor(left), extractor(right));
    }

    NO_UNIQUE_ADDRESS key_compare comp;
  };

  using pointer = typename Container::pointer;
  using const_pointer = typename Container::const_pointer;
  using reference = typename Container::reference;
  using const_reference = typename Container::const_reference;
  using size_type = typename Container::size_type;
  using difference_type = typename Container::difference_type;
  using iterator = typename Container::iterator;
  using const_iterator = typename Container::const_iterator;
  using reverse_iterator = typename Container::reverse_iterator;
  using const_reverse_iterator = typename Container::const_reverse_iterator;
  using container_type = Container;

  // --------------------------------------------------------------------------
  // Lifetime.
  //
  // Constructors that take range guarantee O(N * log^2(N)) + O(N) complexity
  // and take O(N * log(N)) + O(N) if extra memory is available (N is a range
  // length).
  //
  // Assume that move constructors invalidate iterators and references.
  //
  // The constructors that take ranges, lists, and vectors do not require that
  // the input be sorted.
  //
  // When passing the base::sorted_unique tag as the first argument no sort and
  // unique step takes places. This is useful if the underlying container
  // already has the required properties.

  flat_tree() = default;
  flat_tree(const flat_tree&) = default;
  flat_tree(flat_tree&&) = default;

  explicit flat_tree(const key_compare& comp);

  template <class InputIterator>
  flat_tree(InputIterator first,
            InputIterator last,
            const key_compare& comp = key_compare());

  flat_tree(const container_type& items,
            const key_compare& comp = key_compare());

  flat_tree(container_type&& items, const key_compare& comp = key_compare());

  flat_tree(std::initializer_list<value_type> ilist,
            const key_compare& comp = key_compare());

  template <class InputIterator>
  flat_tree(sorted_unique_t,
            InputIterator first,
            InputIterator last,
            const key_compare& comp = key_compare());

  flat_tree(sorted_unique_t,
            const container_type& items,
            const key_compare& comp = key_compare());

  constexpr flat_tree(sorted_unique_t,
                      container_type&& items,
                      const key_compare& comp = key_compare());

  flat_tree(sorted_unique_t,
            std::initializer_list<value_type> ilist,
            const key_compare& comp = key_compare());

  ~flat_tree() = default;

  // --------------------------------------------------------------------------
  // Assignments.
  //
  // Assume that move assignment invalidates iterators and references.

  flat_tree& operator=(const flat_tree&) = default;
  flat_tree& operator=(flat_tree&&) = default;
  // Takes the first if there are duplicates in the initializer list.
  flat_tree& operator=(std::initializer_list<value_type> ilist);

  // --------------------------------------------------------------------------
  // Memory management.
  //
  // Beware that shrink_to_fit() simply forwards the request to the
  // container_type and its implementation is free to optimize otherwise and
  // leave capacity() to be greater that its size.
  //
  // reserve() and shrink_to_fit() invalidate iterators and references.

  void reserve(size_type new_capacity);
  size_type capacity() const;
  void shrink_to_fit();

  // --------------------------------------------------------------------------
  // Size management.
  //
  // clear() leaves the capacity() of the flat_tree unchanged.

  void clear();

  constexpr size_type size() const;
  constexpr size_type max_size() const;
  constexpr bool empty() const;

  // --------------------------------------------------------------------------
  // Iterators.
  //
  // Iterators follow the ordering defined by the key comparator used in
  // construction of the flat_tree.

  iterator begin();
  constexpr const_iterator begin() const;
  const_iterator cbegin() const;

  iterator end();
  constexpr const_iterator end() const;
  const_iterator cend() const;

  reverse_iterator rbegin();
  const_reverse_iterator rbegin() const;
  const_reverse_iterator crbegin() const;

  reverse_iterator rend();
  const_reverse_iterator rend() const;
  const_reverse_iterator crend() const;

  // --------------------------------------------------------------------------
  // Insert operations.
  //
  // Assume that every operation invalidates iterators and references.
  // Insertion of one element can take O(size). Capacity of flat_tree grows in
  // an implementation-defined manner.
  //
  // NOTE: Prefer to build a new flat_tree from a std::vector (or similar)
  // instead of calling insert() repeatedly.

  std::pair<iterator, bool> insert(const value_type& val);
  std::pair<iterator, bool> insert(value_type&& val);

  iterator insert(const_iterator position_hint, const value_type& x);
  iterator insert(const_iterator position_hint, value_type&& x);

  // This method inserts the values from the range [first, last) into the
  // current tree.
  template <class InputIterator>
    requires(std::input_iterator<InputIterator>)
  void insert(InputIterator first, InputIterator last);
  template <class InputIteratorPtr>
  UNSAFE_BUFFER_USAGE void insert(InputIteratorPtr* first,
                                  InputIteratorPtr* last);

  // Inserts the all values from the `range` into the current tree.
  template <class Range>
    requires(std::ranges::input_range<Range>)
  void insert_range(Range&& range);

  template <class... Args>
  std::pair<iterator, bool> emplace(Args&&... args);

  template <class... Args>
  iterator emplace_hint(const_iterator position_hint, Args&&... args);

  // --------------------------------------------------------------------------
  // Underlying type operations.
  //
  // Assume that either operation invalidates iterators and references.

  // Extracts the container_type and returns it to the caller. Ensures that
  // `this` is `empty()` afterwards.
  container_type extract() &&;

  // Replaces the container_type with `body`. Expects that `body` is sorted
  // and has no repeated elements with regard to value_comp().
  void replace(container_type&& body);

  // --------------------------------------------------------------------------
  // Erase operations.
  //
  // Assume that every operation invalidates iterators and references.
  //
  // erase(position), erase(first, last) can take O(size).
  // erase(key) may take O(size) + O(log(size)).
  //
  // Prefer base::EraseIf() or some other variation on erase(remove(), end())
  // idiom when deleting multiple non-consecutive elements.

  iterator erase(iterator position);
  // Artificially templatized to break ambiguity if `iterator` and
  // `const_iterator` are the same type.
  template <typename DummyT = void>
  iterator erase(const_iterator position);
  iterator erase(const_iterator first, const_iterator last);
  size_type erase(const Key& key);
  template <typename K>
  size_type erase(const K& key);

  // --------------------------------------------------------------------------
  // Comparators.

  constexpr key_compare key_comp() const;
  constexpr value_compare value_comp() const;

  // --------------------------------------------------------------------------
  // Search operations.
  //
  // Search operations have O(log(size)) complexity.

  size_type count(const Key& key) const;
  template <typename K>
  size_type count(const K& key) const;

  iterator find(const Key& key);
  const_iterator find(const Key& key) const;
  template <typename K>
  iterator find(const K& key);
  template <typename K>
  const_iterator find(const K& key) const;

  bool contains(const Key& key) const;
  template <typename K>
  bool contains(const K& key) const;

  std::pair<iterator, iterator> equal_range(const Key& key);
  std::pair<const_iterator, const_iterator> equal_range(const Key& key) const;
  template <typename K>
  std::pair<iterator, iterator> equal_range(const K& key);
  template <typename K>
  std::pair<const_iterator, const_iterator> equal_range(const K& key) const;

  iterator lower_bound(const Key& key);
  const_iterator lower_bound(const Key& key) const;
  template <typename K>
  iterator lower_bound(const K& key);
  template <typename K>
  const_iterator lower_bound(const K& key) const;

  iterator upper_bound(const Key& key);
  const_iterator upper_bound(const Key& key) const;
  template <typename K>
  iterator upper_bound(const K& key);
  template <typename K>
  const_iterator upper_bound(const K& key) const;

  // --------------------------------------------------------------------------
  // General operations.
  //
  // Assume that swap invalidates iterators and references.
  //
  // Implementation note: currently we use operator==() and operator<() on
  // std::vector, because they have the same contract we need, so we use them
  // directly for brevity and in case it is more optimal than calling equal()
  // and lexicograhpical_compare(). If the underlying container type is changed,
  // this code may need to be modified.

  void swap(flat_tree& other) noexcept;

  friend bool operator==(const flat_tree& lhs, const flat_tree& rhs) {
    return lhs.body_ == rhs.body_;
  }

  friend auto operator<=>(const flat_tree& lhs, const flat_tree& rhs) {
    return lhs.body_ <=> rhs.body_;
  }

  friend void swap(flat_tree& lhs, flat_tree& rhs) noexcept { lhs.swap(rhs); }

 protected:
  // Emplaces a new item into the tree that is known not to be in it. This
  // is for implementing map operator[].
  template <class... Args>
  iterator unsafe_emplace(const_iterator position, Args&&... args);

  // Attempts to emplace a new element with key |key|. Only if |key| is not yet
  // present, construct value_type from |args| and insert it. Returns an
  // iterator to the element with key |key| and a bool indicating whether an
  // insertion happened.
  template <class K, class... Args>
  std::pair<iterator, bool> emplace_key_args(const K& key, Args&&... args);

  // Similar to |emplace_key_args|, but checks |hint| first as a possible
  // insertion position.
  template <class K, class... Args>
  std::pair<iterator, bool> emplace_hint_key_args(const_iterator hint,
                                                  const K& key,
                                                  Args&&... args);

 private:
  // Helper class for e.g. lower_bound that can compare a value on the left
  // to a key on the right.
  struct KeyValueCompare {
    // The key comparison object must outlive this class.
    explicit KeyValueCompare(const key_compare& comp) : comp_(comp) {}

    template <typename T, typename U>
    bool operator()(const T& lhs, const U& rhs) const {
      return comp_(extract_if_value_type(lhs), extract_if_value_type(rhs));
    }

   private:
    const key_type& extract_if_value_type(const value_type& v) const {
      GetKeyFromValue extractor;
      return extractor(v);
    }

    template <typename K>
    const K& extract_if_value_type(const K& k) const {
      return k;
    }
    // RAW_PTR_EXCLUSION: Binary size increase. There's also little value to
    // rewriting this member as it points to `flat_tree::comp_` and flat_tree
    // itself should be holding raw_ptr/raw_ref if necessary.
    RAW_PTR_EXCLUSION const key_compare& comp_;
  };

  iterator const_cast_it(const_iterator c_it) {
    auto distance = std::distance(cbegin(), c_it);
    return std::next(begin(), distance);
  }

  // This method is inspired by both std::map::insert(P&&) and
  // std::map::insert_or_assign(const K&, V&&). It inserts val if an equivalent
  // element is not present yet, otherwise it overwrites. It returns an iterator
  // to the modified element and a flag indicating whether insertion or
  // assignment happened.
  template <class V>
  std::pair<iterator, bool> insert_or_assign(V&& val) {
    auto position = lower_bound(GetKeyFromValue()(val));

    if (position == end() || value_comp()(val, *position)) {
      return {body_.emplace(position, std::forward<V>(val)), true};
    }

    *position = std::forward<V>(val);
    return {position, false};
  }

  // This method is similar to insert_or_assign, with the following differences:
  // - Instead of searching [begin(), end()) it only searches [first, last).
  // - In case no equivalent element is found, val is appended to the end of the
  //   underlying body and an iterator to the next bigger element in [first,
  //   last) is returned.
  template <class V>
  std::pair<iterator, bool> append_or_assign(iterator first,
                                             iterator last,
                                             V&& val) {
    auto position = std::lower_bound(first, last, val, value_comp());

    if (position == last || value_comp()(val, *position)) {
      // emplace_back might invalidate position, which is why distance needs to
      // be cached.
      const difference_type distance = std::distance(begin(), position);
      body_.emplace_back(std::forward<V>(val));
      return {std::next(begin(), distance), true};
    }

    *position = std::forward<V>(val);
    return {position, false};
  }

  // This method is similar to insert, with the following differences:
  // - Instead of searching [begin(), end()) it only searches [first, last).
  // - In case no equivalent element is found, val is appended to the end of the
  //   underlying body and an iterator to the next bigger element in [first,
  //   last) is returned.
  template <class V>
  std::pair<iterator, bool> append_unique(iterator first,
                                          iterator last,
                                          V&& val) {
    auto position = std::lower_bound(first, last, val, value_comp());

    if (position == last || value_comp()(val, *position)) {
      // emplace_back might invalidate position, which is why distance needs to
      // be cached.
      const difference_type distance = std::distance(begin(), position);
      body_.emplace_back(std::forward<V>(val));
      return {std::next(begin(), distance), true};
    }

    return {position, false};
  }

  void sort_and_unique(iterator first, iterator last) {
    // Preserve stability for the unique code below.
    std::stable_sort(first, last, value_comp());

    // lhs is already <= rhs due to sort, therefore !(lhs < rhs) <=> lhs == rhs.
    auto equal_comp = std::not_fn(value_comp());
    erase(std::unique(first, last, equal_comp), last);
  }

  void sort_and_unique() { sort_and_unique(begin(), end()); }

  // To support comparators that may not be possible to default-construct, we
  // have to store an instance of Compare. Since Compare commonly is stateless,
  // we use the NO_UNIQUE_ADDRESS attribute to save space.
  NO_UNIQUE_ADDRESS key_compare comp_;
  // Declare after |key_compare_comp_| to workaround GCC ICE. For details
  // see https://crbug.com/1156268
  container_type body_;

  // If the compare is not transparent we want to construct key_type once.
  template <typename K>
  using KeyTypeOrK = std::conditional_t<requires {
    typename key_compare::is_transparent;
  }, K, key_type>;
};

// ----------------------------------------------------------------------------
// Lifetime.

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::flat_tree(
    const KeyCompare& comp)
    : comp_(comp) {}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
template <class InputIterator>
flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::flat_tree(
    InputIterator first,
    InputIterator last,
    const KeyCompare& comp)
    : comp_(comp), body_(first, last) {
  sort_and_unique();
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::flat_tree(
    const container_type& items,
    const KeyCompare& comp)
    : comp_(comp), body_(items) {
  sort_and_unique();
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::flat_tree(
    container_type&& items,
    const KeyCompare& comp)
    : comp_(comp), body_(std::move(items)) {
  sort_and_unique();
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::flat_tree(
    std::initializer_list<value_type> ilist,
    const KeyCompare& comp)
    : flat_tree(std::ranges::begin(ilist), std::ranges::end(ilist), comp) {}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
template <class InputIterator>
flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::flat_tree(
    sorted_unique_t,
    InputIterator first,
    InputIterator last,
    const KeyCompare& comp)
    : comp_(comp), body_(first, last) {
  DCHECK(is_sorted_and_unique(*this, value_comp()));
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::flat_tree(
    sorted_unique_t,
    const container_type& items,
    const KeyCompare& comp)
    : comp_(comp), body_(items) {
  DCHECK(is_sorted_and_unique(*this, value_comp()));
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
constexpr flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::flat_tree(
    sorted_unique_t,
    container_type&& items,
    const KeyCompare& comp)
    : comp_(comp), body_(std::move(items)) {
  DCHECK(is_sorted_and_unique(*this, value_comp()));
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::flat_tree(
    sorted_unique_t,
    std::initializer_list<value_type> ilist,
    const KeyCompare& comp)
    : flat_tree(sorted_unique,
                std::ranges::begin(ilist),
                std::ranges::end(ilist),
                comp) {}

// ----------------------------------------------------------------------------
// Assignments.

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::operator=(
    std::initializer_list<value_type> ilist) -> flat_tree& {
  body_ = ilist;
  sort_and_unique();
  return *this;
}

// ----------------------------------------------------------------------------
// Memory management.

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
void flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::reserve(
    size_type new_capacity) {
  body_.reserve(new_capacity);
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::capacity() const
    -> size_type {
  return body_.capacity();
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
void flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::shrink_to_fit() {
  body_.shrink_to_fit();
}

// ----------------------------------------------------------------------------
// Size management.

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
void flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::clear() {
  body_.clear();
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
constexpr auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::size()
    const -> size_type {
  return body_.size();
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
constexpr auto
flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::max_size() const
    -> size_type {
  return body_.max_size();
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
constexpr bool flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::empty()
    const {
  return body_.empty();
}

// ----------------------------------------------------------------------------
// Iterators.

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::begin()
    -> iterator {
  return body_.begin();
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
constexpr auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::begin()
    const -> const_iterator {
  return std::ranges::begin(body_);
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::cbegin() const
    -> const_iterator {
  return body_.cbegin();
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::end() -> iterator {
  return body_.end();
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
constexpr auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::end()
    const -> const_iterator {
  return std::ranges::end(body_);
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::cend() const
    -> const_iterator {
  return body_.cend();
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::rbegin()
    -> reverse_iterator {
  return body_.rbegin();
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::rbegin() const
    -> const_reverse_iterator {
  return body_.rbegin();
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::crbegin() const
    -> const_reverse_iterator {
  return body_.crbegin();
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::rend()
    -> reverse_iterator {
  return body_.rend();
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::rend() const
    -> const_reverse_iterator {
  return body_.rend();
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::crend() const
    -> const_reverse_iterator {
  return body_.crend();
}

// ----------------------------------------------------------------------------
// Insert operations.
//
// Currently we use position_hint the same way as eastl or boost:
// https://github.com/electronicarts/EASTL/blob/master/include/EASTL/vector_set.h#L493

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::insert(
    const value_type& val) -> std::pair<iterator, bool> {
  return emplace_key_args(GetKeyFromValue()(val), val);
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::insert(
    value_type&& val) -> std::pair<iterator, bool> {
  return emplace_key_args(GetKeyFromValue()(val), std::move(val));
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::insert(
    const_iterator position_hint,
    const value_type& val) -> iterator {
  return emplace_hint_key_args(position_hint, GetKeyFromValue()(val), val)
      .first;
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::insert(
    const_iterator position_hint,
    value_type&& val) -> iterator {
  return emplace_hint_key_args(position_hint, GetKeyFromValue()(val),
                               std::move(val))
      .first;
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
template <class InputIteratorPtr>
UNSAFE_BUFFER_USAGE void
flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::insert(
    InputIteratorPtr* input_begin,
    InputIteratorPtr* input_end) {
  // SAFETY: The caller must ensure the pointers are a valid pair.
  auto s = UNSAFE_BUFFERS(base::span(input_begin, input_end));
  insert(s.begin(), s.end());
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
template <class InputIterator>
  requires(std::input_iterator<InputIterator>)
void flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::insert(
    InputIterator input_begin,
    InputIterator input_end) {
  if (input_begin == input_end) {
    return;
  }

  // Dispatch to single element insert if the input range contains a single
  // element.
  if (std::next(input_begin) == input_end) {
    insert(end(), *input_begin);
    return;
  }

  // Provide a convenience lambda to obtain an iterator pointing past the last
  // old element. This needs to be dymanic due to possible re-allocations.
  auto prior_end = [this, size = size()] {
    return std::next(begin(), static_cast<difference_type>(size));
  };

  // For batch updates initialize the first insertion point.
  auto pos_first_new = static_cast<difference_type>(size());

  // Loop over the input range while appending new values and overwriting
  // existing ones, if applicable. Keep track of the first insertion point.
  for (auto it = input_begin; it != input_end; ++it) {
    auto [inserted_at, inserted] = append_unique(begin(), prior_end(), *it);
    if (inserted) {
      pos_first_new =
          std::min(pos_first_new, std::distance(begin(), inserted_at));
    }
  }

  // The new elements might be unordered and contain duplicates, so post-process
  // the just inserted elements and merge them with the rest, inserting them at
  // the previously found spot.
  sort_and_unique(prior_end(), end());
  std::inplace_merge(std::next(begin(), pos_first_new), prior_end(), end(),
                     value_comp());
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
template <class Range>
  requires(std::ranges::input_range<Range>)
void flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::insert_range(
    Range&& range) {
  // SAFETY: A range should return a valid begin/end even if they are pointers.
  UNSAFE_BUFFERS(insert(std::ranges::begin(range), std::ranges::end(range)));
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
template <class... Args>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::emplace(
    Args&&... args) -> std::pair<iterator, bool> {
  return insert(value_type(std::forward<Args>(args)...));
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
template <class... Args>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::emplace_hint(
    const_iterator position_hint,
    Args&&... args) -> iterator {
  return insert(position_hint, value_type(std::forward<Args>(args)...));
}

// ----------------------------------------------------------------------------
// Underlying type operations.

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::
    extract() && -> container_type {
  return std::exchange(body_, container_type());
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
void flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::replace(
    container_type&& body) {
  // Ensure that `body` is sorted and has no repeated elements according to
  // `value_comp()`.
  DCHECK(is_sorted_and_unique(body, value_comp()));
  body_ = std::move(body);
}

// ----------------------------------------------------------------------------
// Erase operations.

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::erase(
    iterator position) -> iterator {
  CHECK(position != body_.end());
  return body_.erase(position);
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
template <typename DummyT>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::erase(
    const_iterator position) -> iterator {
  CHECK(position != body_.end());
  return body_.erase(position);
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::erase(
    const Key& val) -> size_type {
  auto eq_range = equal_range(val);
  auto res =
      static_cast<size_type>(std::distance(eq_range.first, eq_range.second));
  erase(eq_range.first, eq_range.second);
  return res;
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
template <typename K>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::erase(const K& val)
    -> size_type {
  auto eq_range = equal_range(val);
  auto res =
      static_cast<size_type>(std::distance(eq_range.first, eq_range.second));
  erase(eq_range.first, eq_range.second);
  return res;
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::erase(
    const_iterator first,
    const_iterator last) -> iterator {
  return body_.erase(first, last);
}

// ----------------------------------------------------------------------------
// Comparators.

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
constexpr auto
flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::key_comp() const
    -> key_compare {
  return comp_;
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
constexpr auto
flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::value_comp() const
    -> value_compare {
  return value_compare{comp_};
}

// ----------------------------------------------------------------------------
// Search operations.

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
template <typename K>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::count(
    const K& key) const -> size_type {
  auto eq_range = equal_range(key);
  return static_cast<size_type>(std::distance(eq_range.first, eq_range.second));
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::count(
    const Key& key) const -> size_type {
  auto eq_range = equal_range(key);
  return static_cast<size_type>(std::distance(eq_range.first, eq_range.second));
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::find(
    const Key& key) -> iterator {
  return const_cast_it(std::as_const(*this).find(key));
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::find(
    const Key& key) const -> const_iterator {
  auto eq_range = equal_range(key);
  return (eq_range.first == eq_range.second) ? end() : eq_range.first;
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
template <typename K>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::find(const K& key)
    -> iterator {
  return const_cast_it(std::as_const(*this).find(key));
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
template <typename K>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::find(
    const K& key) const -> const_iterator {
  auto eq_range = equal_range(key);
  return (eq_range.first == eq_range.second) ? end() : eq_range.first;
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
bool flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::contains(
    const Key& key) const {
  auto lower = lower_bound(key);
  return lower != end() && !comp_(key, GetKeyFromValue()(*lower));
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
template <typename K>
bool flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::contains(
    const K& key) const {
  auto lower = lower_bound(key);
  return lower != end() && !comp_(key, GetKeyFromValue()(*lower));
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::equal_range(
    const Key& key) -> std::pair<iterator, iterator> {
  auto res = std::as_const(*this).equal_range(key);
  return {const_cast_it(res.first), const_cast_it(res.second)};
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::equal_range(
    const Key& key) const -> std::pair<const_iterator, const_iterator> {
  auto lower = lower_bound(key);

  KeyValueCompare comp(comp_);
  if (lower == end() || comp(key, *lower)) {
    return {lower, lower};
  }

  return {lower, std::next(lower)};
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
template <typename K>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::equal_range(
    const K& key) -> std::pair<iterator, iterator> {
  auto res = std::as_const(*this).equal_range(key);
  return {const_cast_it(res.first), const_cast_it(res.second)};
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
template <typename K>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::equal_range(
    const K& key) const -> std::pair<const_iterator, const_iterator> {
  auto lower = lower_bound(key);

  KeyValueCompare comp(comp_);
  if (lower == end() || comp(key, *lower)) {
    return {lower, lower};
  }

  return {lower, std::next(lower)};
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::lower_bound(
    const Key& key) -> iterator {
  return const_cast_it(std::as_const(*this).lower_bound(key));
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::lower_bound(
    const Key& key) const -> const_iterator {
  KeyValueCompare comp(comp_);
  return std::ranges::lower_bound(*this, key, comp);
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
template <typename K>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::lower_bound(
    const K& key) -> iterator {
  return const_cast_it(std::as_const(*this).lower_bound(key));
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
template <typename K>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::lower_bound(
    const K& key) const -> const_iterator {
  static_assert(std::is_convertible_v<const KeyTypeOrK<K>&, const K&>,
                "Requested type cannot be bound to the container's key_type "
                "which is required for a non-transparent compare.");

  const KeyTypeOrK<K>& key_ref = key;

  KeyValueCompare comp(comp_);
  return std::ranges::lower_bound(*this, key_ref, comp);
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::upper_bound(
    const Key& key) -> iterator {
  return const_cast_it(std::as_const(*this).upper_bound(key));
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::upper_bound(
    const Key& key) const -> const_iterator {
  KeyValueCompare comp(comp_);
  return std::ranges::upper_bound(*this, key, comp);
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
template <typename K>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::upper_bound(
    const K& key) -> iterator {
  return const_cast_it(std::as_const(*this).upper_bound(key));
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
template <typename K>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::upper_bound(
    const K& key) const -> const_iterator {
  static_assert(std::is_convertible_v<const KeyTypeOrK<K>&, const K&>,
                "Requested type cannot be bound to the container's key_type "
                "which is required for a non-transparent compare.");

  const KeyTypeOrK<K>& key_ref = key;

  KeyValueCompare comp(comp_);
  return std::ranges::upper_bound(*this, key_ref, comp);
}

// ----------------------------------------------------------------------------
// General operations.

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
void flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::swap(
    flat_tree& other) noexcept {
  std::swap(*this, other);
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
template <class... Args>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::unsafe_emplace(
    const_iterator position,
    Args&&... args) -> iterator {
  return body_.emplace(position, std::forward<Args>(args)...);
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
template <class K, class... Args>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::emplace_key_args(
    const K& key,
    Args&&... args) -> std::pair<iterator, bool> {
  auto lower = lower_bound(key);
  if (lower == end() || comp_(key, GetKeyFromValue()(*lower))) {
    return {unsafe_emplace(lower, std::forward<Args>(args)...), true};
  }
  return {lower, false};
}

template <class Key, class GetKeyFromValue, class KeyCompare, class Container>
template <class K, class... Args>
auto flat_tree<Key, GetKeyFromValue, KeyCompare, Container>::
    emplace_hint_key_args(const_iterator hint, const K& key, Args&&... args)
        -> std::pair<iterator, bool> {
  KeyValueCompare comp(comp_);
  if ((hint == begin() || comp(*std::prev(hint), key))) {
    if (hint == end() || comp(key, *hint)) {
      // *(hint - 1) < key < *hint => key did not exist and hint is correct.
      return {unsafe_emplace(hint, std::forward<Args>(args)...), true};
    }
    if (!comp(*hint, key)) {
      // key == *hint => no-op, return correct hint.
      return {const_cast_it(hint), false};
    }
  }
  // hint was not helpful, dispatch to hintless version.
  return emplace_key_args(key, std::forward<Args>(args)...);
}

}  // namespace internal

// ----------------------------------------------------------------------------
// Free functions.

// Erases all elements that match predicate. It has O(size) complexity.
template <class Key,
          class GetKeyFromValue,
          class KeyCompare,
          class Container,
          typename Predicate>
size_t EraseIf(
    base::internal::flat_tree<Key, GetKeyFromValue, KeyCompare, Container>&
        container,
    Predicate pred) {
  auto removed = std::ranges::remove_if(container, pred);
  size_t num_removed = removed.size();
  container.erase(removed.begin(), removed.end());
  return num_removed;
}

}  // namespace base

#endif  // BASE_CONTAINERS_FLAT_TREE_H_
