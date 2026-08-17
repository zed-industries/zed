// Copyright (C) 2016 The Qt Company Ltd.
// Copyright (C) 2015 Klarälvdalens Datakonsult AB, a KDAB Group company, info@kdab.com, author Marc Mutz <marc.mutz@kdab.com>
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QHASHFUNCTIONS_H
#define QHASHFUNCTIONS_H

#include <QtCore/qstring.h>
#include <QtCore/qstringfwd.h>
#include <QtCore/qpair.h>

#include <numeric> // for std::accumulate
#include <functional> // for std::hash

#if 0
#pragma qt_class(QHashFunctions)
#endif

#if defined(Q_CC_MSVC)
#pragma warning( push )
#pragma warning( disable : 4311 ) // disable pointer truncation warning
#pragma warning( disable : 4127 ) // conditional expression is constant
#endif

QT_BEGIN_NAMESPACE

class QBitArray;

#if QT_DEPRECATED_SINCE(6,6)
QT_DEPRECATED_VERSION_X_6_6("Use QHashSeed instead")
Q_CORE_EXPORT int qGlobalQHashSeed();
QT_DEPRECATED_VERSION_X_6_6("Use QHashSeed instead")
Q_CORE_EXPORT void qSetGlobalQHashSeed(int newSeed);
#endif

struct QHashSeed
{
    constexpr QHashSeed(size_t d = 0) : data(d) {}
    constexpr operator size_t() const noexcept { return data; }

    static Q_CORE_EXPORT QHashSeed globalSeed() noexcept;
    static Q_CORE_EXPORT void setDeterministicGlobalSeed();
    static Q_CORE_EXPORT void resetRandomGlobalSeed();
private:
    size_t data;
};

namespace QHashPrivate {

Q_DECL_CONST_FUNCTION constexpr size_t hash(size_t key, size_t seed) noexcept
{
    key ^= seed;
    if constexpr (sizeof(size_t) == 4) {
        key ^= key >> 16;
        key *= UINT32_C(0x45d9f3b);
        key ^= key >> 16;
        key *= UINT32_C(0x45d9f3b);
        key ^= key >> 16;
        return key;
    } else {
        quint64 key64 = key;
        key64 ^= key64 >> 32;
        key64 *= UINT64_C(0xd6e8feb86659fd93);
        key64 ^= key64 >> 32;
        key64 *= UINT64_C(0xd6e8feb86659fd93);
        key64 ^= key64 >> 32;
        return size_t(key64);
    }
}

template <typename T, typename = void>
constexpr inline bool HasQHashSingleArgOverload = false;

template <typename T>
constexpr inline bool HasQHashSingleArgOverload<T, std::enable_if_t<
    std::is_convertible_v<decltype(qHash(std::declval<const T &>())), size_t>
>> = true;

template <typename T1, typename T2> static constexpr bool noexceptPairHash();
}

Q_CORE_EXPORT Q_DECL_PURE_FUNCTION size_t qHashBits(const void *p, size_t size, size_t seed = 0) noexcept;

// implementation below qHashMulti
template <typename T1, typename T2> inline size_t qHash(const std::pair<T1, T2> &key, size_t seed = 0)
    noexcept(QHashPrivate::noexceptPairHash<T1, T2>());

