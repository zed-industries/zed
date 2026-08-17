// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_CONTAINERS_CIRCULAR_DEQUE_H_
#define BASE_CONTAINERS_CIRCULAR_DEQUE_H_

#include <algorithm>
#include <cstddef>
#include <iterator>
#include <utility>

#include "base/check.h"
#include "base/containers/span.h"
#include "base/containers/vector_buffer.h"
#include "base/dcheck_is_on.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/numerics/checked_math.h"
#include "base/numerics/safe_conversions.h"
#include "base/types/cxx23_from_range.h"

#if DCHECK_IS_ON()
#include <ostream>
#endif

// base::circular_deque is similar to std::deque. Unlike std::deque, the
// storage is provided in a flat circular buffer conceptually similar to a
// vector. The beginning and end will wrap around as necessary so that
// pushes and pops will be constant time as long as a capacity expansion is
// not required.
//
// The API should be identical to std::deque with the following differences:
//
//  - ITERATORS ARE NOT STABLE. Mutating the container will invalidate all
//    iterators.
//
//  - Insertions may resize the vector and so are not constant time (std::deque
//    guarantees constant time for insertions at the ends).
//
//  - Container-wide comparisons are not implemented. If you want to compare
//    two containers, use an algorithm so the expensive iteration is explicit.
//
// If you want a similar container with only a queue API, use base::queue in
// base/containers/queue.h.
//
// Constructors:
//   circular_deque();
//   circular_deque(size_t count);
//   circular_deque(size_t count, const T& value);
//   circular_deque(InputIterator first, InputIterator last);
//   circular_deque(base::from_range_t, Range range);
//   circular_deque(const circular_deque&);
//   circular_deque(circular_deque&&);
//   circular_deque(std::initializer_list<value_type>);
//
// Assignment functions:
//   circular_deque& operator=(const circular_deque&);
//   circular_deque& operator=(circular_deque&&);
//   circular_deque& operator=(std::initializer_list<T>);
//   void assign(size_t count, const T& value);
//   void assign(InputIterator first, InputIterator last);
//   void assign(std::initializer_list<T> value);
//   void assign_range(Range range);
//
// Random accessors:
//   T& at(size_t);
//   const T& at(size_t) const;
//   T& operator[](size_t);
//   const T& operator[](size_t) const;
//
// End accessors:
//   T& front();
//   const T& front() const;
//   T& back();
//   const T& back() const;
//
// Iterator functions:
//   iterator               begin();
//   const_iterator         begin() const;
//   const_iterator         cbegin() const;
//   iterator               end();
//   const_iterator         end() const;
//   const_iterator         cend() const;
//   reverse_iterator       rbegin();
//   const_reverse_iterator rbegin() const;
//   const_reverse_iterator crbegin() const;
//   reverse_iterator       rend();
//   const_reverse_iterator rend() const;
//   const_reverse_iterator crend() const;
//
// Memory management:
//   void reserve(size_t);  // SEE IMPLEMENTATION FOR SOME GOTCHAS.
//   size_t capacity() const;
//   void shrink_to_fit();
//
// Size management:
//   void clear();
//   bool empty() const;
//   size_t size() const;
//   void resize(size_t);
//   void resize(size_t count, const T& value);
//
// Positional insert and erase:
//   void insert(const_iterator pos, size_type count, const T& value);
//   void insert(const_iterator pos,
//               InputIterator first, InputIterator last);
//   iterator insert(const_iterator pos, const T& value);
//   iterator insert(const_iterator pos, T&& value);
//   iterator emplace(const_iterator pos, Args&&... args);
//   iterator erase(const_iterator pos);
//   iterator erase(const_iterator first, const_iterator last);
//
// End insert and erase:
//   void push_front(const T&);
//   void push_front(T&&);
//   void push_back(const T&);
//   void push_back(T&&);
//   T& emplace_front(Args&&...);
//   T& emplace_back(Args&&...);
//   void pop_front();
//   void pop_back();
//
// General:
//   void swap(circular_deque&);

namespace base {

template <class T>
class circular_deque;

namespace internal {

// Start allocating nonempty buffers with this many entries. This is the
// external capacity so the internal buffer will be one larger (= 4) which is
// more even for the allocator. See the descriptions of internal vs. external
// capacity on the comment above the buffer_ variable below.
constexpr size_t kCircularBufferInitialCapacity = 3;

template <typename T>
class circular_deque_const_iterator {
 public:
  using difference_type = ptrdiff_t;
  using value_type = T;
  using pointer = const T*;
  using reference = const T&;
  using iterator_category = std::random_access_iterator_tag;

  circular_deque_const_iterator() = default;

  // Dereferencing.
  const T& operator*() const {
    CHECK_NE(index_, end_);
    CheckUnstableUsage();
    CheckValidIndex(index_);
    // SAFETY: Increment() and Decrement() and Add() operations ensure that
    // `index_` stays inside [begin_, end_] (while supporting wrap around for
    // the structure. This maintains that `index_` always points at a
    // valid position for the `buffer_`. We also CHECK above that `index_` is
    // not `end_` making it a valid pointer to dereference.
    return UNSAFE_BUFFERS(buffer_[index_]);
  }
  const T* operator->() const {
    CHECK_NE(index_, end_);
    CheckUnstableUsage();
    CheckValidIndex(index_);
    // SAFETY: Increment() and Decrement() and Add() operations ensure that
    // `index_` stays inside [begin_, end_] while supporting wrap around for
    // the structure. This maintains that `index_` always points at a
    // valid position for the `buffer_`. We also CHECK above that `index_` is
    // not `end_` making it a valid pointer to dereference.
    return &UNSAFE_BUFFERS(buffer_[index_]);
  }
  const value_type& operator[](difference_type i) const { return *(*this + i); }

