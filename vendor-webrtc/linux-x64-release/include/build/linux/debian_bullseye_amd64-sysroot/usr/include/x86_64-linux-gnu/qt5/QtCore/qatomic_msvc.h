/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Copyright (C) 2016 Intel Corporation.
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

#ifndef QATOMIC_MSVC_H
#define QATOMIC_MSVC_H

#include <QtCore/qgenericatomic.h>

////////////////////////////////////////////////////////////////////////////////////////////////////

// use compiler intrinsics for all atomic functions
# define QT_INTERLOCKED_PREFIX _
# define QT_INTERLOCKED_PROTOTYPE
# define QT_INTERLOCKED_DECLARE_PROTOTYPES
# define QT_INTERLOCKED_INTRINSIC
# define Q_ATOMIC_INT16_IS_SUPPORTED

# ifdef _WIN64
#  define Q_ATOMIC_INT64_IS_SUPPORTED
# endif

////////////////////////////////////////////////////////////////////////////////////////////////////
// Prototype declaration

#define QT_INTERLOCKED_CONCAT_I(prefix, suffix) \
    prefix ## suffix
#define QT_INTERLOCKED_CONCAT(prefix, suffix) \
    QT_INTERLOCKED_CONCAT_I(prefix, suffix)

// MSVC intrinsics prefix function names with an underscore. Also, if platform
// SDK headers have been included, the Interlocked names may be defined as
// macros.
// To avoid double underscores, we paste the prefix with Interlocked first and
// then the remainder of the function name.
#define QT_INTERLOCKED_FUNCTION(name) \
    QT_INTERLOCKED_CONCAT( \
        QT_INTERLOCKED_CONCAT(QT_INTERLOCKED_PREFIX, Interlocked), name)

#ifndef QT_INTERLOCKED_VOLATILE
# define QT_INTERLOCKED_VOLATILE volatile
#endif

#ifndef QT_INTERLOCKED_PREFIX
#define QT_INTERLOCKED_PREFIX
#endif

#ifndef QT_INTERLOCKED_PROTOTYPE
#define QT_INTERLOCKED_PROTOTYPE
#endif

#ifdef QT_INTERLOCKED_DECLARE_PROTOTYPES
#undef QT_INTERLOCKED_DECLARE_PROTOTYPES

extern "C" {

    long QT_INTERLOCKED_PROTOTYPE QT_INTERLOCKED_FUNCTION( Increment )(long QT_INTERLOCKED_VOLATILE *);
    long QT_INTERLOCKED_PROTOTYPE QT_INTERLOCKED_FUNCTION( Decrement )(long QT_INTERLOCKED_VOLATILE *);
    long QT_INTERLOCKED_PROTOTYPE QT_INTERLOCKED_FUNCTION( CompareExchange )(long QT_INTERLOCKED_VOLATILE *, long, long);
    long QT_INTERLOCKED_PROTOTYPE QT_INTERLOCKED_FUNCTION( Exchange )(long QT_INTERLOCKED_VOLATILE *, long);
    long QT_INTERLOCKED_PROTOTYPE QT_INTERLOCKED_FUNCTION( ExchangeAdd )(long QT_INTERLOCKED_VOLATILE *, long);

# if !defined(__i386__) && !defined(_M_IX86)
    void * QT_INTERLOCKED_FUNCTION( CompareExchangePointer )(void * QT_INTERLOCKED_VOLATILE *, void *, void *);
    void * QT_INTERLOCKED_FUNCTION( ExchangePointer )(void * QT_INTERLOCKED_VOLATILE *, void *);
    __int64 QT_INTERLOCKED_FUNCTION( ExchangeAdd64 )(__int64 QT_INTERLOCKED_VOLATILE *, __int64);
# endif

# ifdef Q_ATOMIC_INT16_IS_SUPPORTED
    short QT_INTERLOCKED_PROTOTYPE QT_INTERLOCKED_FUNCTION( Increment16 )(short QT_INTERLOCKED_VOLATILE *);
    short QT_INTERLOCKED_PROTOTYPE QT_INTERLOCKED_FUNCTION( Decrement16 )(short QT_INTERLOCKED_VOLATILE *);
    short QT_INTERLOCKED_PROTOTYPE QT_INTERLOCKED_FUNCTION( CompareExchange16 )(short QT_INTERLOCKED_VOLATILE *, short, short);
    short QT_INTERLOCKED_PROTOTYPE QT_INTERLOCKED_FUNCTION( Exchange16 )(short QT_INTERLOCKED_VOLATILE *, short);
    short QT_INTERLOCKED_PROTOTYPE QT_INTERLOCKED_FUNCTION( ExchangeAdd16 )(short QT_INTERLOCKED_VOLATILE *, short);
# endif
# ifdef Q_ATOMIC_INT64_IS_SUPPORTED
    __int64 QT_INTERLOCKED_PROTOTYPE QT_INTERLOCKED_FUNCTION( Increment64 )(__int64 QT_INTERLOCKED_VOLATILE *);
    __int64 QT_INTERLOCKED_PROTOTYPE QT_INTERLOCKED_FUNCTION( Decrement64 )(__int64 QT_INTERLOCKED_VOLATILE *);
    __int64 QT_INTERLOCKED_PROTOTYPE QT_INTERLOCKED_FUNCTION( CompareExchange64 )(__int64 QT_INTERLOCKED_VOLATILE *, __int64, __int64);
    __int64 QT_INTERLOCKED_PROTOTYPE QT_INTERLOCKED_FUNCTION( Exchange64 )(__int64 QT_INTERLOCKED_VOLATILE *, __int64);
    //above already: qint64 QT_INTERLOCKED_PROTOTYPE QT_INTERLOCKED_FUNCTION( ExchangeAdd64 )(qint64 QT_INTERLOCKED_VOLATILE *, qint64);
# endif
}