// C++ builtin types
Q_DECL_CONST_FUNCTION constexpr inline size_t qHash(char key, size_t seed = 0) noexcept
{ return QHashPrivate::hash(size_t(key), seed); }
Q_DECL_CONST_FUNCTION constexpr inline size_t qHash(uchar key, size_t seed = 0) noexcept
{ return QHashPrivate::hash(size_t(key), seed); }
Q_DECL_CONST_FUNCTION constexpr inline size_t qHash(signed char key, size_t seed = 0) noexcept
{ return QHashPrivate::hash(size_t(key), seed); }
Q_DECL_CONST_FUNCTION constexpr inline size_t qHash(ushort key, size_t seed = 0) noexcept
{ return QHashPrivate::hash(size_t(key), seed); }
Q_DECL_CONST_FUNCTION constexpr inline size_t qHash(short key, size_t seed = 0) noexcept
{ return QHashPrivate::hash(size_t(key), seed); }
Q_DECL_CONST_FUNCTION constexpr inline size_t qHash(uint key, size_t seed = 0) noexcept
{ return QHashPrivate::hash(size_t(key), seed); }
Q_DECL_CONST_FUNCTION constexpr inline size_t qHash(int key, size_t seed = 0) noexcept
{ return QHashPrivate::hash(size_t(key), seed); }
Q_DECL_CONST_FUNCTION constexpr inline size_t qHash(ulong key, size_t seed = 0) noexcept
{ return QHashPrivate::hash(size_t(key), seed); }
Q_DECL_CONST_FUNCTION constexpr inline size_t qHash(long key, size_t seed = 0) noexcept
{ return QHashPrivate::hash(size_t(key), seed); }
Q_DECL_CONST_FUNCTION constexpr inline size_t qHash(quint64 key, size_t seed = 0) noexcept
{
    if constexpr (sizeof(quint64) > sizeof(size_t))
        key ^= (key >> 32);
    return QHashPrivate::hash(size_t(key), seed);
}
Q_DECL_CONST_FUNCTION constexpr inline size_t qHash(qint64 key, size_t seed = 0) noexcept { return qHash(quint64(key), seed); }
Q_DECL_CONST_FUNCTION inline size_t qHash(float key, size_t seed = 0) noexcept
{
    // ensure -0 gets mapped to 0
    key += 0.0f;
    uint k;
    memcpy(&k, &key, sizeof(float));
    return QHashPrivate::hash(k, seed);
}
Q_CORE_EXPORT Q_DECL_CONST_FUNCTION size_t qHash(double key, size_t seed = 0) noexcept;
#if !defined(Q_OS_DARWIN) || defined(Q_CLANG_QDOC)
Q_CORE_EXPORT Q_DECL_CONST_FUNCTION size_t qHash(long double key, size_t seed = 0) noexcept;
#endif
Q_DECL_CONST_FUNCTION constexpr inline size_t qHash(wchar_t key, size_t seed = 0) noexcept
{ return QHashPrivate::hash(size_t(key), seed); }
Q_DECL_CONST_FUNCTION constexpr inline size_t qHash(char16_t key, size_t seed = 0) noexcept
{ return QHashPrivate::hash(size_t(key), seed); }
Q_DECL_CONST_FUNCTION constexpr inline size_t qHash(char32_t key, size_t seed = 0) noexcept
{ return QHashPrivate::hash(size_t(key), seed); }
#ifdef __cpp_char8_t
Q_DECL_CONST_FUNCTION constexpr inline size_t qHash(char8_t key, size_t seed = 0) noexcept
{ return QHashPrivate::hash(size_t(key), seed); }
#endif
template <class T> inline size_t qHash(const T *key, size_t seed = 0) noexcept
{
    return qHash(reinterpret_cast<quintptr>(key), seed);
}
Q_DECL_CONST_FUNCTION constexpr inline size_t qHash(std::nullptr_t, size_t seed = 0) noexcept
{
    return seed;
}

// (some) Qt types
Q_DECL_CONST_FUNCTION constexpr inline size_t qHash(const QChar key, size_t seed = 0) noexcept { return qHash(key.unicode(), seed); }

#if QT_CORE_REMOVED_SINCE(6, 4)
Q_CORE_EXPORT Q_DECL_PURE_FUNCTION size_t qHash(const QByteArray &key, size_t seed = 0) noexcept;
Q_CORE_EXPORT Q_DECL_PURE_FUNCTION size_t qHash(const QByteArrayView &key, size_t seed = 0) noexcept;
#else
Q_CORE_EXPORT Q_DECL_PURE_FUNCTION size_t qHash(QByteArrayView key, size_t seed = 0) noexcept;
inline Q_DECL_PURE_FUNCTION size_t qHash(const QByteArray &key, size_t seed = 0
        QT6_DECL_NEW_OVERLOAD_TAIL) noexcept
{ return qHash(qToByteArrayViewIgnoringNull(key), seed); }
#endif

