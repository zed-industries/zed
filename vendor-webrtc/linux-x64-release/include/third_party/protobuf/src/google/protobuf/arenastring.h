// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_ARENASTRING_H__
#define GOOGLE_PROTOBUF_ARENASTRING_H__

#include <algorithm>
#include <cstdint>
#include <string>
#include <type_traits>
#include <utility>

#include "absl/log/absl_check.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/arena.h"
#include "google/protobuf/explicitly_constructed.h"
#include "google/protobuf/port.h"

// must be last:
#include "google/protobuf/port_def.inc"

#ifdef SWIG
#error "You cannot SWIG proto headers"
#endif


namespace google {
namespace protobuf {
namespace internal {
class EpsCopyInputStream;

class SwapFieldHelper;

// Lazy string instance to support string fields with non-empty default.
// These are initialized on the first call to .get().
class PROTOBUF_EXPORT LazyString {
 public:
  // We explicitly make LazyString an aggregate so that MSVC can do constant
  // initialization on it without marking it `constexpr`.
  // We do not want to use `constexpr` because it makes it harder to have extern
  // storage for it and causes library bloat.
  struct InitValue {
    const char* ptr;
    size_t size;
  };
  // We keep a union of the initialization value and the std::string to save on
  // space. We don't need the string array after Init() is done.
  union {
    mutable InitValue init_value_;
    alignas(std::string) mutable char string_buf_[sizeof(std::string)];
  };
  mutable std::atomic<const std::string*> inited_;

  const std::string& get() const {
    // This check generates less code than a call-once invocation.
    auto* res = inited_.load(std::memory_order_acquire);
    if (ABSL_PREDICT_FALSE(res == nullptr)) return Init();
    return *res;
  }

 private:
  // Initialize the string in `string_buf_`, update `inited_` and return it.
  // We return it here to avoid having to read it again in the inlined code.
  const std::string& Init() const;
};

class PROTOBUF_EXPORT TaggedStringPtr {
 public:
  // Bit flags qualifying string properties. We can use 2 bits as
  // ptr_ is guaranteed and enforced to be aligned on 4 byte boundaries.
  enum Flags {
    kArenaBit = 0x1,    // ptr is arena allocated
    kMutableBit = 0x2,  // ptr contents are fully mutable
    kMask = 0x3         // Bit mask
  };

  // Composed logical types
  enum Type {
    // Default strings are immutable and never owned.
    kDefault = 0,

    // Allocated strings are mutable and (as the name implies) owned.
    // A heap allocated string must be deleted.
    kAllocated = kMutableBit,

    // Mutable arena strings are strings where the string instance is owned
    // by the arena, but the string contents itself are owned by the string
    // instance. Mutable arena string instances need to be destroyed which is
    // typically done through a cleanup action added to the arena owning it.
    kMutableArena = kArenaBit | kMutableBit,

    // Fixed size arena strings are strings where both the string instance and
    // the string contents are fully owned by the arena. Fixed size arena
    // strings are a platform and c++ library specific customization. Fixed
    // size arena strings are immutable, with the exception of custom internal
    // updates to the content that fit inside the existing capacity.
    // Fixed size arena strings must never be deleted or destroyed.
    kFixedSizeArena = kArenaBit,
  };

  TaggedStringPtr() = default;
  explicit constexpr TaggedStringPtr(const GlobalEmptyString* ptr)
      : ptr_(const_cast<void*>(static_cast<const void*>(ptr))) {}

  // Sets the value to `p`, tagging the value as being a 'default' value.
  // See documentation for kDefault for more info.
  inline const std::string* SetDefault(const std::string* p) {
    return TagAs(kDefault, const_cast<std::string*>(p));
  }

  // Sets the value to `p`, tagging the value as a heap allocated value.
  // Allocated strings are mutable and (as the name implies) owned.
  // `p` must not be null
  inline std::string* SetAllocated(std::string* p) {
    return TagAs(kAllocated, p);
  }

  // Sets the value to `p`, tagging the value as a fixed size arena string.
  // See documentation for kFixedSizeArena for more info.
  // `p` must not be null
  inline std::string* SetFixedSizeArena(std::string* p) {
    return TagAs(kFixedSizeArena, p);
  }