  // Increment and decrement.
  circular_deque_const_iterator& operator++() {
    Increment();
    return *this;
  }
  circular_deque_const_iterator operator++(int) {
    circular_deque_const_iterator ret = *this;
    Increment();
    return ret;
  }
  circular_deque_const_iterator& operator--() {
    Decrement();
    return *this;
  }
  circular_deque_const_iterator operator--(int) {
    circular_deque_const_iterator ret = *this;
    Decrement();
    return ret;
  }

  // Random access mutation.
  friend circular_deque_const_iterator operator+(
      const circular_deque_const_iterator& iter,
      difference_type offset) {
    circular_deque_const_iterator ret = iter;
    ret.Add(offset);
    return ret;
  }
  circular_deque_const_iterator& operator+=(difference_type offset) {
    Add(offset);
    return *this;
  }
  friend circular_deque_const_iterator operator-(
      const circular_deque_const_iterator& iter,
      difference_type offset) {
    circular_deque_const_iterator ret = iter;
    ret.Add(-offset);
    return ret;
  }
  circular_deque_const_iterator& operator-=(difference_type offset) {
    Add(-offset);
    return *this;
  }

  friend std::ptrdiff_t operator-(const circular_deque_const_iterator& lhs,
                                  const circular_deque_const_iterator& rhs) {
    lhs.CheckComparable(rhs);
    return static_cast<std::ptrdiff_t>(lhs.OffsetFromBegin() -
                                       rhs.OffsetFromBegin());
  }

  // Comparisons.
  friend bool operator==(const circular_deque_const_iterator& lhs,
                         const circular_deque_const_iterator& rhs) {
    lhs.CheckComparable(rhs);
    return lhs.index_ == rhs.index_;
  }
  friend std::strong_ordering operator<=>(
      const circular_deque_const_iterator& lhs,
      const circular_deque_const_iterator& rhs) {
    lhs.CheckComparable(rhs);
    // The order is based on the position of the element in the circular_dequeue
    // rather than `index_` at which the element is stored in the ring buffer.
    return lhs.OffsetFromBegin() <=> rhs.OffsetFromBegin();
  }

 protected:
  friend class circular_deque<T>;

  circular_deque_const_iterator(const circular_deque<T>* parent, size_t index)
      : buffer_(parent->buffer_.data()),
        cap_(parent->buffer_.capacity()),
        begin_(parent->begin_),
        end_(parent->end_),
        index_(index) {
    if (begin_ <= end_) {
      CHECK_GE(index_, begin_);
      CHECK_LE(index_, end_);
    } else if (index_ >= begin_) {
      CHECK(index_ < cap_);
    } else {
      CHECK(index_ <= end_);
    }
#if DCHECK_IS_ON()
    parent_deque_ = parent;
    created_generation_ = parent->generation_;
#endif  // DCHECK_IS_ON()
  }

  // Returns the offset from the beginning index of the buffer to the current
  // item.
  size_t OffsetFromBegin() const {
    if (index_ >= begin_) {
      return index_ - begin_;  // On the same side as begin.
    }
    return cap_ - begin_ + index_;
  }

  // The size of the deque, ie. the number of elements in it.
  size_t Size() const {
    if (begin_ <= end_) {
      return end_ - begin_;
    }
    return cap_ - begin_ + end_;
  }

  // Most uses will be ++ and -- so use a simplified implementation.
  void Increment() {
    CheckUnstableUsage();
    CheckValidIndex(index_);
    CHECK_NE(index_, end_);
    index_++;
    if (index_ == cap_) {
      index_ = 0u;
    }
  }
  void Decrement() {
    CheckUnstableUsage();
    CheckValidIndexOrEnd(index_);
    CHECK_NE(index_, begin_);
    if (index_ == 0u) {
      index_ = cap_ - 1u;
    } else {
      index_--;
    }
  }
  void Add(difference_type delta) {
    CheckUnstableUsage();
#if DCHECK_IS_ON()
    if (delta <= 0) {
      CheckValidIndexOrEnd(index_);
    } else {
      CheckValidIndex(index_);
    }
#endif
    // It should be valid to add 0 to any iterator, even if the container is
    // empty and the iterator points to end(). The modulo below will divide
    // by 0 if the buffer capacity is empty, so it's important to check for
    // this case explicitly.
    if (delta == 0) {
      return;
    }

    const auto offset_from_begin =
        // The max allocation size is PTRDIFF_MAX, so this value can't be larger
        // than fits in ptrdiff_t.
        static_cast<difference_type>(OffsetFromBegin());
    const auto deque_size =
        // The max allocation size is PTRDIFF_MAX, so this value can't be larger
        // than fits in ptrdiff_t.
        static_cast<difference_type>(Size());
    if (delta >= 0) {
      // Check `offset_from_begin + delta <= deque_size` without overflowing.
      CHECK_LE(delta, deque_size - offset_from_begin);
    } else {
      // Check `offset_from_begin + delta >= 0` without overflowing. We avoid
      // negating a negative `delta` which can overflow. Instead negate the
      // positive number which can not.
      CHECK_GE(delta, -offset_from_begin) << offset_from_begin;
    }
    const auto new_offset =
        // The above checks verify that `offset_from_begin + delta` is in the
        // range [0, deque_size] and does not overflow, so it also fits in
        // `size_t`.
        static_cast<size_t>(offset_from_begin + delta);
    index_ = (new_offset + begin_) % cap_;
  }

#if DCHECK_IS_ON()
  void CheckValidIndexOrEnd(size_t index) const {
    parent_deque_->CheckValidIndexOrEnd(index_);
  }
  void CheckValidIndex(size_t index) const {
    parent_deque_->CheckValidIndex(index_);
  }
  void CheckUnstableUsage() const {
    DCHECK(parent_deque_);
    // Since circular_deque doesn't guarantee stability, any attempt to
    // dereference this iterator after a mutation (i.e. the generation doesn't
    // match the original) in the container is illegal.
    DCHECK_EQ(created_generation_, parent_deque_->generation_)
        << "circular_deque iterator dereferenced after mutation.";
  }
  void CheckComparable(const circular_deque_const_iterator& other) const {
    DCHECK_EQ(parent_deque_, other.parent_deque_);
    // Since circular_deque doesn't guarantee stability, two iterators that
    // are compared must have been generated without mutating the container.
    // If this fires, the container was mutated between generating the two
    // iterators being compared.
    DCHECK_EQ(created_generation_, other.created_generation_);
  }
#else
  inline void CheckUnstableUsage() const {}
  inline void CheckComparable(const circular_deque_const_iterator&) const {}
  void CheckValidIndexOrEnd(size_t index) const {}
  void CheckValidIndex(size_t index) const {}
#endif  // DCHECK_IS_ON()

