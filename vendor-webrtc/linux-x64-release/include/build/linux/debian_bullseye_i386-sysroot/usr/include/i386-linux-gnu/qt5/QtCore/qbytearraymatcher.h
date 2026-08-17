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

#ifndef QBYTEARRAYMATCHER_H
#define QBYTEARRAYMATCHER_H

#include <QtCore/qbytearray.h>

#include <limits>

QT_BEGIN_NAMESPACE


class QByteArrayMatcherPrivate;

class Q_CORE_EXPORT QByteArrayMatcher
{
public:
    QByteArrayMatcher();
    explicit QByteArrayMatcher(const QByteArray &pattern);
    explicit QByteArrayMatcher(const char *pattern, int length);
    QByteArrayMatcher(const QByteArrayMatcher &other);
    ~QByteArrayMatcher();

    QByteArrayMatcher &operator=(const QByteArrayMatcher &other);

    void setPattern(const QByteArray &pattern);

    int indexIn(const QByteArray &ba, int from = 0) const;
    int indexIn(const char *str, int len, int from = 0) const;
    inline QByteArray pattern() const
    {
        if (q_pattern.isNull())
            return QByteArray(reinterpret_cast<const char*>(p.p), p.l);
        return q_pattern;
    }

private:
    QByteArrayMatcherPrivate *d;
    QByteArray q_pattern;
    struct Data {
        uchar q_skiptable[256];
        const uchar *p;
        int l;
    };
    union {
        uint dummy[256];
        Data p;
    };
};

class QStaticByteArrayMatcherBase
{
    Q_DECL_ALIGN(16)
    struct Skiptable {
        uchar data[256];
    } m_skiptable;
protected:
    explicit Q_DECL_RELAXED_CONSTEXPR QStaticByteArrayMatcherBase(const char *pattern, uint n) noexcept
        : m_skiptable(generate(pattern, n)) {}
    // compiler-generated copy/more ctors/assignment operators are ok!
    // compiler-generated dtor is ok!

    Q_CORE_EXPORT int indexOfIn(const char *needle, uint nlen, const char *haystack, int hlen, int from) const noexcept;

private:
    static Q_DECL_RELAXED_CONSTEXPR Skiptable generate(const char *pattern, uint n) noexcept
    {
        const auto uchar_max = (std::numeric_limits<uchar>::max)();
        uchar max = n > uchar_max ? uchar_max : n;
        Skiptable table = {
            // this verbose initialization code aims to avoid some opaque error messages
            // even on powerful compilers such as GCC 5.3. Even though for GCC a loop
            // format can be found that v5.3 groks, it's probably better to go with this
            // for the time being:
            {
                max, max, max, max, max, max, max, max,   max, max, max, max, max, max, max, max,
                max, max, max, max, max, max, max, max,   max, max, max, max, max, max, max, max,
                max, max, max, max, max, max, max, max,   max, max, max, max, max, max, max, max,
                max, max, max, max, max, max, max, max,   max, max, max, max, max, max, max, max,
                max, max, max, max, max, max, max, max,   max, max, max, max, max, max, max, max,
                max, max, max, max, max, max, max, max,   max, max, max, max, max, max, max, max,
                max, max, max, max, max, max, max, max,   max, max, max, max, max, max, max, max,
                max, max, max, max, max, max, max, max,   max, max, max, max, max, max, max, max,

                max, max, max, max, max, max, max, max,   max, max, max, max, max, max, max, max,
                max, max, max, max, max, max, max, max,   max, max, max, max, max, max, max, max,
                max, max, max, max, max, max, max, max,   max, max, max, max, max, max, max, max,
                max, max, max, max, max, max, max, max,   max, max, max, max, max, max, max, max,
                max, max, max, max, max, max, max, max,   max, max, max, max, max, max, max, max,
                max, max, max, max, max, max, max, max,   max, max, max, max, max, max, max, max,
                max, max, max, max, max, max, max, max,   max, max, max, max, max, max, max, max,
                max, max, max, max, max, max, max, max,   max, max, max, max, max, max, max, max,
            }
        };
        pattern += n - max;
        while (max--)
            table.data[uchar(*pattern++)] = max;
        return table;
    }
};

template <uint N>
class QStaticByteArrayMatcher : QStaticByteArrayMatcherBase
{
    char m_pattern[N];
    Q_STATIC_ASSERT_X(N > 2, "QStaticByteArrayMatcher makes no sense for finding a single-char pattern");
public:
    explicit Q_DECL_RELAXED_CONSTEXPR QStaticByteArrayMatcher(const char (&patternToMatch)[N]) noexcept
        : QStaticByteArrayMatcherBase(patternToMatch, N - 1), m_pattern()
    {
        for (uint i = 0; i < N; ++i)
            m_pattern[i] = patternToMatch[i];
    }

    int indexIn(const QByteArray &haystack, int from = 0) const noexcept
    { return this->indexOfIn(m_pattern, N - 1, haystack.data(), haystack.size(), from); }
    int indexIn(const char *haystack, int hlen, int from = 0) const noexcept
    { return this->indexOfIn(m_pattern, N - 1, haystack, hlen, from); }

    QByteArray pattern() const { return QByteArray(m_pattern, int(N - 1)); }
};

template <uint N>
Q_DECL_RELAXED_CONSTEXPR QStaticByteArrayMatcher<N> qMakeStaticByteArrayMatcher(const char (&pattern)[N]) noexcept
{ return QStaticByteArrayMatcher<N>(pattern); }

QT_END_NAMESPACE

#endif // QBYTEARRAYMATCHER_H
