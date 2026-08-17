/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtConcurrent module of the Qt Toolkit.
**
** $QT_BEGIN_LICENSE:LGPL$
** Commercial License Usage
** Licensees holding valid commercial Qt licenses may use this file in
** accordance with the commercial license agreement provided with the
** Software or, alternatively, in accordance with the terms contained in
** a written agreement between you and The Qt Company. For licensing terms
** and conditions see https://www.qt.io/terms-conditions. For further
** information use the contact form at https://www.qt.io/contact-us.
**
** GNU Lesser General Public License Usage
** Alternatively, this file may be used under the terms of the GNU Lesser
** General Public License version 3 as published by the Free Software
** Foundation and appearing in the file LICENSE.LGPL3 included in the
** packaging of this file. Please review the following information to
** ensure the GNU Lesser General Public License version 3 requirements
** will be met: https://www.gnu.org/licenses/lgpl-3.0.html.
**
** GNU General Public License Usage
** Alternatively, this file may be used under the terms of the GNU
** General Public License version 2.0 or (at your option) the GNU General
** Public license version 3 or any later version approved by the KDE Free
** Qt Foundation. The licenses are as published by the Free Software
** Foundation and appearing in the file LICENSE.GPL2 and LICENSE.GPL3
** included in the packaging of this file. Please review the following
** information to ensure the GNU General Public License requirements will
** be met: https://www.gnu.org/licenses/gpl-2.0.html and
** https://www.gnu.org/licenses/gpl-3.0.html.
**
** $QT_END_LICENSE$
**
****************************************************************************/

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
    management is done on the basis of the median of several timing measuremens,
    and it is done induvidualy for each thread.
*/
class Q_CONCURRENT_EXPORT BlockSizeManager
{
public:
    BlockSizeManager(int iterationCount);
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
    Median<double> controlPartElapsed;
    Median<double> userPartElapsed;
    int m_blockSize;

    Q_DISABLE_COPY(BlockSizeManager)
};

// ### Qt6: Replace BlockSizeManager with V2 implementation
class Q_CONCURRENT_EXPORT BlockSizeManagerV2
{
public:
    explicit BlockSizeManagerV2(int iterationCount);

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
    MedianDouble controlPartElapsed;
    MedianDouble userPartElapsed;
    int m_blockSize;

    Q_DISABLE_COPY(BlockSizeManagerV2)
};

template <typename T>
class ResultReporter
{
public:
    ResultReporter(ThreadEngine<T> *_threadEngine)
    :threadEngine(_threadEngine)
    {

    }

    void reserveSpace(int resultCount)
    {
        currentResultCount = resultCount;
        vector.resize(qMax(resultCount, vector.count()));
    }

    void reportResults(int begin)
    {
        const int useVectorThreshold = 4; // Tunable parameter.
        if (currentResultCount > useVectorThreshold) {
            vector.resize(currentResultCount);
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
    QVector<T> vector;
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
public:
    typedef T ResultType;

    IterateKernel(Iterator _begin, Iterator _end)
        : begin(_begin), end(_end), current(_begin), currentIndex(0),
           forIteration(selectIteration(typename std::iterator_traits<Iterator>::iterator_category())), progressReportingEnabled(true)
    {
        iterationCount =  forIteration ? std::distance(_begin, _end) : 0;
    }

    virtual ~IterateKernel() { }

    virtual bool runIteration(Iterator it, int index , T *result)
        { Q_UNUSED(it); Q_UNUSED(index); Q_UNUSED(result); return false; }
    virtual bool runIterations(Iterator _begin, int beginIndex, int endIndex, T *results)
        { Q_UNUSED(_begin); Q_UNUSED(beginIndex); Q_UNUSED(endIndex); Q_UNUSED(results); return false; }

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
        BlockSizeManagerV2 blockSizeManager(iterationCount);
        ResultReporter<T> resultReporter(this);

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

        ResultReporter<T> resultReporter(this);
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


public:
    const Iterator begin;
    const Iterator end;
    Iterator current;
    QAtomicInt currentIndex;
    bool forIteration;
    QAtomicInt iteratorThreads;
    int iterationCount;

    bool progressReportingEnabled;
    QAtomicInt completed;
};

} // namespace QtConcurrent


QT_END_NAMESPACE

#endif // QT_NO_CONCURRENT

#endif
