// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QFUTUREINTERFACE_H
#define QFUTUREINTERFACE_H

#include <QtCore/qmutex.h>
#include <QtCore/qresultstore.h>
#ifndef QT_NO_EXCEPTIONS
#include <exception>
#endif

#include <utility>

QT_REQUIRE_CONFIG(future);

QT_FORWARD_DECLARE_CLASS(QRunnable)
QT_FORWARD_DECLARE_CLASS(QException)
QT_BEGIN_NAMESPACE


template <typename T> class QFuture;
class QThreadPool;
class QFutureInterfaceBasePrivate;
class QFutureWatcherBase;
class QFutureWatcherBasePrivate;

namespace QtPrivate {
template<typename Function, typename ResultType, typename ParentResultType>
class Continuation;

class ExceptionStore;

template<class Function, class ResultType>
class CanceledHandler;

#ifndef QT_NO_EXCEPTIONS
template<class Function, class ResultType>
class FailureHandler;
#endif
}

class Q_CORE_EXPORT QFutureInterfaceBase
{
public:
    enum State {
        NoState    = 0x00,
        Running    = 0x01,
        Started    = 0x02,
        Finished   = 0x04,
        Canceled   = 0x08,
        Suspending = 0x10,
        Suspended  = 0x20,
        Throttled  = 0x40,
        // Pending means that the future depends on another one, which is not finished yet
        Pending    = 0x80,
    };

    QFutureInterfaceBase(State initialState = NoState);
    QFutureInterfaceBase(const QFutureInterfaceBase &other);
    QFutureInterfaceBase(QFutureInterfaceBase &&other) noexcept
        : d(std::exchange(other.d, nullptr)) {}
    QFutureInterfaceBase &operator=(const QFutureInterfaceBase &other);
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_MOVE_AND_SWAP(QFutureInterfaceBase)
    virtual ~QFutureInterfaceBase();

    // reporting functions available to the engine author:
    void reportStarted();
    void reportFinished();
    void reportCanceled();
#ifndef QT_NO_EXCEPTIONS
    void reportException(const QException &e);
#if QT_VERSION < QT_VERSION_CHECK(7, 0, 0)
    void reportException(std::exception_ptr e);
#else
    void reportException(const std::exception_ptr &e);
#endif
#endif
    void reportResultsReady(int beginIndex, int endIndex);

    void setRunnable(QRunnable *runnable);
    void setThreadPool(QThreadPool *pool);
    QThreadPool *threadPool() const;
    void setFilterMode(bool enable);
    void setProgressRange(int minimum, int maximum);
    int progressMinimum() const;
    int progressMaximum() const;
    bool isProgressUpdateNeeded() const;
    void setProgressValue(int progressValue);
    int progressValue() const;
    void setProgressValueAndText(int progressValue, const QString &progressText);
    QString progressText() const;

    void setExpectedResultCount(int resultCount);
    int expectedResultCount();
    int resultCount() const;

    bool queryState(State state) const;
    bool isRunning() const;
    bool isStarted() const;
    bool isCanceled() const;
    bool isFinished() const;
#if QT_DEPRECATED_SINCE(6, 0)
    QT_DEPRECATED_VERSION_X_6_0("Use isSuspending() or isSuspended() instead.")
    bool isPaused() const;

    QT_DEPRECATED_VERSION_X_6_0("Use setSuspended() instead.")
    void setPaused(bool paused) { setSuspended(paused); }

    QT_DEPRECATED_VERSION_X_6_0("Use toggleSuspended() instead.")
    void togglePaused() { toggleSuspended(); }
#endif
    bool isSuspending() const;
    bool isSuspended() const;
    bool isThrottled() const;
    bool isResultReadyAt(int index) const;
    bool isValid() const;
    int loadState() const;

    void cancel();
    void cancelAndFinish() { cancel(CancelMode::CancelAndFinish); }

    void setSuspended(bool suspend);
    void toggleSuspended();
    void reportSuspended() const;
    void setThrottled(bool enable);

    void waitForFinished();
    bool waitForNextResult();
    void waitForResult(int resultIndex);
    void waitForResume();
    void suspendIfRequested();

    QMutex &mutex() const;
    bool hasException() const;
    QtPrivate::ExceptionStore &exceptionStore();
    QtPrivate::ResultStoreBase &resultStoreBase();
    const QtPrivate::ResultStoreBase &resultStoreBase() const;

    inline bool operator==(const QFutureInterfaceBase &other) const { return d == other.d; }
    inline bool operator!=(const QFutureInterfaceBase &other) const { return d != other.d; }

    // ### Qt 7: inline
    void swap(QFutureInterfaceBase &other) noexcept;

    template<typename T>
    static QFutureInterfaceBase get(const QFuture<T> &future);  // implemented in qfuture.h

