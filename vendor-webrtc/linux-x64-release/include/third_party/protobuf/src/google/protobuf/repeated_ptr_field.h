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
// This header covers RepeatedPtrField.

#ifndef GOOGLE_PROTOBUF_REPEATED_PTR_FIELD_H__
#define GOOGLE_PROTOBUF_REPEATED_PTR_FIELD_H__

#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <iterator>
#include <limits>
#include <new>
#include <string>
#include <type_traits>
#include <utility>

#include "absl/base/attributes.h"
#include "absl/base/optimization.h"
#include "absl/base/prefetch.h"
#include "absl/log/absl_check.h"
#include "absl/meta/type_traits.h"
#include "google/protobuf/arena.h"
#include "google/protobuf/internal_visibility.h"
#include "google/protobuf/message_lite.h"
#include "google/protobuf/port.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

#ifdef SWIG
#error "You cannot SWIG proto headers"
#endif

namespace google {
namespace protobuf {

class Message;
class Reflection;

template <typename T>
struct WeakRepeatedPtrField;

namespace internal {

class MergePartialFromCodedStreamHelper;
class SwapFieldHelper;

}  // namespace internal

namespace internal {
template <typename It>
class RepeatedPtrIterator;
template <typename It, typename VoidPtr>
class RepeatedPtrOverPtrsIterator;
}  // namespace internal

namespace internal {

// Swaps two non-overlapping blocks of memory of size `N`
template <size_t N>
inline void memswap(char* PROTOBUF_RESTRICT a, char* PROTOBUF_RESTRICT b) {
  // `PROTOBUF_RESTRICT` tells compiler that blocks do not overlapping which
  // allows it to generate optimized code for swap_ranges.
  std::swap_ranges(a, a + N, b);
}

// A trait that tells offset of `T::arena_`.
//
// Do not use this struct - it exists for internal use only.
template <typename T>
struct ArenaOffsetHelper {
  static constexpr size_t value = offsetof(T, arena_);
};

// Copies the object in the arena.
// Used in the slow path. Out-of-line for lower binary size cost.
PROTOBUF_EXPORT MessageLite* CloneSlow(Arena* arena, const MessageLite& value);
PROTOBUF_EXPORT std::string* CloneSlow(Arena* arena, const std::string& value);

// Defined further below.
template <typename Type>
class GenericTypeHandler;

// This is the common base class for RepeatedPtrFields.  It deals only in void*
// pointers.  Users should not use this interface directly.
//
// The methods of this interface correspond to the methods of RepeatedPtrField,
// but may have a template argument called TypeHandler.  Its signature is:
//   class TypeHandler {
//    public:
//     using Type = MyType;
//
//     static Type*(*)(Arena*) GetNewFunc();
//     static void GetArena(Type* value);
//
//     static Type* New(Arena* arena, Type&& value);
//     static void Delete(Type*, Arena* arena);
//     static void Clear(Type*);
//
//     // Only needs to be implemented if SpaceUsedExcludingSelf() is called.
//     static int SpaceUsedLong(const Type&);
//
//     static const Type& default_instance();
//   };
class PROTOBUF_EXPORT RepeatedPtrFieldBase {
  template <typename TypeHandler>
  using Value = typename TypeHandler::Type;

  static constexpr int kSSOCapacity = 1;

  using ElementFactory = void* (*)(Arena*);

 protected:
  // We use the same TypeHandler for all Message types to deduplicate generated
  // code.
  template <typename TypeHandler>
  using CommonHandler = typename std::conditional<
      std::is_base_of<MessageLite, Value<TypeHandler>>::value,
      GenericTypeHandler<MessageLite>, TypeHandler>::type;

  constexpr RepeatedPtrFieldBase()
      : tagged_rep_or_elem_(nullptr),
        current_size_(0),
        capacity_proxy_(0),
        arena_(nullptr) {}
  explicit RepeatedPtrFieldBase(Arena* arena)
      : tagged_rep_or_elem_(nullptr),
        current_size_(0),
        capacity_proxy_(0),
        arena_(arena) {}

  RepeatedPtrFieldBase(const RepeatedPtrFieldBase&) = delete;
  RepeatedPtrFieldBase& operator=(const RepeatedPtrFieldBase&) = delete;

  ~RepeatedPtrFieldBase() {
#ifndef NDEBUG
    // Try to trigger segfault / asan failure in non-opt builds if arena_
    // lifetime has ended before the destructor.
    if (arena_) (void)arena_->SpaceAllocated();
#endif
  }

  bool empty() const { return current_size_ == 0; }
  int size() const { return current_size_; }
  // Returns the size of the buffer with pointers to elements.
  //
  // Note:
  //
  //   * prefer `SizeAtCapacity()` to `size() == Capacity()`;
  //   * prefer `AllocatedSizeAtCapacity()` to `allocated_size() == Capacity()`.
  int Capacity() const { return capacity_proxy_ + kSSOCapacity; }

  template <typename TypeHandler>
  const Value<TypeHandler>& at(int index) const {
    ABSL_CHECK_GE(index, 0);
    ABSL_CHECK_LT(index, current_size_);
    return *cast<TypeHandler>(element_at(index));
  }

  template <typename TypeHandler>
  Value<TypeHandler>& at(int index) {
    ABSL_CHECK_GE(index, 0);
    ABSL_CHECK_LT(index, current_size_);
    return *cast<TypeHandler>(element_at(index));
  }

  template <typename TypeHandler>
  Value<TypeHandler>* Mutable(int index) {
    ABSL_DCHECK_GE(index, 0);
    ABSL_DCHECK_LT(index, current_size_);
    return cast<TypeHandler>(element_at(index));
  }

  template <typename TypeHandler>
  Value<TypeHandler>* Add() {
    if (std::is_same<Value<TypeHandler>, std::string>{}) {
      return cast<TypeHandler>(AddString());
    }
    return cast<TypeHandler>(AddMessageLite(TypeHandler::GetNewFunc()));
  }

  template <typename TypeHandler>
  inline void Add(Value<TypeHandler>&& value) {
    static_assert(std::is_move_constructible<Value<TypeHandler>>::value, "");
    static_assert(std::is_move_assignable<Value<TypeHandler>>::value, "");
    if (current_size_ < allocated_size()) {
      *cast<TypeHandler>(element_at(ExchangeCurrentSize(current_size_ + 1))) =
          std::move(value);
      return;
    }
    MaybeExtend();
    if (!using_sso()) ++rep()->allocated_size;
    auto* result = TypeHandler::New(arena_, std::move(value));
    element_at(ExchangeCurrentSize(current_size_ + 1)) = result;
  }

  // Must be called from destructor.
  //
  // Pre-condition: NeedsDestroy() returns true.
  template <typename TypeHandler>
  void Destroy() {
    ABSL_DCHECK(NeedsDestroy());

    // TODO: arena check is redundant once all `RepeatedPtrField`s
    // with non-null arena are owned by the arena.
    if (ABSL_PREDICT_FALSE(arena_ != nullptr)) return;

    using H = CommonHandler<TypeHandler>;
    int n = allocated_size();
    void** elems = elements();
    for (int i = 0; i < n; i++) {
      Delete<H>(elems[i], nullptr);
    }
    if (!using_sso()) {
      internal::SizedDelete(rep(),
                            Capacity() * sizeof(elems[0]) + kRepHeaderSize);
    }
  }

  inline bool NeedsDestroy() const {
    // Either there is an allocated element in SSO buffer or there is an
    // allocated Rep.
    return tagged_rep_or_elem_ != nullptr;
  }

  // Pre-condition: NeedsDestroy() returns true.
  void DestroyProtos();

 public:
  // The next few methods are public so that they can be called from generated
  // code when implicit weak fields are used, but they should never be called by
  // application code.

  template <typename TypeHandler>
  const Value<TypeHandler>& Get(int index) const {
    ABSL_DCHECK_GE(index, 0);
    ABSL_DCHECK_LT(index, current_size_);
    return *cast<TypeHandler>(element_at(index));
  }

  // Creates and adds an element using the given prototype, without introducing
  // a link-time dependency on the concrete message type.
  //
  // Pre-condition: prototype must not be nullptr.
  MessageLite* AddMessage(const MessageLite* prototype);

  template <typename TypeHandler>
  void Clear() {
    const int n = current_size_;
    ABSL_DCHECK_GE(n, 0);
    if (n > 0) {
      using H = CommonHandler<TypeHandler>;
      ClearNonEmpty<H>();
    }
  }

  // Appends all message values from `from` to this instance.
  template <typename T>
  void MergeFrom(const RepeatedPtrFieldBase& from) {
    static_assert(std::is_base_of<MessageLite, T>::value, "");
#ifdef __cpp_if_constexpr
    if constexpr (!std::is_base_of<Message, T>::value) {
      // For LITE objects we use the generic MergeFrom to save on binary size.
      return MergeFrom<MessageLite>(from);
    }
#endif
    MergeFromConcreteMessage(from, Arena::CopyConstruct<T>);
  }

  inline void InternalSwap(RepeatedPtrFieldBase* PROTOBUF_RESTRICT rhs) {
    ABSL_DCHECK(this != rhs);

    // Swap all fields except arena pointer at once.
    internal::memswap<ArenaOffsetHelper<RepeatedPtrFieldBase>::value>(
        reinterpret_cast<char*>(this), reinterpret_cast<char*>(rhs));
  }

