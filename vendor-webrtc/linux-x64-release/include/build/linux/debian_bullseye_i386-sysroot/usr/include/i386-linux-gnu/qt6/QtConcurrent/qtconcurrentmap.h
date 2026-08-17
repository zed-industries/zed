// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTCONCURRENT_MAP_H
#define QTCONCURRENT_MAP_H

#include <QtConcurrent/qtconcurrent_global.h>

#if !defined(QT_NO_CONCURRENT) || defined(Q_CLANG_QDOC)

#include <QtConcurrent/qtconcurrentmapkernel.h>
#include <QtConcurrent/qtconcurrentreducekernel.h>
#include <QtConcurrent/qtconcurrentfunctionwrappers.h>

QT_BEGIN_NAMESPACE



namespace QtConcurrent {

// map() on sequences
template <typename Sequence, typename MapFunctor>
QFuture<void> map(QThreadPool *pool, Sequence &&sequence, MapFunctor &&map)
{
    return startMap(pool, sequence.begin(), sequence.end(), std::forward<MapFunctor>(map));
}

template <typename Sequence, typename MapFunctor>
QFuture<void> map(Sequence &&sequence, MapFunctor &&map)
{
    return startMap(QThreadPool::globalInstance(), sequence.begin(), sequence.end(),
                    std::forward<MapFunctor>(map));
}

// map() on iterators
template <typename Iterator, typename MapFunctor>
QFuture<void> map(QThreadPool *pool, Iterator begin, Iterator end, MapFunctor &&map)
{
    return startMap(pool, begin, end, std::forward<MapFunctor>(map));
}

template <typename Iterator, typename MapFunctor>
QFuture<void> map(Iterator begin, Iterator end, MapFunctor &&map)
{
    return startMap(QThreadPool::globalInstance(), begin, end, std::forward<MapFunctor>(map));
}

// mappedReduced() for sequences.
template <typename ResultType, typename Sequence, typename MapFunctor, typename ReduceFunctor>
QFuture<ResultType> mappedReduced(QThreadPool *pool,
                                  Sequence &&sequence,
                                  MapFunctor &&map,
                                  ReduceFunctor &&reduce,
                                  ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                        | SequentialReduce))
{
    return startMappedReduced<QtPrivate::MapResultType<Sequence, MapFunctor>, ResultType>
        (pool, std::forward<Sequence>(sequence), std::forward<MapFunctor>(map),
         std::forward<ReduceFunctor>(reduce), options);
}

template <typename ResultType, typename Sequence, typename MapFunctor, typename ReduceFunctor>
QFuture<ResultType> mappedReduced(Sequence &&sequence,
                                  MapFunctor &&map,
                                  ReduceFunctor &&reduce,
                                  ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                        | SequentialReduce))
{
    return startMappedReduced<QtPrivate::MapResultType<Sequence, MapFunctor>, ResultType>
        (QThreadPool::globalInstance(), std::forward<Sequence>(sequence),
         std::forward<MapFunctor>(map), std::forward<ReduceFunctor>(reduce), options);
}

#ifdef Q_CLANG_QDOC
template <typename ResultType, typename Sequence, typename MapFunctor, typename ReduceFunctor,
          typename InitialValueType>
#else
template <typename ResultType, typename Sequence, typename MapFunctor, typename ReduceFunctor,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
#endif
QFuture<ResultType> mappedReduced(QThreadPool *pool,
                                  Sequence &&sequence,
                                  MapFunctor &&map,
                                  ReduceFunctor &&reduce,
                                  InitialValueType &&initialValue,
                                  ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                        | SequentialReduce))
{
    return startMappedReduced<QtPrivate::MapResultType<Sequence, MapFunctor>, ResultType>(
            pool, std::forward<Sequence>(sequence), std::forward<MapFunctor>(map),
            std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)), options);
}
#ifdef Q_CLANG_QDOC
template <typename ResultType, typename Sequence, typename MapFunctor, typename ReduceFunctor,
          typename InitialValueType>