  // `buffer_` is not a raw_ptr<...> for performance reasons: Usually
  // on-stack pointer, pointing back to the collection being iterated, owned by
  // object that iterates over it.  Additionally this is supported by the
  // analysis of sampling profiler data and tab_search:top100:2020.
  RAW_PTR_EXCLUSION const T* buffer_ = nullptr;

  size_t cap_ = 0u;
  size_t begin_ = 0u;
  size_t end_ = 0u;
  size_t index_ = 0u;

#if DCHECK_IS_ON()
  RAW_PTR_EXCLUSION const circular_deque<T>* parent_deque_ = nullptr;
  // The generation of the parent deque when this iterator was created. The
  // container will update the generation for every modification so we can
  // test if the container was modified by comparing them.
  uint64_t created_generation_ = 0u;
#endif  // DCHECK_IS_ON()
};

template <typename T>
class circular_deque_iterator : public circular_deque_const_iterator<T> {
  using base = circular_deque_const_iterator<T>;

 public:
  friend class circular_deque<T>;

  using difference_type = std::ptrdiff_t;
  using value_type = T;
  using pointer = T*;
  using reference = T&;
  using iterator_category = std::random_access_iterator_tag;

  // Expose the base class' constructor.
  circular_deque_iterator() : circular_deque_const_iterator<T>() {}

  // Dereferencing.
  T& operator*() const { return const_cast<T&>(base::operator*()); }
  T* operator->() const { return const_cast<T*>(base::operator->()); }
  T& operator[](difference_type i) {
    return const_cast<T&>(base::operator[](i));
  }

  // Random access mutation.
  friend circular_deque_iterator operator+(const circular_deque_iterator& iter,
                                           difference_type offset) {
    circular_deque_iterator ret = iter;
    ret.Add(offset);
    return ret;
  }
  circular_deque_iterator& operator+=(difference_type offset) {
    base::Add(offset);
    return *this;
  }
  friend circular_deque_iterator operator-(const circular_deque_iterator& iter,
                                           difference_type offset) {
    circular_deque_iterator ret = iter;
    ret.Add(-offset);
    return ret;
  }
  circular_deque_iterator& operator-=(difference_type offset) {
    base::Add(-offset);
    return *this;
  }

  // Increment and decrement.
  circular_deque_iterator& operator++() {
    base::Increment();
    return *this;
  }
  circular_deque_iterator operator++(int) {
    circular_deque_iterator ret = *this;
    base::Increment();
    return ret;
  }
  circular_deque_iterator& operator--() {
    base::Decrement();
    return *this;
  }
  circular_deque_iterator operator--(int) {
    circular_deque_iterator ret = *this;
    base::Decrement();
    return ret;
  }

 private:
  circular_deque_iterator(const circular_deque<T>* parent, size_t index)
      : circular_deque_const_iterator<T>(parent, index) {}
};

}  // namespace internal

template <typename T>
class circular_deque {
 private:
  using VectorBuffer = internal::VectorBuffer<T>;

 public:
  using value_type = T;
  using size_type = size_t;
  using difference_type = std::ptrdiff_t;
  using reference = value_type&;
  using const_reference = const value_type&;
  using pointer = value_type*;
  using const_pointer = const value_type*;

  using iterator = internal::circular_deque_iterator<T>;
  using const_iterator = internal::circular_deque_const_iterator<T>;
  using reverse_iterator = std::reverse_iterator<iterator>;
  using const_reverse_iterator = std::reverse_iterator<const_iterator>;

  // ---------------------------------------------------------------------------
  // Constructor

  // Constructs an empty deque.
  constexpr circular_deque() = default;

  // Constructs with `count` copies of a default-constructed T.
  explicit circular_deque(size_type count) { resize(count); }

