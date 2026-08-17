// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.
//
// RepeatedField and RepeatedPtrField are used by generated protocol message
// classes to manipulate repeated fields.  These classes are very similar to
// STL's vector, but include a number of optimizations found to be useful
// specifically in the case of Protocol Buffers.  RepeatedPtrField is
// particularly different from STL vector as it manages ownership of the
// pointers that it contains.
//
// This header covers RepeatedField.

#ifndef GOOGLE_PROTOBUF_REPEATED_FIELD_H__
#define GOOGLE_PROTOBUF_REPEATED_FIELD_H__

#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <iterator>
#include <limits>
#include <memory>
#include <type_traits>
#include <utility>

#include "absl/base/attributes.h"
#include "absl/base/optimization.h"
#include "absl/log/absl_check.h"
#include "absl/meta/type_traits.h"
#include "absl/strings/cord.h"
#include "google/protobuf/arena.h"
#include "google/protobuf/generated_enum_util.h"
#include "google/protobuf/internal_visibility.h"
#include "google/protobuf/message_lite.h"
#include "google/protobuf/port.h"
#include "google/protobuf/repeated_ptr_field.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

#ifdef SWIG
#error "You cannot SWIG proto headers"
#endif

namespace google {
namespace protobuf {

class Message;
class UnknownField;  // For the allowlist

namespace internal {

template <typename T, int kHeapRepHeaderSize>
constexpr int RepeatedFieldLowerClampLimit() {
  // The header is padded to be at least `sizeof(T)` when it would be smaller
  // otherwise.
  static_assert(sizeof(T) <= kHeapRepHeaderSize, "");
  // We want to pad the minimum size to be a power of two bytes, including the
  // header.
  // The first allocation is kHeapRepHeaderSize bytes worth of elements for a
  // total of 2*kHeapRepHeaderSize bytes. For an 8-byte header, we allocate 8
  // bool, 2 ints, or 1 int64.
  return kHeapRepHeaderSize / sizeof(T);
}

// kRepeatedFieldUpperClampLimit is the lowest signed integer value that
// overflows when multiplied by 2 (which is undefined behavior). Sizes above
// this will clamp to the maximum int value instead of following exponential
// growth when growing a repeated field.
#if defined(__cpp_inline_variables)
inline constexpr int kRepeatedFieldUpperClampLimit =
#else
constexpr int kRepeatedFieldUpperClampLimit =
#endif
    (std::numeric_limits<int>::max() / 2) + 1;

template <typename Element>
class RepeatedIterator;

// Sentinel base class.
struct RepeatedFieldBase {};

// We can't skip the destructor for, e.g., arena allocated RepeatedField<Cord>.
template <typename Element,
          bool Trivial = Arena::is_destructor_skippable<Element>::value>
struct RepeatedFieldDestructorSkippableBase : RepeatedFieldBase {};

template <typename Element>
struct RepeatedFieldDestructorSkippableBase<Element, true> : RepeatedFieldBase {
  using DestructorSkippable_ = void;
};

template <size_t kMinSize>
struct HeapRep {
  // Avoid 'implicitly deleted dtor' warnings on certain compilers.
  ~HeapRep() = delete;

  void* elements() { return this + 1; }

  // Align to 8 as sanitizers are picky on the alignment of containers to start
  // at 8 byte offsets even when compiling for 32 bit platforms.
  union {
    alignas(8) Arena* arena;
    // We pad the header to be at least `sizeof(Element)` so that we have
    // power-of-two sized allocations, which enables Arena optimizations.
    char padding[kMinSize];
  };
};

// We use small object optimization (SOO) to store elements inline when possible
// for small repeated fields. We do so in order to avoid memory indirections.
// Note that SOO is disabled on 32-bit platforms due to alignment limitations.

// SOO data is stored in the same space as the size/capacity ints.
enum { kSooCapacityBytes = 2 * sizeof(int) };

// Arena/elements pointers are aligned to at least kSooPtrAlignment bytes so we
// can use the lower bits to encode whether we're in SOO mode and if so, the
// SOO size. NOTE: we also tried using all kSooPtrMask bits to encode SOO size
// and use all ones as a sentinel value for non-SOO mode, but that was slower in
// benchmarks/loadtests.
enum { kSooPtrAlignment = 8 };
// The mask for the size bits in SOO mode, and also a sentinel value indicating
// that the field is not in SOO mode.
enum { kSooPtrMask = ~(kSooPtrAlignment - 1) };
// This bit is 0 when in SOO mode and 1 when in non-SOO mode.
enum { kNotSooBit = kSooPtrAlignment >> 1 };
// These bits are used to encode the size when in SOO mode (sizes are 0-3).
enum { kSooSizeMask = kNotSooBit - 1 };

// The number of elements that can be stored in the SOO rep. On 64-bit
// platforms, this is 1 for int64_t, 2 for int32_t, 3 for bool, and 0 for
// absl::Cord. We return 0 to disable SOO on 32-bit platforms.
constexpr int SooCapacityElements(size_t element_size) {
  if (sizeof(void*) < 8) return 0;
  return std::min<int>(kSooCapacityBytes / element_size, kSooSizeMask);
}

struct LongSooRep {
  // Returns char* rather than void* so callers can do pointer arithmetic.
  char* elements() const {
    auto ret = reinterpret_cast<char*>(elements_int & kSooPtrMask);
    ABSL_DCHECK_NE(ret, nullptr);
    return ret;
  }

  uintptr_t elements_int;
  int size;
  int capacity;
};
struct ShortSooRep {
  constexpr ShortSooRep() = default;
  explicit ShortSooRep(Arena* arena)
      : arena_and_size(reinterpret_cast<uintptr_t>(arena)) {
    ABSL_DCHECK_EQ(size(), 0);
  }

  int size() const { return arena_and_size & kSooSizeMask; }
  bool is_soo() const { return (arena_and_size & kNotSooBit) == 0; }

  uintptr_t arena_and_size = 0;
  union {
    char data[kSooCapacityBytes];
    // NOTE: in some language versions, we can't have a constexpr constructor
    // if we don't initialize all fields, but `data` doesn't need to be
    // initialized so initialize an empty dummy variable instead.
    std::true_type dummy = {};
  };
};
struct SooRep {
  constexpr SooRep() : short_rep() {}
  explicit SooRep(Arena* arena) : short_rep(arena) {}

