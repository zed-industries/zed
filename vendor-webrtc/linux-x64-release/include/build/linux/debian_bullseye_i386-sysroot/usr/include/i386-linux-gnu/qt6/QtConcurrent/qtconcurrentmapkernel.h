// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTCONCURRENT_MAPKERNEL_H
#define QTCONCURRENT_MAPKERNEL_H

#include <QtConcurrent/qtconcurrent_global.h>

#if !defined(QT_NO_CONCURRENT) || defined (Q_CLANG_QDOC)

#include <QtConcurrent/qtconcurrentiteratekernel.h>
#include <QtConcurrent/qtconcurrentreducekernel.h>
#include <QtConcurrent/qtconcurrentfunctionwrappers.h>

QT_BEGIN_NAMESPACE


namespace QtConcurrent {

// map kernel, works with both parallel-for and parallel-while
template <typename Iterator, typename MapFunctor>
class MapKernel : public IterateKernel<Iterator, void>
{
    MapFunctor map;
public:
    typedef void ReturnType;
    template <typename F = MapFunctor>
    MapKernel(QThreadPool *pool, Iterator begin, Iterator end, F &&_map)
        : IterateKernel<Iterator, void>(pool, begin, end), map(std::forward<F>(_map))
    { }

    bool runIteration(Iterator it, int, void *) override
    {
        std::invoke(map, *it);
        return false;
    }

    bool runIterations(Iterator sequenceBeginIterator, int beginIndex, int endIndex, void *) override
    {
        Iterator it = sequenceBeginIterator;
        std::advance(it, beginIndex);
        for (int i = beginIndex; i < endIndex; ++i) {
            runIteration(it, i, nullptr);
            std::advance(it, 1);
        }

        return false;
    }
};

template <typename ReducedResultType,
          typename Iterator,
          typename MapFunctor,
          typename ReduceFunctor,
          typename Reducer = ReduceKernel<ReduceFunctor,
                                          ReducedResultType,
                                          QtPrivate::MapResultType<Iterator, MapFunctor>>>
class MappedReducedKernel : public IterateKernel<Iterator, ReducedResultType>
{
    ReducedResultType &reducedResult;
    MapFunctor map;
    ReduceFunctor reduce;
    Reducer reducer;
    using IntermediateResultsType = QtPrivate::MapResultType<Iterator, MapFunctor>;

public:
    typedef ReducedResultType ReturnType;

    template<typename F1 = MapFunctor, typename F2 = ReduceFunctor>
    MappedReducedKernel(QThreadPool *pool, Iterator begin, Iterator end, F1 &&_map, F2 &&_reduce,
                        ReduceOptions reduceOptions)
        : IterateKernel<Iterator, ReducedResultType>(pool, begin, end),
          reducedResult(this->defaultValue.value),
          map(std::forward<F1>(_map)),
          reduce(std::forward<F2>(_reduce)),
          reducer(pool, reduceOptions)
    { }

    template<typename F1 = MapFunctor, typename F2 = ReduceFunctor>
    MappedReducedKernel(QThreadPool *pool, Iterator begin, Iterator end, F1 &&_map, F2 &&_reduce,
                        ReducedResultType &&initialValue, ReduceOptions reduceOptions)
        : IterateKernel<Iterator, ReducedResultType>(pool, begin, end,
                                                     std::forward<ReducedResultType>(initialValue)),
          reducedResult(this->defaultValue.value),
          map(std::forward<F1>(_map)),
          reduce(std::forward<F2>(_reduce)),
          reducer(pool, reduceOptions)
    {
    }

    bool runIteration(Iterator it, int index, ReducedResultType *) override
    {
        IntermediateResults<IntermediateResultsType> results;
        results.begin = index;
        results.end = index + 1;

        results.vector.append(std::invoke(map, *it));
        reducer.runReduce(reduce, reducedResult, results);
        return false;
    }

    bool runIterations(Iterator sequenceBeginIterator, int beginIndex, int endIndex, ReducedResultType *) override
    {
        IntermediateResults<IntermediateResultsType> results;
        results.begin = beginIndex;
        results.end = endIndex;
        results.vector.reserve(endIndex - beginIndex);

        Iterator it = sequenceBeginIterator;
        std::advance(it, beginIndex);
        for (int i = beginIndex; i < endIndex; ++i) {
            results.vector.append(std::invoke(map, *it));
            std::advance(it, 1);
        }

        reducer.runReduce(reduce, reducedResult, results);
        return false;
    }

