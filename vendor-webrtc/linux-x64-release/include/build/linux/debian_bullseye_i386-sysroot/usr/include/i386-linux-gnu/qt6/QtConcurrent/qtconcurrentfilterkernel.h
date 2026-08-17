// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTCONCURRENT_FILTERKERNEL_H
#define QTCONCURRENT_FILTERKERNEL_H

#include <QtConcurrent/qtconcurrent_global.h>

#if !defined(QT_NO_CONCURRENT) || defined (Q_CLANG_QDOC)

#include <QtConcurrent/qtconcurrentiteratekernel.h>
#include <QtConcurrent/qtconcurrentmapkernel.h>
#include <QtConcurrent/qtconcurrentreducekernel.h>

QT_BEGIN_NAMESPACE



namespace QtConcurrent {

template <typename T>
struct qValueType
{
    typedef typename T::value_type value_type;
};

template <typename T>
struct qValueType<const T*>
{
    typedef T value_type;
};

template <typename T>
struct qValueType<T*>
{
    typedef T value_type;
};

// Implementation of filter
template <typename Sequence, typename KeepFunctor, typename ReduceFunctor>
class FilterKernel : public IterateKernel<typename Sequence::const_iterator, void>
{
    typedef ReduceKernel<ReduceFunctor, Sequence, typename Sequence::value_type> Reducer;
    typedef IterateKernel<typename Sequence::const_iterator, void> IterateKernelType;
    typedef void T;

    Sequence reducedResult;
    Sequence &sequence;
    KeepFunctor keep;
    ReduceFunctor reduce;
    Reducer reducer;

public:
    template <typename Keep = KeepFunctor, typename Reduce = ReduceFunctor>
    FilterKernel(QThreadPool *pool, Sequence &_sequence, Keep &&_keep, Reduce &&_reduce)
        : IterateKernelType(pool, const_cast<const Sequence &>(_sequence).begin(),
                            const_cast<const Sequence &>(_sequence).end()), reducedResult(),
          sequence(_sequence),
          keep(std::forward<Keep>(_keep)),
          reduce(std::forward<Reduce>(_reduce)),
          reducer(pool, OrderedReduce)
    { }

    bool runIteration(typename Sequence::const_iterator it, int index, T *) override
    {
        IntermediateResults<typename Sequence::value_type> results;
        results.begin = index;
        results.end = index + 1;

        if (std::invoke(keep, *it))
            results.vector.append(*it);

        reducer.runReduce(reduce, reducedResult, results);
        return false;
    }

    bool runIterations(typename Sequence::const_iterator sequenceBeginIterator, int begin, int end, T *) override
    {
        IntermediateResults<typename Sequence::value_type> results;
        results.begin = begin;
        results.end = end;
        results.vector.reserve(end - begin);


        typename Sequence::const_iterator it = sequenceBeginIterator;
        std::advance(it, begin);
        for (int i = begin; i < end; ++i) {
            if (std::invoke(keep, *it))
                results.vector.append(*it);
            std::advance(it, 1);
        }

        reducer.runReduce(reduce, reducedResult, results);
        return false;
    }

    void finish() override
    {
        reducer.finish(reduce, reducedResult);
        sequence = std::move(reducedResult);
    }

    inline bool shouldThrottleThread() override
    {
        return IterateKernelType::shouldThrottleThread() || reducer.shouldThrottle();
    }

    inline bool shouldStartThread() override
    {
        return IterateKernelType::shouldStartThread() && reducer.shouldStartThread();
    }

    typedef void ReturnType;
    typedef void ResultType;
};

// Implementation of filter-reduce
template <typename ReducedResultType,
          typename Iterator,
          typename KeepFunctor,
          typename ReduceFunctor,
          typename Reducer = ReduceKernel<ReduceFunctor,
                                          ReducedResultType,
                                          typename qValueType<Iterator>::value_type> >
class FilteredReducedKernel : public IterateKernel<Iterator, ReducedResultType>
{
    ReducedResultType &reducedResult;
    KeepFunctor keep;
    ReduceFunctor reduce;
    Reducer reducer;
    typedef IterateKernel<Iterator, ReducedResultType> IterateKernelType;

public:
    template<typename Keep = KeepFunctor, typename Reduce = ReduceFunctor>
    FilteredReducedKernel(QThreadPool *pool, Iterator begin, Iterator end, Keep &&_keep,
                          Reduce &&_reduce, ReduceOptions reduceOption)
        : IterateKernelType(pool, begin, end),
          reducedResult(this->defaultValue.value),
          keep(std::forward<Keep>(_keep)),
          reduce(std::forward<Reduce>(_reduce)),
          reducer(pool, reduceOption)
    { }

