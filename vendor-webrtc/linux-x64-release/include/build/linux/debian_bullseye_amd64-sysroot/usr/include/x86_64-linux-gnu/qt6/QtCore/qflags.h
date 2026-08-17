// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#include <QtCore/qglobal.h>
#include <QtCore/qcompare_impl.h>

#ifndef QFLAGS_H
#define QFLAGS_H

#include <initializer_list>

QT_BEGIN_NAMESPACE

class QDataStream;

class QFlag
{
    int i;
public:
    constexpr inline Q_IMPLICIT QFlag(int value) noexcept : i(value) {}
    constexpr inline Q_IMPLICIT operator int() const noexcept { return i; }

#if !defined(Q_CC_MSVC)
    // Microsoft Visual Studio has buggy behavior when it comes to
    // unsigned enums: even if the enum is unsigned, the enum tags are
    // always signed
#  if !defined(__LP64__) && !defined(Q_CLANG_QDOC)
    constexpr inline Q_IMPLICIT QFlag(long value) noexcept : i(int(value)) {}
    constexpr inline Q_IMPLICIT QFlag(ulong value) noexcept : i(int(long(value))) {}
#  endif
    constexpr inline Q_IMPLICIT QFlag(uint value) noexcept : i(int(value)) {}
    constexpr inline Q_IMPLICIT QFlag(short value) noexcept : i(int(value)) {}
    constexpr inline Q_IMPLICIT QFlag(ushort value) noexcept : i(int(uint(value))) {}
    constexpr inline Q_IMPLICIT operator uint() const noexcept { return uint(i); }
#endif
};
Q_DECLARE_TYPEINFO(QFlag, Q_PRIMITIVE_TYPE);

class QIncompatibleFlag
{
    int i;
public:
    constexpr inline explicit QIncompatibleFlag(int i) noexcept;
    constexpr inline Q_IMPLICIT operator int() const noexcept { return i; }
};
Q_DECLARE_TYPEINFO(QIncompatibleFlag, Q_PRIMITIVE_TYPE);

constexpr inline QIncompatibleFlag::QIncompatibleFlag(int value) noexcept : i(value) {}


template<typename Enum>
class QFlags
{
    static_assert((sizeof(Enum) <= sizeof(int)),
                  "QFlags uses an int as storage, so an enum with underlying "
                  "long long will overflow.");
    static_assert((std::is_enum<Enum>::value), "QFlags is only usable on enumeration types.");

public:
#if defined(Q_CC_MSVC) || defined(Q_CLANG_QDOC)
    // see above for MSVC
    // the definition below is too complex for qdoc
    typedef int Int;
#else
    typedef typename std::conditional<
            std::is_unsigned<typename std::underlying_type<Enum>::type>::value,
            unsigned int,
            signed int
        >::type Int;
#endif
    typedef Enum enum_type;
    // compiler-generated copy/move ctor/assignment operators are fine!
    constexpr inline QFlags() noexcept : i(0) {}
    constexpr inline Q_IMPLICIT QFlags(Enum flags) noexcept : i(Int(flags)) {}
    constexpr inline Q_IMPLICIT QFlags(QFlag flag) noexcept : i(flag) {}

    constexpr inline QFlags(std::initializer_list<Enum> flags) noexcept
        : i(initializer_list_helper(flags.begin(), flags.end())) {}

    constexpr static inline QFlags fromInt(Int i) noexcept { return QFlags(QFlag(i)); }
    constexpr inline Int toInt() const noexcept { return i; }

#ifndef QT_TYPESAFE_FLAGS
    constexpr inline QFlags &operator&=(int mask) noexcept { i &= mask; return *this; }
    constexpr inline QFlags &operator&=(uint mask) noexcept { i &= mask; return *this; }
#endif
    constexpr inline QFlags &operator&=(QFlags mask) noexcept { i &= mask.i; return *this; }
    constexpr inline QFlags &operator&=(Enum mask) noexcept { i &= Int(mask); return *this; }
    constexpr inline QFlags &operator|=(QFlags other) noexcept { i |= other.i; return *this; }
    constexpr inline QFlags &operator|=(Enum other) noexcept { i |= Int(other); return *this; }
    constexpr inline QFlags &operator^=(QFlags other) noexcept { i ^= other.i; return *this; }
    constexpr inline QFlags &operator^=(Enum other) noexcept { i ^= Int(other); return *this; }

#ifdef QT_TYPESAFE_FLAGS
    constexpr inline explicit operator Int() const noexcept { return i; }
    constexpr inline explicit operator bool() const noexcept { return i; }
    // For some reason, moc goes through QFlag in order to read/write
    // properties of type QFlags; so a conversion to QFlag is also
    // needed here. (It otherwise goes through a QFlags->int->QFlag
    // conversion sequence.)
    constexpr inline explicit operator QFlag() const noexcept { return QFlag(i); }
#else
    constexpr inline Q_IMPLICIT operator Int() const noexcept { return i; }
    constexpr inline bool operator!() const noexcept { return !i; }
#endif