#else
template <typename ResultType, typename Sequence, typename MapFunctor, typename ReduceFunctor,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
#endif
QFuture<ResultType> mappedReduced(Sequence &&sequence,
                                  MapFunctor &&map,
                                  ReduceFunctor &&reduce,
                                  InitialValueType &&initialValue,
                                  ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                        | SequentialReduce))
{
    return startMappedReduced<QtPrivate::MapResultType<Sequence, MapFunctor>, ResultType>
        (QThreadPool::globalInstance(), std::forward<Sequence>(sequence),
         std::forward<MapFunctor>(map), std::forward<ReduceFunctor>(reduce),
         ResultType(std::forward<InitialValueType>(initialValue)),  options);
}

template <typename Sequence, typename MapFunctor, typename ReduceFunctor,
          std::enable_if_t<QtPrivate::isInvocable<MapFunctor, Sequence>::value, int> = 0,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type>
QFuture<ResultType> mappedReduced(QThreadPool *pool,
                                  Sequence &&sequence,
                                  MapFunctor &&map,
                                  ReduceFunctor &&reduce,
                                  ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                        | SequentialReduce))
{
    return startMappedReduced<QtPrivate::MapResultType<Sequence, MapFunctor>, ResultType>
        (pool, std::forward<Sequence>(sequence), std::forward<MapFunctor>(map),
         std::forward<ReduceFunctor>(reduce), options);
}

template <typename Sequence, typename MapFunctor, typename ReduceFunctor,
          std::enable_if_t<QtPrivate::isInvocable<MapFunctor, Sequence>::value, int> = 0,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type>
QFuture<ResultType> mappedReduced(Sequence &&sequence,
                                  MapFunctor &&map,
                                  ReduceFunctor &&reduce,
                                  ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                        | SequentialReduce))
{
    return startMappedReduced<QtPrivate::MapResultType<Sequence, MapFunctor>, ResultType>
        (QThreadPool::globalInstance(), std::forward<Sequence>(sequence),
         std::forward<MapFunctor>(map), std::forward<ReduceFunctor>(reduce), options);
}

#ifdef Q_CLANG_QDOC
template <typename Sequence, typename MapFunctor, typename ReduceFunctor, typename ResultType,
          typename InitialValueType>
#else
template <typename Sequence, typename MapFunctor, typename ReduceFunctor, typename InitialValueType,
          std::enable_if_t<QtPrivate::isInvocable<MapFunctor, Sequence>::value, int> = 0,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
#endif
QFuture<ResultType> mappedReduced(QThreadPool *pool,
                                  Sequence &&sequence,
                                  MapFunctor &&map,
                                  ReduceFunctor &&reduce,
                                  InitialValueType &&initialValue,
                                  ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                        | SequentialReduce))
{
    return startMappedReduced<QtPrivate::MapResultType<Sequence, MapFunctor>, ResultType>(
            pool, std::forward<Sequence>(sequence), std::forward<MapFunctor>(map),
            std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)), options);
}

#ifdef Q_CLANG_QDOC
template <typename Sequence, typename MapFunctor, typename ReduceFunctor, typename ResultType,
          typename InitialValueType>
#else
template <typename Sequence, typename MapFunctor, typename ReduceFunctor, typename InitialValueType,
          std::enable_if_t<QtPrivate::isInvocable<MapFunctor, Sequence>::value, int> = 0,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
#endif
QFuture<ResultType> mappedReduced(Sequence &&sequence,
                                  MapFunctor &&map,
                                  ReduceFunctor &&reduce,
                                  InitialValueType &&initialValue,
                                  ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                        | SequentialReduce))
{
    return startMappedReduced<QtPrivate::MapResultType<Sequence, MapFunctor>, ResultType>
        (QThreadPool::globalInstance(), std::forward<Sequence>(sequence),
         std::forward<MapFunctor>(map), std::forward<ReduceFunctor>(reduce),
         ResultType(std::forward<InitialValueType>(initialValue)), options);
}

