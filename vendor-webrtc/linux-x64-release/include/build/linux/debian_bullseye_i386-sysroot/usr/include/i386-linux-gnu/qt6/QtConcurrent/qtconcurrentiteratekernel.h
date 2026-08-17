// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTCONCURRENT_ITERATEKERNEL_H
#define QTCONCURRENT_ITERATEKERNEL_H

#include <QtConcurrent/qtconcurrent_global.h>

#if !defined(QT_NO_CONCURRENT) || defined(Q_CLANG_QDOC)

#include <QtCore/qatomic.h>
#include <QtConcurrent/qtconcurrentmedian.h>
#include <QtConcurrent/qtconcurrentthreadengine.h>

#include <iterator>

QT_BEGIN_NAMESPACE



namespace QtConcurrent {

/*
    The BlockSizeManager class manages how many iterations a thread should
    reserve and process at a time. This is done by measuring the time spent
    in the user code versus the control part code, and then increasing
    the block size if the ratio between them is to small. The block size
    management is done on the basis of the median of several timing measurements,
    and it is done individually for each thread.
*/
class Q_CONCURRENT_EXPORT BlockSizeManager
{
public:
    explicit BlockSizeManager(QThreadPool *pool, int iterationCount);

    void timeBeforeUser();
    void timeAfterUser();
    int blockSize();

private:
    inline bool blockSizeMaxed()
    {
        return (m_blockSize >= maxBlockSize);
    }

    const int maxBlockSize;
    qint64 beforeUser;
    qint64 afterUser;
    Median controlPartElapsed;
    Median userPartElapsed;
    int m_blockSize;

    Q_DISABLE_COPY(BlockSizeManager)
};

template <typename T>
class ResultReporter
{
public:
    ResultReporter(ThreadEngine<T> *_threadEngine, T &_defaultValue)
        : threadEngine(_threadEngine), defaultValue(_defaultValue)
    {
    }

    void reserveSpace(int resultCount)
    {
        currentResultCount = resultCount;
        resizeList(qMax(resultCount, vector.size()));
    }

    void reportResults(int begin)
    {
        const int useVectorThreshold = 4; // Tunable parameter.
        if (currentResultCount > useVectorThreshold) {
            resizeList(currentResultCount);
            threadEngine->reportResults(vector, begin);
        } else {
            for (int i = 0; i < currentResultCount; ++i)
                threadEngine->reportResult(&vector.at(i), begin + i);
        }
    }

    inline T * getPointer()
    {
        return vector.data();
    }

    int currentResultCount;
    ThreadEngine<T> *threadEngine;
    QList<T> vector;

private:
    void resizeList(qsizetype size)
    {
        if constexpr (std::is_default_constructible_v<T>)
            vector.resize(size);
        else
            vector.resize(size, defaultValue);
    }

    T &defaultValue;
};

template <>
class ResultReporter<void>
{
public:
    inline ResultReporter(ThreadEngine<void> *) { }
    inline void reserveSpace(int) { }
    inline void reportResults(int) { }
    inline void * getPointer() { return nullptr; }
};

template<typename T>
struct DefaultValueContainer
{
    template<typename U = T>
    DefaultValueContainer(U &&_value) : value(std::forward<U>(_value))
    {
    }

    T value;
};

template<>
struct DefaultValueContainer<void>
{
};

inline bool selectIteration(std::bidirectional_iterator_tag)
{
    return false; // while
}

inline bool selectIteration(std::forward_iterator_tag)
{
    return false; // while
}

inline bool selectIteration(std::random_access_iterator_tag)
{
    return true; // for
}

template <typename Iterator, typename T>
class IterateKernel : public ThreadEngine<T>
{
    using IteratorCategory = typename std::iterator_traits<Iterator>::iterator_category;

public:
    typedef T ResultType;

    template<typename U = T, std::enable_if_t<std::is_same_v<U, void>, bool> = true>
    IterateKernel(QThreadPool *pool, Iterator _begin, Iterator _end)
        : ThreadEngine<U>(pool),
          begin(_begin),
          end(_end),
          current(_begin),
          iterationCount(selectIteration(IteratorCategory()) ? std::distance(_begin, _end) : 0),
          forIteration(selectIteration(IteratorCategory())),
          progressReportingEnabled(true)
    {
    }

    template<typename U = T, std::enable_if_t<!std::is_same_v<U, void>, bool> = true>
    IterateKernel(QThreadPool *pool, Iterator _begin, Iterator _end)
        : ThreadEngine<U>(pool),
          begin(_begin),
          end(_end),
          current(_begin),
          iterationCount(selectIteration(IteratorCategory()) ? std::distance(_begin, _end) : 0),
          forIteration(selectIteration(IteratorCategory())),
          progressReportingEnabled(true),
          defaultValue(U())
    {
    }