#endif // QT_INTERLOCKED_DECLARE_PROTOTYPES

#undef QT_INTERLOCKED_PROTOTYPE

////////////////////////////////////////////////////////////////////////////////////////////////////

#ifdef QT_INTERLOCKED_INTRINSIC
#undef QT_INTERLOCKED_INTRINSIC

# pragma intrinsic (_InterlockedIncrement)
# pragma intrinsic (_InterlockedDecrement)
# pragma intrinsic (_InterlockedExchange)
# pragma intrinsic (_InterlockedCompareExchange)
# pragma intrinsic (_InterlockedExchangeAdd)

# if !defined(_M_IX86)
#  pragma intrinsic (_InterlockedCompareExchangePointer)
#  pragma intrinsic (_InterlockedExchangePointer)
#  pragma intrinsic (_InterlockedExchangeAdd64)
# endif

#endif // QT_INTERLOCKED_INTRINSIC

////////////////////////////////////////////////////////////////////////////////////////////////////
// Interlocked* replacement macros

#if defined(__i386__) || defined(_M_IX86)

# define QT_INTERLOCKED_COMPARE_EXCHANGE_POINTER(value, newValue, expectedValue) \
    reinterpret_cast<void *>( \
        QT_INTERLOCKED_FUNCTION(CompareExchange)( \
                reinterpret_cast<long QT_INTERLOCKED_VOLATILE *>(value), \
                long(newValue), \
                long(expectedValue)))

# define QT_INTERLOCKED_EXCHANGE_POINTER(value, newValue) \
    QT_INTERLOCKED_FUNCTION(Exchange)( \
            reinterpret_cast<long QT_INTERLOCKED_VOLATILE *>(value), \
            long(newValue))

# define QT_INTERLOCKED_EXCHANGE_ADD_POINTER(value, valueToAdd) \
    QT_INTERLOCKED_FUNCTION(ExchangeAdd)( \
            reinterpret_cast<long QT_INTERLOCKED_VOLATILE *>(value), \
            (valueToAdd))

#else // !defined(__i386__) && !defined(_M_IX86)

# define QT_INTERLOCKED_COMPARE_EXCHANGE_POINTER(value, newValue, expectedValue) \
    QT_INTERLOCKED_FUNCTION(CompareExchangePointer)( \
            (void * QT_INTERLOCKED_VOLATILE *)(value), \
            (void *) (newValue), \
            (void *) (expectedValue))

# define QT_INTERLOCKED_EXCHANGE_POINTER(value, newValue) \
    QT_INTERLOCKED_FUNCTION(ExchangePointer)( \
            (void * QT_INTERLOCKED_VOLATILE *)(value), \
            (void *) (newValue))

