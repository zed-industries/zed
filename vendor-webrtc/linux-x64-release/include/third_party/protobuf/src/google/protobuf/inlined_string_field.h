// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_INLINED_STRING_FIELD_H__
#define GOOGLE_PROTOBUF_INLINED_STRING_FIELD_H__

#include <string>
#include <utility>

#include "absl/log/absl_check.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/arenastring.h"
#include "google/protobuf/explicitly_constructed.h"
#include "google/protobuf/message_lite.h"
#include "google/protobuf/port.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

#ifdef SWIG
#error "You cannot SWIG proto headers"
#endif

namespace google {
namespace protobuf {

class Arena;

namespace internal {

// InlinedStringField wraps a std::string instance and exposes an API similar to
// ArenaStringPtr's wrapping of a std::string* instance.
//
// default_value parameters are taken for consistency with ArenaStringPtr, but
// are not used for most methods. With inlining, these should be removed from
// the generated binary.
//
// InlinedStringField has a donating mechanism that allows string buffer
// allocated on arena. A string is donated means both the string container and
// the data buffer are on arena. The donating mechanism here is similar to the
// one in ArenaStringPtr with some differences:
//
// When an InlinedStringField is constructed, the donating state is true. This
// is because the string container is directly stored in the message on the
// arena:
//
//   Construction: donated=true
//   Arena:
//   +-----------------------+
//   |Message foo:           |
//   | +-------------------+ |
//   | |InlinedStringField:| |
//   | | +-----+           | |
//   | | | | | |           | |
//   | | +-----+           | |
//   | +-------------------+ |
//   +-----------------------+
//
// When lvalue Set is called, the donating state is still true. String data will
// be allocated on the arena:
//
//   Lvalue Set: donated=true
//   Arena:
//   +-----------------------+
//   |Message foo:           |
//   | +-------------------+ |
//   | |InlinedStringField:| |
//   | | +-----+           | |
//   | | | | | |           | |
//   | | +|----+           | |
//   | +--|----------------+ |
//   |    V                  |
//   |  +----------------+   |
//   |  |'f','o','o',... |   |
//   |  +----------------+   |
//   +-----------------------+
//
// Some operations will undonate a donated string, including: Mutable,
// SetAllocated, Rvalue Set, and Swap with a non-donated string.
//
// For more details of the donating states transitions, go/pd-inlined-string.
class PROTOBUF_EXPORT InlinedStringField {
 public:
  InlinedStringField() : str_() {}
  InlinedStringField(const InlinedStringField&) = delete;
  InlinedStringField& operator=(const InlinedStringField&) = delete;
#if defined(__cpp_lib_constexpr_string) && __cpp_lib_constexpr_string >= 201907L
  // No need to do dynamic initialization here.
  constexpr void Init() {}
  // Add the dummy parameter just to make InlinedStringField(nullptr)
  // unambiguous.
  constexpr InlinedStringField(
      const ExplicitlyConstructed<std::string>* /*default_value*/,
      bool /*dummy*/)
      : str_{} {}
#else
  inline void Init() { ::new (static_cast<void*>(&str_)) std::string(); }
  // Add the dummy parameter just to make InlinedStringField(nullptr)
  // unambiguous.
  constexpr InlinedStringField(
      const ExplicitlyConstructed<std::string>* /*default_value*/,
      bool /*dummy*/)
      : dummy_{} {}
#endif

  explicit InlinedStringField(const std::string& default_value);
  explicit InlinedStringField(Arena* arena);
  InlinedStringField(Arena* arena, const InlinedStringField& rhs);
  ~InlinedStringField() { Destruct(); }

  // Lvalue Set. To save space, we pack the donating states of multiple
  // InlinedStringFields into an uint32_t `donating_states`. The `mask`
  // indicates the position of the bit for this InlinedStringField. `donated` is
  // whether this field is donated.
  //
  // The caller should guarantee that:
  //
  //   `donated == ((donating_states & ~mask) != 0)`
  //
  // This method never changes the `donating_states`.
  void Set(absl::string_view value, Arena* arena, bool donated,
           uint32_t* donating_states, uint32_t mask, MessageLite* msg);

