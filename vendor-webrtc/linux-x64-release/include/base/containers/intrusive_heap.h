// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_CONTAINERS_INTRUSIVE_HEAP_H_
#define BASE_CONTAINERS_INTRUSIVE_HEAP_H_

// Implements a standard max-heap, but with arbitrary element removal. To
// facilitate this, each element has associated with it a HeapHandle (an opaque
// wrapper around the index at which the element is stored), which is maintained
// by the heap as elements move within it.
//
// An IntrusiveHeap is implemented as a standard max-heap over a std::vector<T>,
// like std::make_heap. Insertion, removal and updating are amortized O(lg size)
// (occasional O(size) cost if a new vector allocation is required). Retrieving
// an element by handle is O(1). Looking up the top element is O(1). Insertions,
// removals and updates invalidate all iterators, but handles remain valid.
// Similar to a std::set, all iterators are read-only so as to disallow changing
// elements and violating the heap property. That being said, if the type you
// are storing is able to have its sort key be changed externally you can
// repair the heap by resorting the modified element via a call to "Update".
//
// Example usage:
//
//   // Create a heap, wrapping integer elements with WithHeapHandle in order to
//   // endow them with heap handles.
//   IntrusiveHeap<WithHeapHandle<int>> heap;
//
//   // WithHeapHandle<T> is for simple or opaque types. In cases where you
//   // control the type declaration you can also provide HeapHandle storage by
//   // deriving from InternalHeapHandleStorage.
//   class Foo : public InternalHeapHandleStorage {
//    public:
//     explicit Foo(int);
//     ...
//   };
//   IntrusiveHeap<Foo> heap2;
//
//   // Insert some elements. Like most containers, "insert" returns an iterator
//   // to the element in the container.
//   heap.insert(3);
//   heap.insert(1);
//   auto it = heap.insert(4);
//
//   // By default this is a max heap, so the top element should be 4 at this
//   // point.
//   EXPECT_EQ(4, heap.top().value());
//
//   // Iterators are invalidated by further heap operations, but handles are
//   // not. Grab a handle to the current top element so we can track it across
//   // changes.
//   HeapHandle* handle = it->handle();
//
//   // Insert a new max element. 4 should no longer be the top.
//   heap.insert(5);
//   EXPECT_EQ(5, heap.top().value());
//
//   // We can lookup and erase element 4 by its handle, even though it has
//   // moved. Note that erasing the element invalidates the handle to it.
//   EXPECT_EQ(4, heap.at(*handle).value());
//   heap.erase(*handle);
//   handle = nullptr;
//
//   // Popping the current max (5), makes 3 the new max, as we already erased
//   // element 4.
//   heap.pop();
//   EXPECT_EQ(3, heap.top().value());
//
// Under the hood the HeapHandle is managed by an object implementing the
// HeapHandleAccess interface, which is passed as a parameter to the
// IntrusiveHeap template:
//
//   // Gets the heap handle associated with the element. This should return the
//   // most recently set handle value, or HeapHandle::Invalid(). This is only
//   // called in DCHECK builds.
//   HeapHandle GetHeapHandle(const T*);
//
//   // Changes the result of GetHeapHandle. GetHeapHandle() must return the
//   // most recent value provided to SetHeapHandle() or HeapHandle::Invalid().
//   // In some implementations, where GetHeapHandle() can independently
//   // reproduce the correct value, it is possible that SetHeapHandle() does
//   // nothing.
//   void SetHeapHandle(T*, HeapHandle);
//
//   // Clears the heap handle associated with the given element. After calling
//   // this GetHeapHandle() must return HeapHandle::Invalid().
//   void ClearHeapHandle(T*);
//
// The default implementation of HeapHandleAccess assumes that your type
// provides HeapHandle storage and will simply forward these calls to equivalent
// member functions on the type T:
//
//   void T::SetHeapHandle(HeapHandle)
//   void T::ClearHeapHandle()
//   HeapHandle T::GetHeapHandle() const
//
// The WithHeapHandle and InternalHeapHandleStorage classes in turn provide
// implementations of that contract.
//
// In summary, to provide heap handle support for your type, you can do one of
// the following (from most manual / least magical, to least manual / most
// magical):
//
// 0. use a custom HeapHandleAccessor, and implement storage however you want;
// 1. use the default HeapHandleAccessor, and manually provide storage on your
//    your element type and implement the IntrusiveHeap contract;
// 2. use the default HeapHandleAccessor, and endow your type with handle
//    storage by deriving from a helper class (see InternalHeapHandleStorage);
//    or,
// 3. use the default HeapHandleAccessor, and wrap your type in a container that
//    provides handle storage (see WithHeapHandle<T>).
//
// Approach 0 is suitable for custom types that already implement something akin
// to heap handles, via back pointers or any other mechanism, but where the
// storage is external to the objects in the heap. If you already have the
// ability to determine where in a container an object lives despite it
// being moved, then you don't need the overhead of storing an actual HeapHandle
// whose value can be inferred.
//
// Approach 1 is is suitable in cases like the above, but where the data
// allowing you to determine the index of an element in a container is stored
// directly in the object itself.
//
// Approach 2 is suitable for types whose declarations you control, where you
// are able to use inheritance.
//
// Finally, approach 3 is suitable when you are storing PODs, or a type whose
// declaration you can not change.
//
// Most users should be using approach 2 or 3.

#include <algorithm>
#include <compare>
#include <functional>
#include <iterator>
#include <limits>
#include <memory>
#include <type_traits>
#include <utility>
#include <vector>

