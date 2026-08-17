// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTCONCURRENT_REDUCEKERNEL_H
#define QTCONCURRENT_REDUCEKERNEL_H

#include <QtConcurrent/qtconcurrent_global.h>

#if !defined(QT_NO_CONCURRENT) || defined(Q_CLANG_QDOC)

#include <QtCore/qatomic.h>
#include <QtCore/qlist.h>
#include <QtCore/qmap.h>
#include <QtCore/qmutex.h>
#include <QtCore/qthread.h>
#include <QtCore/qthreadpool.h>

#include <mutex>

QT_BEGIN_NAMESPACE

namespace QtPrivate {

template<typename Sequence>
struct SequenceHolder
{
    SequenceHolder(const Sequence &s) : sequence(s) { }
    SequenceHolder(Sequence &&s) : sequence(std::move(s)) { }
    Sequence sequence;
};

}

namespace QtConcurrent {

/*
    The ReduceQueueStartLimit and ReduceQueueThrottleLimit constants
    limit the reduce queue size for MapReduce. When the number of
    reduce blocks in the queue exceeds ReduceQueueStartLimit,
    MapReduce won't start any new threads, and when it exceeds
    ReduceQueueThrottleLimit running threads will be stopped.
*/
#ifdef Q_CLANG_QDOC
enum ReduceQueueLimits {
    ReduceQueueStartLimit = 20,
    ReduceQueueThrottleLimit = 30
};
#else
enum {
    ReduceQueueStartLimit = 20,
    ReduceQueueThrottleLimit = 30
};
#endif

// IntermediateResults holds a block of intermediate results from a
// map or filter functor. The begin/end offsets indicates the origin
// and range of the block.
template <typename T>
class IntermediateResults
{
public:
    int begin, end;
    QList<T> vector;
};

enum ReduceOption {
    UnorderedReduce = 0x1,
    OrderedReduce = 0x2,
    SequentialReduce = 0x4
    // ParallelReduce = 0x8
};
Q_DECLARE_FLAGS(ReduceOptions, ReduceOption)
#ifndef Q_CLANG_QDOC
Q_DECLARE_OPERATORS_FOR_FLAGS(ReduceOptions)
#endif
// supports both ordered and out-of-order reduction
template <typename ReduceFunctor, typename ReduceResultType, typename T>
class ReduceKernel
{
    typedef QMap<int, IntermediateResults<T> > ResultsMap;

    const ReduceOptions reduceOptions;

    QMutex mutex;
    int progress, resultsMapSize;
    const int threadCount;
    ResultsMap resultsMap;

    bool canReduce(int begin) const
    {
        return (((reduceOptions & UnorderedReduce)
                 && progress == 0)
                || ((reduceOptions & OrderedReduce)
                    && progress == begin));
    }

    void reduceResult(ReduceFunctor &reduce,
                      ReduceResultType &r,
                      const IntermediateResults<T> &result)
    {
        for (int i = 0; i < result.vector.size(); ++i) {
            std::invoke(reduce, r, result.vector.at(i));
        }
    }

    void reduceResults(ReduceFunctor &reduce,
                       ReduceResultType &r,
                       ResultsMap &map)
    {
        typename ResultsMap::iterator it = map.begin();
        while (it != map.end()) {
            reduceResult(reduce, r, it.value());
            ++it;
        }
    }

public:
    ReduceKernel(QThreadPool *pool, ReduceOptions _reduceOptions)
        : reduceOptions(_reduceOptions), progress(0), resultsMapSize(0),
          threadCount(pool->maxThreadCount())
    { }

    void runReduce(ReduceFunctor &reduce,
                   ReduceResultType &r,
                   const IntermediateResults<T> &result)
    {
        std::unique_lock<QMutex> locker(mutex);
        if (!canReduce(result.begin)) {
            ++resultsMapSize;
            resultsMap.insert(result.begin, result);
            return;
        }

        if (reduceOptions & UnorderedReduce) {
            // UnorderedReduce
            progress = -1;

            // reduce this result
            locker.unlock();
            reduceResult(reduce, r, result);
            locker.lock();

            // reduce all stored results as well
            while (!resultsMap.isEmpty()) {
                ResultsMap resultsMapCopy = resultsMap;
                resultsMap.clear();

                locker.unlock();
                reduceResults(reduce, r, resultsMapCopy);
                locker.lock();

                resultsMapSize -= resultsMapCopy.size();
            }

            progress = 0;
        } else {
            // reduce this result
            locker.unlock();
            reduceResult(reduce, r, result);
            locker.lock();

            // OrderedReduce
            progress += result.end - result.begin;

            // reduce as many other results as possible
            typename ResultsMap::iterator it = resultsMap.begin();
            while (it != resultsMap.end()) {
                if (it.value().begin != progress)
                    break;

                locker.unlock();
                reduceResult(reduce, r, it.value());
                locker.lock();

                --resultsMapSize;
                progress += it.value().end - it.value().begin;
                it = resultsMap.erase(it);
            }
        }
    }

    // final reduction
    void finish(ReduceFunctor &reduce, ReduceResultType &r)
    {
        reduceResults(reduce, r, resultsMap);
    }

    inline bool shouldThrottle()
    {
        std::lock_guard<QMutex> locker(mutex);
        return (resultsMapSize > (ReduceQueueThrottleLimit * threadCount));
    }

    inline bool shouldStartThread()
    {
        std::lock_guard<QMutex> locker(mutex);
        return (resultsMapSize <= (ReduceQueueStartLimit * threadCount));
    }
};

template <typename Sequence, typename Base, typename Functor1, typename Functor2>
struct SequenceHolder2 : private QtPrivate::SequenceHolder<Sequence>, public Base
{
    template<typename S = Sequence, typename F1 = Functor1, typename F2 = Functor2>
    SequenceHolder2(QThreadPool *pool, S &&_sequence, F1 &&functor1, F2 &&functor2,
                    ReduceOptions reduceOptions)
        : QtPrivate::SequenceHolder<Sequence>(std::forward<S>(_sequence)),
          Base(pool, this->sequence.cbegin(), this->sequence.cend(),
               std::forward<F1>(functor1), std::forward<F2>(functor2), reduceOptions)
    { }

    template<typename InitialValueType, typename S = Sequence,
             typename F1 = Functor1, typename F2 = Functor2>
    SequenceHolder2(QThreadPool *pool, S &&_sequence, F1 &&functor1, F2 &&functor2,
                    InitialValueType &&initialValue, ReduceOptions reduceOptions)
        : QtPrivate::SequenceHolder<Sequence>(std::forward<S>(_sequence)),
          Base(pool, this->sequence.cbegin(), this->sequence.cend(),
               std::forward<F1>(functor1), std::forward<F2>(functor2),
               std::forward<InitialValueType>(initialValue), reduceOptions)
    { }

    void finish() override
    {
        Base::finish();
        // Clear the sequence to make sure all temporaries are destroyed
        // before finished is signaled.
        this->sequence = Sequence();
    }
};

} // namespace QtConcurrent

QT_END_NAMESPACE

#endif // QT_NO_CONCURRENT

#endif
