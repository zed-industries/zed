// Copyright (C) 2020 The Qt Company Ltd.
// Copyright (C) 2019 Intel Corporation.
// Copyright (C) 2019 Mail.ru Group.
// Copyright (C) 2020 Klarälvdalens Datakonsult AB, a KDAB Group company, info@kdab.com, author Marc Mutz <marc.mutz@kdab.com>
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSTRING_H
#define QSTRING_H

#if defined(QT_NO_CAST_FROM_ASCII) && defined(QT_RESTRICTED_CAST_FROM_ASCII)
#error QT_NO_CAST_FROM_ASCII and QT_RESTRICTED_CAST_FROM_ASCII must not be defined at the same time
#endif

#include <QtCore/qchar.h>
#include <QtCore/qbytearray.h>
#include <QtCore/qbytearrayview.h>
#include <QtCore/qarraydata.h>
#include <QtCore/qnamespace.h>
#include <QtCore/qstringliteral.h>
#include <QtCore/qstringalgorithms.h>
#include <QtCore/qanystringview.h>
#include <QtCore/qstringtokenizer.h>

#include <string>
#include <iterator>

#include <stdarg.h>

#ifdef truncate
#error qstring.h must be included before any header file that defines truncate
#endif

#if defined(Q_OS_DARWIN) || defined(Q_QDOC)
Q_FORWARD_DECLARE_CF_TYPE(CFString);
Q_FORWARD_DECLARE_OBJC_CLASS(NSString);
#endif

#if 0
// Workaround for generating forward headers
#pragma qt_class(QLatin1String)
#pragma qt_class(QLatin1StringView)
#endif

QT_BEGIN_NAMESPACE

class QRegularExpression;
class QRegularExpressionMatch;
class QString;

namespace QtPrivate {
template <bool...B> class BoolList;
}