#include "base/base_export.h"
#include "base/check.h"
#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "base/memory/ptr_util.h"
#include "base/types/cxx23_from_range.h"
#include "third_party/abseil-cpp/absl/container/inlined_vector.h"

namespace base {

// Intended as a wrapper around an |index_| in the vector storage backing an
// IntrusiveHeap. A HeapHandle is associated with each element in an
// IntrusiveHeap, and is maintained by the heap as the object moves around
// within it. It can be used to subsequently remove the element, or update it
// in place.
class BASE_EXPORT HeapHandle {
 public:
  enum : size_t { kInvalidIndex = std::numeric_limits<size_t>::max() };

  constexpr HeapHandle() = default;
  constexpr HeapHandle(const HeapHandle& other) = default;
  HeapHandle(HeapHandle&& other) noexcept
      : index_(std::exchange(other.index_, kInvalidIndex)) {}
  ~HeapHandle() = default;

  HeapHandle& operator=(const HeapHandle& other) = default;
  HeapHandle& operator=(HeapHandle&& other) noexcept {
    index_ = std::exchange(other.index_, kInvalidIndex);
    return *this;
  }

  static HeapHandle Invalid();

  // Resets this handle back to an invalid state.
  void reset() { index_ = kInvalidIndex; }

  // Accessors.
  size_t index() const { return index_; }
  bool IsValid() const { return index_ != kInvalidIndex; }

  // Comparison operators.
  friend bool operator==(const HeapHandle& lhs,
                         const HeapHandle& rhs) = default;
  friend std::strong_ordering operator<=>(const HeapHandle& lhs,
                                          const HeapHandle& rhs) = default;

 private:
  template <typename T, typename Compare, typename HeapHandleAccessor>
  friend class IntrusiveHeap;

  // Only IntrusiveHeaps can create valid HeapHandles.
  explicit HeapHandle(size_t index) : index_(index) {}

  size_t index_ = kInvalidIndex;
};

// The default HeapHandleAccessor, which simply forwards calls to the underlying
// type.
template <typename T>
struct DefaultHeapHandleAccessor {
  void SetHeapHandle(T* element, HeapHandle handle) const {
    element->SetHeapHandle(handle);
  }

  void ClearHeapHandle(T* element) const { element->ClearHeapHandle(); }

  HeapHandle GetHeapHandle(const T* element) const {
    return element->GetHeapHandle();
  }
};

// Intrusive heap class. This is something like a std::vector (insertion and
// removal are similar, objects don't have a fixed address in memory) crossed
// with a std::set (elements are considered immutable once they're in the
// container).
template <typename T,
          typename Compare = std::less<T>,
          typename HeapHandleAccessor = DefaultHeapHandleAccessor<T>>
class IntrusiveHeap {
 private:
  using UnderlyingType = std::vector<T>;

 public:
  //////////////////////////////////////////////////////////////////////////////
  // Types.

  using value_type = typename UnderlyingType::value_type;
  using size_type = typename UnderlyingType::size_type;
  using difference_type = typename UnderlyingType::difference_type;
  using value_compare = Compare;
  using heap_handle_accessor = HeapHandleAccessor;

  using reference = typename UnderlyingType::reference;
  using const_reference = typename UnderlyingType::const_reference;
  using pointer = typename UnderlyingType::pointer;
  using const_pointer = typename UnderlyingType::const_pointer;

  // Iterators are read-only.
  using iterator = typename UnderlyingType::const_iterator;
  using const_iterator = typename UnderlyingType::const_iterator;
  using reverse_iterator = typename UnderlyingType::const_reverse_iterator;
  using const_reverse_iterator =
      typename UnderlyingType::const_reverse_iterator;

  //////////////////////////////////////////////////////////////////////////////
  // Lifetime.

  // Constructs an empty heap, with the default comparator and handle accessor.
  IntrusiveHeap() = default;
  // Constructs an empty heap.
  IntrusiveHeap(const value_compare& comp, const heap_handle_accessor& access)
      : impl_(comp, access) {}

  // Constructs a heap containing all elements in the `range`.
  template <class Range>
    requires(std::ranges::input_range<Range>)
  IntrusiveHeap(base::from_range_t,
                Range&& range,
                const value_compare& comp = value_compare(),
                const heap_handle_accessor& access = heap_handle_accessor())
      : impl_(comp, access) {
    insert_range(std::forward<Range>(range));
  }

  // Construct a heap with elements from `[first, last)`. Prefer the
  // `from_range_t` constructor as it avoid iterator pairs.
  //
  // # Safety
  // The `first` and `last` must be a valid iterator pair for a container with
  // `first <= last` or Undefined Behaviour results.
  template <class InputIterator>
    requires(std::input_iterator<InputIterator>)
  UNSAFE_BUFFER_USAGE IntrusiveHeap(
      InputIterator first,
      InputIterator last,
      const value_compare& comp = value_compare(),
      const heap_handle_accessor& access = heap_handle_accessor())
      : impl_(comp, access) {
    // SAFETY: The caller must provide a valid iterator pair.
    UNSAFE_BUFFERS(insert(first, last));
  }

  // Moves an intrusive heap. The outstanding handles remain valid and end up
  // pointing to the new heap.
  IntrusiveHeap(IntrusiveHeap&& other) = default;

  // Copy constructor for an intrusive heap.
  IntrusiveHeap(const IntrusiveHeap&);

