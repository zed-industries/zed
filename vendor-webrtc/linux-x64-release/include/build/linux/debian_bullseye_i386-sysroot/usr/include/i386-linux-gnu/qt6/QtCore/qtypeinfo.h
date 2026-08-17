// Copyright (C) 2016 The Qt Company Ltd.
// Copyright (C) 2016 Intel Corporation.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#include <QtCore/qglobal.h>
#include <QtCore/qcontainerfwd.h>
#include <variant>
#include <optional>
#include <tuple>

#ifndef QTYPEINFO_H
#define QTYPEINFO_H

QT_BEGIN_NAMESPACE

class QDebug;

/*
   QTypeInfo     - type trait functionality
*/

template <typename T>
inline constexpr bool qIsRelocatable =  std::is_trivially_copyable_v<T> && std::is_trivially_destructible_v<T>;

/*
  The catch-all template.
*/

template <typename T>
class QTypeInfo
{
public:
    enum {
        isPointer = std::is_pointer_v<T>,
        isIntegral = std::is_integral_v<T>,
        isComplex = !std::is_trivial_v<T>,
        isRelocatable = qIsRelocatable<T>,
    };
};

template<>
class QTypeInfo<void>
{
public:
    enum {
        isPointer = false,
        isIntegral = false,
        isComplex = false,
        isRelocatable = false,
    };
};

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
template <class T, class...Ts>
class QTypeInfoMerger
{
    static_assert(sizeof...(Ts) > 0);
public:
    static constexpr bool isComplex = ((QTypeInfo<Ts>::isComplex) || ...);
    static constexpr bool isRelocatable = ((QTypeInfo<Ts>::isRelocatable) && ...);
    static constexpr bool isPointer = false;
    static constexpr bool isIntegral = false;
};

#define Q_DECLARE_MOVABLE_CONTAINER(CONTAINER) \
template <typename ...T> \
class QTypeInfo<CONTAINER<T...>> \
{ \
public: \
    enum { \
        isPointer = false, \
        isIntegral = false, \
        isComplex = true, \
        isRelocatable = true, \
    }; \
}

Q_DECLARE_MOVABLE_CONTAINER(QList);
Q_DECLARE_MOVABLE_CONTAINER(QQueue);
Q_DECLARE_MOVABLE_CONTAINER(QStack);
Q_DECLARE_MOVABLE_CONTAINER(QSet);
Q_DECLARE_MOVABLE_CONTAINER(QMap);
Q_DECLARE_MOVABLE_CONTAINER(QMultiMap);
Q_DECLARE_MOVABLE_CONTAINER(QHash);
Q_DECLARE_MOVABLE_CONTAINER(QMultiHash);
Q_DECLARE_MOVABLE_CONTAINER(QCache);

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
    Q_RELOCATABLE_TYPE = 0x2,
    Q_MOVABLE_TYPE = 0x2,
    Q_DUMMY_TYPE = 0x4,
};