  // Returns true if there are no preallocated elements in the array.
  bool PrepareForParse() { return allocated_size() == current_size_; }

  // Similar to `AddAllocated` but faster.
  //
  // Pre-condition: PrepareForParse() is true.
  void AddAllocatedForParse(void* value) {
    ABSL_DCHECK(PrepareForParse());
    if (ABSL_PREDICT_FALSE(SizeAtCapacity())) {
      *InternalExtend(1) = value;
      ++rep()->allocated_size;
    } else {
      if (using_sso()) {
        tagged_rep_or_elem_ = value;
      } else {
        rep()->elements[current_size_] = value;
        ++rep()->allocated_size;
      }
    }
    ExchangeCurrentSize(current_size_ + 1);
  }

 protected:
  template <typename TypeHandler>
  void RemoveLast() {
    ABSL_DCHECK_GT(current_size_, 0);
    ExchangeCurrentSize(current_size_ - 1);
    using H = CommonHandler<TypeHandler>;
    H::Clear(cast<H>(element_at(current_size_)));
  }

  template <typename TypeHandler>
  void CopyFrom(const RepeatedPtrFieldBase& other) {
    if (&other == this) return;
    Clear<TypeHandler>();
    if (other.empty()) return;
    MergeFrom<typename TypeHandler::Type>(other);
  }

  void CloseGap(int start, int num);

  void Reserve(int capacity);

  template <typename TypeHandler>
  static inline Value<TypeHandler>* copy(const Value<TypeHandler>* value) {
    return cast<TypeHandler>(CloneSlow(nullptr, *value));
  }

  // Used for constructing iterators.
  void* const* raw_data() const { return elements(); }
  void** raw_mutable_data() { return elements(); }

  template <typename TypeHandler>
  Value<TypeHandler>** mutable_data() {
    // TODO:  Breaks C++ aliasing rules.  We should probably remove this
    //   method entirely.
    return reinterpret_cast<Value<TypeHandler>**>(raw_mutable_data());
  }

  template <typename TypeHandler>
  const Value<TypeHandler>* const* data() const {
    // TODO:  Breaks C++ aliasing rules.  We should probably remove this
    //   method entirely.
    return reinterpret_cast<const Value<TypeHandler>* const*>(raw_data());
  }

  template <typename TypeHandler>
  PROTOBUF_NDEBUG_INLINE void Swap(RepeatedPtrFieldBase* other) {
    if (internal::CanUseInternalSwap(GetArena(), other->GetArena())) {
      InternalSwap(other);
    } else {
      SwapFallback<TypeHandler>(other);
    }
  }

  void SwapElements(int index1, int index2) {
    using std::swap;  // enable ADL with fallback
    swap(element_at(index1), element_at(index2));
  }

  template <typename TypeHandler>
  PROTOBUF_NOINLINE size_t SpaceUsedExcludingSelfLong() const {
    size_t allocated_bytes =
        using_sso()
            ? 0
            : static_cast<size_t>(Capacity()) * sizeof(void*) + kRepHeaderSize;
    const int n = allocated_size();
    void* const* elems = elements();
    for (int i = 0; i < n; ++i) {
      allocated_bytes +=
          TypeHandler::SpaceUsedLong(*cast<TypeHandler>(elems[i]));
    }
    return allocated_bytes;
  }

  // Advanced memory management --------------------------------------

  // Like Add(), but if there are no cleared objects to use, returns nullptr.
  template <typename TypeHandler>
  Value<TypeHandler>* AddFromCleared() {
    if (current_size_ < allocated_size()) {
      return cast<TypeHandler>(
          element_at(ExchangeCurrentSize(current_size_ + 1)));
    } else {
      return nullptr;
    }
  }

  template <typename TypeHandler>
  void AddAllocated(Value<TypeHandler>* value) {
    ABSL_DCHECK_NE(value, nullptr);
    Arena* element_arena = TypeHandler::GetArena(value);
    Arena* arena = GetArena();
    if (arena != element_arena || AllocatedSizeAtCapacity()) {
      AddAllocatedSlowWithCopy<TypeHandler>(value, element_arena, arena);
      return;
    }
    // Fast path: underlying arena representation (tagged pointer) is equal to
    // our arena pointer, and we can add to array without resizing it (at
    // least one slot that is not allocated).
    void** elems = elements();
    if (current_size_ < allocated_size()) {
      // Make space at [current] by moving first allocated element to end of
      // allocated list.
      elems[allocated_size()] = elems[current_size_];
    }
    elems[ExchangeCurrentSize(current_size_ + 1)] = value;
    if (!using_sso()) ++rep()->allocated_size;
  }

  template <typename TypeHandler>
  void UnsafeArenaAddAllocated(Value<TypeHandler>* value) {
    ABSL_DCHECK_NE(value, nullptr);
    // Make room for the new pointer.
    if (SizeAtCapacity()) {
      // The array is completely full with no cleared objects, so grow it.
      InternalExtend(1);
      ++rep()->allocated_size;
    } else if (AllocatedSizeAtCapacity()) {
      // There is no more space in the pointer array because it contains some
      // cleared objects awaiting reuse.  We don't want to grow the array in
      // this case because otherwise a loop calling AddAllocated() followed by
      // Clear() would leak memory.
      using H = CommonHandler<TypeHandler>;
      Delete<H>(element_at(current_size_), arena_);
    } else if (current_size_ < allocated_size()) {
      // We have some cleared objects.  We don't care about their order, so we
      // can just move the first one to the end to make space.
      element_at(allocated_size()) = element_at(current_size_);
      ++rep()->allocated_size;
    } else {
      // There are no cleared objects.
      if (!using_sso()) ++rep()->allocated_size;
    }

    element_at(ExchangeCurrentSize(current_size_ + 1)) = value;
  }

  template <typename TypeHandler>
  [[nodiscard]] Value<TypeHandler>* ReleaseLast() {
    Value<TypeHandler>* result = UnsafeArenaReleaseLast<TypeHandler>();
    // Now perform a copy if we're on an arena.
    Arena* arena = GetArena();

    if (internal::DebugHardenForceCopyInRelease()) {
      auto* new_result = copy<TypeHandler>(result);
      if (arena == nullptr) delete result;
      return new_result;
    } else {
      return (arena == nullptr) ? result : copy<TypeHandler>(result);
    }
  }

  // Releases and returns the last element, but does not do out-of-arena copy.
  // Instead, just returns the raw pointer to the contained element in the
  // arena.
  template <typename TypeHandler>
  Value<TypeHandler>* UnsafeArenaReleaseLast() {
    ABSL_DCHECK_GT(current_size_, 0);
    ExchangeCurrentSize(current_size_ - 1);
    auto* result = cast<TypeHandler>(element_at(current_size_));
    if (using_sso()) {
      tagged_rep_or_elem_ = nullptr;
    } else {
      --rep()->allocated_size;
      if (current_size_ < allocated_size()) {
        // There are cleared elements on the end; replace the removed element
        // with the last allocated element.
        element_at(current_size_) = element_at(allocated_size());
      }
    }
    return result;
  }

  int ClearedCount() const { return allocated_size() - current_size_; }

  // Slowpath handles all cases, copying if necessary.
  template <typename TypeHandler>
  PROTOBUF_NOINLINE void AddAllocatedSlowWithCopy(
      // Pass value_arena and my_arena to avoid duplicate virtual call (value)
      // or load (mine).
      Value<TypeHandler>* value, Arena* value_arena, Arena* my_arena) {
    using H = CommonHandler<TypeHandler>;
    // Ensure that either the value is in the same arena, or if not, we do the
    // appropriate thing: Own() it (if it's on heap and we're in an arena) or
    // copy it to our arena/heap (otherwise).
    if (my_arena != nullptr && value_arena == nullptr) {
      my_arena->Own(value);
    } else if (my_arena != value_arena) {
      ABSL_DCHECK(value_arena != nullptr);
      value = cast<TypeHandler>(CloneSlow(my_arena, *value));
    }

    UnsafeArenaAddAllocated<H>(value);
  }

  template <typename TypeHandler>
  PROTOBUF_NOINLINE void SwapFallback(RepeatedPtrFieldBase* other) {
    ABSL_DCHECK(!internal::CanUseInternalSwap(GetArena(), other->GetArena()));

    // Copy semantics in this case. We try to improve efficiency by placing the
    // temporary on |other|'s arena so that messages are copied twice rather
    // than three times.
    RepeatedPtrFieldBase temp(other->GetArena());
    if (!this->empty()) {
      temp.MergeFrom<typename TypeHandler::Type>(*this);
    }
    this->CopyFrom<TypeHandler>(*other);
    other->InternalSwap(&temp);
    if (temp.NeedsDestroy()) {
      temp.Destroy<TypeHandler>();
    }
  }

  // Gets the Arena on which this RepeatedPtrField stores its elements.
  inline Arena* GetArena() const { return arena_; }

  static constexpr size_t InternalGetArenaOffset(internal::InternalVisibility) {
    return PROTOBUF_FIELD_OFFSET(RepeatedPtrFieldBase, arena_);
  }

 private:
  using InternalArenaConstructable_ = void;
  using DestructorSkippable_ = void;

  template <typename T>
  friend class Arena::InternalHelper;

