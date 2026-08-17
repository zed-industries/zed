// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QBITARRAY_H
#define QBITARRAY_H

#include <QtCore/qbytearray.h>

QT_BEGIN_NAMESPACE

class QBitRef;
class Q_CORE_EXPORT QBitArray
{
#ifndef QT_NO_DATASTREAM
    friend Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QBitArray &);
    friend Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QBitArray &);
#endif
    friend Q_CORE_EXPORT size_t qHash(const QBitArray &key, size_t seed) noexcept;
    QByteArray d;

public:
    inline QBitArray() noexcept {}
    explicit QBitArray(qsizetype size, bool val = false);
    QBitArray(const QBitArray &other) : d(other.d) {}
    inline QBitArray &operator=(const QBitArray &other) { d = other.d; return *this; }
    inline QBitArray(QBitArray &&other) noexcept : d(std::move(other.d)) {}
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QBitArray)

    void swap(QBitArray &other) noexcept { d.swap(other.d); }

    inline qsizetype size() const { return (d.size() << 3) - *d.constData(); }
    inline qsizetype count() const { return (d.size() << 3) - *d.constData(); }
    qsizetype count(bool on) const;

    inline bool isEmpty() const { return d.isEmpty(); }
    inline bool isNull() const { return d.isNull(); }

    void resize(qsizetype size);

    inline void detach() { d.detach(); }
    inline bool isDetached() const { return d.isDetached(); }
    inline void clear() { d.clear(); }

    bool testBit(qsizetype i) const;
    void setBit(qsizetype i);
    void setBit(qsizetype i, bool val);
    void clearBit(qsizetype i);
    bool toggleBit(qsizetype i);

    bool at(qsizetype i) const;
    QBitRef operator[](qsizetype i);
    bool operator[](qsizetype i) const;

    QBitArray &operator&=(const QBitArray &);
    QBitArray &operator|=(const QBitArray &);
    QBitArray &operator^=(const QBitArray &);
    QBitArray operator~() const;

    inline bool operator==(const QBitArray &other) const { return d == other.d; }
    inline bool operator!=(const QBitArray &other) const { return d != other.d; }

    inline bool fill(bool val, qsizetype size = -1);
    void fill(bool val, qsizetype first, qsizetype last);

    inline void truncate(qsizetype pos) { if (pos < size()) resize(pos); }

    const char *bits() const { return isEmpty() ? nullptr : d.constData() + 1; }
    static QBitArray fromBits(const char *data, qsizetype len);

    quint32 toUInt32(QSysInfo::Endian endianness, bool *ok = nullptr) const noexcept;

public:
    typedef QByteArray::DataPointer DataPtr;
    inline DataPtr &data_ptr() { return d.data_ptr(); }
};

inline bool QBitArray::fill(bool aval, qsizetype asize)
{ *this = QBitArray((asize < 0 ? this->size() : asize), aval); return true; }

Q_CORE_EXPORT QBitArray operator&(const QBitArray &, const QBitArray &);
Q_CORE_EXPORT QBitArray operator|(const QBitArray &, const QBitArray &);
Q_CORE_EXPORT QBitArray operator^(const QBitArray &, const QBitArray &);

inline bool QBitArray::testBit(qsizetype i) const
{ Q_ASSERT(size_t(i) < size_t(size()));
 return (*(reinterpret_cast<const uchar*>(d.constData())+1+(i>>3)) & (1 << (i & 7))) != 0; }

inline void QBitArray::setBit(qsizetype i)
{ Q_ASSERT(size_t(i) < size_t(size()));
 *(reinterpret_cast<uchar*>(d.data())+1+(i>>3)) |= uchar(1 << (i & 7)); }

inline void QBitArray::clearBit(qsizetype i)
{ Q_ASSERT(size_t(i) < size_t(size()));
 *(reinterpret_cast<uchar*>(d.data())+1+(i>>3)) &= ~uchar(1 << (i & 7)); }

inline void QBitArray::setBit(qsizetype i, bool val)
{ if (val) setBit(i); else clearBit(i); }

inline bool QBitArray::toggleBit(qsizetype i)
{ Q_ASSERT(size_t(i) < size_t(size()));
 uchar b = uchar(1<<(i&7)); uchar* p = reinterpret_cast<uchar*>(d.data())+1+(i>>3);
 uchar c = uchar(*p&b); *p^=b; return c!=0; }

inline bool QBitArray::operator[](qsizetype i) const { return testBit(i); }
inline bool QBitArray::at(qsizetype i) const { return testBit(i); }

class Q_CORE_EXPORT QBitRef
{
private:
    QBitArray &a;
    qsizetype i;
    inline QBitRef(QBitArray &array, qsizetype idx) : a(array), i(idx) { }
    friend class QBitArray;

public:
    inline operator bool() const { return a.testBit(i); }
    inline bool operator!() const { return !a.testBit(i); }
    QBitRef &operator=(const QBitRef &val) { a.setBit(i, val); return *this; }
    QBitRef &operator=(bool val) { a.setBit(i, val); return *this; }
};

inline QBitRef QBitArray::operator[](qsizetype i)
{ Q_ASSERT(i >= 0); return QBitRef(*this, i); }

#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QBitArray &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QBitArray &);
#endif

#ifndef QT_NO_DEBUG_STREAM
Q_CORE_EXPORT QDebug operator<<(QDebug, const QBitArray &);
#endif

Q_DECLARE_SHARED(QBitArray)

QT_END_NAMESPACE

#endif // QBITARRAY_H
