// Copyright (C) 2021 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only
#ifndef QBYTEARRAYVIEW_H
#define QBYTEARRAYVIEW_H

#include <QtCore/qbytearrayalgorithms.h>
#include <QtCore/qstringfwd.h>

#include <string>

QT_BEGIN_NAMESPACE

namespace QtPrivate {

template <typename Byte>
struct IsCompatibleByteTypeHelper
    : std::integral_constant<bool,
                             std::is_same_v<Byte, char> ||
                             std::is_same_v<Byte, uchar> ||
                             std::is_same_v<Byte, signed char> ||
                             std::is_same_v<Byte, std::byte>> {};

template <typename Byte>
struct IsCompatibleByteType
    : IsCompatibleByteTypeHelper<
              typename std::remove_cv_t<typename std::remove_reference_t<Byte>>> {};

template <typename Pointer>
struct IsCompatibleByteArrayPointerHelper : std::false_type {};
template <typename Byte>
struct IsCompatibleByteArrayPointerHelper<Byte *>
    : IsCompatibleByteType<Byte> {};
template<typename Pointer>
struct IsCompatibleByteArrayPointer
    : IsCompatibleByteArrayPointerHelper<
              typename std::remove_cv_t<typename std::remove_reference_t<Pointer>>> {};

template <typename T, typename Enable = void>
struct IsContainerCompatibleWithQByteArrayView : std::false_type {};

template <typename T>
struct IsContainerCompatibleWithQByteArrayView<T, std::enable_if_t<
        std::conjunction_v<
                // lacking concepts and ranges, we accept any T whose std::data yields a suitable
                // pointer ...
                IsCompatibleByteArrayPointer<decltype(std::data(std::declval<const T &>()))>,
                // ... and that has a suitable size ...
                std::is_convertible<decltype(std::size(std::declval<const T &>())), qsizetype>,
                // ... and it's a range as it defines an iterator-like API
                IsCompatibleByteType<typename std::iterator_traits<decltype(
                        std::begin(std::declval<const T &>()))>::value_type>,
                std::is_convertible<decltype(std::begin(std::declval<const T &>())
                                             != std::end(std::declval<const T &>())),
                                    bool>,

                // This needs to be treated specially due to the empty vs null distinction
                std::negation<std::is_same<std::decay_t<T>, QByteArray>>,

                // We handle array literals specially for source compat reasons
                std::negation<std::is_array<T>>,

                // Don't make an accidental copy constructor
                std::negation<std::is_same<std::decay_t<T>, QByteArrayView>>>>> : std::true_type {};

// Used by QLatin1StringView too
template <typename Char>
static constexpr qsizetype lengthHelperPointer(const Char *data) noexcept
{
    return qsizetype(std::char_traits<Char>::length(data));
}

} // namespace QtPrivate

class Q_CORE_EXPORT QByteArrayView
{
public:
    typedef char storage_type;
    typedef const char value_type;
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
    template <typename Byte>
    using if_compatible_byte =
            typename std::enable_if_t<QtPrivate::IsCompatibleByteType<Byte>::value, bool>;

    template <typename Pointer>
    using if_compatible_pointer =
            typename std::enable_if_t<QtPrivate::IsCompatibleByteArrayPointer<Pointer>::value,
                                      bool>;

    template <typename T>
    using if_compatible_qbytearray_like =
            typename std::enable_if_t<std::is_same_v<T, QByteArray>, bool>;

    template <typename T>
    using if_compatible_container =
            typename std::enable_if_t<QtPrivate::IsContainerCompatibleWithQByteArrayView<T>::value,
                                      bool>;

    template <typename Container>
    static constexpr qsizetype lengthHelperContainer(const Container &c) noexcept
    {
        return qsizetype(std::size(c));
    }

    static constexpr qsizetype lengthHelperCharArray(const char *data, size_t size) noexcept
    {
        const auto it = std::char_traits<char>::find(data, size, '\0');
        const auto end = it ? it : std::next(data, size);
        return qsizetype(std::distance(data, end));
    }

