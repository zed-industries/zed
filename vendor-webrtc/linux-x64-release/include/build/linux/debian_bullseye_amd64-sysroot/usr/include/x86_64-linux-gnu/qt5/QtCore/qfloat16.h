/****************************************************************************
**
** Copyright (C) 2019 The Qt Company Ltd.
** Copyright (C) 2016 by Southwest Research Institute (R)
** Contact: http://www.qt-project.org/legal
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

#ifndef QFLOAT16_H
#define QFLOAT16_H

#include <QtCore/qglobal.h>
#include <QtCore/qmetatype.h>
#include <limits>
#include <string.h>

#if defined(QT_COMPILER_SUPPORTS_F16C) && defined(__AVX2__) && !defined(__F16C__)
// All processors that support AVX2 do support F16C too. That doesn't mean
// we're allowed to use the intrinsics directly, so we'll do it only for
// the Intel and Microsoft's compilers.
#  if defined(Q_CC_INTEL) || defined(Q_CC_MSVC)
#    define __F16C__        1
# endif
#endif

#if defined(QT_COMPILER_SUPPORTS_F16C) && defined(__F16C__)
#include <immintrin.h>
#endif

QT_BEGIN_NAMESPACE

#if 0
#pragma qt_class(QFloat16)
#pragma qt_no_master_include
#endif

class qfloat16
{
    struct Wrap
    {
        // To let our private constructor work, without other code seeing
        // ambiguity when constructing from int, double &c.
        quint16 b16;
        constexpr inline explicit Wrap(int value) : b16(value) {}
    };
public:
    constexpr inline qfloat16() noexcept : b16(0) {}
    inline qfloat16(float f) noexcept;
    inline operator float() const noexcept;

    // Support for qIs{Inf,NaN,Finite}:
    bool isInf() const noexcept { return (b16 & 0x7fff) == 0x7c00; }
    bool isNaN() const noexcept { return (b16 & 0x7fff) > 0x7c00; }
    bool isFinite() const noexcept { return (b16 & 0x7fff) < 0x7c00; }
    Q_CORE_EXPORT int fpClassify() const noexcept;
    // Can't specialize std::copysign() for qfloat16
    qfloat16 copySign(qfloat16 sign) const noexcept
    { return qfloat16(Wrap((sign.b16 & 0x8000) | (b16 & 0x7fff))); }
    // Support for std::numeric_limits<qfloat16>
    static constexpr qfloat16 _limit_epsilon()    noexcept { return qfloat16(Wrap(0x1400)); }
    static constexpr qfloat16 _limit_min()        noexcept { return qfloat16(Wrap(0x400)); }
    static constexpr qfloat16 _limit_denorm_min() noexcept { return qfloat16(Wrap(1)); }
    static constexpr qfloat16 _limit_max()        noexcept { return qfloat16(Wrap(0x7bff)); }
    static constexpr qfloat16 _limit_lowest()     noexcept { return qfloat16(Wrap(0xfbff)); }
    static constexpr qfloat16 _limit_infinity()   noexcept { return qfloat16(Wrap(0x7c00)); }
    static constexpr qfloat16 _limit_quiet_NaN()  noexcept { return qfloat16(Wrap(0x7e00)); }
#if QT_CONFIG(signaling_nan)
    static constexpr qfloat16 _limit_signaling_NaN() noexcept { return qfloat16(Wrap(0x7d00)); }
#endif
    inline constexpr bool isNormal() const noexcept
    { return (b16 & 0x7c00) && (b16 & 0x7c00) != 0x7c00; }
private:
    quint16 b16;
    constexpr inline explicit qfloat16(Wrap nibble) noexcept : b16(nibble.b16) {}

    Q_CORE_EXPORT static const quint32 mantissatable[];
    Q_CORE_EXPORT static const quint32 exponenttable[];
    Q_CORE_EXPORT static const quint32 offsettable[];
    Q_CORE_EXPORT static const quint32 basetable[];
    Q_CORE_EXPORT static const quint32 shifttable[];

    friend bool qIsNull(qfloat16 f) noexcept;
#if !defined(QT_NO_FLOAT16_OPERATORS)
    friend qfloat16 operator-(qfloat16 a) noexcept;
#endif
};

Q_DECLARE_TYPEINFO(qfloat16, Q_PRIMITIVE_TYPE);

Q_CORE_EXPORT void qFloatToFloat16(qfloat16 *, const float *, qsizetype length) noexcept;
Q_CORE_EXPORT void qFloatFromFloat16(float *, const qfloat16 *, qsizetype length) noexcept;

// Complement qnumeric.h:
Q_REQUIRED_RESULT inline bool qIsInf(qfloat16 f) noexcept { return f.isInf(); }
Q_REQUIRED_RESULT inline bool qIsNaN(qfloat16 f) noexcept { return f.isNaN(); }
Q_REQUIRED_RESULT inline bool qIsFinite(qfloat16 f) noexcept { return f.isFinite(); }
Q_REQUIRED_RESULT inline int qFpClassify(qfloat16 f) noexcept { return f.fpClassify(); }
// Q_REQUIRED_RESULT quint32 qFloatDistance(qfloat16 a, qfloat16 b);

// The remainder of these utility functions complement qglobal.h
Q_REQUIRED_RESULT inline int qRound(qfloat16 d) noexcept
{ return qRound(static_cast<float>(d)); }

Q_REQUIRED_RESULT inline qint64 qRound64(qfloat16 d) noexcept
{ return qRound64(static_cast<float>(d)); }

Q_REQUIRED_RESULT inline bool qFuzzyCompare(qfloat16 p1, qfloat16 p2) noexcept
{
    float f1 = static_cast<float>(p1);
    float f2 = static_cast<float>(p2);
    // The significand precision for IEEE754 half precision is
    // 11 bits (10 explicitly stored), or approximately 3 decimal
    // digits.  In selecting the fuzzy comparison factor of 102.5f
    // (that is, (2^10+1)/10) below, we effectively select a
    // window of about 1 (least significant) decimal digit about
    // which the two operands can vary and still return true.
    return (qAbs(f1 - f2) * 102.5f <= qMin(qAbs(f1), qAbs(f2)));
}

Q_REQUIRED_RESULT inline bool qIsNull(qfloat16 f) noexcept
{
    return (f.b16 & static_cast<quint16>(0x7fff)) == 0;
}

inline int qIntCast(qfloat16 f) noexcept
{ return int(static_cast<float>(f)); }

#ifndef Q_QDOC
QT_WARNING_PUSH
QT_WARNING_DISABLE_CLANG("-Wc99-extensions")
QT_WARNING_DISABLE_GCC("-Wold-style-cast")
inline qfloat16::qfloat16(float f) noexcept
{
#if defined(QT_COMPILER_SUPPORTS_F16C) && defined(__F16C__)
    __m128 packsingle = _mm_set_ss(f);
    __m128i packhalf = _mm_cvtps_ph(packsingle, 0);
    b16 = _mm_extract_epi16(packhalf, 0);
#elif defined (__ARM_FP16_FORMAT_IEEE)
    __fp16 f16 = __fp16(f);
    memcpy(&b16, &f16, sizeof(quint16));
#else
    quint32 u;
    memcpy(&u, &f, sizeof(quint32));
    b16 = basetable[(u >> 23) & 0x1ff]
          + ((u & 0x007fffff) >> shifttable[(u >> 23) & 0x1ff]);
#endif
}
QT_WARNING_POP

inline qfloat16::operator float() const noexcept
{
#if defined(QT_COMPILER_SUPPORTS_F16C) && defined(__F16C__)
    __m128i packhalf = _mm_cvtsi32_si128(b16);
    __m128 packsingle = _mm_cvtph_ps(packhalf);
    return _mm_cvtss_f32(packsingle);
#elif defined (__ARM_FP16_FORMAT_IEEE)
    __fp16 f16;
    memcpy(&f16, &b16, sizeof(quint16));
    return float(f16);
#else
    quint32 u = mantissatable[offsettable[b16 >> 10] + (b16 & 0x3ff)]
                + exponenttable[b16 >> 10];
    float f;
    memcpy(&f, &u, sizeof(quint32));
    return f;
#endif
}
#endif

#if !defined(QT_NO_FLOAT16_OPERATORS)
inline qfloat16 operator-(qfloat16 a) noexcept
{
    qfloat16 f;
    f.b16 = a.b16 ^ quint16(0x8000);
    return f;
}

inline qfloat16 operator+(qfloat16 a, qfloat16 b) noexcept { return qfloat16(static_cast<float>(a) + static_cast<float>(b)); }
inline qfloat16 operator-(qfloat16 a, qfloat16 b) noexcept { return qfloat16(static_cast<float>(a) - static_cast<float>(b)); }
inline qfloat16 operator*(qfloat16 a, qfloat16 b) noexcept { return qfloat16(static_cast<float>(a) * static_cast<float>(b)); }
inline qfloat16 operator/(qfloat16 a, qfloat16 b) noexcept { return qfloat16(static_cast<float>(a) / static_cast<float>(b)); }

#define QF16_MAKE_ARITH_OP_FP(FP, OP) \
    inline FP operator OP(qfloat16 lhs, FP rhs) noexcept { return static_cast<FP>(lhs) OP rhs; } \
    inline FP operator OP(FP lhs, qfloat16 rhs) noexcept { return lhs OP static_cast<FP>(rhs); }
#define QF16_MAKE_ARITH_OP_EQ_FP(FP, OP_EQ, OP) \
    inline qfloat16& operator OP_EQ(qfloat16& lhs, FP rhs) noexcept \
    { lhs = qfloat16(float(static_cast<FP>(lhs) OP rhs)); return lhs; }
#define QF16_MAKE_ARITH_OP(FP) \
    QF16_MAKE_ARITH_OP_FP(FP, +) \
    QF16_MAKE_ARITH_OP_FP(FP, -) \
    QF16_MAKE_ARITH_OP_FP(FP, *) \
    QF16_MAKE_ARITH_OP_FP(FP, /) \
    QF16_MAKE_ARITH_OP_EQ_FP(FP, +=, +) \
    QF16_MAKE_ARITH_OP_EQ_FP(FP, -=, -) \
    QF16_MAKE_ARITH_OP_EQ_FP(FP, *=, *) \
    QF16_MAKE_ARITH_OP_EQ_FP(FP, /=, /)
QF16_MAKE_ARITH_OP(long double)
QF16_MAKE_ARITH_OP(double)
QF16_MAKE_ARITH_OP(float)
#undef QF16_MAKE_ARITH_OP
#undef QF16_MAKE_ARITH_OP_FP

#define QF16_MAKE_ARITH_OP_INT(OP) \
    inline double operator OP(qfloat16 lhs, int rhs) noexcept { return static_cast<double>(lhs) OP rhs; } \
    inline double operator OP(int lhs, qfloat16 rhs) noexcept { return lhs OP static_cast<double>(rhs); }
QF16_MAKE_ARITH_OP_INT(+)
QF16_MAKE_ARITH_OP_INT(-)
QF16_MAKE_ARITH_OP_INT(*)
QF16_MAKE_ARITH_OP_INT(/)
#undef QF16_MAKE_ARITH_OP_INT

QT_WARNING_PUSH
QT_WARNING_DISABLE_CLANG("-Wfloat-equal")
QT_WARNING_DISABLE_GCC("-Wfloat-equal")
QT_WARNING_DISABLE_INTEL(1572)

inline bool operator>(qfloat16 a, qfloat16 b)  noexcept { return static_cast<float>(a) >  static_cast<float>(b); }
inline bool operator<(qfloat16 a, qfloat16 b)  noexcept { return static_cast<float>(a) <  static_cast<float>(b); }
inline bool operator>=(qfloat16 a, qfloat16 b) noexcept { return static_cast<float>(a) >= static_cast<float>(b); }
inline bool operator<=(qfloat16 a, qfloat16 b) noexcept { return static_cast<float>(a) <= static_cast<float>(b); }
inline bool operator==(qfloat16 a, qfloat16 b) noexcept { return static_cast<float>(a) == static_cast<float>(b); }
inline bool operator!=(qfloat16 a, qfloat16 b) noexcept { return static_cast<float>(a) != static_cast<float>(b); }

#define QF16_MAKE_BOOL_OP_FP(FP, OP) \
    inline bool operator OP(qfloat16 lhs, FP rhs) noexcept { return static_cast<FP>(lhs) OP rhs; } \
    inline bool operator OP(FP lhs, qfloat16 rhs) noexcept { return lhs OP static_cast<FP>(rhs); }
#define QF16_MAKE_BOOL_OP(FP) \
    QF16_MAKE_BOOL_OP_FP(FP, <) \
    QF16_MAKE_BOOL_OP_FP(FP, >) \
    QF16_MAKE_BOOL_OP_FP(FP, >=) \
    QF16_MAKE_BOOL_OP_FP(FP, <=) \
    QF16_MAKE_BOOL_OP_FP(FP, ==) \
    QF16_MAKE_BOOL_OP_FP(FP, !=)
QF16_MAKE_BOOL_OP(long double)
QF16_MAKE_BOOL_OP(double)
QF16_MAKE_BOOL_OP(float)
#undef QF16_MAKE_BOOL_OP
#undef QF16_MAKE_BOOL_OP_FP

#define QF16_MAKE_BOOL_OP_INT(OP) \
    inline bool operator OP(qfloat16 a, int b) noexcept { return static_cast<float>(a) OP b; } \
    inline bool operator OP(int a, qfloat16 b) noexcept { return a OP static_cast<float>(b); }
QF16_MAKE_BOOL_OP_INT(>)
QF16_MAKE_BOOL_OP_INT(<)
QF16_MAKE_BOOL_OP_INT(>=)
QF16_MAKE_BOOL_OP_INT(<=)
QF16_MAKE_BOOL_OP_INT(==)
QF16_MAKE_BOOL_OP_INT(!=)
#undef QF16_MAKE_BOOL_OP_INT

QT_WARNING_POP
#endif // QT_NO_FLOAT16_OPERATORS

/*!
  \internal
*/
Q_REQUIRED_RESULT inline bool qFuzzyIsNull(qfloat16 f) noexcept
{
    return qAbs(static_cast<float>(f)) <= 0.001f;
}