Q_CORE_EXPORT Q_DECL_PURE_FUNCTION size_t qHash(QStringView key, size_t seed = 0) noexcept;
inline Q_DECL_PURE_FUNCTION size_t qHash(const QString &key, size_t seed = 0) noexcept
{ return qHash(QStringView{key}, seed); }
Q_CORE_EXPORT Q_DECL_PURE_FUNCTION size_t qHash(const QBitArray &key, size_t seed = 0) noexcept;
Q_CORE_EXPORT Q_DECL_PURE_FUNCTION size_t qHash(QLatin1StringView key, size_t seed = 0) noexcept;
Q_DECL_CONST_FUNCTION constexpr inline size_t qHash(QKeyCombination key, size_t seed = 0) noexcept
{ return qHash(key.toCombined(), seed); }
Q_CORE_EXPORT Q_DECL_PURE_FUNCTION uint qt_hash(QStringView key, uint chained = 0) noexcept;

template <typename Enum>
Q_DECL_CONST_FUNCTION constexpr inline size_t qHash(QFlags<Enum> flags, size_t seed = 0) noexcept
{ return qHash(flags.toInt(), seed); }

template <typename T, std::enable_if_t<QHashPrivate::HasQHashSingleArgOverload<T>, bool> = true>
size_t qHash(const T &t, size_t seed) noexcept(noexcept(qHash(t)))
{ return qHash(t) ^ seed; }

template<typename T>
bool qHashEquals(const T &a, const T &b)
{
    return a == b;
}

namespace QtPrivate {

struct QHashCombine
{
    typedef size_t result_type;
    template <typename T>
    constexpr result_type operator()(size_t seed, const T &t) const noexcept(noexcept(qHash(t)))
    // combiner taken from N3876 / boost::hash_combine
    { return seed ^ (qHash(t) + 0x9e3779b9 + (seed << 6) + (seed >> 2)) ; }
};

struct QHashCombineCommutative
{
    // QHashCombine is a good hash combiner, but is not commutative,
    // ie. it depends on the order of the input elements. That is
    // usually what we want: {0,1,3} should hash differently than
    // {1,3,0}. Except when it isn't (e.g. for QSet and
    // QHash). Therefore, provide a commutative combiner, too.
    typedef size_t result_type;
    template <typename T>
    constexpr result_type operator()(size_t seed, const T &t) const noexcept(noexcept(qHash(t)))
    { return seed + qHash(t); } // don't use xor!
};

template <typename... T>
using QHashMultiReturnType = decltype(
    std::declval< std::enable_if_t<(sizeof...(T) > 0)> >(),
    (qHash(std::declval<const T &>()), ...),
    size_t{}
);

// workaround for a MSVC ICE,
// https://developercommunity.visualstudio.com/content/problem/996540/internal-compiler-error-on-msvc-1924-when-doing-sf.html
template <typename T>
inline constexpr bool QNothrowHashableHelper_v = noexcept(qHash(std::declval<const T &>()));

template <typename T, typename Enable = void>
struct QNothrowHashable : std::false_type {};

template <typename T>
struct QNothrowHashable<T, std::enable_if_t<QNothrowHashableHelper_v<T>>> : std::true_type {};

template <typename T>
constexpr inline bool QNothrowHashable_v = QNothrowHashable<T>::value;

} // namespace QtPrivate

template <typename... T>
constexpr
#ifdef Q_QDOC
size_t
#else
QtPrivate::QHashMultiReturnType<T...>
#endif
qHashMulti(size_t seed, const T &... args)
    noexcept(std::conjunction_v<QtPrivate::QNothrowHashable<T>...>)
{
    QtPrivate::QHashCombine hash;
    return ((seed = hash(seed, args)), ...), seed;
}

