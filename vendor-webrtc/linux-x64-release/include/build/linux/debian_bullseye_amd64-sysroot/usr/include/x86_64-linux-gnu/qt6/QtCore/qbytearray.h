// Copyright (C) 2022 The Qt Company Ltd.
// Copyright (C) 2016 Intel Corporation.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QBYTEARRAY_H
#define QBYTEARRAY_H

#include <QtCore/qrefcount.h>
#include <QtCore/qnamespace.h>
#include <QtCore/qarraydata.h>
#include <QtCore/qarraydatapointer.h>
#include <QtCore/qcontainerfwd.h>
#include <QtCore/qbytearrayalgorithms.h>
#include <QtCore/qbytearrayview.h>

#include <stdlib.h>
#include <string.h>
#include <stdarg.h>

#include <string>
#include <iterator>

#ifndef QT5_NULL_STRINGS
// Would ideally be off, but in practice breaks too much (Qt 6.0).
#define QT5_NULL_STRINGS 1
#endif

#ifdef truncate
#error qbytearray.h must be included before any header file that defines truncate
#endif

#if defined(Q_OS_DARWIN) || defined(Q_QDOC)
Q_FORWARD_DECLARE_CF_TYPE(CFData);
Q_FORWARD_DECLARE_OBJC_CLASS(NSData);
#endif

QT_BEGIN_NAMESPACE

class QString;
class QDataStream;

using QByteArrayData = QArrayDataPointer<char>;

#  define QByteArrayLiteral(str) \
    (QByteArray(QByteArrayData(nullptr, const_cast<char *>(str), sizeof(str) - 1))) \
    /**/

class Q_CORE_EXPORT QByteArray
{
public:
    using DataPointer = QByteArrayData;
private:
    typedef QTypedArrayData<char> Data;

    DataPointer d;
    static const char _empty;
public:

    enum Base64Option {
        Base64Encoding = 0,
        Base64UrlEncoding = 1,

        KeepTrailingEquals = 0,
        OmitTrailingEquals = 2,

        IgnoreBase64DecodingErrors = 0,
        AbortOnBase64DecodingErrors = 4,
    };
    Q_DECLARE_FLAGS(Base64Options, Base64Option)

    enum class Base64DecodingStatus {
        Ok,
        IllegalInputLength,
        IllegalCharacter,
        IllegalPadding,
    };

    inline constexpr QByteArray() noexcept;
    QByteArray(const char *, qsizetype size = -1);
    QByteArray(qsizetype size, char c);
    QByteArray(qsizetype size, Qt::Initialization);
    inline QByteArray(const QByteArray &) noexcept;
    inline ~QByteArray();

    QByteArray &operator=(const QByteArray &) noexcept;
    QByteArray &operator=(const char *str);
    inline QByteArray(QByteArray && other) noexcept
        = default;
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QByteArray)
    inline void swap(QByteArray &other) noexcept
    { d.swap(other.d); }

    bool isEmpty() const noexcept { return size() == 0; }
    void resize(qsizetype size);
    void resize(qsizetype size, char c);

    QByteArray &fill(char c, qsizetype size = -1);

    inline qsizetype capacity() const;
    inline void reserve(qsizetype size);
    inline void squeeze();

#ifndef QT_NO_CAST_FROM_BYTEARRAY
    inline operator const char *() const;
    inline operator const void *() const;