  // Constructs with `count` copies of `value`.
  circular_deque(size_type count, const T& value) { resize(count, value); }

  // Construct a deque by constructing its elements from each element in
  // `[first, last)`.
  //
  // Prefer using the `from_range_t` constructor, which builds a deque from a
  // range, instead of from problematic iterator pairs.
  //
  // # Safety
  // The `first` and `last` iterators must be from the same container, with
  // `first <= last`.
  template <class InputIterator>
    requires(std::input_iterator<InputIterator>)
  UNSAFE_BUFFER_USAGE circular_deque(InputIterator first, InputIterator last)
      : circular_deque() {
    // SAFETY: The caller is responsible for giving iterator from the same
    // container.
    UNSAFE_BUFFERS(assign(first, last));
  }

  // Constructs a deque from the elements in a range (a container or span),
  // typically by copy-constructing if the range also holds objects of type
  // `T`.
  //
  // Example:
  // ```
  // int values[] = {1, 3};
  // circular_deque<int> deq(base::from_range, values);
  // ```
  template <typename Range>
    requires(std::ranges::input_range<Range>)
  circular_deque(base::from_range_t, Range&& value) : circular_deque() {
    assign_range(std::forward<Range>(value));
  }

  // Copy/move.
  circular_deque(const circular_deque& other) : buffer_(other.size() + 1) {
    assign_range(other);
  }
  circular_deque(circular_deque&& other) noexcept
      : buffer_(std::move(other.buffer_)),
        begin_(std::exchange(other.begin_, 0u)),
        end_(std::exchange(other.end_, 0u)) {}

  circular_deque(std::initializer_list<value_type> init) { assign(init); }

  ~circular_deque() { DestructRange(begin_, end_); }

  // ---------------------------------------------------------------------------
  // Assignments.
  //
  // All of these may invalidate iterators and references.

  circular_deque& operator=(const circular_deque& other) {
    if (&other == this) {
      return *this;
    }

    reserve(other.size());
    assign_range(other);
    return *this;
  }
  circular_deque& operator=(circular_deque&& other) noexcept {
    if (&other == this) {
      return *this;
    }

    // We're about to overwrite the buffer, so don't free it in clear to
    // avoid doing it twice.
    ClearRetainCapacity();
    buffer_ = std::move(other.buffer_);
    begin_ = std::exchange(other.begin_, 0u);
    end_ = std::exchange(other.end_, 0u);
    IncrementGeneration();
    return *this;
  }
  circular_deque& operator=(std::initializer_list<value_type> ilist) {
    reserve(ilist.size());
    assign_range(ilist);
    return *this;
  }

  void assign(size_type count, const value_type& value) {
    ClearRetainCapacity();
    reserve(count);
    for (size_t i = 0; i < count; i++) {
      emplace_back(value);
    }
    IncrementGeneration();
  }

  // Constructs and appends new elements into the container from each element in
  // `[first, last)`, typically by copy-constructing if the iterators are also
  // over objects of type `T`.
  //
  // # Safety
  // Requires that `first` and `last` are valid iterators into a container, with
  // `first <= last`.
  template <typename InputIterator>
    requires(std::forward_iterator<InputIterator>)
  UNSAFE_BUFFER_USAGE void assign(InputIterator first, InputIterator last) {
    // Possible future enhancement, dispatch on iterator tag type. For forward
    // iterators we can use std::difference to preallocate the space required
    // and only do one copy.
    ClearRetainCapacity();
    // SAFETY: Pointers are iterators, so `first` may be a pointer. We require
    // the caller to provide valid pointers such that `last` is for the same
    // allocation and `first <= last`, and we've checked in the loop condition
    // that `first != last` so incrementing will stay a valid pointer for the
    // allocation.
    for (; first != last; UNSAFE_BUFFERS(++first)) {
      emplace_back(*first);
    }
    IncrementGeneration();
  }

  // Copies and appends new elements into the container from each element in
  // the initializer list.
  void assign(std::initializer_list<value_type> value) { assign_range(value); }

  // Constructs and appends new elements into the container from each element in
  // a range (a container or span), typically by copy-constructing if
  // the range also holds objects of type `T`.
  template <typename Range>
    requires(std::ranges::input_range<Range>)
  void assign_range(Range&& range) {
    reserve(std::ranges::distance(range));
    // SAFETY: begin() and end() produce iterators from the same container with
    // begin <= end.
    UNSAFE_BUFFERS(assign(std::ranges::begin(range), std::ranges::end(range)));
  }

  // ---------------------------------------------------------------------------
  // Accessors.
  //
  // Since this class assumes no exceptions, at() and operator[] are equivalent.

  const value_type& at(size_type i) const {
    CHECK_LT(i, size());
    size_t right_size = buffer_.capacity() - begin_;
    if (begin_ <= end_ || i < right_size) {
      return buffer_[begin_ + i];
    }
    return buffer_[i - right_size];
  }
  value_type& at(size_type i) {
    return const_cast<value_type&>(std::as_const(*this).at(i));
  }

  const value_type& operator[](size_type i) const { return at(i); }
  value_type& operator[](size_type i) { return at(i); }

  value_type& front() {
    CHECK(!empty());
    return buffer_[begin_];
  }
  const value_type& front() const {
    CHECK(!empty());
    return buffer_[begin_];
  }