  // ExtensionSet stores repeated message extensions as
  // RepeatedPtrField<MessageLite>, but non-lite ExtensionSets need to implement
  // SpaceUsedLong(), and thus need to call SpaceUsedExcludingSelfLong()
  // reinterpreting MessageLite as Message.  ExtensionSet also needs to make use
  // of AddFromCleared(), which is not part of the public interface.
  friend class ExtensionSet;

  // The MapFieldBase implementation needs to call protected methods directly,
  // reinterpreting pointers as being to Message instead of a specific Message
  // subclass.
  friend class MapFieldBase;
  friend struct MapFieldTestPeer;

  // The table-driven MergePartialFromCodedStream implementation needs to
  // operate on RepeatedPtrField<MessageLite>.
  friend class MergePartialFromCodedStreamHelper;

  friend class AccessorHelper;

  template <typename T>
  friend struct google::protobuf::WeakRepeatedPtrField;

  friend class internal::TcParser;  // TODO: Remove this friend.

  // Expose offset of `arena_` without exposing the member itself.
  // Used to optimize code size of `InternalSwap` method.
  template <typename T>
  friend struct ArenaOffsetHelper;

  // The reflection implementation needs to call protected methods directly,
  // reinterpreting pointers as being to Message instead of a specific Message
  // subclass.
  friend class google::protobuf::Reflection;
  friend class internal::SwapFieldHelper;

  friend class RustRepeatedMessageHelper;

  // Concrete Arena enabled copy function used to copy messages instances.
  // This follows the `Arena::CopyConstruct` signature so that the compiler
  // can have the inlined call into the out of line copy function(s) simply pass
  // the address of `Arena::CopyConstruct` 'as is'.
  using CopyFn = void* (*)(Arena*, const void*);

  struct Rep {
    int allocated_size;
    // Here we declare a huge array as a way of approximating C's "flexible
    // array member" feature without relying on undefined behavior.
    void* elements[(std::numeric_limits<int>::max() - 2 * sizeof(int)) /
                   sizeof(void*)];
  };

  static constexpr size_t kRepHeaderSize = offsetof(Rep, elements);

  // Replaces current_size_ with new_size and returns the previous value of
  // current_size_. This function is intended to be the only place where
  // current_size_ is modified.
  inline int ExchangeCurrentSize(int new_size) {
    return std::exchange(current_size_, new_size);
  }
  inline bool SizeAtCapacity() const {
    // Harden invariant size() <= allocated_size() <= Capacity().
    ABSL_DCHECK_LE(size(), allocated_size());
    ABSL_DCHECK_LE(allocated_size(), Capacity());
    // This is equivalent to `current_size_ == Capacity()`.
    // Assuming `Capacity()` function is inlined, compiler is likely to optimize
    // away "+ kSSOCapacity" and reduce it to "current_size_ > capacity_proxy_"
    // which is an instruction less than "current_size_ == capacity_proxy_ + 1".
    return current_size_ >= Capacity();
  }
  inline bool AllocatedSizeAtCapacity() const {
    // Harden invariant size() <= allocated_size() <= Capacity().
    ABSL_DCHECK_LE(size(), allocated_size());
    ABSL_DCHECK_LE(allocated_size(), Capacity());
    // This combines optimization mentioned in `SizeAtCapacity()` and simplifies
    // `allocated_size()` in sso case.
    return using_sso() ? (tagged_rep_or_elem_ != nullptr)
                       : rep()->allocated_size >= Capacity();
  }

  void* const* elements() const {
    return using_sso() ? &tagged_rep_or_elem_ : +rep()->elements;
  }
  void** elements() {
    return using_sso() ? &tagged_rep_or_elem_ : +rep()->elements;
  }

  void*& element_at(int index) {
    if (using_sso()) {
      ABSL_DCHECK_EQ(index, 0);
      return tagged_rep_or_elem_;
    }
    return rep()->elements[index];
  }
  const void* element_at(int index) const {
    return const_cast<RepeatedPtrFieldBase*>(this)->element_at(index);
  }

  int allocated_size() const {
    return using_sso() ? (tagged_rep_or_elem_ != nullptr ? 1 : 0)
                       : rep()->allocated_size;
  }
  Rep* rep() {
    ABSL_DCHECK(!using_sso());
    return reinterpret_cast<Rep*>(
        reinterpret_cast<uintptr_t>(tagged_rep_or_elem_) - 1);
  }
  const Rep* rep() const {
    return const_cast<RepeatedPtrFieldBase*>(this)->rep();
  }

  bool using_sso() const {
    return (reinterpret_cast<uintptr_t>(tagged_rep_or_elem_) & 1) == 0;
  }

  template <typename TypeHandler>
  static inline Value<TypeHandler>* cast(void* element) {
    return reinterpret_cast<Value<TypeHandler>*>(element);
  }
  template <typename TypeHandler>
  static inline const Value<TypeHandler>* cast(const void* element) {
    return reinterpret_cast<const Value<TypeHandler>*>(element);
  }
  template <typename TypeHandler>
  static inline void Delete(void* obj, Arena* arena) {
    TypeHandler::Delete(cast<TypeHandler>(obj), arena);
  }

  // Out-of-line helper routine for Clear() once the inlined check has
  // determined the container is non-empty
  template <typename TypeHandler>
  PROTOBUF_NOINLINE void ClearNonEmpty() {
    const int n = current_size_;
    void* const* elems = elements();
    int i = 0;
    ABSL_DCHECK_GT(n, 0);
    // do/while loop to avoid initial test because we know n > 0
    do {
      TypeHandler::Clear(cast<TypeHandler>(elems[i++]));
    } while (i < n);
    ExchangeCurrentSize(0);
  }

  // Merges messages from `from` into available, cleared messages sitting in the
  // range `[size(), allocated_size())`. Returns the number of message merged
  // which is `ClearedCount(), from.size())`.
  // Note that this function does explicitly NOT update `current_size_`.
  // This function is out of line as it should be the slow path: this scenario
  // only happens when a caller constructs and fills a repeated field, then
  // shrinks it, and then merges additional messages into it.
  int MergeIntoClearedMessages(const RepeatedPtrFieldBase& from);

  // Appends all messages from `from` to this instance, using the
  // provided `copy_fn` copy function to copy existing messages.
  void MergeFromConcreteMessage(const RepeatedPtrFieldBase& from,
                                CopyFn copy_fn);

  // Extends capacity by at least |extend_amount|. Returns a pointer to the
  // next available element slot.
  //
  // Pre-condition: |extend_amount| must be > 0.
  void** InternalExtend(int extend_amount);

  // Ensures that capacity is big enough to store one more allocated element.
  inline void MaybeExtend() {
    if (AllocatedSizeAtCapacity()) {
      ABSL_DCHECK_EQ(allocated_size(), Capacity());
      InternalExtend(1);
    } else {
      ABSL_DCHECK_NE(allocated_size(), Capacity());
    }
  }

  // Ensures that capacity is at least `n` elements.
  // Returns a pointer to the element directly beyond the last element.
  inline void** InternalReserve(int n) {
    if (n <= Capacity()) {
      void** elements = using_sso() ? &tagged_rep_or_elem_ : rep()->elements;
      return elements + current_size_;
    }
    return InternalExtend(n - Capacity());
  }

  // Internal helpers for Add that keep definition out-of-line.
  void* AddMessageLite(ElementFactory factory);
  void* AddString();

  // Common implementation used by various Add* methods. `factory` is an object
  // used to construct a new element unless there are spare cleared elements
  // ready for reuse. Returns pointer to the new element.
  //
  // Note: avoid inlining this function in methods such as `Add()` as this would
  // drastically increase binary size due to template instantiation and implicit
  // inlining.
  template <typename Factory>
  void* AddInternal(Factory factory);

  // A few notes on internal representation:
  //
  // We use an indirected approach, with struct Rep, to keep
  // sizeof(RepeatedPtrFieldBase) equivalent to what it was before arena support
  // was added; namely, 3 8-byte machine words on x86-64. An instance of Rep is
  // allocated only when the repeated field is non-empty, and it is a
  // dynamically-sized struct (the header is directly followed by elements[]).
  // We place arena_ and current_size_ directly in the object to avoid cache
  // misses due to the indirection, because these fields are checked frequently.
  // Placing all fields directly in the RepeatedPtrFieldBase instance would cost
  // significant performance for memory-sensitive workloads.
  void* tagged_rep_or_elem_;
  int current_size_;
  int capacity_proxy_;  // we store `capacity - kSSOCapacity` as an optimization
  Arena* arena_;
};

// Appends all message values from `from` to this instance using the abstract
// message interface. This overload is used in places like reflection and
// other locations where the underlying type is unavailable
template <>
void RepeatedPtrFieldBase::MergeFrom<MessageLite>(
    const RepeatedPtrFieldBase& from);

template <>
inline void RepeatedPtrFieldBase::MergeFrom<Message>(
    const RepeatedPtrFieldBase& from) {
  return MergeFrom<MessageLite>(from);
}

// Appends all `std::string` values from `from` to this instance.
template <>
void RepeatedPtrFieldBase::MergeFrom<std::string>(
    const RepeatedPtrFieldBase& from);


template <typename Factory>
void* RepeatedPtrFieldBase::AddInternal(Factory factory) {
  Arena* const arena = GetArena();
  if (tagged_rep_or_elem_ == nullptr) {
    ExchangeCurrentSize(1);
    tagged_rep_or_elem_ = factory(arena);
    return tagged_rep_or_elem_;
  }
  absl::PrefetchToLocalCache(tagged_rep_or_elem_);
  if (using_sso()) {
    if (current_size_ == 0) {
      ExchangeCurrentSize(1);
      return tagged_rep_or_elem_;
    }
    void*& result = *InternalExtend(1);
    result = factory(arena);
    Rep* r = rep();
    r->allocated_size = 2;
    ExchangeCurrentSize(2);
    return result;
  }
  Rep* r = rep();
  if (ABSL_PREDICT_FALSE(SizeAtCapacity())) {
    InternalExtend(1);
    r = rep();
  } else {
    if (current_size_ != r->allocated_size) {
      return r->elements[ExchangeCurrentSize(current_size_ + 1)];
    }
  }
  ++r->allocated_size;
  void*& result = r->elements[ExchangeCurrentSize(current_size_ + 1)];
  result = factory(arena);
  return result;
}

PROTOBUF_EXPORT void InternalOutOfLineDeleteMessageLite(MessageLite* message);

template <typename GenericType>
class GenericTypeHandler {
 public:
  using Type = GenericType;