  // Initializer list constructor.
  template <typename U>
  IntrusiveHeap(std::initializer_list<U> ilist,
                const value_compare& comp = value_compare(),
                const heap_handle_accessor& access = heap_handle_accessor())
      : impl_(comp, access) {
    insert_range(ilist);
  }

  ~IntrusiveHeap();

  //////////////////////////////////////////////////////////////////////////////
  // Assignment.

  IntrusiveHeap& operator=(IntrusiveHeap&&) noexcept;
  IntrusiveHeap& operator=(const IntrusiveHeap&);
  IntrusiveHeap& operator=(std::initializer_list<value_type> ilist);

  //////////////////////////////////////////////////////////////////////////////
  // Element access.
  //
  // These provide O(1) const access to the elements in the heap. If you wish to
  // modify an element in the heap you should first remove it from the heap, and
  // then reinsert it into the heap, or use the "Replace*" helper functions. In
  // the rare case where you directly modify an element in the heap you can
  // subsequently repair the heap with "Update".

  const_reference at(size_type pos) const { return impl_.heap_.at(pos); }
  const_reference at(HeapHandle pos) const {
    return impl_.heap_.at(pos.index());
  }
  const_reference operator[](size_type pos) const { return impl_.heap_[pos]; }
  const_reference operator[](HeapHandle pos) const {
    return impl_.heap_[pos.index()];
  }
  const_reference front() const { return impl_.heap_.front(); }
  const_reference back() const { return impl_.heap_.back(); }
  const_reference top() const { return impl_.heap_.front(); }

  // May or may not return a null pointer if size() is zero.
  const_pointer data() const { return impl_.heap_.data(); }

  //////////////////////////////////////////////////////////////////////////////
  // Memory management.

  void reserve(size_type new_capacity) { impl_.heap_.reserve(new_capacity); }
  size_type capacity() const { return impl_.heap_.capacity(); }
  void shrink_to_fit() { impl_.heap_.shrink_to_fit(); }

  //////////////////////////////////////////////////////////////////////////////
  // Size management.

  void clear();
  size_type size() const { return impl_.heap_.size(); }
  size_type max_size() const { return impl_.heap_.max_size(); }
  bool empty() const { return impl_.heap_.empty(); }

  //////////////////////////////////////////////////////////////////////////////
  // Iterators.
  //
  // Only constant iterators are allowed.

  const_iterator begin() const { return impl_.heap_.cbegin(); }
  const_iterator cbegin() const { return impl_.heap_.cbegin(); }

  const_iterator end() const { return impl_.heap_.cend(); }
  const_iterator cend() const { return impl_.heap_.cend(); }

  const_reverse_iterator rbegin() const { return impl_.heap_.crbegin(); }
  const_reverse_iterator crbegin() const { return impl_.heap_.crbegin(); }

  const_reverse_iterator rend() const { return impl_.heap_.crend(); }
  const_reverse_iterator crend() const { return impl_.heap_.crend(); }

  //////////////////////////////////////////////////////////////////////////////
  // Insertion (these are std::multiset like, with no position hints).
  //
  // All insertion operations invalidate iterators, pointers and references.
  // Handles remain valid. Insertion of one element is amortized O(lg size)
  // (occasional O(size) cost if a new vector allocation is required).

  const_iterator insert(const value_type& value) { return InsertImpl(value); }
  const_iterator insert(value_type&& value) {
    return InsertImpl(std::move(value));
  }

  // Inserts all elements in `range` into the heap.
  template <class Range>
    requires(std::ranges::input_range<Range>)
  void insert_range(Range&& range) {
    for (const auto& i : range) {
      InsertImpl(value_type(i));
    }
  }

  // Inserts all elements in `[first, last)` into the heap. Prefer to use
  // `insert_range()` as it avoids iterator pairs.
  //
  // # Safety
  // The `first` and `last` must be a valid iterator pair for a container with
  // `first <= last` or Undefined Behaviour results.
  template <class InputIterator>
    requires(std::input_iterator<InputIterator>)
  UNSAFE_BUFFER_USAGE void insert(InputIterator first, InputIterator last) {
    for (auto it = first; it != last;
         // SAFETY: The caller must provide a valid iterator pair.
         UNSAFE_BUFFERS(++it)) {
      InsertImpl(value_type(*it));
    }
  }

  template <typename... Args>
  const_iterator emplace(Args&&... args) {
    return InsertImpl(value_type(std::forward<Args>(args)...));
  }

  //////////////////////////////////////////////////////////////////////////////
  // Removing elements.
  //
  // Erasing invalidates all outstanding iterators, pointers and references.
  // Handles remain valid. Removing one element is amortized O(lg size)
  // (occasional O(size) cost if a new vector allocation is required).
  //
  // Note that it is safe for the element being removed to be in an invalid
  // state (modified such that it may currently violate the heap property)
  // when this called.

  // Takes the element from the heap at the given position, erasing that entry
  // from the heap. This can only be called if |value_type| is movable.
  value_type take(size_type pos);

  // Version of take that will accept iterators and handles. This can only be
  // called if |value_type| is movable.
  template <typename P>
  value_type take(P pos) {
    return take(ToIndex(pos));
  }

  // Takes the top element from the heap.
  value_type take_top() { return take(0u); }

  // Erases the element at the given position |pos|.
  void erase(size_type pos);

  // Version of erase that will accept iterators and handles.
  template <typename P>
  void erase(P pos) {
    erase(ToIndex(pos));
  }