# define QT_INTERLOCKED_EXCHANGE_ADD_POINTER(value, valueToAdd) \
    QT_INTERLOCKED_FUNCTION(ExchangeAdd64)( \
            reinterpret_cast<qint64 QT_INTERLOCKED_VOLATILE *>(value), \
            (valueToAdd))

#endif // !defined(__i386__) && !defined(_M_IX86)

////////////////////////////////////////////////////////////////////////////////////////////////////

QT_BEGIN_NAMESPACE

#if 0
// silence syncqt warnings
QT_END_NAMESPACE
#pragma qt_sync_skip_header_check
#pragma qt_sync_stop_processing
#endif

#define Q_ATOMIC_INT_REFERENCE_COUNTING_IS_ALWAYS_NATIVE
#define Q_ATOMIC_INT_REFERENCE_COUNTING_IS_WAIT_FREE

#define Q_ATOMIC_INT_TEST_AND_SET_IS_ALWAYS_NATIVE
#define Q_ATOMIC_INT_TEST_AND_SET_IS_WAIT_FREE

#define Q_ATOMIC_INT_FETCH_AND_STORE_IS_ALWAYS_NATIVE
#define Q_ATOMIC_INT_FETCH_AND_STORE_IS_WAIT_FREE

#define Q_ATOMIC_INT_FETCH_AND_ADD_IS_ALWAYS_NATIVE
#define Q_ATOMIC_INT_FETCH_AND_ADD_IS_WAIT_FREE

#define Q_ATOMIC_INT32_IS_SUPPORTED

#define Q_ATOMIC_INT32_REFERENCE_COUNTING_IS_ALWAYS_NATIVE
#define Q_ATOMIC_INT32_REFERENCE_COUNTING_IS_WAIT_FREE

#define Q_ATOMIC_INT32_TEST_AND_SET_IS_ALWAYS_NATIVE
#define Q_ATOMIC_INT32_TEST_AND_SET_IS_WAIT_FREE

#define Q_ATOMIC_INT32_FETCH_AND_STORE_IS_ALWAYS_NATIVE
#define Q_ATOMIC_INT32_FETCH_AND_STORE_IS_WAIT_FREE

#define Q_ATOMIC_INT32_FETCH_AND_ADD_IS_ALWAYS_NATIVE
#define Q_ATOMIC_INT32_FETCH_AND_ADD_IS_WAIT_FREE

#define Q_ATOMIC_POINTER_TEST_AND_SET_IS_ALWAYS_NATIVE
#define Q_ATOMIC_POINTER_TEST_AND_SET_IS_WAIT_FREE

#define Q_ATOMIC_POINTER_FETCH_AND_STORE_IS_ALWAYS_NATIVE
#define Q_ATOMIC_POINTER_FETCH_AND_STORE_IS_WAIT_FREE

#define Q_ATOMIC_POINTER_FETCH_AND_ADD_IS_ALWAYS_NATIVE
#define Q_ATOMIC_POINTER_FETCH_AND_ADD_IS_WAIT_FREE

#ifdef Q_ATOMIC_INT16_IS_SUPPORTED
# define Q_ATOMIC_INT16_REFERENCE_COUNTING_IS_ALWAYS_NATIVE
# define Q_ATOMIC_INT16_REFERENCE_COUNTING_IS_WAIT_FREE

# define Q_ATOMIC_INT16_TEST_AND_SET_IS_ALWAYS_NATIVE
# define Q_ATOMIC_INT16_TEST_AND_SET_IS_WAIT_FREE

# define Q_ATOMIC_INT16_FETCH_AND_STORE_IS_ALWAYS_NATIVE
# define Q_ATOMIC_INT16_FETCH_AND_STORE_IS_WAIT_FREE

# define Q_ATOMIC_INT16_FETCH_AND_ADD_IS_ALWAYS_NATIVE
# define Q_ATOMIC_INT16_FETCH_AND_ADD_IS_WAIT_FREE

template<> struct QAtomicOpsSupport<2> { enum { IsSupported = 1 }; };
#endif