  // Sets the value to `p`, tagging the value as a mutable arena string.
  // See documentation for kMutableArena for more info.
  // `p` must not be null
  inline std::string* SetMutableArena(std::string* p) {
    return TagAs(kMutableArena, p);
  }

  // Returns true if the contents of the current string are fully mutable.
  inline bool IsMutable() const { return as_int() & kMutableBit; }

  // Returns true if the current string is an immutable default value.
  inline bool IsDefault() const { return (as_int() & kMask) == kDefault; }

  // If the current string is a heap-allocated mutable value, returns a pointer
  // to it.  Returns nullptr otherwise.
  inline std::string* GetIfAllocated() const {
    auto allocated = as_int() ^ kAllocated;
    if (allocated & kMask) return nullptr;

    auto ptr = reinterpret_cast<std::string*>(allocated);
    PROTOBUF_ASSUME(ptr != nullptr);
    return ptr;
  }

  // Returns true if the current string is an arena allocated value.
  // This means it's either a mutable or fixed size arena string.
  inline bool IsArena() const { return as_int() & kArenaBit; }

  // Returns true if the current string is a fixed size arena allocated value.
  inline bool IsFixedSizeArena() const {
    return (as_int() & kMask) == kFixedSizeArena;
  }

  // Returns the contained string pointer.
  inline std::string* Get() const {
    return reinterpret_cast<std::string*>(as_int() & ~kMask);
  }

  // Returns true if the contained pointer is null, indicating some error.
  // The Null value is only used during parsing for temporary values.
  // A persisted ArenaStringPtr value is never null.
  inline bool IsNull() const { return ptr_ == nullptr; }

  // Returns a copy of this instance. In debug builds, the returned value may be
  // a forced copy regardless if the current instance is a compile time default.
  TaggedStringPtr Copy(Arena* arena) const;

  // Identical to the above `Copy` function except that in debug builds,
  // `default_value` can be used to substitute an empty default with a
  // hardened copy of the default value.
  TaggedStringPtr Copy(Arena* arena, const LazyString& default_value) const;

 private:
  static inline void assert_aligned(const void* p) {
    static_assert(kMask <= alignof(void*), "Pointer underaligned for bit mask");
    static_assert(kMask <= alignof(std::string),
                  "std::string underaligned for bit mask");
    ABSL_DCHECK_EQ(reinterpret_cast<uintptr_t>(p) & kMask, 0UL);
  }

  // Creates a heap or arena allocated copy of this instance.
  TaggedStringPtr ForceCopy(Arena* arena) const;

  inline std::string* TagAs(Type type, std::string* p) {
    ABSL_DCHECK(p != nullptr);
    assert_aligned(p);
    ptr_ = reinterpret_cast<void*>(reinterpret_cast<uintptr_t>(p) | type);
    return p;
  }

  uintptr_t as_int() const { return reinterpret_cast<uintptr_t>(ptr_); }
  void* ptr_;
};

static_assert(std::is_trivial<TaggedStringPtr>::value,
              "TaggedStringPtr must be trivial");

// This class encapsulates a pointer to a std::string with or without arena
// owned contents, tagged by the bottom bits of the string pointer. It is a
// high-level wrapper that almost directly corresponds to the interface required
// by string fields in generated code. It replaces the old std::string* pointer
// in such cases.
//
// The string pointer is tagged to be either a default, externally owned value,
// a mutable heap allocated value, or an arena allocated value. The object uses
// a single global instance of an empty string that is used as the initial
// default value. Fields that have empty default values directly use this global
// default. Fields that have non empty default values are supported through
// lazily initialized default values managed by the LazyString class.
//
// Generated code and reflection code both ensure that ptr_ is never null.
// Because ArenaStringPtr is used in oneof unions, its constructor is a NOP and
// the field is always manually initialized via method calls.
//
// See TaggedStringPtr for more information about the types of string values
// being held, and the mutable and ownership invariants for each type.
struct PROTOBUF_EXPORT ArenaStringPtr {
  // Default constructor, leaves current instance uninitialized (does nothing)
  ArenaStringPtr() = default;