// mappedReduced() for iterators
template <typename ResultType, typename Iterator, typename MapFunctor, typename ReduceFunctor>
QFuture<ResultType> mappedReduced(QThreadPool *pool,
                                  Iterator begin,
                                  Iterator end,
                                  MapFunctor &&map,
                                  ReduceFunctor &&reduce,
                                  ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                        | SequentialReduce))
{
    return startMappedReduced<QtPrivate::MapResultType<Iterator, MapFunctor>, ResultType>
        (pool, begin, end, std::forward<MapFunctor>(map), std::forward<ReduceFunctor>(reduce),
         options);
}

template <typename ResultType, typename Iterator, typename MapFunctor, typename ReduceFunctor>
QFuture<ResultType> mappedReduced(Iterator begin,
                                  Iterator end,
                                  MapFunctor &&map,
                                  ReduceFunctor &&reduce,
                                  ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                        | SequentialReduce))
{
    return startMappedReduced<QtPrivate::MapResultType<Iterator, MapFunctor>, ResultType>
        (QThreadPool::globalInstance(), begin, end, std::forward<MapFunctor>(map),
         std::forward<ReduceFunctor>(reduce), options);
}

#ifdef Q_CLANG_QDOC
template <typename ResultType, typename Iterator, typename MapFunctor, typename ReduceFunctor,
          typename InitialValueType>
#else
template <typename ResultType, typename Iterator, typename MapFunctor, typename ReduceFunctor,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
#endif
QFuture<ResultType> mappedReduced(QThreadPool *pool,
                                  Iterator begin,
                                  Iterator end,
                                  MapFunctor &&map,
                                  ReduceFunctor &&reduce,
                                  InitialValueType &&initialValue,
                                  ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                        | SequentialReduce))
{
    return startMappedReduced<QtPrivate::MapResultType<Iterator, MapFunctor>, ResultType>
        (pool, begin, end, std::forward<MapFunctor>(map), std::forward<ReduceFunctor>(reduce),
         ResultType(std::forward<InitialValueType>(initialValue)), options);
}

#ifdef Q_CLANG_QDOC
template <typename ResultType, typename Iterator, typename MapFunctor, typename ReduceFunctor,
          typename InitialValueType>
#else
template <typename ResultType, typename Iterator, typename MapFunctor, typename ReduceFunctor,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
#endif
QFuture<ResultType> mappedReduced(Iterator begin,
                                  Iterator end,
                                  MapFunctor &&map,
                                  ReduceFunctor &&reduce,
                                  InitialValueType &&initialValue,
                                  ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                        | SequentialReduce))
{
    return startMappedReduced<QtPrivate::MapResultType<Iterator, MapFunctor>, ResultType>
        (QThreadPool::globalInstance(), begin, end, std::forward<MapFunctor>(map),
         std::forward<ReduceFunctor>(reduce),
         ResultType(std::forward<InitialValueType>(initialValue)), options);
}

template <typename Iterator, typename MapFunctor, typename ReduceFunctor,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type>
QFuture<ResultType> mappedReduced(QThreadPool *pool,
                                  Iterator begin,
                                  Iterator end,
                                  MapFunctor &&map,
                                  ReduceFunctor &&reduce,
                                  ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                        | SequentialReduce))
{
    return startMappedReduced<QtPrivate::MapResultType<Iterator, MapFunctor>, ResultType>(
            pool, begin, end, std::forward<MapFunctor>(map), std::forward<ReduceFunctor>(reduce),
            options);
}

template <typename Iterator, typename MapFunctor, typename ReduceFunctor,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type>
QFuture<ResultType> mappedReduced(Iterator begin,
                                  Iterator end,
                                  MapFunctor &&map,
                                  ReduceFunctor &&reduce,
                                  ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                        | SequentialReduce))
{
    return startMappedReduced<QtPrivate::MapResultType<Iterator, MapFunctor>, ResultType>
        (QThreadPool::globalInstance(), begin, end, std::forward<MapFunctor>(map),
         std::forward<ReduceFunctor>(reduce), options);
}