#ifdef Q_ATOMIC_INT64_IS_SUPPORTED
# define Q_ATOMIC_INT64_REFERENCE_COUNTING_IS_ALWAYS_NATIVE
# define Q_ATOMIC_INT64_REFERENCE_COUNTING_IS_WAIT_FREE

# define Q_ATOMIC_INT64_TEST_AND_SET_IS_ALWAYS_NATIVE
# define Q_ATOMIC_INT64_TEST_AND_SET_IS_WAIT_FREE

# define Q_ATOMIC_INT64_FETCH_AND_STORE_IS_ALWAYS_NATIVE
# define Q_ATOMIC_INT64_FETCH_AND_STORE_IS_WAIT_FREE

# define Q_ATOMIC_INT64_FETCH_AND_ADD_IS_ALWAYS_NATIVE
# define Q_ATOMIC_INT64_FETCH_AND_ADD_IS_WAIT_FREE

template<> struct QAtomicOpsSupport<8> { enum { IsSupported = 1 }; };
#endif

////////////////////////////////////////////////////////////////////////////////////////////////////

template <int N> struct QAtomicWindowsType { typedef typename QIntegerForSize<N>::Signed Type; };
template <> struct QAtomicWindowsType<4> { typedef long Type; };


template <int N> struct QAtomicOpsBySize : QGenericAtomicOps<QAtomicOpsBySize<N> >
{
    static inline Q_DECL_CONSTEXPR bool isReferenceCountingNative() noexcept { return true; }
    static inline Q_DECL_CONSTEXPR bool isReferenceCountingWaitFree() noexcept { return true; }
    template <typename T> static bool ref(T &_q_value) noexcept;
    template <typename T> static bool deref(T &_q_value) noexcept;

    static inline Q_DECL_CONSTEXPR bool isTestAndSetNative() noexcept { return true; }
    static inline Q_DECL_CONSTEXPR bool isTestAndSetWaitFree() noexcept { return true; }
    template <typename T> static bool testAndSetRelaxed(T &_q_value, T expectedValue, T newValue) noexcept;
    template <typename T>
    static bool testAndSetRelaxed(T &_q_value, T expectedValue, T newValue, T *currentValue) noexcept;

    static inline Q_DECL_CONSTEXPR bool isFetchAndStoreNative() noexcept { return true; }
    static inline Q_DECL_CONSTEXPR bool isFetchAndStoreWaitFree() noexcept { return true; }
    template <typename T> static T fetchAndStoreRelaxed(T &_q_value, T newValue) noexcept;

    static inline Q_DECL_CONSTEXPR bool isFetchAndAddNative() noexcept { return true; }
    static inline Q_DECL_CONSTEXPR bool isFetchAndAddWaitFree() noexcept { return true; }
    template <typename T> static T fetchAndAddRelaxed(T &_q_value, typename QAtomicAdditiveType<T>::AdditiveT valueToAdd) noexcept;

private:
    typedef typename QAtomicWindowsType<N>::Type Type;
    template <typename T> static inline Type *atomic(T *t)
    { Q_STATIC_ASSERT(sizeof(T) == sizeof(Type)); return reinterpret_cast<Type *>(t); }
    template <typename T> static inline Type value(T t)
    { Q_STATIC_ASSERT(sizeof(T) == sizeof(Type)); return Type(t); }
};

template <typename T>
struct QAtomicOps : QAtomicOpsBySize<sizeof(T)>
{
    typedef T Type;
};

template<> template<typename T>
inline bool QAtomicOpsBySize<4>::ref(T &_q_value) noexcept
{
    return QT_INTERLOCKED_FUNCTION(Increment)(atomic(&_q_value)) != 0;
}

template<> template<typename T>
inline bool QAtomicOpsBySize<4>::deref(T &_q_value) noexcept
{
    return QT_INTERLOCKED_FUNCTION(Decrement)(atomic(&_q_value)) != 0;
}

template<> template<typename T>
inline bool QAtomicOpsBySize<4>::testAndSetRelaxed(T &_q_value, T expectedValue, T newValue) noexcept
{
    return QT_INTERLOCKED_FUNCTION(CompareExchange)(atomic(&_q_value), value(newValue), value(expectedValue)) == value(expectedValue);
}

