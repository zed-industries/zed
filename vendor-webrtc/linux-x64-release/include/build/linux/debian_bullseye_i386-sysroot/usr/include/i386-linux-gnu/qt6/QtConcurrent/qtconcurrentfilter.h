// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTCONCURRENT_FILTER_H
#define QTCONCURRENT_FILTER_H

#include <QtConcurrent/qtconcurrent_global.h>

#if !defined(QT_NO_CONCURRENT) || defined(Q_CLANG_QDOC)

#include <QtConcurrent/qtconcurrentfilterkernel.h>
#include <QtConcurrent/qtconcurrentfunctionwrappers.h>

QT_BEGIN_NAMESPACE

namespace QtConcurrent {

//! [QtConcurrent-1]
template <typename Sequence, typename KeepFunctor, typename ReduceFunctor>
ThreadEngineStarter<void> filterInternal(QThreadPool *pool, Sequence &sequence,
                                         KeepFunctor &&keep, ReduceFunctor &&reduce)
{
    typedef FilterKernel<Sequence, std::decay_t<KeepFunctor>, std::decay_t<ReduceFunctor>>
            KernelType;
    return startThreadEngine(new KernelType(pool, sequence, std::forward<KeepFunctor>(keep),
                                            std::forward<ReduceFunctor>(reduce)));
}

// filter() on sequences
template <typename Sequence, typename KeepFunctor>
QFuture<void> filter(QThreadPool *pool, Sequence &sequence, KeepFunctor &&keep)
{
    return filterInternal(pool, sequence, std::forward<KeepFunctor>(keep),
                          QtPrivate::PushBackWrapper());
}

template <typename Sequence, typename KeepFunctor>
QFuture<void> filter(Sequence &sequence, KeepFunctor &&keep)
{
    return filterInternal(QThreadPool::globalInstance(),
                          sequence, std::forward<KeepFunctor>(keep), QtPrivate::PushBackWrapper());
}

// filteredReduced() on sequences
template <typename ResultType, typename Sequence, typename KeepFunctor, typename ReduceFunctor>
QFuture<ResultType> filteredReduced(QThreadPool *pool,
                                    Sequence &&sequence,
                                    KeepFunctor &&keep,
                                    ReduceFunctor &&reduce,
                                    ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                          | SequentialReduce))
{
    return startFilteredReduced<ResultType>(pool, std::forward<Sequence>(sequence),
                                            std::forward<KeepFunctor>(keep),
                                            std::forward<ReduceFunctor>(reduce), options);
}

template <typename ResultType, typename Sequence, typename KeepFunctor, typename ReduceFunctor>
QFuture<ResultType> filteredReduced(Sequence &&sequence,
                                    KeepFunctor &&keep,
                                    ReduceFunctor &&reduce,
                                    ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                          | SequentialReduce))
{
    return startFilteredReduced<ResultType>(
            QThreadPool::globalInstance(), std::forward<Sequence>(sequence),
            std::forward<KeepFunctor>(keep), std::forward<ReduceFunctor>(reduce), options);
}

#ifdef Q_CLANG_QDOC
template <typename ResultType, typename Sequence, typename KeepFunctor, typename ReduceFunctor,
          typename InitialValueType>
#else
template <typename ResultType, typename Sequence, typename KeepFunctor, typename ReduceFunctor,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
#endif
QFuture<ResultType> filteredReduced(QThreadPool *pool,
                                    Sequence &&sequence,
                                    KeepFunctor &&keep,
                                    ReduceFunctor &&reduce,
                                    InitialValueType &&initialValue,
                                    ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                          | SequentialReduce))
{
    return startFilteredReduced<ResultType>(
            pool, std::forward<Sequence>(sequence), std::forward<KeepFunctor>(keep),
            std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)), options);
}

#ifdef Q_CLANG_QDOC
template <typename ResultType, typename Sequence, typename KeepFunctor, typename ReduceFunctor,
          typename InitialValueType>
#else
template <typename ResultType, typename Sequence, typename KeepFunctor, typename ReduceFunctor,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
#endif
QFuture<ResultType> filteredReduced(Sequence &&sequence,
                                    KeepFunctor &&keep,
                                    ReduceFunctor &&reduce,
                                    InitialValueType &&initialValue,
                                    ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                          | SequentialReduce))
{
    return startFilteredReduced<ResultType>(
            QThreadPool::globalInstance(), std::forward<Sequence>(sequence),
            std::forward<KeepFunctor>(keep), std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)), options);
}