#define Q_DECLARE_TYPEINFO_BODY(TYPE, FLAGS) \
class QTypeInfo<TYPE > \
{ \
public: \
    enum { \
        isComplex = (((FLAGS) & Q_PRIMITIVE_TYPE) == 0) && !std::is_trivial_v<TYPE>, \
        isRelocatable = !isComplex || ((FLAGS) & Q_RELOCATABLE_TYPE) || qIsRelocatable<TYPE>, \
        isPointer = false, \
        isIntegral = std::is_integral< TYPE >::value, \
    }; \
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
*/

#define Q_DECLARE_SHARED_IMPL(TYPE, FLAGS) \
Q_DECLARE_TYPEINFO(TYPE, FLAGS); \
inline void swap(TYPE &value1, TYPE &value2) \
    noexcept(noexcept(value1.swap(value2))) \
{ value1.swap(value2); }
#define Q_DECLARE_SHARED(TYPE) Q_DECLARE_SHARED_IMPL(TYPE, Q_RELOCATABLE_TYPE)

namespace QTypeTraits
{

/*
    The templates below aim to find out whether one can safely instantiate an operator==() or
    operator<() for a type.

    This is tricky for containers, as most containers have unconstrained comparison operators, even though they
    rely on the corresponding operators for its content.
    This is especially true for all of the STL template classes that have a comparison operator defined, and
    leads to the situation, that the compiler would try to instantiate the operator, and fail if any
    of its template arguments does not have the operator implemented.

    The code tries to cover the relevant cases for Qt and the STL, by checking (recusrsively) the value_type
    of a container (if it exists), and checking the template arguments of pair, tuple and variant.
*/
namespace detail {

// find out whether T is a conteiner
// this is required to check the value type of containers for the existence of the comparison operator
template <typename, typename = void>
struct is_container : std::false_type {};
template <typename T>
struct is_container<T, std::void_t<
        typename T::value_type,
        std::is_convertible<decltype(std::declval<T>().begin() != std::declval<T>().end()), bool>
>> : std::true_type {};


// Checks the existence of the comparison operator for the class itself
QT_WARNING_PUSH
QT_WARNING_DISABLE_FLOAT_COMPARE
template <typename, typename = void>
struct has_operator_equal : std::false_type {};
template <typename T>
struct has_operator_equal<T, std::void_t<decltype(bool(std::declval<const T&>() == std::declval<const T&>()))>>
        : std::true_type {};
QT_WARNING_POP

// Two forward declarations
template<typename T, bool = is_container<T>::value>
struct expand_operator_equal_container;
template<typename T>
struct expand_operator_equal_tuple;

// the entry point for the public method
template<typename T>
using expand_operator_equal = expand_operator_equal_container<T>;

// if T isn't a container check if it's a tuple like object
template<typename T, bool>
struct expand_operator_equal_container : expand_operator_equal_tuple<T> {};
// if T::value_type exists, check first T::value_type, then T itself
template<typename T>
struct expand_operator_equal_container<T, true> :
        std::conjunction<
        std::disjunction<
            std::is_same<T, typename T::value_type>, // avoid endless recursion
            expand_operator_equal<typename T::value_type>
        >, expand_operator_equal_tuple<T>> {};

// recursively check the template arguments of a tuple like object
template<typename ...T>
using expand_operator_equal_recursive = std::conjunction<expand_operator_equal<T>...>;

template<typename T>
struct expand_operator_equal_tuple : has_operator_equal<T> {};
template<typename T>
struct expand_operator_equal_tuple<std::optional<T>> : has_operator_equal<T> {};
template<typename T1, typename T2>
struct expand_operator_equal_tuple<std::pair<T1, T2>> : expand_operator_equal_recursive<T1, T2> {};
template<typename ...T>
struct expand_operator_equal_tuple<std::tuple<T...>> : expand_operator_equal_recursive<T...> {};
template<typename ...T>
struct expand_operator_equal_tuple<std::variant<T...>> : expand_operator_equal_recursive<T...> {};

// the same for operator<(), see above for explanations
template <typename, typename = void>
struct has_operator_less_than : std::false_type{};
template <typename T>
struct has_operator_less_than<T, std::void_t<decltype(bool(std::declval<const T&>() < std::declval<const T&>()))>>
        : std::true_type{};

template<typename T, bool = is_container<T>::value>
struct expand_operator_less_than_container;
template<typename T>
struct expand_operator_less_than_tuple;

template<typename T>
using expand_operator_less_than = expand_operator_less_than_container<T>;

template<typename T, bool>
struct expand_operator_less_than_container : expand_operator_less_than_tuple<T> {};
template<typename T>
struct expand_operator_less_than_container<T, true> :
        std::conjunction<
            std::disjunction<
                std::is_same<T, typename T::value_type>,
                expand_operator_less_than<typename T::value_type>
            >, expand_operator_less_than_tuple<T>
        > {};

template<typename ...T>
using expand_operator_less_than_recursive = std::conjunction<expand_operator_less_than<T>...>;

template<typename T>
struct expand_operator_less_than_tuple : has_operator_less_than<T> {};
template<typename T>
struct expand_operator_less_than_tuple<std::optional<T>> : has_operator_less_than<T> {};
template<typename T1, typename T2>
struct expand_operator_less_than_tuple<std::pair<T1, T2>> : expand_operator_less_than_recursive<T1, T2> {};
template<typename ...T>
struct expand_operator_less_than_tuple<std::tuple<T...>> : expand_operator_less_than_recursive<T...> {};
template<typename ...T>
struct expand_operator_less_than_tuple<std::variant<T...>> : expand_operator_less_than_recursive<T...> {};

}

template<typename T, typename = void>
struct is_dereferenceable : std::false_type {};

template<typename T>
struct is_dereferenceable<T, std::void_t<decltype(std::declval<T>().operator->())> >
    : std::true_type {};

template <typename T>
inline constexpr bool is_dereferenceable_v = is_dereferenceable<T>::value;

template<typename T>
struct has_operator_equal : detail::expand_operator_equal<T> {};
template<typename T>
inline constexpr bool has_operator_equal_v = has_operator_equal<T>::value;

template <typename Container, typename T>
using has_operator_equal_container = std::disjunction<std::is_base_of<Container, T>, QTypeTraits::has_operator_equal<T>>;

template<typename T>
struct has_operator_less_than : detail::expand_operator_less_than<T> {};
template<typename T>
inline constexpr bool has_operator_less_than_v = has_operator_less_than<T>::value;

template <typename Container, typename T>
using has_operator_less_than_container = std::disjunction<std::is_base_of<Container, T>, QTypeTraits::has_operator_less_than<T>>;

template <typename ...T>
using compare_eq_result = std::enable_if_t<std::conjunction_v<QTypeTraits::has_operator_equal<T>...>, bool>;

template <typename Container, typename ...T>
using compare_eq_result_container = std::enable_if_t<std::conjunction_v<QTypeTraits::has_operator_equal_container<Container, T>...>, bool>;

template <typename ...T>
using compare_lt_result = std::enable_if_t<std::conjunction_v<QTypeTraits::has_operator_less_than<T>...>, bool>;

template <typename Container, typename ...T>
using compare_lt_result_container = std::enable_if_t<std::conjunction_v<QTypeTraits::has_operator_less_than_container<Container, T>...>, bool>;

namespace detail {

template<typename T>
const T &const_reference();
template<typename T>
T &reference();

}

template <typename Stream, typename, typename = void>
struct has_ostream_operator : std::false_type {};
template <typename Stream, typename T>
struct has_ostream_operator<Stream, T, std::void_t<decltype(detail::reference<Stream>() << detail::const_reference<T>())>>
        : std::true_type {};
template <typename Stream, typename T>
inline constexpr bool has_ostream_operator_v = has_ostream_operator<Stream, T>::value;

template <typename Stream, typename Container, typename T>
using has_ostream_operator_container = std::disjunction<std::is_base_of<Container, T>, QTypeTraits::has_ostream_operator<Stream, T>>;

template <typename Stream, typename, typename = void>
struct has_istream_operator : std::false_type {};
template <typename Stream, typename T>
struct has_istream_operator<Stream, T, std::void_t<decltype(detail::reference<Stream>() >> detail::reference<T>())>>
        : std::true_type {};
template <typename Stream, typename T>
inline constexpr bool has_istream_operator_v = has_istream_operator<Stream, T>::value;
template <typename Stream, typename Container, typename T>
using has_istream_operator_container = std::disjunction<std::is_base_of<Container, T>, QTypeTraits::has_istream_operator<Stream, T>>;

template <typename Stream, typename T>
inline constexpr bool has_stream_operator_v = has_ostream_operator_v<Stream, T> && has_istream_operator_v<Stream, T>;

}


QT_END_NAMESPACE
#endif // QTYPEINFO_H