  static constexpr auto GetNewFunc() { return Arena::DefaultConstruct<Type>; }
  static inline Arena* GetArena(Type* value) {
    return Arena::InternalGetArena(value);
  }

  static inline Type* New(Arena* arena, Type&& value) {
    return Arena::Create<Type>(arena, std::move(value));
  }
  static inline void Delete(Type* value, Arena* arena) {
    if (arena != nullptr) return;
#ifdef __cpp_if_constexpr
    if constexpr (std::is_base_of<MessageLite, Type>::value) {
      // Using virtual destructor to reduce generated code size that would have
      // happened otherwise due to inlined `~Type()`.
      InternalOutOfLineDeleteMessageLite(value);
    } else {
      delete value;
    }
#else
    delete value;
#endif
  }
  static inline void Clear(Type* value) { value->Clear(); }
  static inline size_t SpaceUsedLong(const Type& value) {
    return value.SpaceUsedLong();
  }

  static const Type& default_instance() {
    return *static_cast<const GenericType*>(
        MessageTraits<Type>::default_instance());
  }
};

template <>
inline Arena* GenericTypeHandler<MessageLite>::GetArena(MessageLite* value) {
  return value->GetArena();
}

// Message specialization bodies defined in message.cc. This split is necessary
// to allow proto2-lite (which includes this header) to be independent of
// Message.
template <>
PROTOBUF_EXPORT Arena* GenericTypeHandler<Message>::GetArena(Message* value);

PROTOBUF_EXPORT void* NewStringElement(Arena* arena);

template <>
class GenericTypeHandler<std::string> {
 public:
  using Type = std::string;

  static constexpr auto GetNewFunc() { return NewStringElement; }
  static inline Arena* GetArena(Type*) { return nullptr; }

  static PROTOBUF_NOINLINE Type* New(Arena* arena, Type&& value) {
    return Arena::Create<Type>(arena, std::move(value));
  }
  static inline void Delete(Type* value, Arena* arena) {
    if (arena == nullptr) {
      delete value;
    }
  }
  static inline void Clear(Type* value) { value->clear(); }
  static inline void Merge(const Type& from, Type* to) { *to = from; }
  static size_t SpaceUsedLong(const Type& value) {
    return sizeof(value) + StringSpaceUsedExcludingSelfLong(value);
  }

  static const Type& default_instance() {
    return GetEmptyStringAlreadyInited();
  }
};

}  // namespace internal

// RepeatedPtrField is like RepeatedField, but used for repeated strings or
// Messages.
template <typename Element>
class RepeatedPtrField final : private internal::RepeatedPtrFieldBase {
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
        absl::disjunction<
            internal::is_supported_string_type<Element>,
            internal::is_supported_message_type<Element>>::value,
        "We only support string and Message types in RepeatedPtrField.");
    static_assert(alignof(Element) <= internal::ArenaAlignDefault::align,
                  "Overaligned types are not supported");
  }

 public:
  using value_type = Element;
  using size_type = int;
  using difference_type = ptrdiff_t;
  using reference = Element&;
  using const_reference = const Element&;
  using pointer = Element*;
  using const_pointer = const Element*;
  using iterator = internal::RepeatedPtrIterator<Element>;
  using const_iterator = internal::RepeatedPtrIterator<const Element>;
  using reverse_iterator = std::reverse_iterator<iterator>;
  using const_reverse_iterator = std::reverse_iterator<const_iterator>;
  // Custom STL-like iterator that iterates over and returns the underlying
  // pointers to Element rather than Element itself.
  using pointer_iterator =
      internal::RepeatedPtrOverPtrsIterator<Element*, void*>;
  using const_pointer_iterator =
      internal::RepeatedPtrOverPtrsIterator<const Element* const,
                                            const void* const>;

  constexpr RepeatedPtrField();

  // Arena enabled constructors: for internal use only.
  RepeatedPtrField(internal::InternalVisibility, Arena* arena)
      : RepeatedPtrField(arena) {}
  RepeatedPtrField(internal::InternalVisibility, Arena* arena,
                   const RepeatedPtrField& rhs)
      : RepeatedPtrField(arena, rhs) {}

  // TODO: make constructor private
  explicit RepeatedPtrField(Arena* arena);

  template <typename Iter,
            typename = typename std::enable_if<std::is_constructible<
                Element, decltype(*std::declval<Iter>())>::value>::type>
  RepeatedPtrField(Iter begin, Iter end);

  RepeatedPtrField(const RepeatedPtrField& rhs)
      : RepeatedPtrField(nullptr, rhs) {}
  RepeatedPtrField& operator=(const RepeatedPtrField& other)
      ABSL_ATTRIBUTE_LIFETIME_BOUND;

  RepeatedPtrField(RepeatedPtrField&& rhs) noexcept
      : RepeatedPtrField(nullptr, std::move(rhs)) {}
  RepeatedPtrField& operator=(RepeatedPtrField&& other) noexcept
      ABSL_ATTRIBUTE_LIFETIME_BOUND;

  ~RepeatedPtrField();

  bool empty() const;
  int size() const;

  const_reference Get(int index) const ABSL_ATTRIBUTE_LIFETIME_BOUND;
  pointer Mutable(int index) ABSL_ATTRIBUTE_LIFETIME_BOUND;

  // Unlike std::vector, adding an element to a RepeatedPtrField doesn't always
  // make a new element; it might re-use an element left over from when the
  // field was Clear()'d or resize()'d smaller.  For this reason, Add() is the
  // fastest API for adding a new element.
  pointer Add() ABSL_ATTRIBUTE_LIFETIME_BOUND;

  // `Add(std::move(value));` is equivalent to `*Add() = std::move(value);`
  // It will either move-construct to the end of this field, or swap value
  // with the new-or-recycled element at the end of this field.  Note that
  // this operation is very slow if this RepeatedPtrField is not on the
  // same Arena, if any, as `value`.
  void Add(Element&& value);

  // Copying to the end of this RepeatedPtrField is slowest of all; it can't
  // reliably copy-construct to the last element of this RepeatedPtrField, for
  // example (unlike std::vector).
  // We currently block this API.  The right way to add to the end is to call
  // Add() and modify the element it points to.
  // If you must add an existing value, call `*Add() = value;`
  void Add(const Element& value) = delete;

  // Append elements in the range [begin, end) after reserving
  // the appropriate number of elements.
  template <typename Iter>
  void Add(Iter begin, Iter end);

  const_reference operator[](int index) const ABSL_ATTRIBUTE_LIFETIME_BOUND {
    return Get(index);
  }
  reference operator[](int index) ABSL_ATTRIBUTE_LIFETIME_BOUND {
    return *Mutable(index);
  }

  const_reference at(int index) const ABSL_ATTRIBUTE_LIFETIME_BOUND;
  reference at(int index) ABSL_ATTRIBUTE_LIFETIME_BOUND;

  // Removes the last element in the array.
  // Ownership of the element is retained by the array.
  void RemoveLast();

  // Deletes elements with indices in the range [start .. start+num-1].
  // Caution: moves all elements with indices [start+num .. ].
  // Calling this routine inside a loop can cause quadratic behavior.
  void DeleteSubrange(int start, int num);

  ABSL_ATTRIBUTE_REINITIALIZES void Clear();

  // Appends the elements from `other` after this instance.
  // The end result length will be `other.size() + this->size()`.
  void MergeFrom(const RepeatedPtrField& other);

  // Replaces the contents with a copy of the elements from `other`.
  ABSL_ATTRIBUTE_REINITIALIZES void CopyFrom(const RepeatedPtrField& other);

  // Replaces the contents with RepeatedPtrField(begin, end).
  template <typename Iter>
  ABSL_ATTRIBUTE_REINITIALIZES void Assign(Iter begin, Iter end);

  // Reserves space to expand the field to at least the given size.  This only
  // resizes the pointer array; it doesn't allocate any objects.  If the
  // array is grown, it will always be at least doubled in size.
  void Reserve(int new_size);

  int Capacity() const;

  // Gets the underlying array.  This pointer is possibly invalidated by
  // any add or remove operation.
  Element**
  mutable_data() ABSL_ATTRIBUTE_LIFETIME_BOUND;
  const Element* const* data() const ABSL_ATTRIBUTE_LIFETIME_BOUND;