    constexpr inline QFlags operator|(QFlags other) const noexcept { return QFlags(QFlag(i | other.i)); }
    constexpr inline QFlags operator|(Enum other) const noexcept { return QFlags(QFlag(i | Int(other))); }
    constexpr inline QFlags operator^(QFlags other) const noexcept { return QFlags(QFlag(i ^ other.i)); }
    constexpr inline QFlags operator^(Enum other) const noexcept { return QFlags(QFlag(i ^ Int(other))); }
#ifndef QT_TYPESAFE_FLAGS
    constexpr inline QFlags operator&(int mask) const noexcept { return QFlags(QFlag(i & mask)); }
    constexpr inline QFlags operator&(uint mask) const noexcept { return QFlags(QFlag(i & mask)); }
#endif
    constexpr inline QFlags operator&(QFlags other) const noexcept { return QFlags(QFlag(i & other.i)); }
    constexpr inline QFlags operator&(Enum other) const noexcept { return QFlags(QFlag(i & Int(other))); }
    constexpr inline QFlags operator~() const noexcept { return QFlags(QFlag(~i)); }

    constexpr inline void operator+(QFlags other) const noexcept = delete;
    constexpr inline void operator+(Enum other) const noexcept = delete;
    constexpr inline void operator+(int other) const noexcept = delete;
    constexpr inline void operator-(QFlags other) const noexcept = delete;
    constexpr inline void operator-(Enum other) const noexcept = delete;
    constexpr inline void operator-(int other) const noexcept = delete;

    constexpr inline bool testFlag(Enum flag) const noexcept { return testFlags(flag); }
    constexpr inline bool testFlags(QFlags flags) const noexcept { return flags.i ? ((i & flags.i) == flags.i) : i == Int(0); }
    constexpr inline bool testAnyFlag(Enum flag) const noexcept { return testAnyFlags(flag); }
    constexpr inline bool testAnyFlags(QFlags flags) const noexcept { return (i & flags.i) != Int(0); }
    constexpr inline QFlags &setFlag(Enum flag, bool on = true) noexcept
    {
        return on ? (*this |= flag) : (*this &= ~QFlags(flag));
    }

    friend constexpr inline bool operator==(QFlags lhs, QFlags rhs) noexcept
    { return lhs.i == rhs.i; }
    friend constexpr inline bool operator!=(QFlags lhs, QFlags rhs) noexcept
    { return lhs.i != rhs.i; }
    friend constexpr inline bool operator==(QFlags lhs, Enum rhs) noexcept
    { return lhs == QFlags(rhs); }
    friend constexpr inline bool operator!=(QFlags lhs, Enum rhs) noexcept
    { return lhs != QFlags(rhs); }
    friend constexpr inline bool operator==(Enum lhs, QFlags rhs) noexcept
    { return QFlags(lhs) == rhs; }
    friend constexpr inline bool operator!=(Enum lhs, QFlags rhs) noexcept
    { return QFlags(lhs) != rhs; }

#ifdef QT_TYPESAFE_FLAGS
    // Provide means of comparing flags against a literal 0; opt-in
    // because otherwise they're ambiguous against operator==(int,int)
    // after a QFlags->int conversion.
    friend constexpr inline bool operator==(QFlags flags, QtPrivate::CompareAgainstLiteralZero) noexcept
    { return flags.i == Int(0); }
    friend constexpr inline bool operator!=(QFlags flags, QtPrivate::CompareAgainstLiteralZero) noexcept
    { return flags.i != Int(0); }
    friend constexpr inline bool operator==(QtPrivate::CompareAgainstLiteralZero, QFlags flags) noexcept
    { return Int(0) == flags.i; }
    friend constexpr inline bool operator!=(QtPrivate::CompareAgainstLiteralZero, QFlags flags) noexcept
    { return Int(0) != flags.i; }
#endif

private:
    constexpr static inline Int initializer_list_helper(typename std::initializer_list<Enum>::const_iterator it,
                                                               typename std::initializer_list<Enum>::const_iterator end)
    noexcept
    {
        return (it == end ? Int(0) : (Int(*it) | initializer_list_helper(it + 1, end)));
    }

    Int i;
};

#ifndef Q_MOC_RUN
#define Q_DECLARE_FLAGS(Flags, Enum)\
typedef QFlags<Enum> Flags;
#endif

#ifdef QT_TYPESAFE_FLAGS