  value_type& back() {
    CHECK(!empty());
    return *(end() - 1);
  }
  const value_type& back() const {
    CHECK(!empty());
    return *(end() - 1);
  }

  // ---------------------------------------------------------------------------
  // Iterators.

  iterator begin() { return iterator(this, begin_); }
  const_iterator begin() const { return const_iterator(this, begin_); }
  const_iterator cbegin() const { return const_iterator(this, begin_); }

  iterator end() { return iterator(this, end_); }
  const_iterator end() const { return const_iterator(this, end_); }
  const_iterator cend() const { return const_iterator(this, end_); }

  reverse_iterator rbegin() { return reverse_iterator(end()); }
  const_reverse_iterator rbegin() const {
    return const_reverse_iterator(end());
  }
  const_reverse_iterator crbegin() const { return rbegin(); }

  reverse_iterator rend() { return reverse_iterator(begin()); }
  const_reverse_iterator rend() const {
    return const_reverse_iterator(begin());
  }
  const_reverse_iterator crend() const { return rend(); }

  // ---------------------------------------------------------------------------
  // Memory management.

  // IMPORTANT NOTE ON reserve(...): This class implements auto-shrinking of
  // the buffer when elements are deleted and there is "too much" wasted space.
  // So if you call reserve() with a large size in anticipation of pushing many
  // elements, but pop an element before the queue is full, the capacity you
  // reserved may be lost.
  //
  // As a result, it's only worthwhile to call reserve() when you're adding
  // many things at once with no intermediate operations.
  void reserve(size_type new_capacity) {
    if (new_capacity > capacity()) {
      SetCapacityTo(new_capacity);
    }
  }

  size_type capacity() const {
    // One item is wasted to indicate end().
    return buffer_.capacity() == 0 ? 0 : buffer_.capacity() - 1;
  }

  void shrink_to_fit() {
    if (empty()) {
      // Optimize empty case to really delete everything if there was
      // something.
      if (buffer_.capacity()) {
        buffer_ = VectorBuffer();
      }
    } else {
      SetCapacityTo(size());
    }
  }

  // ---------------------------------------------------------------------------
  // Size management.

  // This will additionally reset the capacity() to 0.
  void clear() {
    // This can't resize(0) because that requires a default constructor to
    // compile, which not all contained classes may implement.
    ClearRetainCapacity();
    buffer_ = VectorBuffer();
  }

  bool empty() const { return begin_ == end_; }

  size_type size() const {
    if (begin_ <= end_) {
      return end_ - begin_;
    }
    return buffer_.capacity() - begin_ + end_;
  }

  // When reducing size, the elements are deleted from the end. When expanding
  // size, elements are added to the end with |value| or the default
  // constructed version. Even when using resize(count) to shrink, a default
  // constructor is required for the code to compile, even though it will not
  // be called.
  //
  // There are two versions rather than using a default value to avoid
  // creating a temporary when shrinking (when it's not needed). Plus if
  // the default constructor is desired when expanding usually just calling it
  // for each element is faster than making a default-constructed temporary and
  // copying it.
  void resize(size_type count) {
    // SEE BELOW VERSION if you change this. The code is mostly the same.
    if (count > size()) {
      // This could be slighly more efficient but expanding a queue with
      // identical elements is unusual and the extra computations of emplacing
      // one-by-one will typically be small relative to calling the constructor
      // for every item.
      ExpandCapacityIfNecessary(count - size());
      while (size() < count) {
        emplace_back();
      }
    } else if (count < size()) {
      size_t new_end = (begin_ + count) % buffer_.capacity();
      DestructRange(new_end, end_);
      end_ = new_end;

      ShrinkCapacityIfNecessary();
    }
    IncrementGeneration();
  }
  void resize(size_type count, const value_type& value) {
    // SEE ABOVE VERSION if you change this. The code is mostly the same.
    if (count > size()) {
      ExpandCapacityIfNecessary(count - size());
      while (size() < count) {
        emplace_back(value);
      }
    } else if (count < size()) {
      size_t new_end = (begin_ + count) % buffer_.capacity();
      DestructRange(new_end, end_);
      end_ = new_end;

      ShrinkCapacityIfNecessary();
    }
    IncrementGeneration();
  }

  // ---------------------------------------------------------------------------
  // Insert and erase.
  //
  // Insertion and deletion in the middle is O(n) and invalidates all existing
  // iterators.
  //
  // The implementation of insert isn't optimized as much as it could be. If
  // the insertion requires that the buffer be grown, it will first be grown
  // and everything moved, and then the items will be inserted, potentially
  // moving some items twice. This simplifies the implemntation substantially
  // and means less generated templatized code. Since this is an uncommon
  // operation for deques, and already relatively slow, it doesn't seem worth
  // the benefit to optimize this.

  void insert(const_iterator pos, size_type count, const T& value) {
    ValidateIterator(pos);

    // Optimize insert at the beginning.
    if (pos == begin()) {
      ExpandCapacityIfNecessary(count);
      for (size_t i = 0; i < count; i++) {
        push_front(value);
      }
      return;
    }

    CHECK_LT(pos.index_, buffer_.capacity());
    iterator insert_cur(this, pos.index_);
    iterator insert_end;
    MakeRoomFor(count, &insert_cur, &insert_end);
    while (insert_cur < insert_end) {
      std::construct_at(buffer_.get_at(insert_cur.index_), value);
      ++insert_cur;
    }

    IncrementGeneration();
  }