    template <typename Keep = KeepFunctor, typename Reduce = ReduceFunctor>
    FilteredReducedKernel(QThreadPool *pool, Iterator begin, Iterator end, Keep &&_keep,
                          Reduce &&_reduce, ReducedResultType &&initialValue,
                          ReduceOptions reduceOption)
        : IterateKernelType(pool, begin, end, std::forward<ReducedResultType>(initialValue)),
          reducedResult(this->defaultValue.value),
          keep(std::forward<Keep>(_keep)),
          reduce(std::forward<Reduce>(_reduce)),
          reducer(pool, reduceOption)
    {
    }

    bool runIteration(Iterator it, int index, ReducedResultType *) override
    {
        IntermediateResults<typename qValueType<Iterator>::value_type> results;
        results.begin = index;
        results.end = index + 1;

        if (std::invoke(keep, *it))
            results.vector.append(*it);

        reducer.runReduce(reduce, reducedResult, results);
        return false;
    }

    bool runIterations(Iterator sequenceBeginIterator, int begin, int end, ReducedResultType *) override
    {
        IntermediateResults<typename qValueType<Iterator>::value_type> results;
        results.begin = begin;
        results.end = end;
        results.vector.reserve(end - begin);

        Iterator it = sequenceBeginIterator;
        std::advance(it, begin);
        for (int i = begin; i < end; ++i) {
            if (std::invoke(keep, *it))
                results.vector.append(*it);
            std::advance(it, 1);
        }

        reducer.runReduce(reduce, reducedResult, results);
        return false;
    }

    void finish() override
    {
        reducer.finish(reduce, reducedResult);
    }

    inline bool shouldThrottleThread() override
    {
        return IterateKernelType::shouldThrottleThread() || reducer.shouldThrottle();
    }

    inline bool shouldStartThread() override
    {
        return IterateKernelType::shouldStartThread() && reducer.shouldStartThread();
    }

    typedef ReducedResultType ReturnType;
    typedef ReducedResultType ResultType;
    ReducedResultType *result() override
    {
        return &reducedResult;
    }
};

// Implementation of filter that reports individual results via QFutureInterface
template <typename Iterator, typename KeepFunctor>
class FilteredEachKernel : public IterateKernel<Iterator, typename qValueType<Iterator>::value_type>
{
    typedef typename qValueType<Iterator>::value_type T;
    typedef IterateKernel<Iterator, T> IterateKernelType;

    KeepFunctor keep;

public:
    typedef T ReturnType;
    typedef T ResultType;

    template <typename Keep = KeepFunctor>
    FilteredEachKernel(QThreadPool *pool, Iterator begin, Iterator end, Keep &&_keep)
        : IterateKernelType(pool, begin, end), keep(std::forward<Keep>(_keep))
    { }

    void start() override
    {
        if (this->futureInterface)
            this->futureInterface->setFilterMode(true);
        IterateKernelType::start();
    }

    bool runIteration(Iterator it, int index, T *) override
    {
        if (std::invoke(keep, *it))
            this->reportResult(&(*it), index);
        else
            this->reportResult(nullptr, index);
        return false;
    }

