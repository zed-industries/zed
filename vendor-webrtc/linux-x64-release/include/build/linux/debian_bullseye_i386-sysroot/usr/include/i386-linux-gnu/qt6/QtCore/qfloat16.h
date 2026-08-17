// Copyright (C) 2022 The Qt Company Ltd.
// Copyright (C) 2016 by Southwest Research Institute (R)
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QFLOAT16_H
#define QFLOAT16_H

#include <QtCore/qglobal.h>
#include <QtCore/qmetatype.h>
#include <QtCore/qnamespace.h>
#include <limits>
#include <string.h>

#if defined(QT_COMPILER_SUPPORTS_F16C) && defined(__AVX2__) && !defined(__F16C__)
// All processors that support AVX2 do support F16C too, so we could enable the
// feature unconditionally if __AVX2__ is defined. However, all currently
// supported compilers except Microsoft's are able to define __F16C__ on their
// own when the user enables the feature, so we'll trust them.
#  if defined(Q_CC_MSVC) && !defined(Q_CC_CLANG)
#    define __F16C__        1
#  endif
#endif

#if defined(QT_COMPILER_SUPPORTS_F16C) && defined(__F16C__)
#include <immintrin.h>
#endif

QT_BEGIN_NAMESPACE

#if 0
#pragma qt_class(QFloat16)
#pragma qt_no_master_include
#endif

#ifndef QT_NO_DATASTREAM
class QDataStream;
#endif

class qfloat16
{
    struct Wrap
    {
        // To let our private constructor work, without other code seeing
        // ambiguity when constructing from int, double &c.
        quint16 b16;
        constexpr inline explicit Wrap(int value) : b16(quint16(value)) {}
    };
public:
    constexpr inline qfloat16() noexcept : b16(0) {}
    explicit qfloat16(Qt::Initialization) noexcept { }
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
    Q_CORE_EXPORT static const quint16 basetable[];
    Q_CORE_EXPORT static const quint16 shifttable[];
    Q_CORE_EXPORT static const quint32 roundtable[];

    friend bool qIsNull(qfloat16 f) noexcept;

    friend inline qfloat16 operator-(qfloat16 a) noexcept
    {
        qfloat16 f;
        f.b16 = a.b16 ^ quint16(0x8000);
        return f;
    }

    friend inline qfloat16 operator+(qfloat16 a, qfloat16 b) noexcept { return qfloat16(static_cast<float>(a) + static_cast<float>(b)); }
    friend inline qfloat16 operator-(qfloat16 a, qfloat16 b) noexcept { return qfloat16(static_cast<float>(a) - static_cast<float>(b)); }
    friend inline qfloat16 operator*(qfloat16 a, qfloat16 b) noexcept { return qfloat16(static_cast<float>(a) * static_cast<float>(b)); }
    friend inline qfloat16 operator/(qfloat16 a, qfloat16 b) noexcept { return qfloat16(static_cast<float>(a) / static_cast<float>(b)); }

#define QF16_MAKE_ARITH_OP_FP(FP, OP) \
    friend inline FP operator OP(qfloat16 lhs, FP rhs) noexcept { return static_cast<FP>(lhs) OP rhs; } \
    friend inline FP operator OP(FP lhs, qfloat16 rhs) noexcept { return lhs OP static_cast<FP>(rhs); }
#define QF16_MAKE_ARITH_OP_EQ_FP(FP, OP_EQ, OP) \
    friend inline qfloat16& operator OP_EQ(qfloat16& lhs, FP rhs) noexcept \
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
    friend inline double operator OP(qfloat16 lhs, int rhs) noexcept { return static_cast<double>(lhs) OP rhs; } \
    friend inline double operator OP(int lhs, qfloat16 rhs) noexcept { return lhs OP static_cast<double>(rhs); }

    QF16_MAKE_ARITH_OP_INT(+)
    QF16_MAKE_ARITH_OP_INT(-)
    QF16_MAKE_ARITH_OP_INT(*)
    QF16_MAKE_ARITH_OP_INT(/)
#undef QF16_MAKE_ARITH_OP_INT

QT_WARNING_PUSH
QT_WARNING_DISABLE_FLOAT_COMPARE

