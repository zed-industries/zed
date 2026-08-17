// Copyright (C) 2021 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QMATH_H
#define QMATH_H

#if 0
#pragma qt_class(QtMath)
#endif

#include <QtCore/qglobal.h>
#include <QtCore/qalgorithms.h>

#if __has_include(<bit>) && __cplusplus > 201703L
#include <bit>
#endif

#ifndef _USE_MATH_DEFINES
#  define _USE_MATH_DEFINES
#  define undef_USE_MATH_DEFINES
#endif

#include <cmath>

#ifdef undef_USE_MATH_DEFINES
#  undef _USE_MATH_DEFINES
#  undef undef_USE_MATH_DEFINES
#endif

QT_BEGIN_NAMESPACE

#define QT_SINE_TABLE_SIZE 256

extern Q_CORE_EXPORT const qreal qt_sine_table[QT_SINE_TABLE_SIZE];

template <typename T> int qCeil(T v)
{
    using std::ceil;
    return int(ceil(v));
}

template <typename T> int qFloor(T v)
{
    using std::floor;
    return int(floor(v));
}

template <typename T> auto qFabs(T v)
{
    using std::fabs;
    return fabs(v);
}

template <typename T> auto qSin(T v)
{
    using std::sin;
    return sin(v);
}

template <typename T> auto qCos(T v)
{
    using std::cos;
    return cos(v);
}

template <typename T> auto qTan(T v)
{
    using std::tan;
    return tan(v);
}

template <typename T> auto qAcos(T v)
{
    using std::acos;
    return acos(v);
}

template <typename T> auto qAsin(T v)
{
    using std::asin;
    return asin(v);
}

template <typename T> auto qAtan(T v)
{
    using std::atan;
    return atan(v);
}

template <typename T1, typename T2> auto qAtan2(T1 y, T2 x)
{
    using std::atan2;
    return atan2(y, x);
}

template <typename T> auto qSqrt(T v)
{
    using std::sqrt;
    return sqrt(v);
}

namespace QtPrivate {
template <typename R, typename F> // For qfloat16 to specialize
struct QHypotType { using type = decltype(std::hypot(R(1), F(1))); };

// Implements hypot() without limiting number of arguments:
template <typename T>
class QHypotHelper
{
    T scale, total;
    template <typename F> friend class QHypotHelper;
    QHypotHelper(T first, T prior) : scale(first), total(prior) {}
public:
    QHypotHelper(T first) : scale(qAbs(first)), total(1) {}
    T result() const
    { return qIsFinite(scale) ? scale > 0 ? scale * T(std::sqrt(total)) : T(0) : scale; }

    template<typename F, typename ...Fs>
    auto add(F first, Fs... rest) const
    { return add(first).add(rest...); }

    template<typename F, typename R = typename QHypotType<T, F>::type>
    QHypotHelper<R> add(F next) const
    {
        if (qIsInf(scale) || (qIsNaN(scale) && !qIsInf(next)))
            return QHypotHelper<R>(scale, R(1));
        if (qIsNaN(next))
            return QHypotHelper<R>(next, R(1));
        const R val = qAbs(next);
        if (!(scale > 0) || qIsInf(next))
            return QHypotHelper<R>(val, R(1));
        if (!(val > 0))
            return QHypotHelper<R>(scale, total);
        if (val > scale) {
            const R ratio = scale / next;
            return QHypotHelper<R>(val, total * ratio * ratio + 1);
        }
        const R ratio = next / scale;
        return QHypotHelper<R>(scale, total + ratio * ratio);
    }
};
} // QtPrivate

template<typename F, typename ...Fs>
auto qHypot(F first, Fs... rest)
{
    return QtPrivate::QHypotHelper<F>(first).add(rest...).result();
}

// However, where possible, use the standard library implementations:
template <typename Tx, typename Ty>
auto qHypot(Tx x, Ty y)
{
    // C99 has hypot(), hence C++11 has std::hypot()
    using std::hypot;
    return hypot(x, y);
}

#if defined(__cpp_lib_hypot) && __cpp_lib_hypot >= 201603L // Expected to be true
template <typename Tx, typename Ty, typename Tz>
auto qHypot(Tx x, Ty y, Tz z)
{
    using std::hypot;
    return hypot(x, y, z);
}
#endif // else: no need to over-ride the arbitrarily-many-arg form

template <typename T> auto qLn(T v)
{
    using std::log;
    return log(v);
}

template <typename T> auto qExp(T v)
{
    using std::exp;
    return exp(v);
}

template <typename T1, typename T2> auto qPow(T1 x, T2 y)
{
    using std::pow;
    return pow(x, y);
}

// TODO: use template variables (e.g. Qt::pi<type>) for these once we have C++14 support:

#ifndef M_E
#define M_E (2.7182818284590452354)
#endif

#ifndef M_LOG2E
#define M_LOG2E (1.4426950408889634074)
#endif

#ifndef M_LOG10E
#define M_LOG10E (0.43429448190325182765)
#endif

#ifndef M_LN2
#define M_LN2 (0.69314718055994530942)
#endif

#ifndef M_LN10
#define M_LN10 (2.30258509299404568402)
#endif