    bool runIterations(Iterator sequenceBeginIterator, int begin, int end, T *) override
    {
        const int count = end - begin;
        IntermediateResults<typename qValueType<Iterator>::value_type> results;
        results.begin = begin;
        results.end = end;
        results.vector.reserve(count);

        Iterator it = sequenceBeginIterator;
        std::advance(it, begin);
        for (int i = begin; i < end; ++i) {
            if (std::invoke(keep, *it))
                results.vector.append(*it);
            std::advance(it, 1);
        }

        this->reportResults(results.vector, begin, count);
        return false;
    }
};

//! [QtConcurrent-2]
template <typename Iterator, typename KeepFunctor>
inline
ThreadEngineStarter<typename qValueType<Iterator>::value_type>
startFiltered(QThreadPool *pool, Iterator begin, Iterator end, KeepFunctor &&functor)
{
    return startThreadEngine(new FilteredEachKernel<Iterator, std::decay_t<KeepFunctor>>
                             (pool, begin, end, std::forward<KeepFunctor>(functor)));
}

//! [QtConcurrent-3]
template <typename Sequence, typename KeepFunctor>
inline decltype(auto) startFiltered(QThreadPool *pool, Sequence &&sequence, KeepFunctor &&functor)
{
    using DecayedSequence = std::decay_t<Sequence>;
    using DecayedFunctor = std::decay_t<KeepFunctor>;
    using SequenceHolderType = SequenceHolder1<DecayedSequence,
            FilteredEachKernel<typename DecayedSequence::const_iterator, DecayedFunctor>,
            DecayedFunctor>;
    return startThreadEngine(new SequenceHolderType(pool, std::forward<Sequence>(sequence),
                                                    std::forward<KeepFunctor>(functor)));
}

//! [QtConcurrent-4]
template <typename ResultType, typename Sequence, typename MapFunctor, typename ReduceFunctor>
inline ThreadEngineStarter<ResultType> startFilteredReduced(QThreadPool *pool,
                                                            Sequence &&sequence,
                                                            MapFunctor &&mapFunctor,
                                                            ReduceFunctor &&reduceFunctor,
                                                            ReduceOptions options)
{
    using DecayedSequence = std::decay_t<Sequence>;
    using DecayedMapFunctor = std::decay_t<MapFunctor>;
    using DecayedReduceFunctor = std::decay_t<ReduceFunctor>;
    using Iterator = typename DecayedSequence::const_iterator;
    using Reducer = ReduceKernel<DecayedReduceFunctor, ResultType,
                                 typename qValueType<Iterator>::value_type>;
    using FilteredReduceType = FilteredReducedKernel<ResultType, Iterator, DecayedMapFunctor,
                                                     DecayedReduceFunctor, Reducer>;
    using SequenceHolderType = SequenceHolder2<DecayedSequence, FilteredReduceType,
                                               DecayedMapFunctor, DecayedReduceFunctor>;
    return startThreadEngine(new SequenceHolderType(pool, std::forward<Sequence>(sequence),
                                                    std::forward<MapFunctor>(mapFunctor),
                                                    std::forward<ReduceFunctor>(reduceFunctor),
                                                    options));
}


//! [QtConcurrent-5]
template <typename ResultType, typename Iterator, typename MapFunctor, typename ReduceFunctor>
inline ThreadEngineStarter<ResultType> startFilteredReduced(QThreadPool *pool,
                                                            Iterator begin,
                                                            Iterator end,
                                                            MapFunctor &&mapFunctor,
                                                            ReduceFunctor &&reduceFunctor,
                                                            ReduceOptions options)
{
    using Reducer = ReduceKernel<std::decay_t<ReduceFunctor>, ResultType,
                                 typename qValueType<Iterator>::value_type>;
    using FilteredReduceType = FilteredReducedKernel<ResultType, Iterator, std::decay_t<MapFunctor>,
                                                     std::decay_t<ReduceFunctor>, Reducer>;
    return startThreadEngine(
            new FilteredReduceType(pool, begin, end, std::forward<MapFunctor>(mapFunctor),
                                   std::forward<ReduceFunctor>(reduceFunctor), options));
}

// Repeat the two functions above, but now with an initial value!
//! [QtConcurrent-6]
template <typename ResultType, typename Sequence, typename MapFunctor, typename ReduceFunctor>
inline ThreadEngineStarter<ResultType> startFilteredReduced(QThreadPool *pool,
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
    using Reducer = ReduceKernel<DecayedReduceFunctor, ResultType,
                                 typename qValueType<Iterator>::value_type>;
    using FilteredReduceType = FilteredReducedKernel<ResultType, Iterator, DecayedMapFunctor,
                                                     DecayedReduceFunctor, Reducer>;
    using SequenceHolderType = SequenceHolder2<DecayedSequence, FilteredReduceType,
                                               DecayedMapFunctor, DecayedReduceFunctor>;
    return startThreadEngine(new SequenceHolderType(
            pool, std::forward<Sequence>(sequence), std::forward<MapFunctor>(mapFunctor),
            std::forward<ReduceFunctor>(reduceFunctor), std::forward<ResultType>(initialValue),
            options));
}

//! [QtConcurrent-7]
template <typename ResultType, typename Iterator, typename MapFunctor, typename ReduceFunctor>
inline ThreadEngineStarter<ResultType> startFilteredReduced(QThreadPool *pool,
                                                            Iterator begin,
                                                            Iterator end,
                                                            MapFunctor &&mapFunctor,
                                                            ReduceFunctor &&reduceFunctor,
                                                            ResultType &&initialValue,
                                                            ReduceOptions options)
{
    using Reducer = ReduceKernel<std::decay_t<ReduceFunctor>, ResultType,
                                 typename qValueType<Iterator>::value_type>;
    using FilteredReduceType = FilteredReducedKernel<ResultType, Iterator, std::decay_t<MapFunctor>,
                                                     std::decay_t<ReduceFunctor>, Reducer>;
    return startThreadEngine(
            new FilteredReduceType(pool, begin, end, std::forward<MapFunctor>(mapFunctor),
                                   std::forward<ReduceFunctor>(reduceFunctor),
                                   std::forward<ResultType>(initialValue), options));
}


} // namespace QtConcurrent


QT_END_NAMESPACE

#endif // QT_NO_CONCURRENT

#endif