    template<typename U = T, std::enable_if_t<!std::is_same_v<U, void>, bool> = true>
    IterateKernel(QThreadPool *pool, Iterator _begin, Iterator _end, U &&_defaultValue)
        : ThreadEngine<U>(pool),
          begin(_begin),
          end(_end),
          current(_begin),
          iterationCount(selectIteration(IteratorCategory()) ? std::distance(_begin, _end) : 0),
          forIteration(selectIteration(IteratorCategory())),
          progressReportingEnabled(true),
          defaultValue(std::forward<U>(_defaultValue))
    {
    }

    virtual ~IterateKernel() { }

    virtual bool runIteration(Iterator, int , T *) { return false; }
    virtual bool runIterations(Iterator, int, int, T *) { return false; }

    void start() override
    {
        progressReportingEnabled = this->isProgressReportingEnabled();
        if (progressReportingEnabled && iterationCount > 0)
            this->setProgressRange(0, iterationCount);
    }

    bool shouldStartThread() override
    {
        if (forIteration)
            return (currentIndex.loadRelaxed() < iterationCount) && !this->shouldThrottleThread();
        else // whileIteration
            return (iteratorThreads.loadRelaxed() == 0);
    }

    ThreadFunctionResult threadFunction() override
    {
        if (forIteration)
            return this->forThreadFunction();
        else // whileIteration
            return this->whileThreadFunction();
    }

    ThreadFunctionResult forThreadFunction()
    {
        BlockSizeManager blockSizeManager(ThreadEngineBase::threadPool, iterationCount);
        ResultReporter<T> resultReporter = createResultsReporter();

        for(;;) {
            if (this->isCanceled())
                break;

            const int currentBlockSize = blockSizeManager.blockSize();

            if (currentIndex.loadRelaxed() >= iterationCount)
                break;

            // Atomically reserve a block of iterationCount for this thread.
            const int beginIndex = currentIndex.fetchAndAddRelease(currentBlockSize);
            const int endIndex = qMin(beginIndex + currentBlockSize, iterationCount);

            if (beginIndex >= endIndex) {
                // No more work
                break;
            }

            this->waitForResume(); // (only waits if the qfuture is paused.)

            if (shouldStartThread())
                this->startThread();

            const int finalBlockSize = endIndex - beginIndex; // block size adjusted for possible end-of-range
            resultReporter.reserveSpace(finalBlockSize);

            // Call user code with the current iteration range.
            blockSizeManager.timeBeforeUser();
            const bool resultsAvailable = this->runIterations(begin, beginIndex, endIndex, resultReporter.getPointer());
            blockSizeManager.timeAfterUser();

            if (resultsAvailable)
                resultReporter.reportResults(beginIndex);

            // Report progress if progress reporting enabled.
            if (progressReportingEnabled) {
                completed.fetchAndAddAcquire(finalBlockSize);
                this->setProgressValue(this->completed.loadRelaxed());
            }

            if (this->shouldThrottleThread())
                return ThrottleThread;
        }
        return ThreadFinished;
    }

    ThreadFunctionResult whileThreadFunction()
    {
        if (iteratorThreads.testAndSetAcquire(0, 1) == false)
            return ThreadFinished;

        ResultReporter<T> resultReporter = createResultsReporter();
        resultReporter.reserveSpace(1);

        while (current != end) {
            // The following two lines breaks support for input iterators according to
            // the sgi docs: dereferencing prev after calling ++current is not allowed
            // on input iterators. (prev is dereferenced inside user.runIteration())
            Iterator prev = current;
            ++current;
            int index = currentIndex.fetchAndAddRelaxed(1);
            iteratorThreads.testAndSetRelease(1, 0);

            this->waitForResume(); // (only waits if the qfuture is paused.)

            if (shouldStartThread())
                this->startThread();

            const bool resultAavailable = this->runIteration(prev, index, resultReporter.getPointer());
            if (resultAavailable)
                resultReporter.reportResults(index);

            if (this->shouldThrottleThread())
                return ThrottleThread;

            if (iteratorThreads.testAndSetAcquire(0, 1) == false)
                return ThreadFinished;
        }

        return ThreadFinished;
    }

private:
    ResultReporter<T> createResultsReporter()
    {
        if constexpr (!std::is_same_v<T, void>)
            return ResultReporter<T>(this, defaultValue.value);
        else
            return ResultReporter<T>(this);
    }

public:
    const Iterator begin;
    const Iterator end;
    Iterator current;
    QAtomicInt currentIndex;
    QAtomicInt iteratorThreads;
    QAtomicInt completed;
    const int iterationCount;
    const bool forIteration;
    bool progressReportingEnabled;
    DefaultValueContainer<ResultType> defaultValue;
};

} // namespace QtConcurrent


QT_END_NAMESPACE

#endif // QT_NO_CONCURRENT

#endif