  template <class InputIterator>
    requires(std::forward_iterator<InputIterator>)
  void insert(const_iterator pos, InputIterator first, InputIterator last) {
    ValidateIterator(pos);

    const size_t inserted_items =
        checked_cast<size_t>(std::distance(first, last));
    if (inserted_items == 0u) {
      return;  // Can divide by 0 when doing modulo below, so return early.
    }

    // Make a hole to copy the items into.
    iterator insert_cur;
    iterator insert_end;
    if (pos == begin()) {
      // Optimize insert at the beginning, nothing needs to be shifted and the
      // hole is the |inserted_items| block immediately before |begin_|.
      ExpandCapacityIfNecessary(inserted_items);
      const size_t old_begin = begin_;
      begin_ = (old_begin + buffer_.capacity() - inserted_items) %
               buffer_.capacity();
      insert_cur = begin();
      insert_end = iterator(this, old_begin);
    } else {
      CHECK_LT(pos.index_, buffer_.capacity());
      insert_cur = iterator(this, pos.index_);
      MakeRoomFor(inserted_items, &insert_cur, &insert_end);
    }

    // Copy the items.
    while (insert_cur < insert_end) {
      std::construct_at(buffer_.get_at(insert_cur.index_), *first);
      ++insert_cur;
      // SAFETY: The input iterator may be a pointer, in which case we will
      // produce UB if `first` is incremented past `last`. We use checked_cast
      // of std::distance to an unsigned value above, which ensures that `last
      // >= first`. Then we need that `insert_end - insert_cur <= last - first`:
      // - If inserting at the start, pos == begin() and `insert_cur` is
      //   positioned at `begin_ - (last - first)`, and `insert_end` is
      //   positioned at `begin_` so we have
      //   `insert_end - insert_cur == last - first`.
      // - If inserting elsewhere, `MakeRoomFor(last - first, ...)` returns an
      // iterator
      //   pair with distance of `last - first`, so we have
      //   `insert_end - insert_cur == last - first`.
      UNSAFE_BUFFERS(++first);
    }

    IncrementGeneration();
  }

  // These all return an iterator to the inserted item. Existing iterators will
  // be invalidated.
  iterator insert(const_iterator pos, const T& value) {
    return emplace(pos, value);
  }
  iterator insert(const_iterator pos, T&& value) {
    return emplace(pos, std::move(value));
  }
  template <class... Args>
  iterator emplace(const_iterator pos, Args&&... args) {
    ValidateIterator(pos);

    // Optimize insert at beginning which doesn't require shifting.
    if (pos == cbegin()) {
      emplace_front(std::forward<Args>(args)...);
      return begin();
    }

    // Do this before we make the new iterators we return.
    IncrementGeneration();

    CHECK_LT(pos.index_, buffer_.capacity());
    iterator insert_begin(this, pos.index_);
    iterator insert_end;
    MakeRoomFor(1, &insert_begin, &insert_end);
    std::construct_at(buffer_.get_at(insert_begin.index_),
                      std::forward<Args>(args)...);

    return insert_begin;
  }

  // Calling erase() won't automatically resize the buffer smaller like resize
  // or the pop functions. Erase is slow and relatively uncommon, and for
  // normal deque usage a pop will normally be done on a regular basis that
  // will prevent excessive buffer usage over long periods of time. It's not
  // worth having the extra code for every template instantiation of erase()
  // to resize capacity downward to a new buffer.
  iterator erase(const_iterator pos) { return erase(pos, pos + 1); }
  iterator erase(const_iterator pos_begin, const_iterator pos_end) {
    ValidateIterator(pos_begin);
    ValidateIterator(pos_end);

    IncrementGeneration();

    if (pos_begin.index_ == pos_end.index_) {
      // Nothing deleted. Need to return early to avoid falling through to
      // moving items on top of themselves.
      return iterator(this, pos_begin.index_);
    }

    // First, call the destructor on the deleted items.
    DestructRange(pos_begin.index_, pos_end.index_);

    if (pos_begin.index_ == begin_) {
      // This deletion is from the beginning. Nothing needs to be copied, only
      // begin_ needs to be updated.
      begin_ = pos_end.index_;
      return iterator(this, pos_end.index_);
    }

    // In an erase operation, the shifted items all move logically to the left,
    // so move them from left-to-right.
    //
    // The elements are being moved to memory where the T objects were
    // previously destroyed.
    //
    // TODO(danakj): We could skip destruction and do MoveAssignRange here, for
    // the elements that are being replaced.
    size_t move_src = pos_end.index_;
    const size_t move_src_end = end_;
    size_t move_dest = pos_begin.index_;
    const size_t cap = buffer_.capacity();
    while (move_src != move_src_end) {
      VectorBuffer::MoveConstructRange(buffer_.subspan(move_src, 1u),
                                       buffer_.subspan(move_dest, 1u));
      move_src = (move_src + 1u) % cap;
      move_dest = (move_dest + 1u) % cap;
    }

    end_ = move_dest;

    // Since we did not reallocate and only changed things after the erase
    // element(s), the input iterator's index points to the thing following the
    // deletion.
    return iterator(this, pos_begin.index_);
  }

  // ---------------------------------------------------------------------------
  // Begin/end operations.

  void push_front(const T& value) { emplace_front(value); }
  void push_front(T&& value) { emplace_front(std::move(value)); }

  void push_back(const T& value) { emplace_back(value); }
  void push_back(T&& value) { emplace_back(std::move(value)); }