#endif
    inline char *data();
    inline const char *data() const noexcept;
    const char *constData() const noexcept { return data(); }
    inline void detach();
    inline bool isDetached() const;
    inline bool isSharedWith(const QByteArray &other) const noexcept
    { return data() == other.data() && size() == other.size(); }
    void clear();

    inline char at(qsizetype i) const;
    inline char operator[](qsizetype i) const;
    [[nodiscard]] inline char &operator[](qsizetype i);
    [[nodiscard]] char front() const { return at(0); }
    [[nodiscard]] inline char &front();
    [[nodiscard]] char back() const { return at(size() - 1); }
    [[nodiscard]] inline char &back();

    qsizetype indexOf(char c, qsizetype from = 0) const;
    qsizetype indexOf(QByteArrayView bv, qsizetype from = 0) const
    { return QtPrivate::findByteArray(qToByteArrayViewIgnoringNull(*this), from, bv); }

    qsizetype lastIndexOf(char c, qsizetype from = -1) const;
    qsizetype lastIndexOf(QByteArrayView bv) const
    { return lastIndexOf(bv, size()); }
    qsizetype lastIndexOf(QByteArrayView bv, qsizetype from) const
    { return QtPrivate::lastIndexOf(qToByteArrayViewIgnoringNull(*this), from, bv); }

    inline bool contains(char c) const;
    inline bool contains(QByteArrayView bv) const;
    qsizetype count(char c) const;
    qsizetype count(QByteArrayView bv) const
    { return QtPrivate::count(qToByteArrayViewIgnoringNull(*this), bv); }

    inline int compare(QByteArrayView a, Qt::CaseSensitivity cs = Qt::CaseSensitive) const noexcept;

    [[nodiscard]] QByteArray left(qsizetype len) const;
    [[nodiscard]] QByteArray right(qsizetype len) const;
    [[nodiscard]] QByteArray mid(qsizetype index, qsizetype len = -1) const;

    [[nodiscard]] QByteArray first(qsizetype n) const
    { Q_ASSERT(n >= 0); Q_ASSERT(n <= size()); return QByteArray(data(), n); }
    [[nodiscard]] QByteArray last(qsizetype n) const
    { Q_ASSERT(n >= 0); Q_ASSERT(n <= size()); return QByteArray(data() + size() - n, n); }
    [[nodiscard]] QByteArray sliced(qsizetype pos) const
    { Q_ASSERT(pos >= 0); Q_ASSERT(pos <= size()); return QByteArray(data() + pos, size() - pos); }
    [[nodiscard]] QByteArray sliced(qsizetype pos, qsizetype n) const
    { Q_ASSERT(pos >= 0); Q_ASSERT(n >= 0); Q_ASSERT(size_t(pos) + size_t(n) <= size_t(size())); return QByteArray(data() + pos, n); }
    [[nodiscard]] QByteArray chopped(qsizetype len) const
    { Q_ASSERT(len >= 0); Q_ASSERT(len <= size()); return first(size() - len); }

    bool startsWith(QByteArrayView bv) const
    { return QtPrivate::startsWith(qToByteArrayViewIgnoringNull(*this), bv); }
    bool startsWith(char c) const { return size() > 0 && front() == c; }

    bool endsWith(char c) const { return size() > 0 && back() == c; }
    bool endsWith(QByteArrayView bv) const
    { return QtPrivate::endsWith(qToByteArrayViewIgnoringNull(*this), bv); }

    bool isUpper() const;
    bool isLower() const;

    [[nodiscard]] bool isValidUtf8() const noexcept
    {
        return QtPrivate::isValidUtf8(qToByteArrayViewIgnoringNull(*this));
    }

    void truncate(qsizetype pos);
    void chop(qsizetype n);

#if !defined(Q_CLANG_QDOC)
    [[nodiscard]] QByteArray toLower() const &
    { return toLower_helper(*this); }
    [[nodiscard]] QByteArray toLower() &&
    { return toLower_helper(*this); }
    [[nodiscard]] QByteArray toUpper() const &
    { return toUpper_helper(*this); }
    [[nodiscard]] QByteArray toUpper() &&
    { return toUpper_helper(*this); }
    [[nodiscard]] QByteArray trimmed() const &
    { return trimmed_helper(*this); }
    [[nodiscard]] QByteArray trimmed() &&
    { return trimmed_helper(*this); }
    [[nodiscard]] QByteArray simplified() const &
    { return simplified_helper(*this); }
    [[nodiscard]] QByteArray simplified() &&
    { return simplified_helper(*this); }
#else
    [[nodiscard]] QByteArray toLower() const;
    [[nodiscard]] QByteArray toUpper() const;
    [[nodiscard]] QByteArray trimmed() const;
    [[nodiscard]] QByteArray simplified() const;
