// Copyright (C) 2020 Klarälvdalens Datakonsult AB, a KDAB Group company, info@kdab.com, author Marc Mutz <marc.mutz@kdab.com>
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only
#ifndef QUTF8STRINGVIEW_H
#define QUTF8STRINGVIEW_H

#include <QtCore/qstringalgorithms.h>
#include <QtCore/qstringfwd.h>
#include <QtCore/qarraydata.h> // for QContainerImplHelper
#include <QtCore/qbytearrayview.h>

#include <string>

QT_BEGIN_NAMESPACE

namespace QtPrivate {
template <typename Char>
using IsCompatibleChar8TypeHelper = std::disjunction<
#ifdef __cpp_char8_t
        std::is_same<Char, char8_t>,
#endif
        std::is_same<Char, char>,
        std::is_same<Char, uchar>,
        std::is_same<Char, signed char>
    >;
template <typename Char>
using IsCompatibleChar8Type
    = IsCompatibleChar8TypeHelper<std::remove_cv_t<std::remove_reference_t<Char>>>;

template <typename Pointer>
struct IsCompatiblePointer8Helper : std::false_type {};
template <typename Char>
struct IsCompatiblePointer8Helper<Char*>
    : IsCompatibleChar8Type<Char> {};
template <typename Pointer>
using IsCompatiblePointer8
    = IsCompatiblePointer8Helper<std::remove_cv_t<std::remove_reference_t<Pointer>>>;

template <typename T, typename Enable = void>
struct IsContainerCompatibleWithQUtf8StringView : std::false_type {};

template <typename T>
struct IsContainerCompatibleWithQUtf8StringView<T, std::enable_if_t<std::conjunction_v<
        // lacking concepts and ranges, we accept any T whose std::data yields a suitable pointer ...
        IsCompatiblePointer8<decltype(std::data(std::declval<const T &>()))>,
        // ... and that has a suitable size ...
        std::is_convertible<
            decltype(std::size(std::declval<const T &>())),
            qsizetype
        >,
        // ... and it's a range as it defines an iterator-like API
        IsCompatibleChar8Type<typename std::iterator_traits<
            decltype(std::begin(std::declval<const T &>()))>::value_type
        >,
        std::is_convertible<
            decltype( std::begin(std::declval<const T &>()) != std::end(std::declval<const T &>()) ),
            bool
        >,

        // This needs to be treated specially due to the empty vs null distinction
        std::negation<std::is_same<std::decay_t<T>, QByteArray>>,

        // This has a compatible value_type, but explicitly a different encoding
        std::negation<std::is_same<std::decay_t<T>, QLatin1StringView>>,

        // Don't make an accidental copy constructor
        std::negation<std::disjunction<
            std::is_same<std::decay_t<T>, QBasicUtf8StringView<true>>,
            std::is_same<std::decay_t<T>, QBasicUtf8StringView<false>>
        >>
    >>> : std::true_type {};

struct hide_char8_t {
#ifdef __cpp_char8_t
    using type = char8_t;
#endif
};

struct wrap_char { using type = char; };

} // namespace QtPrivate

#ifdef Q_CLANG_QDOC
#define QBasicUtf8StringView QUtf8StringView
#else
template <bool UseChar8T>
#endif
class QBasicUtf8StringView
{
public:
#ifndef Q_CLANG_QDOC
    using storage_type = typename std::conditional<UseChar8T,
            QtPrivate::hide_char8_t,
            QtPrivate::wrap_char
        >::type::type;
#else
    using storage_type = typename QtPrivate::hide_char8_t;
#endif
    typedef const storage_type value_type;
    typedef qptrdiff difference_type;
    typedef qsizetype size_type;
    typedef value_type &reference;
    typedef value_type &const_reference;
    typedef value_type *pointer;
    typedef value_type *const_pointer;

    typedef pointer iterator;
    typedef const_pointer const_iterator;
    typedef std::reverse_iterator<iterator> reverse_iterator;
    typedef std::reverse_iterator<const_iterator> const_reverse_iterator;

private:
    template <typename Char>
    using if_compatible_char = std::enable_if_t<QtPrivate::IsCompatibleChar8Type<Char>::value, bool>;

    template <typename Pointer>
    using if_compatible_pointer = std::enable_if_t<QtPrivate::IsCompatiblePointer8<Pointer>::value, bool>;

    template <typename T>
    using if_compatible_qstring_like = std::enable_if_t<std::is_same_v<T, QByteArray>, bool>;

    template <typename T>
    using if_compatible_container = std::enable_if_t<QtPrivate::IsContainerCompatibleWithQUtf8StringView<T>::value, bool>;

    template <typename Container>
    static constexpr qsizetype lengthHelperContainer(const Container &c) noexcept
    {
        return qsizetype(std::size(c));
    }

    // Note: Do not replace with std::size(const Char (&)[N]), cause the result
    // will be of by one.
    template <typename Char, size_t N>
    static constexpr qsizetype lengthHelperContainer(const Char (&str)[N]) noexcept
    {
        const auto it = std::char_traits<Char>::find(str, N, Char(0));
        const auto end = it ? it : std::next(str, N);
        return qsizetype(std::distance(str, end));
    }

