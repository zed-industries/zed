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

#include <QtCore/qglobal.h>

#ifndef QTYPEINFO_H
#define QTYPEINFO_H

QT_BEGIN_NAMESPACE

/*
   QTypeInfo     - type trait functionality
*/

template <typename T>
static constexpr bool qIsRelocatable()
{
#if defined(Q_CC_CLANG) || !defined(Q_CC_GNU) || Q_CC_GNU >= 501
    return std::is_trivially_copyable<T>::value && std::is_trivially_destructible<T>::value;
#else
    return std::is_enum<T>::value || std::is_integral<T>::value;
#endif
}

template <typename T>
static constexpr bool qIsTrivial()
{
#if defined(Q_CC_CLANG) || !defined(Q_CC_GNU) || Q_CC_GNU >= 501
    return std::is_trivial<T>::value;
#else
    return std::is_enum<T>::value || std::is_integral<T>::value;
#endif
}

/*
  The catch-all template.
*/

template <typename T>
class QTypeInfo
{
public:
    enum {
        isSpecialized = std::is_enum<T>::value, // don't require every enum to be marked manually
        isPointer = false,
        isIntegral = std::is_integral<T>::value,
        isComplex = !qIsTrivial<T>(),
        isStatic = true,
        isRelocatable = qIsRelocatable<T>(),
        isLarge = (sizeof(T)>sizeof(void*)),
        isDummy = false, //### Qt6: remove
        sizeOf = sizeof(T)
    };
};

template<>
class QTypeInfo<void>
{
public:
    enum {
        isSpecialized = true,
        isPointer = false,
        isIntegral = false,
        isComplex = false,
        isStatic = false,
        isRelocatable = false,
        isLarge = false,
        isDummy = false,
        sizeOf = 0
    };
};

template <typename T>
class QTypeInfo<T*>
{
public:
    enum {
        isSpecialized = true,
        isPointer = true,
        isIntegral = false,
        isComplex = false,
        isStatic = false,
        isRelocatable = true,
        isLarge = false,
        isDummy = false,
        sizeOf = sizeof(T*)
    };
};

/*!
    \class QTypeInfoQuery
    \inmodule QtCore
    \internal
    \brief QTypeInfoQuery is used to query the values of a given QTypeInfo<T>

    We use it because there may be some QTypeInfo<T> specializations in user
    code that don't provide certain flags that we added after Qt 5.0. They are:
    \list
      \li isRelocatable: defaults to !isStatic
    \endlist

    DO NOT specialize this class elsewhere.
*/
// apply defaults for a generic QTypeInfo<T> that didn't provide the new values
template <typename T, typename = void>
struct QTypeInfoQuery : public QTypeInfo<T>
{
    enum { isRelocatable = !QTypeInfo<T>::isStatic };
};

// if QTypeInfo<T>::isRelocatable exists, use it
template <typename T>
struct QTypeInfoQuery<T, typename std::enable_if<QTypeInfo<T>::isRelocatable || true>::type> : public QTypeInfo<T>
{};

/*!
    \class QTypeInfoMerger
    \inmodule QtCore
    \internal

    \brief QTypeInfoMerger merges the QTypeInfo flags of T1, T2... and presents them
    as a QTypeInfo<T> would do.

    Let's assume that we have a simple set of structs:

    \snippet code/src_corelib_global_qglobal.cpp 50

    To create a proper QTypeInfo specialization for A struct, we have to check
    all sub-components; B, C and D, then take the lowest common denominator and call
    Q_DECLARE_TYPEINFO with the resulting flags. An easier and less fragile approach is to
    use QTypeInfoMerger, which does that automatically. So struct A would have
    the following QTypeInfo definition:

    \snippet code/src_corelib_global_qglobal.cpp 51
*/
template <class T, class T1, class T2 = T1, class T3 = T1, class T4 = T1>
class QTypeInfoMerger
{
public:
    enum {
        isSpecialized = true,
        isComplex = QTypeInfoQuery<T1>::isComplex || QTypeInfoQuery<T2>::isComplex
                    || QTypeInfoQuery<T3>::isComplex || QTypeInfoQuery<T4>::isComplex,
        isStatic = QTypeInfoQuery<T1>::isStatic || QTypeInfoQuery<T2>::isStatic
                    || QTypeInfoQuery<T3>::isStatic || QTypeInfoQuery<T4>::isStatic,
        isRelocatable = QTypeInfoQuery<T1>::isRelocatable && QTypeInfoQuery<T2>::isRelocatable
                    && QTypeInfoQuery<T3>::isRelocatable && QTypeInfoQuery<T4>::isRelocatable,
        isLarge = sizeof(T) > sizeof(void*),
        isPointer = false,
        isIntegral = false,
        isDummy = false,
        sizeOf = sizeof(T)
    };
};