#ifndef Q_CLANG_QDOC
template <typename Sequence, typename KeepFunctor, typename ReduceFunctor,
          std::enable_if_t<QtPrivate::isInvocable<KeepFunctor, Sequence>::value, int> = 0,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type>
QFuture<ResultType> filteredReduced(QThreadPool *pool,
                                    Sequence &&sequence,
                                    KeepFunctor &&keep,
                                    ReduceFunctor &&reduce,
                                    ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                          | SequentialReduce))
{
    return startFilteredReduced<ResultType>(pool, std::forward<Sequence>(sequence),
                                            std::forward<KeepFunctor>(keep),
                                            std::forward<ReduceFunctor>(reduce), options);
}

template <typename Sequence, typename KeepFunctor, typename ReduceFunctor,
          std::enable_if_t<QtPrivate::isInvocable<KeepFunctor, Sequence>::value, int> = 0,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type>
QFuture<ResultType> filteredReduced(Sequence &&sequence,
                                    KeepFunctor &&keep,
                                    ReduceFunctor &&reduce,
                                    ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                          | SequentialReduce))
{
    return startFilteredReduced<ResultType>(
            QThreadPool::globalInstance(), std::forward<Sequence>(sequence),
            std::forward<KeepFunctor>(keep), std::forward<ReduceFunctor>(reduce), options);
}

template <typename Sequence, typename KeepFunctor, typename ReduceFunctor,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInvocable<KeepFunctor, Sequence>::value, int> = 0,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
QFuture<ResultType> filteredReduced(QThreadPool *pool,
                                    Sequence &&sequence,
                                    KeepFunctor &&keep,
                                    ReduceFunctor &&reduce,
                                    InitialValueType &&initialValue,
                                    ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                          | SequentialReduce))
{
    return startFilteredReduced<ResultType>(
            pool, std::forward<Sequence>(sequence), std::forward<KeepFunctor>(keep),
            std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)), options);
}

template <typename Sequence, typename KeepFunctor, typename ReduceFunctor,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInvocable<KeepFunctor, Sequence>::value, int> = 0,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
QFuture<ResultType> filteredReduced(Sequence &&sequence,
                                    KeepFunctor &&keep,
                                    ReduceFunctor &&reduce,
                                    InitialValueType &&initialValue,
                                    ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                          | SequentialReduce))
{
    return startFilteredReduced<ResultType>(
            QThreadPool::globalInstance(), std::forward<Sequence>(sequence),
            std::forward<KeepFunctor>(keep), std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)), options);
}
#endif

// filteredReduced() on iterators
template <typename ResultType, typename Iterator, typename KeepFunctor, typename ReduceFunctor>
QFuture<ResultType> filteredReduced(QThreadPool *pool,
                                    Iterator begin,
                                    Iterator end,
                                    KeepFunctor &&keep,
                                    ReduceFunctor &&reduce,
                                    ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                          | SequentialReduce))
{
    return startFilteredReduced<ResultType>(pool, begin, end, std::forward<KeepFunctor>(keep),
                                            std::forward<ReduceFunctor>(reduce), options);
}

template <typename ResultType, typename Iterator, typename KeepFunctor, typename ReduceFunctor>
QFuture<ResultType> filteredReduced(Iterator begin,
                                    Iterator end,
                                    KeepFunctor &&keep,
                                    ReduceFunctor &&reduce,
                                    ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                          | SequentialReduce))
{
    return startFilteredReduced<ResultType>(QThreadPool::globalInstance(), begin, end,
                                            std::forward<KeepFunctor>(keep),
                                            std::forward<ReduceFunctor>(reduce), options);
}

#ifdef Q_CLANG_QDOC
template <typename ResultType, typename Iterator, typename KeepFunctor, typename ReduceFunctor,
          typename InitialValueType>
#else
template <typename ResultType, typename Iterator, typename KeepFunctor, typename ReduceFunctor,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
#endif
QFuture<ResultType> filteredReduced(QThreadPool *pool,
                                    Iterator begin,
                                    Iterator end,
                                    KeepFunctor &&keep,
                                    ReduceFunctor &&reduce,
                                    InitialValueType &&initialValue,
                                    ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                          | SequentialReduce))
{
    return startFilteredReduced<ResultType>(
            pool, begin, end, std::forward<KeepFunctor>(keep), std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)), options);
}