  // Constexpr constructor, initializes to a constexpr, empty string value.
  constexpr ArenaStringPtr(const GlobalEmptyString* default_value,
                           ConstantInitialized)
      : tagged_ptr_(default_value) {}

  // Arena enabled constructor for strings without a default value.
  // Initializes this instance to a constexpr, empty string value, unless debug
  // hardening is enabled, in which case this instance will hold a forced copy.
  explicit ArenaStringPtr(Arena* arena)
      : tagged_ptr_(&fixed_address_empty_string) {
    if (DebugHardenForceCopyDefaultString()) {
      Set(absl::string_view(""), arena);
    }
  }

  // Arena enabled constructor for strings with a non-empty default value.
  // Initializes this instance to a constexpr, empty string value, unless debug
  // hardening is enabled, in which case this instance will be forced to hold a
  // forced copy of the value in `default_value`.
  ArenaStringPtr(Arena* arena, const LazyString& default_value)
      : tagged_ptr_(&fixed_address_empty_string) {
    if (DebugHardenForceCopyDefaultString()) {
      Set(absl::string_view(default_value.get()), arena);
    }
  }

  // Arena enabled copy constructor for strings without a default value.
  // This instance will be initialized with a copy of the value in `rhs`.
  // If `rhs` holds a default (empty) value, then this instance will also be
  // initialized with the default empty value, unless debug hardening is
  // enabled, in which case this instance will be forced to hold a copy of
  // an empty default value.
  ArenaStringPtr(Arena* arena, const ArenaStringPtr& rhs)
      : tagged_ptr_(rhs.tagged_ptr_.Copy(arena)) {}

  // Arena enabled copy constructor for strings with a non-empty default value.
  // This instance will be initialized with a copy of the value in `rhs`.
  // If `rhs` holds a default (empty) value, then this instance will also be
  // initialized with the default empty value, unless debug hardening is
  // enabled, in which case this instance will be forced to hold forced copy
  // of the value in `default_value`.
  ArenaStringPtr(Arena* arena, const ArenaStringPtr& rhs,
                 const LazyString& default_value)
      : tagged_ptr_(rhs.tagged_ptr_.Copy(arena, default_value)) {}

  // Called from generated code / reflection runtime only. Resets value to point
  // to a default string pointer, with the semantics that this ArenaStringPtr
  // does not own the pointed-to memory. Disregards initial value of ptr_ (so
  // this is the *ONLY* safe method to call after construction or when
  // reinitializing after becoming the active field in a oneof union).
  inline void InitDefault();

  // Similar to `InitDefault` except that it allows the default value to be
  // initialized to an externally owned string. This method is called from
  // parsing code. `str` must not be null and outlive this instance.
  inline void InitExternal(const std::string* str);

  // Called from generated code / reflection runtime only. Resets the value of
  // this instances to the heap allocated value in `str`. `str` must not be
  // null. Invokes `arena->Own(str)` to transfer ownership into the arena if
  // `arena` is not null, else, `str` will be owned by ArenaStringPtr. This
  // function should only be used to initialize a ArenaStringPtr or on an
  // instance known to not carry any heap allocated value.
  inline void InitAllocated(std::string* str, Arena* arena);

  void Set(absl::string_view value, Arena* arena);
  void Set(std::string&& value, Arena* arena);
  template <typename... OverloadDisambiguator>
  void Set(const std::string& value, Arena* arena);
  void Set(const char* s, Arena* arena);
  void Set(const char* s, size_t n, Arena* arena);

  void SetBytes(absl::string_view value, Arena* arena);
  void SetBytes(std::string&& value, Arena* arena);
  template <typename... OverloadDisambiguator>
  void SetBytes(const std::string& value, Arena* arena);
  void SetBytes(const char* s, Arena* arena);
  void SetBytes(const void* p, size_t n, Arena* arena);

  template <typename RefWrappedType>
  void Set(std::reference_wrapper<RefWrappedType> const_string_ref,
           ::google::protobuf::Arena* arena) {
    Set(const_string_ref.get(), arena);
  }

  // Returns a mutable std::string reference.
  // The version accepting a `LazyString` value is used in the generated code to
  // initialize mutable copies for fields with a non-empty default where the
  // default value is lazily initialized.
  std::string* Mutable(Arena* arena);
  std::string* Mutable(const LazyString& default_value, Arena* arena);