    bool isChainCanceled() const;

protected:
    // ### Qt 7: remove const from refT/derefT
    bool refT() const noexcept;
    bool derefT() const noexcept;
    void reset();
    void rethrowPossibleException();
public:

#ifndef QFUTURE_TEST
private:
#endif
    QFutureInterfaceBasePrivate *d;

private:
    friend class QFutureWatcherBase;
    friend class QFutureWatcherBasePrivate;

    template<typename Function, typename ResultType, typename ParentResultType>
    friend class QtPrivate::Continuation;

    template<class Function, class ResultType>
    friend class QtPrivate::CanceledHandler;

#ifndef QT_NO_EXCEPTIONS
    template<class Function, class ResultType>
    friend class QtPrivate::FailureHandler;
#endif

    template<class T>
    friend class QPromise;

protected:
    void setContinuation(std::function<void(const QFutureInterfaceBase &)> func);
    void setContinuation(std::function<void(const QFutureInterfaceBase &)> func,
                         QFutureInterfaceBasePrivate *continuationFutureData);
    void cleanContinuation();
    void runContinuation() const;

    void setLaunchAsync(bool value);
    bool launchAsync() const;

    bool isRunningOrPending() const;

    enum class CancelMode { CancelOnly, CancelAndFinish };
    void cancel(CancelMode mode);
};

inline void swap(QFutureInterfaceBase &lhs, QFutureInterfaceBase &rhs) noexcept
{
    lhs.swap(rhs);
}

template <typename T>
class QFutureInterface : public QFutureInterfaceBase
{
public:
    QFutureInterface(State initialState = NoState)
        : QFutureInterfaceBase(initialState)
    {
        refT();
    }
    QFutureInterface(const QFutureInterface &other)
        : QFutureInterfaceBase(other)
    {
        refT();
    }
    QFutureInterface(const QFutureInterfaceBase &dd) : QFutureInterfaceBase(dd) { refT(); }
    QFutureInterface(QFutureInterfaceBase &&dd) noexcept : QFutureInterfaceBase(std::move(dd)) { refT(); }
    QFutureInterface &operator=(const QFutureInterface &other)
    {
        QFutureInterface copy(other);
        swap(copy);
        return *this;
    }
    QFutureInterface(QFutureInterface &&other) = default;
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_MOVE_AND_SWAP(QFutureInterface)

    ~QFutureInterface()
    {
        if (!derefT() && !hasException())
            resultStoreBase().template clear<T>();
    }

    static QFutureInterface canceledResult()
    { return QFutureInterface(State(Started | Finished | Canceled)); }

    inline QFuture<T> future(); // implemented in qfuture.h

    inline bool reportResult(const T *result, int index = -1);
    inline bool reportAndMoveResult(T &&result, int index = -1);
    inline bool reportResult(T &&result, int index = -1);
    inline bool reportResult(const T &result, int index = -1);
    inline bool reportResults(const QList<T> &results, int beginIndex = -1, int count = -1);
    inline bool reportFinished(const T *result);
    void reportFinished()
    {
        QFutureInterfaceBase::reportFinished();
        QFutureInterfaceBase::runContinuation();
    }

    inline const T &resultReference(int index) const;
    inline const T *resultPointer(int index) const;
    inline QList<T> results();

    T takeResult();
#if 0
    // TODO: Enable and make it return a QList, when QList is fixed to support move-only types
    std::vector<T> takeResults();
#endif

#ifndef QT_NO_EXCEPTIONS
    void reportException(const std::exception_ptr &e)
    {
        if (hasException())
            return;

        resultStoreBase().template clear<T>();
        QFutureInterfaceBase::reportException(e);
    }
    void reportException(const QException &e)
    {
        if (hasException())
            return;

        resultStoreBase().template clear<T>();
        QFutureInterfaceBase::reportException(e);
    }
#endif
};

template <typename T>
inline bool QFutureInterface<T>::reportResult(const T *result, int index)
{
    QMutexLocker<QMutex> locker{&mutex()};
    if (this->queryState(Canceled) || this->queryState(Finished))
        return false;

    Q_ASSERT(!hasException());
    QtPrivate::ResultStoreBase &store = resultStoreBase();

    const int resultCountBefore = store.count();
    const int insertIndex = store.addResult<T>(index, result);
    if (insertIndex == -1)
        return false;
    if (store.filterMode()) {
        this->reportResultsReady(resultCountBefore, store.count());
    } else {
        this->reportResultsReady(insertIndex, insertIndex + 1);
    }
    return true;
}

