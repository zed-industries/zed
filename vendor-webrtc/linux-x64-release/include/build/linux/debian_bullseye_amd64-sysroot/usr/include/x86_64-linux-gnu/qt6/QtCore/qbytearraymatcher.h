// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QBYTEARRAYMATCHER_H
#define QBYTEARRAYMATCHER_H

#include <QtCore/qbytearray.h>

#include <QtCore/q20algorithm.h>
#include <iterator>
#include <limits>

QT_BEGIN_NAMESPACE


class QByteArrayMatcherPrivate;

class Q_CORE_EXPORT QByteArrayMatcher
{
public:
    QByteArrayMatcher();
    explicit QByteArrayMatcher(const QByteArray &pattern);
    explicit QByteArrayMatcher(QByteArrayView pattern)
        : QByteArrayMatcher(pattern.data(), pattern.size())
    {}
    explicit QByteArrayMatcher(const char *pattern, qsizetype length = -1);
    QByteArrayMatcher(const QByteArrayMatcher &other);
    ~QByteArrayMatcher();

    QByteArrayMatcher &operator=(const QByteArrayMatcher &other);

    void setPattern(const QByteArray &pattern);

#if QT_CORE_REMOVED_SINCE(6, 3)
    qsizetype indexIn(const QByteArray &ba, qsizetype from = 0) const;
#else
    Q_WEAK_OVERLOAD
    qsizetype indexIn(const QByteArray &ba, qsizetype from = 0) const
    { return indexIn(QByteArrayView{ba}, from); }
#endif
    qsizetype indexIn(const char *str, qsizetype len, qsizetype from = 0) const;
    qsizetype indexIn(QByteArrayView data, qsizetype from = 0) const;
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
        qsizetype l;
    };
    union {
        uint dummy[256];
        Data p;
    };
};

class QStaticByteArrayMatcherBase
{
    alignas(16)
    struct Skiptable {
        uchar data[256];
    } m_skiptable;
protected:
    explicit constexpr QStaticByteArrayMatcherBase(const char *pattern, size_t n) noexcept
        : m_skiptable(generate(pattern, n)) {}
    // compiler-generated copy/more ctors/assignment operators are ok!
    ~QStaticByteArrayMatcherBase() = default;

#if QT_CORE_REMOVED_SINCE(6, 3) && QT_POINTER_SIZE != 4
    Q_CORE_EXPORT int indexOfIn(const char *needle, uint nlen, const char *haystack, int hlen, int from) const noexcept;
#endif
    Q_CORE_EXPORT qsizetype indexOfIn(const char *needle, size_t nlen,
                                      const char *haystack, qsizetype hlen,
                                      qsizetype from) const noexcept;

private:
    static constexpr Skiptable generate(const char *pattern, size_t n) noexcept
    {
        const auto uchar_max = (std::numeric_limits<uchar>::max)();
        uchar max = n > uchar_max ? uchar_max : uchar(n);
        Skiptable table = {};
        q20::fill(std::begin(table.data), std::end(table.data), max);
        pattern += n - max;
        while (max--)
            table.data[uchar(*pattern++)] = max;
        return table;
    }
};

template <size_t N>
class QStaticByteArrayMatcher : QStaticByteArrayMatcherBase
{
    char m_pattern[N];
    // N includes the terminating '\0'!
    static_assert(N > 2, "QStaticByteArrayMatcher makes no sense for finding a single-char pattern");
public:
    explicit constexpr QStaticByteArrayMatcher(const char (&patternToMatch)[N]) noexcept
        : QStaticByteArrayMatcherBase(patternToMatch, N - 1), m_pattern()
    {
        for (size_t i = 0; i < N; ++i)
            m_pattern[i] = patternToMatch[i];
    }

    Q_WEAK_OVERLOAD
    qsizetype indexIn(const QByteArray &haystack, qsizetype from = 0) const noexcept
    { return this->indexOfIn(m_pattern, N - 1, haystack.data(), haystack.size(), from); }
    qsizetype indexIn(const char *haystack, qsizetype hlen, qsizetype from = 0) const noexcept
    { return this->indexOfIn(m_pattern, N - 1, haystack, hlen, from); }
    qsizetype indexIn(QByteArrayView haystack, qsizetype from = 0) const noexcept
    { return this->indexOfIn(m_pattern, N - 1, haystack.data(), haystack.size(), from); }

    QByteArray pattern() const { return QByteArray(m_pattern, qsizetype(N - 1)); }
};

template <size_t N>
constexpr QStaticByteArrayMatcher<N> qMakeStaticByteArrayMatcher(const char (&pattern)[N]) noexcept
{ return QStaticByteArrayMatcher<N>(pattern); }

QT_END_NAMESPACE

#endif // QBYTEARRAYMATCHER_H