    void finish() override
    {
        reducer.finish(reduce, reducedResult);
    }

    bool shouldThrottleThread() override
    {
        return IterateKernel<Iterator, ReducedResultType>::shouldThrottleThread() || reducer.shouldThrottle();
    }

    bool shouldStartThread() override
    {
        return IterateKernel<Iterator, ReducedResultType>::shouldStartThread() && reducer.shouldStartThread();
    }

    typedef ReducedResultType ResultType;
    ReducedResultType *result() override
    {
        return &reducedResult;
    }
};

template <typename Iterator, typename MapFunctor>
class MappedEachKernel : public IterateKernel<Iterator, QtPrivate::MapResultType<Iterator, MapFunctor>>
{
    MapFunctor map;
    using T = QtPrivate::MapResultType<Iterator, MapFunctor>;

public:
    template <typename F = MapFunctor>
    MappedEachKernel(QThreadPool *pool, Iterator begin, Iterator end, F &&_map)
        : IterateKernel<Iterator, T>(pool, begin, end), map(std::forward<F>(_map))
    { }

    bool runIteration(Iterator it, int,  T *result) override
    {
        *result = std::invoke(map, *it);
        return true;
    }

    bool runIterations(Iterator sequenceBeginIterator, int beginIndex, int endIndex, T *results) override
    {

        Iterator it = sequenceBeginIterator;
        std::advance(it, beginIndex);
        for (int i = beginIndex; i < endIndex; ++i) {
            runIteration(it, i, results + (i - beginIndex));
            std::advance(it, 1);
        }

        return true;
    }
};

//! [qtconcurrentmapkernel-1]
template <typename Iterator, typename Functor>
inline ThreadEngineStarter<void> startMap(QThreadPool *pool, Iterator begin,
                                          Iterator end, Functor &&functor)
{
    return startThreadEngine(new MapKernel<Iterator, std::decay_t<Functor>>(
            pool, begin, end, std::forward<Functor>(functor)));
}

//! [qtconcurrentmapkernel-2]
template <typename T, typename Iterator, typename Functor>
inline ThreadEngineStarter<T> startMapped(QThreadPool *pool, Iterator begin,
                                          Iterator end, Functor &&functor)
{
    return startThreadEngine(new MappedEachKernel<Iterator, std::decay_t<Functor>>(
            pool, begin, end, std::forward<Functor>(functor)));
}

/*
    The SequnceHolder class is used to hold a reference to the
    sequence we are working on.
*/
template <typename Sequence, typename Base, typename Functor>
struct SequenceHolder1 : private QtPrivate::SequenceHolder<Sequence>, public Base
{
    template<typename S = Sequence, typename F = Functor>
    SequenceHolder1(QThreadPool *pool, S &&_sequence, F &&functor)
        : QtPrivate::SequenceHolder<Sequence>(std::forward<S>(_sequence)),
          Base(pool, this->sequence.cbegin(), this->sequence.cend(), std::forward<F>(functor))
    { }