#endif

    [[nodiscard]] QByteArray leftJustified(qsizetype width, char fill = ' ', bool truncate = false) const;
    [[nodiscard]] QByteArray rightJustified(qsizetype width, char fill = ' ', bool truncate = false) const;

    QByteArray &prepend(char c)
    { return insert(0, QByteArrayView(&c, 1)); }
    inline QByteArray &prepend(qsizetype count, char c);
    QByteArray &prepend(const char *s)
    { return insert(0, QByteArrayView(s, qsizetype(qstrlen(s)))); }
    QByteArray &prepend(const char *s, qsizetype len)
    { return insert(0, QByteArrayView(s, len)); }
    QByteArray &prepend(const QByteArray &a);
    QByteArray &prepend(QByteArrayView a)
    { return insert(0, a); }

    QByteArray &append(char c);
    inline QByteArray &append(qsizetype count, char c);
    QByteArray &append(const char *s)
    { return append(s, -1); }
    QByteArray &append(const char *s, qsizetype len)
    { return append(QByteArrayView(s, len < 0 ? qsizetype(qstrlen(s)) : len)); }
    QByteArray &append(const QByteArray &a);
    QByteArray &append(QByteArrayView a)
    { return insert(size(), a); }

    QByteArray &insert(qsizetype i, QByteArrayView data);
    inline QByteArray &insert(qsizetype i, const char *s)
    { return insert(i, QByteArrayView(s)); }
    inline QByteArray &insert(qsizetype i, const QByteArray &data)
    { return insert(i, QByteArrayView(data)); }
    QByteArray &insert(qsizetype i, qsizetype count, char c);
    QByteArray &insert(qsizetype i, char c)
    { return insert(i, QByteArrayView(&c, 1)); }
    QByteArray &insert(qsizetype i, const char *s, qsizetype len)
    { return insert(i, QByteArrayView(s, len)); }

    QByteArray &remove(qsizetype index, qsizetype len);
    template <typename Predicate>
    QByteArray &removeIf(Predicate pred)
    {
        QtPrivate::sequential_erase_if(*this, pred);
        return *this;
    }

    QByteArray &replace(qsizetype index, qsizetype len, const char *s, qsizetype alen)
    { return replace(index, len, QByteArrayView(s, alen)); }
    QByteArray &replace(qsizetype index, qsizetype len, QByteArrayView s);
    QByteArray &replace(char before, QByteArrayView after)
    { return replace(QByteArrayView(&before, 1), after); }
    QByteArray &replace(const char *before, qsizetype bsize, const char *after, qsizetype asize)
    { return replace(QByteArrayView(before, bsize), QByteArrayView(after, asize)); }
    QByteArray &replace(QByteArrayView before, QByteArrayView after);
    QByteArray &replace(char before, char after);

    QByteArray &operator+=(char c)
    { return append(c); }
    QByteArray &operator+=(const char *s)
    { return append(s); }
    QByteArray &operator+=(const QByteArray &a)
    { return append(a); }
    QByteArray &operator+=(QByteArrayView a)
    { return append(a); }

    QList<QByteArray> split(char sep) const;

    [[nodiscard]] QByteArray repeated(qsizetype times) const;

#if !defined(QT_NO_CAST_FROM_ASCII) && !defined(QT_RESTRICTED_CAST_FROM_ASCII)
    QT_ASCII_CAST_WARN inline bool operator==(const QString &s2) const;
    QT_ASCII_CAST_WARN inline bool operator!=(const QString &s2) const;
    QT_ASCII_CAST_WARN inline bool operator<(const QString &s2) const;
    QT_ASCII_CAST_WARN inline bool operator>(const QString &s2) const;
    QT_ASCII_CAST_WARN inline bool operator<=(const QString &s2) const;
    QT_ASCII_CAST_WARN inline bool operator>=(const QString &s2) const;
