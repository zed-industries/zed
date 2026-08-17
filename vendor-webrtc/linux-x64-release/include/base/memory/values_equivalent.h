// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_VALUES_EQUIVALENT_H_
#define BASE_MEMORY_VALUES_EQUIVALENT_H_

#include <functional>
#include <memory>
#include <type_traits>

#include "base/memory/scoped_refptr.h"

namespace base {

namespace internal {
template <typename T>
concept IsPointer = std::is_pointer_v<T>;
}  // namespace internal

// Compares two pointers for equality, returns the dereferenced value comparison
// if both are non-null.
// Behaves like std::optional<T>::operator==(const std::optional<T>&) but for
// pointers, with an optional predicate.
// If `p` is specified, `p(const T& x, const T& y)` should return whether `x`
// and `y` are equal. It's called with `(*a, *b)` when `a != b && a && b`.
template <typename T, typename Predicate = std::equal_to<>>
bool ValuesEquivalent(const T* a, const T* b, Predicate p = {}) {
  if (a == b) {
    return true;
  }
  if (!a || !b) {
    return false;
  }
  return p(*a, *b);
}

// Specialize for smart pointers like std::unique_ptr and base::scoped_refptr
// that provide a T* get() method.
// Example usage:
//   struct Example {
//     std::unique_ptr<Child> child;
//     bool operator==(const Example& other) const {
//       return base::ValuesEquivalent(child, other.child);
//     }
//   };
template <typename T, typename Predicate = std::equal_to<>>
  requires requires(const T& t) {
    { t.get() } -> internal::IsPointer;
  }
bool ValuesEquivalent(const T& x, const T& y, Predicate p = {}) {
  return ValuesEquivalent(x.get(), y.get(), std::move(p));
}

// Specialize for smart pointers like blink::Persistent and blink::Member that
// provide a T* Get() method.
// Example usage:
//   namespace blink {
//   struct Example : public GarbageCollected<Example> {
//     Member<Child> child;
//     bool operator==(const Example& other) const {
//       return base::ValuesEquivalent(child, other.child);
//     }
//     void Trace(Visitor*) const;
//   };
//   }  // namespace blink
template <typename T, typename Predicate = std::equal_to<>>
  requires requires(const T& t) {
    { t.Get() } -> internal::IsPointer;
  }
bool ValuesEquivalent(const T& x, const T& y, Predicate p = {}) {
  return ValuesEquivalent(x.Get(), y.Get(), std::move(p));
}

}  // namespace base

#endif  // BASE_MEMORY_VALUES_EQUIVALENT_H_
