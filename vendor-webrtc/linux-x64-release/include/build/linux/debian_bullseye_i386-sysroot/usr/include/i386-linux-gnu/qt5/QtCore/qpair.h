/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtCore module of the Qt Toolkit.
**
** $QT_BEGIN_LICENSE:LGPL$
** Commercial License Usage
** Licensees holding valid commercial Qt licenses may use this file in
** accordance with the commercial license agreement provided with the
** Software or, alternatively, in accordance with the terms contained in
** a written agreement between you and The Qt Company. For licensing terms
** and conditions see https://www.qt.io/terms-conditions. For further
** information use the contact form at https://www.qt.io/contact-us.
**
** GNU Lesser General Public License Usage
** Alternatively, this file may be used under the terms of the GNU Lesser
** General Public License version 3 as published by the Free Software
** Foundation and appearing in the file LICENSE.LGPL3 included in the
** packaging of this file. Please review the following information to
** ensure the GNU Lesser General Public License version 3 requirements
** will be met: https://www.gnu.org/licenses/lgpl-3.0.html.
**
** GNU General Public License Usage
** Alternatively, this file may be used under the terms of the GNU
** General Public License version 2.0 or (at your option) the GNU General
** Public license version 3 or any later version approved by the KDE Free
** Qt Foundation. The licenses are as published by the Free Software
** Foundation and appearing in the file LICENSE.GPL2 and LICENSE.GPL3
** included in the packaging of this file. Please review the following
** information to ensure the GNU General Public License requirements will
** be met: https://www.gnu.org/licenses/gpl-2.0.html and
** https://www.gnu.org/licenses/gpl-3.0.html.
**
** $QT_END_LICENSE$
**
****************************************************************************/

#ifndef QPAIR_H
#define QPAIR_H

#include <QtCore/qglobal.h>

QT_BEGIN_NAMESPACE


template <class T1, class T2>
struct QPair
{
    typedef T1 first_type;
    typedef T2 second_type;

    Q_DECL_CONSTEXPR QPair()
        noexcept((std::is_nothrow_default_constructible<T1>::value &&
                              std::is_nothrow_default_constructible<T2>::value))
        : first(), second() {}
    Q_DECL_CONSTEXPR QPair(const T1 &t1, const T2 &t2)
        noexcept((std::is_nothrow_copy_constructible<T1>::value &&
                              std::is_nothrow_copy_constructible<T2>::value))
        : first(t1), second(t2) {}
    // compiler-generated copy/move ctor/assignment operators are fine!

    template <typename TT1, typename TT2>
    Q_DECL_CONSTEXPR QPair(const QPair<TT1, TT2> &p)
        noexcept((std::is_nothrow_constructible<T1, TT1&>::value &&
                              std::is_nothrow_constructible<T2, TT2&>::value))
        : first(p.first), second(p.second) {}
    template <typename TT1, typename TT2>
    Q_DECL_RELAXED_CONSTEXPR QPair &operator=(const QPair<TT1, TT2> &p)
        noexcept((std::is_nothrow_assignable<T1, TT1&>::value &&
                              std::is_nothrow_assignable<T2, TT2&>::value))
    { first = p.first; second = p.second; return *this; }
    template <typename TT1, typename TT2>
    Q_DECL_CONSTEXPR QPair(QPair<TT1, TT2> &&p)
        noexcept((std::is_nothrow_constructible<T1, TT1>::value &&
                              std::is_nothrow_constructible<T2, TT2>::value))
        // can't use std::move here as it's not constexpr in C++11:
        : first(static_cast<TT1 &&>(p.first)), second(static_cast<TT2 &&>(p.second)) {}
    template <typename TT1, typename TT2>
    Q_DECL_RELAXED_CONSTEXPR QPair &operator=(QPair<TT1, TT2> &&p)
        noexcept((std::is_nothrow_assignable<T1, TT1>::value &&
                              std::is_nothrow_assignable<T2, TT2>::value))
    { first = std::move(p.first); second = std::move(p.second); return *this; }

    Q_DECL_RELAXED_CONSTEXPR void swap(QPair &other)
        noexcept(noexcept(qSwap(other.first, other.first)) && noexcept(qSwap(other.second, other.second)))
    {
        // use qSwap() to pick up ADL swaps automatically:
        qSwap(first, other.first);
        qSwap(second, other.second);
    }

    T1 first;
    T2 second;
};

#if defined(__cpp_deduction_guides) && __cpp_deduction_guides >= 201606
template<class T1, class T2>
QPair(T1, T2) -> QPair<T1, T2>;
#endif

template <typename T1, typename T2>
void swap(QPair<T1, T2> &lhs, QPair<T1, T2> &rhs) noexcept(noexcept(lhs.swap(rhs)))
{ lhs.swap(rhs); }

// mark QPair<T1,T2> as complex/movable/primitive depending on the
// typeinfos of the constituents:
template<class T1, class T2>
class QTypeInfo<QPair<T1, T2> > : public QTypeInfoMerger<QPair<T1, T2>, T1, T2> {}; // Q_DECLARE_TYPEINFO

template <class T1, class T2>
Q_DECL_CONSTEXPR Q_INLINE_TEMPLATE bool operator==(const QPair<T1, T2> &p1, const QPair<T1, T2> &p2)
    noexcept(noexcept(p1.first == p2.first && p1.second == p2.second))
{ return p1.first == p2.first && p1.second == p2.second; }

template <class T1, class T2>
Q_DECL_CONSTEXPR Q_INLINE_TEMPLATE bool operator!=(const QPair<T1, T2> &p1, const QPair<T1, T2> &p2)
    noexcept(noexcept(!(p1 == p2)))
{ return !(p1 == p2); }

template <class T1, class T2>
Q_DECL_CONSTEXPR Q_INLINE_TEMPLATE bool operator<(const QPair<T1, T2> &p1, const QPair<T1, T2> &p2)
    noexcept(noexcept(p1.first < p2.first || (!(p2.first < p1.first) && p1.second < p2.second)))
{
    return p1.first < p2.first || (!(p2.first < p1.first) && p1.second < p2.second);
}

template <class T1, class T2>
Q_DECL_CONSTEXPR Q_INLINE_TEMPLATE bool operator>(const QPair<T1, T2> &p1, const QPair<T1, T2> &p2)
    noexcept(noexcept(p2 < p1))
{
    return p2 < p1;
}

template <class T1, class T2>
Q_DECL_CONSTEXPR Q_INLINE_TEMPLATE bool operator<=(const QPair<T1, T2> &p1, const QPair<T1, T2> &p2)
    noexcept(noexcept(!(p2 < p1)))
{
    return !(p2 < p1);
}

template <class T1, class T2>
Q_DECL_CONSTEXPR Q_INLINE_TEMPLATE bool operator>=(const QPair<T1, T2> &p1, const QPair<T1, T2> &p2)
    noexcept(noexcept(!(p1 < p2)))
{
    return !(p1 < p2);
}

template <class T1, class T2>
Q_DECL_CONSTEXPR Q_OUTOFLINE_TEMPLATE QPair<T1, T2> qMakePair(const T1 &x, const T2 &y)
    noexcept(noexcept(QPair<T1, T2>(x, y)))
{
    return QPair<T1, T2>(x, y);
}

QT_END_NAMESPACE

#endif // QPAIR_H