#endif
    friend inline bool operator==(const QByteArray &a1, const QByteArray &a2) noexcept
    { return QByteArrayView(a1) == QByteArrayView(a2); }
    friend inline bool operator==(const QByteArray &a1, const char *a2) noexcept
    { return a2 ? QtPrivate::compareMemory(a1, a2) == 0 : a1.isEmpty(); }
    friend inline bool operator==(const char *a1, const QByteArray &a2) noexcept
    { return a1 ? QtPrivate::compareMemory(a1, a2) == 0 : a2.isEmpty(); }
    friend inline bool operator!=(const QByteArray &a1, const QByteArray &a2) noexcept
    { return !(a1==a2); }
    friend inline bool operator!=(const QByteArray &a1, const char *a2) noexcept
    { return a2 ? QtPrivate::compareMemory(a1, a2) != 0 : !a1.isEmpty(); }
    friend inline bool operator!=(const char *a1, const QByteArray &a2) noexcept
    { return a1 ? QtPrivate::compareMemory(a1, a2) != 0 : !a2.isEmpty(); }
    friend inline bool operator<(const QByteArray &a1, const QByteArray &a2) noexcept
    { return QtPrivate::compareMemory(QByteArrayView(a1), QByteArrayView(a2)) < 0; }
    friend inline bool operator<(const QByteArray &a1, const char *a2) noexcept
    { return QtPrivate::compareMemory(a1, a2) < 0; }
    friend inline bool operator<(const char *a1, const QByteArray &a2) noexcept
    { return QtPrivate::compareMemory(a1, a2) < 0; }
    friend inline bool operator<=(const QByteArray &a1, const QByteArray &a2) noexcept
    { return QtPrivate::compareMemory(QByteArrayView(a1), QByteArrayView(a2)) <= 0; }
    friend inline bool operator<=(const QByteArray &a1, const char *a2) noexcept
    { return QtPrivate::compareMemory(a1, a2) <= 0; }
    friend inline bool operator<=(const char *a1, const QByteArray &a2) noexcept
    { return QtPrivate::compareMemory(a1, a2) <= 0; }
    friend inline bool operator>(const QByteArray &a1, const QByteArray &a2) noexcept
    { return QtPrivate::compareMemory(QByteArrayView(a1), QByteArrayView(a2)) > 0; }
    friend inline bool operator>(const QByteArray &a1, const char *a2) noexcept
    { return QtPrivate::compareMemory(a1, a2) > 0; }
    friend inline bool operator>(const char *a1, const QByteArray &a2) noexcept
    { return QtPrivate::compareMemory(a1, a2) > 0; }
    friend inline bool operator>=(const QByteArray &a1, const QByteArray &a2) noexcept
    { return QtPrivate::compareMemory(QByteArrayView(a1), QByteArrayView(a2)) >= 0; }
    friend inline bool operator>=(const QByteArray &a1, const char *a2) noexcept
    { return QtPrivate::compareMemory(a1, a2) >= 0; }
    friend inline bool operator>=(const char *a1, const QByteArray &a2) noexcept
    { return QtPrivate::compareMemory(a1, a2) >= 0; }

    // Check isEmpty() instead of isNull() for backwards compatibility.
    friend inline bool operator==(const QByteArray &a1, std::nullptr_t) noexcept { return a1.isEmpty(); }
    friend inline bool operator!=(const QByteArray &a1, std::nullptr_t) noexcept { return !a1.isEmpty(); }
    friend inline bool operator< (const QByteArray &  , std::nullptr_t) noexcept { return false; }
    friend inline bool operator> (const QByteArray &a1, std::nullptr_t) noexcept { return !a1.isEmpty(); }
    friend inline bool operator<=(const QByteArray &a1, std::nullptr_t) noexcept { return a1.isEmpty(); }
    friend inline bool operator>=(const QByteArray &  , std::nullptr_t) noexcept { return true; }

    friend inline bool operator==(std::nullptr_t, const QByteArray &a2) noexcept { return a2 == nullptr; }
    friend inline bool operator!=(std::nullptr_t, const QByteArray &a2) noexcept { return a2 != nullptr; }
    friend inline bool operator< (std::nullptr_t, const QByteArray &a2) noexcept { return a2 >  nullptr; }
    friend inline bool operator> (std::nullptr_t, const QByteArray &a2) noexcept { return a2 <  nullptr; }
    friend inline bool operator<=(std::nullptr_t, const QByteArray &a2) noexcept { return a2 >= nullptr; }
    friend inline bool operator>=(std::nullptr_t, const QByteArray &a2) noexcept { return a2 <= nullptr; }

    short toShort(bool *ok = nullptr, int base = 10) const;
    ushort toUShort(bool *ok = nullptr, int base = 10) const;
    int toInt(bool *ok = nullptr, int base = 10) const;
    uint toUInt(bool *ok = nullptr, int base = 10) const;
    long toLong(bool *ok = nullptr, int base = 10) const;
    ulong toULong(bool *ok = nullptr, int base = 10) const;
    qlonglong toLongLong(bool *ok = nullptr, int base = 10) const;
    qulonglong toULongLong(bool *ok = nullptr, int base = 10) const;
    float toFloat(bool *ok = nullptr) const;
    double toDouble(bool *ok = nullptr) const;
    QByteArray toBase64(Base64Options options = Base64Encoding) const;
    QByteArray toHex(char separator = '\0') const;
    QByteArray toPercentEncoding(const QByteArray &exclude = QByteArray(),
                                 const QByteArray &include = QByteArray(),
                                 char percent = '%') const;
    [[nodiscard]] QByteArray percentDecoded(char percent = '%') const;

    inline QByteArray &setNum(short, int base = 10);
    inline QByteArray &setNum(ushort, int base = 10);
    inline QByteArray &setNum(int, int base = 10);
    inline QByteArray &setNum(uint, int base = 10);
    inline QByteArray &setNum(long, int base = 10);
    inline QByteArray &setNum(ulong, int base = 10);
    QByteArray &setNum(qlonglong, int base = 10);
    QByteArray &setNum(qulonglong, int base = 10);
    inline QByteArray &setNum(float, char format = 'g', int precision = 6);
    QByteArray &setNum(double, char format = 'g', int precision = 6);
    QByteArray &setRawData(const char *a, qsizetype n);

    [[nodiscard]] static QByteArray number(int, int base = 10);
    [[nodiscard]] static QByteArray number(uint, int base = 10);
    [[nodiscard]] static QByteArray number(long, int base = 10);
    [[nodiscard]] static QByteArray number(ulong, int base = 10);
    [[nodiscard]] static QByteArray number(qlonglong, int base = 10);
    [[nodiscard]] static QByteArray number(qulonglong, int base = 10);
    [[nodiscard]] static QByteArray number(double, char format = 'g', int precision = 6);
    [[nodiscard]] static QByteArray fromRawData(const char *data, qsizetype size)
    {
        return QByteArray(DataPointer(nullptr, const_cast<char *>(data), size));
    }

    class FromBase64Result;
    [[nodiscard]] static FromBase64Result fromBase64Encoding(QByteArray &&base64, Base64Options options = Base64Encoding);
    [[nodiscard]] static FromBase64Result fromBase64Encoding(const QByteArray &base64, Base64Options options = Base64Encoding);
    [[nodiscard]] static QByteArray fromBase64(const QByteArray &base64, Base64Options options = Base64Encoding);
    [[nodiscard]] static QByteArray fromHex(const QByteArray &hexEncoded);
    [[nodiscard]] static QByteArray fromPercentEncoding(const QByteArray &pctEncoded, char percent = '%');