  // Removes the element at the top of the heap (accessible via "top", or
  // "front" or "take").
  void pop() { erase(0u); }

  // Erases every element that matches the predicate. This is done in-place for
  // maximum efficiency. Also, to avoid re-entrancy issues, elements are deleted
  // at the very end.
  // Note: This function is currently tuned for a use-case where there are
  // usually 8 or less elements removed at a time. Consider adding a template
  // parameter if a different tuning is needed.
  template <typename Functor>
  void EraseIf(Functor predicate) {
    // Stable partition ensures that if no elements are erased, the heap remains
    // intact.
    auto erase_start = std::stable_partition(
        impl_.heap_.begin(), impl_.heap_.end(),
        [&](const auto& element) { return !predicate(element); });

    // Clear the heap handle of every element that will be erased.
    for (size_t i = static_cast<size_t>(erase_start - impl_.heap_.begin());
         i < impl_.heap_.size(); ++i) {
      ClearHeapHandle(i);
    }

    // Deleting an element can potentially lead to reentrancy, we move all the
    // elements to be erased into a temporary container before deleting them.
    // This is to avoid changing the underlying container during the erase()
    // call.
    absl::InlinedVector<value_type, 8> elements_to_delete;
    std::move(erase_start, impl_.heap_.end(),
              std::back_inserter(elements_to_delete));

    impl_.heap_.erase(erase_start, impl_.heap_.end());

    // If no elements were removed, then the heap is still intact.
    if (elements_to_delete.empty()) {
      return;
    }

    // Repair the heap and ensure handles are pointing to the right index.
    std::ranges::make_heap(impl_.heap_, value_comp());
    for (size_t i = 0; i < size(); ++i) {
      SetHeapHandle(i);
    }

    // Explicitly delete elements last.
    elements_to_delete.clear();
  }

  //////////////////////////////////////////////////////////////////////////////
  // Updating.
  //
  // Amortized cost of O(lg size).

  // Replaces the element corresponding to |handle| with a new |element|.
  const_iterator Replace(size_type pos, const T& element) {
    return ReplaceImpl(pos, element);
  }
  const_iterator Replace(size_type pos, T&& element) {
    return ReplaceImpl(pos, std::move_if_noexcept(element));
  }

  // Versions of Replace that will accept handles and iterators.
  template <typename P>
  const_iterator Replace(P pos, const T& element) {
    return ReplaceImpl(ToIndex(pos), element);
  }
  template <typename P>
  const_iterator Replace(P pos, T&& element) {
    return ReplaceImpl(ToIndex(pos), std::move_if_noexcept(element));
  }

  // Replaces the top element in the heap with the provided element.
  const_iterator ReplaceTop(const T& element) {
    return ReplaceTopImpl(element);
  }
  const_iterator ReplaceTop(T&& element) {
    return ReplaceTopImpl(std::move_if_noexcept(element));
  }

  // Causes the object at the given location to be resorted into an appropriate
  // position in the heap. To be used if the object in the heap was externally
  // modified, and the heap needs to be repaired. This only works if a single
  // heap element has been modified, otherwise the behaviour is undefined.
  const_iterator Update(size_type pos);
  template <typename P>
  const_iterator Update(P pos) {
    return Update(ToIndex(pos));
  }

  // Applies a modification function to the object at the given location, then
  // repairs the heap. To be used to modify an element in the heap in-place
  // while keeping the heap intact.
  template <typename P, typename UnaryOperation>
  const_iterator Modify(P pos, UnaryOperation unary_op) {
    size_type index = ToIndex(pos);
    unary_op(impl_.heap_.at(index));
    return Update(index);
  }

  //////////////////////////////////////////////////////////////////////////////
  // Access to helper functors.

  const value_compare& value_comp() const { return impl_.get_value_compare(); }

  const heap_handle_accessor& heap_handle_access() const {
    return impl_.get_heap_handle_access();
  }

  //////////////////////////////////////////////////////////////////////////////
  // General operations.

  void swap(IntrusiveHeap& other) noexcept;
  friend void swap(IntrusiveHeap& lhs, IntrusiveHeap& rhs) { lhs.swap(rhs); }

  // Comparison operators. These check for exact equality. Two heaps that are
  // semantically equivalent (contain the same elements, but in different
  // orders) won't compare as equal using these operators.
  friend bool operator==(const IntrusiveHeap& lhs, const IntrusiveHeap& rhs) {
    return lhs.impl_.heap_ == rhs.impl_.heap_;
  }

  //////////////////////////////////////////////////////////////////////////////
  // Utility functions.

  // Converts iterators and handles to indices. Helpers for templated versions
  // of insert/erase/Replace.
  size_type ToIndex(HeapHandle handle) { return handle.index(); }
  size_type ToIndex(const_iterator pos);
  size_type ToIndex(const_reverse_iterator pos);

 private:
  // Templated version of ToIndex that lets insert/erase/Replace work with all
  // integral types.
  template <typename I>
    requires(std::is_integral_v<I>)
  size_type ToIndex(I pos) {
    return static_cast<size_type>(pos);
  }

  // Returns the last valid index in |heap_|.
  size_type GetLastIndex() const { return impl_.heap_.size() - 1; }

  // Helper functions for setting heap handles.
  void SetHeapHandle(size_type i);
  void ClearHeapHandle(size_type i);
  HeapHandle GetHeapHandle(size_type i);

