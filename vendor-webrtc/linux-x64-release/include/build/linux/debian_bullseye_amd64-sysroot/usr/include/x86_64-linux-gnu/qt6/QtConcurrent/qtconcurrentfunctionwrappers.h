// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTCONCURRENT_FUNCTIONWRAPPERS_H
#define QTCONCURRENT_FUNCTIONWRAPPERS_H

#include <QtConcurrent/qtconcurrentcompilertest.h>
#include <QtConcurrent/qtconcurrentreducekernel.h>
#include <QtCore/qfuture.h>

#include <tuple>

#if !defined(QT_NO_CONCURRENT) || defined(Q_CLANG_QDOC)

QT_BEGIN_NAMESPACE

namespace QtPrivate {

struct PushBackWrapper
{
    template <class C, class U>
    inline void operator()(C &c, const U &u) const
    {
        return c.push_back(u);
    }

    template <class C, class U>
    inline void operator()(C &c, U &&u) const
    {
        return c.push_back(u);
    }
};

// -- MapResultType

template <class T, class Enable = void>
struct Argument
{
    using Type = void;
};

template <class Sequence>
struct Argument<Sequence, typename std::enable_if<IsIterableValue<Sequence>>::type>
{
    using Type = std::decay_t<decltype(*std::declval<Sequence>().begin())>;
};

template <class Iterator>
struct Argument<Iterator, typename std::enable_if<IsDereferenceableValue<Iterator>>::type>
{
    using Type = std::decay_t<decltype(*std::declval<Iterator>())>;
};

template <class T>
using ArgumentType = typename Argument<T>::Type;

template <class T, class MapFunctor>
struct MapResult
{
    static_assert(std::is_invocable_v<std::decay_t<MapFunctor>, ArgumentType<T>>,
                  "It's not possible to invoke the function with passed argument.");
    using Type = std::invoke_result_t<std::decay_t<MapFunctor>, ArgumentType<T>>;
};

template <class T, class MapFunctor>
using MapResultType = typename MapResult<T, MapFunctor>::Type;

// -- ReduceResultType

template <class T>
struct ReduceResultType;

template <class U, class V>
struct ReduceResultType<void(*)(U&,V)>
{
    using ResultType = U;
};

template <class T, class C, class U>
struct ReduceResultType<T(C::*)(U)>
{
    using ResultType = C;
};

template <class U, class V>
struct ReduceResultType<std::function<void(U&, V)>>
{
    using ResultType = U;
};

template <typename R, typename ...A>
struct ReduceResultType<R(*)(A...)>
{
    using ResultType = typename std::tuple_element<0, std::tuple<A...>>::type;
};

template <class U, class V>
struct ReduceResultType<void(*)(U&,V) noexcept>
{
    using ResultType = U;
};

template <class T, class C, class U>
struct ReduceResultType<T(C::*)(U) noexcept>
{
    using ResultType = C;
};

template<class T, class Enable = void>
inline constexpr bool hasCallOperator_v = false;

template<class T>
inline constexpr bool hasCallOperator_v<T, std::void_t<decltype(&T::operator())>> = true;

template<class T, class Enable = void>
inline constexpr bool isIterator_v = false;

template<class T>
inline constexpr bool isIterator_v<T, std::void_t<typename std::iterator_traits<T>::value_type>> =
        true;

template <class Callable, class Sequence>
using isInvocable = std::is_invocable<Callable, typename std::decay_t<Sequence>::value_type>;

template <class InitialValueType, class ResultType>
inline constexpr bool isInitialValueCompatible_v = std::conjunction_v<
        std::is_convertible<InitialValueType, ResultType>,
        std::negation<std::is_same<std::decay_t<InitialValueType>, QtConcurrent::ReduceOption>>>;

template<class Callable, class Enable = void>
struct ReduceResultTypeHelper
{
};

template <class Callable>
struct ReduceResultTypeHelper<Callable,
        typename std::enable_if_t<std::is_function_v<std::remove_pointer_t<std::decay_t<Callable>>>
                                  || std::is_member_function_pointer_v<std::decay_t<Callable>>>>
{
    using type = typename QtPrivate::ReduceResultType<std::decay_t<Callable>>::ResultType;
};

template <class Callable>
struct ReduceResultTypeHelper<Callable,
        typename std::enable_if_t<!std::is_function_v<std::remove_pointer_t<std::decay_t<Callable>>>
                                  && hasCallOperator_v<std::decay_t<Callable>>>>
{
    using type = std::decay_t<typename QtPrivate::ArgResolver<Callable>::First>;
};

// -- MapSequenceResultType

template <class InputSequence, class MapFunctor>
struct MapSequenceResultType
{
    static_assert(std::is_same_v<typename InputSequence::value_type,
                                 QtPrivate::MapResultType<InputSequence, MapFunctor>>,
                  "Couldn't deduce the output sequence type, you must specify it explicitly.");
    typedef InputSequence ResultType;
};

#ifndef QT_NO_TEMPLATE_TEMPLATE_PARAMETERS

template <template <typename...> class InputSequence, typename MapFunctor, typename ...T>
struct MapSequenceResultType<InputSequence<T...>, MapFunctor>
{
    typedef InputSequence<QtPrivate::MapResultType<InputSequence<T...>, MapFunctor>> ResultType;
};

#endif // QT_NO_TEMPLATE_TEMPLATE_PARAMETER

} // namespace QtPrivate.


QT_END_NAMESPACE

#endif // QT_NO_CONCURRENT

#endif