template <typename... T>
constexpr
#ifdef Q_QDOC
size_t
#else
QtPrivate::QHashMultiReturnType<T...>
#endif
qHashMultiCommutative(size_t seed, const T &... args)
    noexcept(std::conjunction_v<QtPrivate::QNothrowHashable<T>...>)
{
    QtPrivate::QHashCombineCommutative hash;
    return ((seed = hash(seed, args)), ...), seed;
}

template <typename InputIterator>
inline size_t qHashRange(InputIterator first, InputIterator last, size_t seed = 0)
    noexcept(noexcept(qHash(*first))) // assume iterator operations don't throw
{
    return std::accumulate(first, last, seed, QtPrivate::QHashCombine());
}

template <typename InputIterator>
inline size_t qHashRangeCommutative(InputIterator first, InputIterator last, size_t seed = 0)
    noexcept(noexcept(qHash(*first))) // assume iterator operations don't throw
{
    return std::accumulate(first, last, seed, QtPrivate::QHashCombineCommutative());
}

namespace QHashPrivate {
template <typename T1, typename T2> static constexpr bool noexceptPairHash()
{
    size_t seed = 0;
    return noexcept(qHash(std::declval<T1>(), seed)) && noexcept(qHash(std::declval<T2>(), seed));
}
} // QHashPrivate

template <typename T1, typename T2> inline size_t qHash(const std::pair<T1, T2> &key, size_t seed)
    noexcept(QHashPrivate::noexceptPairHash<T1, T2>())
{
    return qHashMulti(seed, key.first, key.second);
}

#define QT_SPECIALIZE_STD_HASH_TO_CALL_QHASH(Class, Arguments)      \
    QT_BEGIN_INCLUDE_NAMESPACE                                      \
    namespace std {                                                 \
        template <>                                                 \
        struct hash< QT_PREPEND_NAMESPACE(Class) > {                \
            using argument_type = QT_PREPEND_NAMESPACE(Class);      \
            using result_type = size_t;                             \
            size_t operator()(Arguments s) const                    \
              noexcept(QT_PREPEND_NAMESPACE(                        \
                     QtPrivate::QNothrowHashable_v)<argument_type>) \
            {                                                       \
                /* this seeds qHash with the result of */           \
                /* std::hash applied to an int, to reap */          \
                /* any protection against predictable hash */       \
                /* values the std implementation may provide */     \
                using QT_PREPEND_NAMESPACE(qHash);                  \
                return qHash(s, qHash(std::hash<int>{}(0)));        \
            }                                                       \
        };                                                          \
    }                                                               \
    QT_END_INCLUDE_NAMESPACE                                        \
    /*end*/

#define QT_SPECIALIZE_STD_HASH_TO_CALL_QHASH_BY_CREF(Class) \
    QT_SPECIALIZE_STD_HASH_TO_CALL_QHASH(Class, const argument_type &)
#define QT_SPECIALIZE_STD_HASH_TO_CALL_QHASH_BY_VALUE(Class) \
    QT_SPECIALIZE_STD_HASH_TO_CALL_QHASH(Class, argument_type)

QT_SPECIALIZE_STD_HASH_TO_CALL_QHASH_BY_CREF(QString)
QT_SPECIALIZE_STD_HASH_TO_CALL_QHASH_BY_VALUE(QStringView)
QT_SPECIALIZE_STD_HASH_TO_CALL_QHASH_BY_VALUE(QLatin1StringView)
QT_SPECIALIZE_STD_HASH_TO_CALL_QHASH_BY_VALUE(QByteArrayView)
QT_SPECIALIZE_STD_HASH_TO_CALL_QHASH_BY_CREF(QByteArray)
QT_SPECIALIZE_STD_HASH_TO_CALL_QHASH_BY_CREF(QBitArray)

QT_END_NAMESPACE

#if defined(Q_CC_MSVC)
#pragma warning( pop )
#endif

#endif // QHASHFUNCTIONS_H