  bool is_soo() const {
    static_assert(sizeof(LongSooRep) == sizeof(ShortSooRep), "");
    static_assert(offsetof(SooRep, long_rep) == offsetof(SooRep, short_rep),
                  "");
    static_assert(offsetof(LongSooRep, elements_int) ==
                      offsetof(ShortSooRep, arena_and_size),
                  "");
    return short_rep.is_soo();
  }
  Arena* soo_arena() const {
    ABSL_DCHECK(is_soo());
    return reinterpret_cast<Arena*>(short_rep.arena_and_size & kSooPtrMask);
  }
  int size(bool is_soo) const {
    ABSL_DCHECK_EQ(is_soo, this->is_soo());
#if !defined(__clang__) && defined(__GNUC__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmaybe-uninitialized"
#endif
    return is_soo ? short_rep.size() : long_rep.size;
#if !defined(__clang__) && defined(__GNUC__)
#pragma GCC diagnostic pop
#endif
  }
  void set_size(bool is_soo, int size) {
    ABSL_DCHECK_EQ(is_soo, this->is_soo());
    if (is_soo) {
      ABSL_DCHECK_LE(size, kSooSizeMask);
      short_rep.arena_and_size &= kSooPtrMask;
      short_rep.arena_and_size |= size;
    } else {
      long_rep.size = size;
    }
  }
  // Initializes the SooRep in non-SOO mode with the given capacity and heap
  // allocation.
  void set_non_soo(bool was_soo, int capacity, void* elements) {
    ABSL_DCHECK_EQ(was_soo, is_soo());
    ABSL_DCHECK_NE(elements, nullptr);
    ABSL_DCHECK_EQ(reinterpret_cast<uintptr_t>(elements) % kSooPtrAlignment,
                   uintptr_t{0});
    if (was_soo) long_rep.size = short_rep.size();
    long_rep.capacity = capacity;
    long_rep.elements_int = reinterpret_cast<uintptr_t>(elements) | kNotSooBit;
  }