#ifdef Q_CLANG_QDOC
template <typename Iterator, typename MapFunctor, typename ReduceFunctor, typename ResultType,
          typename InitialValueType>
#else
template <typename Iterator, typename MapFunctor, typename ReduceFunctor,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
#endif
QFuture<ResultType> mappedReduced(QThreadPool *pool,
                                  Iterator begin,
                                  Iterator end,
                                  MapFunctor &&map,
                                  ReduceFunctor &&reduce,
                                  InitialValueType &&initialValue,
                                  ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                        | SequentialReduce))
{
    return startMappedReduced<QtPrivate::MapResultType<Iterator, MapFunctor>, ResultType>
        (pool, begin, end, std::forward<MapFunctor>(map), std::forward<ReduceFunctor>(reduce),
         ResultType(std::forward<InitialValueType>(initialValue)), options);
}

#ifdef Q_CLANG_QDOC
template <typename Iterator, typename MapFunctor, typename ReduceFunctor, typename ResultType,
          typename InitialValueType>
#else
template<typename Iterator, typename MapFunctor, typename ReduceFunctor,
         std::enable_if_t<QtPrivate::isIterator_v<Iterator>, int> = 0,
         typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type,
         typename InitialValueType,
         std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                          int> = 0>
#endif
QFuture<ResultType> mappedReduced(Iterator begin,
                                  Iterator end,
                                  MapFunctor &&map,
                                  ReduceFunctor &&reduce,
                                  InitialValueType &&initialValue,
                                  ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                        | SequentialReduce))
{
    return startMappedReduced<QtPrivate::MapResultType<Iterator, MapFunctor>, ResultType>
        (QThreadPool::globalInstance(), begin, end, std::forward<MapFunctor>(map),
         std::forward<ReduceFunctor>(reduce),
         ResultType(std::forward<InitialValueType>(initialValue)), options);
}

// mapped() for sequences
template <typename Sequence, typename MapFunctor>
QFuture<QtPrivate::MapResultType<Sequence, MapFunctor>> mapped(
                                  QThreadPool *pool,
                                  Sequence &&sequence,
                                  MapFunctor &&map)
{
    return startMapped<QtPrivate::MapResultType<Sequence, MapFunctor>>(
            pool, std::forward<Sequence>(sequence), std::forward<MapFunctor>(map));
}

template <typename Sequence, typename MapFunctor>
QFuture<QtPrivate::MapResultType<Sequence, MapFunctor>> mapped(
                                  Sequence &&sequence,
                                  MapFunctor &&map)
{
    return startMapped<QtPrivate::MapResultType<Sequence, MapFunctor>>
        (QThreadPool::globalInstance(), std::forward<Sequence>(sequence),
         std::forward<MapFunctor>(map));
}

// mapped() for iterator ranges.
template <typename Iterator, typename MapFunctor>
QFuture<QtPrivate::MapResultType<Iterator, MapFunctor>> mapped(
                                  QThreadPool *pool,
                                  Iterator begin,
                                  Iterator end,
                                  MapFunctor &&map)
{
    return startMapped<QtPrivate::MapResultType<Iterator, MapFunctor>>(
            pool, begin, end, std::forward<MapFunctor>(map));
}

template <typename Iterator, typename MapFunctor>
QFuture<QtPrivate::MapResultType<Iterator, MapFunctor>> mapped(
                                  Iterator begin,
                                  Iterator end,
                                  MapFunctor &&map)
{
    return startMapped<QtPrivate::MapResultType<Iterator, MapFunctor>>
        (QThreadPool::globalInstance(), begin, end, std::forward<MapFunctor>(map));
}

// blockingMap() for sequences
template <typename Sequence, typename MapFunctor>
void blockingMap(QThreadPool *pool, Sequence &&sequence, MapFunctor map)
{
    QFuture<void> future =
            startMap(pool, sequence.begin(), sequence.end(), std::forward<MapFunctor>(map));
    future.waitForFinished();
}

