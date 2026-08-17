// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QFUTUREWATCHER_H
#define QFUTUREWATCHER_H

#include <QtCore/qfuture.h>
#include <QtCore/qobject.h>

QT_REQUIRE_CONFIG(future);

QT_BEGIN_NAMESPACE


class QEvent;

class QFutureWatcherBasePrivate;
class Q_CORE_EXPORT QFutureWatcherBase : public QObject
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QFutureWatcherBase)

public:
    explicit QFutureWatcherBase(QObject *parent = nullptr);
    // de-inline dtor

    int progressValue() const;
    int progressMinimum() const;
    int progressMaximum() const;
    QString progressText() const;

    bool isStarted() const;
    bool isFinished() const;
    bool isRunning() const;
    bool isCanceled() const;
#if QT_DEPRECATED_SINCE(6, 0)
    QT_DEPRECATED_VERSION_X_6_0("Use isSuspending() or isSuspended() instead.")
    bool isPaused() const;
#endif
    bool isSuspending() const;
    bool isSuspended() const;

    void waitForFinished();

    void setPendingResultsLimit(int limit);

    bool event(QEvent *event) override;

Q_SIGNALS:
    void started();
    void finished();
    void canceled();
#if QT_DEPRECATED_SINCE(6, 0)
    QT_DEPRECATED_VERSION_X_6_0("Use suspending() instead.")
    void paused();
#endif
    void suspending();
    void suspended();
    void resumed();
    void resultReadyAt(int resultIndex);
    void resultsReadyAt(int beginIndex, int endIndex);
    void progressRangeChanged(int minimum, int maximum);
    void progressValueChanged(int progressValue);
    void progressTextChanged(const QString &progressText);

public Q_SLOTS:
    void cancel();
    void setSuspended(bool suspend);
    void suspend();
    void resume();
    void toggleSuspended();

#if QT_DEPRECATED_SINCE(6, 0)
    QT_DEPRECATED_VERSION_X_6_0("Use setSuspended() instead.")
    void setPaused(bool paused);

    QT_DEPRECATED_VERSION_X_6_0("Use suspended() instead.")
    void pause();

    QT_DEPRECATED_VERSION_X_6_0("Use toggleSuspended() instead.")
    void togglePaused();
#endif

protected:
    void connectNotify (const QMetaMethod &signal) override;
    void disconnectNotify (const QMetaMethod &signal) override;

    // called from setFuture() implemented in template sub-classes
    void connectOutputInterface();
    void disconnectOutputInterface(bool pendingAssignment = false);

private:
    // implemented in the template sub-classes
    virtual const QFutureInterfaceBase &futureInterface() const = 0;
    virtual QFutureInterfaceBase &futureInterface() = 0;
};

template <typename T>
class QFutureWatcher : public QFutureWatcherBase
{
public:
    explicit QFutureWatcher(QObject *_parent = nullptr)
        : QFutureWatcherBase(_parent)
    { }
    ~QFutureWatcher()
    { disconnectOutputInterface(); }

    void setFuture(const QFuture<T> &future);
    QFuture<T> future() const
    { return m_future; }

    template<typename U = T, typename = QtPrivate::EnableForNonVoid<U>>
    T result() const { return m_future.result(); }

    template<typename U = T, typename = QtPrivate::EnableForNonVoid<U>>
    T resultAt(int index) const { return m_future.resultAt(index); }

#ifdef Q_QDOC
    int progressValue() const;
    int progressMinimum() const;
    int progressMaximum() const;
    QString progressText() const;

    bool isStarted() const;
    bool isFinished() const;
    bool isRunning() const;
    bool isCanceled() const;
#if QT_DEPRECATED_SINCE(6, 0)
    bool isPaused() const;
#endif
    bool isSuspending() const;
    bool isSuspended() const;

    void waitForFinished();

    void setPendingResultsLimit(int limit);

Q_SIGNALS:
    void started();
    void finished();
    void canceled();
#if QT_DEPRECATED_SINCE(6, 0)
    void paused();
#endif
    void suspending();
    void suspended();
    void resumed();
    void resultReadyAt(int resultIndex);
    void resultsReadyAt(int beginIndex, int endIndex);
    void progressRangeChanged(int minimum, int maximum);
    void progressValueChanged(int progressValue);
    void progressTextChanged(const QString &progressText);

public Q_SLOTS:
    void cancel();
    void setSuspended(bool suspend);
    void suspend();
    void resume();
    void toggleSuspended();
#if QT_DEPRECATED_SINCE(6, 0)
    void setPaused(bool paused);
    void pause();
    void togglePaused();
#endif // QT_DEPRECATED_SINCE(6, 0)

#endif // Q_QDOC

private:
    QFuture<T> m_future;
    const QFutureInterfaceBase &futureInterface() const override { return m_future.d; }
    QFutureInterfaceBase &futureInterface() override { return m_future.d; }
};

template <typename T>
Q_INLINE_TEMPLATE void QFutureWatcher<T>::setFuture(const QFuture<T> &_future)
{
    if (_future.d == m_future.d)
        return;

    disconnectOutputInterface(true);
    m_future = _future;
    connectOutputInterface();
}

QT_END_NAMESPACE

#endif // QFUTUREWATCHER_H