#define Q_DECLARE_MOVABLE_CONTAINER(CONTAINER) \
template <typename T> class CONTAINER; \
template <typename T> \
class QTypeInfo< CONTAINER<T> > \
{ \
public: \
    enum { \
        isSpecialized = true, \
        isPointer = false, \
        isIntegral = false, \
        isComplex = true, \
        isRelocatable = true, \
        isStatic = false, \
        isLarge = (sizeof(CONTAINER<T>) > sizeof(void*)), \
        isDummy = false, \
        sizeOf = sizeof(CONTAINER<T>) \
    }; \
}

Q_DECLARE_MOVABLE_CONTAINER(QList);
Q_DECLARE_MOVABLE_CONTAINER(QVector);
Q_DECLARE_MOVABLE_CONTAINER(QQueue);
Q_DECLARE_MOVABLE_CONTAINER(QStack);
Q_DECLARE_MOVABLE_CONTAINER(QSet);

#undef Q_DECLARE_MOVABLE_CONTAINER

/* These cannot be movable before ### Qt 6, for BC reasons */
#define Q_DECLARE_MOVABLE_CONTAINER(CONTAINER) \
template <typename K, typename V> class CONTAINER; \
template <typename K, typename V> \
class QTypeInfo< CONTAINER<K, V> > \
{ \
public: \
    enum { \
        isSpecialized = true, \
        isPointer = false, \
        isIntegral = false, \
        isComplex = true, \
        isStatic = (QT_VERSION < QT_VERSION_CHECK(6, 0, 0)), \
        isRelocatable = true, \
        isLarge = (sizeof(CONTAINER<K, V>) > sizeof(void*)), \
        isDummy = false, \
        sizeOf = sizeof(CONTAINER<K, V>) \
    }; \
}

Q_DECLARE_MOVABLE_CONTAINER(QMap);
Q_DECLARE_MOVABLE_CONTAINER(QMultiMap);
Q_DECLARE_MOVABLE_CONTAINER(QHash);
Q_DECLARE_MOVABLE_CONTAINER(QMultiHash);

#undef Q_DECLARE_MOVABLE_CONTAINER

/*
   Specialize a specific type with:

     Q_DECLARE_TYPEINFO(type, flags);

   where 'type' is the name of the type to specialize and 'flags' is
   logically-OR'ed combination of the flags below.
*/
enum { /* TYPEINFO flags */
    Q_COMPLEX_TYPE = 0,
    Q_PRIMITIVE_TYPE = 0x1,
    Q_STATIC_TYPE = 0,
    Q_MOVABLE_TYPE = 0x2,               // ### Qt6: merge movable and relocatable once QList no longer depends on it
    Q_DUMMY_TYPE = 0x4,
    Q_RELOCATABLE_TYPE = 0x8
};