#if defined(Q_OS_DARWIN) || defined(Q_QDOC)
    static QByteArray fromCFData(CFDataRef data);
    static QByteArray fromRawCFData(CFDataRef data);
    CFDataRef toCFData() const Q_DECL_CF_RETURNS_RETAINED;
    CFDataRef toRawCFData() const Q_DECL_CF_RETURNS_RETAINED;
    static QByteArray fromNSData(const NSData *data);
    static QByteArray fromRawNSData(const NSData *data);
    NSData *toNSData() const Q_DECL_NS_RETURNS_AUTORELEASED;
    NSData *toRawNSData() const Q_DECL_NS_RETURNS_AUTORELEASED;
#endif

    typedef char *iterator;
    typedef const char *const_iterator;
    typedef iterator Iterator;
    typedef const_iterator ConstIterator;
    typedef std::reverse_iterator<iterator> reverse_iterator;
    typedef std::reverse_iterator<const_iterator> const_reverse_iterator;
    iterator begin() { return data(); }
    const_iterator begin() const noexcept { return data(); }
    const_iterator cbegin() const noexcept { return begin(); }
    const_iterator constBegin() const noexcept { return begin(); }
    iterator end() { return data() + size(); }
    const_iterator end() const noexcept { return data() + size(); }
    const_iterator cend() const noexcept { return end(); }
    const_iterator constEnd() const noexcept { return end(); }
    reverse_iterator rbegin() { return reverse_iterator(end()); }
    reverse_iterator rend() { return reverse_iterator(begin()); }
    const_reverse_iterator rbegin() const noexcept { return const_reverse_iterator(end()); }
    const_reverse_iterator rend() const noexcept { return const_reverse_iterator(begin()); }
    const_reverse_iterator crbegin() const noexcept { return rbegin(); }
    const_reverse_iterator crend() const noexcept { return rend(); }

    // stl compatibility
    typedef qsizetype size_type;
    typedef qptrdiff difference_type;
    typedef const char & const_reference;
    typedef char & reference;
    typedef char *pointer;
    typedef const char *const_pointer;
    typedef char value_type;
    void push_back(char c)
    { append(c); }
    void push_back(const char *s)
    { append(s); }
    void push_back(const QByteArray &a)
    { append(a); }
    void push_back(QByteArrayView a)
    { append(a); }
    void push_front(char c)
    { prepend(c); }
    void push_front(const char *c)
    { prepend(c); }
    void push_front(const QByteArray &a)
    { prepend(a); }
    void push_front(QByteArrayView a)
    { prepend(a); }
    void shrink_to_fit() { squeeze(); }
    iterator erase(const_iterator first, const_iterator last);

    static QByteArray fromStdString(const std::string &s);
    std::string toStdString() const;

    inline qsizetype size() const noexcept { return d->size; }