    template <typename Char>
    static const storage_type *castHelper(const Char *str) noexcept
    { return reinterpret_cast<const storage_type*>(str); }
    static constexpr const storage_type *castHelper(const storage_type *str) noexcept
    { return str; }

public:
    constexpr QBasicUtf8StringView() noexcept
        : m_data(nullptr), m_size(0) {}
    constexpr QBasicUtf8StringView(std::nullptr_t) noexcept
        : QBasicUtf8StringView() {}

    template <typename Char, if_compatible_char<Char> = true>
    constexpr QBasicUtf8StringView(const Char *str, qsizetype len)
        : m_data(castHelper(str)),
          m_size((Q_ASSERT(len >= 0), Q_ASSERT(str || !len), len)) {}

    template <typename Char, if_compatible_char<Char> = true>
    constexpr QBasicUtf8StringView(const Char *f, const Char *l)
        : QBasicUtf8StringView(f, l - f) {}

#ifdef Q_CLANG_QDOC
    template <typename Char, size_t N>
    constexpr QBasicUtf8StringView(const Char (&array)[N]) noexcept;

    template <typename Char>
    constexpr QBasicUtf8StringView(const Char *str) noexcept;
#else
    template <typename Pointer, if_compatible_pointer<Pointer> = true>
    constexpr QBasicUtf8StringView(const Pointer &str) noexcept
        : QBasicUtf8StringView(str,
            str ? std::char_traits<std::remove_cv_t<std::remove_pointer_t<Pointer>>>::length(str) : 0) {}
#endif

#ifdef Q_CLANG_QDOC
    QBasicUtf8StringView(const QByteArray &str) noexcept;
#else
    template <typename String, if_compatible_qstring_like<String> = true>
    QBasicUtf8StringView(const String &str) noexcept
        : QBasicUtf8StringView(str.isNull() ? nullptr : str.data(), qsizetype(str.size())) {}
#endif

    template <typename Container, if_compatible_container<Container> = true>
    constexpr QBasicUtf8StringView(const Container &c) noexcept
        : QBasicUtf8StringView(std::data(c), lengthHelperContainer(c)) {}

#ifdef __cpp_char8_t
    constexpr QBasicUtf8StringView(QBasicUtf8StringView<!UseChar8T> other)
        : QBasicUtf8StringView(other.data(), other.size()) {}
#endif

    template <typename Char, size_t Size, if_compatible_char<Char> = true>
    [[nodiscard]] constexpr static QBasicUtf8StringView fromArray(const Char (&string)[Size]) noexcept
    { return QBasicUtf8StringView(string, Size); }

    [[nodiscard]] inline QString toString() const; // defined in qstring.h

    [[nodiscard]] constexpr qsizetype size() const noexcept { return m_size; }
    [[nodiscard]] const_pointer data() const noexcept { return reinterpret_cast<const_pointer>(m_data); }
#if defined(__cpp_char8_t) || defined(Q_CLANG_QDOC)
    [[nodiscard]] const char8_t *utf8() const noexcept { return reinterpret_cast<const char8_t*>(m_data); }
#endif

    [[nodiscard]] constexpr storage_type operator[](qsizetype n) const
    { return Q_ASSERT(n >= 0), Q_ASSERT(n < size()), m_data[n]; }

    //
    // QString API
    //

    [[nodiscard]] constexpr storage_type at(qsizetype n) const { return (*this)[n]; }

    [[nodiscard]]
    constexpr QBasicUtf8StringView mid(qsizetype pos, qsizetype n = -1) const
    {
        using namespace QtPrivate;
        auto result = QContainerImplHelper::mid(size(), &pos, &n);
        return result == QContainerImplHelper::Null ? QBasicUtf8StringView() : QBasicUtf8StringView(m_data + pos, n);
    }
    [[nodiscard]]
    constexpr QBasicUtf8StringView left(qsizetype n) const
    {
        if (size_t(n) >= size_t(size()))
            n = size();
        return QBasicUtf8StringView(m_data, n);
    }
    [[nodiscard]]
    constexpr QBasicUtf8StringView right(qsizetype n) const
    {
        if (size_t(n) >= size_t(size()))
            n = size();
        return QBasicUtf8StringView(m_data + m_size - n, n);
    }

    [[nodiscard]] constexpr QBasicUtf8StringView sliced(qsizetype pos) const
    { verify(pos); return QBasicUtf8StringView{m_data + pos, m_size - pos}; }
    [[nodiscard]] constexpr QBasicUtf8StringView sliced(qsizetype pos, qsizetype n) const
    { verify(pos, n); return QBasicUtf8StringView(m_data + pos, n); }
    [[nodiscard]] constexpr QBasicUtf8StringView first(qsizetype n) const
    { verify(n); return QBasicUtf8StringView(m_data, n); }
    [[nodiscard]] constexpr QBasicUtf8StringView last(qsizetype n) const
    { verify(n); return QBasicUtf8StringView(m_data + m_size - n, n); }
    [[nodiscard]] constexpr QBasicUtf8StringView chopped(qsizetype n) const
    { verify(n); return QBasicUtf8StringView(m_data, m_size - n); }

