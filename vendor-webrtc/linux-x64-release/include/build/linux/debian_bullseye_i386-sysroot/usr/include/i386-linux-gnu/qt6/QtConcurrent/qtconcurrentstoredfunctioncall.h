// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTCONCURRENT_STOREDFUNCTIONCALL_H
#define QTCONCURRENT_STOREDFUNCTIONCALL_H

#include <QtConcurrent/qtconcurrent_global.h>

#ifndef QT_NO_CONCURRENT
#include <QtConcurrent/qtconcurrentrunbase.h>
#include <QtCore/qpromise.h>

#include <type_traits>

QT_BEGIN_NAMESPACE

#ifndef Q_QDOC

namespace QtConcurrent {

template<typename...>
struct NonMemberFunctionResolver;

template <class Function, class PromiseType, class... Args>
struct NonMemberFunctionResolver<Function, PromiseType, Args...>
{
    using Type = std::tuple<std::decay_t<Function>, QPromise<PromiseType> &, std::decay_t<Args>...>;
    static_assert(std::is_invocable_v<std::decay_t<Function>, QPromise<PromiseType> &, std::decay_t<Args>...>,
                  "It's not possible to invoke the function with passed arguments.");
    static_assert(std::is_void_v<std::invoke_result_t<std::decay_t<Function>, QPromise<PromiseType> &, std::decay_t<Args>...>>,
                  "The function must return void type.");

    static constexpr void invoke(std::decay_t<Function> function, QPromise<PromiseType> &promise,
                                 std::decay_t<Args>... args)
    {
        std::invoke(function, promise, args...);
    }
    static Type initData(Function &&f, QPromise<PromiseType> &promise, Args &&...args)
    {
        return Type { std::forward<Function>(f), std::ref(promise), std::forward<Args>(args)... };
    }
};

template<typename...>
struct MemberFunctionResolver;

template <typename Function, typename PromiseType, typename Arg, typename ... Args>
struct MemberFunctionResolver<Function, PromiseType, Arg, Args...>
{
    using Type = std::tuple<std::decay_t<Function>, std::decay_t<Arg>, QPromise<PromiseType> &, std::decay_t<Args>...>;
    static_assert(std::is_invocable_v<std::decay_t<Function>, std::decay_t<Arg>, QPromise<PromiseType> &, std::decay_t<Args>...>,
                  "It's not possible to invoke the function with passed arguments.");
    static_assert(std::is_void_v<std::invoke_result_t<std::decay_t<Function>, std::decay_t<Arg>, QPromise<PromiseType> &, std::decay_t<Args>...>>,
                  "The function must return void type.");

    static constexpr void invoke(std::decay_t<Function> function, std::decay_t<Arg> object,
                                 QPromise<PromiseType> &promise, std::decay_t<Args>... args)
    {
        std::invoke(function, object, promise, args...);
    }
    static Type initData(Function &&f, QPromise<PromiseType> &promise, Arg &&fa, Args &&...args)
    {
        return Type { std::forward<Function>(f), std::forward<Arg>(fa), std::ref(promise), std::forward<Args>(args)... };
    }
};

template <class IsMember, class Function, class PromiseType, class... Args>
struct FunctionResolverHelper;

template <class Function, class PromiseType, class... Args>
struct FunctionResolverHelper<std::false_type, Function, PromiseType, Args...>
        : public NonMemberFunctionResolver<Function, PromiseType, Args...>
{
};

template <class Function, class PromiseType, class... Args>
struct FunctionResolverHelper<std::true_type, Function, PromiseType, Args...>
        : public MemberFunctionResolver<Function, PromiseType, Args...>
{
};

template <class Function, class PromiseType, class... Args>
struct FunctionResolver
        : public FunctionResolverHelper<typename std::is_member_function_pointer<
                 std::decay_t<Function>>::type, Function, PromiseType, Args...>
{
};

template <class Function, class ...Args>
struct InvokeResult
{
    static_assert(std::is_invocable_v<std::decay_t<Function>, std::decay_t<Args>...>,
                  "It's not possible to invoke the function with passed arguments.");

