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

#ifndef QMATH_H
#define QMATH_H

#if 0
#pragma qt_class(QtMath)
#endif

#include <QtCore/qglobal.h>
#include <QtCore/qalgorithms.h>

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

inline int qCeil(qreal v)
{
    using std::ceil;
    return int(ceil(v));
}

inline int qFloor(qreal v)
{
    using std::floor;
    return int(floor(v));
}

inline qreal qFabs(qreal v)
{
    using std::fabs;
    return fabs(v);
}

inline qreal qSin(qreal v)
{
    using std::sin;
    return sin(v);
}

inline qreal qCos(qreal v)
{
    using std::cos;
    return cos(v);
}

inline qreal qTan(qreal v)
{
    using std::tan;
    return tan(v);
}

inline qreal qAcos(qreal v)
{
    using std::acos;
    return acos(v);
}

inline qreal qAsin(qreal v)
{
    using std::asin;
    return asin(v);
}

inline qreal qAtan(qreal v)
{
    using std::atan;
    return atan(v);
}

inline qreal qAtan2(qreal y, qreal x)
{
    using std::atan2;
    return atan2(y, x);
}

inline qreal qSqrt(qreal v)
{
    using std::sqrt;
    return sqrt(v);
}

inline qreal qLn(qreal v)
{
    using std::log;
    return log(v);
}

inline qreal qExp(qreal v)
{
    using std::exp;
    return exp(v);
}

inline qreal qPow(qreal x, qreal y)
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

Q_DECL_CONSTEXPR inline float qDegreesToRadians(float degrees)
{
    return degrees * float(M_PI/180);
}

Q_DECL_CONSTEXPR inline double qDegreesToRadians(double degrees)
{
    return degrees * (M_PI / 180);
}

Q_DECL_CONSTEXPR inline float qRadiansToDegrees(float radians)
{
    return radians * float(180/M_PI);
}

Q_DECL_CONSTEXPR inline double qRadiansToDegrees(double radians)
{
    return radians * (180 / M_PI);
}


Q_DECL_RELAXED_CONSTEXPR inline quint32 qNextPowerOfTwo(quint32 v)
{
#if defined(QT_HAS_BUILTIN_CLZ)
    if (v == 0)
        return 1;
    return 2U << (31 ^ QAlgorithmsPrivate::qt_builtin_clz(v));
#else
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    ++v;
    return v;
#endif
}

Q_DECL_RELAXED_CONSTEXPR inline quint64 qNextPowerOfTwo(quint64 v)
{
#if defined(QT_HAS_BUILTIN_CLZLL)
    if (v == 0)
        return 1;
    return Q_UINT64_C(2) << (63 ^ QAlgorithmsPrivate::qt_builtin_clzll(v));
#else
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v |= v >> 32;
    ++v;
    return v;
#endif
}

Q_DECL_RELAXED_CONSTEXPR inline quint32 qNextPowerOfTwo(qint32 v)
{
    return qNextPowerOfTwo(quint32(v));
}

Q_DECL_RELAXED_CONSTEXPR inline quint64 qNextPowerOfTwo(qint64 v)
{
    return qNextPowerOfTwo(quint64(v));
}

QT_END_NAMESPACE

#endif // QMATH_H