  // Rvalue Set. If this field is donated, this method will undonate this field
  // by mutating the `donating_states` according to `mask`.
  void Set(std::string&& value, Arena* arena, bool donated,
           uint32_t* donating_states, uint32_t mask, MessageLite* msg);

  void Set(const char* str, ::google::protobuf::Arena* arena, bool donated,
           uint32_t* donating_states, uint32_t mask, MessageLite* msg);

  void Set(const char* str, size_t size, ::google::protobuf::Arena* arena, bool donated,
           uint32_t* donating_states, uint32_t mask, MessageLite* msg);

  template <typename RefWrappedType>
  void Set(std::reference_wrapper<RefWrappedType> const_string_ref,
           ::google::protobuf::Arena* arena, bool donated, uint32_t* donating_states,
           uint32_t mask, MessageLite* msg);

  void SetBytes(absl::string_view value, Arena* arena, bool donated,
                uint32_t* donating_states, uint32_t mask, MessageLite* msg);

  void SetBytes(std::string&& value, Arena* arena, bool donated,
                uint32_t* donating_states, uint32_t mask, MessageLite* msg);

  void SetBytes(const char* str, ::google::protobuf::Arena* arena, bool donated,
                uint32_t* donating_states, uint32_t mask, MessageLite* msg);

  void SetBytes(const void* p, size_t size, ::google::protobuf::Arena* arena,
                bool donated, uint32_t* donating_states, uint32_t mask,
                MessageLite* msg);

  template <typename RefWrappedType>
  void SetBytes(std::reference_wrapper<RefWrappedType> const_string_ref,
                ::google::protobuf::Arena* arena, bool donated, uint32_t* donating_states,
                uint32_t mask, MessageLite* msg);

  PROTOBUF_NDEBUG_INLINE void SetNoArena(absl::string_view value);
  PROTOBUF_NDEBUG_INLINE void SetNoArena(std::string&& value);

  // Basic accessors.
  PROTOBUF_NDEBUG_INLINE const std::string& Get() const { return GetNoArena(); }
  PROTOBUF_NDEBUG_INLINE const std::string& GetNoArena() const;

  // Mutable returns a std::string* instance that is heap-allocated. If this
  // field is donated, this method undonates this field by mutating the
  // `donating_states` according to `mask`, and copies the content of the
  // original string to the returning string.
  std::string* Mutable(Arena* arena, bool donated, uint32_t* donating_states,
                       uint32_t mask, MessageLite* msg);
  std::string* Mutable(const LazyString& default_value, Arena* arena,
                       bool donated, uint32_t* donating_states, uint32_t mask,
                       MessageLite* msg);

  // Mutable(nullptr_t) is an overload to explicitly support Mutable(nullptr)
  // calls used by the internal parser logic. This provides API equivalence with
  // ArenaStringPtr, while still protecting against calls with arena pointers.
  std::string* Mutable(std::nullptr_t);
  std::string* MutableNoCopy(std::nullptr_t);

  // Takes a std::string that is heap-allocated, and takes ownership. The
  // std::string's destructor is registered with the arena. Used to implement
  // set_allocated_<field> in generated classes.
  //
  // If this field is donated, this method undonates this field by mutating the
  // `donating_states` according to `mask`.
  void SetAllocated(const std::string* default_value, std::string* value,
                    Arena* arena, bool donated, uint32_t* donating_states,
                    uint32_t mask, MessageLite* msg);

  void SetAllocatedNoArena(const std::string* default_value,
                           std::string* value);

  // Release returns a std::string* instance that is heap-allocated and is not
  // Own()'d by any arena. If the field is not set, this returns nullptr. The
  // caller retains ownership. Clears this field back to nullptr state. Used to
  // implement release_<field>() methods on generated classes.
  [[nodiscard]] std::string* Release(Arena* arena, bool donated);
  [[nodiscard]] std::string* Release();

  // --------------------------------------------------------
  // Below functions will be removed in subsequent code change
  // --------------------------------------------------------
#ifdef DEPRECATED_METHODS_TO_BE_DELETED
  [[nodiscard]] std::string* Release(const std::string*, Arena* arena,
                                     bool donated) {
    return Release(arena, donated);
  }

  [[nodiscard]] std::string* ReleaseNonDefault(const std::string*,
                                               Arena* arena) {
    return Release();
  }

  std::string* ReleaseNonDefaultNoArena(const std::string* default_value) {
    return Release();
  }

