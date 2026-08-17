// Copyright (C) 2021 Klarälvdalens Datakonsult AB, a KDAB Group company, info@kdab.com, author Marc Mutz <marc.mutz@kdab.com>
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only
#ifndef Q20ALGORITHM_H
#define Q20ALGORITHM_H

#include <QtCore/qglobal.h>

#include <algorithm>
#include <QtCore/q20functional.h>

//
//  W A R N I N G
//  -------------
//
// This file is not part of the Qt API. Types and functions defined in this
// file can reliably be replaced by their std counterparts, once available.
// You may use these definitions in your own code, but be aware that we
// will remove them once Qt depends on the C++ version that supports
// them in namespace std. There will be NO deprecation warning, the
// definitions will JUST go away.
//
// If you can't agree to these terms, don't use these definitions!
//
// We mean it.
//

QT_BEGIN_NAMESPACE

namespace q20 {
// like std::<algorithm> (ie. not ranged, but constexpr)
#ifdef __cpp_lib_constexpr_algorithms
using std::copy;
using std::copy_if;
using std::copy_n;
using std::fill;
using std::fill_n;
using std::is_sorted_until;
using std::is_sorted;
using std::transform;
#else
template <typename InputIterator, typename OutputIterator>
constexpr OutputIterator
copy(InputIterator first, InputIterator last, OutputIterator dest)
{
    while (first != last) {
        *dest = *first;
        ++first;
        ++dest;
    }
    return dest;
}

template <typename InputIterator, typename OutputIterator, typename UnaryPredicate>
constexpr OutputIterator
copy_if(InputIterator first, InputIterator last, OutputIterator dest, UnaryPredicate pred)
{
    while (first != last) {
        if (pred(*first)) {
            *dest = *first;
            ++dest;
        }
        ++first;
    }
    return dest;
}

template <typename InputIterator, typename Size, typename OutputIterator>
constexpr OutputIterator
copy_n(InputIterator first, Size n, OutputIterator dest)
{
    while (n > Size{0}) {
        *dest = *first;
        ++first;
        ++dest;
        --n;
    }
    return dest;
}

template <typename ForwardIterator, typename Value>
constexpr void
fill(ForwardIterator first, ForwardIterator last, const Value &value)
{
    while (first != last) {
        *first = value;
        ++first;
    }
}

template <typename OutputIterator, typename Size, typename Value>
constexpr OutputIterator
fill_n(OutputIterator first, Size n, const Value &value)
{
    while (n > Size{0}) {
        *first = value;
        ++first;
        --n;
    }
    return first;
}

template <typename ForwardIterator, typename BinaryPredicate = std::less<>>
constexpr ForwardIterator
is_sorted_until(ForwardIterator first, ForwardIterator last, BinaryPredicate p = {})
{
    if (first == last)
        return first;
    auto prev = first;
    while (++first != last) {
        if (p(*first, *prev))
            return first;
        prev = first;
    }
    return first;
}

template <typename ForwardIterator, typename BinaryPredicate = std::less<>>
constexpr bool is_sorted(ForwardIterator first, ForwardIterator last, BinaryPredicate p = {})
{
    return q20::is_sorted_until(first, last, p) == last;
}

template <typename InputIterator, typename OutputIterator, typename UnaryFunction>
constexpr OutputIterator
transform(InputIterator first, InputIterator last, OutputIterator dest, UnaryFunction op)
{
    while (first != last) {
        *dest = op(*first);
        ++first;
        ++dest;
    }
    return dest;
}

// binary transform missing on purpose (no users)

#endif
}

namespace q20::ranges {
// like std::ranges::{any,all,none}_of, just unconstrained, so no range-overload
#ifdef __cpp_lib_ranges
using std::ranges::any_of;
using std::ranges::all_of;
using std::ranges::none_of;
#else
[[maybe_unused]] inline constexpr struct { // Niebloid
    template <typename InputIterator, typename Sentinel,
              typename Predicate, typename Projection = q20::identity>
    constexpr bool operator()(InputIterator first, Sentinel last, Predicate pred, Projection proj = {}) const
    {
        while (first != last) {
            if (std::invoke(pred, std::invoke(proj, *first)))
                return true;
            ++first;
        }
        return false;
    }
} any_of;
[[maybe_unused]] inline constexpr struct { // Niebloid
    template <typename InputIterator, typename Sentinel,
              typename Predicate, typename Projection = q20::identity>
    constexpr bool operator()(InputIterator first, Sentinel last, Predicate pred, Projection proj = {}) const
    {
        while (first != last) {
            if (!std::invoke(pred, std::invoke(proj, *first)))
                return false;
            ++first;
        }
        return true;
    }
} all_of;
[[maybe_unused]] inline constexpr struct { // Niebloid
    template <typename InputIterator, typename Sentinel,
              typename Predicate, typename Projection = q20::identity>
    constexpr bool operator()(InputIterator first, Sentinel last, Predicate pred, Projection proj = {}) const
    {
        while (first != last) {
            if (std::invoke(pred, std::invoke(proj, *first)))
                return false;
            ++first;
        }
        return true;
    }
} none_of;
#endif // __cpp_lib_ranges
} // namespace q20::ranges

QT_END_NAMESPACE

#endif /* Q20ALGORITHM_H */