#ifdef Q_CLANG_QDOC
template <typename ResultType, typename Iterator, typename KeepFunctor, typename ReduceFunctor,
          typename InitialValueType>
#else
template <typename ResultType, typename Iterator, typename KeepFunctor, typename ReduceFunctor,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
#endif
QFuture<ResultType> filteredReduced(Iterator begin,
                                    Iterator end,
                                    KeepFunctor &&keep,
                                    ReduceFunctor &&reduce,
                                    InitialValueType &&initialValue,
                                    ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                          | SequentialReduce))
{
    return startFilteredReduced<ResultType>(
            QThreadPool::globalInstance(), begin, end, std::forward<KeepFunctor>(keep),
            std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)), options);
}

#ifndef Q_CLANG_QDOC
template <typename Iterator, typename KeepFunctor, typename ReduceFunctor,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type>
QFuture<ResultType> filteredReduced(QThreadPool *pool,
                                    Iterator begin,
                                    Iterator end,
                                    KeepFunctor &&keep,
                                    ReduceFunctor &&reduce,
                                    ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                          | SequentialReduce))
{
   return startFilteredReduced<ResultType>(pool, begin, end, std::forward<KeepFunctor>(keep),
                                           std::forward<ReduceFunctor>(reduce), options);
}

template <typename Iterator, typename KeepFunctor, typename ReduceFunctor,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type>
QFuture<ResultType> filteredReduced(Iterator begin,
                                    Iterator end,
                                    KeepFunctor &&keep,
                                    ReduceFunctor &&reduce,
                                    ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                          | SequentialReduce))
{
   return startFilteredReduced<ResultType>(QThreadPool::globalInstance(), begin, end,
                                           std::forward<KeepFunctor>(keep),
                                           std::forward<ReduceFunctor>(reduce), options);
}

template <typename Iterator, typename KeepFunctor, typename ReduceFunctor,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
QFuture<ResultType> filteredReduced(QThreadPool *pool,
                                    Iterator begin,
                                    Iterator end,
                                    KeepFunctor &&keep,
                                    ReduceFunctor &&reduce,
                                    InitialValueType &&initialValue,
                                    ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                          | SequentialReduce))
{
    return startFilteredReduced<ResultType>(
            pool, begin, end, std::forward<KeepFunctor>(keep), std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)), options);
}

template <typename Iterator, typename KeepFunctor, typename ReduceFunctor,
          std::enable_if_t<QtPrivate::isIterator_v<Iterator>, int> = 0,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
QFuture<ResultType> filteredReduced(Iterator begin,
                                    Iterator end,
                                    KeepFunctor &&keep,
                                    ReduceFunctor &&reduce,
                                    InitialValueType &&initialValue,
                                    ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                          | SequentialReduce))
{
    return startFilteredReduced<ResultType>(
            QThreadPool::globalInstance(), begin, end, std::forward<KeepFunctor>(keep),
            std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)), options);
}
#endif

// filtered() on sequences
template <typename Sequence, typename KeepFunctor>
QFuture<typename std::decay_t<Sequence>::value_type> filtered(QThreadPool *pool,Sequence &&sequence,
                                                              KeepFunctor &&keep)
{
    return startFiltered(pool, std::forward<Sequence>(sequence), std::forward<KeepFunctor>(keep));
}

template <typename Sequence, typename KeepFunctor>
QFuture<typename std::decay_t<Sequence>::value_type> filtered(Sequence &&sequence,
                                                              KeepFunctor &&keep)
{
    return startFiltered(QThreadPool::globalInstance(), std::forward<Sequence>(sequence),
                         std::forward<KeepFunctor>(keep));
}

// filtered() on iterators
template <typename Iterator, typename KeepFunctor>
QFuture<typename qValueType<Iterator>::value_type> filtered(QThreadPool *pool,
                                                            Iterator begin,
                                                            Iterator end,
                                                            KeepFunctor &&keep)
{
    return startFiltered(pool, begin, end, std::forward<KeepFunctor>(keep));
}