#if QT_DEPRECATED_SINCE(6, 4)
    QT_DEPRECATED_VERSION_X_6_4("Use size() or length() instead.")
    inline qsizetype count() const noexcept { return size(); }
#endif
    inline qsizetype length() const noexcept { return size(); }
    QT_CORE_INLINE_SINCE(6, 4)
    bool isNull() const noexcept;

    inline DataPointer &data_ptr() { return d; }
#if QT_VERSION < QT_VERSION_CHECK(7, 0, 0)
    explicit inline QByteArray(const DataPointer &dd) : d(dd) {}
#endif
    explicit inline QByteArray(DataPointer &&dd) : d(std::move(dd)) {}

private:
    void reallocData(qsizetype alloc, QArrayData::AllocationOption option);
    void reallocGrowData(qsizetype n);
    void expand(qsizetype i);

    static QByteArray toLower_helper(const QByteArray &a);
    static QByteArray toLower_helper(QByteArray &a);
    static QByteArray toUpper_helper(const QByteArray &a);
    static QByteArray toUpper_helper(QByteArray &a);
    static QByteArray trimmed_helper(const QByteArray &a);
    static QByteArray trimmed_helper(QByteArray &a);
    static QByteArray simplified_helper(const QByteArray &a);
    static QByteArray simplified_helper(QByteArray &a);

    friend class QString;
    friend Q_CORE_EXPORT QByteArray qUncompress(const uchar *data, qsizetype nbytes);
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QByteArray::Base64Options)

inline constexpr QByteArray::QByteArray() noexcept {}
inline QByteArray::~QByteArray() {}

inline char QByteArray::at(qsizetype i) const
{ Q_ASSERT(size_t(i) < size_t(size())); return d.data()[i]; }
inline char QByteArray::operator[](qsizetype i) const
{ Q_ASSERT(size_t(i) < size_t(size())); return d.data()[i]; }