  // Gets a mutable pointer with unspecified contents.
  // This function is identical to Mutable(), except it is optimized for the
  // case where the caller is not interested in the current contents. For
  // example, if the current field is not mutable, it will re-initialize the
  // value with an empty string rather than a (non-empty) default value.
  // Likewise, if the current value is a fixed size arena string with contents,
  // it will be initialized into an empty mutable arena string.
  std::string* MutableNoCopy(Arena* arena);

  // Basic accessors.
  PROTOBUF_NDEBUG_INLINE const std::string& Get() const {
    // Unconditionally mask away the tag.
    return *tagged_ptr_.Get();
  }

  // Returns a pointer to the stored contents for this instance.
  // This method is for internal debugging and tracking purposes only.
  PROTOBUF_NDEBUG_INLINE const std::string* UnsafeGetPointer() const
      ABSL_ATTRIBUTE_RETURNS_NONNULL {
    return tagged_ptr_.Get();
  }

  // Release returns a std::string* instance that is heap-allocated and is not
  // Own()'d by any arena. If the field is not set, this returns nullptr. The
  // caller retains ownership. Clears this field back to the default state.
  // Used to implement release_<field>() methods on generated classes.
  [[nodiscard]] std::string* Release();

  // Takes a std::string that is heap-allocated, and takes ownership. The
  // std::string's destructor is registered with the arena. Used to implement
  // set_allocated_<field> in generated classes.
  void SetAllocated(std::string* value, Arena* arena);

  // Frees storage (if not on an arena).
  void Destroy();

  // Clears content, but keeps allocated std::string, to avoid the overhead of
  // heap operations. After this returns, the content (as seen by the user) will
  // always be the empty std::string. Assumes that |default_value| is an empty
  // std::string.
  void ClearToEmpty();

  // Clears content, assuming that the current value is not the empty
  // string default.
  void ClearNonDefaultToEmpty();

  // Clears content, but keeps allocated std::string if arena != nullptr, to
  // avoid the overhead of heap operations. After this returns, the content
  // (as seen by the user) will always be equal to |default_value|.
  void ClearToDefault(const LazyString& default_value, ::google::protobuf::Arena* arena);

  // Swaps internal pointers. Arena-safety semantics: this is guarded by the
  // logic in Swap()/UnsafeArenaSwap() at the message level, so this method is
  // 'unsafe' if called directly.
  PROTOBUF_NDEBUG_INLINE static void InternalSwap(ArenaStringPtr* rhs,
                                                  ArenaStringPtr* lhs,
                                                  Arena* arena);

  // Internal setter used only at parse time to directly set a donated string
  // value.
  void UnsafeSetTaggedPointer(TaggedStringPtr value) { tagged_ptr_ = value; }
  // Generated code only! An optimization, in certain cases the generated
  // code is certain we can obtain a std::string with no default checks and
  // tag tests.
  std::string* UnsafeMutablePointer() ABSL_ATTRIBUTE_RETURNS_NONNULL;

  // Returns true if this instances holds an immutable default value.
  inline bool IsDefault() const { return tagged_ptr_.IsDefault(); }

 private:
  template <typename... Args>
  inline std::string* NewString(Arena* arena, Args&&... args) {
    if (arena == nullptr) {
      auto* s = new std::string(std::forward<Args>(args)...);
      return tagged_ptr_.SetAllocated(s);
    } else {
      auto* s = Arena::Create<std::string>(arena, std::forward<Args>(args)...);
      return tagged_ptr_.SetMutableArena(s);
    }
  }

  TaggedStringPtr tagged_ptr_;

  bool IsFixedSizeArena() const { return false; }

  // Swaps tagged pointer without debug hardening. This is to allow python
  // protobuf to maintain pointer stability even in DEBUG builds.
  PROTOBUF_NDEBUG_INLINE static void UnsafeShallowSwap(ArenaStringPtr* rhs,
                                                       ArenaStringPtr* lhs) {
    std::swap(lhs->tagged_ptr_, rhs->tagged_ptr_);
  }

  friend class ::google::protobuf::internal::SwapFieldHelper;
  friend class TcParser;

  // Slow paths.

