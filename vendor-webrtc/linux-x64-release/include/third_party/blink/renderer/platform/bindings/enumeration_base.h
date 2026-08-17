// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_ENUMERATION_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_ENUMERATION_BASE_H_

#include <type_traits>

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8.h"

namespace blink {

namespace bindings {

class PLATFORM_EXPORT EnumerationBase {
  DISALLOW_NEW();

 public:
  ~EnumerationBase() = default;

  // Returns the IDL enumeration value as a string.
  // https://webidl.spec.whatwg.org/#dfn-enumeration-value
  const char* AsCStr() const { return string_literal_; }
  String AsString() const { return string_literal_; }
  AtomicString AsAtomicString() const { return AtomicString(string_literal_); }

  // Returns the string representation to be used by CHECK_OP family.
  // This member function is meant only for CHECK_EQ, etc.
  String ToString() const {
    return String::Format("IDL enum value \"%s\"", string_literal_);
  }

  // Returns true if the value is invalid.  The instance in this state must be
  // created only when an exception is thrown.
  bool IsEmpty() const { return !string_literal_; }

 protected:
  // Integer type that is convertible from/to enum values defined in subclasses.
  using enum_int_t = size_t;

  constexpr EnumerationBase() = default;
  explicit constexpr EnumerationBase(enum_int_t enum_value,
                                     const char* string_literal)
      : enum_value_(enum_value), string_literal_(string_literal) {}
  constexpr EnumerationBase(const EnumerationBase&) = default;
  constexpr EnumerationBase(EnumerationBase&&) = default;

  EnumerationBase& operator=(const EnumerationBase&) = default;
  EnumerationBase& operator=(EnumerationBase&&) = default;

  enum_int_t GetEnumValue() const { return enum_value_; }

 private:
  // Subclasses are supposed to define a C++ enum class and a table of strings
  // that represent IDL enumeration values.  |enum_value_| is an enum value of
  // the C++ enum class (must be convertible from/to enum_int_t) and
  // |string_literal_| is a pointer to a string in the table.
  enum_int_t enum_value_ = 0;
  const char* string_literal_ = nullptr;
};

}  // namespace bindings

template <typename EnumTypeClass>
typename std::enable_if_t<
    std::is_base_of<bindings::EnumerationBase, EnumTypeClass>::value,
    bool>
operator==(const EnumTypeClass& lhs, const EnumTypeClass& rhs) {
  DCHECK(!lhs.IsEmpty() && !rhs.IsEmpty());
  return lhs.AsEnum() == rhs.AsEnum();
}

template <typename EnumTypeClass>
typename std::enable_if_t<
    std::is_base_of<bindings::EnumerationBase, EnumTypeClass>::value,
    bool>
operator!=(const EnumTypeClass& lhs, const EnumTypeClass& rhs) {
  return !(lhs == rhs);
}

template <typename EnumTypeClass>
typename std::enable_if_t<
    std::is_base_of<bindings::EnumerationBase, EnumTypeClass>::value,
    bool>
operator==(const EnumTypeClass& lhs, typename EnumTypeClass::Enum rhs) {
  DCHECK(!lhs.IsEmpty());
  return lhs.AsEnum() == rhs;
}

template <typename EnumTypeClass>
typename std::enable_if_t<
    std::is_base_of<bindings::EnumerationBase, EnumTypeClass>::value,
    bool>
operator==(typename EnumTypeClass::Enum lhs, const EnumTypeClass& rhs) {
  DCHECK(!rhs.IsEmpty());
  return lhs == rhs.AsEnum();
}

template <typename EnumTypeClass>
typename std::enable_if_t<
    std::is_base_of<bindings::EnumerationBase, EnumTypeClass>::value,
    bool>
operator!=(const EnumTypeClass& lhs, typename EnumTypeClass::Enum rhs) {
  return !(lhs == rhs);
}

template <typename EnumTypeClass>
typename std::enable_if_t<
    std::is_base_of<bindings::EnumerationBase, EnumTypeClass>::value,
    bool>
operator!=(typename EnumTypeClass::Enum lhs, const EnumTypeClass& rhs) {
  return !(lhs == rhs);
}

// Migration adapters
template <typename EnumTypeClass>
typename std::enable_if_t<
    std::is_base_of<bindings::EnumerationBase, EnumTypeClass>::value,
    bool>
operator==(const EnumTypeClass& lhs, const String& rhs) {
  DCHECK(!lhs.IsEmpty());
  return lhs.AsString() == rhs;
}

template <typename EnumTypeClass>
typename std::enable_if_t<
    std::is_base_of<bindings::EnumerationBase, EnumTypeClass>::value,
    bool>
operator!=(const EnumTypeClass& lhs, const String& rhs) {
  return !(lhs == rhs);
}

template <typename EnumTypeClass>
typename std::enable_if_t<
    std::is_base_of<bindings::EnumerationBase, EnumTypeClass>::value,
    bool>
operator==(const EnumTypeClass& lhs, const AtomicString& rhs) {
  DCHECK(!lhs.IsEmpty());
  return lhs.AsString() == rhs;
}

template <typename EnumTypeClass>
typename std::enable_if_t<
    std::is_base_of<bindings::EnumerationBase, EnumTypeClass>::value,
    bool>
operator==(const EnumTypeClass& lhs, const char* rhs) {
  DCHECK(!lhs.IsEmpty());
  return lhs.AsString() == rhs;
}

template <typename EnumTypeClass>
typename std::enable_if_t<
    std::is_base_of<bindings::EnumerationBase, EnumTypeClass>::value,
    bool>
operator==(const char* lhs, const EnumTypeClass& rhs) {
  return rhs == lhs;
}

// TODO(crbug.com/1092328): IDLEnumAsString is an adapter between the old and
// new bindings generators.  The old bindings generator implements IDL
// enumeration with String class, however, the new bindings generator implements
// it with subclasses of EnumerationBase.  IDLEnumAsString resolves this
// difference.
inline const String& IDLEnumAsString(const String& value) {
  return value;
}

inline String IDLEnumAsString(const bindings::EnumerationBase& value) {
  return value.AsString();
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_ENUMERATION_BASE_H_