  // Helpers for doing comparisons between elements inside and outside of the
  // heap.
  bool Less(size_type i, size_type j);
  bool Less(const T& element, size_type i);
  bool Less(size_type i, const T& element);

  // The following function are all related to the basic heap algorithm
  // underpinning this data structure. They are templated so that they work with
  // both movable (U = T&&) and non-movable (U = const T&) types.

  // Primitive helpers for adding removing / elements to the heap. To minimize
  // moves, the heap is implemented by making a hole where an element used to
  // be (or where a new element will soon be), and moving the hole around,
  // before finally filling the hole or deleting the entry corresponding to the
  // hole.
  void MakeHole(size_type pos);
  template <typename U>
  void FillHole(size_type hole, U element);
  void MoveHole(size_type new_hole_pos, size_type old_hole_pos);

  // Moves a hold up the tree and fills it with the provided |element|. Returns
  // the final index of the element.
  template <typename U>
  size_type MoveHoleUpAndFill(size_type hole_pos, U element);

  // Moves a hole down the tree and fills it with the provided |element|. If
  // |kFillWithLeaf| is true it will deterministically move the hole all the
  // way down the tree, avoiding a second comparison per level, before
  // potentially moving it back up the tree.
  struct WithLeafElement {
    static constexpr bool kIsLeafElement = true;
  };
  struct WithElement {
    static constexpr bool kIsLeafElement = false;
  };
  template <typename FillElementType, typename U>
  size_type MoveHoleDownAndFill(size_type hole_pos, U element);

  // Implementation of Insert and Replace built on top of the MoveHole
  // primitives.
  template <typename U>
  const_iterator InsertImpl(U element);
  template <typename U>
  const_iterator ReplaceImpl(size_type pos, U element);
  template <typename U>
  const_iterator ReplaceTopImpl(U element);

  // To support comparators that may not be possible to default-construct, we
  // have to store an instance of value_compare. Using this to store all
  // internal state of IntrusiveHeap and using private inheritance to store
  // compare lets us take advantage of an empty base class optimization to avoid
  // extra space in the common case when Compare has no state.
  struct Impl : private value_compare, private heap_handle_accessor {
    Impl(const value_compare& value_comp,
         const heap_handle_accessor& heap_handle_access)
        : value_compare(value_comp), heap_handle_accessor(heap_handle_access) {}

    Impl() = default;
    Impl(Impl&&) = default;
    Impl(const Impl&) = default;
    Impl& operator=(Impl&& other) = default;
    Impl& operator=(const Impl& other) = default;

    const value_compare& get_value_compare() const { return *this; }
    value_compare& get_value_compare() { return *this; }

    const heap_handle_accessor& get_heap_handle_access() const { return *this; }
    heap_handle_accessor& get_heap_handle_access() { return *this; }

    // The items in the heap.
    UnderlyingType heap_;
  } impl_;
};

// Helper class to endow an object with internal HeapHandle storage. By deriving
// from this type you endow your class with self-owned storage for a HeapHandle.
// This is a move-only type so that the handle follows the element across moves
// and resizes of the underlying vector.
class BASE_EXPORT InternalHeapHandleStorage {
 public:
  InternalHeapHandleStorage();
  InternalHeapHandleStorage(const InternalHeapHandleStorage&) = delete;
  InternalHeapHandleStorage(InternalHeapHandleStorage&& other) noexcept;
  virtual ~InternalHeapHandleStorage();

  InternalHeapHandleStorage& operator=(const InternalHeapHandleStorage&) =
      delete;
  InternalHeapHandleStorage& operator=(
      InternalHeapHandleStorage&& other) noexcept;

  // Allows external clients to get a pointer to the heap handle. This allows
  // them to remove the element from the heap regardless of its location.
  HeapHandle* handle() const { return handle_.get(); }

  // Implementation of IntrusiveHeap contract. Inlined to keep heap code as fast
  // as possible.
  void SetHeapHandle(HeapHandle handle) {
    DCHECK(handle.IsValid());
    if (handle_) {
      *handle_ = handle;
    }
  }
  void ClearHeapHandle() {
    if (handle_) {
      handle_->reset();
    }
  }
  HeapHandle GetHeapHandle() const {
    if (handle_) {
      return *handle_;
    }
    return HeapHandle::Invalid();
  }

  // Utility functions.
  void swap(InternalHeapHandleStorage& other) noexcept;
  friend void swap(InternalHeapHandleStorage& lhs,
                   InternalHeapHandleStorage& rhs) {
    lhs.swap(rhs);
  }

 private:
  std::unique_ptr<HeapHandle> handle_;
};

// Spiritually akin to a std::pair<T, std::unique_ptr<HeapHandle>>. Can be used
// to wrap arbitrary types and provide them with a HeapHandle, making them
// appropriate for use in an IntrusiveHeap. This is a move-only type.
template <typename T>
class WithHeapHandle : public InternalHeapHandleStorage {
 public:
  WithHeapHandle() = default;
  // Allow implicit conversion of any type that T supports for ease of use with
  // InstrusiveHeap constructors/insert/emplace.
  template <typename U>
  WithHeapHandle(U value) : value_(std::move_if_noexcept(value)) {}
  WithHeapHandle(T&& value) noexcept : value_(std::move(value)) {}
  // Constructor that forwards all arguments along to |value_|.
  template <class... Args>
  explicit WithHeapHandle(Args&&... args);
  WithHeapHandle(const WithHeapHandle&) = delete;
  WithHeapHandle(WithHeapHandle&& other) noexcept = default;
  ~WithHeapHandle() override = default;

