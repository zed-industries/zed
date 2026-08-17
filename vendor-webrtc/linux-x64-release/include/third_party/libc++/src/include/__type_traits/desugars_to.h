//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___TYPE_TRAITS_DESUGARS_TO_H
#define _LIBCPP___TYPE_TRAITS_DESUGARS_TO_H

#include <__config>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD

// Tags to represent the canonical operations.

// syntactically, the operation is equivalent to calling `a == b`
struct __equal_tag {};

// syntactically, the operation is equivalent to calling `a + b`
struct __plus_tag {};

// syntactically, the operation is equivalent to calling `a < b`
struct __less_tag {};

// syntactically, the operation is equivalent to calling `a > b`
struct __greater_tag {};

// syntactically, the operation is equivalent to calling `a < b`, and these expressions
// have to be true for any `a` and `b`:
// - `(a < b) == (b > a)`
// - `(!(a < b) && !(b < a)) == (a == b)`
// For example, this is satisfied for std::less on integral types, but also for ranges::less on all types due to
// additional semantic requirements on that operation.
struct __totally_ordered_less_tag {};

// This class template is used to determine whether an operation "desugars"
// (or boils down) to a given canonical operation.
//
// For example, `std::equal_to<>`, our internal `std::__equal_to` helper and
// `ranges::equal_to` are all just fancy ways of representing a transparent
// equality operation, so they all desugar to `__equal_tag`.
//
// This is useful to optimize some functions in cases where we know e.g. the
// predicate being passed is actually going to call a builtin operator, or has
// some specific semantics.
template <class _CanonicalTag, class _Operation, class... _Args>
inline const bool __desugars_to_v = false;

// For the purpose of determining whether something desugars to something else,
// we disregard const and ref qualifiers on the operation itself.
template <class _CanonicalTag, class _Operation, class... _Args>
inline const bool __desugars_to_v<_CanonicalTag, _Operation const, _Args...> =
    __desugars_to_v<_CanonicalTag, _Operation, _Args...>;
template <class _CanonicalTag, class _Operation, class... _Args>
inline const bool __desugars_to_v<_CanonicalTag, _Operation&, _Args...> =
    __desugars_to_v<_CanonicalTag, _Operation, _Args...>;
template <class _CanonicalTag, class _Operation, class... _Args>
inline const bool __desugars_to_v<_CanonicalTag, _Operation&&, _Args...> =
    __desugars_to_v<_CanonicalTag, _Operation, _Args...>;

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___TYPE_TRAITS_DESUGARS_TO_H