    using Type = std::invoke_result_t<std::decay_t<Function>, std::decay_t<Args>...>;
};

template <class Function, class ...Args>
using InvokeResultType = typename InvokeResult<Function, Args...>::Type;

template <class ...Types>
using DecayedTuple = std::tuple<std::decay_t<Types>...>;

template <class Function, class ...Args>
struct StoredFunctionCall : public RunFunctionTaskBase<InvokeResultType<Function, Args...>>
{
    StoredFunctionCall(DecayedTuple<Function, Args...> &&_data)
        : data(std::move(_data))
    {}

protected:
    void runFunctor() override
    {
        constexpr auto invoke = [] (std::decay_t<Function> function,
                                    std::decay_t<Args>... args) -> auto {
            return std::invoke(function, args...);
        };

        if constexpr (std::is_void_v<InvokeResultType<Function, Args...>>) {
            std::apply(invoke, std::move(data));
        } else {
            auto result = std::apply(invoke, std::move(data));

            using T = InvokeResultType<Function, Args...>;
            if constexpr (std::is_move_constructible_v<T>)
                this->promise.reportAndMoveResult(std::move(result));
            else if constexpr (std::is_copy_constructible_v<T>)
                this->promise.reportResult(result);
        }
    }

private:
    DecayedTuple<Function, Args...> data;
};

template <class Function, class PromiseType, class ...Args>
struct StoredFunctionCallWithPromise : public RunFunctionTaskBase<PromiseType>
{
    using Resolver = FunctionResolver<Function, PromiseType, Args...>;
    using DataType = typename Resolver::Type;
    StoredFunctionCallWithPromise(Function &&f, Args &&...args)
        : prom(this->promise),
          data(std::move(Resolver::initData(std::forward<Function>(f), std::ref(prom),
                                            std::forward<Args>(args)...)))
    {}

    StoredFunctionCallWithPromise(DecayedTuple<Function, Args...> &&_data)
        : StoredFunctionCallWithPromise(std::move(_data),
               std::index_sequence_for<std::decay_t<Function>, std::decay_t<Args>...>())
    {}

protected:
    void runFunctor() override
    {
        std::apply(Resolver::invoke, std::move(data));
    }

private:
    // helper to pack back the tuple into parameter pack
    template<std::size_t... Is>
    StoredFunctionCallWithPromise(DecayedTuple<Function, Args...> &&_data,
                                  std::index_sequence<Is...>)
        : StoredFunctionCallWithPromise(std::move(std::get<Is>(_data))...)
    {}

    QPromise<PromiseType> prom;
    DataType data;
};

template<typename...>
struct NonPromiseTaskResolver;

template <typename Function, typename ... Args>
struct NonPromiseTaskResolver<Function, Args...>
{
    using TaskWithArgs = DecayedTuple<Function, Args...>;
    static auto run(TaskWithArgs &&args, const TaskStartParameters &startParameters) {
        return (new StoredFunctionCall<Function, Args...>(std::move(args)))
                ->start(startParameters);
    }
};

template<typename...>
struct PromiseTaskResolver;

template <typename Function, typename ... Args>
struct PromiseTaskResolver<Function, Args...>
{
    static_assert(QtPrivate::ArgResolver<Function>::IsPromise::value,
        "The first argument of passed callable object isn't a QPromise<T> & type. "
        "Did you intend to pass a callable which takes a QPromise<T> & type as a first argument? "
        "Otherwise it's not possible to invoke the function with passed arguments.");
    using TaskWithArgs = DecayedTuple<Function, Args...>;
    static auto run(TaskWithArgs &&args, const TaskStartParameters &startParameters) {
        using PromiseType = typename QtPrivate::ArgResolver<Function>::PromiseType;
        return (new StoredFunctionCallWithPromise<Function, PromiseType, Args...>(std::move(args)))
                   ->start(startParameters);
    }
};

template <class IsDirectlyInvocable, class Function, class... Args>
struct TaskResolverHelper;

template <class Function, class... Args>
struct TaskResolverHelper<std::true_type, Function, Args...>
        : public NonPromiseTaskResolver<Function, Args...>
{
};

template <class Function, class... Args>
struct TaskResolverHelper<std::false_type, Function, Args...>
        : public PromiseTaskResolver<Function, Args...>
{
};

template <class Function, class... Args>
struct TaskResolver : public TaskResolverHelper<typename std::is_invocable<std::decay_t<Function>,
        std::decay_t<Args>...>::type, Function, Args...>
{
};

} //namespace QtConcurrent

#endif // Q_QDOC

QT_END_NAMESPACE

#endif // QT_NO_CONCURRENT

#endif