  WithHeapHandle& operator=(const WithHeapHandle&) = delete;
  WithHeapHandle& operator=(WithHeapHandle&& other) = default;

  T& value() LIFETIME_BOUND { return value_; }
  const T& value() const LIFETIME_BOUND { return value_; }

  // Utility functions.
  void swap(WithHeapHandle& other) noexcept;
  friend void swap(WithHeapHandle& lhs, WithHeapHandle& rhs) { lhs.swap(rhs); }

  // Comparison operators, for compatibility with ordered STL containers.
  friend bool operator==(const WithHeapHandle& lhs, const WithHeapHandle& rhs) {
    return lhs.value_ == rhs.value_;
  }
  friend auto operator<=>(const WithHeapHandle& lhs,
                          const WithHeapHandle& rhs) {
    return lhs.value_ <=> rhs.value_;
  }

 private:
  T value_;
};

////////////////////////////////////////////////////////////////////////////////
// IMPLEMENTATION DETAILS

namespace intrusive_heap {

BASE_EXPORT inline size_t ParentIndex(size_t i) {
  DCHECK_NE(0u, i);
  return (i - 1) / 2;
}

BASE_EXPORT inline size_t LeftIndex(size_t i) {
  return 2 * i + 1;
}

template <typename HandleType>
bool IsInvalid(const HandleType& handle) {
  return !handle || !handle->IsValid();
}

BASE_EXPORT inline void CheckInvalidOrEqualTo(HeapHandle handle, size_t index) {
  if (handle.IsValid()) {
    DCHECK_EQ(index, handle.index());
  }
}

}  // namespace intrusive_heap

////////////////////////////////////////////////////////////////////////////////
// IntrusiveHeap

template <typename T, typename Compare, typename HeapHandleAccessor>
IntrusiveHeap<T, Compare, HeapHandleAccessor>::IntrusiveHeap(
    const IntrusiveHeap& other)
    : impl_(other.impl_) {
  for (size_t i = 0; i < size(); ++i) {
    SetHeapHandle(i);
  }
}

template <typename T, typename Compare, typename HeapHandleAccessor>
IntrusiveHeap<T, Compare, HeapHandleAccessor>::~IntrusiveHeap() {
  clear();
}

template <typename T, typename Compare, typename HeapHandleAccessor>
IntrusiveHeap<T, Compare, HeapHandleAccessor>&
IntrusiveHeap<T, Compare, HeapHandleAccessor>::operator=(
    IntrusiveHeap&& other) noexcept {
  clear();
  impl_ = std::move(other.impl_);
  return *this;
}

template <typename T, typename Compare, typename HeapHandleAccessor>
IntrusiveHeap<T, Compare, HeapHandleAccessor>&
IntrusiveHeap<T, Compare, HeapHandleAccessor>::operator=(
    const IntrusiveHeap& other) {
  clear();
  impl_ = other.impl_;
  for (size_t i = 0; i < size(); ++i) {
    SetHeapHandle(i);
  }
  return *this;
}

template <typename T, typename Compare, typename HeapHandleAccessor>
IntrusiveHeap<T, Compare, HeapHandleAccessor>&
IntrusiveHeap<T, Compare, HeapHandleAccessor>::operator=(
    std::initializer_list<value_type> ilist) {
  clear();
  insert_range(ilist);
}

template <typename T, typename Compare, typename HeapHandleAccessor>
void IntrusiveHeap<T, Compare, HeapHandleAccessor>::clear() {
  // Make all of the handles invalid before cleaning up the heap.
  for (size_type i = 0; i < size(); ++i) {
    ClearHeapHandle(i);
  }

  // Clear the heap.
  impl_.heap_.clear();
}

template <typename T, typename Compare, typename HeapHandleAccessor>
typename IntrusiveHeap<T, Compare, HeapHandleAccessor>::value_type
IntrusiveHeap<T, Compare, HeapHandleAccessor>::take(size_type pos) {
  // Make a hole by taking the element out of the heap.
  MakeHole(pos);
  value_type val = std::move(impl_.heap_[pos]);

  // If the element being taken is already the last element then the heap
  // doesn't need to be repaired.
  if (pos != GetLastIndex()) {
    MakeHole(GetLastIndex());

    // Move the hole down the heap, filling it with the current leaf at the
    // very end of the heap.
    MoveHoleDownAndFill<WithLeafElement>(
        pos, std::move(impl_.heap_[GetLastIndex()]));
  }

  impl_.heap_.pop_back();

  return val;
}

// This is effectively identical to "take", but it avoids an unnecessary move.
template <typename T, typename Compare, typename HeapHandleAccessor>
void IntrusiveHeap<T, Compare, HeapHandleAccessor>::erase(size_type pos) {
  DCHECK_LT(pos, size());
  // Make a hole by taking the element out of the heap.
  MakeHole(pos);

  // If the element being erased is already the last element then the heap
  // doesn't need to be repaired.
  if (pos != GetLastIndex()) {
    MakeHole(GetLastIndex());

    // Move the hole down the heap, filling it with the current leaf at the
    // very end of the heap.
    MoveHoleDownAndFill<WithLeafElement>(
        pos, std::move_if_noexcept(impl_.heap_[GetLastIndex()]));
  }

  impl_.heap_.pop_back();
}

template <typename T, typename Compare, typename HeapHandleAccessor>
typename IntrusiveHeap<T, Compare, HeapHandleAccessor>::const_iterator
IntrusiveHeap<T, Compare, HeapHandleAccessor>::Update(size_type pos) {
  DCHECK_LT(pos, size());
  MakeHole(pos);

  // Determine if we're >= parent, in which case we may need to go up.
  bool child_greater_eq_parent = false;
  size_type i = 0;
  if (pos > 0) {
    i = intrusive_heap::ParentIndex(pos);
    child_greater_eq_parent = !Less(pos, i);
  }

  if (child_greater_eq_parent) {
    i = MoveHoleUpAndFill(pos, std::move_if_noexcept(impl_.heap_[pos]));
  } else {
    i = MoveHoleDownAndFill<WithElement>(
        pos, std::move_if_noexcept(impl_.heap_[pos]));
  }

  return cbegin() + i;
}

template <typename T, typename Compare, typename HeapHandleAccessor>
void IntrusiveHeap<T, Compare, HeapHandleAccessor>::swap(
    IntrusiveHeap& other) noexcept {
  std::swap(impl_.get_value_compare(), other.impl_.get_value_compare());
  std::swap(impl_.get_heap_handle_access(),
            other.impl_.get_heap_handle_access());
  std::swap(impl_.heap_, other.impl_.heap_);
}

template <typename T, typename Compare, typename HeapHandleAccessor>
typename IntrusiveHeap<T, Compare, HeapHandleAccessor>::size_type
IntrusiveHeap<T, Compare, HeapHandleAccessor>::ToIndex(const_iterator pos) {
  DCHECK(cbegin() <= pos);
  DCHECK(pos <= cend());
  if (pos == cend()) {
    return HeapHandle::kInvalidIndex;
  }
  return pos - cbegin();
}

template <typename T, typename Compare, typename HeapHandleAccessor>
typename IntrusiveHeap<T, Compare, HeapHandleAccessor>::size_type
IntrusiveHeap<T, Compare, HeapHandleAccessor>::ToIndex(
    const_reverse_iterator pos) {
  DCHECK(crbegin() <= pos);
  DCHECK(pos <= crend());
  if (pos == crend()) {
    return HeapHandle::kInvalidIndex;
  }
  return (pos.base() - cbegin()) - 1;
}

template <typename T, typename Compare, typename HeapHandleAccessor>
void IntrusiveHeap<T, Compare, HeapHandleAccessor>::SetHeapHandle(size_type i) {
  impl_.get_heap_handle_access().SetHeapHandle(&impl_.heap_[i], HeapHandle(i));
  intrusive_heap::CheckInvalidOrEqualTo(GetHeapHandle(i), i);
}

template <typename T, typename Compare, typename HeapHandleAccessor>
void IntrusiveHeap<T, Compare, HeapHandleAccessor>::ClearHeapHandle(
    size_type i) {
  impl_.get_heap_handle_access().ClearHeapHandle(&impl_.heap_[i]);
  DCHECK(!GetHeapHandle(i).IsValid());
}

template <typename T, typename Compare, typename HeapHandleAccessor>
HeapHandle IntrusiveHeap<T, Compare, HeapHandleAccessor>::GetHeapHandle(
    size_type i) {
  return impl_.get_heap_handle_access().GetHeapHandle(&impl_.heap_[i]);
}

template <typename T, typename Compare, typename HeapHandleAccessor>
bool IntrusiveHeap<T, Compare, HeapHandleAccessor>::Less(size_type i,
                                                         size_type j) {
  DCHECK_LT(i, size());
  DCHECK_LT(j, size());
  return impl_.get_value_compare()(impl_.heap_[i], impl_.heap_[j]);
}

template <typename T, typename Compare, typename HeapHandleAccessor>
bool IntrusiveHeap<T, Compare, HeapHandleAccessor>::Less(const T& element,
                                                         size_type i) {
  DCHECK_LT(i, size());
  return impl_.get_value_compare()(element, impl_.heap_[i]);
}

template <typename T, typename Compare, typename HeapHandleAccessor>
bool IntrusiveHeap<T, Compare, HeapHandleAccessor>::Less(size_type i,
                                                         const T& element) {
  DCHECK_LT(i, size());
  return impl_.get_value_compare()(impl_.heap_[i], element);
}

template <typename T, typename Compare, typename HeapHandleAccessor>
void IntrusiveHeap<T, Compare, HeapHandleAccessor>::MakeHole(size_type pos) {
  DCHECK_LT(pos, size());
  ClearHeapHandle(pos);
}

template <typename T, typename Compare, typename HeapHandleAccessor>
template <typename U>
void IntrusiveHeap<T, Compare, HeapHandleAccessor>::FillHole(size_type hole_pos,
                                                             U element) {
  // The hole that we're filling may not yet exist. This can occur when
  // inserting a new element into the heap.
  DCHECK_LE(hole_pos, size());
  if (hole_pos == size()) {
    impl_.heap_.push_back(std::move_if_noexcept(element));
  } else {
    impl_.heap_[hole_pos] = std::move_if_noexcept(element);
  }
  SetHeapHandle(hole_pos);
}

template <typename T, typename Compare, typename HeapHandleAccessor>
void IntrusiveHeap<T, Compare, HeapHandleAccessor>::MoveHole(
    size_type new_hole_pos,
    size_type old_hole_pos) {
  // The old hole position may be one past the end. This occurs when a new
  // element is being added.
  DCHECK_NE(new_hole_pos, old_hole_pos);
  DCHECK_LT(new_hole_pos, size());
  DCHECK_LE(old_hole_pos, size());

  if (old_hole_pos == size()) {
    impl_.heap_.push_back(std::move_if_noexcept(impl_.heap_[new_hole_pos]));
  } else {
    impl_.heap_[old_hole_pos] =
        std::move_if_noexcept(impl_.heap_[new_hole_pos]);
  }
  SetHeapHandle(old_hole_pos);
}

template <typename T, typename Compare, typename HeapHandleAccessor>
template <typename U>
typename IntrusiveHeap<T, Compare, HeapHandleAccessor>::size_type
IntrusiveHeap<T, Compare, HeapHandleAccessor>::MoveHoleUpAndFill(
    size_type hole_pos,
    U element) {
  // Moving 1 spot beyond the end is fine. This happens when we insert a new
  // element.
  DCHECK_LE(hole_pos, size());

  // Stop when the element is as far up as it can go.
  while (hole_pos != 0) {
    // If our parent is >= to us, we can stop.
    size_type parent = intrusive_heap::ParentIndex(hole_pos);
    if (!Less(parent, element)) {
      break;
    }

    MoveHole(parent, hole_pos);
    hole_pos = parent;
  }

  FillHole(hole_pos, std::move_if_noexcept(element));
  return hole_pos;
}

template <typename T, typename Compare, typename HeapHandleAccessor>
template <typename FillElementType, typename U>
typename IntrusiveHeap<T, Compare, HeapHandleAccessor>::size_type
IntrusiveHeap<T, Compare, HeapHandleAccessor>::MoveHoleDownAndFill(
    size_type hole_pos,
    U element) {
  DCHECK_LT(hole_pos, size());

  // If we're filling with a leaf, then that leaf element is about to be erased.
  // We pretend that the space doesn't exist in the heap.
  const size_type n = size() - (FillElementType::kIsLeafElement ? 1 : 0);

  DCHECK_LT(hole_pos, n);
  DCHECK(!GetHeapHandle(hole_pos).IsValid());

  while (true) {
    // If this spot has no children, then we've gone down as far as we can go.
    size_type left = intrusive_heap::LeftIndex(hole_pos);
    if (left >= n) {
      break;
    }
    size_type right = left + 1;

    // Get the larger of the potentially two child nodes.
    size_type largest = left;
    if (right < n && Less(left, right)) {
      largest = right;
    }

    // If we're not deterministically moving the element all the way down to
    // become a leaf, then stop when it is >= the largest of the children.
    if (!FillElementType::kIsLeafElement && !Less(element, largest)) {
      break;
    }

    MoveHole(largest, hole_pos);
    hole_pos = largest;
  }

  if (FillElementType::kIsLeafElement) {
    // If we're filling with a leaf node we may need to bubble the leaf back up
    // the tree a bit to repair the heap.
    hole_pos = MoveHoleUpAndFill(hole_pos, std::move_if_noexcept(element));
  } else {
    FillHole(hole_pos, std::move_if_noexcept(element));
  }
  return hole_pos;
}

template <typename T, typename Compare, typename HeapHandleAccessor>
template <typename U>
typename IntrusiveHeap<T, Compare, HeapHandleAccessor>::const_iterator
IntrusiveHeap<T, Compare, HeapHandleAccessor>::InsertImpl(U element) {
  // MoveHoleUpAndFill can tolerate the initial hole being in a slot that
  // doesn't yet exist. It will be created by MoveHole by copy/move, thus
  // removing the need for a default constructor.
  size_type i = MoveHoleUpAndFill(size(), std::move_if_noexcept(element));
  return cbegin() + static_cast<difference_type>(i);
}

template <typename T, typename Compare, typename HeapHandleAccessor>
template <typename U>
typename IntrusiveHeap<T, Compare, HeapHandleAccessor>::const_iterator
IntrusiveHeap<T, Compare, HeapHandleAccessor>::ReplaceImpl(size_type pos,
                                                           U element) {
  // If we're greater than our parent we need to go up, otherwise we may need
  // to go down.
  MakeHole(pos);
  size_type i = 0;
  if (!Less(element, pos)) {
    i = MoveHoleUpAndFill(pos, std::move_if_noexcept(element));
  } else {
    i = MoveHoleDownAndFill<WithElement>(pos, std::move_if_noexcept(element));
  }
  return cbegin() + static_cast<difference_type>(i);
}

template <typename T, typename Compare, typename HeapHandleAccessor>
template <typename U>
typename IntrusiveHeap<T, Compare, HeapHandleAccessor>::const_iterator
IntrusiveHeap<T, Compare, HeapHandleAccessor>::ReplaceTopImpl(U element) {
  MakeHole(0u);
  size_type i =
      MoveHoleDownAndFill<WithElement>(0u, std::move_if_noexcept(element));
  return cbegin() + static_cast<difference_type>(i);
}

////////////////////////////////////////////////////////////////////////////////
// WithHeapHandle

template <typename T>
template <class... Args>
WithHeapHandle<T>::WithHeapHandle(Args&&... args)
    : value_(std::forward<Args>(args)...) {}

template <typename T>
void WithHeapHandle<T>::swap(WithHeapHandle& other) noexcept {
  InternalHeapHandleStorage::swap(other);
  std::swap(value_, other.value_);
}

}  // namespace base

#endif  // BASE_CONTAINERS_INTRUSIVE_HEAP_H_
