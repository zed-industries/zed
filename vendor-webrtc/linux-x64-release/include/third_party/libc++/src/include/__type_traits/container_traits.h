// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___TYPE_TRAITS_CONTAINER_TRAITS_H
#define _LIBCPP___TYPE_TRAITS_CONTAINER_TRAITS_H

#include <__config>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

// // __container_traits is a general purpose utility containing traits describing various containers operations.
// It currently only has one trait: `__emplacement_has_strong_exception_safety_guarantee`, but it's
// intended to be extended in the future.
//
// These traits should only be used for optimization or QoI purposes. In particular, since this is a libc++ internal
// mechanism, no user-defined containers should be expected to specialize these traits (in fact it would be illegal for
// them to do so). Hence, when using these traits to implement something, make sure that a container that fails to
// specialize these traits does not result in non-conforming code.
//
// When a trait is nonsensical for a type, this class still provides a fallback value for that trait.
// For example, `std::array` does not support `insert` or `emplace`, so
// `__emplacement_has_strong_exception_safety_guarantee` is false for such types.
template <class _Container>
struct __container_traits {
  // A trait that tells whether a single element insertion/emplacement via member function
  // `insert(...)` or `emplace(...)` has strong exception guarantee, that is, if the function
  // exits via an exception, the original container is unaffected
  static _LIBCPP_CONSTEXPR const bool __emplacement_has_strong_exception_safety_guarantee = false;
};

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___TYPE_TRAITS_CONTAINER_TRAITS_H
