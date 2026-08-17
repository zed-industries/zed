// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_CASTING_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_CASTING_H_

#include <concepts>
#include <type_traits>

#include "third_party/blink/renderer/platform/wtf/assertions.h"

namespace blink {

// Helpers for downcasting in a class hierarchy.
//
//   IsA<T>(x): returns true if |x| can be safely downcast to T*. Usage of this
//       should not be common; if it is paired with a call to To<T>, consider
//       using DynamicTo<T> instead (see below). Note that this also returns
//       false if |x| is nullptr.
//
//   To<T>(x): unconditionally downcasts and returns |x| as a T*. CHECKs if the
//       downcast is unsafe. Use when IsA<T>(x) is known to be true due to
//       external invariants and not on a performance sensitive path.
//       If |x| is nullptr, returns nullptr.
//
//   DynamicTo<T>(x): downcasts and returns |x| as a T* iff IsA<T>(x) is true,
//       and nullptr otherwise. This is useful for combining a conditional
//       branch on IsA<T>(x) and an invocation of To<T>(x), e.g.:
//           if (IsA<DerivedClass>(x))
//             To<DerivedClass>(x)->...
//       can be written:
//           if (auto* derived = DynamicTo<DerivedClass>(x))
//             derived->...;
//
//   UnsafeTo<T>(x): unconditionally downcasts and returns |x| as a T*. DCHECKs
//       if the downcast is unsafe. Use when IsA<T>(x) is known to be true due
//       to external invariants. Prefer To<T> over this method, but this is ok
//       to use in performance sensitive code. If |x| is nullptr, returns
//       nullptr.
//
// Marking downcasts as safe is done by specializing the DowncastTraits
// template:
//
// template <>
// struct DowncastTraits<DerivedClass> {
//   static bool AllowFrom(const BaseClass& b) {
//     return b.IsDerivedClass();
//   }
//   static bool AllowFrom(const AnotherBaseClass& b) {
//     return b.type() == AnotherBaseClass::kDerivedClassTag;
//   }
// };
//
// int main() {
//   BaseClass* base = CreateDerived();
//   AnotherBaseClass* another_base = CreateDerived();
//   UnrelatedClass* unrelated = CreateUnrelated();
//
//   std::cout << std::boolalpha;
//   std::cout << IsA<Derived>(base) << '\n';          // prints true
//   std::cout << IsA<Derived>(another_base) << '\n';  // prints true
//   std::cout << IsA<Derived>(unrelated) << '\n';     // prints false
// }
template <typename Derived>
struct DowncastTraits;

namespace internal {

template <typename Derived, typename Base>
struct DowncastTraitsHelper {
  static_assert(sizeof(Derived) == 0,
                "Unknown type, this error typically means you need to include "
                "the header of the type being cast to.");
};

template <typename Derived, typename Base>
  requires(!std::is_base_of_v<Derived, Base>)
struct DowncastTraitsHelper<Derived, Base> {
  static bool AllowFrom(const Base& from) {
    return DowncastTraits<Derived>::AllowFrom(from);
  }
};

// If Derived is actually a base class of Base, unconditionally return true to
// skip the type checks.
template <typename Derived, typename Base>
  requires(std::is_base_of_v<Derived, Base>)
struct DowncastTraitsHelper<Derived, Base> {
  static bool AllowFrom(const Base&) { return true; }
};

}  // namespace internal

// Returns true iff the conversion from Base to Derived is allowed. For the
// pointer overloads, returns false if the input pointer is nullptr.
template <typename Derived, typename Base>
bool IsA(const Base& from) {
  static_assert(std::is_base_of_v<Base, Derived>, "Unnecessary type check");
  return internal::DowncastTraitsHelper<Derived, const Base>::AllowFrom(from);
}

template <typename Derived, typename Base>
bool IsA(const Base* from) {
  static_assert(std::is_base_of_v<Base, Derived>, "Unnecessary type check");
  return from && IsA<Derived>(*from);
}

template <typename Derived, typename Base>
bool IsA(Base& from) {
  static_assert(std::is_base_of_v<Base, Derived>, "Unnecessary type check");
  return internal::DowncastTraitsHelper<Derived, const Base>::AllowFrom(
      const_cast<const Base&>(from));
}

template <typename Derived, typename Base>
bool IsA(Base* from) {
  static_assert(std::is_base_of_v<Base, Derived>, "Unnecessary type check");
  return from && IsA<Derived>(*from);
}

// Unconditionally downcasts from Base to Derived. Internally, this asserts that
// |from| is a Derived to help catch bad casts. For the pointer overloads,
// returns nullptr if the input pointer is nullptr.
template <typename Derived, typename Base>
const Derived& To(const Base& from) {
  CHECK(IsA<Derived>(from));
  return static_cast<const Derived&>(from);
}

template <typename Derived, typename Base>
const Derived* To(const Base* from) {
  return from ? &To<Derived>(*from) : nullptr;
}

template <typename Derived, typename Base>
Derived& To(Base& from) {
  CHECK(IsA<Derived>(from));
  return static_cast<Derived&>(from);
}
template <typename Derived, typename Base>
Derived* To(Base* from) {
  return from ? &To<Derived>(*from) : nullptr;
}

// Safely downcasts from Base to Derived. If |from| is not a Derived, returns
// nullptr; otherwise, downcasts from Base to Derived. For the pointer
// overloads, returns nullptr if the input pointer is nullptr.
template <typename Derived, typename Base>
const Derived* DynamicTo(const Base* from) {
  // TOOD(https://crbug.com/1449302): Figure out why IsA<T> + To<T> does not
  // optimize correctly.
  return IsA<Derived>(from) ? static_cast<const Derived*>(from) : nullptr;
}

template <typename Derived, typename Base>
const Derived* DynamicTo(const Base& from) {
  // TOOD(https://crbug.com/1449302): Figure out why IsA<T> + To<T> does not
  // optimize correctly.
  return IsA<Derived>(from) ? &static_cast<const Derived&>(from) : nullptr;
}

template <typename Derived, typename Base>
Derived* DynamicTo(Base* from) {
  // TOOD(https://crbug.com/1449302): Figure out why IsA<T> + To<T> does not
  // optimize correctly.
  return IsA<Derived>(from) ? static_cast<Derived*>(from) : nullptr;
}

template <typename Derived, typename Base>
Derived* DynamicTo(Base& from) {
  // TOOD(https://crbug.com/1449302): Figure out why IsA<T> + To<T> does not
  // optimize correctly.
  return IsA<Derived>(from) ? &static_cast<Derived&>(from) : nullptr;
}

// Unconditionally downcasts from Base to Derived. Internally, this asserts
// that |from| is a Derived to help catch bad casts in testing/fuzzing. For the
// pointer overloads, returns nullptr if the input pointer is nullptr.
template <typename Derived, typename Base>
const Derived& UnsafeTo(const Base& from) {
  SECURITY_DCHECK(IsA<Derived>(from));
  return static_cast<const Derived&>(from);
}

template <typename Derived, typename Base>
const Derived* UnsafeTo(const Base* from) {
  return from ? &UnsafeTo<Derived>(*from) : nullptr;
}

template <typename Derived, typename Base>
Derived& UnsafeTo(Base& from) {
  SECURITY_DCHECK(IsA<Derived>(from));
  return static_cast<Derived&>(from);
}
template <typename Derived, typename Base>
Derived* UnsafeTo(Base* from) {
  return from ? &UnsafeTo<Derived>(*from) : nullptr;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_CASTING_H_