template <typename Sequence, typename MapFunctor>
void blockingMap(Sequence &&sequence, MapFunctor &&map)
{
    QFuture<void> future = startMap(QThreadPool::globalInstance(), sequence.begin(), sequence.end(),
                                    std::forward<MapFunctor>(map));
    future.waitForFinished();
}

// blockingMap() for iterator ranges
template <typename Iterator, typename MapFunctor>
void blockingMap(QThreadPool *pool, Iterator begin, Iterator end, MapFunctor &&map)
{
    QFuture<void> future = startMap(pool, begin, end, map);
    future.waitForFinished();
}

template <typename Iterator, typename MapFunctor>
void blockingMap(Iterator begin, Iterator end, MapFunctor &&map)
{
    QFuture<void> future = startMap(QThreadPool::globalInstance(), begin, end,
                                    std::forward<MapFunctor>(map));
    future.waitForFinished();
}

// blockingMappedReduced() for sequences
template <typename ResultType, typename Sequence, typename MapFunctor, typename ReduceFunctor>
ResultType blockingMappedReduced(QThreadPool *pool,
                                 Sequence &&sequence,
                                 MapFunctor &&map,
                                 ReduceFunctor &&reduce,
                                 ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                       | SequentialReduce))
{
    QFuture<ResultType> future =
            mappedReduced<ResultType>(pool, std::forward<Sequence>(sequence),
                                      std::forward<MapFunctor>(map),
                                      std::forward<ReduceFunctor>(reduce), options);
    return future.takeResult();
}

template <typename ResultType, typename Sequence, typename MapFunctor, typename ReduceFunctor>
ResultType blockingMappedReduced(Sequence &&sequence,
                                 MapFunctor &&map,
                                 ReduceFunctor &&reduce,
                                 ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                       | SequentialReduce))
{
    QFuture<ResultType> future =
            mappedReduced<ResultType>(std::forward<Sequence>(sequence),
                                      std::forward<MapFunctor>(map),
                                      std::forward<ReduceFunctor>(reduce), options);
    return future.takeResult();
}

#ifdef Q_CLANG_QDOC
template <typename ResultType, typename Sequence, typename MapFunctor, typename ReduceFunctor,
          typename InitialValueType>
#else
template <typename ResultType, typename Sequence, typename MapFunctor, typename ReduceFunctor,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
#endif
ResultType blockingMappedReduced(QThreadPool *pool,
                                 Sequence &&sequence,
                                 MapFunctor &&map,
                                 ReduceFunctor &&reduce,
                                 InitialValueType &&initialValue,
                                 ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                       | SequentialReduce))
{
    QFuture<ResultType> future = mappedReduced<ResultType>(
            pool, std::forward<Sequence>(sequence), std::forward<MapFunctor>(map),
            std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)), options);
    return future.takeResult();
}

#ifdef Q_CLANG_QDOC
template <typename ResultType, typename Sequence, typename MapFunctor, typename ReduceFunctor,
          typename InitialValueType>
#else
template <typename ResultType, typename Sequence, typename MapFunctor, typename ReduceFunctor,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
#endif
ResultType blockingMappedReduced(Sequence &&sequence,
                                 MapFunctor &&map,
                                 ReduceFunctor &&reduce,
                                 InitialValueType &&initialValue,
                                 ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                       | SequentialReduce))
{
    QFuture<ResultType> future = mappedReduced<ResultType>(
            std::forward<Sequence>(sequence), std::forward<MapFunctor>(map),
                std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)), options);
    return future.takeResult();
}

template <typename MapFunctor, typename ReduceFunctor, typename Sequence,
          std::enable_if_t<QtPrivate::isInvocable<MapFunctor, Sequence>::value, int> = 0,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type>
ResultType blockingMappedReduced(QThreadPool *pool,
                                 Sequence &&sequence,
                                 MapFunctor &&map,
                                 ReduceFunctor &&reduce,
                                 ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                       | SequentialReduce))
{
    QFuture<ResultType> future =
            mappedReduced<ResultType>(pool, std::forward<Sequence>(sequence),
                                      std::forward<MapFunctor>(map),
                                      std::forward<ReduceFunctor>(reduce), options);
    return future.takeResult();
}