    template <typename Byte>
    static const storage_type *castHelper(const Byte *data) noexcept
    { return reinterpret_cast<const storage_type*>(data); }
    static constexpr const storage_type *castHelper(const storage_type *data) noexcept
    { return data; }

public:
    constexpr QByteArrayView() noexcept
        : m_size(0), m_data(nullptr) {}
    constexpr QByteArrayView(std::nullptr_t) noexcept
        : QByteArrayView() {}

    template <typename Byte, if_compatible_byte<Byte> = true>
    constexpr QByteArrayView(const Byte *data, qsizetype len)
        : m_size((Q_ASSERT(len >= 0), Q_ASSERT(data || !len), len)),
          m_data(castHelper(data)) {}

    template <typename Byte, if_compatible_byte<Byte> = true>
    constexpr QByteArrayView(const Byte *first, const Byte *last)
        : QByteArrayView(first, last - first) {}

#ifdef Q_QDOC
    template <typename Byte>
    constexpr QByteArrayView(const Byte *data) noexcept;
#else
    template <typename Pointer, if_compatible_pointer<Pointer> = true>
    constexpr QByteArrayView(const Pointer &data) noexcept
        : QByteArrayView(
              data, data ? QtPrivate::lengthHelperPointer(data) : 0) {}
#endif

#ifdef Q_QDOC
    QByteArrayView(const QByteArray &data) noexcept;
#else
    template <typename ByteArray, if_compatible_qbytearray_like<ByteArray> = true>
    QByteArrayView(const ByteArray &ba) noexcept
        : QByteArrayView(ba.isNull() ? nullptr : ba.data(), qsizetype(ba.size())) {}
#endif

    template <typename Container, if_compatible_container<Container> = true>
    constexpr QByteArrayView(const Container &c) noexcept
        : QByteArrayView(std::data(c), lengthHelperContainer(c)) {}
    template <size_t Size>
    constexpr QByteArrayView(const char (&data)[Size]) noexcept
        : QByteArrayView(data, lengthHelperCharArray(data, Size)) {}

#ifdef Q_QDOC
    template <typename Byte, size_t Size>
#else
    template <typename Byte, size_t Size, if_compatible_byte<Byte> = true>
#endif
    [[nodiscard]] constexpr static QByteArrayView fromArray(const Byte (&data)[Size]) noexcept
    { return QByteArrayView(data, Size); }
    [[nodiscard]] inline QByteArray toByteArray() const; // defined in qbytearray.h

    [[nodiscard]] constexpr qsizetype size() const noexcept { return m_size; }
    [[nodiscard]] constexpr const_pointer data() const noexcept { return m_data; }
    [[nodiscard]] constexpr const_pointer constData() const noexcept { return data(); }

    [[nodiscard]] constexpr char operator[](qsizetype n) const
    { Q_ASSERT(n >= 0); Q_ASSERT(n < size()); return m_data[n]; }

    //
    // QByteArray API
    //
    [[nodiscard]] constexpr char at(qsizetype n) const { return (*this)[n]; }

    [[nodiscard]] constexpr QByteArrayView first(qsizetype n) const
    { Q_ASSERT(n >= 0); Q_ASSERT(n <= size()); return QByteArrayView(data(), n); }
    [[nodiscard]] constexpr QByteArrayView last(qsizetype n) const
    { Q_ASSERT(n >= 0); Q_ASSERT(n <= size()); return QByteArrayView(data() + size() - n, n); }
    [[nodiscard]] constexpr QByteArrayView sliced(qsizetype pos) const
    { Q_ASSERT(pos >= 0); Q_ASSERT(pos <= size()); return QByteArrayView(data() + pos, size() - pos); }
    [[nodiscard]] constexpr QByteArrayView sliced(qsizetype pos, qsizetype n) const
    { Q_ASSERT(pos >= 0); Q_ASSERT(n >= 0); Q_ASSERT(size_t(pos) + size_t(n) <= size_t(size())); return QByteArrayView(data() + pos, n); }
    [[nodiscard]] constexpr QByteArrayView chopped(qsizetype len) const
    { Q_ASSERT(len >= 0); Q_ASSERT(len <= size()); return first(size() - len); }