  template <class... Args>
  reference emplace_front(Args&&... args) {
    ExpandCapacityIfNecessary(1);
    if (begin_ == 0) {
      begin_ = buffer_.capacity() - 1;
    } else {
      begin_--;
    }
    IncrementGeneration();
    std::construct_at(buffer_.get_at(begin_), std::forward<Args>(args)...);
    return front();
  }

  template <class... Args>
  reference emplace_back(Args&&... args) {
    ExpandCapacityIfNecessary(1);
    std::construct_at(buffer_.get_at(end_), std::forward<Args>(args)...);
    if (end_ == buffer_.capacity() - 1) {
      end_ = 0;
    } else {
      end_++;
    }
    IncrementGeneration();
    return back();
  }

  void pop_front() {
    CHECK(!empty());
    DestructRange(begin_, begin_ + 1u);
    begin_++;
    if (begin_ == buffer_.capacity()) {
      begin_ = 0;
    }

    ShrinkCapacityIfNecessary();

    // Technically popping will not invalidate any iterators since the
    // underlying buffer will be stable. But in the future we may want to add a
    // feature that resizes the buffer smaller if there is too much wasted
    // space. This ensures we can make such a change safely.
    IncrementGeneration();
  }
  void pop_back() {
    CHECK(!empty());
    if (end_ == 0) {
      end_ = buffer_.capacity() - 1;
    } else {
      end_--;
    }
    DestructRange(end_, end_ + 1u);

    ShrinkCapacityIfNecessary();

    // See pop_front comment about why this is here.
    IncrementGeneration();
  }

  // ---------------------------------------------------------------------------
  // General operations.

  void swap(circular_deque& other) {
    std::swap(buffer_, other.buffer_);
    std::swap(begin_, other.begin_);
    std::swap(end_, other.end_);
    IncrementGeneration();
  }

  friend void swap(circular_deque& lhs, circular_deque& rhs) { lhs.swap(rhs); }

 private:
  friend internal::circular_deque_iterator<T>;
  friend internal::circular_deque_const_iterator<T>;

  // Moves the items in the given circular buffer to the current one. The source
  // is moved from so will become invalid. The destination buffer must have
  // already been allocated with enough size.
  //
  // # Safety
  // `from_begin` and `from_end` must be less-than and less-than-or-equal-to the
  // capacity of `from_buf` respectively, with `from_begin <= from_end`, or
  // Undefined Behaviour may result.
  UNSAFE_BUFFER_USAGE static void MoveBuffer(VectorBuffer& from_buf,
                                             size_t from_begin,
                                             size_t from_end,
                                             VectorBuffer& to_buf,
                                             size_t* to_begin,
                                             size_t* to_end) {
    *to_begin = 0;
    if (from_begin < from_end) {
      // Contiguous.
      VectorBuffer::MoveConstructRange(
          from_buf.subspan(from_begin, from_end - from_begin),
          to_buf.subspan(0u, from_end - from_begin));
      *to_end = from_end - from_begin;
    } else if (from_begin > from_end) {
      // Discontiguous, copy the right side to the beginning of the new buffer.
      span<T> right_side = from_buf.subspan(from_begin);
      VectorBuffer::MoveConstructRange(right_side,
                                       to_buf.subspan(0u, right_side.size()));
      // Append the left side.
      span<T> left_side = from_buf.subspan(0u, from_end);
      VectorBuffer::MoveConstructRange(
          left_side, to_buf.subspan(right_side.size(), left_side.size()));
      *to_end = left_side.size() + right_side.size();
    } else {
      // No items.
      *to_end = 0;
    }
  }

  // Expands the buffer size. This assumes the size is larger than the
  // number of elements in the vector (it won't call delete on anything).
  void SetCapacityTo(size_t new_capacity) {
    // Use the capacity + 1 as the internal buffer size to differentiate
    // empty and full (see definition of buffer_ below).
    VectorBuffer new_buffer(new_capacity + 1u);
    // SAFETY: This class maintains an invariant that `begin_` and `end_` are
    // less than `buffer_`'s capacity.
    UNSAFE_BUFFERS(
        MoveBuffer(buffer_, begin_, end_, new_buffer, &begin_, &end_));
    buffer_ = std::move(new_buffer);
  }
  void ExpandCapacityIfNecessary(size_t additional_elts) {
    const size_t cur_size = size();
    const size_t cur_capacity = capacity();

    // Protect against overflow when adding `additional_elts`, and exceeding the
    // max allocation size.
    CHECK_LE(additional_elts, PTRDIFF_MAX - cur_size);

    size_t min_new_capacity = cur_size + additional_elts;
    if (cur_capacity >= min_new_capacity) {
      return;  // Already enough room.
    }

    min_new_capacity =
        std::max(min_new_capacity, internal::kCircularBufferInitialCapacity);

    // std::vector always grows by at least 50%. WTF::Deque grows by at least
    // 25%. We expect queue workloads to generally stay at a similar size and
    // grow less than a vector might, so use 25%.
    SetCapacityTo(std::max(min_new_capacity, cur_capacity + cur_capacity / 4u));
  }