    friend inline bool operator>(qfloat16 a, qfloat16 b)  noexcept { return static_cast<float>(a) >  static_cast<float>(b); }
    friend inline bool operator<(qfloat16 a, qfloat16 b)  noexcept { return static_cast<float>(a) <  static_cast<float>(b); }
    friend inline bool operator>=(qfloat16 a, qfloat16 b) noexcept { return static_cast<float>(a) >= static_cast<float>(b); }
    friend inline bool operator<=(qfloat16 a, qfloat16 b) noexcept { return static_cast<float>(a) <= static_cast<float>(b); }
    friend inline bool operator==(qfloat16 a, qfloat16 b) noexcept { return static_cast<float>(a) == static_cast<float>(b); }
    friend inline bool operator!=(qfloat16 a, qfloat16 b) noexcept { return static_cast<float>(a) != static_cast<float>(b); }

#define QF16_MAKE_BOOL_OP_FP(FP, OP) \
    friend inline bool operator OP(qfloat16 lhs, FP rhs) noexcept { return static_cast<FP>(lhs) OP rhs; } \
    friend inline bool operator OP(FP lhs, qfloat16 rhs) noexcept { return lhs OP static_cast<FP>(rhs); }
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
    friend inline bool operator OP(qfloat16 a, int b) noexcept { return static_cast<float>(a) OP static_cast<float>(b); } \
    friend inline bool operator OP(int a, qfloat16 b) noexcept { return static_cast<float>(a) OP static_cast<float>(b); }

    QF16_MAKE_BOOL_OP_INT(>)
    QF16_MAKE_BOOL_OP_INT(<)
    QF16_MAKE_BOOL_OP_INT(>=)
    QF16_MAKE_BOOL_OP_INT(<=)
    QF16_MAKE_BOOL_OP_INT(==)
    QF16_MAKE_BOOL_OP_INT(!=)
#undef QF16_MAKE_BOOL_OP_INT

QT_WARNING_POP

#ifndef QT_NO_DATASTREAM
    friend Q_CORE_EXPORT QDataStream &operator<<(QDataStream &ds, qfloat16 f);
    friend Q_CORE_EXPORT QDataStream &operator>>(QDataStream &ds, qfloat16 &f);
#endif
};

Q_DECLARE_TYPEINFO(qfloat16, Q_PRIMITIVE_TYPE);

Q_CORE_EXPORT void qFloatToFloat16(qfloat16 *, const float *, qsizetype length) noexcept;
Q_CORE_EXPORT void qFloatFromFloat16(float *, const qfloat16 *, qsizetype length) noexcept;

// Complement qnumeric.h:
[[nodiscard]] inline bool qIsInf(qfloat16 f) noexcept { return f.isInf(); }
[[nodiscard]] inline bool qIsNaN(qfloat16 f) noexcept { return f.isNaN(); }
[[nodiscard]] inline bool qIsFinite(qfloat16 f) noexcept { return f.isFinite(); }
[[nodiscard]] inline int qFpClassify(qfloat16 f) noexcept { return f.fpClassify(); }
// [[nodiscard]] quint32 qFloatDistance(qfloat16 a, qfloat16 b);

// The remainder of these utility functions complement qglobal.h
[[nodiscard]] inline int qRound(qfloat16 d) noexcept
{ return qRound(static_cast<float>(d)); }

[[nodiscard]] inline qint64 qRound64(qfloat16 d) noexcept
{ return qRound64(static_cast<float>(d)); }

[[nodiscard]] inline bool qFuzzyCompare(qfloat16 p1, qfloat16 p2) noexcept
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

/*!
  \internal
*/
[[nodiscard]] inline bool qFuzzyIsNull(qfloat16 f) noexcept
{
    return qAbs(f) < 0.00976f; // 1/102.5 to 3 significant digits; see qFuzzyCompare()
}