  void Set(const std::string*, absl::string_view value, Arena* arena,
           bool donated, uint32_t* donating_states, uint32_t mask,
           MessageLite* msg) {
    Set(value, arena, donated, donating_states, mask, msg);
  }

  void Set(const std::string*, std::string&& value, Arena* arena, bool donated,
           uint32_t* donating_states, uint32_t mask, MessageLite* msg) {
    Set(std::move(value), arena, donated, donating_states, mask, msg);
  }


  template <typename FirstParam>
  void Set(FirstParam, const char* str, ::google::protobuf::Arena* arena, bool donated,
           uint32_t* donating_states, uint32_t mask, MessageLite* msg) {
    Set(str, arena, donated, donating_states, mask, msg);
  }

  template <typename FirstParam>
  void Set(FirstParam p1, const char* str, size_t size, ::google::protobuf::Arena* arena,
           bool donated, uint32_t* donating_states, uint32_t mask,
           MessageLite* msg) {
    Set(str, size, arena, donated, donating_states, mask, msg);
  }

  template <typename FirstParam, typename RefWrappedType>
  void Set(FirstParam p1,
           std::reference_wrapper<RefWrappedType> const_string_ref,
           ::google::protobuf::Arena* arena, bool donated, uint32_t* donating_states,
           uint32_t mask, MessageLite* msg) {
    Set(const_string_ref, arena, donated, donating_states, mask, msg);
  }

  void SetBytes(const std::string*, absl::string_view value, Arena* arena,
                bool donated, uint32_t* donating_states, uint32_t mask,
                MessageLite* msg) {
    Set(value, arena, donated, donating_states, mask, msg);
  }


  void SetBytes(const std::string*, std::string&& value, Arena* arena,
                bool donated, uint32_t* donating_states, uint32_t mask,
                MessageLite* msg) {
    Set(std::move(value), arena, donated, donating_states, mask, msg);
  }

  template <typename FirstParam>
  void SetBytes(FirstParam p1, const char* str, ::google::protobuf::Arena* arena,
                bool donated, uint32_t* donating_states, uint32_t mask,
                MessageLite* msg) {
    SetBytes(str, arena, donated, donating_states, mask, msg);
  }

  template <typename FirstParam>
  void SetBytes(FirstParam p1, const void* p, size_t size,
                ::google::protobuf::Arena* arena, bool donated, uint32_t* donating_states,
                uint32_t mask, MessageLite* msg) {
    SetBytes(p, size, arena, donated, donating_states, mask, msg);
  }

  template <typename FirstParam, typename RefWrappedType>
  void SetBytes(FirstParam p1,
                std::reference_wrapper<RefWrappedType> const_string_ref,
                ::google::protobuf::Arena* arena, bool donated, uint32_t* donating_states,
                uint32_t mask, MessageLite* msg) {
    SetBytes(const_string_ref.get(), arena, donated, donating_states, mask,
             msg);
  }

  void SetNoArena(const std::string*, absl::string_view value) {
    SetNoArena(value);
  }
  void SetNoArena(const std::string*, std::string&& value) {
    SetNoArena(std::move(value));
  }

  std::string* Mutable(ArenaStringPtr::EmptyDefault, Arena* arena, bool donated,
                       uint32_t* donating_states, uint32_t mask,
                       MessageLite* msg) {
    return Mutable(arena, donated, donating_states, mask, msg);
  }

  PROTOBUF_NDEBUG_INLINE std::string* MutableNoArenaNoDefault(
      const std::string* /*default_value*/) {
    return MutableNoCopy(nullptr);
  }

#endif  // DEPRECATED_METHODS_TO_BE_DELETED

  // Arena-safety semantics: this is guarded by the logic in
  // Swap()/UnsafeArenaSwap() at the message level, so this method is
  // 'unsafe' if called directly.
  PROTOBUF_NDEBUG_INLINE static void InternalSwap(
      InlinedStringField* lhs, bool lhs_arena_dtor_registered,
      MessageLite* lhs_msg,  //
      InlinedStringField* rhs, bool rhs_arena_dtor_registered,
      MessageLite* rhs_msg, Arena* arena);