    constexpr void truncate(qsizetype n)
    { verify(n); m_size = n; }
    constexpr void chop(qsizetype n)
    { verify(n); m_size -= n; }

    [[nodiscard]] inline bool isValidUtf8() const noexcept
    {
        return QByteArrayView(reinterpret_cast<const char *>(data()), size()).isValidUtf8();
    }

    //
    // STL compatibility API:
    //
    [[nodiscard]] const_iterator begin()   const noexcept { return data(); }
    [[nodiscard]] const_iterator end()     const noexcept { return data() + size(); }
    [[nodiscard]] const_iterator cbegin()  const noexcept { return begin(); }
    [[nodiscard]] const_iterator cend()    const noexcept { return end(); }
    [[nodiscard]] const_reverse_iterator rbegin()  const noexcept { return const_reverse_iterator(end()); }
    [[nodiscard]] const_reverse_iterator rend()    const noexcept { return const_reverse_iterator(begin()); }
    [[nodiscard]] const_reverse_iterator crbegin() const noexcept { return rbegin(); }
    [[nodiscard]] const_reverse_iterator crend()   const noexcept { return rend(); }

    [[nodiscard]] constexpr bool empty() const noexcept { return size() == 0; }
    [[nodiscard]] constexpr storage_type front() const { return Q_ASSERT(!empty()), m_data[0]; }
    [[nodiscard]] constexpr storage_type back()  const { return Q_ASSERT(!empty()), m_data[m_size - 1]; }

    //
    // Qt compatibility API:
    //
    [[nodiscard]] constexpr bool isNull() const noexcept { return !m_data; }
    [[nodiscard]] constexpr bool isEmpty() const noexcept { return empty(); }
    [[nodiscard]] constexpr qsizetype length() const noexcept
    { return size(); }

private:
    [[nodiscard]] static inline int compare(QBasicUtf8StringView lhs, QBasicUtf8StringView rhs) noexcept
    {
        return QtPrivate::compareStrings(QBasicUtf8StringView<false>(lhs.data(), lhs.size()),
                                         QBasicUtf8StringView<false>(rhs.data(), rhs.size()));
    }

    [[nodiscard]] friend inline bool operator==(QBasicUtf8StringView lhs, QBasicUtf8StringView rhs) noexcept
    {
        return lhs.size() == rhs.size()
               && QtPrivate::equalStrings(QBasicUtf8StringView<false>(lhs.data(), lhs.size()),
                                          QBasicUtf8StringView<false>(rhs.data(), rhs.size()));
    }
    [[nodiscard]] friend inline bool operator!=(QBasicUtf8StringView lhs, QBasicUtf8StringView rhs) noexcept
    { return !operator==(lhs, rhs); }

#ifdef __cpp_impl_three_way_comparison
    [[nodiscard]] friend inline auto operator<=>(QBasicUtf8StringView lhs, QBasicUtf8StringView rhs) noexcept
    { return QBasicUtf8StringView::compare(lhs, rhs) <=> 0; }
#else
    [[nodiscard]] friend inline bool operator<=(QBasicUtf8StringView lhs, QBasicUtf8StringView rhs) noexcept
    { return QBasicUtf8StringView::compare(lhs, rhs) <= 0; }
    [[nodiscard]] friend inline bool operator>=(QBasicUtf8StringView lhs, QBasicUtf8StringView rhs) noexcept
    { return QBasicUtf8StringView::compare(lhs, rhs) >= 0; }
    [[nodiscard]] friend inline bool operator<(QBasicUtf8StringView lhs, QBasicUtf8StringView rhs) noexcept
    { return QBasicUtf8StringView::compare(lhs, rhs) < 0; }
    [[nodiscard]] friend inline bool operator>(QBasicUtf8StringView lhs, QBasicUtf8StringView rhs) noexcept
    { return QBasicUtf8StringView::compare(lhs, rhs) > 0; }
#endif

    Q_ALWAYS_INLINE constexpr void verify(qsizetype pos, qsizetype n = 0) const
    {
        Q_ASSERT(pos >= 0);
        Q_ASSERT(pos <= size());
        Q_ASSERT(n >= 0);
        Q_ASSERT(n <= size() - pos);
    }
    const storage_type *m_data;
    qsizetype m_size;
};

#ifdef Q_CLANG_QDOC
#undef QBasicUtf8StringView
#else
template <bool UseChar8T>
Q_DECLARE_TYPEINFO_BODY(QBasicUtf8StringView<UseChar8T>, Q_PRIMITIVE_TYPE);
#endif // Q_CLANG_QDOC

template <typename QStringLike, std::enable_if_t<std::is_same_v<QStringLike, QByteArray>, bool> = true>
[[nodiscard]] inline q_no_char8_t::QUtf8StringView qToUtf8StringViewIgnoringNull(const QStringLike &s) noexcept
{ return q_no_char8_t::QUtf8StringView(s.data(), s.size()); }

QT_END_NAMESPACE

#endif /* QUTF8STRINGVIEW_H */