[[nodiscard]] inline bool qIsNull(qfloat16 f) noexcept
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
    const quint32 signAndExp = u >> 23;
    const quint16 base = basetable[signAndExp];
    const quint16 shift = shifttable[signAndExp];
    const quint32 round = roundtable[signAndExp];
    quint32 mantissa = (u & 0x007fffff);
    if ((signAndExp & 0xff) == 0xff) {
        if (mantissa) // keep nan from truncating to inf
            mantissa = qMax(1U << shift, mantissa);
    } else {
        // Round half to even. First round up by adding one in the most
        // significant bit we'll be discarding:
        mantissa += round;
        // If the last bit we'll be keeping is now set, but all later bits are
        // clear, we were at half and shouldn't have rounded up; decrement will
        // clear this last kept bit. Any later set bit hides the decrement.
        if (mantissa & (1 << shift))
            --mantissa;
    }

    // We use add as the mantissa may overflow causing
    // the exp part to shift exactly one value.
    b16 = quint16(base + (mantissa >> shift));
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

/*
  qHypot compatibility; see ../kernel/qmath.h
*/
namespace QtPrivate {
template <typename R>
struct QHypotType<R, qfloat16> { using type = decltype(std::hypot(R(1), 1.0f)); };
template <typename R>
struct QHypotType<qfloat16, R> { using type = decltype(std::hypot(1.0f, R(1))); };
template <> struct QHypotType<qfloat16, qfloat16> { using type = qfloat16; };
}
// Avoid passing qfloat16 to std::hypot(), while ensuring return types
// consistent with the above:
template<typename F, typename ...Fs> auto qHypot(F first, Fs... rest);
template <typename T, typename std::enable_if<!std::is_same<qfloat16, T>::value, int>::type = 0>
auto qHypot(T x, qfloat16 y) { return qHypot(x, float(y)); }
template <typename T, typename std::enable_if<!std::is_same<qfloat16, T>::value, int>::type = 0>
auto qHypot(qfloat16 x, T y) { return qHypot(float(x), y); }
template <> inline auto qHypot(qfloat16 x, qfloat16 y)
{
#if (defined(QT_COMPILER_SUPPORTS_F16C) && defined(__F16C__)) || defined (__ARM_FP16_FORMAT_IEEE)
    return QtPrivate::QHypotHelper<qfloat16>(x).add(y).result();
#else
    return qfloat16(qHypot(float(x), float(y)));
#endif
}
#if defined(__cpp_lib_hypot) && __cpp_lib_hypot >= 201603L // Expected to be true
// If any are not qfloat16, convert each qfloat16 to float:
/* (The following splits the some-but-not-all-qfloat16 cases up, using
   (X|Y|Z)&~(X&Y&Z) = X ? ~(Y&Z) : Y|Z = X&~(Y&Z) | ~X&Y | ~X&~Y&Z,
   into non-overlapping cases, to avoid ambiguity.) */
template <typename Ty, typename Tz,
          typename std::enable_if<
              // Ty, Tz aren't both qfloat16:
              !(std::is_same_v<qfloat16, Ty> && std::is_same_v<qfloat16, Tz>), int>::type = 0>
auto qHypot(qfloat16 x, Ty y, Tz z) { return qHypot(float(x), y, z); }
template <typename Tx, typename Tz,
          typename std::enable_if<
              // Tx isn't qfloat16:
              !std::is_same_v<qfloat16, Tx>, int>::type = 0>
auto qHypot(Tx x, qfloat16 y, Tz z) { return qHypot(x, float(y), z); }
template <typename Tx, typename Ty,
          typename std::enable_if<
              // Neither Tx nor Ty is qfloat16:
              !std::is_same_v<qfloat16, Tx> && !std::is_same_v<qfloat16, Ty>, int>::type = 0>
auto qHypot(Tx x, Ty y, qfloat16 z) { return qHypot(x, y, float(z)); }
// If all are qfloat16, stay with qfloat16 (albeit via float, if no native support):
template <>
inline auto qHypot(qfloat16 x, qfloat16 y, qfloat16 z)
{
#if (defined(QT_COMPILER_SUPPORTS_F16C) && defined(__F16C__)) || defined (__ARM_FP16_FORMAT_IEEE)
    return QtPrivate::QHypotHelper<qfloat16>(x).add(y).add(z).result();
#else
    return qfloat16(qHypot(float(x), float(y), float(z)));
#endif
}
#endif // 3-arg std::hypot() is available

QT_END_NAMESPACE

QT_DECL_METATYPE_EXTERN(qfloat16, Q_CORE_EXPORT)

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