template<typename T>
bool QFutureInterface<T>::reportAndMoveResult(T &&result, int index)
{
    QMutexLocker<QMutex> locker{&mutex()};
    if (queryState(Canceled) || queryState(Finished))
        return false;

    Q_ASSERT(!hasException());
    QtPrivate::ResultStoreBase &store = resultStoreBase();

    const int oldResultCount = store.count();
    const int insertIndex = store.moveResult(index, std::forward<T>(result));
    // Let's make sure it's not in pending results.
    if (insertIndex != -1 && (!store.filterMode() || oldResultCount < store.count()))
        reportResultsReady(insertIndex, store.count());
    return insertIndex != -1;
}

template<typename T>
bool QFutureInterface<T>::reportResult(T &&result, int index)
{
    return reportAndMoveResult(std::move(result), index);
}

template <typename T>
inline bool QFutureInterface<T>::reportResult(const T &result, int index)
{
    return reportResult(&result, index);
}

template<typename T>
inline bool QFutureInterface<T>::reportResults(const QList<T> &_results, int beginIndex, int count)
{
    QMutexLocker<QMutex> locker{&mutex()};
    if (this->queryState(Canceled) || this->queryState(Finished))
        return false;

    Q_ASSERT(!hasException());
    auto &store = resultStoreBase();

    const int resultCountBefore = store.count();
    const int insertIndex = store.addResults(beginIndex, &_results, count);
    if (insertIndex == -1)
        return false;
    if (store.filterMode()) {
        this->reportResultsReady(resultCountBefore, store.count());
    } else {
        this->reportResultsReady(insertIndex, insertIndex + _results.size());
    }
    return true;
}

template <typename T>
inline bool QFutureInterface<T>::reportFinished(const T *result)
{
    bool resultReported = false;
    if (result)
        resultReported = reportResult(result);
    reportFinished();
    return resultReported;
}

template <typename T>
inline const T &QFutureInterface<T>::resultReference(int index) const
{
    Q_ASSERT(!hasException());

    QMutexLocker<QMutex> locker{&mutex()};
    return resultStoreBase().resultAt(index).template value<T>();
}

template <typename T>
inline const T *QFutureInterface<T>::resultPointer(int index) const
{
    Q_ASSERT(!hasException());

    QMutexLocker<QMutex> locker{&mutex()};
    return resultStoreBase().resultAt(index).template pointer<T>();
}

template <typename T>
inline QList<T> QFutureInterface<T>::results()
{
    if (this->isCanceled()) {
        rethrowPossibleException();
        return QList<T>();
    }

    QFutureInterfaceBase::waitForResult(-1);

    QList<T> res;
    QMutexLocker<QMutex> locker{&mutex()};

    QtPrivate::ResultIteratorBase it = resultStoreBase().begin();
    while (it != resultStoreBase().end()) {
        res.append(it.value<T>());
        ++it;
    }

    return res;
}

template<typename T>
T QFutureInterface<T>::takeResult()
{
    Q_ASSERT(isValid());

    // Note: we wait for all, this is intentional,
    // not to mess with other unready results.
    waitForResult(-1);

    Q_ASSERT(!hasException());

    const QMutexLocker<QMutex> locker{&mutex()};
    QtPrivate::ResultIteratorBase position = resultStoreBase().resultAt(0);
    T ret(std::move_if_noexcept(position.value<T>()));
    reset();
    resultStoreBase().template clear<T>();

    return ret;
}

#if 0
template<typename T>
std::vector<T> QFutureInterface<T>::takeResults()
{
    Q_ASSERT(isValid());

    waitForResult(-1);

    Q_ASSERT(!hasException());

    std::vector<T> res;
    res.reserve(resultCount());

    const QMutexLocker<QMutex> locker{&mutex()};

    QtPrivate::ResultIteratorBase it = resultStoreBase().begin();
    for (auto endIt = resultStoreBase().end(); it != endIt; ++it)
        res.push_back(std::move_if_noexcept(it.value<T>()));

    reset();
    resultStoreBase().template clear<T>();

    return res;
}
#endif

template <>
class QFutureInterface<void> : public QFutureInterfaceBase
{
public:
    explicit QFutureInterface<void>(State initialState = NoState)
        : QFutureInterfaceBase(initialState)
    { }

    QFutureInterface(const QFutureInterfaceBase &dd) : QFutureInterfaceBase(dd) { }

    static QFutureInterface<void> canceledResult()
    { return QFutureInterface(State(Started | Finished | Canceled)); }


    inline QFuture<void> future(); // implemented in qfuture.h

    bool reportResult(const void *, int) { return false; }
    bool reportResults(const QList<void> &, int) { return false; }
    bool reportFinished(const void *)
    {
        reportFinished();
        return false;
    }
    void reportFinished()
    {
        QFutureInterfaceBase::reportFinished();
        QFutureInterfaceBase::runContinuation();
    }
};

template<typename T>
inline void swap(QFutureInterface<T> &a, QFutureInterface<T> &b) noexcept
{
    a.swap(b);
}

QT_END_NAMESPACE

#endif // QFUTUREINTERFACE_H