template <typename MapFunctor, typename ReduceFunctor, typename Sequence,
          std::enable_if_t<QtPrivate::isInvocable<MapFunctor, Sequence>::value, int> = 0,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type>
ResultType blockingMappedReduced(Sequence &&sequence,
                                 MapFunctor &&map,
                                 ReduceFunctor &&reduce,
                                 ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                       | SequentialReduce))
{
    QFuture<ResultType> future =
            mappedReduced<ResultType>(std::forward<Sequence>(sequence),
                                      std::forward<MapFunctor>(map),
                                      std::forward<ReduceFunctor>(reduce), options);
    return future.takeResult();
}

#ifdef Q_CLANG_QDOC
template <typename MapFunctor, typename ReduceFunctor, typename Sequence, typename ResultType,
          typename InitialValueType>
#else
template <typename MapFunctor, typename ReduceFunctor, typename Sequence, typename InitialValueType,
          std::enable_if_t<QtPrivate::isInvocable<MapFunctor, Sequence>::value, int> = 0,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
#endif
ResultType blockingMappedReduced(QThreadPool *pool,
                                 Sequence &&sequence,
                                 MapFunctor &&map,
                                 ReduceFunctor &&reduce,
                                 InitialValueType &&initialValue,
                                 ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                       | SequentialReduce))
{
    QFuture<ResultType> future = mappedReduced<ResultType>(
            pool, std::forward<Sequence>(sequence), std::forward<MapFunctor>(map),
            std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)), options);
    return future.takeResult();
}

#ifdef Q_CLANG_QDOC
template <typename MapFunctor, typename ReduceFunctor, typename Sequence, typename ResultType,
          typename InitialValueType>
#else
template<typename MapFunctor, typename ReduceFunctor, typename Sequence, typename InitialValueType,
         std::enable_if_t<QtPrivate::isInvocable<MapFunctor, Sequence>::value, int> = 0,
         typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type,
         std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                          int> = 0>
#endif
ResultType blockingMappedReduced(Sequence &&sequence,
                                 MapFunctor &&map,
                                 ReduceFunctor &&reduce,
                                 InitialValueType &&initialValue,
                                 ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                       | SequentialReduce))
{
    QFuture<ResultType> future = mappedReduced<ResultType>(
            std::forward<Sequence>(sequence), std::forward<MapFunctor>(map),
            std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)), options);
    return future.takeResult();
}

// blockingMappedReduced() for iterator ranges
template <typename ResultType, typename Iterator, typename MapFunctor, typename ReduceFunctor>
ResultType blockingMappedReduced(QThreadPool *pool,
                                 Iterator begin,
                                 Iterator end,
                                 MapFunctor &&map,
                                 ReduceFunctor &&reduce,
                                 ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                       | SequentialReduce))
{
    QFuture<ResultType> future =
            mappedReduced<ResultType>(pool, begin, end, std::forward<MapFunctor>(map),
                                      std::forward<ReduceFunctor>(reduce), options);
    return future.takeResult();
}

template <typename ResultType, typename Iterator, typename MapFunctor, typename ReduceFunctor>
ResultType blockingMappedReduced(Iterator begin,
                                 Iterator end,
                                 MapFunctor &&map,
                                 ReduceFunctor &&reduce,
                                 ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                       | SequentialReduce))
{
    QFuture<ResultType> future =
            mappedReduced<ResultType>(begin, end, std::forward<MapFunctor>(map),
                                      std::forward<ReduceFunctor>(reduce), options);
    return future.takeResult();
}

#ifdef Q_CLANG_QDOC
template <typename ResultType, typename Iterator, typename MapFunctor, typename ReduceFunctor,
          typename InitialValueType>