  union {
    LongSooRep long_rep;
    ShortSooRep short_rep;
  };
};

}  // namespace internal

// RepeatedField is used to represent repeated fields of a primitive type (in
// other words, everything except strings and nested Messages).  Most users will
// not ever use a RepeatedField directly; they will use the get-by-index,
// set-by-index, and add accessors that are generated for all repeated fields.
// Actually, in addition to primitive types, we use RepeatedField for repeated
// Cords, because the Cord class is in fact just a reference-counted pointer.
// We have to specialize several methods in the Cord case to get the memory
// management right; e.g. swapping when appropriate, etc.
template <typename Element>
class RepeatedField final
    : private internal::RepeatedFieldDestructorSkippableBase<Element> {
  static_assert(
      alignof(Arena) >= alignof(Element),
      "We only support types that have an alignment smaller than Arena");
  static_assert(!std::is_const<Element>::value,
                "We do not support const value types.");
  static_assert(!std::is_volatile<Element>::value,
                "We do not support volatile value types.");
  static_assert(!std::is_pointer<Element>::value,
                "We do not support pointer value types.");
  static_assert(!std::is_reference<Element>::value,
                "We do not support reference value types.");
  static constexpr PROTOBUF_ALWAYS_INLINE void StaticValidityCheck() {
    static_assert(
        absl::disjunction<internal::is_supported_integral_type<Element>,
                          internal::is_supported_floating_point_type<Element>,
                          std::is_same<absl::Cord, Element>,
                          std::is_same<UnknownField, Element>,
                          is_proto_enum<Element>>::value,
        "We only support non-string scalars in RepeatedField.");
  }

 public:
  using value_type = Element;
  using size_type = int;
  using difference_type = ptrdiff_t;
  using reference = Element&;
  using const_reference = const Element&;
  using pointer = Element*;
  using const_pointer = const Element*;
  using iterator = internal::RepeatedIterator<Element>;
  using const_iterator = internal::RepeatedIterator<const Element>;
  using reverse_iterator = std::reverse_iterator<iterator>;
  using const_reverse_iterator = std::reverse_iterator<const_iterator>;

  constexpr RepeatedField();
  RepeatedField(const RepeatedField& rhs) : RepeatedField(nullptr, rhs) {}

  // TODO: make this constructor private
  explicit RepeatedField(Arena* arena);

  template <typename Iter,
            typename = typename std::enable_if<std::is_constructible<
                Element, decltype(*std::declval<Iter>())>::value>::type>
  RepeatedField(Iter begin, Iter end);

  // Arena enabled constructors: for internal use only.
  RepeatedField(internal::InternalVisibility, Arena* arena)
      : RepeatedField(arena) {}
  RepeatedField(internal::InternalVisibility, Arena* arena,
                const RepeatedField& rhs)
      : RepeatedField(arena, rhs) {}

  RepeatedField& operator=(const RepeatedField& other)
      ABSL_ATTRIBUTE_LIFETIME_BOUND;

  RepeatedField(RepeatedField&& rhs) noexcept
      : RepeatedField(nullptr, std::move(rhs)) {}
  RepeatedField& operator=(RepeatedField&& other) noexcept
      ABSL_ATTRIBUTE_LIFETIME_BOUND;

  ~RepeatedField();

  bool empty() const;
  int size() const;

  const_reference Get(int index) const ABSL_ATTRIBUTE_LIFETIME_BOUND;
  pointer Mutable(int index) ABSL_ATTRIBUTE_LIFETIME_BOUND;

  const_reference operator[](int index) const ABSL_ATTRIBUTE_LIFETIME_BOUND {
    return Get(index);
  }
  reference operator[](int index) ABSL_ATTRIBUTE_LIFETIME_BOUND {
    return *Mutable(index);
  }

  const_reference at(int index) const ABSL_ATTRIBUTE_LIFETIME_BOUND;
  reference at(int index) ABSL_ATTRIBUTE_LIFETIME_BOUND;

  void Set(int index, const Element& value);
  void Add(Element value);

  // Appends a new element and returns a pointer to it.
  // The new element is uninitialized if |Element| is a POD type.
  pointer Add() ABSL_ATTRIBUTE_LIFETIME_BOUND;
  // Appends elements in the range [begin, end) after reserving
  // the appropriate number of elements.
  template <typename Iter>
  void Add(Iter begin, Iter end);

  // Removes the last element in the array.
  void RemoveLast();

  // Extracts elements with indices in "[start .. start+num-1]".
  // Copies them into "elements[0 .. num-1]" if "elements" is not nullptr.
  // Caution: also moves elements with indices [start+num ..].
  // Calling this routine inside a loop can cause quadratic behavior.
  void ExtractSubrange(int start, int num, Element* elements);

  ABSL_ATTRIBUTE_REINITIALIZES void Clear();

  // Appends the elements from `other` after this instance.
  // The end result length will be `other.size() + this->size()`.
  void MergeFrom(const RepeatedField& other);

  // Replaces the contents with a copy of the elements from `other`.
  ABSL_ATTRIBUTE_REINITIALIZES void CopyFrom(const RepeatedField& other);

  // Replaces the contents with RepeatedField(begin, end).
  template <typename Iter>
  ABSL_ATTRIBUTE_REINITIALIZES void Assign(Iter begin, Iter end);

  // Reserves space to expand the field to at least the given size.  If the
  // array is grown, it will always be at least doubled in size.
  void Reserve(int new_size);

  // Resizes the RepeatedField to a new, smaller size.  This is O(1).
  // Except for RepeatedField<Cord>, for which it is O(size-new_size).
  void Truncate(int new_size);

  void AddAlreadyReserved(Element value);
  int Capacity() const;

  // Adds `n` elements to this instance asserting there is enough capacity.
  // The added elements are uninitialized if `Element` is trivial.
  pointer AddAlreadyReserved() ABSL_ATTRIBUTE_LIFETIME_BOUND;
  pointer AddNAlreadyReserved(int n) ABSL_ATTRIBUTE_LIFETIME_BOUND;

  // Like STL resize.  Uses value to fill appended elements.
  // Like Truncate() if new_size <= size(), otherwise this is
  // O(new_size - size()).
  void Resize(size_type new_size, const Element& value);

  // Gets the underlying array.  This pointer is possibly invalidated by
  // any add or remove operation.
  pointer mutable_data() ABSL_ATTRIBUTE_LIFETIME_BOUND;
  const_pointer data() const ABSL_ATTRIBUTE_LIFETIME_BOUND;

  // Swaps entire contents with "other". If they are separate arenas, then
  // copies data between each other.
  void Swap(RepeatedField* other);

  // Swaps two elements.
  void SwapElements(int index1, int index2);

  iterator begin() ABSL_ATTRIBUTE_LIFETIME_BOUND;
  const_iterator begin() const ABSL_ATTRIBUTE_LIFETIME_BOUND;
  const_iterator cbegin() const ABSL_ATTRIBUTE_LIFETIME_BOUND;
  iterator end() ABSL_ATTRIBUTE_LIFETIME_BOUND;
  const_iterator end() const ABSL_ATTRIBUTE_LIFETIME_BOUND;
  const_iterator cend() const ABSL_ATTRIBUTE_LIFETIME_BOUND;

  // Reverse iterator support
  reverse_iterator rbegin() ABSL_ATTRIBUTE_LIFETIME_BOUND {
    return reverse_iterator(end());
  }
  const_reverse_iterator rbegin() const ABSL_ATTRIBUTE_LIFETIME_BOUND {
    return const_reverse_iterator(end());
  }
  reverse_iterator rend() ABSL_ATTRIBUTE_LIFETIME_BOUND {
    return reverse_iterator(begin());
  }
  const_reverse_iterator rend() const ABSL_ATTRIBUTE_LIFETIME_BOUND {
    return const_reverse_iterator(begin());
  }

  // Returns the number of bytes used by the repeated field, excluding
  // sizeof(*this)
  size_t SpaceUsedExcludingSelfLong() const;

  int SpaceUsedExcludingSelf() const {
    return internal::ToIntSize(SpaceUsedExcludingSelfLong());
  }

  // Removes the element referenced by position.
  //
  // Returns an iterator to the element immediately following the removed
  // element.
  //
  // Invalidates all iterators at or after the removed element, including end().
  iterator erase(const_iterator position) ABSL_ATTRIBUTE_LIFETIME_BOUND;

  // Removes the elements in the range [first, last).
  //
  // Returns an iterator to the element immediately following the removed range.
  //
  // Invalidates all iterators at or after the removed range, including end().
  iterator erase(const_iterator first,
                 const_iterator last) ABSL_ATTRIBUTE_LIFETIME_BOUND;

  // Gets the Arena on which this RepeatedField stores its elements.
  // Note: this can be inaccurate for split default fields so we make this
  // function non-const.
  inline Arena* GetArena() { return GetArena(is_soo()); }

  // For internal use only.
  //
  // This is public due to it being called by generated code.
  inline void InternalSwap(RepeatedField* other);

  static constexpr size_t InternalGetArenaOffset(internal::InternalVisibility) {
    return PROTOBUF_FIELD_OFFSET(RepeatedField, soo_rep_) +
           PROTOBUF_FIELD_OFFSET(internal::ShortSooRep, arena_and_size);
  }

 private:
  using InternalArenaConstructable_ = void;
  // We use std::max in order to share template instantiations between
  // different element types.
  using HeapRep = internal::HeapRep<std::max<size_t>(sizeof(Element), 8)>;

  template <typename T>
  friend class Arena::InternalHelper;

  friend class Arena;

  static constexpr int kSooCapacityElements =
      internal::SooCapacityElements(sizeof(Element));

  static constexpr int kInitialSize = 0;
  static PROTOBUF_CONSTEXPR const size_t kHeapRepHeaderSize = sizeof(HeapRep);

  RepeatedField(Arena* arena, const RepeatedField& rhs);
  RepeatedField(Arena* arena, RepeatedField&& rhs);

  inline Arena* GetArena(bool is_soo) const {
    return is_soo ? soo_rep_.soo_arena() : heap_rep()->arena;
  }

  bool is_soo() const { return soo_rep_.is_soo(); }
  int size(bool is_soo) const { return soo_rep_.size(is_soo); }
  int Capacity(bool is_soo) const {
#if !defined(__clang__) && defined(__GNUC__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmaybe-uninitialized"
#endif
    return is_soo ? kSooCapacityElements : soo_rep_.long_rep.capacity;
#if !defined(__clang__) && defined(__GNUC__)
#pragma GCC diagnostic pop
#endif
  }
  void set_size(bool is_soo, int size) {
    ABSL_DCHECK_LE(size, Capacity(is_soo));
    soo_rep_.set_size(is_soo, size);
  }

  // Swaps entire contents with "other". Should be called only if the caller can
  // guarantee that both repeated fields are on the same arena or are on the
  // heap. Swapping between different arenas is disallowed and caught by a
  // ABSL_DCHECK (see API docs for details).
  void UnsafeArenaSwap(RepeatedField* other);

  // Copy constructs `n` instances in place into the array `dst`.
  // This function is identical to `std::uninitialized_copy_n(src, n, dst)`
  // except that we explicit declare the memory to not be aliased, which will
  // result in `memcpy` code generation instead of `memmove` for trivial types.
  static inline void UninitializedCopyN(const Element* PROTOBUF_RESTRICT src,
                                        int n, Element* PROTOBUF_RESTRICT dst) {
    std::uninitialized_copy_n(src, n, dst);
  }

  // Copy constructs `[begin, end)` instances in place into the array `dst`.
  // See above `UninitializedCopyN()` function comments for more information.
  template <typename Iter>
  static inline void UninitializedCopy(Iter begin, Iter end,
                                       Element* PROTOBUF_RESTRICT dst) {
    std::uninitialized_copy(begin, end, dst);
  }

  // Destroys all elements in [begin, end).
  // This function does nothing if `Element` is trivial.
  static void Destroy(const Element* begin, const Element* end) {
    if (!std::is_trivial<Element>::value) {
      std::for_each(begin, end, [&](const Element& e) { e.~Element(); });
    }
  }

  template <typename Iter>
  void AddForwardIterator(Iter begin, Iter end);

  template <typename Iter>
  void AddInputIterator(Iter begin, Iter end);

  // Reserves space to expand the field to at least the given size.
  // If the array is grown, it will always be at least doubled in size.
  // If `annotate_size` is true (the default), then this function will annotate
  // the old container from `old_size` to `Capacity()` (unpoison memory)
  // directly before it is being released, and annotate the new container from
  // `Capacity()` to `old_size` (poison unused memory).
  void Grow(bool was_soo, int old_size, int new_size);
  void GrowNoAnnotate(bool was_soo, int old_size, int new_size);

  // Annotates a change in size of this instance. This function should be called
  // with (capacity, old_size) after new memory has been allocated and filled
  // from previous memory, and UnpoisonBuffer() should be called right before
  // (previously annotated) memory is released.
  void AnnotateSize(int old_size, int new_size) const {
    if (old_size != new_size) {
      ABSL_ATTRIBUTE_UNUSED const bool is_soo = this->is_soo();
      ABSL_ATTRIBUTE_UNUSED const Element* elem = unsafe_elements(is_soo);
      // ABSL_ANNOTATE_CONTIGUOUS_CONTAINER(elem, elem + Capacity(is_soo),
      //                                   elem + old_size, elem + new_size);
      if (new_size < old_size) {
        ABSL_ANNOTATE_MEMORY_IS_UNINITIALIZED(
            elem + new_size, (old_size - new_size) * sizeof(Element));
      }
    }
  }

  // Unpoisons the memory buffer.
  void UnpoisonBuffer() const {
    AnnotateSize(size(), Capacity());
    if (is_soo()) {
      // We need to manually unpoison the SOO buffer because in reflection for
      // split repeated fields, we poison the whole SOO buffer even when we
      // don't actually use the whole SOO buffer (e.g. for RepeatedField<bool>).
      internal::UnpoisonMemoryRegion(soo_rep_.short_rep.data,
                                     sizeof(soo_rep_.short_rep.data));
    }
  }

  // Replaces size with new_size and returns the previous value of
  // size. This function is intended to be the only place where
  // size is modified, with the exception of `AddInputIterator()`
  // where the size of added items is not known in advance.
  inline int ExchangeCurrentSize(bool is_soo, int new_size) {
    const int prev_size = size(is_soo);
    AnnotateSize(prev_size, new_size);
    set_size(is_soo, new_size);
    return prev_size;
  }

  // Returns a pointer to elements array.
  // pre-condition: Capacity() > 0.
  Element* elements(bool is_soo) {
    ABSL_DCHECK_GT(Capacity(is_soo), 0);
    return unsafe_elements(is_soo);
  }
  const Element* elements(bool is_soo) const {
    return const_cast<RepeatedField*>(this)->elements(is_soo);
  }

  // Returns a pointer to elements array if it exists; otherwise an invalid
  // pointer is returned. This only happens for empty repeated fields, where you
  // can't dereference this pointer anyway (it's empty).
  Element* unsafe_elements(bool is_soo) {
    return is_soo ? reinterpret_cast<Element*>(soo_rep_.short_rep.data)
                  : reinterpret_cast<Element*>(soo_rep_.long_rep.elements());
  }
  const Element* unsafe_elements(bool is_soo) const {
    return const_cast<RepeatedField*>(this)->unsafe_elements(is_soo);
  }

  // Returns a pointer to the HeapRep struct.
  // pre-condition: the HeapRep must have been allocated, ie !is_soo().
  HeapRep* heap_rep() const {
    ABSL_DCHECK(!is_soo());
    return reinterpret_cast<HeapRep*>(soo_rep_.long_rep.elements() -
                                      kHeapRepHeaderSize);
  }

  // Internal helper to delete all elements and deallocate the storage.
  template <bool in_destructor = false>
  void InternalDeallocate() {
    ABSL_DCHECK(!is_soo());
    const size_t bytes = Capacity(false) * sizeof(Element) + kHeapRepHeaderSize;
    if (heap_rep()->arena == nullptr) {
      internal::SizedDelete(heap_rep(), bytes);
    } else if (!in_destructor) {
      // If we are in the destructor, we might be being destroyed as part of
      // the arena teardown. We can't try and return blocks to the arena then.
      heap_rep()->arena->ReturnArrayMemory(heap_rep(), bytes);
    }
  }

  // A note on the representation here (see also comment below for
  // RepeatedPtrFieldBase's struct HeapRep):
  //
  // We maintain the same sizeof(RepeatedField) as before we added arena support
  // so that we do not degrade performance by bloating memory usage. Directly
  // adding an arena_ element to RepeatedField is quite costly. By using
  // indirection in this way, we keep the same size when the RepeatedField is
  // empty (common case), and add only an 8-byte header to the elements array
  // when non-empty. We make sure to place the size fields directly in the
  // RepeatedField class to avoid costly cache misses due to the indirection.
  internal::SooRep soo_rep_{};
};

// implementation ====================================================

template <typename Element>
constexpr RepeatedField<Element>::RepeatedField() {
  StaticValidityCheck();
#ifdef __cpp_lib_is_constant_evaluated
  if (!std::is_constant_evaluated()) {
    AnnotateSize(kSooCapacityElements, 0);
  }
#endif  // __cpp_lib_is_constant_evaluated
}

template <typename Element>
inline RepeatedField<Element>::RepeatedField(Arena* arena) : soo_rep_(arena) {
  StaticValidityCheck();
  AnnotateSize(kSooCapacityElements, 0);
}

template <typename Element>
inline RepeatedField<Element>::RepeatedField(Arena* arena,
                                             const RepeatedField& rhs)
    : soo_rep_(arena) {
  StaticValidityCheck();
  AnnotateSize(kSooCapacityElements, 0);
  const bool rhs_is_soo = rhs.is_soo();
  if (auto size = rhs.size(rhs_is_soo)) {
    bool is_soo = true;
    if (size > kSooCapacityElements) {
      Grow(is_soo, 0, size);
      is_soo = false;
    }
    ExchangeCurrentSize(is_soo, size);
    UninitializedCopyN(rhs.elements(rhs_is_soo), size, unsafe_elements(is_soo));
  }
}

template <typename Element>
template <typename Iter, typename>
RepeatedField<Element>::RepeatedField(Iter begin, Iter end) {
  StaticValidityCheck();
  AnnotateSize(kSooCapacityElements, 0);
  Add(begin, end);
}

template <typename Element>
RepeatedField<Element>::~RepeatedField() {
  StaticValidityCheck();
  const bool is_soo = this->is_soo();
#ifndef NDEBUG
  // Try to trigger segfault / asan failure in non-opt builds if arena_
  // lifetime has ended before the destructor.
  auto arena = GetArena(is_soo);
  if (arena) (void)arena->SpaceAllocated();
#endif
  const int size = this->size(is_soo);
  if (size > 0) {
    Element* elem = unsafe_elements(is_soo);
    Destroy(elem, elem + size);
  }
  UnpoisonBuffer();
  if (!is_soo) InternalDeallocate<true>();
}

template <typename Element>
inline RepeatedField<Element>& RepeatedField<Element>::operator=(
    const RepeatedField& other) ABSL_ATTRIBUTE_LIFETIME_BOUND {
  if (this != &other) CopyFrom(other);
  return *this;
}

template <typename Element>
inline RepeatedField<Element>::RepeatedField(Arena* arena, RepeatedField&& rhs)
    : RepeatedField(arena) {
  if (internal::CanMoveWithInternalSwap(arena, rhs.GetArena())) {
    InternalSwap(&rhs);
  } else {
    // We don't just call Swap(&rhs) here because it would perform 3 copies if
    // rhs is on a different arena.
    CopyFrom(rhs);
  }
}

template <typename Element>
inline RepeatedField<Element>& RepeatedField<Element>::operator=(
    RepeatedField&& other) noexcept ABSL_ATTRIBUTE_LIFETIME_BOUND {
  // We don't just call Swap(&other) here because it would perform 3 copies if
  // the two fields are on different arenas.
  if (this != &other) {
    if (internal::CanMoveWithInternalSwap(GetArena(), other.GetArena())) {
      InternalSwap(&other);
    } else {
      CopyFrom(other);
    }
  }
  return *this;
}

template <typename Element>
inline bool RepeatedField<Element>::empty() const {
  return size() == 0;
}

template <typename Element>
inline int RepeatedField<Element>::size() const {
  return size(is_soo());
}

template <typename Element>
inline int RepeatedField<Element>::Capacity() const {
  return Capacity(is_soo());
}

template <typename Element>
inline void RepeatedField<Element>::AddAlreadyReserved(Element value) {
  const bool is_soo = this->is_soo();
  const int old_size = size(is_soo);
  ABSL_DCHECK_LT(old_size, Capacity(is_soo));
  void* p = elements(is_soo) + ExchangeCurrentSize(is_soo, old_size + 1);
  ::new (p) Element(std::move(value));
}

template <typename Element>
inline Element* RepeatedField<Element>::AddAlreadyReserved()
    ABSL_ATTRIBUTE_LIFETIME_BOUND {
  const bool is_soo = this->is_soo();
  const int old_size = size(is_soo);
  ABSL_DCHECK_LT(old_size, Capacity(is_soo));
  // new (p) <TrivialType> compiles into nothing: this is intentional as this
  // function is documented to return uninitialized data for trivial types.
  void* p = elements(is_soo) + ExchangeCurrentSize(is_soo, old_size + 1);
  return ::new (p) Element;
}

template <typename Element>
inline Element* RepeatedField<Element>::AddNAlreadyReserved(int n)
    ABSL_ATTRIBUTE_LIFETIME_BOUND {
  const bool is_soo = this->is_soo();
  const int old_size = size(is_soo);
  ABSL_ATTRIBUTE_UNUSED const int capacity = Capacity(is_soo);
  ABSL_DCHECK_GE(capacity - old_size, n) << capacity << ", " << old_size;
  Element* p =
      unsafe_elements(is_soo) + ExchangeCurrentSize(is_soo, old_size + n);
  for (Element *begin = p, *end = p + n; begin != end; ++begin) {
    new (static_cast<void*>(begin)) Element;
  }
  return p;
}

template <typename Element>
inline void RepeatedField<Element>::Resize(int new_size, const Element& value) {
  ABSL_DCHECK_GE(new_size, 0);
  bool is_soo = this->is_soo();
  const int old_size = size(is_soo);
  if (new_size > old_size) {
    if (new_size > Capacity(is_soo)) {
      Grow(is_soo, old_size, new_size);
      is_soo = false;
    }
    Element* elem = elements(is_soo);
    Element* first = elem + ExchangeCurrentSize(is_soo, new_size);
    std::uninitialized_fill(first, elem + new_size, value);
  } else if (new_size < old_size) {
    Element* elem = unsafe_elements(is_soo);
    Destroy(elem + new_size, elem + old_size);
    ExchangeCurrentSize(is_soo, new_size);
  }
}

template <typename Element>
inline const Element& RepeatedField<Element>::Get(int index) const
    ABSL_ATTRIBUTE_LIFETIME_BOUND {
  ABSL_DCHECK_GE(index, 0);
  ABSL_DCHECK_LT(index, size());
  return elements(is_soo())[index];
}

template <typename Element>
inline const Element& RepeatedField<Element>::at(int index) const
    ABSL_ATTRIBUTE_LIFETIME_BOUND {
  ABSL_CHECK_GE(index, 0);
  ABSL_CHECK_LT(index, size());
  return elements(is_soo())[index];
}

template <typename Element>
inline Element& RepeatedField<Element>::at(int index)
    ABSL_ATTRIBUTE_LIFETIME_BOUND {
  ABSL_CHECK_GE(index, 0);
  ABSL_CHECK_LT(index, size());
  return elements(is_soo())[index];
}

template <typename Element>
inline Element* RepeatedField<Element>::Mutable(int index)
    ABSL_ATTRIBUTE_LIFETIME_BOUND {
  ABSL_DCHECK_GE(index, 0);
  ABSL_DCHECK_LT(index, size());
  return &elements(is_soo())[index];
}

template <typename Element>
inline void RepeatedField<Element>::Set(int index, const Element& value) {
  *Mutable(index) = value;
}

template <typename Element>
inline void RepeatedField<Element>::Add(Element value) {
  bool is_soo = this->is_soo();
  const int old_size = size(is_soo);
  int capacity = Capacity(is_soo);
  Element* elem = unsafe_elements(is_soo);
  if (ABSL_PREDICT_FALSE(old_size == capacity)) {
    Grow(is_soo, old_size, old_size + 1);
    is_soo = false;
    capacity = Capacity(is_soo);
    elem = unsafe_elements(is_soo);
  }
  int new_size = old_size + 1;
  void* p = elem + ExchangeCurrentSize(is_soo, new_size);
  ::new (p) Element(std::move(value));

  // The below helps the compiler optimize dense loops.
  // Note: we can't call functions in PROTOBUF_ASSUME so use local variables.
  ABSL_ATTRIBUTE_UNUSED const bool final_is_soo = this->is_soo();
  PROTOBUF_ASSUME(is_soo == final_is_soo);
  ABSL_ATTRIBUTE_UNUSED const int final_size = size(is_soo);
  PROTOBUF_ASSUME(new_size == final_size);
  ABSL_ATTRIBUTE_UNUSED Element* const final_elements = unsafe_elements(is_soo);
  PROTOBUF_ASSUME(elem == final_elements);
  ABSL_ATTRIBUTE_UNUSED const int final_capacity = Capacity(is_soo);
  PROTOBUF_ASSUME(capacity == final_capacity);
}

template <typename Element>
inline Element* RepeatedField<Element>::Add() ABSL_ATTRIBUTE_LIFETIME_BOUND {
  bool is_soo = this->is_soo();
  const int old_size = size(is_soo);
  if (ABSL_PREDICT_FALSE(old_size == Capacity())) {
    Grow(is_soo, old_size, old_size + 1);
    is_soo = false;
  }
  void* p = unsafe_elements(is_soo) + ExchangeCurrentSize(is_soo, old_size + 1);
  return ::new (p) Element;
}

template <typename Element>
template <typename Iter>
inline void RepeatedField<Element>::AddForwardIterator(Iter begin, Iter end) {
  bool is_soo = this->is_soo();
  const int old_size = size(is_soo);
  int capacity = Capacity(is_soo);
  Element* elem = unsafe_elements(is_soo);
  int new_size = old_size + static_cast<int>(std::distance(begin, end));
  if (ABSL_PREDICT_FALSE(new_size > capacity)) {
    Grow(is_soo, old_size, new_size);
    is_soo = false;
    elem = unsafe_elements(is_soo);
    capacity = Capacity(is_soo);
  }
  UninitializedCopy(begin, end, elem + ExchangeCurrentSize(is_soo, new_size));

  // The below helps the compiler optimize dense loops.
  // Note: we can't call functions in PROTOBUF_ASSUME so use local variables.
  ABSL_ATTRIBUTE_UNUSED const bool final_is_soo = this->is_soo();
  PROTOBUF_ASSUME(is_soo == final_is_soo);
  ABSL_ATTRIBUTE_UNUSED const int final_size = size(is_soo);
  PROTOBUF_ASSUME(new_size == final_size);
  ABSL_ATTRIBUTE_UNUSED Element* const final_elements = unsafe_elements(is_soo);
  PROTOBUF_ASSUME(elem == final_elements);
  ABSL_ATTRIBUTE_UNUSED const int final_capacity = Capacity(is_soo);
  PROTOBUF_ASSUME(capacity == final_capacity);
}

template <typename Element>
template <typename Iter>
inline void RepeatedField<Element>::AddInputIterator(Iter begin, Iter end) {
  bool is_soo = this->is_soo();
  int size = this->size(is_soo);
  int capacity = Capacity(is_soo);
  Element* elem = unsafe_elements(is_soo);
  Element* first = elem + size;
  Element* last = elem + capacity;
  UnpoisonBuffer();

  while (begin != end) {
    if (ABSL_PREDICT_FALSE(first == last)) {
      size = first - elem;
      GrowNoAnnotate(is_soo, size, size + 1);
      is_soo = false;
      elem = unsafe_elements(is_soo);
      capacity = Capacity(is_soo);
      first = elem + size;
      last = elem + capacity;
    }
    ::new (static_cast<void*>(first)) Element(*begin);
    ++begin;
    ++first;
  }

  const int new_size = first - elem;
  set_size(is_soo, new_size);
  AnnotateSize(capacity, new_size);
}

template <typename Element>
template <typename Iter>
inline void RepeatedField<Element>::Add(Iter begin, Iter end) {
  if (std::is_base_of<
          std::forward_iterator_tag,
          typename std::iterator_traits<Iter>::iterator_category>::value) {
    AddForwardIterator(begin, end);
  } else {
    AddInputIterator(begin, end);
  }
}

template <typename Element>
inline void RepeatedField<Element>::RemoveLast() {
  const bool is_soo = this->is_soo();
  const int old_size = size(is_soo);
  ABSL_DCHECK_GT(old_size, 0);
  elements(is_soo)[old_size - 1].~Element();
  ExchangeCurrentSize(is_soo, old_size - 1);
}

template <typename Element>
void RepeatedField<Element>::ExtractSubrange(int start, int num,
                                             Element* elements) {
  ABSL_DCHECK_GE(start, 0);
  ABSL_DCHECK_GE(num, 0);
  const bool is_soo = this->is_soo();
  const int old_size = size(is_soo);
  ABSL_DCHECK_LE(start + num, old_size);
  Element* elem = unsafe_elements(is_soo);

  // Save the values of the removed elements if requested.
  if (elements != nullptr) {
    for (int i = 0; i < num; ++i) elements[i] = std::move(elem[i + start]);
  }

  // Slide remaining elements down to fill the gap.
  if (num > 0) {
    for (int i = start + num; i < old_size; ++i)
      elem[i - num] = std::move(elem[i]);
    Truncate(old_size - num);
  }
}

template <typename Element>
inline void RepeatedField<Element>::Clear() {
  const bool is_soo = this->is_soo();
  Element* elem = unsafe_elements(is_soo);
  Destroy(elem, elem + size(is_soo));
  ExchangeCurrentSize(is_soo, 0);
}

template <typename Element>
inline void RepeatedField<Element>::MergeFrom(const RepeatedField& other) {
  ABSL_DCHECK_NE(&other, this);
  const bool other_is_soo = other.is_soo();
  if (auto other_size = other.size(other_is_soo)) {
    const int old_size = size();
    Reserve(old_size + other_size);
    const bool is_soo = this->is_soo();
    Element* dst =
        elements(is_soo) + ExchangeCurrentSize(is_soo, old_size + other_size);
    UninitializedCopyN(other.elements(other_is_soo), other_size, dst);
  }
}

template <typename Element>
inline void RepeatedField<Element>::CopyFrom(const RepeatedField& other) {
  if (&other == this) return;
  Clear();
  MergeFrom(other);
}

template <typename Element>
template <typename Iter>
inline void RepeatedField<Element>::Assign(Iter begin, Iter end) {
  Clear();
  Add(begin, end);
}

template <typename Element>
inline typename RepeatedField<Element>::iterator RepeatedField<Element>::erase(
    const_iterator position) ABSL_ATTRIBUTE_LIFETIME_BOUND {
  return erase(position, position + 1);
}

template <typename Element>
inline typename RepeatedField<Element>::iterator RepeatedField<Element>::erase(
    const_iterator first, const_iterator last) ABSL_ATTRIBUTE_LIFETIME_BOUND {
  size_type first_offset = first - cbegin();
  if (first != last) {
    Truncate(std::copy(last, cend(), begin() + first_offset) - cbegin());
  }
  return begin() + first_offset;
}

template <typename Element>
inline Element* RepeatedField<Element>::mutable_data()
    ABSL_ATTRIBUTE_LIFETIME_BOUND {
  return unsafe_elements(is_soo());
}

template <typename Element>
inline const Element* RepeatedField<Element>::data() const
    ABSL_ATTRIBUTE_LIFETIME_BOUND {
  return unsafe_elements(is_soo());
}

template <typename Element>
inline void RepeatedField<Element>::InternalSwap(
    RepeatedField* PROTOBUF_RESTRICT other) {
  ABSL_DCHECK(this != other);

  // We need to unpoison during the swap in case we're in SOO mode.
  UnpoisonBuffer();
  other->UnpoisonBuffer();

  internal::memswap<sizeof(internal::SooRep)>(
      reinterpret_cast<char*>(&this->soo_rep_),
      reinterpret_cast<char*>(&other->soo_rep_));

  AnnotateSize(Capacity(), size());
  other->AnnotateSize(other->Capacity(), other->size());
}

template <typename Element>
void RepeatedField<Element>::Swap(RepeatedField* other) {
  if (this == other) return;
  Arena* arena = GetArena();
  Arena* other_arena = other->GetArena();
  if (internal::CanUseInternalSwap(arena, other_arena)) {
    InternalSwap(other);
  } else {
    RepeatedField<Element> temp(other_arena);
    temp.MergeFrom(*this);
    CopyFrom(*other);
    other->UnsafeArenaSwap(&temp);
  }
}

template <typename Element>
void RepeatedField<Element>::UnsafeArenaSwap(RepeatedField* other) {
  if (this == other) return;
  ABSL_DCHECK_EQ(GetArena(), other->GetArena());
  InternalSwap(other);
}

template <typename Element>
void RepeatedField<Element>::SwapElements(int index1, int index2) {
  Element* elem = elements(is_soo());
  using std::swap;  // enable ADL with fallback
  swap(elem[index1], elem[index2]);
}

template <typename Element>
inline typename RepeatedField<Element>::iterator RepeatedField<Element>::begin()
    ABSL_ATTRIBUTE_LIFETIME_BOUND {
  return iterator(unsafe_elements(is_soo()));
}
template <typename Element>
inline typename RepeatedField<Element>::const_iterator
RepeatedField<Element>::begin() const ABSL_ATTRIBUTE_LIFETIME_BOUND {
  return const_iterator(unsafe_elements(is_soo()));
}
template <typename Element>
inline typename RepeatedField<Element>::const_iterator
RepeatedField<Element>::cbegin() const ABSL_ATTRIBUTE_LIFETIME_BOUND {
  return const_iterator(unsafe_elements(is_soo()));
}
template <typename Element>
inline typename RepeatedField<Element>::iterator RepeatedField<Element>::end()
    ABSL_ATTRIBUTE_LIFETIME_BOUND {
  const bool is_soo = this->is_soo();
  return iterator(unsafe_elements(is_soo) + size(is_soo));
}
template <typename Element>
inline typename RepeatedField<Element>::const_iterator
RepeatedField<Element>::end() const ABSL_ATTRIBUTE_LIFETIME_BOUND {
  const bool is_soo = this->is_soo();
  return const_iterator(unsafe_elements(is_soo) + size(is_soo));
}
template <typename Element>
inline typename RepeatedField<Element>::const_iterator
RepeatedField<Element>::cend() const ABSL_ATTRIBUTE_LIFETIME_BOUND {
  const bool is_soo = this->is_soo();
  return const_iterator(unsafe_elements(is_soo) + size(is_soo));
}

template <typename Element>
inline size_t RepeatedField<Element>::SpaceUsedExcludingSelfLong() const {
  const int capacity = Capacity();
  return capacity > kSooCapacityElements
             ? capacity * sizeof(Element) + kHeapRepHeaderSize
             : 0;
}

namespace internal {
// Returns the new size for a reserved field based on its 'capacity' and the
// requested 'new_size'. The result is clamped to the closed interval:
//   [internal::kMinRepeatedFieldAllocationSize,
//    std::numeric_limits<int>::max()]
// Requires: new_size > capacity
template <typename T, int kHeapRepHeaderSize>
inline int CalculateReserveSize(int capacity, int new_size) {
  constexpr int lower_limit =
      RepeatedFieldLowerClampLimit<T, kHeapRepHeaderSize>();
  if (new_size < lower_limit) {
    // Clamp to smallest allowed size.
    return lower_limit;
  }
  constexpr int kMaxSizeBeforeClamp =
      (std::numeric_limits<int>::max() - kHeapRepHeaderSize) / 2;
  if (ABSL_PREDICT_FALSE(capacity > kMaxSizeBeforeClamp)) {
    return std::numeric_limits<int>::max();
  }
  constexpr int kSooCapacityElements = SooCapacityElements(sizeof(T));
  if (kSooCapacityElements > 0 && kSooCapacityElements < lower_limit) {
    // In this case, we need to set capacity to 0 here to ensure power-of-two
    // sized allocations.
    if (capacity < lower_limit) capacity = 0;
  } else {
    ABSL_DCHECK(capacity == 0 || capacity >= lower_limit)
        << capacity << " " << lower_limit;
  }
  // We want to double the number of bytes, not the number of elements, to try
  // to stay within power-of-two allocations.
  // The allocation has kHeapRepHeaderSize + sizeof(T) * capacity.
  int doubled_size = 2 * capacity + kHeapRepHeaderSize / sizeof(T);
  return std::max(doubled_size, new_size);
}
}  // namespace internal

template <typename Element>
void RepeatedField<Element>::Reserve(int new_size) {
  const bool was_soo = is_soo();
  if (ABSL_PREDICT_FALSE(new_size > Capacity(was_soo))) {
    Grow(was_soo, size(was_soo), new_size);
  }
}

// Avoid inlining of Reserve(): new, copy, and delete[] lead to a significant
// amount of code bloat.
template <typename Element>
PROTOBUF_NOINLINE void RepeatedField<Element>::GrowNoAnnotate(bool was_soo,
                                                              int old_size,
                                                              int new_size) {
  const int old_capacity = Capacity(was_soo);
  ABSL_DCHECK_GT(new_size, old_capacity);
  HeapRep* new_rep;
  Arena* arena = GetArena();

  new_size = internal::CalculateReserveSize<Element, kHeapRepHeaderSize>(
      old_capacity, new_size);

  ABSL_DCHECK_LE(static_cast<size_t>(new_size),
                 (std::numeric_limits<size_t>::max() - kHeapRepHeaderSize) /
                     sizeof(Element))
      << "Requested size is too large to fit into size_t.";
  size_t bytes =
      kHeapRepHeaderSize + sizeof(Element) * static_cast<size_t>(new_size);
  if (arena == nullptr) {
    ABSL_DCHECK_LE((bytes - kHeapRepHeaderSize) / sizeof(Element),
                   static_cast<size_t>(std::numeric_limits<int>::max()))
        << "Requested size is too large to fit element count into int.";
    internal::SizedPtr res = internal::AllocateAtLeast(bytes);
    size_t num_available =
        std::min((res.n - kHeapRepHeaderSize) / sizeof(Element),
                 static_cast<size_t>(std::numeric_limits<int>::max()));
    new_size = static_cast<int>(num_available);
    new_rep = static_cast<HeapRep*>(res.p);
  } else {
    new_rep =
        reinterpret_cast<HeapRep*>(Arena::CreateArray<char>(arena, bytes));
  }
  new_rep->arena = arena;

  if (old_size > 0) {
    Element* pnew = static_cast<Element*>(new_rep->elements());
    Element* pold = elements(was_soo);
    // TODO: add absl::is_trivially_relocatable<Element>
    if (std::is_trivial<Element>::value) {
      memcpy(static_cast<void*>(pnew), pold, old_size * sizeof(Element));
    } else {
      for (Element* end = pnew + old_size; pnew != end; ++pnew, ++pold) {
        ::new (static_cast<void*>(pnew)) Element(std::move(*pold));
        pold->~Element();
      }
    }
  }
  if (!was_soo) InternalDeallocate();

  soo_rep_.set_non_soo(was_soo, new_size, new_rep->elements());
}

// Ideally we would be able to use:
//   template <bool annotate_size = true>
//   void Grow();
// However, as explained in b/266411038#comment9, this causes issues
// in shared libraries for Youtube (and possibly elsewhere).
template <typename Element>
PROTOBUF_NOINLINE void RepeatedField<Element>::Grow(bool was_soo, int old_size,
                                                    int new_size) {
  UnpoisonBuffer();
  GrowNoAnnotate(was_soo, old_size, new_size);
  AnnotateSize(Capacity(), old_size);
}

template <typename Element>
inline void RepeatedField<Element>::Truncate(int new_size) {
  const bool is_soo = this->is_soo();
  const int old_size = size(is_soo);
  ABSL_DCHECK_LE(new_size, old_size);
  if (new_size < old_size) {
    Element* elem = unsafe_elements(is_soo);
    Destroy(elem + new_size, elem + old_size);
    ExchangeCurrentSize(is_soo, new_size);
  }
}

template <>
PROTOBUF_EXPORT size_t
RepeatedField<absl::Cord>::SpaceUsedExcludingSelfLong() const;


// -------------------------------------------------------------------

// Iterators and helper functions that follow the spirit of the STL
// std::back_insert_iterator and std::back_inserter but are tailor-made
// for RepeatedField and RepeatedPtrField. Typical usage would be:
//
//   std::copy(some_sequence.begin(), some_sequence.end(),
//             RepeatedFieldBackInserter(proto.mutable_sequence()));
//
// Ported by johannes from util/gtl/proto-array-iterators.h

namespace internal {

// STL-like iterator implementation for RepeatedField.  You should not
// refer to this class directly; use RepeatedField<T>::iterator instead.
//
// Note: All of the iterator operators *must* be inlined to avoid performance
// regressions.  This is caused by the extern template declarations below (which
// are required because of the RepeatedField extern template declarations).  If
// any of these functions aren't explicitly inlined (e.g. defined in the class),
// the compiler isn't allowed to inline them.
template <typename Element>
class RepeatedIterator {
 private:
  using traits =
      std::iterator_traits<typename std::remove_const<Element>::type*>;

