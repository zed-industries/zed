// Copyright (C) 2021 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QNUMERIC_H
#define QNUMERIC_H

#include <QtCore/qglobal.h>
#include <cmath>
#include <limits>
#include <type_traits>

// min() and max() may be #defined by windows.h if that is included before, but we need them
// for std::numeric_limits below. You should not use the min() and max() macros, so we just #undef.
#ifdef min
#  undef min
#  undef max
#endif

#if defined(Q_CC_MSVC)
#  include <intrin.h>
#  include <float.h>
#  if defined(Q_PROCESSOR_X86_64) || defined(Q_PROCESSOR_ARM_64)
#    define Q_INTRINSIC_MUL_OVERFLOW64
#    define Q_UMULH(v1, v2) __umulh(v1, v2);
#    define Q_SMULH(v1, v2) __mulh(v1, v2);
#    pragma intrinsic(__umulh)
#    pragma intrinsic(__mulh)
#  endif
#endif

# if defined(Q_OS_INTEGRITY) && defined(Q_PROCESSOR_ARM_64)
#  include <arm64_ghs.h>
#  define Q_INTRINSIC_MUL_OVERFLOW64
#  define Q_UMULH(v1, v2) __MULUH64(v1, v2);
#  define Q_SMULH(v1, v2) __MULSH64(v1, v2);
#endif

QT_BEGIN_NAMESPACE

// To match std::is{inf,nan,finite} functions:
template <typename T>
constexpr typename std::enable_if<std::is_integral<T>::value, bool>::type
qIsInf(T) { return false; }
template <typename T>
constexpr typename std::enable_if<std::is_integral<T>::value, bool>::type
qIsNaN(T) { return false; }
template <typename T>
constexpr typename std::enable_if<std::is_integral<T>::value, bool>::type
qIsFinite(T) { return true; }

// Floating-point types (see qfloat16.h for its overloads).
Q_CORE_EXPORT Q_DECL_CONST_FUNCTION bool qIsInf(double d);
Q_CORE_EXPORT Q_DECL_CONST_FUNCTION bool qIsNaN(double d);
Q_CORE_EXPORT Q_DECL_CONST_FUNCTION bool qIsFinite(double d);
Q_CORE_EXPORT Q_DECL_CONST_FUNCTION int qFpClassify(double val);
Q_CORE_EXPORT Q_DECL_CONST_FUNCTION bool qIsInf(float f);
Q_CORE_EXPORT Q_DECL_CONST_FUNCTION bool qIsNaN(float f);
Q_CORE_EXPORT Q_DECL_CONST_FUNCTION bool qIsFinite(float f);
Q_CORE_EXPORT Q_DECL_CONST_FUNCTION int qFpClassify(float val);

#if QT_CONFIG(signaling_nan)
Q_CORE_EXPORT Q_DECL_CONST_FUNCTION double qSNaN();
#endif
Q_CORE_EXPORT Q_DECL_CONST_FUNCTION double qQNaN();
Q_CORE_EXPORT Q_DECL_CONST_FUNCTION double qInf();

Q_CORE_EXPORT quint32 qFloatDistance(float a, float b);
Q_CORE_EXPORT quint64 qFloatDistance(double a, double b);

#define Q_INFINITY (QT_PREPEND_NAMESPACE(qInf)())
#if QT_CONFIG(signaling_nan)
#  define Q_SNAN (QT_PREPEND_NAMESPACE(qSNaN)())
#endif
#define Q_QNAN (QT_PREPEND_NAMESPACE(qQNaN)())

// Overflow math.
// This provides efficient implementations for int, unsigned, qsizetype and
// size_t. Implementations for 8- and 16-bit types will work but may not be as
// efficient. Implementations for 64-bit may be missing on 32-bit platforms.

#if (Q_CC_GNU >= 500 || __has_builtin(__builtin_add_overflow)) \
    && !(QT_POINTER_SIZE == 4 && defined(Q_CC_CLANG))
// GCC 5 and Clang 3.8 have builtins to detect overflows
// 32 bit Clang has the builtins but tries to link a library which hasn't
#define Q_INTRINSIC_MUL_OVERFLOW64

template <typename T> inline
typename std::enable_if_t<std::is_unsigned_v<T> || std::is_signed_v<T>, bool>
qAddOverflow(T v1, T v2, T *r)
{ return __builtin_add_overflow(v1, v2, r); }

template <typename T> inline
typename std::enable_if_t<std::is_unsigned_v<T> || std::is_signed_v<T>, bool>
qSubOverflow(T v1, T v2, T *r)
{ return __builtin_sub_overflow(v1, v2, r); }