  void ShrinkCapacityIfNecessary() {
    // Don't auto-shrink below this size.
    if (capacity() <= internal::kCircularBufferInitialCapacity) {
      return;
    }

    // Shrink when 100% of the size() is wasted.
    size_t sz = size();
    size_t empty_spaces = capacity() - sz;
    if (empty_spaces < sz) {
      return;
    }

    // Leave 1/4 the size as free capacity, not going below the initial
    // capacity.
    size_t new_capacity =
        std::max(internal::kCircularBufferInitialCapacity, sz + sz / 4);
    if (new_capacity < capacity()) {
      // Count extra item to convert to internal capacity.
      SetCapacityTo(new_capacity);
    }
  }

  // Backend for clear() but does not resize the internal buffer.
  void ClearRetainCapacity() {
    // This can't resize(0) because that requires a default constructor to
    // compile, which not all contained classes may implement.

    // SAFETY: This class maintains an invariant that `begin_` and `end_` are
    // less than `buffer_`'s capacity. `new_end` is computed modulo the capacity
    // so it is in range.
    DestructRange(begin_, end_);
    begin_ = 0;
    end_ = 0;
    IncrementGeneration();
  }

  // Calls destructors for the given begin->end indices. The indices may wrap
  // around. The buffer is not resized, and the begin_ and end_ members are
  // not changed.
  void DestructRange(size_t begin, size_t end) {
    if (end == begin) {
      return;
    } else if (end > begin) {
      VectorBuffer::DestructRange(buffer_.subspan(begin, end - begin));
    } else {
      VectorBuffer::DestructRange(buffer_.subspan(begin));
      VectorBuffer::DestructRange(buffer_.subspan(0u, end));
    }
  }

  // Makes room for |count| items starting at |*insert_begin|. Since iterators
  // are not stable across buffer resizes, |*insert_begin| will be updated to
  // point to the beginning of the newly opened position in the new array (it's
  // in/out), and the end of the newly opened position (it's out-only).
  void MakeRoomFor(size_t count, iterator* insert_begin, iterator* insert_end) {
    if (count == 0) {
      *insert_end = *insert_begin;
      return;
    }

    // The offset from the beginning will be stable across reallocations.
    size_t begin_offset = insert_begin->OffsetFromBegin();
    ExpandCapacityIfNecessary(count);

    // Update the new end and prepare the iterators for copying. The newly
    // used space contains uninitialized memory.
    const size_t cap = buffer_.capacity();
    size_t src = end_;
    end_ = (end_ + count) % cap;
    size_t dest = end_;

    *insert_begin = iterator(this, (begin_ + begin_offset) % cap);
    *insert_end = iterator(this, (insert_begin->index_ + count) % cap);

    // Move the elements. This will always involve shifting logically to the
    // right, so move in a right-to-left order.
    while (true) {
      if (src == insert_begin->index_) {
        break;
      }
      src = (src + cap - 1u) % cap;
      dest = (dest + cap - 1u) % cap;
      VectorBuffer::MoveConstructRange(buffer_.subspan(src, 1u),
                                       buffer_.subspan(dest, 1u));
    }
  }

#if DCHECK_IS_ON()
  // Asserts the given index is dereferencable. The index is an index into the
  // buffer, not an index used by operator[] or at() which will be offsets from
  // begin.
  void CheckValidIndex(size_t i) const {
    if (begin_ <= end_) {
      DCHECK(i >= begin_ && i < end_);
    } else {
      DCHECK((i >= begin_ && i < buffer_.capacity()) || i < end_);
    }
  }

  // Asserts the given index is either dereferencable or points to end().
  void CheckValidIndexOrEnd(size_t i) const {
    if (i != end_) {
      CheckValidIndex(i);
    }
  }

  void ValidateIterator(const const_iterator& i) const {
    DCHECK(i.parent_deque_ == this);
    i.CheckUnstableUsage();
  }

  // See generation_ below.
  void IncrementGeneration() { generation_++; }
#else
  // No-op versions of these functions for release builds.
  void CheckValidIndex(size_t) const {}
  void CheckValidIndexOrEnd(size_t) const {}
  void ValidateIterator(const const_iterator& i) const {}
  void IncrementGeneration() {}
#endif

  // Danger, the buffer_.capacity() is the "internal capacity" which is
  // capacity() + 1 since there is an extra item to indicate the end. Otherwise
  // being completely empty and completely full are indistinguishable (begin ==
  // end). We could add a separate flag to avoid it, but that adds significant
  // extra complexity since every computation will have to check for it. Always
  // keeping one extra unused element in the buffer makes iterator computations
  // much simpler.
  //
  // Container internal code will want to use buffer_.capacity() for offset
  // computations rather than capacity().
  VectorBuffer buffer_;
  size_type begin_ = 0;
  size_type end_ = 0;

#if DCHECK_IS_ON()
  // Incremented every time a modification is made that could affect iterator
  // invalidations.
  uint64_t generation_ = 0;
#endif
};

// Implementations of base::Erase[If] (see base/stl_util.h).
template <class T, class Value>
size_t Erase(circular_deque<T>& container, const Value& value) {
  auto removed = std::ranges::remove(container, value);
  size_t num_removed = removed.size();
  container.erase(removed.begin(), removed.end());
  return num_removed;
}

template <class T, class Predicate>
size_t EraseIf(circular_deque<T>& container, Predicate pred) {
  auto removed = std::ranges::remove_if(container, pred);
  size_t num_removed = removed.size();
  container.erase(removed.begin(), removed.end());
  return num_removed;
}

}  // namespace base

#endif  // BASE_CONTAINERS_CIRCULAR_DEQUE_H_