#ifndef QT_NO_CAST_FROM_BYTEARRAY
inline QByteArray::operator const char *() const
{ return data(); }
inline QByteArray::operator const void *() const
{ return data(); }
#endif
inline char *QByteArray::data()
{
    detach();
    Q_ASSERT(d.data());
    return d.data();
}
inline const char *QByteArray::data() const noexcept
{
#if QT5_NULL_STRINGS == 1
    return d.data() ? d.data() : &_empty;
#else
    return d.data();
#endif
}
inline void QByteArray::detach()
{ if (d->needsDetach()) reallocData(size(), QArrayData::KeepSize); }
inline bool QByteArray::isDetached() const
{ return !d->isShared(); }
inline QByteArray::QByteArray(const QByteArray &a) noexcept : d(a.d)
{}

inline qsizetype QByteArray::capacity() const { return qsizetype(d->constAllocatedCapacity()); }

inline void QByteArray::reserve(qsizetype asize)
{
    if (d->needsDetach() || asize > capacity() - d->freeSpaceAtBegin())
        reallocData(qMax(size(), asize), QArrayData::KeepSize);
    if (d->constAllocatedCapacity())
        d->setFlag(Data::CapacityReserved);
}

inline void QByteArray::squeeze()
{
    if (!d.isMutable())
        return;
    if (d->needsDetach() || size() < capacity())
        reallocData(size(), QArrayData::KeepSize);
    if (d->constAllocatedCapacity())
        d->clearFlag(Data::CapacityReserved);
}

inline char &QByteArray::operator[](qsizetype i)
{ Q_ASSERT(i >= 0 && i < size()); return data()[i]; }
inline char &QByteArray::front() { return operator[](0); }
inline char &QByteArray::back() { return operator[](size() - 1); }
inline QByteArray &QByteArray::append(qsizetype n, char ch)
{ return insert(size(), n, ch); }
inline QByteArray &QByteArray::prepend(qsizetype n, char ch)
{ return insert(0, n, ch); }
inline bool QByteArray::contains(char c) const
{ return indexOf(c) != -1; }
inline bool QByteArray::contains(QByteArrayView bv) const
{ return indexOf(bv) != -1; }
inline int QByteArray::compare(QByteArrayView a, Qt::CaseSensitivity cs) const noexcept
{
    return cs == Qt::CaseSensitive ? QtPrivate::compareMemory(*this, a) :
                                     qstrnicmp(data(), size(), a.data(), a.size());
}
#if !defined(QT_USE_QSTRINGBUILDER)
inline const QByteArray operator+(const QByteArray &a1, const QByteArray &a2)
{ return QByteArray(a1) += a2; }
inline const QByteArray operator+(const QByteArray &a1, const char *a2)
{ return QByteArray(a1) += a2; }
inline const QByteArray operator+(const QByteArray &a1, char a2)
{ return QByteArray(a1) += a2; }
inline const QByteArray operator+(const char *a1, const QByteArray &a2)
{ return QByteArray(a1) += a2; }
inline const QByteArray operator+(char a1, const QByteArray &a2)
{ return QByteArray(&a1, 1) += a2; }
#endif // QT_USE_QSTRINGBUILDER

inline QByteArray &QByteArray::setNum(short n, int base)
{ return setNum(qlonglong(n), base); }
inline QByteArray &QByteArray::setNum(ushort n, int base)
{ return setNum(qulonglong(n), base); }
inline QByteArray &QByteArray::setNum(int n, int base)
{ return setNum(qlonglong(n), base); }
inline QByteArray &QByteArray::setNum(uint n, int base)
{ return setNum(qulonglong(n), base); }
inline QByteArray &QByteArray::setNum(long n, int base)
{ return setNum(qlonglong(n), base); }
inline QByteArray &QByteArray::setNum(ulong n, int base)
{ return setNum(qulonglong(n), base); }
inline QByteArray &QByteArray::setNum(float n, char format, int precision)
{ return setNum(double(n), format, precision); }