 public:
  // Note: value_type is never cv-qualified.
  using value_type = typename traits::value_type;
  using difference_type = typename traits::difference_type;
  using pointer = Element*;
  using reference = Element&;
  using iterator_category = typename traits::iterator_category;
  using iterator_concept = typename IteratorConceptSupport<traits>::tag;

  constexpr RepeatedIterator() noexcept : it_(nullptr) {}

  // Allows "upcasting" from RepeatedIterator<T**> to
  // RepeatedIterator<const T*const*>.
  template <typename OtherElement,
            typename std::enable_if<std::is_convertible<
                OtherElement*, pointer>::value>::type* = nullptr>
  constexpr RepeatedIterator(
      const RepeatedIterator<OtherElement>& other) noexcept
      : it_(other.it_) {}

  // dereferenceable
  constexpr reference operator*() const noexcept { return *it_; }
  constexpr pointer operator->() const noexcept { return it_; }

 private:
  // Helper alias to hide the internal type.
  using iterator = RepeatedIterator<Element>;

 public:
  // {inc,dec}rementable
  iterator& operator++() noexcept {
    ++it_;
    return *this;
  }
  iterator operator++(int) noexcept { return iterator(it_++); }
  iterator& operator--() noexcept {
    --it_;
    return *this;
  }
  iterator operator--(int) noexcept { return iterator(it_--); }

