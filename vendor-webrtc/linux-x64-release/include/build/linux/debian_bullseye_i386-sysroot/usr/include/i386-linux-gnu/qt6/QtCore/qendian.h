// Copyright (C) 2021 The Qt Company Ltd.
// Copyright (C) 2021 Intel Corporation.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QENDIAN_H
#define QENDIAN_H

#include <QtCore/qfloat16.h>
#include <QtCore/qglobal.h>

#include <limits>

// include stdlib.h and hope that it defines __GLIBC__ for glibc-based systems
#include <stdlib.h>
#include <string.h>

#ifdef min // MSVC
#undef min
#undef max
#endif

QT_BEGIN_NAMESPACE

/*
 * ENDIAN FUNCTIONS
*/

// Used to implement a type-safe and alignment-safe copy operation
// If you want to avoid the memcpy, you must write specializations for these functions
template <typename T> Q_ALWAYS_INLINE void qToUnaligned(const T src, void *dest)
{
    // Using sizeof(T) inside memcpy function produces internal compiler error with
    // MSVC2008/ARM in tst_endian -> use extra indirection to resolve size of T.
    const size_t size = sizeof(T);
#if __has_builtin(__builtin_memcpy)
    __builtin_memcpy
#else
    memcpy
#endif
            (dest, &src, size);
}

template <typename T> Q_ALWAYS_INLINE T qFromUnaligned(const void *src)
{
    T dest;
    const size_t size = sizeof(T);
#if __has_builtin(__builtin_memcpy)
    __builtin_memcpy
#else
    memcpy
#endif
            (&dest, src, size);
    return dest;
}

// These definitions are written so that they are recognized by most compilers
// as bswap and replaced with single instruction builtins if available.
inline constexpr quint64 qbswap_helper(quint64 source)
{
    return 0
        | ((source & Q_UINT64_C(0x00000000000000ff)) << 56)
        | ((source & Q_UINT64_C(0x000000000000ff00)) << 40)
        | ((source & Q_UINT64_C(0x0000000000ff0000)) << 24)
        | ((source & Q_UINT64_C(0x00000000ff000000)) << 8)
        | ((source & Q_UINT64_C(0x000000ff00000000)) >> 8)
        | ((source & Q_UINT64_C(0x0000ff0000000000)) >> 24)
        | ((source & Q_UINT64_C(0x00ff000000000000)) >> 40)
        | ((source & Q_UINT64_C(0xff00000000000000)) >> 56);
}

inline constexpr quint32 qbswap_helper(quint32 source)
{
    return 0
        | ((source & 0x000000ff) << 24)
        | ((source & 0x0000ff00) << 8)
        | ((source & 0x00ff0000) >> 8)
        | ((source & 0xff000000) >> 24);
}

inline constexpr quint16 qbswap_helper(quint16 source)
{
    return quint16( 0
                    | ((source & 0x00ff) << 8)
                    | ((source & 0xff00) >> 8) );
}

inline constexpr quint8 qbswap_helper(quint8 source)
{
    return source;
}

/*
 * T qbswap(T source).
 * Changes the byte order of a value from big-endian to little-endian or vice versa.
 * This function can be used if you are not concerned about alignment issues,
 * and it is therefore a bit more convenient and in most cases more efficient.
*/
template <typename T, typename = std::enable_if_t<std::is_integral_v<T>>>
inline constexpr T qbswap(T source)
{
    return T(qbswap_helper(typename QIntegerForSizeof<T>::Unsigned(source)));
}

// floating specializations
template<typename Float>
Float qbswapFloatHelper(Float source)
{
    // memcpy call in qFromUnaligned is recognized by optimizer as a correct way of type prunning
    auto temp = qFromUnaligned<typename QIntegerForSizeof<Float>::Unsigned>(&source);
    temp = qbswap(temp);
    return qFromUnaligned<Float>(&temp);
}

inline qfloat16 qbswap(qfloat16 source)
{
    return qbswapFloatHelper(source);
}

inline float qbswap(float source)
{
    return qbswapFloatHelper(source);
}

inline double qbswap(double source)
{
    return qbswapFloatHelper(source);
}

/*
 * qbswap(const T src, const void *dest);
 * Changes the byte order of \a src from big-endian to little-endian or vice versa
 * and stores the result in \a dest.
 * There is no alignment requirements for \a dest.
*/
template <typename T> inline void qbswap(const T src, void *dest)
{
    qToUnaligned<T>(qbswap(src), dest);
}

template <int Size> void *qbswap(const void *source, qsizetype count, void *dest) noexcept;
template<> inline void *qbswap<1>(const void *source, qsizetype count, void *dest) noexcept
{
    return source != dest ? memcpy(dest, source, size_t(count)) : dest;
}
template<> Q_CORE_EXPORT void *qbswap<2>(const void *source, qsizetype count, void *dest) noexcept;
template<> Q_CORE_EXPORT void *qbswap<4>(const void *source, qsizetype count, void *dest) noexcept;
template<> Q_CORE_EXPORT void *qbswap<8>(const void *source, qsizetype count, void *dest) noexcept;