#ifndef M_PI
#define M_PI (3.14159265358979323846)
#endif

#ifndef M_PI_2
#define M_PI_2 (1.57079632679489661923)
#endif

#ifndef M_PI_4
#define M_PI_4 (0.78539816339744830962)
#endif

#ifndef M_1_PI
#define M_1_PI (0.31830988618379067154)
#endif

#ifndef M_2_PI
#define M_2_PI (0.63661977236758134308)
#endif

#ifndef M_2_SQRTPI
#define M_2_SQRTPI (1.12837916709551257390)
#endif

#ifndef M_SQRT2
#define M_SQRT2 (1.41421356237309504880)
#endif

#ifndef M_SQRT1_2
#define M_SQRT1_2 (0.70710678118654752440)
#endif

inline qreal qFastSin(qreal x)
{
    int si = int(x * (0.5 * QT_SINE_TABLE_SIZE / M_PI)); // Would be more accurate with qRound, but slower.
    qreal d = x - si * (2.0 * M_PI / QT_SINE_TABLE_SIZE);
    int ci = si + QT_SINE_TABLE_SIZE / 4;
    si &= QT_SINE_TABLE_SIZE - 1;
    ci &= QT_SINE_TABLE_SIZE - 1;
    return qt_sine_table[si] + (qt_sine_table[ci] - 0.5 * qt_sine_table[si] * d) * d;
}

inline qreal qFastCos(qreal x)
{
    int ci = int(x * (0.5 * QT_SINE_TABLE_SIZE / M_PI)); // Would be more accurate with qRound, but slower.
    qreal d = x - ci * (2.0 * M_PI / QT_SINE_TABLE_SIZE);
    int si = ci + QT_SINE_TABLE_SIZE / 4;
    si &= QT_SINE_TABLE_SIZE - 1;
    ci &= QT_SINE_TABLE_SIZE - 1;
    return qt_sine_table[si] - (qt_sine_table[ci] + 0.5 * qt_sine_table[si] * d) * d;
}

constexpr inline float qDegreesToRadians(float degrees)
{
    return degrees * float(M_PI / 180);
}

constexpr inline double qDegreesToRadians(double degrees)
{
    return degrees * (M_PI / 180);
}

constexpr inline long double qDegreesToRadians(long double degrees)
{
    return degrees * (M_PI / 180);
}

template <typename T, std::enable_if_t<std::is_integral_v<T>, bool> = true>
constexpr inline double qDegreesToRadians(T degrees)
{
    return qDegreesToRadians(static_cast<double>(degrees));
}

constexpr inline float qRadiansToDegrees(float radians)
{
    return radians * float(180 / M_PI);
}

constexpr inline double qRadiansToDegrees(double radians)
{
    return radians * (180 / M_PI);
}

constexpr inline long double qRadiansToDegrees(long double radians)
{
    return radians * (180 / M_PI);
}

// A qRadiansToDegrees(Integral) overload isn't here; it's extremely
// questionable that someone is manipulating quantities in radians
// using integral datatypes...

namespace QtPrivate {
constexpr inline quint32 qConstexprNextPowerOfTwo(quint32 v)
{
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    ++v;
    return v;
}

constexpr inline quint64 qConstexprNextPowerOfTwo(quint64 v)
{
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v |= v >> 32;
    ++v;
    return v;
}

constexpr inline quint32 qConstexprNextPowerOfTwo(qint32 v)
{
    return qConstexprNextPowerOfTwo(quint32(v));
}

constexpr inline quint64 qConstexprNextPowerOfTwo(qint64 v)
{
    return qConstexprNextPowerOfTwo(quint64(v));
}
} // namespace QtPrivate

constexpr inline quint32 qNextPowerOfTwo(quint32 v)
{
    Q_ASSERT(static_cast<qint32>(v) >= 0); // There is a next power of two
#if defined(__cpp_lib_int_pow2) && __cpp_lib_int_pow2 >= 202002L
    return std::bit_ceil(v + 1);
#elif defined(QT_HAS_BUILTIN_CLZ)
    if (v == 0)
        return 1;
    return 2U << (31 ^ QAlgorithmsPrivate::qt_builtin_clz(v));
#else
    return QtPrivate::qConstexprNextPowerOfTwo(v);
#endif
}

constexpr inline quint64 qNextPowerOfTwo(quint64 v)
{
    Q_ASSERT(static_cast<qint64>(v) >= 0); // There is a next power of two
#if defined(__cpp_lib_int_pow2) && __cpp_lib_int_pow2 >= 202002L
    return std::bit_ceil(v + 1);
#elif defined(QT_HAS_BUILTIN_CLZLL)
    if (v == 0)
        return 1;
    return Q_UINT64_C(2) << (63 ^ QAlgorithmsPrivate::qt_builtin_clzll(v));
#else
    return QtPrivate::qConstexprNextPowerOfTwo(v);
#endif
}

constexpr inline quint32 qNextPowerOfTwo(qint32 v)
{
    return qNextPowerOfTwo(quint32(v));
}

constexpr inline quint64 qNextPowerOfTwo(qint64 v)
{
    return qNextPowerOfTwo(quint64(v));
}

QT_END_NAMESPACE

#endif // QMATH_H