    constexpr void truncate(qsizetype n)
    { Q_ASSERT(n >= 0); Q_ASSERT(n <= size()); m_size = n; }
    constexpr void chop(qsizetype n)
    { Q_ASSERT(n >= 0); Q_ASSERT(n <= size()); m_size -= n; }

    // Defined in qbytearray.cpp:
    [[nodiscard]] QByteArrayView trimmed() const noexcept
    { return QtPrivate::trimmed(*this); }
    [[nodiscard]] short toShort(bool *ok = nullptr, int base = 10) const
    { return QtPrivate::toIntegral<short>(*this, ok, base); }
    [[nodiscard]] ushort toUShort(bool *ok = nullptr, int base = 10) const
    { return QtPrivate::toIntegral<ushort>(*this, ok, base); }
    [[nodiscard]] int toInt(bool *ok = nullptr, int base = 10) const
    { return QtPrivate::toIntegral<int>(*this, ok, base); }
    [[nodiscard]] uint toUInt(bool *ok = nullptr, int base = 10) const
    { return QtPrivate::toIntegral<uint>(*this, ok, base); }
    [[nodiscard]] long toLong(bool *ok = nullptr, int base = 10) const
    { return QtPrivate::toIntegral<long>(*this, ok, base); }
    [[nodiscard]] ulong toULong(bool *ok = nullptr, int base = 10) const
    { return QtPrivate::toIntegral<ulong>(*this, ok, base); }
    [[nodiscard]] qlonglong toLongLong(bool *ok = nullptr, int base = 10) const
    { return QtPrivate::toIntegral<qlonglong>(*this, ok, base); }
    [[nodiscard]] qulonglong toULongLong(bool *ok = nullptr, int base = 10) const
    { return QtPrivate::toIntegral<qulonglong>(*this, ok, base); }
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

    [[nodiscard]] bool startsWith(QByteArrayView other) const noexcept
    { return QtPrivate::startsWith(*this, other); }
    [[nodiscard]] bool startsWith(char c) const noexcept
    { return !empty() && front() == c; }

    [[nodiscard]] bool endsWith(QByteArrayView other) const noexcept
    { return QtPrivate::endsWith(*this, other); }
    [[nodiscard]] bool endsWith(char c) const noexcept
    { return !empty() && back() == c; }

    [[nodiscard]] qsizetype indexOf(QByteArrayView a, qsizetype from = 0) const noexcept
    { return QtPrivate::findByteArray(*this, from, a); }
    [[nodiscard]] qsizetype indexOf(char ch, qsizetype from = 0) const noexcept
    { return QtPrivate::findByteArray(*this, from, QByteArrayView(&ch, 1)); }

    [[nodiscard]] bool contains(QByteArrayView a) const noexcept
    { return indexOf(a) != qsizetype(-1); }
    [[nodiscard]] bool contains(char c) const noexcept
    { return indexOf(c) != qsizetype(-1); }

    [[nodiscard]] qsizetype lastIndexOf(QByteArrayView a) const noexcept
    { return lastIndexOf(a, size()); }
    [[nodiscard]] qsizetype lastIndexOf(QByteArrayView a, qsizetype from) const noexcept
    { return QtPrivate::lastIndexOf(*this, from, a); }
    [[nodiscard]] qsizetype lastIndexOf(char ch, qsizetype from = -1) const noexcept
    { return QtPrivate::lastIndexOf(*this, from, QByteArrayView(&ch, 1)); }

    [[nodiscard]] qsizetype count(QByteArrayView a) const noexcept
    { return QtPrivate::count(*this, a); }
    [[nodiscard]] qsizetype count(char ch) const noexcept
    { return QtPrivate::count(*this, QByteArrayView(&ch, 1)); }