template<> template <typename T>
inline bool QAtomicOpsBySize<4>::testAndSetRelaxed(T &_q_value, T expectedValue, T newValue, T *currentValue) noexcept
{
    *currentValue = T(QT_INTERLOCKED_FUNCTION(CompareExchange)(atomic(&_q_value), newValue, expectedValue));
    return *currentValue == expectedValue;
}

template<> template<typename T>
inline T QAtomicOpsBySize<4>::fetchAndStoreRelaxed(T &_q_value, T newValue) noexcept
{
    return QT_INTERLOCKED_FUNCTION(Exchange)(atomic(&_q_value), value(newValue));
}

template<> template<typename T>
inline T QAtomicOpsBySize<4>::fetchAndAddRelaxed(T &_q_value, typename QAtomicAdditiveType<T>::AdditiveT valueToAdd) noexcept
{
    return QT_INTERLOCKED_FUNCTION(ExchangeAdd)(atomic(&_q_value), value<T>(valueToAdd * QAtomicAdditiveType<T>::AddScale));
}

#ifdef Q_ATOMIC_INT16_IS_SUPPORTED
template<> template<typename T>
inline bool QAtomicOpsBySize<2>::ref(T &_q_value) noexcept
{
    return QT_INTERLOCKED_FUNCTION(Increment16)(atomic(&_q_value)) != 0;
}

template<> template<typename T>
inline bool QAtomicOpsBySize<2>::deref(T &_q_value) noexcept
{
    return QT_INTERLOCKED_FUNCTION(Decrement16)(atomic(&_q_value)) != 0;
}

template<> template<typename T>
inline bool QAtomicOpsBySize<2>::testAndSetRelaxed(T &_q_value, T expectedValue, T newValue) noexcept
{
    return QT_INTERLOCKED_FUNCTION(CompareExchange16)(atomic(&_q_value), value(newValue), value(expectedValue)) == value(expectedValue);
}

template<> template <typename T>
inline bool QAtomicOpsBySize<2>::testAndSetRelaxed(T &_q_value, T expectedValue, T newValue, T *currentValue) noexcept
{
    *currentValue = T(QT_INTERLOCKED_FUNCTION(CompareExchange16)(atomic(&_q_value), newValue, expectedValue));
    return *currentValue == expectedValue;
}

template<> template<typename T>
inline T QAtomicOpsBySize<2>::fetchAndStoreRelaxed(T &_q_value, T newValue) noexcept
{
    return QT_INTERLOCKED_FUNCTION(Exchange16)(atomic(&_q_value), value(newValue));
}

template<> template<typename T>
inline T QAtomicOpsBySize<2>::fetchAndAddRelaxed(T &_q_value, typename QAtomicAdditiveType<T>::AdditiveT valueToAdd) noexcept
{
    return QT_INTERLOCKED_FUNCTION(ExchangeAdd16)(atomic(&_q_value), value<T>(valueToAdd * QAtomicAdditiveType<T>::AddScale));
}
#endif

#ifdef Q_ATOMIC_INT64_IS_SUPPORTED
template<> template<typename T>
inline bool QAtomicOpsBySize<8>::ref(T &_q_value) noexcept
{
    return QT_INTERLOCKED_FUNCTION(Increment64)(atomic(&_q_value)) != 0;
}

template<> template<typename T>
inline bool QAtomicOpsBySize<8>::deref(T &_q_value) noexcept
{
    return QT_INTERLOCKED_FUNCTION(Decrement64)(atomic(&_q_value)) != 0;
}

template<> template<typename T>
inline bool QAtomicOpsBySize<8>::testAndSetRelaxed(T &_q_value, T expectedValue, T newValue) noexcept
{
    return QT_INTERLOCKED_FUNCTION(CompareExchange64)(atomic(&_q_value), value(newValue), value(expectedValue)) == value(expectedValue);
}

template<> template <typename T>
inline bool QAtomicOpsBySize<8>::testAndSetRelaxed(T &_q_value, T expectedValue, T newValue, T *currentValue) noexcept
{
    *currentValue = T(QT_INTERLOCKED_FUNCTION(CompareExchange64)(atomic(&_q_value), newValue, expectedValue));
    return *currentValue == expectedValue;
}