    void finish() override
    {
        Base::finish();
        // Clear the sequence to make sure all temporaries are destroyed
        // before finished is signaled.
        this->sequence = Sequence();
    }
};

//! [qtconcurrentmapkernel-3]
template <typename T, typename Sequence, typename Functor>
inline ThreadEngineStarter<T> startMapped(QThreadPool *pool, Sequence &&sequence,
                                          Functor &&functor)
{
    using DecayedSequence = std::decay_t<Sequence>;
    using DecayedFunctor = std::decay_t<Functor>;
    using SequenceHolderType = SequenceHolder1<
            DecayedSequence,
            MappedEachKernel<typename DecayedSequence::const_iterator, DecayedFunctor>,
            DecayedFunctor>;

    return startThreadEngine(new SequenceHolderType(pool, std::forward<Sequence>(sequence),
                                                    std::forward<Functor>(functor)));
}

//! [qtconcurrentmapkernel-4]
template <typename IntermediateType, typename ResultType, typename Sequence, typename MapFunctor,
          typename ReduceFunctor>
inline ThreadEngineStarter<ResultType> startMappedReduced(QThreadPool *pool,
                                                          Sequence &&sequence,
                                                          MapFunctor &&mapFunctor,
                                                          ReduceFunctor &&reduceFunctor,
                                                          ReduceOptions options)
{
    using DecayedSequence = std::decay_t<Sequence>;
    using DecayedMapFunctor = std::decay_t<MapFunctor>;
    using DecayedReduceFunctor = std::decay_t<ReduceFunctor>;
    using Iterator = typename DecayedSequence::const_iterator;
    using Reducer = ReduceKernel<DecayedReduceFunctor, ResultType, IntermediateType>;
    using MappedReduceType = MappedReducedKernel<ResultType, Iterator, DecayedMapFunctor,
                                                 DecayedReduceFunctor, Reducer>;
    using SequenceHolderType = SequenceHolder2<DecayedSequence, MappedReduceType, DecayedMapFunctor,
                                               DecayedReduceFunctor>;
    return startThreadEngine(new SequenceHolderType(pool, std::forward<Sequence>(sequence),
                                                    std::forward<MapFunctor>(mapFunctor),
                                                    std::forward<ReduceFunctor>(reduceFunctor),
                                                    options));
}

//! [qtconcurrentmapkernel-5]
template <typename IntermediateType, typename ResultType, typename Iterator, typename MapFunctor,
          typename ReduceFunctor>
inline ThreadEngineStarter<ResultType> startMappedReduced(QThreadPool *pool,
                                                          Iterator begin,
                                                          Iterator end,
                                                          MapFunctor &&mapFunctor,
                                                          ReduceFunctor &&reduceFunctor,
                                                          ReduceOptions options)
{
    using Reducer =
            ReduceKernel<std::decay_t<ReduceFunctor>, std::decay_t<ResultType>, IntermediateType>;
    using MappedReduceType = MappedReducedKernel<ResultType, Iterator, std::decay_t<MapFunctor>,
                                                 std::decay_t<ReduceFunctor>, Reducer>;
    return startThreadEngine(new MappedReduceType(pool, begin, end,
                                                  std::forward<MapFunctor>(mapFunctor),
                                                  std::forward<ReduceFunctor>(reduceFunctor),
                                                  options));
}

//! [qtconcurrentmapkernel-6]
template <typename IntermediateType, typename ResultType, typename Sequence, typename MapFunctor,
          typename ReduceFunctor>
inline ThreadEngineStarter<ResultType> startMappedReduced(QThreadPool *pool,
                                                          Sequence &&sequence,
                                                          MapFunctor &&mapFunctor,
                                                          ReduceFunctor &&reduceFunctor,
                                                          ResultType &&initialValue,
                                                          ReduceOptions options)
{
    using DecayedSequence = std::decay_t<Sequence>;
    using DecayedMapFunctor = std::decay_t<MapFunctor>;
    using DecayedReduceFunctor = std::decay_t<ReduceFunctor>;
    using Iterator = typename DecayedSequence::const_iterator;
    using Reducer = ReduceKernel<DecayedReduceFunctor, ResultType, IntermediateType>;
    using MappedReduceType = MappedReducedKernel<ResultType, Iterator, DecayedMapFunctor,
                                                 DecayedReduceFunctor, Reducer>;
    using SequenceHolderType = SequenceHolder2<DecayedSequence, MappedReduceType, DecayedMapFunctor,
                                               DecayedReduceFunctor>;
    return startThreadEngine(
            new SequenceHolderType(pool, std::forward<Sequence>(sequence),
                                   std::forward<MapFunctor>(mapFunctor),
                                   std::forward<ReduceFunctor>(reduceFunctor),
                                   std::forward<ResultType>(initialValue), options));
}

//! [qtconcurrentmapkernel-7]
template <typename IntermediateType, typename ResultType, typename Iterator, typename MapFunctor,
          typename ReduceFunctor>
inline ThreadEngineStarter<ResultType> startMappedReduced(QThreadPool *pool,
                                                          Iterator begin,
                                                          Iterator end,
                                                          MapFunctor &&mapFunctor,
                                                          ReduceFunctor &&reduceFunctor,
                                                          ResultType &&initialValue,
                                                          ReduceOptions options)
{
    using Reducer = ReduceKernel<std::decay_t<ReduceFunctor>, ResultType, IntermediateType>;
    using MappedReduceType = MappedReducedKernel<ResultType, Iterator, std::decay_t<MapFunctor>,
                                                 std::decay_t<ReduceFunctor>, Reducer>;
    return startThreadEngine(new MappedReduceType(pool, begin, end,
                                                  std::forward<MapFunctor>(mapFunctor),
                                                  std::forward<ReduceFunctor>(reduceFunctor),
                                                  std::forward<ResultType>(initialValue), options));
}

} // namespace QtConcurrent


QT_END_NAMESPACE

#endif // QT_NO_CONCURRENT

#endif
