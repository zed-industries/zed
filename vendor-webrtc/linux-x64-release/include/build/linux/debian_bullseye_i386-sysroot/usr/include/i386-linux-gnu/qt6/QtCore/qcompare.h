// Copyright (C) 2020 Klarälvdalens Datakonsult AB, a KDAB Group company, info@kdab.com, author Giuseppe D'Angelo <giuseppe.dangelo@kdab.com>
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QCOMPARE_H
#define QCOMPARE_H

#if 0
#pragma qt_class(QtCompare)
#endif

#include <QtCore/qglobal.h>
#include <QtCore/qcompare_impl.h>

QT_BEGIN_NAMESPACE

namespace QtPrivate {
using CompareUnderlyingType = qint8;

// [cmp.categories.pre] / 1
enum class Ordering : CompareUnderlyingType
{
    Equal = 0,
    Equivalent = Equal,
    Less = -1,
    Greater = 1
};

enum class Uncomparable : CompareUnderlyingType
{
    Unordered = -127
};

} // namespace QtPrivate

// [cmp.partialord]
class QPartialOrdering
{
public:
    static const QPartialOrdering Less;
    static const QPartialOrdering Equivalent;
    static const QPartialOrdering Greater;
    static const QPartialOrdering Unordered;

    friend constexpr bool operator==(QPartialOrdering p, QtPrivate::CompareAgainstLiteralZero) noexcept
    { return p.isOrdered() && p.m_order == 0; }
    friend constexpr bool operator!=(QPartialOrdering p, QtPrivate::CompareAgainstLiteralZero) noexcept
    { return p.isOrdered() && p.m_order != 0; }
    friend constexpr bool operator< (QPartialOrdering p, QtPrivate::CompareAgainstLiteralZero) noexcept
    { return p.isOrdered() && p.m_order <  0; }
    friend constexpr bool operator<=(QPartialOrdering p, QtPrivate::CompareAgainstLiteralZero) noexcept
    { return p.isOrdered() && p.m_order <= 0; }
    friend constexpr bool operator> (QPartialOrdering p, QtPrivate::CompareAgainstLiteralZero) noexcept
    { return p.isOrdered() && p.m_order >  0; }
    friend constexpr bool operator>=(QPartialOrdering p, QtPrivate::CompareAgainstLiteralZero) noexcept
    { return p.isOrdered() && p.m_order >= 0; }

    friend constexpr bool operator==(QtPrivate::CompareAgainstLiteralZero, QPartialOrdering p) noexcept
    { return p.isOrdered() && 0 == p.m_order; }
    friend constexpr bool operator!=(QtPrivate::CompareAgainstLiteralZero, QPartialOrdering p) noexcept
    { return p.isOrdered() && 0 != p.m_order; }
    friend constexpr bool operator< (QtPrivate::CompareAgainstLiteralZero, QPartialOrdering p) noexcept
    { return p.isOrdered() && 0 <  p.m_order; }
    friend constexpr bool operator<=(QtPrivate::CompareAgainstLiteralZero, QPartialOrdering p) noexcept
    { return p.isOrdered() && 0 <= p.m_order; }
    friend constexpr bool operator> (QtPrivate::CompareAgainstLiteralZero, QPartialOrdering p) noexcept
    { return p.isOrdered() && 0 >  p.m_order; }
    friend constexpr bool operator>=(QtPrivate::CompareAgainstLiteralZero, QPartialOrdering p) noexcept
    { return p.isOrdered() && 0 >= p.m_order; }

    friend constexpr bool operator==(QPartialOrdering p1, QPartialOrdering p2) noexcept
    { return p1.m_order == p2.m_order; }
    friend constexpr bool operator!=(QPartialOrdering p1, QPartialOrdering p2) noexcept
    { return p1.m_order != p2.m_order; }

private:
    constexpr explicit QPartialOrdering(QtPrivate::Ordering order) noexcept
        : m_order(static_cast<QtPrivate::CompareUnderlyingType>(order))
    {}
    constexpr explicit QPartialOrdering(QtPrivate::Uncomparable order) noexcept
        : m_order(static_cast<QtPrivate::CompareUnderlyingType>(order))
    {}

    // instead of the exposition only is_ordered member in [cmp.partialord],
    // use a private function
    constexpr bool isOrdered() noexcept
    { return m_order != static_cast<QtPrivate::CompareUnderlyingType>(QtPrivate::Uncomparable::Unordered); }

    QtPrivate::CompareUnderlyingType m_order;
};

inline constexpr QPartialOrdering QPartialOrdering::Less(QtPrivate::Ordering::Less);
inline constexpr QPartialOrdering QPartialOrdering::Equivalent(QtPrivate::Ordering::Equivalent);
inline constexpr QPartialOrdering QPartialOrdering::Greater(QtPrivate::Ordering::Greater);
inline constexpr QPartialOrdering QPartialOrdering::Unordered(QtPrivate::Uncomparable::Unordered);

QT_END_NAMESPACE

#endif // QCOMPARE_H