template <typename T> inline
typename std::enable_if_t<std::is_unsigned_v<T> || std::is_signed_v<T>, bool>
qMulOverflow(T v1, T v2, T *r)
{ return __builtin_mul_overflow(v1, v2, r); }

#else
// Generic implementations

template <typename T> inline typename std::enable_if_t<std::is_unsigned_v<T>, bool>
qAddOverflow(T v1, T v2, T *r)
{
    // unsigned additions are well-defined
    *r = v1 + v2;
    return v1 > T(v1 + v2);
}

template <typename T> inline typename std::enable_if_t<std::is_signed_v<T>, bool>
qAddOverflow(T v1, T v2, T *r)
{
    // Here's how we calculate the overflow:
    // 1) unsigned addition is well-defined, so we can always execute it
    // 2) conversion from unsigned back to signed is implementation-
    //    defined and in the implementations we use, it's a no-op.
    // 3) signed integer overflow happens if the sign of the two input operands
    //    is the same but the sign of the result is different. In other words,
    //    the sign of the result must be the same as the sign of either
    //    operand.

    using U = typename std::make_unsigned_t<T>;
    *r = T(U(v1) + U(v2));

    // If int is two's complement, assume all integer types are too.
    if (std::is_same_v<int32_t, int>) {
        // Two's complement equivalent (generates slightly shorter code):
        //  x ^ y             is negative if x and y have different signs
        //  x & y             is negative if x and y are negative
        // (x ^ z) & (y ^ z)  is negative if x and z have different signs
        //                    AND y and z have different signs
        return ((v1 ^ *r) & (v2 ^ *r)) < 0;
    }

    bool s1 = (v1 < 0);
    bool s2 = (v2 < 0);
    bool sr = (*r < 0);
    return s1 != sr && s2 != sr;
    // also: return s1 == s2 && s1 != sr;
}

template <typename T> inline typename std::enable_if_t<std::is_unsigned_v<T>, bool>
qSubOverflow(T v1, T v2, T *r)
{
    // unsigned subtractions are well-defined
    *r = v1 - v2;
    return v1 < v2;
}

template <typename T> inline typename std::enable_if_t<std::is_signed_v<T>, bool>
qSubOverflow(T v1, T v2, T *r)
{
    // See above for explanation. This is the same with some signs reversed.
    // We can't use qAddOverflow(v1, -v2, r) because it would be UB if
    // v2 == std::numeric_limits<T>::min().

    using U = typename std::make_unsigned_t<T>;
    *r = T(U(v1) - U(v2));

    if (std::is_same_v<int32_t, int>)
        return ((v1 ^ *r) & (~v2 ^ *r)) < 0;

    bool s1 = (v1 < 0);
    bool s2 = !(v2 < 0);
    bool sr = (*r < 0);
    return s1 != sr && s2 != sr;
    // also: return s1 == s2 && s1 != sr;
}

template <typename T> inline
typename std::enable_if_t<std::is_unsigned_v<T> || std::is_signed_v<T>, bool>
qMulOverflow(T v1, T v2, T *r)
{
    // use the next biggest type
    // Note: for 64-bit systems where __int128 isn't supported, this will cause an error.
    using LargerInt = QIntegerForSize<sizeof(T) * 2>;
    using Larger = typename std::conditional_t<std::is_signed_v<T>,
            typename LargerInt::Signed, typename LargerInt::Unsigned>;
    Larger lr = Larger(v1) * Larger(v2);
    *r = T(lr);
    return lr > (std::numeric_limits<T>::max)() || lr < (std::numeric_limits<T>::min)();
}

# if defined(Q_INTRINSIC_MUL_OVERFLOW64)
template <> inline bool qMulOverflow(quint64 v1, quint64 v2, quint64 *r)
{
    *r = v1 * v2;
    return Q_UMULH(v1, v2);
}
template <> inline bool qMulOverflow(qint64 v1, qint64 v2, qint64 *r)
{
    // This is slightly more complex than the unsigned case above: the sign bit
    // of 'low' must be replicated as the entire 'high', so the only valid
    // values for 'high' are 0 and -1. Use unsigned multiply since it's the same
    // as signed for the low bits and use a signed right shift to verify that
    // 'high' is nothing but sign bits that match the sign of 'low'.

    qint64 high = Q_SMULH(v1, v2);
    *r = qint64(quint64(v1) * quint64(v2));
    return (*r >> 63) != high;
}