  // Frees storage (if not on an arena).
  PROTOBUF_NDEBUG_INLINE void Destroy(const std::string* default_value,
                                      Arena* arena) {
    if (arena == nullptr) {
      DestroyNoArena(default_value);
    }
  }
  PROTOBUF_NDEBUG_INLINE void DestroyNoArena(const std::string* default_value);

  // Clears content, but keeps allocated std::string, to avoid the overhead of
  // heap operations. After this returns, the content (as seen by the user) will
  // always be the empty std::string.
  PROTOBUF_NDEBUG_INLINE void ClearToEmpty() { ClearNonDefaultToEmpty(); }
  PROTOBUF_NDEBUG_INLINE void ClearNonDefaultToEmpty() {
    get_mutable()->clear();
  }

  // Clears content, but keeps allocated std::string if arena != nullptr, to
  // avoid the overhead of heap operations. After this returns, the content (as
  // seen by the user) will always be equal to |default_value|.
  void ClearToDefault(const LazyString& default_value, Arena* arena,
                      bool donated);

  // Generated code / reflection only! Returns a mutable pointer to the string.
  PROTOBUF_NDEBUG_INLINE std::string* UnsafeMutablePointer();

  // InlinedStringField doesn't have things like the `default_value` pointer in
  // ArenaStringPtr.
  static constexpr bool IsDefault() { return false; }
  static constexpr bool IsDefault(const std::string*) { return false; }

 private:
  // ScopedCheckInvariants checks all string in-variants at destruction.
  class ScopedCheckInvariants;

  void Destruct() { get_mutable()->~basic_string(); }

  PROTOBUF_NDEBUG_INLINE std::string* get_mutable();
  PROTOBUF_NDEBUG_INLINE const std::string* get_const() const;

  union {
    std::string str_;
    char dummy_;
  };

  std::string* MutableSlow(::google::protobuf::Arena* arena, bool donated,
                           uint32_t* donating_states, uint32_t mask,
                           MessageLite* msg);