#if QT_CORE_INLINE_IMPL_SINCE(6, 4)
bool QByteArray::isNull() const noexcept
{
    return d->isNull();
}
#endif

#if !defined(QT_NO_DATASTREAM) || defined(QT_BOOTSTRAPPED)
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QByteArray &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QByteArray &);
#endif

#ifndef QT_NO_COMPRESS
Q_CORE_EXPORT QByteArray qCompress(const uchar* data, qsizetype nbytes, int compressionLevel = -1);
Q_CORE_EXPORT QByteArray qUncompress(const uchar* data, qsizetype nbytes);
inline QByteArray qCompress(const QByteArray& data, int compressionLevel = -1)
{ return qCompress(reinterpret_cast<const uchar *>(data.constData()), data.size(), compressionLevel); }
inline QByteArray qUncompress(const QByteArray& data)
{ return qUncompress(reinterpret_cast<const uchar*>(data.constData()), data.size()); }
#endif

Q_DECLARE_SHARED(QByteArray)

class QByteArray::FromBase64Result
{
public:
    QByteArray decoded;
    QByteArray::Base64DecodingStatus decodingStatus;

    void swap(QByteArray::FromBase64Result &other) noexcept
    {
        decoded.swap(other.decoded);
        std::swap(decodingStatus, other.decodingStatus);
    }

    explicit operator bool() const noexcept { return decodingStatus == QByteArray::Base64DecodingStatus::Ok; }

#if defined(Q_COMPILER_REF_QUALIFIERS) && !defined(Q_QDOC)
    QByteArray &operator*() & noexcept { return decoded; }
    const QByteArray &operator*() const & noexcept { return decoded; }
    QByteArray &&operator*() && noexcept { return std::move(decoded); }
#else
    QByteArray &operator*() noexcept { return decoded; }
    const QByteArray &operator*() const noexcept { return decoded; }
#endif

    friend inline bool operator==(const QByteArray::FromBase64Result &lhs, const QByteArray::FromBase64Result &rhs) noexcept
    {
        if (lhs.decodingStatus != rhs.decodingStatus)
            return false;

        if (lhs.decodingStatus == QByteArray::Base64DecodingStatus::Ok && lhs.decoded != rhs.decoded)
            return false;

        return true;
    }

    friend inline bool operator!=(const QByteArray::FromBase64Result &lhs, const QByteArray::FromBase64Result &rhs) noexcept
    {
        return !(lhs == rhs);
    }
};

Q_DECLARE_SHARED(QByteArray::FromBase64Result)


Q_CORE_EXPORT Q_DECL_PURE_FUNCTION size_t qHash(const QByteArray::FromBase64Result &key, size_t seed = 0) noexcept;

template <typename T>
qsizetype erase(QByteArray &ba, const T &t)
{
    return QtPrivate::sequential_erase(ba, t);
}

template <typename Predicate>
qsizetype erase_if(QByteArray &ba, Predicate pred)
{
    return QtPrivate::sequential_erase_if(ba, pred);
}

//
// QByteArrayView members that require QByteArray:
//
QByteArray QByteArrayView::toByteArray() const
{
    return QByteArray(data(), size());
}

namespace Qt {
inline namespace Literals {
inline namespace StringLiterals {

inline QByteArray operator"" _ba(const char *str, size_t size) noexcept
{
    return QByteArray(QByteArrayData(nullptr, const_cast<char *>(str), qsizetype(size)));
}

} // StringLiterals
} // Literals
} // Qt

inline namespace QtLiterals {
#if QT_DEPRECATED_SINCE(6, 8)

QT_DEPRECATED_VERSION_X_6_8("Use _ba from Qt::StringLiterals namespace instead.")
inline QByteArray operator"" _qba(const char *str, size_t size) noexcept
{
    return Qt::StringLiterals::operator""_ba(str, size);
}

#endif // QT_DEPRECATED_SINCE(6, 8)
} // QtLiterals

QT_END_NAMESPACE

#endif // QBYTEARRAY_H