    inline int compare(QByteArrayView a, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept;

    [[nodiscard]] inline bool isValidUtf8() const noexcept { return QtPrivate::isValidUtf8(*this); }

    //
    // STL compatibility API:
    //
    [[nodiscard]] constexpr const_iterator begin()   const noexcept { return data(); }
    [[nodiscard]] constexpr const_iterator end()     const noexcept { return data() + size(); }
    [[nodiscard]] constexpr const_iterator cbegin()  const noexcept { return begin(); }
    [[nodiscard]] constexpr const_iterator cend()    const noexcept { return end(); }
    [[nodiscard]] constexpr const_reverse_iterator rbegin()  const noexcept { return const_reverse_iterator(end()); }
    [[nodiscard]] constexpr const_reverse_iterator rend()    const noexcept { return const_reverse_iterator(begin()); }
    [[nodiscard]] constexpr const_reverse_iterator crbegin() const noexcept { return rbegin(); }
    [[nodiscard]] constexpr const_reverse_iterator crend()   const noexcept { return rend(); }

    [[nodiscard]] constexpr bool empty() const noexcept { return size() == 0; }
    [[nodiscard]] constexpr char front() const { Q_ASSERT(!empty()); return m_data[0]; }
    [[nodiscard]] constexpr char back()  const { Q_ASSERT(!empty()); return m_data[m_size - 1]; }

    //
    // Qt compatibility API:
    //
    [[nodiscard]] constexpr bool isNull() const noexcept { return !m_data; }
    [[nodiscard]] constexpr bool isEmpty() const noexcept { return empty(); }
    [[nodiscard]] constexpr qsizetype length() const noexcept
    { return size(); }
    [[nodiscard]] constexpr char first() const { return front(); }
    [[nodiscard]] constexpr char last()  const { return back(); }

    friend inline bool operator==(QByteArrayView lhs, QByteArrayView rhs) noexcept
    { return lhs.size() == rhs.size() && QtPrivate::compareMemory(lhs, rhs) == 0; }
    friend inline bool operator!=(QByteArrayView lhs, QByteArrayView rhs) noexcept
    { return !(lhs == rhs); }
    friend inline bool operator< (QByteArrayView lhs, QByteArrayView rhs) noexcept
    { return QtPrivate::compareMemory(lhs, rhs) <  0; }
    friend inline bool operator<=(QByteArrayView lhs, QByteArrayView rhs) noexcept
    { return QtPrivate::compareMemory(lhs, rhs) <= 0; }
    friend inline bool operator> (QByteArrayView lhs, QByteArrayView rhs) noexcept
    { return !(lhs <= rhs); }
    friend inline bool operator>=(QByteArrayView lhs, QByteArrayView rhs) noexcept
    { return !(lhs < rhs); }

private:
    qsizetype m_size;
    const storage_type *m_data;
};
Q_DECLARE_TYPEINFO(QByteArrayView, Q_PRIMITIVE_TYPE);

template<typename QByteArrayLike,
         std::enable_if_t<std::is_same_v<QByteArrayLike, QByteArray>, bool> = true>
[[nodiscard]] inline QByteArrayView qToByteArrayViewIgnoringNull(const QByteArrayLike &b) noexcept
{ return QByteArrayView(b.data(), b.size()); }

inline int QByteArrayView::compare(QByteArrayView a, Qt::CaseSensitivity cs) const noexcept
{
    return cs == Qt::CaseSensitive ? QtPrivate::compareMemory(*this, a) :
                                     qstrnicmp(data(), size(), a.data(), a.size());
}

#if QT_DEPRECATED_SINCE(6, 0)
QT_DEPRECATED_VERSION_X_6_0("Use the QByteArrayView overload.")
inline quint16 qChecksum(const char *s, qsizetype len,
                         Qt::ChecksumType standard = Qt::ChecksumIso3309)
{ return qChecksum(QByteArrayView(s, len), standard); }
#endif

QT_END_NAMESPACE

#endif // QBYTEARRAYVIEW_H