  // When constructed in an Arena, we want our destructor to be skipped.
  friend class ::google::protobuf::Arena;
  typedef void InternalArenaConstructable_;
  typedef void DestructorSkippable_;
};

inline std::string* InlinedStringField::get_mutable() { return &str_; }

inline const std::string* InlinedStringField::get_const() const {
  return &str_;
}

inline InlinedStringField::InlinedStringField(
    const std::string& default_value) {
  new (get_mutable()) std::string(default_value);
}


#ifdef GOOGLE_PROTOBUF_INTERNAL_DONATE_STEAL_INLINE
constexpr uint32_t InitDonatingStates() { return ~0u; }
inline void InternalRegisterArenaDtor(Arena*, void*, void (*)(void*)) {}
#else   // !GOOGLE_PROTOBUF_INTERNAL_DONATE_STEAL_INLINE
constexpr uint32_t InitDonatingStates() { return 0u; }
inline void InternalRegisterArenaDtor(Arena* arena, void* object,
                                      void (*destruct)(void*)) {
  if (arena != nullptr) {
    arena->OwnCustomDestructor(object, destruct);
  }
}
#endif  // GOOGLE_PROTOBUF_INTERNAL_DONATE_STEAL_INLINE

inline InlinedStringField::InlinedStringField(Arena* /*arena*/) : str_() {}

inline InlinedStringField::InlinedStringField(Arena* arena,
                                              const InlinedStringField& rhs) {
  const std::string& src = *rhs.get_const();
  ::new (static_cast<void*>(&str_)) std::string(src);
}

inline const std::string& InlinedStringField::GetNoArena() const {
  return *get_const();
}

inline void InlinedStringField::SetAllocatedNoArena(
    const std::string* /*default_value*/, std::string* value) {
  if (value == nullptr) {
    // Currently, inlined string field can't have non empty default.
    get_mutable()->clear();
  } else {
    get_mutable()->assign(std::move(*value));
    delete value;
  }
}

inline void InlinedStringField::DestroyNoArena(const std::string*) {
  // This is invoked from the generated message's ArenaDtor, which is used to
  // clean up objects not allocated on the Arena.
  this->~InlinedStringField();
}

inline void InlinedStringField::SetNoArena(absl::string_view value) {
  get_mutable()->assign(value.data(), value.length());
}

inline void InlinedStringField::SetNoArena(std::string&& value) {
  get_mutable()->assign(std::move(value));
}

PROTOBUF_NDEBUG_INLINE void InlinedStringField::InternalSwap(
    InlinedStringField* lhs, bool lhs_arena_dtor_registered,
    MessageLite* lhs_msg,  //
    InlinedStringField* rhs, bool rhs_arena_dtor_registered,
    MessageLite* rhs_msg, Arena* arena) {
#ifdef GOOGLE_PROTOBUF_INTERNAL_DONATE_STEAL_INLINE
  lhs->get_mutable()->swap(*rhs->get_mutable());
  if (!lhs_arena_dtor_registered && rhs_arena_dtor_registered) {
    lhs_msg->OnDemandRegisterArenaDtor(arena);
  } else if (lhs_arena_dtor_registered && !rhs_arena_dtor_registered) {
    rhs_msg->OnDemandRegisterArenaDtor(arena);
  }
#else
  (void)arena;
  (void)lhs_arena_dtor_registered;
  (void)rhs_arena_dtor_registered;
  (void)lhs_msg;
  (void)rhs_msg;
  lhs->get_mutable()->swap(*rhs->get_mutable());
#endif
}

inline void InlinedStringField::Set(absl::string_view value, Arena* arena,
                                    bool donated, uint32_t* /*donating_states*/,
                                    uint32_t /*mask*/, MessageLite* /*msg*/) {
  (void)arena;
  (void)donated;
  SetNoArena(value);
}

inline void InlinedStringField::Set(const char* str, ::google::protobuf::Arena* arena,
                                    bool donated, uint32_t* donating_states,
                                    uint32_t mask, MessageLite* msg) {
  Set(absl::string_view(str), arena, donated, donating_states, mask, msg);
}

inline void InlinedStringField::Set(const char* str, size_t size,
                                    ::google::protobuf::Arena* arena, bool donated,
                                    uint32_t* donating_states, uint32_t mask,
                                    MessageLite* msg) {
  Set(absl::string_view{str, size}, arena, donated, donating_states, mask, msg);
}

inline void InlinedStringField::SetBytes(absl::string_view value, Arena* arena,
                                         bool donated,
                                         uint32_t* donating_states,
                                         uint32_t mask, MessageLite* msg) {
  Set(value, arena, donated, donating_states, mask, msg);
}

inline void InlinedStringField::SetBytes(std::string&& value, Arena* arena,
                                         bool donated,
                                         uint32_t* donating_states,
                                         uint32_t mask, MessageLite* msg) {
  Set(std::move(value), arena, donated, donating_states, mask, msg);
}

inline void InlinedStringField::SetBytes(const char* str,
                                         ::google::protobuf::Arena* arena, bool donated,
                                         uint32_t* donating_states,
                                         uint32_t mask, MessageLite* msg) {
  Set(str, arena, donated, donating_states, mask, msg);
}

inline void InlinedStringField::SetBytes(const void* p, size_t size,
                                         ::google::protobuf::Arena* arena, bool donated,
                                         uint32_t* donating_states,
                                         uint32_t mask, MessageLite* msg) {
  Set(static_cast<const char*>(p), size, arena, donated, donating_states, mask,
      msg);
}

template <typename RefWrappedType>
inline void InlinedStringField::Set(
    std::reference_wrapper<RefWrappedType> const_string_ref,
    ::google::protobuf::Arena* arena, bool donated, uint32_t* donating_states,
    uint32_t mask, MessageLite* msg) {
  Set(const_string_ref.get(), arena, donated, donating_states, mask, msg);
}

template <typename RefWrappedType>
inline void InlinedStringField::SetBytes(
    std::reference_wrapper<RefWrappedType> const_string_ref,
    ::google::protobuf::Arena* arena, bool donated, uint32_t* donating_states,
    uint32_t mask, MessageLite* msg) {
  Set(const_string_ref.get(), arena, donated, donating_states, mask, msg);
}

inline std::string* InlinedStringField::UnsafeMutablePointer() {
  return get_mutable();
}

inline std::string* InlinedStringField::Mutable(std::nullptr_t) {
  return get_mutable();
}

inline std::string* InlinedStringField::MutableNoCopy(std::nullptr_t) {
  return get_mutable();
}

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_INLINED_STRING_FIELD_H__