#else
template <typename ResultType, typename Iterator, typename MapFunctor, typename ReduceFunctor,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
#endif
ResultType blockingMappedReduced(QThreadPool *pool,
                                 Iterator begin,
                                 Iterator end,
                                 MapFunctor &&map,
                                 ReduceFunctor &&reduce,
                                 InitialValueType &&initialValue,
                                 ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                       | SequentialReduce))
{
    QFuture<ResultType> future = mappedReduced<ResultType>(
            pool, begin, end, std::forward<MapFunctor>(map), std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)),
            options);
    return future.takeResult();
}

#ifdef Q_CLANG_QDOC
template <typename ResultType, typename Iterator, typename MapFunctor, typename ReduceFunctor,
          typename InitialValueType>
#else
template <typename ResultType, typename Iterator, typename MapFunctor, typename ReduceFunctor,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
#endif
ResultType blockingMappedReduced(Iterator begin,
                                 Iterator end,
                                 MapFunctor &&map,
                                 ReduceFunctor &&reduce,
                                 InitialValueType &&initialValue,
                                 ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                       | SequentialReduce))
{
    QFuture<ResultType> future = mappedReduced<ResultType>(
            begin, end, std::forward<MapFunctor>(map), std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)),
            options);
    return future.takeResult();
}

template <typename Iterator, typename MapFunctor, typename ReduceFunctor,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type>
ResultType blockingMappedReduced(QThreadPool *pool,
                                 Iterator begin,
                                 Iterator end,
                                 MapFunctor &&map,
                                 ReduceFunctor &&reduce,
                                 ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                       | SequentialReduce))
{
    QFuture<ResultType> future =
            mappedReduced<ResultType>(pool, begin, end, std::forward<MapFunctor>(map),
                                      std::forward<ReduceFunctor>(reduce), options);
    return future.takeResult();
}

template <typename Iterator, typename MapFunctor, typename ReduceFunctor,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type>
ResultType blockingMappedReduced(Iterator begin,
                                 Iterator end,
                                 MapFunctor &&map,
                                 ReduceFunctor &&reduce,
                                 ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                       | SequentialReduce))
{
    QFuture<ResultType> future =
            mappedReduced<ResultType>(begin, end, std::forward<MapFunctor>(map),
                                      std::forward<ReduceFunctor>(reduce), options);
    return future.takeResult();
}

#ifdef Q_CLANG_QDOC
template <typename Iterator, typename MapFunctor, typename ReduceFunctor, typename ResultType,
          typename InitialValueType>
#else
template <typename Iterator, typename MapFunctor, typename ReduceFunctor,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
#endif
ResultType blockingMappedReduced(QThreadPool *pool,
                                 Iterator begin,
                                 Iterator end,
                                 MapFunctor &&map,
                                 ReduceFunctor &&reduce,
                                 InitialValueType &&initialValue,
                                 ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                       | SequentialReduce))
{
    QFuture<ResultType> future = mappedReduced<ResultType>(
            pool, begin, end, std::forward<MapFunctor>(map), std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)), options);
    return future.takeResult();
}

#ifdef Q_CLANG_QDOC
template <typename Iterator, typename MapFunctor, typename ReduceFunctor, typename ResultType,
          typename InitialValueType>
#else
template <typename Iterator, typename MapFunctor, typename ReduceFunctor,
          std::enable_if_t<QtPrivate::isIterator_v<Iterator>, int> = 0,
          typename ResultType = typename QtPrivate::ReduceResultTypeHelper<ReduceFunctor>::type,
          typename InitialValueType,
          std::enable_if_t<QtPrivate::isInitialValueCompatible_v<InitialValueType, ResultType>,
                           int> = 0>
#endif
ResultType blockingMappedReduced(Iterator begin,
                                 Iterator end,
                                 MapFunctor &&map,
                                 ReduceFunctor &&reduce,
                                 InitialValueType &&initialValue,
                                 ReduceOptions options = ReduceOptions(UnorderedReduce
                                                                       | SequentialReduce))
{
    QFuture<ResultType> future = mappedReduced<ResultType>(
            begin, end, std::forward<MapFunctor>(map), std::forward<ReduceFunctor>(reduce),
            ResultType(std::forward<InitialValueType>(initialValue)), options);
    return future.takeResult();
}