#if Q_BYTE_ORDER == Q_BIG_ENDIAN

template <typename T> inline constexpr T qToBigEndian(T source)
{ return source; }
template <typename T> inline constexpr T qFromBigEndian(T source)
{ return source; }
template <typename T> inline constexpr T qToLittleEndian(T source)
{ return qbswap(source); }
template <typename T> inline constexpr T qFromLittleEndian(T source)
{ return qbswap(source); }
template <typename T> inline void qToBigEndian(T src, void *dest)
{ qToUnaligned<T>(src, dest); }
template <typename T> inline void qToLittleEndian(T src, void *dest)
{ qbswap<T>(src, dest); }

template <typename T> inline void qToBigEndian(const void *source, qsizetype count, void *dest)
{ if (source != dest) memcpy(dest, source, count * sizeof(T)); }
template <typename T> inline void qToLittleEndian(const void *source, qsizetype count, void *dest)
{ qbswap<sizeof(T)>(source, count, dest); }
template <typename T> inline void qFromBigEndian(const void *source, qsizetype count, void *dest)
{ if (source != dest) memcpy(dest, source, count * sizeof(T)); }
template <typename T> inline void qFromLittleEndian(const void *source, qsizetype count, void *dest)
{ qbswap<sizeof(T)>(source, count, dest); }
#else // Q_LITTLE_ENDIAN

template <typename T> inline constexpr T qToBigEndian(T source)
{ return qbswap(source); }
template <typename T> inline constexpr T qFromBigEndian(T source)
{ return qbswap(source); }
template <typename T> inline constexpr T qToLittleEndian(T source)
{ return source; }
template <typename T> inline constexpr T qFromLittleEndian(T source)
{ return source; }
template <typename T> inline void qToBigEndian(T src, void *dest)
{ qbswap<T>(src, dest); }
template <typename T> inline void qToLittleEndian(T src, void *dest)
{ qToUnaligned<T>(src, dest); }

template <typename T> inline void qToBigEndian(const void *source, qsizetype count, void *dest)
{ qbswap<sizeof(T)>(source, count, dest); }
template <typename T> inline void qToLittleEndian(const void *source, qsizetype count, void *dest)
{ if (source != dest) memcpy(dest, source, count * sizeof(T)); }
template <typename T> inline void qFromBigEndian(const void *source, qsizetype count, void *dest)
{ qbswap<sizeof(T)>(source, count, dest); }
template <typename T> inline void qFromLittleEndian(const void *source, qsizetype count, void *dest)
{ if (source != dest) memcpy(dest, source, count * sizeof(T)); }
#endif // Q_BYTE_ORDER == Q_BIG_ENDIAN


/* T qFromLittleEndian(const void *src)
 * This function will read a little-endian encoded value from \a src
 * and return the value in host-endian encoding.
 * There is no requirement that \a src must be aligned.
*/
template <typename T> inline T qFromLittleEndian(const void *src)
{
    return qFromLittleEndian(qFromUnaligned<T>(src));
}

template <> inline quint8 qFromLittleEndian<quint8>(const void *src)
{ return static_cast<const quint8 *>(src)[0]; }
template <> inline qint8 qFromLittleEndian<qint8>(const void *src)
{ return static_cast<const qint8 *>(src)[0]; }

/* This function will read a big-endian (also known as network order) encoded value from \a src
 * and return the value in host-endian encoding.
 * There is no requirement that \a src must be aligned.
*/
template <class T> inline T qFromBigEndian(const void *src)
{
    return qFromBigEndian(qFromUnaligned<T>(src));
}

template <> inline quint8 qFromBigEndian<quint8>(const void *src)
{ return static_cast<const quint8 *>(src)[0]; }
template <> inline qint8 qFromBigEndian<qint8>(const void *src)
{ return static_cast<const qint8 *>(src)[0]; }

template<class S>
class QSpecialInteger
{
    typedef typename S::StorageType T;
    T val;
public:
    QSpecialInteger() = default;
    explicit constexpr QSpecialInteger(T i) : val(S::toSpecial(i)) {}

    QSpecialInteger &operator =(T i) { val = S::toSpecial(i); return *this; }
    operator T() const { return S::fromSpecial(val); }

    bool operator ==(QSpecialInteger<S> i) const { return val == i.val; }
    bool operator !=(QSpecialInteger<S> i) const { return val != i.val; }