template<> template<typename T>
inline T QAtomicOpsBySize<8>::fetchAndStoreRelaxed(T &_q_value, T newValue) noexcept
{
    return QT_INTERLOCKED_FUNCTION(Exchange64)(atomic(&_q_value), value(newValue));
}

template<> template<typename T>
inline T QAtomicOpsBySize<8>::fetchAndAddRelaxed(T &_q_value, typename QAtomicAdditiveType<T>::AdditiveT valueToAdd) noexcept
{
    return QT_INTERLOCKED_FUNCTION(ExchangeAdd64)(atomic(&_q_value), value<T>(valueToAdd * QAtomicAdditiveType<T>::AddScale));
}
#endif

// Specialization for pointer types, since we have Interlocked*Pointer() variants in some configurations
template <typename T>
struct QAtomicOps<T *> : QGenericAtomicOps<QAtomicOps<T *> >
{
    typedef T *Type;

    static inline Q_DECL_CONSTEXPR bool isTestAndSetNative() noexcept { return true; }
    static inline Q_DECL_CONSTEXPR bool isTestAndSetWaitFree() noexcept { return true; }
    static bool testAndSetRelaxed(T *&_q_value, T *expectedValue, T *newValue) noexcept;
    static bool testAndSetRelaxed(T *&_q_value, T *expectedValue, T *newValue, T **currentValue) noexcept;

    static inline Q_DECL_CONSTEXPR bool isFetchAndStoreNative() noexcept { return true; }
    static inline Q_DECL_CONSTEXPR bool isFetchAndStoreWaitFree() noexcept { return true; }
    static T *fetchAndStoreRelaxed(T *&_q_value, T *newValue) noexcept;

    static inline Q_DECL_CONSTEXPR bool isFetchAndAddNative() noexcept { return true; }
    static inline Q_DECL_CONSTEXPR bool isFetchAndAddWaitFree() noexcept { return true; }
    static T *fetchAndAddRelaxed(T *&_q_value, qptrdiff valueToAdd) noexcept;
};

template <typename T>
inline bool QAtomicOps<T *>::testAndSetRelaxed(T *&_q_value, T *expectedValue, T *newValue) noexcept
{
    return QT_INTERLOCKED_COMPARE_EXCHANGE_POINTER(&_q_value, newValue, expectedValue) == expectedValue;
}

template <typename T>
inline bool QAtomicOps<T *>::testAndSetRelaxed(T *&_q_value, T *expectedValue, T *newValue, T **currentValue) noexcept
{
    *currentValue = reinterpret_cast<T *>(QT_INTERLOCKED_COMPARE_EXCHANGE_POINTER(&_q_value, newValue, expectedValue));
    return *currentValue == expectedValue;
}

template <typename T>
inline T *QAtomicOps<T *>::fetchAndStoreRelaxed(T *&_q_value, T *newValue) noexcept
{
    return reinterpret_cast<T *>(QT_INTERLOCKED_EXCHANGE_POINTER(&_q_value, newValue));
}

template <typename T>
inline T *QAtomicOps<T *>::fetchAndAddRelaxed(T *&_q_value, qptrdiff valueToAdd) noexcept
{
    return reinterpret_cast<T *>(QT_INTERLOCKED_EXCHANGE_ADD_POINTER(&_q_value, valueToAdd * sizeof(T)));
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cleanup

#undef QT_INTERLOCKED_CONCAT_I
#undef QT_INTERLOCKED_CONCAT
#undef QT_INTERLOCKED_FUNCTION
#undef QT_INTERLOCKED_PREFIX

#undef QT_INTERLOCKED_VOLATILE

#undef QT_INTERLOCKED_INCREMENT
#undef QT_INTERLOCKED_DECREMENT
#undef QT_INTERLOCKED_COMPARE_EXCHANGE
#undef QT_INTERLOCKED_EXCHANGE
#undef QT_INTERLOCKED_EXCHANGE_ADD
#undef QT_INTERLOCKED_COMPARE_EXCHANGE_POINTER
#undef QT_INTERLOCKED_EXCHANGE_POINTER
#undef QT_INTERLOCKED_EXCHANGE_ADD_POINTER

QT_END_NAMESPACE
#endif // QATOMIC_MSVC_H