#   if defined(Q_OS_INTEGRITY) && defined(Q_PROCESSOR_ARM_64)
template <> inline bool qMulOverflow(uint64_t v1, uint64_t v2, uint64_t *r)
{
    return qMulOverflow<quint64>(v1,v2,reinterpret_cast<quint64*>(r));
}

template <> inline bool qMulOverflow(int64_t v1, int64_t v2, int64_t *r)
{
    return qMulOverflow<qint64>(v1,v2,reinterpret_cast<qint64*>(r));
}
#    endif // OS_INTEGRITY ARM64
#  endif // Q_INTRINSIC_MUL_OVERFLOW64

#  if defined(Q_CC_MSVC) && defined(Q_PROCESSOR_X86)
// We can use intrinsics for the unsigned operations with MSVC
template <> inline bool qAddOverflow(unsigned v1, unsigned v2, unsigned *r)
{ return _addcarry_u32(0, v1, v2, r); }

// 32-bit qMulOverflow is fine with the generic code above

template <> inline bool qAddOverflow(quint64 v1, quint64 v2, quint64 *r)
{
#    if defined(Q_PROCESSOR_X86_64)
    return _addcarry_u64(0, v1, v2, reinterpret_cast<unsigned __int64 *>(r));
#    else
    uint low, high;
    uchar carry = _addcarry_u32(0, unsigned(v1), unsigned(v2), &low);
    carry = _addcarry_u32(carry, v1 >> 32, v2 >> 32, &high);
    *r = (quint64(high) << 32) | low;
    return carry;
#    endif // !x86-64
}
#  endif // MSVC X86
#endif // !GCC

// Implementations for addition, subtraction or multiplication by a
// compile-time constant. For addition and subtraction, we simply call the code
// that detects overflow at runtime. For multiplication, we compare to the
// maximum possible values before multiplying to ensure no overflow happens.

template <typename T, T V2> bool qAddOverflow(T v1, std::integral_constant<T, V2>, T *r)
{
    return qAddOverflow(v1, V2, r);
}

template <auto V2, typename T> bool qAddOverflow(T v1, T *r)
{
    return qAddOverflow(v1, std::integral_constant<T, V2>{}, r);
}

template <typename T, T V2> bool qSubOverflow(T v1, std::integral_constant<T, V2>, T *r)
{
    return qSubOverflow(v1, V2, r);
}

template <auto V2, typename T> bool qSubOverflow(T v1, T *r)
{
    return qSubOverflow(v1, std::integral_constant<T, V2>{}, r);
}

template <typename T, T V2> bool qMulOverflow(T v1, std::integral_constant<T, V2>, T *r)
{
    // Runtime detection for anything smaller than or equal to a register
    // width, as most architectures' multiplication instructions actually
    // produce a result twice as wide as the input registers, allowing us to
    // efficiently detect the overflow.
    if constexpr (sizeof(T) <= sizeof(qregisteruint)) {
        return qMulOverflow(v1, V2, r);

#ifdef Q_INTRINSIC_MUL_OVERFLOW64
    } else if constexpr (sizeof(T) <= sizeof(quint64)) {
        // If we have intrinsics detecting overflow of 64-bit multiplications,
        // then detect overflows through them up to 64 bits.
        return qMulOverflow(v1, V2, r);
#endif

    } else if constexpr (V2 == 0 || V2 == 1) {
        // trivial cases (and simplify logic below due to division by zero)
        *r = v1 * V2;
        return false;
    } else if constexpr (V2 == -1) {
        // multiplication by -1 is valid *except* for signed minimum values
        // (necessary to avoid diving min() by -1, which is an overflow)
        if (v1 < 0 && v1 == (std::numeric_limits<T>::min)())
            return true;
        *r = -v1;
        return false;
    } else {
        // For 64-bit multiplications on 32-bit platforms, let's instead compare v1
        // against the bounds that would overflow.
        constexpr T Highest = (std::numeric_limits<T>::max)() / V2;
        constexpr T Lowest = (std::numeric_limits<T>::min)() / V2;
        if constexpr (Highest > Lowest) {
            if (v1 > Highest || v1 < Lowest)
                return true;
        } else {
            // this can only happen if V2 < 0
            static_assert(V2 < 0);
            if (v1 > Lowest || v1 < Highest)
                return true;
        }

        *r = v1 * V2;
        return false;
    }
}

template <auto V2, typename T> bool qMulOverflow(T v1, T *r)
{
    if constexpr (V2 == 2)
        return qAddOverflow(v1, v1, r);
    return qMulOverflow(v1, std::integral_constant<T, V2>{}, r);
}

QT_END_NAMESPACE

#endif // QNUMERIC_H