  // equality_comparable
  friend constexpr bool operator==(const iterator& x,
                                   const iterator& y) noexcept {
    return x.it_ == y.it_;
  }
  friend constexpr bool operator!=(const iterator& x,
                                   const iterator& y) noexcept {
    return x.it_ != y.it_;
  }

  // less_than_comparable
  friend constexpr bool operator<(const iterator& x,
                                  const iterator& y) noexcept {
    return x.it_ < y.it_;
  }
  friend constexpr bool operator<=(const iterator& x,
                                   const iterator& y) noexcept {
    return x.it_ <= y.it_;
  }
  friend constexpr bool operator>(const iterator& x,
                                  const iterator& y) noexcept {
    return x.it_ > y.it_;
  }
  friend constexpr bool operator>=(const iterator& x,
                                   const iterator& y) noexcept {
    return x.it_ >= y.it_;
  }

  // addable, subtractable
  iterator& operator+=(difference_type d) noexcept {
    it_ += d;
    return *this;
  }
  constexpr iterator operator+(difference_type d) const noexcept {
    return iterator(it_ + d);
  }
  friend constexpr iterator operator+(const difference_type d,
                                      iterator it) noexcept {
    return it + d;
  }

  iterator& operator-=(difference_type d) noexcept {
    it_ -= d;
    return *this;
  }
  iterator constexpr operator-(difference_type d) const noexcept {
    return iterator(it_ - d);
  }