// These are opt-in, for backwards compatibility
#define QT_DECLARE_TYPESAFE_OPERATORS_FOR_FLAGS_ENUM(Flags) \
[[maybe_unused]] \
constexpr inline Flags operator~(Flags::enum_type e) noexcept \
{ return ~Flags(e); } \
[[maybe_unused]] \
constexpr inline void operator|(Flags::enum_type f1, int f2) noexcept = delete;
#else
#define QT_DECLARE_TYPESAFE_OPERATORS_FOR_FLAGS_ENUM(Flags) \
[[maybe_unused]] \
constexpr inline QIncompatibleFlag operator|(Flags::enum_type f1, int f2) noexcept \
{ return QIncompatibleFlag(int(f1) | f2); }
#endif

#define Q_DECLARE_OPERATORS_FOR_FLAGS(Flags) \
[[maybe_unused]] \
constexpr inline QFlags<Flags::enum_type> operator|(Flags::enum_type f1, Flags::enum_type f2) noexcept \
{ return QFlags<Flags::enum_type>(f1) | f2; } \
[[maybe_unused]] \
constexpr inline QFlags<Flags::enum_type> operator|(Flags::enum_type f1, QFlags<Flags::enum_type> f2) noexcept \
{ return f2 | f1; } \
[[maybe_unused]] \
constexpr inline QFlags<Flags::enum_type> operator&(Flags::enum_type f1, Flags::enum_type f2) noexcept \
{ return QFlags<Flags::enum_type>(f1) & f2; } \
[[maybe_unused]] \
constexpr inline QFlags<Flags::enum_type> operator&(Flags::enum_type f1, QFlags<Flags::enum_type> f2) noexcept \
{ return f2 & f1; } \
[[maybe_unused]] \
constexpr inline QFlags<Flags::enum_type> operator^(Flags::enum_type f1, Flags::enum_type f2) noexcept \
{ return QFlags<Flags::enum_type>(f1) ^ f2; } \
[[maybe_unused]] \
constexpr inline QFlags<Flags::enum_type> operator^(Flags::enum_type f1, QFlags<Flags::enum_type> f2) noexcept \
{ return f2 ^ f1; } \
constexpr inline void operator+(Flags::enum_type f1, Flags::enum_type f2) noexcept = delete; \
constexpr inline void operator+(Flags::enum_type f1, QFlags<Flags::enum_type> f2) noexcept = delete; \
constexpr inline void operator+(int f1, QFlags<Flags::enum_type> f2) noexcept = delete; \
constexpr inline void operator-(Flags::enum_type f1, Flags::enum_type f2) noexcept = delete; \
constexpr inline void operator-(Flags::enum_type f1, QFlags<Flags::enum_type> f2) noexcept = delete; \
constexpr inline void operator-(int f1, QFlags<Flags::enum_type> f2) noexcept = delete; \
constexpr inline void operator+(int f1, Flags::enum_type f2) noexcept = delete; \
constexpr inline void operator+(Flags::enum_type f1, int f2) noexcept = delete; \
constexpr inline void operator-(int f1, Flags::enum_type f2) noexcept = delete; \
constexpr inline void operator-(Flags::enum_type f1, int f2) noexcept = delete; \
QT_DECLARE_TYPESAFE_OPERATORS_FOR_FLAGS_ENUM(Flags)

// restore bit-wise enum-enum operators deprecated in C++20,
// but used in a few places in the API
#if __cplusplus > 201702L // assume compilers don't warn if in C++17 mode
  // in C++20 mode, provide user-defined operators to override the deprecated operations:
# define Q_DECLARE_MIXED_ENUM_OPERATOR(op, Ret, LHS, RHS) \
    [[maybe_unused]] \
    constexpr inline Ret operator op (LHS lhs, RHS rhs) noexcept \
    { return static_cast<Ret>(qToUnderlying(lhs) op qToUnderlying(rhs)); } \
    /* end */
#else
  // in C++17 mode, statically-assert that this compiler's result of the
  // operation is the same that the C++20 version would produce:
# define Q_DECLARE_MIXED_ENUM_OPERATOR(op, Ret, LHS, RHS) \
    static_assert(std::is_same_v<decltype(std::declval<LHS>() op std::declval<RHS>()), Ret>);
#endif

#define Q_DECLARE_MIXED_ENUM_OPERATORS(Ret, Flags, Enum) \
    Q_DECLARE_MIXED_ENUM_OPERATOR(|, Ret, Flags, Enum) \
    Q_DECLARE_MIXED_ENUM_OPERATOR(&, Ret, Flags, Enum) \
    Q_DECLARE_MIXED_ENUM_OPERATOR(^, Ret, Flags, Enum) \
    /* end */

#define Q_DECLARE_MIXED_ENUM_OPERATORS_SYMMETRIC(Ret, Flags, Enum) \
    Q_DECLARE_MIXED_ENUM_OPERATORS(Ret, Flags, Enum) \
    Q_DECLARE_MIXED_ENUM_OPERATORS(Ret, Enum, Flags) \
    /* end */


QT_END_NAMESPACE

#endif // QFLAGS_H