template <typename Iterator, typename KeepFunctor>
QFuture<typename qValueType<Iterator>::value_type> filtered(Iterator begin,
                                                            Iterator end,
                                                            KeepFunctor &&keep)
{
    return startFiltered(QThreadPool::globalInstance(), begin, end,
                         std::forward<KeepFunctor>(keep));
}

// blocking filter() on sequences
template <typename Sequence, typename KeepFunctor>
void blockingFilter(QThreadPool *pool, Sequence &sequence, KeepFunctor &&keep)
{
    QFuture<void> future = filter(pool, sequence, std::forward<KeepFunctor>(keep));
    future.waitForFinished();
}

template <typename Sequence, typename KeepFunctor>
void blockingFilter(Sequence &sequence, KeepFunctor &&keep)
{
    QFuture<void> future = filter(sequence, std::forward<KeepFunctor>(keep));
    future.waitForFinished();
}

// blocking filteredReduced() on sequences
template <typename ResultType, typename Sequence, typename KeepFunctor, typename ReduceFunctor>
ResultType blockingFilteredReduced(QThreadPool *pool,
                                   Sequence &&sequence,
                                   KeepFunctor &&keep,
                                   ReduceFunctor &&reduce,
                                   ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                         | SequentialReduce))
{
    QFuture<ResultType> future = filteredReduced<ResultType>(
            pool, std::forward<Sequence>(sequence), std::forward<KeepFunctor>(keep),
            std::forward<ReduceFunctor>(reduce), options);
    return future.takeResult();
}

template <typename ResultType, typename Sequence, typename KeepFunctor, typename ReduceFunctor>
ResultType blockingFilteredReduced(Sequence &&sequence,
                                   KeepFunctor &&keep,
                                   ReduceFunctor &&reduce,
                                   ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                         | SequentialReduce))
{
    QFuture<ResultType> future = filteredReduced<ResultType>(
            std::forward<Sequence>(sequence), std::forward<KeepFunctor>(keep),
            std::forward<ReduceFunctor>(reduce), options);
    return future.takeResult();
}

#ifdef Q_CLANG_QDOC
template <typename ResultType, typename Sequence, typename KeepFunctor, typename ReduceFunctor,
          typename InitialValueType>
#else
template <typename ResultType, typename Sequence, typename KeepFunctor, typename ReduceFunctor,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
#endif
ResultType blockingFilteredReduced(QThreadPool *pool,
                                   Sequence &&sequence,
                                   KeepFunctor &&keep,
                                   ReduceFunctor &&reduce,
                                   InitialValueType &&initialValue,
                                   ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                         | SequentialReduce))
{
    QFuture<ResultType> future = filteredReduced<ResultType>(
            pool, std::forward<Sequence>(sequence), std::forward<KeepFunctor>(keep),
            std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)), options);
    return future.takeResult();
}

#ifdef Q_CLANG_QDOC
template <typename ResultType, typename Sequence, typename KeepFunctor, typename ReduceFunctor,
          typename InitialValueType>
#else
template <typename ResultType, typename Sequence, typename KeepFunctor, typename ReduceFunctor,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
#endif
ResultType blockingFilteredReduced(Sequence &&sequence,
                                   KeepFunctor &&keep,
                                   ReduceFunctor &&reduce,
                                   InitialValueType &&initialValue,
                                   ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                         | SequentialReduce))
{
    QFuture<ResultType> future = filteredReduced<ResultType>(
            std::forward<Sequence>(sequence), std::forward<KeepFunctor>(keep),
            std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)), options);
    return future.takeResult();
}

#ifndef Q_CLANG_QDOC
template <typename Sequence, typename KeepFunctor, typename ReduceFunctor,
          std::enable_if_t<QtPrivate::isInvocable<KeepFunctor, Sequence>::value, int> = 0,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type>
ResultType blockingFilteredReduced(QThreadPool *pool,
                                   Sequence &&sequence,
                                   KeepFunctor &&keep,
                                   ReduceFunctor &&reduce,
                                   ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                         | SequentialReduce))
{
    QFuture<ResultType> future = filteredReduced<ResultType>(
            pool, std::forward<Sequence>(sequence), std::forward<KeepFunctor>(keep),
            std::forward<ReduceFunctor>(reduce), options);
    return future.takeResult();
}