QT_END_NAMESPACE

Q_DECLARE_METATYPE(qfloat16)

namespace std {
template<>
class numeric_limits<QT_PREPEND_NAMESPACE(qfloat16)> : public numeric_limits<float>
{
public:
    /*
      Treat quint16 b16 as if it were:
      uint S: 1; // b16 >> 15 (sign); can be set for zero
      uint E: 5; // (b16 >> 10) & 0x1f (offset exponent)
      uint M: 10; // b16 & 0x3ff (adjusted mantissa)

      for E == 0: magnitude is M / 2.^{24}
      for 0 < E < 31: magnitude is (1. + M / 2.^{10}) * 2.^{E - 15)
      for E == 31: not finite
     */
    static constexpr int digits = 11;
    static constexpr int min_exponent = -13;
    static constexpr int max_exponent = 16;

    static constexpr int digits10 = 3;
    static constexpr int max_digits10 = 5;
    static constexpr int min_exponent10 = -4;
    static constexpr int max_exponent10 = 4;

    static constexpr QT_PREPEND_NAMESPACE(qfloat16) epsilon()
    { return QT_PREPEND_NAMESPACE(qfloat16)::_limit_epsilon(); }
    static constexpr QT_PREPEND_NAMESPACE(qfloat16) (min)()
    { return QT_PREPEND_NAMESPACE(qfloat16)::_limit_min(); }
    static constexpr QT_PREPEND_NAMESPACE(qfloat16) denorm_min()
    { return QT_PREPEND_NAMESPACE(qfloat16)::_limit_denorm_min(); }
    static constexpr QT_PREPEND_NAMESPACE(qfloat16) (max)()
    { return QT_PREPEND_NAMESPACE(qfloat16)::_limit_max(); }
    static constexpr QT_PREPEND_NAMESPACE(qfloat16) lowest()
    { return QT_PREPEND_NAMESPACE(qfloat16)::_limit_lowest(); }
    static constexpr QT_PREPEND_NAMESPACE(qfloat16) infinity()
    { return QT_PREPEND_NAMESPACE(qfloat16)::_limit_infinity(); }
    static constexpr QT_PREPEND_NAMESPACE(qfloat16) quiet_NaN()
    { return QT_PREPEND_NAMESPACE(qfloat16)::_limit_quiet_NaN(); }
#if QT_CONFIG(signaling_nan)
    static constexpr QT_PREPEND_NAMESPACE(qfloat16) signaling_NaN()
    { return QT_PREPEND_NAMESPACE(qfloat16)::_limit_signaling_NaN(); }
#else
    static constexpr bool has_signaling_NaN = false;
#endif
};

template<> class numeric_limits<const QT_PREPEND_NAMESPACE(qfloat16)>
    : public numeric_limits<QT_PREPEND_NAMESPACE(qfloat16)> {};
template<> class numeric_limits<volatile QT_PREPEND_NAMESPACE(qfloat16)>
    : public numeric_limits<QT_PREPEND_NAMESPACE(qfloat16)> {};
template<> class numeric_limits<const volatile QT_PREPEND_NAMESPACE(qfloat16)>
    : public numeric_limits<QT_PREPEND_NAMESPACE(qfloat16)> {};

// Adding overloads to std isn't allowed, so we can't extend this to support
// for fpclassify(), isnormal() &c. (which, furthermore, are macros on MinGW).
} // namespace std

#endif // QFLOAT16_H