  // indexable
  constexpr reference operator[](difference_type d) const noexcept {
    return it_[d];
  }

  // random access iterator
  friend constexpr difference_type operator-(iterator it1,
                                             iterator it2) noexcept {
    return it1.it_ - it2.it_;
  }

 private:
  template <typename OtherElement>
  friend class RepeatedIterator;

  // Allow construction from RepeatedField.
  friend class RepeatedField<value_type>;
  explicit RepeatedIterator(pointer it) noexcept : it_(it) {}

  // The internal iterator.
  pointer it_;
};

// A back inserter for RepeatedField objects.
template <typename T>
class RepeatedFieldBackInsertIterator {
 public:
  using iterator_category = std::output_iterator_tag;
  using value_type = T;
  using pointer = void;
  using reference = void;
  using difference_type = std::ptrdiff_t;

  explicit RepeatedFieldBackInsertIterator(
      RepeatedField<T>* const mutable_field)
      : field_(mutable_field) {}
  RepeatedFieldBackInsertIterator<T>& operator=(const T& value) {
    field_->Add(value);
    return *this;
  }
  RepeatedFieldBackInsertIterator<T>& operator*() { return *this; }
  RepeatedFieldBackInsertIterator<T>& operator++() { return *this; }
  RepeatedFieldBackInsertIterator<T>& operator++(int /* unused */) {
    return *this;
  }

 private:
  RepeatedField<T>* field_;
};

}  // namespace internal

// Provides a back insert iterator for RepeatedField instances,
// similar to std::back_inserter().
template <typename T>
internal::RepeatedFieldBackInsertIterator<T> RepeatedFieldBackInserter(
    RepeatedField<T>* const mutable_field) {
  return internal::RepeatedFieldBackInsertIterator<T>(mutable_field);
}


}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_REPEATED_FIELD_H__