    QSpecialInteger &operator +=(T i)
    {   return (*this = S::fromSpecial(val) + i); }
    QSpecialInteger &operator -=(T i)
    {   return (*this = S::fromSpecial(val) - i); }
    QSpecialInteger &operator *=(T i)
    {   return (*this = S::fromSpecial(val) * i); }
    QSpecialInteger &operator >>=(T i)
    {   return (*this = S::fromSpecial(val) >> i); }
    QSpecialInteger &operator <<=(T i)
    {   return (*this = S::fromSpecial(val) << i); }
    QSpecialInteger &operator /=(T i)
    {   return (*this = S::fromSpecial(val) / i); }
    QSpecialInteger &operator %=(T i)
    {   return (*this = S::fromSpecial(val) % i); }
    QSpecialInteger &operator |=(T i)
    {   return (*this = S::fromSpecial(val) | i); }
    QSpecialInteger &operator &=(T i)
    {   return (*this = S::fromSpecial(val) & i); }
    QSpecialInteger &operator ^=(T i)
    {   return (*this = S::fromSpecial(val) ^ i); }
    QSpecialInteger &operator ++()
    {   return (*this = S::fromSpecial(val) + 1); }
    QSpecialInteger &operator --()
    {   return (*this = S::fromSpecial(val) - 1); }
    QSpecialInteger operator ++(int)
    {
        QSpecialInteger<S> pre = *this;
        *this += 1;
        return pre;
    }
    QSpecialInteger operator --(int)
    {
        QSpecialInteger<S> pre = *this;
        *this -= 1;
        return pre;
    }

    static constexpr QSpecialInteger max()
    { return QSpecialInteger((std::numeric_limits<T>::max)()); }
    static constexpr QSpecialInteger min()
    { return QSpecialInteger((std::numeric_limits<T>::min)()); }
};

template<typename T>
class QLittleEndianStorageType {
public:
    typedef T StorageType;
    static constexpr T toSpecial(T source) { return qToLittleEndian(source); }
    static constexpr T fromSpecial(T source) { return qFromLittleEndian(source); }
};

template<typename T>
class QBigEndianStorageType {
public:
    typedef T StorageType;
    static constexpr T toSpecial(T source) { return qToBigEndian(source); }
    static constexpr T fromSpecial(T source) { return qFromBigEndian(source); }
};

#ifdef Q_CLANG_QDOC
template<typename T>
class QLEInteger {
public:
    explicit constexpr QLEInteger(T i);
    QLEInteger &operator =(T i);
    operator T() const;
    bool operator ==(QLEInteger i) const;
    bool operator !=(QLEInteger i) const;
    QLEInteger &operator +=(T i);
    QLEInteger &operator -=(T i);
    QLEInteger &operator *=(T i);
    QLEInteger &operator >>=(T i);
    QLEInteger &operator <<=(T i);
    QLEInteger &operator /=(T i);
    QLEInteger &operator %=(T i);
    QLEInteger &operator |=(T i);
    QLEInteger &operator &=(T i);
    QLEInteger &operator ^=(T i);
    QLEInteger &operator ++();
    QLEInteger &operator --();
    QLEInteger operator ++(int);
    QLEInteger operator --(int);

    static constexpr QLEInteger max();
    static constexpr QLEInteger min();
};

template<typename T>
class QBEInteger {
public:
    explicit constexpr QBEInteger(T i);
    QBEInteger &operator =(T i);
    operator T() const;
    bool operator ==(QBEInteger i) const;
    bool operator !=(QBEInteger i) const;
    QBEInteger &operator +=(T i);
    QBEInteger &operator -=(T i);
    QBEInteger &operator *=(T i);
    QBEInteger &operator >>=(T i);
    QBEInteger &operator <<=(T i);
    QBEInteger &operator /=(T i);
    QBEInteger &operator %=(T i);
    QBEInteger &operator |=(T i);
    QBEInteger &operator &=(T i);
    QBEInteger &operator ^=(T i);
    QBEInteger &operator ++();
    QBEInteger &operator --();
    QBEInteger operator ++(int);
    QBEInteger operator --(int);

    static constexpr QBEInteger max();
    static constexpr QBEInteger min();
};
#else

template<typename T>
using QLEInteger = QSpecialInteger<QLittleEndianStorageType<T>>;

template<typename T>
using QBEInteger = QSpecialInteger<QBigEndianStorageType<T>>;
#endif
template <typename T>
class QTypeInfo<QLEInteger<T> >
    : public QTypeInfoMerger<QLEInteger<T>, T> {};

template <typename T>
class QTypeInfo<QBEInteger<T> >
    : public QTypeInfoMerger<QBEInteger<T>, T> {};

typedef QLEInteger<qint16> qint16_le;
typedef QLEInteger<qint32> qint32_le;
typedef QLEInteger<qint64> qint64_le;
typedef QLEInteger<quint16> quint16_le;
typedef QLEInteger<quint32> quint32_le;
typedef QLEInteger<quint64> quint64_le;

typedef QBEInteger<qint16> qint16_be;
typedef QBEInteger<qint32> qint32_be;
typedef QBEInteger<qint64> qint64_be;
typedef QBEInteger<quint16> quint16_be;
typedef QBEInteger<quint32> quint32_be;
typedef QBEInteger<quint64> quint64_be;

QT_END_NAMESPACE

#endif // QENDIAN_H