  // Swaps entire contents with "other". If they are on separate arenas, then
  // copies data.
  void Swap(RepeatedPtrField* other);

  // Swaps entire contents with "other". Caller should guarantee that either
  // both fields are on the same arena or both are on the heap. Swapping between
  // different arenas with this function is disallowed and is caught via
  // ABSL_DCHECK.
  void UnsafeArenaSwap(RepeatedPtrField* other);

  // Swaps two elements.
  void SwapElements(int index1, int index2);

  iterator begin() ABSL_ATTRIBUTE_LIFETIME_BOUND;
  const_iterator begin() const ABSL_ATTRIBUTE_LIFETIME_BOUND;
  const_iterator cbegin() const ABSL_ATTRIBUTE_LIFETIME_BOUND;
  iterator end() ABSL_ATTRIBUTE_LIFETIME_BOUND;
  const_iterator end() const ABSL_ATTRIBUTE_LIFETIME_BOUND;
  const_iterator cend() const ABSL_ATTRIBUTE_LIFETIME_BOUND;

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

  pointer_iterator pointer_begin() ABSL_ATTRIBUTE_LIFETIME_BOUND;
  const_pointer_iterator pointer_begin() const ABSL_ATTRIBUTE_LIFETIME_BOUND;
  pointer_iterator pointer_end() ABSL_ATTRIBUTE_LIFETIME_BOUND;
  const_pointer_iterator pointer_end() const ABSL_ATTRIBUTE_LIFETIME_BOUND;

  // Returns (an estimate of) the number of bytes used by the repeated field,
  // excluding sizeof(*this).
  size_t SpaceUsedExcludingSelfLong() const;

  int SpaceUsedExcludingSelf() const {
    return internal::ToIntSize(SpaceUsedExcludingSelfLong());
  }

  // Advanced memory management --------------------------------------
  // When hardcore memory management becomes necessary -- as it sometimes
  // does here at Google -- the following methods may be useful.

  // Adds an already-allocated object, passing ownership to the
  // RepeatedPtrField.
  //
  // Note that some special behavior occurs with respect to arenas:
  //
  //   (i) if this field holds submessages, the new submessage will be copied if
  //   the original is in an arena and this RepeatedPtrField is either in a
  //   different arena, or on the heap.
  //   (ii) if this field holds strings, the passed-in string *must* be
  //   heap-allocated, not arena-allocated. There is no way to dynamically check
  //   this at runtime, so User Beware.
  // Requires:  value != nullptr
  void AddAllocated(Element* value);

  // Removes and returns the last element, passing ownership to the caller.
  // Requires:  size() > 0
  //
  // If this RepeatedPtrField is on an arena, an object copy is required to pass
  // ownership back to the user (for compatible semantics). Use
  // UnsafeArenaReleaseLast() if this behavior is undesired.
  [[nodiscard]] Element* ReleaseLast();

  // Adds an already-allocated object, skipping arena-ownership checks. The user
  // must guarantee that the given object is in the same arena as this
  // RepeatedPtrField.
  // It is also useful in legacy code that uses temporary ownership to avoid
  // copies. Example:
  //   RepeatedPtrField<T> temp_field;
  //   temp_field.UnsafeArenaAddAllocated(new T);
  //   ... // Do something with temp_field
  //   temp_field.UnsafeArenaExtractSubrange(0, temp_field.size(), nullptr);
  // If you put temp_field on the arena this fails, because the ownership
  // transfers to the arena at the "AddAllocated" call and is not released
  // anymore, causing a double delete. UnsafeArenaAddAllocated prevents this.
  // Requires:  value != nullptr
  void UnsafeArenaAddAllocated(Element* value);

  // Removes and returns the last element.  Unlike ReleaseLast, the returned
  // pointer is always to the original object.  This may be in an arena, in
  // which case it would have the arena's lifetime.
  // Requires: current_size_ > 0
  pointer UnsafeArenaReleaseLast();

  // Extracts elements with indices in the range "[start .. start+num-1]".
  // The caller assumes ownership of the extracted elements and is responsible
  // for deleting them when they are no longer needed.
  // If "elements" is non-nullptr, then pointers to the extracted elements
  // are stored in "elements[0 .. num-1]" for the convenience of the caller.
  // If "elements" is nullptr, then the caller must use some other mechanism
  // to perform any further operations (like deletion) on these elements.
  // Caution: implementation also moves elements with indices [start+num ..].
  // Calling this routine inside a loop can cause quadratic behavior.
  //
  // Memory copying behavior is identical to ReleaseLast(), described above: if
  // this RepeatedPtrField is on an arena, an object copy is performed for each
  // returned element, so that all returned element pointers are to
  // heap-allocated copies. If this copy is not desired, the user should call
  // UnsafeArenaExtractSubrange().
  void ExtractSubrange(int start, int num, Element** elements);

  // Identical to ExtractSubrange() described above, except that no object
  // copies are ever performed. Instead, the raw object pointers are returned.
  // Thus, if on an arena, the returned objects must not be freed, because they
  // will not be heap-allocated objects.
  void UnsafeArenaExtractSubrange(int start, int num, Element** elements);

  // When elements are removed by calls to RemoveLast() or Clear(), they
  // are not actually freed.  Instead, they are cleared and kept so that
  // they can be reused later.  This can save lots of CPU time when
  // repeatedly reusing a protocol message for similar purposes.
  //
  // Hardcore programs may choose to manipulate these cleared objects
  // to better optimize memory management using the following routines.


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

  // Gets the arena on which this RepeatedPtrField stores its elements.
  inline Arena* GetArena();

  // For internal use only.
  //
  // This is public due to it being called by generated code.
  void InternalSwap(RepeatedPtrField* PROTOBUF_RESTRICT other) {
    internal::RepeatedPtrFieldBase::InternalSwap(other);
  }

  using RepeatedPtrFieldBase::InternalGetArenaOffset;

 private:
  using InternalArenaConstructable_ = void;
  using DestructorSkippable_ = void;

  friend class Arena;

  friend class internal::TcParser;

  template <typename T>
  friend struct WeakRepeatedPtrField;

  // Note:  RepeatedPtrField SHOULD NOT be subclassed by users.
  using TypeHandler = internal::GenericTypeHandler<Element>;

  RepeatedPtrField(Arena* arena, const RepeatedPtrField& rhs);
  RepeatedPtrField(Arena* arena, RepeatedPtrField&& rhs);


  void AddAllocatedForParse(Element* p) {
    return RepeatedPtrFieldBase::AddAllocatedForParse(p);
  }
};

// -------------------------------------------------------------------

template <typename Element>
constexpr RepeatedPtrField<Element>::RepeatedPtrField()
    : RepeatedPtrFieldBase() {
  StaticValidityCheck();
}

template <typename Element>
inline RepeatedPtrField<Element>::RepeatedPtrField(Arena* arena)
    : RepeatedPtrFieldBase(arena) {
  // We can't have StaticValidityCheck here because that requires Element to be
  // a complete type, and in split repeated fields cases, we call
  // CreateMessage<RepeatedPtrField<T>> for incomplete Ts.
}

template <typename Element>
inline RepeatedPtrField<Element>::RepeatedPtrField(Arena* arena,
                                                   const RepeatedPtrField& rhs)
    : RepeatedPtrFieldBase(arena) {
  StaticValidityCheck();
  MergeFrom(rhs);
}

template <typename Element>
template <typename Iter, typename>
inline RepeatedPtrField<Element>::RepeatedPtrField(Iter begin, Iter end) {
  StaticValidityCheck();
  Add(begin, end);
}