template <typename Sequence, typename KeepFunctor, typename ReduceFunctor,
          std::enable_if_t<QtPrivate::isInvocable<KeepFunctor, Sequence>::value, int> = 0,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type>
ResultType blockingFilteredReduced(Sequence &&sequence,
                                   KeepFunctor &&keep,
                                   ReduceFunctor &&reduce,
                                   ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                         | SequentialReduce))
{
    QFuture<ResultType> future = filteredReduced<ResultType>(
            std::forward<Sequence>(sequence), std::forward<KeepFunctor>(keep),
            std::forward<ReduceFunctor>(reduce), options);
    return future.takeResult();
}

template <typename Sequence, typename KeepFunctor, typename ReduceFunctor,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInvocable<KeepFunctor, Sequence>::value, int> = 0,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
ResultType blockingFilteredReduced(QThreadPool *pool,
                                   Sequence &&sequence,
                                   KeepFunctor &&keep,
                                   ReduceFunctor &&reduce,
                                   InitialValueType &&initialValue,
                                   ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                         | SequentialReduce))
{
    QFuture<ResultType> future = filteredReduced<ResultType>(
            pool, std::forward<Sequence>(sequence), std::forward<KeepFunctor>(keep),
            std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)), options);
    return future.takeResult();
}

template <typename Sequence, typename KeepFunctor, typename ReduceFunctor,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInvocable<KeepFunctor, Sequence>::value, int> = 0,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
ResultType blockingFilteredReduced(Sequence &&sequence,
                                   KeepFunctor &&keep,
                                   ReduceFunctor &&reduce,
                                   InitialValueType &&initialValue,
                                   ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                         | SequentialReduce))
{
    QFuture<ResultType> future = filteredReduced<ResultType>(
            std::forward<Sequence>(sequence), std::forward<KeepFunctor>(keep),
            std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)), options);
    return future.takeResult();
}
#endif

// blocking filteredReduced() on iterators
template <typename ResultType, typename Iterator, typename KeepFunctor, typename ReduceFunctor>
ResultType blockingFilteredReduced(QThreadPool *pool,
                                   Iterator begin,
                                   Iterator end,
                                   KeepFunctor &&keep,
                                   ReduceFunctor &&reduce,
                                   ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                         | SequentialReduce))
{
    QFuture<ResultType> future =
            filteredReduced<ResultType>(pool, begin, end, std::forward<KeepFunctor>(keep),
                                        std::forward<ReduceFunctor>(reduce), options);
    return future.takeResult();
}

template <typename ResultType, typename Iterator, typename KeepFunctor, typename ReduceFunctor>
ResultType blockingFilteredReduced(Iterator begin,
                                   Iterator end,
                                   KeepFunctor &&keep,
                                   ReduceFunctor &&reduce,
                                   ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                         | SequentialReduce))
{
    QFuture<ResultType> future =
            filteredReduced<ResultType>(begin, end, std::forward<KeepFunctor>(keep),
                                        std::forward<ReduceFunctor>(reduce), options);
    return future.takeResult();
}

#ifdef Q_CLANG_QDOC
template <typename ResultType, typename Iterator, typename KeepFunctor, typename ReduceFunctor,
          typename InitialValueType>
#else
template <typename ResultType, typename Iterator, typename KeepFunctor, typename ReduceFunctor,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
#endif
ResultType blockingFilteredReduced(QThreadPool *pool,
                                   Iterator begin,
                                   Iterator end,
                                   KeepFunctor &&keep,
                                   ReduceFunctor &&reduce,
                                   InitialValueType &&initialValue,
                                   ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                         | SequentialReduce))
{
    QFuture<ResultType> future = filteredReduced<ResultType>(
            pool, begin, end, std::forward<KeepFunctor>(keep), std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)), options);
    return future.takeResult();
}

#ifdef Q_CLANG_QDOC
template <typename ResultType, typename Iterator, typename KeepFunctor, typename ReduceFunctor,
          typename InitialValueType>
#else
template <typename ResultType, typename Iterator, typename KeepFunctor, typename ReduceFunctor,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
#endif
ResultType blockingFilteredReduced(Iterator begin,
                                   Iterator end,
                                   KeepFunctor &&keep,
                                   ReduceFunctor &&reduce,
                                   InitialValueType &&initialValue,
                                   ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                         | SequentialReduce))
{
    QFuture<ResultType> future = filteredReduced<ResultType>(
            begin, end, std::forward<KeepFunctor>(keep), std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)), options);
    return future.takeResult();
}