// mapped() for sequences with a different putput sequence type.
template <typename OutputSequence, typename InputSequence, typename MapFunctor>
OutputSequence blockingMapped(QThreadPool *pool, InputSequence &&sequence, MapFunctor &&map)
{
    return blockingMappedReduced<OutputSequence>(pool, std::forward<InputSequence>(sequence),
                                                 std::forward<MapFunctor>(map),
                                                 QtPrivate::PushBackWrapper(), OrderedReduce);
}

template <typename OutputSequence, typename InputSequence, typename MapFunctor>
OutputSequence blockingMapped(InputSequence &&sequence, MapFunctor &&map)
{
    return blockingMappedReduced<OutputSequence>(
            QThreadPool::globalInstance(), std::forward<InputSequence>(sequence),
            std::forward<MapFunctor>(map), QtPrivate::PushBackWrapper(), OrderedReduce);
}

template <typename MapFunctor, typename InputSequence>
auto blockingMapped(QThreadPool *pool, InputSequence &&sequence, MapFunctor &&map)
{
    using OutputSequence = typename QtPrivate::MapSequenceResultType<std::decay_t<InputSequence>,
                                                                     MapFunctor>::ResultType;
    return blockingMappedReduced<OutputSequence>(pool, std::forward<InputSequence>(sequence),
                                                 std::forward<MapFunctor>(map),
                                                 QtPrivate::PushBackWrapper(), OrderedReduce);
}

template <typename MapFunctor, typename InputSequence>
auto blockingMapped(InputSequence &&sequence, MapFunctor &&map)
{
    using OutputSequence = typename QtPrivate::MapSequenceResultType<std::decay_t<InputSequence>,
                                                                     MapFunctor>::ResultType;
    return blockingMappedReduced<OutputSequence>(QThreadPool::globalInstance(),
                                                 std::forward<InputSequence>(sequence),
                                                 std::forward<MapFunctor>(map),
                                                 QtPrivate::PushBackWrapper(), OrderedReduce);
}

// mapped()  for iterator ranges
template <typename Sequence, typename Iterator, typename MapFunctor>
Sequence blockingMapped(QThreadPool *pool, Iterator begin, Iterator end, MapFunctor &&map)
{
    return blockingMappedReduced<Sequence>(pool, begin, end, std::forward<MapFunctor>(map),
        QtPrivate::PushBackWrapper(), OrderedReduce);
}

template <typename Sequence, typename Iterator, typename MapFunctor>
Sequence blockingMapped(Iterator begin, Iterator end, MapFunctor &&map)
{
    return blockingMappedReduced<Sequence>(QThreadPool::globalInstance(), begin, end,
                                           std::forward<MapFunctor>(map),
                                           QtPrivate::PushBackWrapper(), OrderedReduce);
}

template <typename Iterator, typename MapFunctor>
auto blockingMapped(QThreadPool *pool, Iterator begin, Iterator end, MapFunctor &&map)
{
    using OutputSequence = QtPrivate::MapResultType<Iterator, MapFunctor>;
    return blockingMappedReduced<OutputSequence>(pool, begin, end, std::forward<MapFunctor>(map),
        QtPrivate::PushBackWrapper(), OrderedReduce);
}

template <typename Iterator, typename MapFunctor>
auto blockingMapped(Iterator begin, Iterator end, MapFunctor &&map)
{
    using OutputSequence = QtPrivate::MapResultType<Iterator, MapFunctor>;
    return blockingMappedReduced<OutputSequence>(QThreadPool::globalInstance(), begin, end,
                                                 std::forward<MapFunctor>(map),
                                                 QtPrivate::PushBackWrapper(), OrderedReduce);
}

} // namespace QtConcurrent


QT_END_NAMESPACE

#endif // QT_NO_CONCURRENT

#endif