template <typename Element>
RepeatedPtrField<Element>::~RepeatedPtrField() {
  StaticValidityCheck();
  if (!NeedsDestroy()) return;
#ifdef __cpp_if_constexpr
  if constexpr (std::is_base_of<MessageLite, Element>::value) {
#else
  if (std::is_base_of<MessageLite, Element>::value) {
#endif
    DestroyProtos();
  } else {
    Destroy<TypeHandler>();
  }
}

template <typename Element>
inline RepeatedPtrField<Element>& RepeatedPtrField<Element>::operator=(
    const RepeatedPtrField& other) ABSL_ATTRIBUTE_LIFETIME_BOUND {
  if (this != &other) CopyFrom(other);
  return *this;
}

template <typename Element>
inline RepeatedPtrField<Element>::RepeatedPtrField(Arena* arena,
                                                   RepeatedPtrField&& rhs)
    : RepeatedPtrField(arena) {
  // We don't just call Swap(&rhs) here because it would perform 3 copies if rhs
  // is on a different arena.
  if (internal::CanMoveWithInternalSwap(arena, rhs.GetArena())) {
    InternalSwap(&rhs);
  } else {
    CopyFrom(rhs);
  }
}

template <typename Element>
inline RepeatedPtrField<Element>& RepeatedPtrField<Element>::operator=(
    RepeatedPtrField&& other) noexcept ABSL_ATTRIBUTE_LIFETIME_BOUND {
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
inline bool RepeatedPtrField<Element>::empty() const {
  return RepeatedPtrFieldBase::empty();
}

template <typename Element>
inline int RepeatedPtrField<Element>::size() const {
  return RepeatedPtrFieldBase::size();
}

template <typename Element>
inline const Element& RepeatedPtrField<Element>::Get(int index) const
    ABSL_ATTRIBUTE_LIFETIME_BOUND {
  return RepeatedPtrFieldBase::Get<TypeHandler>(index);
}

template <typename Element>
inline const Element& RepeatedPtrField<Element>::at(int index) const
    ABSL_ATTRIBUTE_LIFETIME_BOUND {
  return RepeatedPtrFieldBase::at<TypeHandler>(index);
}

template <typename Element>
inline Element& RepeatedPtrField<Element>::at(int index)
    ABSL_ATTRIBUTE_LIFETIME_BOUND {
  return RepeatedPtrFieldBase::at<TypeHandler>(index);
}

template <typename Element>
inline Element* RepeatedPtrField<Element>::Mutable(int index)
    ABSL_ATTRIBUTE_LIFETIME_BOUND {
  return RepeatedPtrFieldBase::Mutable<TypeHandler>(index);
}

template <typename Element>
inline Element* RepeatedPtrField<Element>::Add() ABSL_ATTRIBUTE_LIFETIME_BOUND {
  return RepeatedPtrFieldBase::Add<TypeHandler>();
}

template <typename Element>
inline void RepeatedPtrField<Element>::Add(Element&& value) {
  RepeatedPtrFieldBase::Add<TypeHandler>(std::move(value));
}

template <typename Element>
template <typename Iter>
inline void RepeatedPtrField<Element>::Add(Iter begin, Iter end) {
  if (std::is_base_of<
          std::forward_iterator_tag,
          typename std::iterator_traits<Iter>::iterator_category>::value) {
    int reserve = static_cast<int>(std::distance(begin, end));
    Reserve(size() + reserve);
  }
  for (; begin != end; ++begin) {
    *Add() = *begin;
  }
}

template <typename Element>
inline void RepeatedPtrField<Element>::RemoveLast() {
  RepeatedPtrFieldBase::RemoveLast<TypeHandler>();
}

template <typename Element>
inline void RepeatedPtrField<Element>::DeleteSubrange(int start, int num) {
  ABSL_DCHECK_GE(start, 0);
  ABSL_DCHECK_GE(num, 0);
  ABSL_DCHECK_LE(start + num, size());
  void** subrange = raw_mutable_data() + start;
  Arena* arena = GetArena();
  for (int i = 0; i < num; ++i) {
    using H = CommonHandler<TypeHandler>;
    H::Delete(static_cast<Element*>(subrange[i]), arena);
  }
  UnsafeArenaExtractSubrange(start, num, nullptr);
}

template <typename Element>
inline void RepeatedPtrField<Element>::ExtractSubrange(int start, int num,
                                                       Element** elements) {
  ABSL_DCHECK_GE(start, 0);
  ABSL_DCHECK_GE(num, 0);
  ABSL_DCHECK_LE(start + num, size());

  if (num == 0) return;

  ABSL_DCHECK_NE(elements, nullptr)
      << "Releasing elements without transferring ownership is an unsafe "
         "operation.  Use UnsafeArenaExtractSubrange.";
  if (elements != nullptr) {
    Arena* arena = GetArena();
    auto* extracted = data() + start;
    if (internal::DebugHardenForceCopyInRelease()) {
      // Always copy.
      for (int i = 0; i < num; ++i) {
        elements[i] = copy<TypeHandler>(extracted[i]);
      }
      if (arena == nullptr) {
        for (int i = 0; i < num; ++i) {
          delete extracted[i];
        }
      }
    } else {
      // If we're on an arena, we perform a copy for each element so that the
      // returned elements are heap-allocated. Otherwise, just forward it.
      if (arena != nullptr) {
        for (int i = 0; i < num; ++i) {
          elements[i] = copy<TypeHandler>(extracted[i]);
        }
      } else {
        memcpy(elements, extracted, num * sizeof(Element*));
      }
    }
  }
  CloseGap(start, num);
}

template <typename Element>
inline void RepeatedPtrField<Element>::UnsafeArenaExtractSubrange(
    int start, int num, Element** elements) {
  ABSL_DCHECK_GE(start, 0);
  ABSL_DCHECK_GE(num, 0);
  ABSL_DCHECK_LE(start + num, size());

  if (num > 0) {
    // Save the values of the removed elements if requested.
    if (elements != nullptr) {
      memcpy(elements, data() + start, num * sizeof(Element*));
    }
    CloseGap(start, num);
  }
}

template <typename Element>
inline void RepeatedPtrField<Element>::Clear() {
  RepeatedPtrFieldBase::Clear<TypeHandler>();
}

template <typename Element>
inline void RepeatedPtrField<Element>::MergeFrom(
    const RepeatedPtrField& other) {
  if (other.empty()) return;
  RepeatedPtrFieldBase::MergeFrom<Element>(other);
}

template <typename Element>
inline void RepeatedPtrField<Element>::CopyFrom(const RepeatedPtrField& other) {
  RepeatedPtrFieldBase::CopyFrom<TypeHandler>(other);
}

template <typename Element>
template <typename Iter>
inline void RepeatedPtrField<Element>::Assign(Iter begin, Iter end) {
  Clear();
  Add(begin, end);
}

template <typename Element>
inline typename RepeatedPtrField<Element>::iterator
RepeatedPtrField<Element>::erase(const_iterator position)
    ABSL_ATTRIBUTE_LIFETIME_BOUND {
  return erase(position, position + 1);
}

template <typename Element>
inline typename RepeatedPtrField<Element>::iterator
RepeatedPtrField<Element>::erase(const_iterator first, const_iterator last)
    ABSL_ATTRIBUTE_LIFETIME_BOUND {
  size_type pos_offset = static_cast<size_type>(std::distance(cbegin(), first));
  size_type last_offset = static_cast<size_type>(std::distance(cbegin(), last));
  DeleteSubrange(pos_offset, last_offset - pos_offset);
  return begin() + pos_offset;
}

template <typename Element>
inline Element** RepeatedPtrField<Element>::mutable_data()
    ABSL_ATTRIBUTE_LIFETIME_BOUND {
  return RepeatedPtrFieldBase::mutable_data<TypeHandler>();
}

template <typename Element>
inline const Element* const* RepeatedPtrField<Element>::data() const
    ABSL_ATTRIBUTE_LIFETIME_BOUND {
  return RepeatedPtrFieldBase::data<TypeHandler>();
}

template <typename Element>
inline void RepeatedPtrField<Element>::Swap(RepeatedPtrField* other) {
  if (this == other) return;
  RepeatedPtrFieldBase::Swap<TypeHandler>(other);
}

template <typename Element>
inline void RepeatedPtrField<Element>::UnsafeArenaSwap(
    RepeatedPtrField* other) {
  if (this == other) return;
  ABSL_DCHECK_EQ(GetArena(), other->GetArena());
  RepeatedPtrFieldBase::InternalSwap(other);
}

template <typename Element>
inline void RepeatedPtrField<Element>::SwapElements(int index1, int index2) {
  RepeatedPtrFieldBase::SwapElements(index1, index2);
}

template <typename Element>
inline Arena* RepeatedPtrField<Element>::GetArena() {
  return RepeatedPtrFieldBase::GetArena();
}

template <typename Element>
inline size_t RepeatedPtrField<Element>::SpaceUsedExcludingSelfLong() const {
  // `google::protobuf::Message` has a virtual method `SpaceUsedLong`, hence we can
  // instantiate just one function for all protobuf messages.
  // Note: std::is_base_of requires that `Element` is a concrete class.
  using H = typename std::conditional<std::is_base_of<Message, Element>::value,
                                      internal::GenericTypeHandler<Message>,
                                      TypeHandler>::type;
  return RepeatedPtrFieldBase::SpaceUsedExcludingSelfLong<H>();
}

template <typename Element>
inline void RepeatedPtrField<Element>::AddAllocated(Element* value) {
  RepeatedPtrFieldBase::AddAllocated<TypeHandler>(value);
}

template <typename Element>
inline void RepeatedPtrField<Element>::UnsafeArenaAddAllocated(Element* value) {
  RepeatedPtrFieldBase::UnsafeArenaAddAllocated<TypeHandler>(value);
}

template <typename Element>
inline Element* RepeatedPtrField<Element>::ReleaseLast() {
  return RepeatedPtrFieldBase::ReleaseLast<TypeHandler>();
}

template <typename Element>
inline Element* RepeatedPtrField<Element>::UnsafeArenaReleaseLast() {
  return RepeatedPtrFieldBase::UnsafeArenaReleaseLast<TypeHandler>();
}


template <typename Element>
inline void RepeatedPtrField<Element>::Reserve(int new_size) {
  return RepeatedPtrFieldBase::Reserve(new_size);
}

template <typename Element>
inline int RepeatedPtrField<Element>::Capacity() const {
  return RepeatedPtrFieldBase::Capacity();
}

// -------------------------------------------------------------------

namespace internal {

// This class gives the Rust implementation access to some protected methods on
// RepeatedPtrFieldBase. These methods allow us to operate solely on the
// MessageLite interface so that we do not need to generate code for each
// concrete message type.
class RustRepeatedMessageHelper {
 public:
  static RepeatedPtrFieldBase* New() { return new RepeatedPtrFieldBase; }

  static void Delete(RepeatedPtrFieldBase* field) {
    if (field->NeedsDestroy()) {
      field->DestroyProtos();
    }
    delete field;
  }

  static size_t Size(const RepeatedPtrFieldBase& field) {
    return static_cast<size_t>(field.size());
  }

  static void Reserve(RepeatedPtrFieldBase& field, size_t additional) {
    field.Reserve(field.size() + additional);
  }

  static const MessageLite& At(const RepeatedPtrFieldBase& field,
                               size_t index) {
    return field.at<GenericTypeHandler<MessageLite>>(index);
  }

  static MessageLite& At(RepeatedPtrFieldBase& field, size_t index) {
    return field.at<GenericTypeHandler<MessageLite>>(index);
  }
};

// STL-like iterator implementation for RepeatedPtrField.  You should not
// refer to this class directly; use RepeatedPtrField<T>::iterator instead.
//
// The iterator for RepeatedPtrField<T>, RepeatedPtrIterator<T>, is
// very similar to iterator_ptr<T**> in util/gtl/iterator_adaptors.h,
// but adds random-access operators and is modified to wrap a void** base
// iterator (since RepeatedPtrField stores its array as a void* array and
// casting void** to T** would violate C++ aliasing rules).
//
// This code based on net/proto/proto-array-internal.h by Jeffrey Yasskin
// (jyasskin@google.com).
template <typename Element>
class RepeatedPtrIterator {
 public:
  using iterator = RepeatedPtrIterator<Element>;
  using iterator_category = std::random_access_iterator_tag;
  using value_type = typename std::remove_const<Element>::type;
  using difference_type = std::ptrdiff_t;
  using pointer = Element*;
  using reference = Element&;

  RepeatedPtrIterator() : it_(nullptr) {}
  explicit RepeatedPtrIterator(void* const* it) : it_(it) {}

  // Allows "upcasting" from RepeatedPtrIterator<T**> to
  // RepeatedPtrIterator<const T*const*>.
  template <typename OtherElement,
            typename std::enable_if<std::is_convertible<
                OtherElement*, pointer>::value>::type* = nullptr>
  RepeatedPtrIterator(const RepeatedPtrIterator<OtherElement>& other)
      : it_(other.it_) {}

  // dereferenceable
  reference operator*() const { return *reinterpret_cast<Element*>(*it_); }
  pointer operator->() const { return &(operator*()); }

  // {inc,dec}rementable
  iterator& operator++() {
    ++it_;
    return *this;
  }
  iterator operator++(int) { return iterator(it_++); }
  iterator& operator--() {
    --it_;
    return *this;
  }
  iterator operator--(int) { return iterator(it_--); }

  // equality_comparable
  friend bool operator==(const iterator& x, const iterator& y) {
    return x.it_ == y.it_;
  }
  friend bool operator!=(const iterator& x, const iterator& y) {
    return x.it_ != y.it_;
  }

  // less_than_comparable
  friend bool operator<(const iterator& x, const iterator& y) {
    return x.it_ < y.it_;
  }
  friend bool operator<=(const iterator& x, const iterator& y) {
    return x.it_ <= y.it_;
  }
  friend bool operator>(const iterator& x, const iterator& y) {
    return x.it_ > y.it_;
  }
  friend bool operator>=(const iterator& x, const iterator& y) {
    return x.it_ >= y.it_;
  }

  // addable, subtractable
  iterator& operator+=(difference_type d) {
    it_ += d;
    return *this;
  }
  friend iterator operator+(iterator it, const difference_type d) {
    it += d;
    return it;
  }
  friend iterator operator+(const difference_type d, iterator it) {
    it += d;
    return it;
  }
  iterator& operator-=(difference_type d) {
    it_ -= d;
    return *this;
  }
  friend iterator operator-(iterator it, difference_type d) {
    it -= d;
    return it;
  }

  // indexable
  reference operator[](difference_type d) const { return *(*this + d); }

  // random access iterator
  friend difference_type operator-(iterator it1, iterator it2) {
    return it1.it_ - it2.it_;
  }

 private:
  template <typename OtherElement>
  friend class RepeatedPtrIterator;

  // The internal iterator.
  void* const* it_;
};

template <typename Traits, typename = void>
struct IteratorConceptSupport {
  using tag = typename Traits::iterator_category;
};

template <typename Traits>
struct IteratorConceptSupport<Traits,
                              absl::void_t<typename Traits::iterator_concept>> {
  using tag = typename Traits::iterator_concept;
};

// Provides an iterator that operates on pointers to the underlying objects
// rather than the objects themselves as RepeatedPtrIterator does.
// Consider using this when working with stl algorithms that change
// the array.
// The VoidPtr template parameter holds the type-agnostic pointer value
// referenced by the iterator.  It should either be "void *" for a mutable
// iterator, or "const void* const" for a constant iterator.
template <typename Element, typename VoidPtr>
class RepeatedPtrOverPtrsIterator {
 private:
  using traits =
      std::iterator_traits<typename std::remove_const<Element>::type*>;

 public:
  using value_type = typename traits::value_type;
  using difference_type = typename traits::difference_type;
  using pointer = Element*;
  using reference = Element&;
  using iterator_category = typename traits::iterator_category;
  using iterator_concept = typename IteratorConceptSupport<traits>::tag;

  using iterator = RepeatedPtrOverPtrsIterator<Element, VoidPtr>;

  RepeatedPtrOverPtrsIterator() : it_(nullptr) {}
  explicit RepeatedPtrOverPtrsIterator(VoidPtr* it) : it_(it) {}

  // Allows "upcasting" from RepeatedPtrOverPtrsIterator<T**> to
  // RepeatedPtrOverPtrsIterator<const T*const*>.
  template <
      typename OtherElement, typename OtherVoidPtr,
      typename std::enable_if<
          std::is_convertible<OtherElement*, pointer>::value &&
          std::is_convertible<OtherVoidPtr, VoidPtr>::value>::type* = nullptr>
  RepeatedPtrOverPtrsIterator(
      const RepeatedPtrOverPtrsIterator<OtherElement, OtherVoidPtr>& other)
      : it_(other.it_) {}

  // dereferenceable
  reference operator*() const { return *reinterpret_cast<Element*>(it_); }
  pointer operator->() const { return reinterpret_cast<Element*>(it_); }

  // {inc,dec}rementable
  iterator& operator++() {
    ++it_;
    return *this;
  }
  iterator operator++(int) { return iterator(it_++); }
  iterator& operator--() {
    --it_;
    return *this;
  }
  iterator operator--(int) { return iterator(it_--); }

  // equality_comparable
  friend bool operator==(const iterator& x, const iterator& y) {
    return x.it_ == y.it_;
  }
  friend bool operator!=(const iterator& x, const iterator& y) {
    return x.it_ != y.it_;
  }

  // less_than_comparable
  friend bool operator<(const iterator& x, const iterator& y) {
    return x.it_ < y.it_;
  }
  friend bool operator<=(const iterator& x, const iterator& y) {
    return x.it_ <= y.it_;
  }
  friend bool operator>(const iterator& x, const iterator& y) {
    return x.it_ > y.it_;
  }
  friend bool operator>=(const iterator& x, const iterator& y) {
    return x.it_ >= y.it_;
  }

  // addable, subtractable
  iterator& operator+=(difference_type d) {
    it_ += d;
    return *this;
  }
  friend iterator operator+(iterator it, difference_type d) {
    it += d;
    return it;
  }
  friend iterator operator+(difference_type d, iterator it) {
    it += d;
    return it;
  }
  iterator& operator-=(difference_type d) {
    it_ -= d;
    return *this;
  }
  friend iterator operator-(iterator it, difference_type d) {
    it -= d;
    return it;
  }

  // indexable
  reference operator[](difference_type d) const { return *(*this + d); }

  // random access iterator
  friend difference_type operator-(iterator it1, iterator it2) {
    return it1.it_ - it2.it_;
  }

 private:
  template <typename OtherElement, typename OtherVoidPtr>
  friend class RepeatedPtrOverPtrsIterator;

  // The internal iterator.
  VoidPtr* it_;
};

}  // namespace internal

template <typename Element>
inline typename RepeatedPtrField<Element>::iterator
RepeatedPtrField<Element>::begin() ABSL_ATTRIBUTE_LIFETIME_BOUND {
  return iterator(raw_data());
}
template <typename Element>
inline typename RepeatedPtrField<Element>::const_iterator
RepeatedPtrField<Element>::begin() const ABSL_ATTRIBUTE_LIFETIME_BOUND {
  return iterator(raw_data());
}
template <typename Element>
inline typename RepeatedPtrField<Element>::const_iterator
RepeatedPtrField<Element>::cbegin() const ABSL_ATTRIBUTE_LIFETIME_BOUND {
  return begin();
}
template <typename Element>
inline typename RepeatedPtrField<Element>::iterator
RepeatedPtrField<Element>::end() ABSL_ATTRIBUTE_LIFETIME_BOUND {
  return iterator(raw_data() + size());
}
template <typename Element>
inline typename RepeatedPtrField<Element>::const_iterator
RepeatedPtrField<Element>::end() const ABSL_ATTRIBUTE_LIFETIME_BOUND {
  return iterator(raw_data() + size());
}
template <typename Element>
inline typename RepeatedPtrField<Element>::const_iterator
RepeatedPtrField<Element>::cend() const ABSL_ATTRIBUTE_LIFETIME_BOUND {
  return end();
}

template <typename Element>
inline typename RepeatedPtrField<Element>::pointer_iterator
RepeatedPtrField<Element>::pointer_begin() ABSL_ATTRIBUTE_LIFETIME_BOUND {
  return pointer_iterator(raw_mutable_data());
}
template <typename Element>
inline typename RepeatedPtrField<Element>::const_pointer_iterator
RepeatedPtrField<Element>::pointer_begin() const ABSL_ATTRIBUTE_LIFETIME_BOUND {
  return const_pointer_iterator(const_cast<const void* const*>(raw_data()));
}
template <typename Element>
inline typename RepeatedPtrField<Element>::pointer_iterator
RepeatedPtrField<Element>::pointer_end() ABSL_ATTRIBUTE_LIFETIME_BOUND {
  return pointer_iterator(raw_mutable_data() + size());
}
template <typename Element>
inline typename RepeatedPtrField<Element>::const_pointer_iterator
RepeatedPtrField<Element>::pointer_end() const ABSL_ATTRIBUTE_LIFETIME_BOUND {
  return const_pointer_iterator(
      const_cast<const void* const*>(raw_data() + size()));
}

// Iterators and helper functions that follow the spirit of the STL
// std::back_insert_iterator and std::back_inserter but are tailor-made
// for RepeatedField and RepeatedPtrField. Typical usage would be:
//
//   std::copy(some_sequence.begin(), some_sequence.end(),
//             RepeatedFieldBackInserter(proto.mutable_sequence()));
//
// Ported by johannes from util/gtl/proto-array-iterators.h

namespace internal {

// A back inserter for RepeatedPtrField objects.
template <typename T>
class RepeatedPtrFieldBackInsertIterator {
 public:
  using iterator_category = std::output_iterator_tag;
  using value_type = T;
  using pointer = void;
  using reference = void;
  using difference_type = std::ptrdiff_t;

  RepeatedPtrFieldBackInsertIterator(RepeatedPtrField<T>* const mutable_field)
      : field_(mutable_field) {}
  RepeatedPtrFieldBackInsertIterator<T>& operator=(const T& value) {
    *field_->Add() = value;
    return *this;
  }
  RepeatedPtrFieldBackInsertIterator<T>& operator=(
      const T* const ptr_to_value) {
    *field_->Add() = *ptr_to_value;
    return *this;
  }
  RepeatedPtrFieldBackInsertIterator<T>& operator=(T&& value) {
    *field_->Add() = std::move(value);
    return *this;
  }
  RepeatedPtrFieldBackInsertIterator<T>& operator*() { return *this; }
  RepeatedPtrFieldBackInsertIterator<T>& operator++() { return *this; }
  RepeatedPtrFieldBackInsertIterator<T>& operator++(int /* unused */) {
    return *this;
  }

 private:
  RepeatedPtrField<T>* field_;
};

// A back inserter for RepeatedPtrFields that inserts by transferring ownership
// of a pointer.
template <typename T>
class AllocatedRepeatedPtrFieldBackInsertIterator {
 public:
  using iterator_category = std::output_iterator_tag;
  using value_type = T;
  using pointer = void;
  using reference = void;
  using difference_type = std::ptrdiff_t;

  explicit AllocatedRepeatedPtrFieldBackInsertIterator(
      RepeatedPtrField<T>* const mutable_field)
      : field_(mutable_field) {}
  AllocatedRepeatedPtrFieldBackInsertIterator<T>& operator=(
      T* const ptr_to_value) {
    field_->AddAllocated(ptr_to_value);
    return *this;
  }
  AllocatedRepeatedPtrFieldBackInsertIterator<T>& operator*() { return *this; }
  AllocatedRepeatedPtrFieldBackInsertIterator<T>& operator++() { return *this; }
  AllocatedRepeatedPtrFieldBackInsertIterator<T>& operator++(int /* unused */) {
    return *this;
  }

 private:
  RepeatedPtrField<T>* field_;
};

// Almost identical to AllocatedRepeatedPtrFieldBackInsertIterator. This one
// uses the UnsafeArenaAddAllocated instead.
template <typename T>
class UnsafeArenaAllocatedRepeatedPtrFieldBackInsertIterator {
 public:
  using iterator_category = std::output_iterator_tag;
  using value_type = T;
  using pointer = void;
  using reference = void;
  using difference_type = std::ptrdiff_t;

  explicit UnsafeArenaAllocatedRepeatedPtrFieldBackInsertIterator(
      RepeatedPtrField<T>* const mutable_field)
      : field_(mutable_field) {}
  UnsafeArenaAllocatedRepeatedPtrFieldBackInsertIterator<T>& operator=(
      T const* const ptr_to_value) {
    field_->UnsafeArenaAddAllocated(const_cast<T*>(ptr_to_value));
    return *this;
  }
  UnsafeArenaAllocatedRepeatedPtrFieldBackInsertIterator<T>& operator*() {
    return *this;
  }
  UnsafeArenaAllocatedRepeatedPtrFieldBackInsertIterator<T>& operator++() {
    return *this;
  }
  UnsafeArenaAllocatedRepeatedPtrFieldBackInsertIterator<T>& operator++(
      int /* unused */) {
    return *this;
  }

 private:
  RepeatedPtrField<T>* field_;
};

// A utility function for logging that doesn't need any template types.
PROTOBUF_EXPORT void LogIndexOutOfBounds(int index, int size);

// A utility function for logging that doesn't need any template types. Same as
// LogIndexOutOfBounds, but aborts the program in all cases by logging to FATAL
// instead of DFATAL.
[[noreturn]] PROTOBUF_EXPORT void LogIndexOutOfBoundsAndAbort(int index,
                                                              int size);

template <typename T>
const T& CheckedGetOrDefault(const RepeatedPtrField<T>& field, int index) {
  if (ABSL_PREDICT_FALSE(index < 0 || index >= field.size())) {
    LogIndexOutOfBounds(index, field.size());
    return GenericTypeHandler<T>::default_instance();
  }
  return field.Get(index);
}

template <typename T>
inline void CheckIndexInBoundsOrAbort(const RepeatedPtrField<T>& field,
                                      int index) {
  if (ABSL_PREDICT_FALSE(index < 0 || index >= field.size())) {
    LogIndexOutOfBoundsAndAbort(index, field.size());
  }
}

template <typename T>
const T& CheckedGetOrAbort(const RepeatedPtrField<T>& field, int index) {
  CheckIndexInBoundsOrAbort(field, index);
  return field.Get(index);
}

template <typename T>
inline T* CheckedMutableOrAbort(RepeatedPtrField<T>* field, int index) {
  CheckIndexInBoundsOrAbort(*field, index);
  return field->Mutable(index);
}

}  // namespace internal

// Provides a back insert iterator for RepeatedPtrField instances,
// similar to std::back_inserter().
template <typename T>
internal::RepeatedPtrFieldBackInsertIterator<T> RepeatedPtrFieldBackInserter(
    RepeatedPtrField<T>* const mutable_field) {
  return internal::RepeatedPtrFieldBackInsertIterator<T>(mutable_field);
}

// Special back insert iterator for RepeatedPtrField instances, just in
// case someone wants to write generic template code that can access both
// RepeatedFields and RepeatedPtrFields using a common name.
template <typename T>
internal::RepeatedPtrFieldBackInsertIterator<T> RepeatedFieldBackInserter(
    RepeatedPtrField<T>* const mutable_field) {
  return internal::RepeatedPtrFieldBackInsertIterator<T>(mutable_field);
}

// Provides a back insert iterator for RepeatedPtrField instances
// similar to std::back_inserter() which transfers the ownership while
// copying elements.
template <typename T>
internal::AllocatedRepeatedPtrFieldBackInsertIterator<T>
AllocatedRepeatedPtrFieldBackInserter(
    RepeatedPtrField<T>* const mutable_field) {
  return internal::AllocatedRepeatedPtrFieldBackInsertIterator<T>(
      mutable_field);
}

// Similar to AllocatedRepeatedPtrFieldBackInserter, using
// UnsafeArenaAddAllocated instead of AddAllocated.
// This is slightly faster if that matters. It is also useful in legacy code
// that uses temporary ownership to avoid copies. Example:
//   RepeatedPtrField<T> temp_field;
//   temp_field.UnsafeArenaAddAllocated(new T);
//   ... // Do something with temp_field
//   temp_field.UnsafeArenaExtractSubrange(0, temp_field.size(), nullptr);
// Putting temp_field on the arena fails because the ownership transfers to the
// arena at the "AddAllocated" call and is not released anymore causing a
// double delete. This function uses UnsafeArenaAddAllocated to prevent this.
template <typename T>
internal::UnsafeArenaAllocatedRepeatedPtrFieldBackInsertIterator<T>
UnsafeArenaAllocatedRepeatedPtrFieldBackInserter(
    RepeatedPtrField<T>* const mutable_field) {
  return internal::UnsafeArenaAllocatedRepeatedPtrFieldBackInsertIterator<T>(
      mutable_field);
}


namespace internal {
// Size optimization for `memswap<N>` - supplied below N is used by every
// `RepeatedPtrField<T>`.
extern template PROTOBUF_EXPORT_TEMPLATE_DECLARE void
memswap<ArenaOffsetHelper<RepeatedPtrFieldBase>::value>(
    char* PROTOBUF_RESTRICT, char* PROTOBUF_RESTRICT);
}  // namespace internal

}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_REPEATED_PTR_FIELD_H__