#ifndef Q_CLANG_QDOC
template <typename Iterator, typename KeepFunctor, typename ReduceFunctor,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type>
ResultType blockingFilteredReduced(QThreadPool *pool,
                                   Iterator begin,
                                   Iterator end,
                                   KeepFunctor &&keep,
                                   ReduceFunctor &&reduce,
                                   ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                         | SequentialReduce))
{
    QFuture<ResultType> future =
            filteredReduced<ResultType>(pool, begin, end, std::forward<KeepFunctor>(keep),
                                        std::forward<ReduceFunctor>(reduce), options);
    return future.takeResult();
}

template <typename Iterator, typename KeepFunctor, typename ReduceFunctor,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type>
ResultType blockingFilteredReduced(Iterator begin,
                                   Iterator end,
                                   KeepFunctor &&keep,
                                   ReduceFunctor &&reduce,
                                   ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                         | SequentialReduce))
{
    QFuture<ResultType> future =
            filteredReduced<ResultType>(begin, end, std::forward<KeepFunctor>(keep),
                                        std::forward<ReduceFunctor>(reduce), options);
    return future.takeResult();
}

template <typename Iterator, typename KeepFunctor, typename ReduceFunctor,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
ResultType blockingFilteredReduced(QThreadPool *pool,
                                   Iterator begin,
                                   Iterator end, KeepFunctor &&keep,
                                   ReduceFunctor &&reduce,
                                   InitialValueType &&initialValue,
                                   ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                         | SequentialReduce))
{
    QFuture<ResultType> future = filteredReduced<ResultType>(
            pool, begin, end, std::forward<KeepFunctor>(keep), std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)), options);
    return future.takeResult();
}

template <typename Iterator, typename KeepFunctor, typename ReduceFunctor,
          std::enable_if_t<QtPrivate::isIterator_v<Iterator>, int> = 0,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
ResultType blockingFilteredReduced(Iterator begin,
                                   Iterator end,
                                   KeepFunctor &&keep,
                                   ReduceFunctor &&reduce,
                                   InitialValueType &&initialValue,
                                   ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                         | SequentialReduce))
{
    QFuture<ResultType> future = filteredReduced<ResultType>(
            begin, end, std::forward<KeepFunctor>(keep), std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)), options);
    return future.takeResult();
}
#endif

// blocking filtered() on sequences
template <typename Sequence, typename KeepFunctor>
std::decay_t<Sequence> blockingFiltered(QThreadPool *pool, Sequence &&sequence, KeepFunctor &&keep)
{
    return blockingFilteredReduced<std::decay_t<Sequence>>(
            pool, std::forward<Sequence>(sequence), std::forward<KeepFunctor>(keep),
            QtPrivate::PushBackWrapper(), OrderedReduce);
}

template <typename Sequence, typename KeepFunctor>
std::decay_t<Sequence> blockingFiltered(Sequence &&sequence, KeepFunctor &&keep)
{
    return blockingFilteredReduced<std::decay_t<Sequence>>(
            QThreadPool::globalInstance(), std::forward<Sequence>(sequence),
            std::forward<KeepFunctor>(keep), QtPrivate::PushBackWrapper(), OrderedReduce);
}

// blocking filtered() on iterators
template <typename OutputSequence, typename Iterator, typename KeepFunctor>
OutputSequence blockingFiltered(QThreadPool *pool, Iterator begin, Iterator end, KeepFunctor &&keep)
{
    return blockingFilteredReduced<OutputSequence>(pool, begin, end,
                                                   std::forward<KeepFunctor>(keep),
                                                   QtPrivate::PushBackWrapper(), OrderedReduce);
}

template <typename OutputSequence, typename Iterator, typename KeepFunctor>
OutputSequence blockingFiltered(Iterator begin, Iterator end, KeepFunctor &&keep)
{
    return blockingFilteredReduced<OutputSequence>(QThreadPool::globalInstance(), begin, end,
                                                   std::forward<KeepFunctor>(keep),
                                                   QtPrivate::PushBackWrapper(), OrderedReduce);
}

} // namespace QtConcurrent

QT_END_NAMESPACE

#endif // QT_NO_CONCURRENT

#endif