#define Q_DECLARE_TYPEINFO_BODY(TYPE, FLAGS) \
class QTypeInfo<TYPE > \
{ \
public: \
    enum { \
        isSpecialized = true, \
        isComplex = (((FLAGS) & Q_PRIMITIVE_TYPE) == 0) && !qIsTrivial<TYPE>(), \
        isStatic = (((FLAGS) & (Q_MOVABLE_TYPE | Q_PRIMITIVE_TYPE)) == 0), \
        isRelocatable = !isStatic || ((FLAGS) & Q_RELOCATABLE_TYPE) || qIsRelocatable<TYPE>(), \
        isLarge = (sizeof(TYPE)>sizeof(void*)), \
        isPointer = false, \
        isIntegral = std::is_integral< TYPE >::value, \
        isDummy = (((FLAGS) & Q_DUMMY_TYPE) != 0), \
        sizeOf = sizeof(TYPE) \
    }; \
    static inline const char *name() { return #TYPE; } \
}

#define Q_DECLARE_TYPEINFO(TYPE, FLAGS) \
template<> \
Q_DECLARE_TYPEINFO_BODY(TYPE, FLAGS)

/* Specialize QTypeInfo for QFlags<T> */
template<typename T> class QFlags;
template<typename T>
Q_DECLARE_TYPEINFO_BODY(QFlags<T>, Q_PRIMITIVE_TYPE);

/*
   Specialize a shared type with:

     Q_DECLARE_SHARED(type)

   where 'type' is the name of the type to specialize.  NOTE: shared
   types must define a member-swap, and be defined in the same
   namespace as Qt for this to work.

   If the type was already released without Q_DECLARE_SHARED applied,
   _and_ without an explicit Q_DECLARE_TYPEINFO(type, Q_MOVABLE_TYPE),
   then use Q_DECLARE_SHARED_NOT_MOVABLE_UNTIL_QT6(type) to mark the
   type shared (incl. swap()), without marking it movable (which
   would change the memory layout of QList, a BiC change.
*/

#define Q_DECLARE_SHARED_IMPL(TYPE, FLAGS) \
Q_DECLARE_TYPEINFO(TYPE, FLAGS); \
inline void swap(TYPE &value1, TYPE &value2) \
    noexcept(noexcept(value1.swap(value2))) \
{ value1.swap(value2); }
#define Q_DECLARE_SHARED(TYPE) Q_DECLARE_SHARED_IMPL(TYPE, Q_MOVABLE_TYPE)
#define Q_DECLARE_SHARED_NOT_MOVABLE_UNTIL_QT6(TYPE) \
                               Q_DECLARE_SHARED_IMPL(TYPE, QT_VERSION >= QT_VERSION_CHECK(6,0,0) ? Q_MOVABLE_TYPE : Q_RELOCATABLE_TYPE)

/*
   QTypeInfo primitive specializations
*/
Q_DECLARE_TYPEINFO(bool, Q_PRIMITIVE_TYPE);
Q_DECLARE_TYPEINFO(char, Q_PRIMITIVE_TYPE);
Q_DECLARE_TYPEINFO(signed char, Q_PRIMITIVE_TYPE);
Q_DECLARE_TYPEINFO(uchar, Q_PRIMITIVE_TYPE);
Q_DECLARE_TYPEINFO(short, Q_PRIMITIVE_TYPE);
Q_DECLARE_TYPEINFO(ushort, Q_PRIMITIVE_TYPE);
Q_DECLARE_TYPEINFO(int, Q_PRIMITIVE_TYPE);
Q_DECLARE_TYPEINFO(uint, Q_PRIMITIVE_TYPE);
Q_DECLARE_TYPEINFO(long, Q_PRIMITIVE_TYPE);
Q_DECLARE_TYPEINFO(ulong, Q_PRIMITIVE_TYPE);
Q_DECLARE_TYPEINFO(qint64, Q_PRIMITIVE_TYPE);
Q_DECLARE_TYPEINFO(quint64, Q_PRIMITIVE_TYPE);
Q_DECLARE_TYPEINFO(float, Q_PRIMITIVE_TYPE);
Q_DECLARE_TYPEINFO(double, Q_PRIMITIVE_TYPE);

#if QT_VERSION >= QT_VERSION_CHECK(6,0,0)
// ### Qt 6: remove the other branch
// This was required so that QList<T> for these types allocates out of the array storage
Q_DECLARE_TYPEINFO(long double, Q_PRIMITIVE_TYPE);
#  ifdef Q_COMPILER_UNICODE_STRINGS
Q_DECLARE_TYPEINFO(char16_t, Q_PRIMITIVE_TYPE);
Q_DECLARE_TYPEINFO(char32_t, Q_PRIMITIVE_TYPE);
#  endif
#  if !defined(Q_CC_MSVC) || defined(_NATIVE_WCHAR_T_DEFINED)
Q_DECLARE_TYPEINFO(wchar_t, Q_PRIMITIVE_TYPE);
#  endif
#else
#  ifndef Q_OS_DARWIN
Q_DECLARE_TYPEINFO(long double, Q_PRIMITIVE_TYPE);
#  else
Q_DECLARE_TYPEINFO(long double, Q_RELOCATABLE_TYPE);
#  endif
#  ifdef Q_COMPILER_UNICODE_STRINGS
Q_DECLARE_TYPEINFO(char16_t, Q_RELOCATABLE_TYPE);
Q_DECLARE_TYPEINFO(char32_t, Q_RELOCATABLE_TYPE);
#  endif
#  if !defined(Q_CC_MSVC) || defined(_NATIVE_WCHAR_T_DEFINED)
Q_DECLARE_TYPEINFO(wchar_t, Q_RELOCATABLE_TYPE);
#  endif
#endif // Qt 6

QT_END_NAMESPACE
#endif // QTYPEINFO_H