#if QT_VERSION >= QT_VERSION_CHECK(7, 0, 0) || defined(QT_BOOTSTRAPPED) || defined(Q_CLANG_QDOC)
#    define Q_L1S_VIEW_IS_PRIMARY
class QLatin1StringView
#else
class QLatin1String
#endif
{
public:
#ifdef Q_L1S_VIEW_IS_PRIMARY
    constexpr inline QLatin1StringView() noexcept : m_size(0), m_data(nullptr) {}
    constexpr QLatin1StringView(std::nullptr_t) noexcept : QLatin1StringView() {}
    constexpr inline explicit QLatin1StringView(const char *s) noexcept
        : m_size(s ? qsizetype(QtPrivate::lengthHelperPointer(s)) : 0), m_data(s) {}
    constexpr QLatin1StringView(const char *f, const char *l)
        : QLatin1StringView(f, qsizetype(l - f)) {}
    constexpr inline QLatin1StringView(const char *s, qsizetype sz) noexcept : m_size(sz), m_data(s) {}
    explicit QLatin1StringView(const QByteArray &s) noexcept : m_size(s.size()), m_data(s.constData()) {}
    constexpr explicit QLatin1StringView(QByteArrayView s) noexcept : m_size(s.size()), m_data(s.data()) {}
#else
    constexpr inline QLatin1String() noexcept : m_size(0), m_data(nullptr) {}
    Q_WEAK_OVERLOAD
    constexpr QLatin1String(std::nullptr_t) noexcept : QLatin1String() {}
    constexpr inline explicit QLatin1String(const char *s) noexcept
        : m_size(s ? qsizetype(QtPrivate::lengthHelperPointer(s)) : 0), m_data(s) {}
    constexpr QLatin1String(const char *f, const char *l)
        : QLatin1String(f, qsizetype(l - f)) {}
    constexpr inline QLatin1String(const char *s, qsizetype sz) noexcept : m_size(sz), m_data(s) {}
    explicit QLatin1String(const QByteArray &s) noexcept : m_size(s.size()), m_data(s.constData()) {}
    constexpr explicit QLatin1String(QByteArrayView s) noexcept : m_size(s.size()), m_data(s.data()) {}
#endif // !Q_L1S_VIEW_IS_PRIMARY

    inline QString toString() const;

    constexpr const char *latin1() const noexcept { return m_data; }
    constexpr qsizetype size() const noexcept { return m_size; }
    constexpr const char *data() const noexcept { return m_data; }
    [[nodiscard]] constexpr const char *constData() const noexcept { return data(); }
    [[nodiscard]] constexpr const char *constBegin() const noexcept { return begin(); }
    [[nodiscard]] constexpr const char *constEnd() const noexcept { return end(); }

    [[nodiscard]] constexpr QLatin1Char first() const { return front(); }
    [[nodiscard]] constexpr QLatin1Char last() const { return back(); }

    [[nodiscard]] constexpr qsizetype length() const noexcept { return size(); }

    constexpr bool isNull() const noexcept { return !data(); }
    constexpr bool isEmpty() const noexcept { return !size(); }

    [[nodiscard]] constexpr bool empty() const noexcept { return size() == 0; }

    template <typename...Args>
    [[nodiscard]] inline QString arg(Args &&...args) const;

    [[nodiscard]] constexpr QLatin1Char at(qsizetype i) const
    { return Q_ASSERT(i >= 0), Q_ASSERT(i < size()), QLatin1Char(m_data[i]); }
    [[nodiscard]] constexpr QLatin1Char operator[](qsizetype i) const { return at(i); }

    [[nodiscard]] constexpr QLatin1Char front() const { return at(0); }
    [[nodiscard]] constexpr QLatin1Char back() const { return at(size() - 1); }

    [[nodiscard]] int compare(QStringView other, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return QtPrivate::compareStrings(*this, other, cs); }
    [[nodiscard]] int compare(QLatin1StringView other, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return QtPrivate::compareStrings(*this, other, cs); }
    [[nodiscard]] constexpr int compare(QChar c) const noexcept
    { return isEmpty() ? -1 : front() == c ? int(size() > 1) : uchar(m_data[0]) - c.unicode(); }
    [[nodiscard]] int compare(QChar c, Qt::CaseSensitivity cs) const noexcept
    { return QtPrivate::compareStrings(*this, QStringView(&c, 1), cs); }

    [[nodiscard]] bool startsWith(QStringView s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return QtPrivate::startsWith(*this, s, cs); }
    [[nodiscard]] bool startsWith(QLatin1StringView s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return QtPrivate::startsWith(*this, s, cs); }
    [[nodiscard]] constexpr bool startsWith(QChar c) const noexcept
    { return !isEmpty() && front() == c; }
    [[nodiscard]] inline bool startsWith(QChar c, Qt::CaseSensitivity cs) const noexcept
    { return QtPrivate::startsWith(*this, QStringView(&c, 1), cs); }

    [[nodiscard]] bool endsWith(QStringView s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return QtPrivate::endsWith(*this, s, cs); }
    [[nodiscard]] bool endsWith(QLatin1StringView s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return QtPrivate::endsWith(*this, s, cs); }
    [[nodiscard]] constexpr bool endsWith(QChar c) const noexcept
    { return !isEmpty() && back() == c; }
    [[nodiscard]] inline bool endsWith(QChar c, Qt::CaseSensitivity cs) const noexcept
    { return QtPrivate::endsWith(*this, QStringView(&c, 1), cs); }

    [[nodiscard]] qsizetype indexOf(QStringView s, qsizetype from = 0, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return QtPrivate::findString(*this, from, s, cs); }
    [[nodiscard]] qsizetype indexOf(QLatin1StringView s, qsizetype from = 0, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return QtPrivate::findString(*this, from, s, cs); }
    [[nodiscard]] qsizetype indexOf(QChar c, qsizetype from = 0, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return QtPrivate::findString(*this, from, QStringView(&c, 1), cs); }

    [[nodiscard]] bool contains(QStringView s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return indexOf(s, 0, cs) != -1; }
    [[nodiscard]] bool contains(QLatin1StringView s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return indexOf(s, 0, cs) != -1; }
    [[nodiscard]] inline bool contains(QChar c, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return indexOf(QStringView(&c, 1), 0, cs) != -1; }

    [[nodiscard]] qsizetype lastIndexOf(QStringView s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return lastIndexOf(s, size(), cs); }
    [[nodiscard]] qsizetype lastIndexOf(QStringView s, qsizetype from, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return QtPrivate::lastIndexOf(*this, from, s, cs); }
    [[nodiscard]] qsizetype lastIndexOf(QLatin1StringView s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return lastIndexOf(s, size(), cs); }
    [[nodiscard]] qsizetype lastIndexOf(QLatin1StringView s, qsizetype from, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return QtPrivate::lastIndexOf(*this, from, s, cs); }
    [[nodiscard]] qsizetype lastIndexOf(QChar c, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return lastIndexOf(c, -1, cs); }
    [[nodiscard]] qsizetype lastIndexOf(QChar c, qsizetype from, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return QtPrivate::lastIndexOf(*this, from, QStringView(&c, 1), cs); }

    [[nodiscard]] qsizetype count(QStringView str, Qt::CaseSensitivity cs = Qt::CaseSensitive) const
    { return QtPrivate::count(*this, str, cs); }
    [[nodiscard]] qsizetype count(QLatin1StringView str, Qt::CaseSensitivity cs = Qt::CaseSensitive) const
    { return QtPrivate::count(*this, str, cs); }
    [[nodiscard]] qsizetype count(QChar ch, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return QtPrivate::count(*this, ch, cs); }

    [[nodiscard]] short toShort(bool *ok = nullptr, int base = 10) const
    { return QtPrivate::toIntegral<short>(QByteArrayView(*this), ok, base); }
    [[nodiscard]] ushort toUShort(bool *ok = nullptr, int base = 10) const
    { return QtPrivate::toIntegral<ushort>(QByteArrayView(*this), ok, base); }
    [[nodiscard]] int toInt(bool *ok = nullptr, int base = 10) const
    { return QtPrivate::toIntegral<int>(QByteArrayView(*this), ok, base); }
    [[nodiscard]] uint toUInt(bool *ok = nullptr, int base = 10) const
    { return QtPrivate::toIntegral<uint>(QByteArrayView(*this), ok, base); }
    [[nodiscard]] long toLong(bool *ok = nullptr, int base = 10) const
    { return QtPrivate::toIntegral<long>(QByteArrayView(*this), ok, base); }
    [[nodiscard]] ulong toULong(bool *ok = nullptr, int base = 10) const
    { return QtPrivate::toIntegral<ulong>(QByteArrayView(*this), ok, base); }
    [[nodiscard]] qlonglong toLongLong(bool *ok = nullptr, int base = 10) const
    { return QtPrivate::toIntegral<qlonglong>(QByteArrayView(*this), ok, base); }
    [[nodiscard]] qulonglong toULongLong(bool *ok = nullptr, int base = 10) const
    { return QtPrivate::toIntegral<qulonglong>(QByteArrayView(*this), ok, base); }
    [[nodiscard]] float toFloat(bool *ok = nullptr) const
    {
        const auto r = QtPrivate::toFloat(*this);
        if (ok)
            *ok = bool(r);
        return r.value_or(0.0f);
    }
    [[nodiscard]] double toDouble(bool *ok = nullptr) const
    {
        const auto r = QtPrivate::toDouble(*this);
        if (ok)
            *ok = bool(r);
        return r.value_or(0.0);
    }

    using value_type = const char;
    using reference = value_type&;
    using const_reference = reference;
    using iterator = value_type*;
    using const_iterator = iterator;
    using difference_type = qsizetype; // violates Container concept requirements
    using size_type = qsizetype;       // violates Container concept requirements

    constexpr const_iterator begin() const noexcept { return data(); }
    constexpr const_iterator cbegin() const noexcept { return data(); }
    constexpr const_iterator end() const noexcept { return data() + size(); }
    constexpr const_iterator cend() const noexcept { return data() + size(); }

    using reverse_iterator = std::reverse_iterator<iterator>;
    using const_reverse_iterator = reverse_iterator;

    const_reverse_iterator rbegin() const noexcept { return const_reverse_iterator(end()); }
    const_reverse_iterator crbegin() const noexcept { return const_reverse_iterator(end()); }
    const_reverse_iterator rend() const noexcept { return const_reverse_iterator(begin()); }
    const_reverse_iterator crend() const noexcept { return const_reverse_iterator(begin()); }

    [[nodiscard]] constexpr QLatin1StringView mid(qsizetype pos, qsizetype n = -1) const
    {
        using namespace QtPrivate;
        auto result = QContainerImplHelper::mid(size(), &pos, &n);
        return result == QContainerImplHelper::Null ? QLatin1StringView()
                                                    : QLatin1StringView(m_data + pos, n);
    }
    [[nodiscard]] constexpr QLatin1StringView left(qsizetype n) const
    {
        if (size_t(n) >= size_t(size()))
            n = size();
        return {m_data, n};
    }
    [[nodiscard]] constexpr QLatin1StringView right(qsizetype n) const
    {
        if (size_t(n) >= size_t(size()))
            n = size();
        return {m_data + m_size - n, n};
    }

    [[nodiscard]] constexpr QLatin1StringView sliced(qsizetype pos) const
    { verify(pos); return {m_data + pos, m_size - pos}; }
    [[nodiscard]] constexpr QLatin1StringView sliced(qsizetype pos, qsizetype n) const
    { verify(pos, n); return {m_data + pos, n}; }
    [[nodiscard]] constexpr QLatin1StringView first(qsizetype n) const
    { verify(n); return {m_data, n}; }
    [[nodiscard]] constexpr QLatin1StringView last(qsizetype n) const
    { verify(n); return {m_data + size() - n, n}; }
    [[nodiscard]] constexpr QLatin1StringView chopped(qsizetype n) const
    { verify(n); return {m_data, size() - n}; }

    constexpr void chop(qsizetype n)
    { verify(n); m_size -= n; }
    constexpr void truncate(qsizetype n)
    { verify(n); m_size = n; }

    [[nodiscard]] QLatin1StringView trimmed() const noexcept { return QtPrivate::trimmed(*this); }

    template <typename Needle, typename...Flags>
    [[nodiscard]] inline constexpr auto tokenize(Needle &&needle, Flags...flags) const
        noexcept(noexcept(qTokenize(std::declval<const QLatin1StringView &>(),
                                    std::forward<Needle>(needle), flags...)))
            -> decltype(qTokenize(*this, std::forward<Needle>(needle), flags...))
    { return qTokenize(*this, std::forward<Needle>(needle), flags...); }

    friend inline bool operator==(QLatin1StringView s1, QLatin1StringView s2) noexcept
    { return s1.size() == s2.size() && (!s1.size() || !memcmp(s1.latin1(), s2.latin1(), s1.size())); }
    friend inline bool operator!=(QLatin1StringView s1, QLatin1StringView s2) noexcept
    { return !(s1 == s2); }
    friend inline bool operator<(QLatin1StringView s1, QLatin1StringView s2) noexcept
    {
        const qsizetype len = qMin(s1.size(), s2.size());
        const int r = len ? memcmp(s1.latin1(), s2.latin1(), len) : 0;
        return r < 0 || (r == 0 && s1.size() < s2.size());
    }
    friend inline bool operator>(QLatin1StringView s1, QLatin1StringView s2) noexcept
    { return s2 < s1; }
    friend inline bool operator<=(QLatin1StringView s1, QLatin1StringView s2) noexcept
    { return !(s1 > s2); }
    friend inline bool operator>=(QLatin1StringView s1, QLatin1StringView s2) noexcept
    { return !(s1 < s2); }

    // QChar <> QLatin1StringView
    friend inline bool operator==(QChar lhs, QLatin1StringView rhs) noexcept { return rhs.size() == 1 && lhs == rhs.front(); }
    friend inline bool operator< (QChar lhs, QLatin1StringView rhs) noexcept { return compare_helper(&lhs, 1, rhs) < 0; }
    friend inline bool operator> (QChar lhs, QLatin1StringView rhs) noexcept { return compare_helper(&lhs, 1, rhs) > 0; }
    friend inline bool operator!=(QChar lhs, QLatin1StringView rhs) noexcept { return !(lhs == rhs); }
    friend inline bool operator<=(QChar lhs, QLatin1StringView rhs) noexcept { return !(lhs >  rhs); }
    friend inline bool operator>=(QChar lhs, QLatin1StringView rhs) noexcept { return !(lhs <  rhs); }

    friend inline bool operator==(QLatin1StringView lhs, QChar rhs) noexcept { return   rhs == lhs; }
    friend inline bool operator!=(QLatin1StringView lhs, QChar rhs) noexcept { return !(rhs == lhs); }
    friend inline bool operator< (QLatin1StringView lhs, QChar rhs) noexcept { return   rhs >  lhs; }
    friend inline bool operator> (QLatin1StringView lhs, QChar rhs) noexcept { return   rhs <  lhs; }
    friend inline bool operator<=(QLatin1StringView lhs, QChar rhs) noexcept { return !(rhs <  lhs); }
    friend inline bool operator>=(QLatin1StringView lhs, QChar rhs) noexcept { return !(rhs >  lhs); }

    // QStringView <> QLatin1StringView
    friend inline bool operator==(QStringView lhs, QLatin1StringView rhs) noexcept
    { return lhs.size() == rhs.size() && QtPrivate::equalStrings(lhs, rhs); }
    friend inline bool operator!=(QStringView lhs, QLatin1StringView rhs) noexcept { return !(lhs == rhs); }
    friend inline bool operator< (QStringView lhs, QLatin1StringView rhs) noexcept { return QtPrivate::compareStrings(lhs, rhs) <  0; }
    friend inline bool operator<=(QStringView lhs, QLatin1StringView rhs) noexcept { return QtPrivate::compareStrings(lhs, rhs) <= 0; }
    friend inline bool operator> (QStringView lhs, QLatin1StringView rhs) noexcept { return QtPrivate::compareStrings(lhs, rhs) >  0; }
    friend inline bool operator>=(QStringView lhs, QLatin1StringView rhs) noexcept { return QtPrivate::compareStrings(lhs, rhs) >= 0; }

    friend inline bool operator==(QLatin1StringView lhs, QStringView rhs) noexcept
    { return lhs.size() == rhs.size() && QtPrivate::equalStrings(lhs, rhs); }
    friend inline bool operator!=(QLatin1StringView lhs, QStringView rhs) noexcept { return !(lhs == rhs); }
    friend inline bool operator< (QLatin1StringView lhs, QStringView rhs) noexcept { return QtPrivate::compareStrings(lhs, rhs) <  0; }
    friend inline bool operator<=(QLatin1StringView lhs, QStringView rhs) noexcept { return QtPrivate::compareStrings(lhs, rhs) <= 0; }
    friend inline bool operator> (QLatin1StringView lhs, QStringView rhs) noexcept { return QtPrivate::compareStrings(lhs, rhs) >  0; }
    friend inline bool operator>=(QLatin1StringView lhs, QStringView rhs) noexcept { return QtPrivate::compareStrings(lhs, rhs) >= 0; }


#if !defined(QT_NO_CAST_FROM_ASCII) && !defined(QT_RESTRICTED_CAST_FROM_ASCII)
    QT_ASCII_CAST_WARN inline bool operator==(const char *s) const;
    QT_ASCII_CAST_WARN inline bool operator!=(const char *s) const;
    QT_ASCII_CAST_WARN inline bool operator<(const char *s) const;
    QT_ASCII_CAST_WARN inline bool operator>(const char *s) const;
    QT_ASCII_CAST_WARN inline bool operator<=(const char *s) const;
    QT_ASCII_CAST_WARN inline bool operator>=(const char *s) const;

    QT_ASCII_CAST_WARN inline bool operator==(const QByteArray &s) const;
    QT_ASCII_CAST_WARN inline bool operator!=(const QByteArray &s) const;
    QT_ASCII_CAST_WARN inline bool operator<(const QByteArray &s) const;
    QT_ASCII_CAST_WARN inline bool operator>(const QByteArray &s) const;
    QT_ASCII_CAST_WARN inline bool operator<=(const QByteArray &s) const;
    QT_ASCII_CAST_WARN inline bool operator>=(const QByteArray &s) const;

    QT_ASCII_CAST_WARN friend bool operator==(const char *s1, QLatin1StringView s2) { return compare_helper(s2, s1) == 0; }
    QT_ASCII_CAST_WARN friend bool operator!=(const char *s1, QLatin1StringView s2) { return compare_helper(s2, s1) != 0; }
    QT_ASCII_CAST_WARN friend bool operator< (const char *s1, QLatin1StringView s2) { return compare_helper(s2, s1) >  0; }
    QT_ASCII_CAST_WARN friend bool operator> (const char *s1, QLatin1StringView s2) { return compare_helper(s2, s1) <  0; }
    QT_ASCII_CAST_WARN friend bool operator<=(const char *s1, QLatin1StringView s2) { return compare_helper(s2, s1) >= 0; }
    QT_ASCII_CAST_WARN friend bool operator>=(const char *s1, QLatin1StringView s2) { return compare_helper(s2, s1) <= 0; }
#endif // !defined(QT_NO_CAST_FROM_ASCII) && !defined(QT_RESTRICTED_CAST_FROM_ASCII)

private:
#if !defined(QT_NO_CAST_FROM_ASCII) && !defined(QT_RESTRICTED_CAST_FROM_ASCII)
    static inline int compare_helper(const QLatin1StringView &s1, const char *s2);
#endif
    Q_ALWAYS_INLINE constexpr void verify(qsizetype pos, qsizetype n = 0) const
    {
        Q_ASSERT(pos >= 0);
        Q_ASSERT(pos <= size());
        Q_ASSERT(n >= 0);
        Q_ASSERT(n <= size() - pos);
    }
    Q_CORE_EXPORT static int compare_helper(const QChar *data1, qsizetype length1,
                                            QLatin1StringView s2,
                                            Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;
    qsizetype m_size;
    const char *m_data;
};
#ifdef Q_L1S_VIEW_IS_PRIMARY
Q_DECLARE_TYPEINFO(QLatin1StringView, Q_RELOCATABLE_TYPE);
#else
Q_DECLARE_TYPEINFO(QLatin1String, Q_RELOCATABLE_TYPE);
#endif

// Qt 4.x compatibility

//
// QLatin1StringView inline implementations
//
constexpr bool QtPrivate::isLatin1(QLatin1StringView) noexcept
{ return true; }

//
// QStringView members that require QLatin1StringView:
//
int QStringView::compare(QLatin1StringView s, Qt::CaseSensitivity cs) const noexcept
{ return QtPrivate::compareStrings(*this, s, cs); }
bool QStringView::startsWith(QLatin1StringView s, Qt::CaseSensitivity cs) const noexcept
{ return QtPrivate::startsWith(*this, s, cs); }
bool QStringView::endsWith(QLatin1StringView s, Qt::CaseSensitivity cs) const noexcept
{ return QtPrivate::endsWith(*this, s, cs); }
qsizetype QStringView::indexOf(QLatin1StringView s, qsizetype from, Qt::CaseSensitivity cs) const noexcept
{ return QtPrivate::findString(*this, from, s, cs); }
bool QStringView::contains(QLatin1StringView s, Qt::CaseSensitivity cs) const noexcept
{ return indexOf(s, 0, cs) != qsizetype(-1); }
qsizetype QStringView::lastIndexOf(QLatin1StringView s, Qt::CaseSensitivity cs) const noexcept
{ return QtPrivate::lastIndexOf(*this, size(), s, cs); }
qsizetype QStringView::lastIndexOf(QLatin1StringView s, qsizetype from, Qt::CaseSensitivity cs) const noexcept
{ return QtPrivate::lastIndexOf(*this, from, s, cs); }
qsizetype QStringView::count(QLatin1StringView s, Qt::CaseSensitivity cs) const
{ return QtPrivate::count(*this, s, cs); }


//
// QAnyStringView members that require QLatin1StringView
//

constexpr QAnyStringView::QAnyStringView(QLatin1StringView str) noexcept
    : m_data{str.data()}, m_size{size_t(str.size()) | Tag::Latin1} {}

constexpr QLatin1StringView QAnyStringView::asLatin1StringView() const
{
    Q_ASSERT(isLatin1());
    return {m_data_utf8, size()};
}

template <typename Visitor>
constexpr decltype(auto) QAnyStringView::visit(Visitor &&v) const
{
    if (isUtf16())
        return std::forward<Visitor>(v)(asStringView());
    else if (isLatin1())
        return std::forward<Visitor>(v)(asLatin1StringView());
    else
        return std::forward<Visitor>(v)(asUtf8StringView());
}

//
// QAnyStringView members that require QAnyStringView::visit()
//

constexpr QChar QAnyStringView::front() const
{
    return visit([] (auto that) { return QAnyStringView::toQChar(that.front()); });
}
constexpr QChar QAnyStringView::back() const
{
    return visit([] (auto that) { return QAnyStringView::toQChar(that.back()); });
}


class Q_CORE_EXPORT QString
{
    typedef QTypedArrayData<char16_t> Data;
public:
    typedef QStringPrivate DataPointer;

    inline constexpr QString() noexcept;
    explicit QString(const QChar *unicode, qsizetype size = -1);
    QString(QChar c);
    QString(qsizetype size, QChar c);
    inline QString(QLatin1StringView latin1);
#if defined(__cpp_char8_t) || defined(Q_CLANG_QDOC)
    Q_WEAK_OVERLOAD
    inline QString(const char8_t *str)
        : QString(fromUtf8(str))
    {}
#endif
    inline QString(const QString &) noexcept;
    inline ~QString();
    QString &operator=(QChar c);
    QString &operator=(const QString &) noexcept;
    QString &operator=(QLatin1StringView latin1);
    inline QString(QString &&other) noexcept
        = default;
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QString)
    void swap(QString &other) noexcept { d.swap(other.d); }
    inline qsizetype size() const { return d.size; }
#if QT_DEPRECATED_SINCE(6, 4)
    QT_DEPRECATED_VERSION_X_6_4("Use size() or length() instead.")
    inline qsizetype count() const { return d.size; }
#endif
    inline qsizetype length() const { return d.size; }
    inline bool isEmpty() const;
    void resize(qsizetype size);
    void resize(qsizetype size, QChar fillChar);

    QString &fill(QChar c, qsizetype size = -1);
    void truncate(qsizetype pos);
    void chop(qsizetype n);

    inline qsizetype capacity() const;
    inline void reserve(qsizetype size);
    inline void squeeze();

    inline const QChar *unicode() const;
    inline QChar *data();
    inline const QChar *data() const;
    inline const QChar *constData() const;

    inline void detach();
    inline bool isDetached() const;
    inline bool isSharedWith(const QString &other) const { return d.isSharedWith(other.d); }
    void clear();

    inline const QChar at(qsizetype i) const;
    const QChar operator[](qsizetype i) const;
    [[nodiscard]] QChar &operator[](qsizetype i);

    [[nodiscard]] inline QChar front() const { return at(0); }
    [[nodiscard]] inline QChar &front();
    [[nodiscard]] inline QChar back() const { return at(size() - 1); }
    [[nodiscard]] inline QChar &back();

    [[nodiscard]] QString arg(qlonglong a, int fieldwidth=0, int base=10,
                QChar fillChar = u' ') const;
    [[nodiscard]] QString arg(qulonglong a, int fieldwidth=0, int base=10,
                QChar fillChar = u' ') const;
    [[nodiscard]] QString arg(long a, int fieldwidth=0, int base=10,
                QChar fillChar = u' ') const;
    [[nodiscard]] QString arg(ulong a, int fieldwidth=0, int base=10,
                QChar fillChar = u' ') const;
    [[nodiscard]] QString arg(int a, int fieldWidth = 0, int base = 10,
                QChar fillChar = u' ') const;
    [[nodiscard]] QString arg(uint a, int fieldWidth = 0, int base = 10,
                QChar fillChar = u' ') const;
    [[nodiscard]] QString arg(short a, int fieldWidth = 0, int base = 10,
                QChar fillChar = u' ') const;
    [[nodiscard]] QString arg(ushort a, int fieldWidth = 0, int base = 10,
                QChar fillChar = u' ') const;
    [[nodiscard]] QString arg(double a, int fieldWidth = 0, char format = 'g', int precision = -1,
                QChar fillChar = u' ') const;
    [[nodiscard]] QString arg(char a, int fieldWidth = 0,
                QChar fillChar = u' ') const;
    [[nodiscard]] QString arg(QChar a, int fieldWidth = 0,
                QChar fillChar = u' ') const;
    [[nodiscard]] QString arg(const QString &a, int fieldWidth = 0,
                QChar fillChar = u' ') const;
    [[nodiscard]] QString arg(QStringView a, int fieldWidth = 0,
                QChar fillChar = u' ') const;
    [[nodiscard]] QString arg(QLatin1StringView a, int fieldWidth = 0,
                QChar fillChar = u' ') const;
private:
    template <typename T>
    struct is_convertible_to_view_or_qstring_helper
        : std::integral_constant<bool,
            std::is_convertible<T, QString>::value ||
            std::is_convertible<T, QStringView>::value ||
            std::is_convertible<T, QLatin1StringView>::value> {};
    template <typename T>
    struct is_convertible_to_view_or_qstring
        : is_convertible_to_view_or_qstring_helper<typename std::decay<T>::type> {};
public:
    template <typename...Args>
    [[nodiscard]]
#ifdef Q_CLANG_QDOC
    QString
#else
    typename std::enable_if<
        sizeof...(Args) >= 2 && std::is_same<
            QtPrivate::BoolList<is_convertible_to_view_or_qstring<Args>::value..., true>,
            QtPrivate::BoolList<true, is_convertible_to_view_or_qstring<Args>::value...>
        >::value,
        QString
    >::type
#endif
    arg(Args &&...args) const
    { return qToStringViewIgnoringNull(*this).arg(std::forward<Args>(args)...); }

    static QString vasprintf(const char *format, va_list ap) Q_ATTRIBUTE_FORMAT_PRINTF(1, 0);
    static QString asprintf(const char *format, ...) Q_ATTRIBUTE_FORMAT_PRINTF(1, 2);

    [[nodiscard]] qsizetype indexOf(QChar c, qsizetype from = 0, Qt::CaseSensitivity cs = Qt::CaseSensitive) const;
    [[nodiscard]] qsizetype indexOf(QLatin1StringView s, qsizetype from = 0, Qt::CaseSensitivity cs = Qt::CaseSensitive) const;
    [[nodiscard]] qsizetype indexOf(const QString &s, qsizetype from = 0, Qt::CaseSensitivity cs = Qt::CaseSensitive) const;
    [[nodiscard]] qsizetype indexOf(QStringView s, qsizetype from = 0, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return QtPrivate::findString(*this, from, s, cs); }
    [[nodiscard]] qsizetype lastIndexOf(QChar c, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return lastIndexOf(c, -1, cs); }
    [[nodiscard]] qsizetype lastIndexOf(QChar c, qsizetype from, Qt::CaseSensitivity cs = Qt::CaseSensitive) const;
    [[nodiscard]] qsizetype lastIndexOf(QLatin1StringView s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const
    { return lastIndexOf(s, size(), cs); }
    [[nodiscard]] qsizetype lastIndexOf(QLatin1StringView s, qsizetype from, Qt::CaseSensitivity cs = Qt::CaseSensitive) const;
    [[nodiscard]] qsizetype lastIndexOf(const QString &s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const
    { return lastIndexOf(s, size(), cs); }
    [[nodiscard]] qsizetype lastIndexOf(const QString &s, qsizetype from, Qt::CaseSensitivity cs = Qt::CaseSensitive) const;

    [[nodiscard]] qsizetype lastIndexOf(QStringView s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return lastIndexOf(s, size(), cs); }
    [[nodiscard]] qsizetype lastIndexOf(QStringView s, qsizetype from, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return QtPrivate::lastIndexOf(*this, from, s, cs); }

    [[nodiscard]] inline bool contains(QChar c, Qt::CaseSensitivity cs = Qt::CaseSensitive) const;
    [[nodiscard]] inline bool contains(const QString &s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const;
    [[nodiscard]] inline bool contains(QLatin1StringView s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const;
    [[nodiscard]] inline bool contains(QStringView s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept;
    [[nodiscard]] qsizetype count(QChar c, Qt::CaseSensitivity cs = Qt::CaseSensitive) const;
    [[nodiscard]] qsizetype count(const QString &s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const;
    [[nodiscard]] qsizetype count(QStringView s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const;

#if QT_CONFIG(regularexpression)
    [[nodiscard]] qsizetype indexOf(const QRegularExpression &re, qsizetype from = 0,
                                    QRegularExpressionMatch *rmatch = nullptr) const;
#ifdef Q_QDOC
    [[nodiscard]] qsizetype lastIndexOf(const QRegularExpression &re, QRegularExpressionMatch *rmatch = nullptr) const;
#else
    // prevent an ambiguity when called like this: lastIndexOf(re, 0)
    template <typename T = QRegularExpressionMatch, std::enable_if_t<std::is_same_v<T, QRegularExpressionMatch>, bool> = false>
    [[nodiscard]] qsizetype lastIndexOf(const QRegularExpression &re, T *rmatch = nullptr) const
    { return lastIndexOf(re, size(), rmatch); }
#endif
    [[nodiscard]] qsizetype lastIndexOf(const QRegularExpression &re, qsizetype from,
                                        QRegularExpressionMatch *rmatch = nullptr) const;
    [[nodiscard]] bool contains(const QRegularExpression &re, QRegularExpressionMatch *rmatch = nullptr) const;
    [[nodiscard]] qsizetype count(const QRegularExpression &re) const;
#endif

    enum SectionFlag {
        SectionDefault             = 0x00,
        SectionSkipEmpty           = 0x01,
        SectionIncludeLeadingSep   = 0x02,
        SectionIncludeTrailingSep  = 0x04,
        SectionCaseInsensitiveSeps = 0x08
    };
    Q_DECLARE_FLAGS(SectionFlags, SectionFlag)

    [[nodiscard]] QString section(QChar sep, qsizetype start, qsizetype end = -1, SectionFlags flags = SectionDefault) const;
    [[nodiscard]] QString section(const QString &in_sep, qsizetype start, qsizetype end = -1, SectionFlags flags = SectionDefault) const;
#if QT_CONFIG(regularexpression)
    [[nodiscard]] QString section(const QRegularExpression &re, qsizetype start, qsizetype end = -1, SectionFlags flags = SectionDefault) const;
#endif
    [[nodiscard]] QString left(qsizetype n) const;
    [[nodiscard]] QString right(qsizetype n) const;
    [[nodiscard]] QString mid(qsizetype position, qsizetype n = -1) const;

    [[nodiscard]] QString first(qsizetype n) const
    { Q_ASSERT(n >= 0); Q_ASSERT(n <= size()); return QString(data(), n); }
    [[nodiscard]] QString last(qsizetype n) const
    { Q_ASSERT(n >= 0); Q_ASSERT(n <= size()); return QString(data() + size() - n, n); }
    [[nodiscard]] QString sliced(qsizetype pos) const
    { Q_ASSERT(pos >= 0); Q_ASSERT(pos <= size()); return QString(data() + pos, size() - pos); }
    [[nodiscard]] QString sliced(qsizetype pos, qsizetype n) const
    { Q_ASSERT(pos >= 0); Q_ASSERT(n >= 0); Q_ASSERT(size_t(pos) + size_t(n) <= size_t(size())); return QString(data() + pos, n); }
    [[nodiscard]] QString chopped(qsizetype n) const
    { Q_ASSERT(n >= 0); Q_ASSERT(n <= size()); return first(size() - n); }


    bool startsWith(const QString &s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const;
    [[nodiscard]] bool startsWith(QStringView s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return QtPrivate::startsWith(*this, s, cs); }
    bool startsWith(QLatin1StringView s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const;
    bool startsWith(QChar c, Qt::CaseSensitivity cs = Qt::CaseSensitive) const;

    bool endsWith(const QString &s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const;
    [[nodiscard]] bool endsWith(QStringView s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return QtPrivate::endsWith(*this, s, cs); }
    bool endsWith(QLatin1StringView s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const;
    bool endsWith(QChar c, Qt::CaseSensitivity cs = Qt::CaseSensitive) const;

    bool isUpper() const;
    bool isLower() const;

    [[nodiscard]] QString leftJustified(qsizetype width, QChar fill = u' ', bool trunc = false) const;
    [[nodiscard]] QString rightJustified(qsizetype width, QChar fill = u' ', bool trunc = false) const;

#if !defined(Q_CLANG_QDOC)
    [[nodiscard]] QString toLower() const &
    { return toLower_helper(*this); }
    [[nodiscard]] QString toLower() &&
    { return toLower_helper(*this); }
    [[nodiscard]] QString toUpper() const &
    { return toUpper_helper(*this); }
    [[nodiscard]] QString toUpper() &&
    { return toUpper_helper(*this); }
    [[nodiscard]] QString toCaseFolded() const &
    { return toCaseFolded_helper(*this); }
    [[nodiscard]] QString toCaseFolded() &&
    { return toCaseFolded_helper(*this); }
    [[nodiscard]] QString trimmed() const &
    { return trimmed_helper(*this); }
    [[nodiscard]] QString trimmed() &&
    { return trimmed_helper(*this); }
    [[nodiscard]] QString simplified() const &
    { return simplified_helper(*this); }
    [[nodiscard]] QString simplified() &&
    { return simplified_helper(*this); }
#else
    [[nodiscard]] QString toLower() const;
    [[nodiscard]] QString toUpper() const;
    [[nodiscard]] QString toCaseFolded() const;
    [[nodiscard]] QString trimmed() const;
    [[nodiscard]] QString simplified() const;
#endif
    [[nodiscard]] QString toHtmlEscaped() const;

    QString &insert(qsizetype i, QChar c);
    QString &insert(qsizetype i, const QChar *uc, qsizetype len);
    inline QString &insert(qsizetype i, const QString &s) { return insert(i, s.constData(), s.size()); }
    inline QString &insert(qsizetype i, QStringView v) { return insert(i, v.data(), v.size()); }
    QString &insert(qsizetype i, QLatin1StringView s);

    QString &append(QChar c);
    QString &append(const QChar *uc, qsizetype len);
    QString &append(const QString &s);
    inline QString &append(QStringView v) { return append(v.data(), v.size()); }
    QString &append(QLatin1StringView s);

    inline QString &prepend(QChar c) { return insert(0, c); }
    inline QString &prepend(const QChar *uc, qsizetype len) { return insert(0, uc, len); }
    inline QString &prepend(const QString &s) { return insert(0, s); }
    inline QString &prepend(QStringView v) { return prepend(v.data(), v.size()); }
    inline QString &prepend(QLatin1StringView s) { return insert(0, s); }

    inline QString &operator+=(QChar c) { return append(c); }

    inline QString &operator+=(const QString &s) { return append(s); }
    inline QString &operator+=(QStringView v) { return append(v); }
    inline QString &operator+=(QLatin1StringView s) { return append(s); }

    QString &remove(qsizetype i, qsizetype len);
    QString &remove(QChar c, Qt::CaseSensitivity cs = Qt::CaseSensitive);
    QString &remove(QLatin1StringView s, Qt::CaseSensitivity cs = Qt::CaseSensitive);
    QString &remove(const QString &s, Qt::CaseSensitivity cs = Qt::CaseSensitive);
    template <typename Predicate>
    QString &removeIf(Predicate pred)
    {
        QtPrivate::sequential_erase_if(*this, pred);
        return *this;
    }
    QString &replace(qsizetype i, qsizetype len, QChar after);
    QString &replace(qsizetype i, qsizetype len, const QChar *s, qsizetype slen);
    QString &replace(qsizetype i, qsizetype len, const QString &after);
    QString &replace(QChar before, QChar after, Qt::CaseSensitivity cs = Qt::CaseSensitive);
    QString &replace(const QChar *before, qsizetype blen, const QChar *after, qsizetype alen, Qt::CaseSensitivity cs = Qt::CaseSensitive);
    QString &replace(QLatin1StringView before, QLatin1StringView after, Qt::CaseSensitivity cs = Qt::CaseSensitive);
    QString &replace(QLatin1StringView before, const QString &after, Qt::CaseSensitivity cs = Qt::CaseSensitive);
    QString &replace(const QString &before, QLatin1StringView after, Qt::CaseSensitivity cs = Qt::CaseSensitive);
    QString &replace(const QString &before, const QString &after,
                     Qt::CaseSensitivity cs = Qt::CaseSensitive);
    QString &replace(QChar c, const QString &after, Qt::CaseSensitivity cs = Qt::CaseSensitive);
    QString &replace(QChar c, QLatin1StringView after, Qt::CaseSensitivity cs = Qt::CaseSensitive);
#if QT_CONFIG(regularexpression)
    QString &replace(const QRegularExpression &re, const QString  &after);
    inline QString &remove(const QRegularExpression &re)
    { return replace(re, QString()); }
#endif

public:
    [[nodiscard]]
    QStringList split(const QString &sep, Qt::SplitBehavior behavior = Qt::KeepEmptyParts,
                      Qt::CaseSensitivity cs = Qt::CaseSensitive) const;
    [[nodiscard]]
    QStringList split(QChar sep, Qt::SplitBehavior behavior = Qt::KeepEmptyParts,
                      Qt::CaseSensitivity cs = Qt::CaseSensitive) const;
#ifndef QT_NO_REGULAREXPRESSION
    [[nodiscard]]
    QStringList split(const QRegularExpression &sep,
                      Qt::SplitBehavior behavior = Qt::KeepEmptyParts) const;
#endif

    template <typename Needle, typename...Flags>
    [[nodiscard]] inline auto tokenize(Needle &&needle, Flags...flags) const &
        noexcept(noexcept(qTokenize(std::declval<const QString &>(), std::forward<Needle>(needle), flags...)))
            -> decltype(qTokenize(*this, std::forward<Needle>(needle), flags...))
    { return qTokenize(qToStringViewIgnoringNull(*this), std::forward<Needle>(needle), flags...); }

    template <typename Needle, typename...Flags>
    [[nodiscard]] inline auto tokenize(Needle &&needle, Flags...flags) const &&
        noexcept(noexcept(qTokenize(std::declval<const QString>(), std::forward<Needle>(needle), flags...)))
            -> decltype(qTokenize(std::move(*this), std::forward<Needle>(needle), flags...))
    { return qTokenize(std::move(*this), std::forward<Needle>(needle), flags...); }

    template <typename Needle, typename...Flags>
    [[nodiscard]] inline auto tokenize(Needle &&needle, Flags...flags) &&
        noexcept(noexcept(qTokenize(std::declval<QString>(), std::forward<Needle>(needle), flags...)))
            -> decltype(qTokenize(std::move(*this), std::forward<Needle>(needle), flags...))
    { return qTokenize(std::move(*this), std::forward<Needle>(needle), flags...); }


    enum NormalizationForm {
        NormalizationForm_D,
        NormalizationForm_C,
        NormalizationForm_KD,
        NormalizationForm_KC
    };
    [[nodiscard]] QString normalized(NormalizationForm mode, QChar::UnicodeVersion version = QChar::Unicode_Unassigned) const;

    [[nodiscard]] QString repeated(qsizetype times) const;

    const ushort *utf16() const; // ### Qt 7 char16_t

#if !defined(Q_CLANG_QDOC)
    [[nodiscard]] QByteArray toLatin1() const &
    { return toLatin1_helper(*this); }
    [[nodiscard]] QByteArray toLatin1() &&
    { return toLatin1_helper_inplace(*this); }
    [[nodiscard]] QByteArray toUtf8() const &
    { return toUtf8_helper(*this); }
    [[nodiscard]] QByteArray toUtf8() &&
    { return toUtf8_helper(*this); }
    [[nodiscard]] QByteArray toLocal8Bit() const &
    { return toLocal8Bit_helper(isNull() ? nullptr : constData(), size()); }
    [[nodiscard]] QByteArray toLocal8Bit() &&
    { return toLocal8Bit_helper(isNull() ? nullptr : constData(), size()); }
#else
    [[nodiscard]] QByteArray toLatin1() const;
    [[nodiscard]] QByteArray toUtf8() const;
    [[nodiscard]] QByteArray toLocal8Bit() const;
#endif
    [[nodiscard]] QList<uint> toUcs4() const; // ### Qt 7 char32_t

    // note - this are all inline so we can benefit from strlen() compile time optimizations
    static QString fromLatin1(QByteArrayView ba);
    Q_WEAK_OVERLOAD
    static inline QString fromLatin1(const QByteArray &ba) { return fromLatin1(QByteArrayView(ba)); }
    static inline QString fromLatin1(const char *str, qsizetype size)
    {
        return fromLatin1(QByteArrayView(str, !str || size < 0 ? qstrlen(str) : size));
    }
    static QString fromUtf8(QByteArrayView utf8);
    Q_WEAK_OVERLOAD
    static inline QString fromUtf8(const QByteArray &ba) { return fromUtf8(QByteArrayView(ba)); }
    static inline QString fromUtf8(const char *utf8, qsizetype size)
    {
        return fromUtf8(QByteArrayView(utf8, !utf8 || size < 0 ? qstrlen(utf8) : size));
    }
#if defined(__cpp_char8_t) || defined(Q_CLANG_QDOC)
    Q_WEAK_OVERLOAD
    static inline QString fromUtf8(const char8_t *str)
    { return fromUtf8(reinterpret_cast<const char *>(str)); }
    Q_WEAK_OVERLOAD
    static inline QString fromUtf8(const char8_t *str, qsizetype size)
    { return fromUtf8(reinterpret_cast<const char *>(str), size); }
#endif
    static QString fromLocal8Bit(QByteArrayView ba);
    Q_WEAK_OVERLOAD
    static inline QString fromLocal8Bit(const QByteArray &ba) { return fromLocal8Bit(QByteArrayView(ba)); }
    static inline QString fromLocal8Bit(const char *str, qsizetype size)
    {
        return fromLocal8Bit(QByteArrayView(str, !str || size < 0 ? qstrlen(str) : size));
    }
    static QString fromUtf16(const char16_t *, qsizetype size = -1);
    static QString fromUcs4(const char32_t *, qsizetype size = -1);
    static QString fromRawData(const QChar *, qsizetype size);

#if QT_DEPRECATED_SINCE(6, 0)
    QT_DEPRECATED_VERSION_X_6_0("Use char16_t* overload.")
    static QString fromUtf16(const ushort *str, qsizetype size = -1)
    { return fromUtf16(reinterpret_cast<const char16_t *>(str), size); }
    QT_DEPRECATED_VERSION_X_6_0("Use char32_t* overload.")
    static QString fromUcs4(const uint *str, qsizetype size = -1)
    { return fromUcs4(reinterpret_cast<const char32_t *>(str), size); }
#endif

    inline qsizetype toWCharArray(wchar_t *array) const;
    [[nodiscard]] static inline QString fromWCharArray(const wchar_t *string, qsizetype size = -1);

    QString &setRawData(const QChar *unicode, qsizetype size);
    QString &setUnicode(const QChar *unicode, qsizetype size);
    inline QString &setUtf16(const ushort *utf16, qsizetype size); // ### Qt 7 char16_t

    int compare(const QString &s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept;
    int compare(QLatin1StringView other, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept;
    inline int compare(QStringView s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept;
    int compare(QChar ch, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept
    { return compare(QStringView{&ch, 1}, cs); }

    static inline int compare(const QString &s1, const QString &s2,
                              Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept
    { return s1.compare(s2, cs); }

    static inline int compare(const QString &s1, QLatin1StringView s2,
                              Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept
    { return s1.compare(s2, cs); }
    static inline int compare(QLatin1StringView s1, const QString &s2,
                              Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept
    { return -s2.compare(s1, cs); }
    static int compare(const QString &s1, QStringView s2, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept
    { return s1.compare(s2, cs); }
    static int compare(QStringView s1, const QString &s2, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept
    { return -s2.compare(s1, cs); }

    int localeAwareCompare(const QString& s) const;
    int localeAwareCompare(QStringView s) const;
    static int localeAwareCompare(const QString& s1, const QString& s2)
    { return s1.localeAwareCompare(s2); }

    static int localeAwareCompare(QStringView s1, QStringView s2);

    short toShort(bool *ok=nullptr, int base=10) const
    { return toIntegral_helper<short>(*this, ok, base); }
    ushort toUShort(bool *ok=nullptr, int base=10) const
    { return toIntegral_helper<ushort>(*this, ok, base); }
    int toInt(bool *ok=nullptr, int base=10) const
    { return toIntegral_helper<int>(*this, ok, base); }
    uint toUInt(bool *ok=nullptr, int base=10) const
    { return toIntegral_helper<uint>(*this, ok, base); }
    long toLong(bool *ok=nullptr, int base=10) const
    { return toIntegral_helper<long>(*this, ok, base); }
    ulong toULong(bool *ok=nullptr, int base=10) const
    { return toIntegral_helper<ulong>(*this, ok, base); }
    qlonglong toLongLong(bool *ok=nullptr, int base=10) const;
    qulonglong toULongLong(bool *ok=nullptr, int base=10) const;
    float toFloat(bool *ok=nullptr) const;
    double toDouble(bool *ok=nullptr) const;

    QString &setNum(short, int base=10);
    QString &setNum(ushort, int base=10);
    QString &setNum(int, int base=10);
    QString &setNum(uint, int base=10);
    QString &setNum(long, int base=10);
    QString &setNum(ulong, int base=10);
    QString &setNum(qlonglong, int base=10);
    QString &setNum(qulonglong, int base=10);
    QString &setNum(float, char format='g', int precision=6);
    QString &setNum(double, char format='g', int precision=6);

    static QString number(int, int base=10);
    static QString number(uint, int base=10);
    static QString number(long, int base=10);
    static QString number(ulong, int base=10);
    static QString number(qlonglong, int base=10);
    static QString number(qulonglong, int base=10);
    static QString number(double, char format='g', int precision=6);

    friend bool operator==(const QString &s1, const QString &s2) noexcept
    { return (s1.size() == s2.size()) && QtPrivate::compareStrings(s1, s2, Qt::CaseSensitive) == 0; }
    friend bool operator< (const QString &s1, const QString &s2) noexcept
    { return QtPrivate::compareStrings(s1, s2, Qt::CaseSensitive) < 0; }
    friend bool operator> (const QString &s1, const QString &s2) noexcept { return s2 < s1; }
    friend bool operator!=(const QString &s1, const QString &s2) noexcept { return !(s1 == s2); }
    friend bool operator<=(const QString &s1, const QString &s2) noexcept { return !(s1 > s2); }
    friend bool operator>=(const QString &s1, const QString &s2) noexcept { return !(s1 < s2); }

    friend bool operator==(const QString &s1, QLatin1StringView s2) noexcept
    { return (s1.size() == s2.size()) && QtPrivate::compareStrings(s1, s2, Qt::CaseSensitive) == 0; }
    friend bool operator< (const QString &s1, QLatin1StringView s2) noexcept
    { return QtPrivate::compareStrings(s1, s2, Qt::CaseSensitive) < 0; }
    friend bool operator> (const QString &s1, QLatin1StringView s2) noexcept
    { return QtPrivate::compareStrings(s1, s2, Qt::CaseSensitive) > 0; }
    friend bool operator!=(const QString &s1, QLatin1StringView s2) noexcept { return !(s1 == s2); }
    friend bool operator<=(const QString &s1, QLatin1StringView s2) noexcept { return !(s1 > s2); }
    friend bool operator>=(const QString &s1, QLatin1StringView s2) noexcept { return !(s1 < s2); }

    friend bool operator==(QLatin1StringView s1, const QString &s2) noexcept { return s2 == s1; }
    friend bool operator< (QLatin1StringView s1, const QString &s2) noexcept { return s2 > s1; }
    friend bool operator> (QLatin1StringView s1, const QString &s2) noexcept { return s2 < s1; }
    friend bool operator!=(QLatin1StringView s1, const QString &s2) noexcept { return s2 != s1; }
    friend bool operator<=(QLatin1StringView s1, const QString &s2) noexcept { return s2 >= s1; }
    friend bool operator>=(QLatin1StringView s1, const QString &s2) noexcept { return s2 <= s1; }

    // Check isEmpty() instead of isNull() for backwards compatibility.
    friend bool operator==(const QString &s1, std::nullptr_t) noexcept { return s1.isEmpty(); }
    friend bool operator!=(const QString &s1, std::nullptr_t) noexcept { return !s1.isEmpty(); }
    friend bool operator< (const QString &  , std::nullptr_t) noexcept { return false; }
    friend bool operator> (const QString &s1, std::nullptr_t) noexcept { return !s1.isEmpty(); }
    friend bool operator<=(const QString &s1, std::nullptr_t) noexcept { return s1.isEmpty(); }
    friend bool operator>=(const QString &  , std::nullptr_t) noexcept { return true; }
    friend bool operator==(std::nullptr_t, const QString &s2) noexcept { return s2 == nullptr; }
    friend bool operator!=(std::nullptr_t, const QString &s2) noexcept { return s2 != nullptr; }
    friend bool operator< (std::nullptr_t, const QString &s2) noexcept { return s2 >  nullptr; }
    friend bool operator> (std::nullptr_t, const QString &s2) noexcept { return s2 <  nullptr; }
    friend bool operator<=(std::nullptr_t, const QString &s2) noexcept { return s2 >= nullptr; }
    friend bool operator>=(std::nullptr_t, const QString &s2) noexcept { return s2 <= nullptr; }

    friend bool operator==(const QString &s1, const char16_t *s2) noexcept { return s1 == QStringView(s2); }
    friend bool operator!=(const QString &s1, const char16_t *s2) noexcept { return s1 != QStringView(s2); }
    friend bool operator< (const QString &s1, const char16_t *s2) noexcept { return s1 <  QStringView(s2); }
    friend bool operator> (const QString &s1, const char16_t *s2) noexcept { return s1 >  QStringView(s2); }
    friend bool operator<=(const QString &s1, const char16_t *s2) noexcept { return s1 <= QStringView(s2); }
    friend bool operator>=(const QString &s1, const char16_t *s2) noexcept { return s1 >= QStringView(s2); }

    friend bool operator==(const char16_t *s1, const QString &s2) noexcept { return s2 == s1; }
    friend bool operator!=(const char16_t *s1, const QString &s2) noexcept { return s2 != s1; }
    friend bool operator< (const char16_t *s1, const QString &s2) noexcept { return s2 >  s1; }
    friend bool operator> (const char16_t *s1, const QString &s2) noexcept { return s2 <  s1; }
    friend bool operator<=(const char16_t *s1, const QString &s2) noexcept { return s2 >= s1; }
    friend bool operator>=(const char16_t *s1, const QString &s2) noexcept { return s2 <= s1; }

    // QChar <> QString
    friend inline bool operator==(QChar lhs, const QString &rhs) noexcept
    { return rhs.size() == 1 && lhs == rhs.front(); }
    friend inline bool operator< (QChar lhs, const QString &rhs) noexcept
    { return compare_helper(&lhs, 1, rhs.data(), rhs.size()) < 0; }
    friend inline bool operator> (QChar lhs, const QString &rhs) noexcept
    { return compare_helper(&lhs, 1, rhs.data(), rhs.size()) > 0; }

    friend inline bool operator!=(QChar lhs, const QString &rhs) noexcept { return !(lhs == rhs); }
    friend inline bool operator<=(QChar lhs, const QString &rhs) noexcept { return !(lhs >  rhs); }
    friend inline bool operator>=(QChar lhs, const QString &rhs) noexcept { return !(lhs <  rhs); }

    friend inline bool operator==(const QString &lhs, QChar rhs) noexcept { return   rhs == lhs; }
    friend inline bool operator!=(const QString &lhs, QChar rhs) noexcept { return !(rhs == lhs); }
    friend inline bool operator< (const QString &lhs, QChar rhs) noexcept { return   rhs >  lhs; }
    friend inline bool operator> (const QString &lhs, QChar rhs) noexcept { return   rhs <  lhs; }
    friend inline bool operator<=(const QString &lhs, QChar rhs) noexcept { return !(rhs <  lhs); }
    friend inline bool operator>=(const QString &lhs, QChar rhs) noexcept { return !(rhs >  lhs); }

    // ASCII compatibility
#if defined(QT_RESTRICTED_CAST_FROM_ASCII)
    template <qsizetype N>
    inline QString(const char (&ch)[N])
        : QString(fromUtf8(ch))
    {}
    template <qsizetype N>
    QString(char (&)[N]) = delete;
    template <qsizetype N>
    inline QString &operator=(const char (&ch)[N])
    { return (*this = fromUtf8(ch, N - 1)); }
    template <qsizetype N>
    QString &operator=(char (&)[N]) = delete;
#endif
#if !defined(QT_NO_CAST_FROM_ASCII) && !defined(QT_RESTRICTED_CAST_FROM_ASCII)
    QT_ASCII_CAST_WARN inline QString(const char *ch)
        : QString(fromUtf8(ch))
    {}
    QT_ASCII_CAST_WARN inline QString(const QByteArray &a)
        : QString(fromUtf8(a))
    {}
    QT_ASCII_CAST_WARN inline QString &operator=(const char *ch)
    { return (*this = fromUtf8(ch)); }
    QT_ASCII_CAST_WARN inline QString &operator=(const QByteArray &a)
    { return (*this = fromUtf8(a)); }

    // these are needed, so it compiles with STL support enabled
    QT_ASCII_CAST_WARN inline QString &prepend(const char *s)
    { return prepend(QString::fromUtf8(s)); }
    QT_ASCII_CAST_WARN inline QString &prepend(const QByteArray &s)
    { return prepend(QString::fromUtf8(s)); }
    QT_ASCII_CAST_WARN inline QString &append(const char *s)
    { return append(QString::fromUtf8(s)); }
    QT_ASCII_CAST_WARN inline QString &append(const QByteArray &s)
    { return append(QString::fromUtf8(s)); }
    QT_ASCII_CAST_WARN inline QString &insert(qsizetype i, const char *s)
    { return insert(i, QString::fromUtf8(s)); }
    QT_ASCII_CAST_WARN inline QString &insert(qsizetype i, const QByteArray &s)
    { return insert(i, QString::fromUtf8(s)); }
    QT_ASCII_CAST_WARN inline QString &operator+=(const char *s)
    { return append(QString::fromUtf8(s)); }
    QT_ASCII_CAST_WARN inline QString &operator+=(const QByteArray &s)
    { return append(QString::fromUtf8(s)); }

    QT_ASCII_CAST_WARN inline bool operator==(const char *s) const;
    QT_ASCII_CAST_WARN inline bool operator!=(const char *s) const;
    QT_ASCII_CAST_WARN inline bool operator<(const char *s) const;
    QT_ASCII_CAST_WARN inline bool operator<=(const char *s) const;
    QT_ASCII_CAST_WARN inline bool operator>(const char *s) const;
    QT_ASCII_CAST_WARN inline bool operator>=(const char *s) const;

    QT_ASCII_CAST_WARN inline bool operator==(const QByteArray &s) const;
    QT_ASCII_CAST_WARN inline bool operator!=(const QByteArray &s) const;
    QT_ASCII_CAST_WARN inline bool operator<(const QByteArray &s) const;
    QT_ASCII_CAST_WARN inline bool operator>(const QByteArray &s) const;
    QT_ASCII_CAST_WARN inline bool operator<=(const QByteArray &s) const;
    QT_ASCII_CAST_WARN inline bool operator>=(const QByteArray &s) const;

    QT_ASCII_CAST_WARN friend bool operator==(const char *s1, const QString &s2)
    { return QString::compare_helper(s2.constData(), s2.size(), s1, -1) == 0; }
    QT_ASCII_CAST_WARN friend bool operator!=(const char *s1, const QString &s2)
    { return QString::compare_helper(s2.constData(), s2.size(), s1, -1) != 0; }
    QT_ASCII_CAST_WARN friend bool operator< (const char *s1, const QString &s2)
    { return QString::compare_helper(s2.constData(), s2.size(), s1, -1) > 0; }
    QT_ASCII_CAST_WARN friend bool operator> (const char *s1, const QString &s2)
    { return QString::compare_helper(s2.constData(), s2.size(), s1, -1) < 0; }
    QT_ASCII_CAST_WARN friend bool operator<=(const char *s1, const QString &s2)
    { return QString::compare_helper(s2.constData(), s2.size(), s1, -1) >= 0; }
    QT_ASCII_CAST_WARN friend bool operator>=(const char *s1, const QString &s2)
    { return QString::compare_helper(s2.constData(), s2.size(), s1, -1) <= 0; }
#endif

    typedef QChar *iterator;
    typedef const QChar *const_iterator;
    typedef iterator Iterator;
    typedef const_iterator ConstIterator;
    typedef std::reverse_iterator<iterator> reverse_iterator;
    typedef std::reverse_iterator<const_iterator> const_reverse_iterator;
    inline iterator begin();
    inline const_iterator begin() const;
    inline const_iterator cbegin() const;
    inline const_iterator constBegin() const;
    inline iterator end();
    inline const_iterator end() const;
    inline const_iterator cend() const;
    inline const_iterator constEnd() const;
    reverse_iterator rbegin() { return reverse_iterator(end()); }
    reverse_iterator rend() { return reverse_iterator(begin()); }
    const_reverse_iterator rbegin() const { return const_reverse_iterator(end()); }
    const_reverse_iterator rend() const { return const_reverse_iterator(begin()); }
    const_reverse_iterator crbegin() const { return const_reverse_iterator(end()); }
    const_reverse_iterator crend() const { return const_reverse_iterator(begin()); }

    // STL compatibility
    typedef qsizetype size_type;
    typedef qptrdiff difference_type;
    typedef const QChar & const_reference;
    typedef QChar & reference;
    typedef QChar *pointer;
    typedef const QChar *const_pointer;
    typedef QChar value_type;
    inline void push_back(QChar c) { append(c); }
    inline void push_back(const QString &s) { append(s); }
    inline void push_front(QChar c) { prepend(c); }
    inline void push_front(const QString &s) { prepend(s); }
    void shrink_to_fit() { squeeze(); }
    iterator erase(const_iterator first, const_iterator last);

    static inline QString fromStdString(const std::string &s);
    inline std::string toStdString() const;
    static inline QString fromStdWString(const std::wstring &s);
    inline std::wstring toStdWString() const;

    static inline QString fromStdU16String(const std::u16string &s);
    inline std::u16string toStdU16String() const;
    static inline QString fromStdU32String(const std::u32string &s);
    inline std::u32string toStdU32String() const;

#if defined(Q_OS_DARWIN) || defined(Q_QDOC)
    static QString fromCFString(CFStringRef string);
    CFStringRef toCFString() const Q_DECL_CF_RETURNS_RETAINED;
    static QString fromNSString(const NSString *string);
    NSString *toNSString() const Q_DECL_NS_RETURNS_AUTORELEASED;
#endif

    inline bool isNull() const { return d->isNull(); }


    bool isSimpleText() const;
    bool isRightToLeft() const;
    [[nodiscard]] bool isValidUtf16() const noexcept
    { return QStringView(*this).isValidUtf16(); }

    QString(qsizetype size, Qt::Initialization);
    explicit QString(DataPointer &&dd) : d(std::move(dd)) {}

private:
#if defined(QT_NO_CAST_FROM_ASCII)
    QString &operator+=(const char *s);
    QString &operator+=(const QByteArray &s);
    QString(const char *ch);
    QString(const QByteArray &a);
    QString &operator=(const char  *ch);
    QString &operator=(const QByteArray &a);
#endif

    DataPointer d;
    static const char16_t _empty;

    void reallocData(qsizetype alloc, QArrayData::AllocationOption option);
    void reallocGrowData(qsizetype n);
    static int compare_helper(const QChar *data1, qsizetype length1,
                              const QChar *data2, qsizetype length2,
                              Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;
    static int compare_helper(const QChar *data1, qsizetype length1,
                              const char *data2, qsizetype length2,
                              Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;
    static int localeAwareCompare_helper(const QChar *data1, qsizetype length1,
                                         const QChar *data2, qsizetype length2);
    static QString toLower_helper(const QString &str);
    static QString toLower_helper(QString &str);
    static QString toUpper_helper(const QString &str);
    static QString toUpper_helper(QString &str);
    static QString toCaseFolded_helper(const QString &str);
    static QString toCaseFolded_helper(QString &str);
    static QString trimmed_helper(const QString &str);
    static QString trimmed_helper(QString &str);
    static QString simplified_helper(const QString &str);
    static QString simplified_helper(QString &str);
    static QByteArray toLatin1_helper(const QString &);
    static QByteArray toLatin1_helper_inplace(QString &);
    static QByteArray toUtf8_helper(const QString &);
    static QByteArray toLocal8Bit_helper(const QChar *data, qsizetype size);
    static qsizetype toUcs4_helper(const ushort *uc, qsizetype length, uint *out); // ### Qt 7 char16_t
    static qlonglong toIntegral_helper(QStringView string, bool *ok, int base);
    static qulonglong toIntegral_helper(QStringView string, bool *ok, uint base);
    void replace_helper(size_t *indices, qsizetype nIndices, qsizetype blen, const QChar *after, qsizetype alen);
    friend class QStringView;
    friend class QByteArray;
    friend struct QAbstractConcatenable;

    template <typename T> static
    T toIntegral_helper(QStringView string, bool *ok, int base)
    {
        using Int64 = typename std::conditional<std::is_unsigned<T>::value, qulonglong, qlonglong>::type;
        using Int32 = typename std::conditional<std::is_unsigned<T>::value, uint, int>::type;

        // we select the right overload by casting base to int or uint
        Int64 val = toIntegral_helper(string, ok, Int32(base));
        if (T(val) != val) {
            if (ok)
                *ok = false;
            val = 0;
        }
        return T(val);
    }

public:
    inline DataPointer &data_ptr() { return d; }
    inline const DataPointer &data_ptr() const { return d; }
};

//
// QLatin1StringView inline members that require QString:
//

QString QLatin1StringView::toString() const { return *this; }

//
// QStringView inline members that require QString:
//

QString QStringView::toString() const
{ return Q_ASSERT(size() == size()), QString(data(), size()); }

qint64 QStringView::toLongLong(bool *ok, int base) const
{ return QString::toIntegral_helper<qint64>(*this, ok, base); }
quint64 QStringView::toULongLong(bool *ok, int base) const
{ return QString::toIntegral_helper<quint64>(*this, ok, base); }
long QStringView::toLong(bool *ok, int base) const
{ return QString::toIntegral_helper<long>(*this, ok, base); }
ulong QStringView::toULong(bool *ok, int base) const
{ return QString::toIntegral_helper<ulong>(*this, ok, base); }
int QStringView::toInt(bool *ok, int base) const
{ return QString::toIntegral_helper<int>(*this, ok, base); }
uint QStringView::toUInt(bool *ok, int base) const
{ return QString::toIntegral_helper<uint>(*this, ok, base); }
short QStringView::toShort(bool *ok, int base) const
{ return QString::toIntegral_helper<short>(*this, ok, base); }
ushort QStringView::toUShort(bool *ok, int base) const
{ return QString::toIntegral_helper<ushort>(*this, ok, base); }

//
// QUtf8StringView inline members that require QString:
//

template <bool UseChar8T>
QString QBasicUtf8StringView<UseChar8T>::toString() const
{
    return QString::fromUtf8(data(), size());
}

//
// QAnyStringView inline members that require QString:
//

QAnyStringView::QAnyStringView(const QByteArray &str) noexcept
    : QAnyStringView{str.isNull() ? nullptr : str.data(), str.size()} {}
QAnyStringView::QAnyStringView(const QString &str) noexcept
    : QAnyStringView{str.isNull() ? nullptr : str.data(), str.size()} {}

QString QAnyStringView::toString() const
{ return QtPrivate::convertToQString(*this); }

//
// QString inline members
//
inline QString::QString(QLatin1StringView latin1)
{ *this = QString::fromLatin1(latin1.data(), latin1.size()); }
inline const QChar QString::at(qsizetype i) const
{ Q_ASSERT(size_t(i) < size_t(size())); return QChar(d.data()[i]); }
inline const QChar QString::operator[](qsizetype i) const
{ Q_ASSERT(size_t(i) < size_t(size())); return QChar(d.data()[i]); }
inline bool QString::isEmpty() const
{ return d.size == 0; }
inline const QChar *QString::unicode() const
{ return data(); }
inline const QChar *QString::data() const
{
#if QT5_NULL_STRINGS == 1
    return reinterpret_cast<const QChar *>(d.data() ? d.data() : &_empty);
#else
    return reinterpret_cast<const QChar *>(d.data());
#endif
}
inline QChar *QString::data()
{
    detach();
    Q_ASSERT(d.data());
    return reinterpret_cast<QChar *>(d.data());
}
inline const QChar *QString::constData() const
{ return data(); }
inline void QString::detach()
{ if (d->needsDetach()) reallocData(d.size, QArrayData::KeepSize); }
inline bool QString::isDetached() const
{ return !d->isShared(); }
inline void QString::clear()
{ if (!isNull()) *this = QString(); }
inline QString::QString(const QString &other) noexcept : d(other.d)
{ }
inline qsizetype QString::capacity() const { return qsizetype(d->constAllocatedCapacity()); }
inline QString &QString::setNum(short n, int base)
{ return setNum(qlonglong(n), base); }
inline QString &QString::setNum(ushort n, int base)
{ return setNum(qulonglong(n), base); }
inline QString &QString::setNum(int n, int base)
{ return setNum(qlonglong(n), base); }
inline QString &QString::setNum(uint n, int base)
{ return setNum(qulonglong(n), base); }
inline QString &QString::setNum(long n, int base)
{ return setNum(qlonglong(n), base); }
inline QString &QString::setNum(ulong n, int base)
{ return setNum(qulonglong(n), base); }
inline QString &QString::setNum(float n, char f, int prec)
{ return setNum(double(n),f,prec); }
inline QString QString::arg(int a, int fieldWidth, int base, QChar fillChar) const
{ return arg(qlonglong(a), fieldWidth, base, fillChar); }
inline QString QString::arg(uint a, int fieldWidth, int base, QChar fillChar) const
{ return arg(qulonglong(a), fieldWidth, base, fillChar); }
inline QString QString::arg(long a, int fieldWidth, int base, QChar fillChar) const
{ return arg(qlonglong(a), fieldWidth, base, fillChar); }
inline QString QString::arg(ulong a, int fieldWidth, int base, QChar fillChar) const
{ return arg(qulonglong(a), fieldWidth, base, fillChar); }
inline QString QString::arg(short a, int fieldWidth, int base, QChar fillChar) const
{ return arg(qlonglong(a), fieldWidth, base, fillChar); }
inline QString QString::arg(ushort a, int fieldWidth, int base, QChar fillChar) const
{ return arg(qulonglong(a), fieldWidth, base, fillChar); }

inline QString QString::section(QChar asep, qsizetype astart, qsizetype aend, SectionFlags aflags) const
{ return section(QString(asep), astart, aend, aflags); }

QT_WARNING_PUSH
QT_WARNING_DISABLE_MSVC(4127)   // "conditional expression is constant"
QT_WARNING_DISABLE_INTEL(111)   // "statement is unreachable"

inline qsizetype QString::toWCharArray(wchar_t *array) const
{
    return qToStringViewIgnoringNull(*this).toWCharArray(array);
}

qsizetype QStringView::toWCharArray(wchar_t *array) const
{
    if (sizeof(wchar_t) == sizeof(QChar)) {
        if (auto src = data())
            memcpy(array, src, sizeof(QChar) * size());
        return size();
    } else {
        return QString::toUcs4_helper(reinterpret_cast<const ushort *>(data()), size(),
                                      reinterpret_cast<uint *>(array));
    }
}

QT_WARNING_POP

inline QString QString::fromWCharArray(const wchar_t *string, qsizetype size)
{
    return sizeof(wchar_t) == sizeof(QChar) ? fromUtf16(reinterpret_cast<const char16_t *>(string), size)
                                            : fromUcs4(reinterpret_cast<const char32_t *>(string), size);
}

inline constexpr QString::QString() noexcept {}
inline QString::~QString() {}

inline void QString::reserve(qsizetype asize)
{
    if (d->needsDetach() || asize >= capacity() - d.freeSpaceAtBegin())
        reallocData(qMax(asize, size()), QArrayData::KeepSize);
    if (d->constAllocatedCapacity())
        d->setFlag(Data::CapacityReserved);
}

inline void QString::squeeze()
{
    if (!d.isMutable())
        return;
    if (d->needsDetach() || size() < capacity())
        reallocData(d.size, QArrayData::KeepSize);
    if (d->constAllocatedCapacity())
        d->clearFlag(Data::CapacityReserved);
}

inline QString &QString::setUtf16(const ushort *autf16, qsizetype asize)
{ return setUnicode(reinterpret_cast<const QChar *>(autf16), asize); }
inline QChar &QString::operator[](qsizetype i)
{ Q_ASSERT(i >= 0 && i < size()); return data()[i]; }
inline QChar &QString::front() { return operator[](0); }
inline QChar &QString::back() { return operator[](size() - 1); }
inline QString::iterator QString::begin()
{ detach(); return reinterpret_cast<QChar*>(d.data()); }
inline QString::const_iterator QString::begin() const
{ return reinterpret_cast<const QChar*>(d.data()); }
inline QString::const_iterator QString::cbegin() const
{ return reinterpret_cast<const QChar*>(d.data()); }
inline QString::const_iterator QString::constBegin() const
{ return reinterpret_cast<const QChar*>(d.data()); }
inline QString::iterator QString::end()
{ detach(); return reinterpret_cast<QChar*>(d.data() + d.size); }
inline QString::const_iterator QString::end() const
{ return reinterpret_cast<const QChar*>(d.data() + d.size); }
inline QString::const_iterator QString::cend() const
{ return reinterpret_cast<const QChar*>(d.data() + d.size); }
inline QString::const_iterator QString::constEnd() const
{ return reinterpret_cast<const QChar*>(d.data() + d.size); }
inline bool QString::contains(const QString &s, Qt::CaseSensitivity cs) const
{ return indexOf(s, 0, cs) != -1; }
inline bool QString::contains(QLatin1StringView s, Qt::CaseSensitivity cs) const
{ return indexOf(s, 0, cs) != -1; }
inline bool QString::contains(QChar c, Qt::CaseSensitivity cs) const
{ return indexOf(c, 0, cs) != -1; }
inline bool QString::contains(QStringView s, Qt::CaseSensitivity cs) const noexcept
{ return indexOf(s, 0, cs) != -1; }

#if !defined(QT_NO_CAST_FROM_ASCII) && !defined(QT_RESTRICTED_CAST_FROM_ASCII)
inline bool QString::operator==(const char *s) const
{ return QString::compare_helper(constData(), size(), s, -1) == 0; }
inline bool QString::operator!=(const char *s) const
{ return QString::compare_helper(constData(), size(), s, -1) != 0; }
inline bool QString::operator<(const char *s) const
{ return QString::compare_helper(constData(), size(), s, -1) < 0; }
inline bool QString::operator>(const char *s) const
{ return QString::compare_helper(constData(), size(), s, -1) > 0; }
inline bool QString::operator<=(const char *s) const
{ return QString::compare_helper(constData(), size(), s, -1) <= 0; }
inline bool QString::operator>=(const char *s) const
{ return QString::compare_helper(constData(), size(), s, -1) >= 0; }

QT_ASCII_CAST_WARN inline bool QLatin1StringView::operator==(const char *s) const
{ return QString::fromUtf8(s) == *this; }
QT_ASCII_CAST_WARN inline bool QLatin1StringView::operator!=(const char *s) const
{ return QString::fromUtf8(s) != *this; }
QT_ASCII_CAST_WARN inline bool QLatin1StringView::operator<(const char *s) const
{ return QString::fromUtf8(s) > *this; }
QT_ASCII_CAST_WARN inline bool QLatin1StringView::operator>(const char *s) const
{ return QString::fromUtf8(s) < *this; }
QT_ASCII_CAST_WARN inline bool QLatin1StringView::operator<=(const char *s) const
{ return QString::fromUtf8(s) >= *this; }
QT_ASCII_CAST_WARN inline bool QLatin1StringView::operator>=(const char *s) const
{ return QString::fromUtf8(s) <= *this; }

QT_ASCII_CAST_WARN inline bool QLatin1StringView::operator==(const QByteArray &s) const
{ return QString::fromUtf8(s) == *this; }
QT_ASCII_CAST_WARN inline bool QLatin1StringView::operator!=(const QByteArray &s) const
{ return QString::fromUtf8(s) != *this; }
QT_ASCII_CAST_WARN inline bool QLatin1StringView::operator<(const QByteArray &s) const
{ return QString::fromUtf8(s) > *this; }
QT_ASCII_CAST_WARN inline bool QLatin1StringView::operator>(const QByteArray &s) const
{ return QString::fromUtf8(s) < *this; }
QT_ASCII_CAST_WARN inline bool QLatin1StringView::operator<=(const QByteArray &s) const
{ return QString::fromUtf8(s) >= *this; }
QT_ASCII_CAST_WARN inline bool QLatin1StringView::operator>=(const QByteArray &s) const
{ return QString::fromUtf8(s) <= *this; }

inline int QLatin1StringView::compare_helper(const QLatin1StringView &s1, const char *s2)
{
    return QString::compare(s1, QString::fromUtf8(s2));
}

QT_ASCII_CAST_WARN inline bool QString::operator==(const QByteArray &s) const
{ return QString::compare_helper(constData(), size(), s.constData(), s.size()) == 0; }
QT_ASCII_CAST_WARN inline bool QString::operator!=(const QByteArray &s) const
{ return QString::compare_helper(constData(), size(), s.constData(), s.size()) != 0; }
QT_ASCII_CAST_WARN inline bool QString::operator<(const QByteArray &s) const
{ return QString::compare_helper(constData(), size(), s.constData(), s.size()) < 0; }
QT_ASCII_CAST_WARN inline bool QString::operator>(const QByteArray &s) const
{ return QString::compare_helper(constData(), size(), s.constData(), s.size()) > 0; }
QT_ASCII_CAST_WARN inline bool QString::operator<=(const QByteArray &s) const
{ return QString::compare_helper(constData(), size(), s.constData(), s.size()) <= 0; }
QT_ASCII_CAST_WARN inline bool QString::operator>=(const QByteArray &s) const
{ return QString::compare_helper(constData(), size(), s.constData(), s.size()) >= 0; }

inline bool QByteArray::operator==(const QString &s) const
{ return QString::compare_helper(s.constData(), s.size(), constData(), size()) == 0; }
inline bool QByteArray::operator!=(const QString &s) const
{ return QString::compare_helper(s.constData(), s.size(), constData(), size()) != 0; }
inline bool QByteArray::operator<(const QString &s) const
{ return QString::compare_helper(s.constData(), s.size(), constData(), size()) > 0; }
inline bool QByteArray::operator>(const QString &s) const
{ return QString::compare_helper(s.constData(), s.size(), constData(), size()) < 0; }
inline bool QByteArray::operator<=(const QString &s) const
{ return QString::compare_helper(s.constData(), s.size(), constData(), size()) >= 0; }
inline bool QByteArray::operator>=(const QString &s) const
{ return QString::compare_helper(s.constData(), s.size(), constData(), size()) <= 0; }
#endif // !defined(QT_NO_CAST_FROM_ASCII) && !defined(QT_RESTRICTED_CAST_FROM_ASCII)

#if !defined(QT_USE_FAST_OPERATOR_PLUS) && !defined(QT_USE_QSTRINGBUILDER)
inline const QString operator+(const QString &s1, const QString &s2)
{ QString t(s1); t += s2; return t; }
inline const QString operator+(const QString &s1, QChar s2)
{ QString t(s1); t += s2; return t; }
inline const QString operator+(QChar s1, const QString &s2)
{ QString t(s1); t += s2; return t; }
#  if !defined(QT_NO_CAST_FROM_ASCII) && !defined(QT_RESTRICTED_CAST_FROM_ASCII)
QT_ASCII_CAST_WARN inline const QString operator+(const QString &s1, const char *s2)
{ QString t(s1); t += QString::fromUtf8(s2); return t; }
QT_ASCII_CAST_WARN inline const QString operator+(const char *s1, const QString &s2)
{ QString t = QString::fromUtf8(s1); t += s2; return t; }
QT_ASCII_CAST_WARN inline const QString operator+(const QByteArray &ba, const QString &s)
{ QString t = QString::fromUtf8(ba); t += s; return t; }
QT_ASCII_CAST_WARN inline const QString operator+(const QString &s, const QByteArray &ba)
{ QString t(s); t += QString::fromUtf8(ba); return t; }
#  endif // QT_NO_CAST_FROM_ASCII
#endif // QT_USE_QSTRINGBUILDER

inline std::string QString::toStdString() const
{ return toUtf8().toStdString(); }

inline QString QString::fromStdString(const std::string &s)
{ return fromUtf8(s.data(), qsizetype(s.size())); }

inline std::wstring QString::toStdWString() const
{
    std::wstring str;
    str.resize(size());
    str.resize(toWCharArray(str.data()));
    return str;
}

inline QString QString::fromStdWString(const std::wstring &s)
{ return fromWCharArray(s.data(), qsizetype(s.size())); }

inline QString QString::fromStdU16String(const std::u16string &s)
{ return fromUtf16(s.data(), qsizetype(s.size())); }

inline std::u16string QString::toStdU16String() const
{ return std::u16string(reinterpret_cast<const char16_t*>(data()), size()); }

inline QString QString::fromStdU32String(const std::u32string &s)
{ return fromUcs4(s.data(), qsizetype(s.size())); }

inline std::u32string QString::toStdU32String() const
{
    std::u32string u32str(size(), char32_t(0));
    qsizetype len = toUcs4_helper(reinterpret_cast<const ushort *>(constData()),
                                  size(), reinterpret_cast<uint*>(&u32str[0]));
    u32str.resize(len);
    return u32str;
}

#if !defined(QT_NO_DATASTREAM) || defined(QT_BOOTSTRAPPED)
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QString &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QString &);
#endif

Q_DECLARE_SHARED(QString)
Q_DECLARE_OPERATORS_FOR_FLAGS(QString::SectionFlags)

inline int QString::compare(QStringView s, Qt::CaseSensitivity cs) const noexcept
{ return -s.compare(*this, cs); }

inline int QString::localeAwareCompare(QStringView s) const
{ return localeAwareCompare_helper(constData(), size(), s.constData(), s.size()); }
inline int QString::localeAwareCompare(QStringView s1, QStringView s2)
{ return localeAwareCompare_helper(s1.constData(), s1.size(), s2.constData(), s2.size()); }
inline int QStringView::localeAwareCompare(QStringView other) const
{ return QString::localeAwareCompare(*this, other); }

namespace QtPrivate {
// used by qPrintable() and qUtf8Printable() macros
inline const QString &asString(const QString &s)    { return s; }
inline QString &&asString(QString &&s)              { return std::move(s); }
}

//
// QStringView::arg() implementation
//

namespace QtPrivate {

struct ArgBase {
    enum Tag : uchar { L1, U8, U16 } tag;
};

struct QStringViewArg : ArgBase {
    QStringView string;
    QStringViewArg() = default;
    constexpr explicit QStringViewArg(QStringView v) noexcept : ArgBase{U16}, string{v} {}
};

struct QLatin1StringArg : ArgBase {
    QLatin1StringView string;
    QLatin1StringArg() = default;
    constexpr explicit QLatin1StringArg(QLatin1StringView v) noexcept : ArgBase{L1}, string{v} {}
};

[[nodiscard]] Q_CORE_EXPORT QString argToQString(QStringView pattern, size_t n, const ArgBase **args);
[[nodiscard]] Q_CORE_EXPORT QString argToQString(QLatin1StringView pattern, size_t n, const ArgBase **args);

template <typename StringView, typename...Args>
[[nodiscard]] Q_ALWAYS_INLINE QString argToQStringDispatch(StringView pattern, const Args &...args)
{
    const ArgBase *argBases[] = {&args..., /* avoid zero-sized array */ nullptr};
    return QtPrivate::argToQString(pattern, sizeof...(Args), argBases);
}

          inline QStringViewArg   qStringLikeToArg(const QString &s) noexcept { return QStringViewArg{qToStringViewIgnoringNull(s)}; }
constexpr inline QStringViewArg   qStringLikeToArg(QStringView s) noexcept { return QStringViewArg{s}; }
          inline QStringViewArg   qStringLikeToArg(const QChar &c) noexcept { return QStringViewArg{QStringView{&c, 1}}; }
constexpr inline QLatin1StringArg qStringLikeToArg(QLatin1StringView s) noexcept { return QLatin1StringArg{s}; }

} // namespace QtPrivate

template <typename...Args>
Q_ALWAYS_INLINE
QString QStringView::arg(Args &&...args) const
{
    return QtPrivate::argToQStringDispatch(*this, QtPrivate::qStringLikeToArg(args)...);
}

template <typename...Args>
Q_ALWAYS_INLINE
QString QLatin1StringView::arg(Args &&...args) const
{
    return QtPrivate::argToQStringDispatch(*this, QtPrivate::qStringLikeToArg(args)...);
}

template <typename T>
qsizetype erase(QString &s, const T &t)
{
    return QtPrivate::sequential_erase(s, t);
}

template <typename Predicate>
qsizetype erase_if(QString &s, Predicate pred)
{
    return QtPrivate::sequential_erase_if(s, pred);
}

namespace Qt {
inline namespace Literals {
inline namespace StringLiterals {

constexpr inline QLatin1StringView operator"" _L1(const char *str, size_t size) noexcept
{
    return {str, qsizetype(size)};
}

inline QString operator"" _s(const char16_t *str, size_t size) noexcept
{
    return QString(QStringPrivate(nullptr, const_cast<char16_t *>(str), qsizetype(size)));
}

} // StringLiterals
} // Literals
} // Qt

inline namespace QtLiterals {
#if QT_DEPRECATED_SINCE(6, 8)

QT_DEPRECATED_VERSION_X_6_8("Use _s from Qt::StringLiterals namespace instead.")
inline QString operator"" _qs(const char16_t *str, size_t size) noexcept
{
    return Qt::StringLiterals::operator""_s(str, size);
}

#endif // QT_DEPRECATED_SINCE(6, 8)
} // QtLiterals

QT_END_NAMESPACE

#if defined(QT_USE_FAST_OPERATOR_PLUS) || defined(QT_USE_QSTRINGBUILDER)
#include <QtCore/qstringbuilder.h>
#endif

#ifdef Q_L1S_VIEW_IS_PRIMARY
#    undef Q_L1S_VIEW_IS_PRIMARY
#endif

#endif // QSTRING_H
