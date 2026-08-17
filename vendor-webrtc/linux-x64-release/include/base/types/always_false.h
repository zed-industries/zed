// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TYPES_ALWAYS_FALSE_H_
#define BASE_TYPES_ALWAYS_FALSE_H_

namespace base {

// A helper that can be used with a static_assert() that must always fail (e.g.
// for an undesirable template instantiation). Such a static_assert() cannot
// simply be written as static_assert(false, ...) because that would always fail
// to compile, even if the template was never instantiated. Instead, a common
// idiom is to force the static_assert() to depend on a template parameter so
// that it is only evaluated when the template is instantiated:
//
// template <typename U = T>
// void SomeDangerousMethodThatShouldNeverCompile() {
//   static_assert(base::AlwaysFalse<U>, "explanatory message here");
// }
//
//
// The issue of not being able to use static_assert(false, ...) in a
// non-instantiated template was fixed in C++23. When Chromium switches to
// building with C++23, remove this file and use false directly, and search
// across the Chromium codebase for "AlwaysFalse", as there are other
// implementations in places that cannot depend on this file.
//
// References:
// - https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2022/p2593r0.html
// - https://github.com/cplusplus/papers/issues/1251

namespace internal {

template <typename... Args>
struct AlwaysFalseHelper {
  static constexpr bool kValue = false;
};

}  // namespace internal

template <typename... Args>
inline constexpr bool AlwaysFalse =
    internal::AlwaysFalseHelper<Args...>::kValue;

}  // namespace base

#endif  // BASE_TYPES_ALWAYS_FALSE_H_