  // MutableSlow requires that !IsString() || IsDefault
  // Variadic to support 0 args for empty default and 1 arg for LazyString.
  template <typename... Lazy>
  std::string* MutableSlow(::google::protobuf::Arena* arena, const Lazy&... lazy_default);

  friend class EpsCopyInputStream;
};

inline TaggedStringPtr TaggedStringPtr::Copy(Arena* arena) const {
  if (DebugHardenForceCopyDefaultString()) {
    // Harden by forcing an allocated string value.
    return IsNull() ? *this : ForceCopy(arena);
  }
  return IsDefault() ? *this : ForceCopy(arena);
}

inline TaggedStringPtr TaggedStringPtr::Copy(
    Arena* arena, const LazyString& default_value) const {
  if (DebugHardenForceCopyDefaultString()) {
    // Harden by forcing an allocated string value.
    TaggedStringPtr hardened(*this);
    if (IsDefault()) {
      hardened.SetDefault(&default_value.get());
    }
    return hardened.ForceCopy(arena);
  }
  return IsDefault() ? *this : ForceCopy(arena);
}

inline void ArenaStringPtr::InitDefault() {
  tagged_ptr_ = TaggedStringPtr(&fixed_address_empty_string);
}

inline void ArenaStringPtr::InitExternal(const std::string* str) {
  tagged_ptr_.SetDefault(str);
}

inline void ArenaStringPtr::InitAllocated(std::string* str, Arena* arena) {
  if (arena != nullptr) {
    tagged_ptr_.SetMutableArena(str);
    arena->Own(str);
  } else {
    tagged_ptr_.SetAllocated(str);
  }
}

inline void ArenaStringPtr::Set(const char* s, Arena* arena) {
  Set(absl::string_view{s}, arena);
}

inline void ArenaStringPtr::Set(const char* s, size_t n, Arena* arena) {
  Set(absl::string_view{s, n}, arena);
}

inline void ArenaStringPtr::SetBytes(absl::string_view value, Arena* arena) {
  Set(value, arena);
}

template <>
PROTOBUF_EXPORT void ArenaStringPtr::Set(const std::string& value,
                                         Arena* arena);

template <>
inline void ArenaStringPtr::SetBytes(const std::string& value, Arena* arena) {
  Set(value, arena);
}

inline void ArenaStringPtr::SetBytes(std::string&& value, Arena* arena) {
  Set(std::move(value), arena);
}

inline void ArenaStringPtr::SetBytes(const char* s, Arena* arena) {
  Set(s, arena);
}

inline void ArenaStringPtr::SetBytes(const void* p, size_t n, Arena* arena) {
  Set(absl::string_view{static_cast<const char*>(p), n}, arena);
}

PROTOBUF_NDEBUG_INLINE void ArenaStringPtr::InternalSwap(ArenaStringPtr* rhs,
                                                         ArenaStringPtr* lhs,
                                                         Arena* arena) {
  // Silence unused variable warnings in release buildls.
  (void)arena;
  std::swap(lhs->tagged_ptr_, rhs->tagged_ptr_);
  if (internal::DebugHardenForceCopyInSwap()) {
    for (auto* p : {lhs, rhs}) {
      if (p->IsDefault()) continue;
      std::string* old_value = p->tagged_ptr_.Get();
      std::string* new_value =
          p->IsFixedSizeArena()
              ? Arena::Create<std::string>(arena, *old_value)
              : Arena::Create<std::string>(arena, std::move(*old_value));
      if (arena == nullptr) {
        delete old_value;
        p->tagged_ptr_.SetAllocated(new_value);
      } else {
        p->tagged_ptr_.SetMutableArena(new_value);
      }
    }
  }
}

inline void ArenaStringPtr::ClearNonDefaultToEmpty() {
  // Unconditionally mask away the tag.
  ABSL_DCHECK(!tagged_ptr_.IsDefault());
  tagged_ptr_.Get()->clear();
}

inline std::string* ArenaStringPtr::UnsafeMutablePointer() {
  ABSL_DCHECK(tagged_ptr_.IsMutable());
  ABSL_DCHECK(tagged_ptr_.Get() != nullptr);
  return tagged_ptr_.Get();
}


}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_ARENASTRING_H__
