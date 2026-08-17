// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTBASE_QTTASKBUILDER_H
#define QTBASE_QTTASKBUILDER_H

#if !defined(QT_NO_CONCURRENT) || defined(Q_CLANG_QDOC)

#include <QtConcurrent/qtconcurrentstoredfunctioncall.h>

QT_BEGIN_NAMESPACE

#ifdef Q_CLANG_QDOC

namespace QtConcurrent {

enum class FutureResult { Ignore };

using InvokeResultType = int;

template <class Task, class ...Args>
class QTaskBuilder
{
public:
    [[nodiscard]]
    QFuture<InvokeResultType> spawn();

    void spawn(FutureResult);

    template <class ...ExtraArgs>
    [[nodiscard]]
    QTaskBuilder<Task, ExtraArgs...> withArguments(ExtraArgs &&...args);

    [[nodiscard]]
    QTaskBuilder<Task, Args...> &onThreadPool(QThreadPool &newThreadPool);

    [[nodiscard]]
    QTaskBuilder<Task, Args...> &withPriority(int newPriority);
};

} // namespace QtConcurrent

#else

namespace QtConcurrent {

enum class FutureResult { Ignore };

template <class Task, class ...Args>
class QTaskBuilder
{
public:
    [[nodiscard]]
    auto spawn()
    {
        return TaskResolver<std::decay_t<Task>, std::decay_t<Args>...>::run(
                    std::move(taskWithArgs), startParameters);
    }

    // We don't want to run with promise when we don't return a QFuture
    void spawn(FutureResult)
    {
        (new StoredFunctionCall<Task, Args...>(std::move(taskWithArgs)))
            ->start(startParameters);
    }

    template <class ...ExtraArgs>
    [[nodiscard]]
    constexpr auto withArguments(ExtraArgs &&...args)
    {
        static_assert(std::tuple_size_v<TaskWithArgs> == 1,
                      "This function cannot be invoked if "
                      "arguments have already been passed.");

        static_assert(sizeof...(ExtraArgs) >= 1,
                      "One or more arguments must be passed.");

        // We have to re-create a builder, because the type has changed
        return QTaskBuilder<Task, ExtraArgs...>(
                   startParameters,
                   std::get<0>(std::move(taskWithArgs)),
                   std::forward<ExtraArgs>(args)...
               );
    }

    [[nodiscard]]
    constexpr auto &onThreadPool(QThreadPool &newThreadPool)
    {
        startParameters.threadPool = &newThreadPool;
        return *this;
    }

    [[nodiscard]]
    constexpr auto &withPriority(int newPriority)
    {
        startParameters.priority = newPriority;
        return *this;
    }

protected: // Methods
    constexpr explicit QTaskBuilder(Task &&task, Args &&...arguments)
        : taskWithArgs{std::forward<Task>(task), std::forward<Args>(arguments)...}
    {}

    constexpr QTaskBuilder(
        const TaskStartParameters &parameters, Task &&task, Args &&...arguments)
        : taskWithArgs{std::forward<Task>(task), std::forward<Args>(arguments)...}
        , startParameters{parameters}
    {}

private: // Methods
    // Required for creating a builder from "task" function
    template <class T>
    friend constexpr auto task(T &&t);

    // Required for creating a new builder from "withArguments" function
    template <class T, class ...A>
    friend class QTaskBuilder;

private: // Data
    using TaskWithArgs = DecayedTuple<Task, Args...>;

    TaskWithArgs taskWithArgs;
    TaskStartParameters startParameters;
};

} // namespace QtConcurrent

#endif // Q_CLANG_QDOC

QT_END_NAMESPACE

#endif // !defined(QT_NO_CONCURRENT)

#endif //QTBASE_QTTASKBUILDER_H
