// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTCONCURRENT_RUNBASE_H
#define QTCONCURRENT_RUNBASE_H

#include <QtConcurrent/qtconcurrent_global.h>

#ifndef QT_NO_CONCURRENT

#include <QtCore/qfuture.h>
#include <QtCore/qrunnable.h>
#include <QtCore/qthreadpool.h>

#include <type_traits>
#include <utility>

QT_BEGIN_NAMESPACE


#ifndef Q_QDOC

namespace QtConcurrent {

template <typename T>
struct SelectSpecialization
{
    template <class Normal, class Void>
    struct Type { typedef Normal type; };
};

template <>
struct SelectSpecialization<void>
{
    template <class Normal, class Void>
    struct Type { typedef Void type; };
};

struct TaskStartParameters
{
    QThreadPool *threadPool = QThreadPool::globalInstance();
    int priority = 0;
};

template <typename T>
class RunFunctionTaskBase : public QRunnable
{
public:
    QFuture<T> start()
    {
        return start(TaskStartParameters());
    }

    QFuture<T> start(const TaskStartParameters &parameters)
    {
        promise.setThreadPool(parameters.threadPool);
        promise.setRunnable(this);
        promise.reportStarted();
        QFuture<T> theFuture = promise.future();

        if (parameters.threadPool) {
            parameters.threadPool->start(this, parameters.priority);
        } else {
            promise.reportCanceled();
            promise.reportFinished();
            delete this;
        }
        return theFuture;
    }

    // For backward compatibility
    QFuture<T> start(QThreadPool *pool) { return start({pool, 0});  }

    void run() override
    {
        if (promise.isCanceled()) {
            promise.reportFinished();
            return;
        }
#ifndef QT_NO_EXCEPTIONS
        try {
#endif
            runFunctor();
#ifndef QT_NO_EXCEPTIONS
        } catch (QException &e) {
            promise.reportException(e);
        } catch (...) {
            promise.reportException(QUnhandledException(std::current_exception()));
        }
#endif
        promise.reportFinished();
    }

protected:
    virtual void runFunctor() = 0;

    QFutureInterface<T> promise;
};

} //namespace QtConcurrent

#endif //Q_QDOC

QT_END_NAMESPACE

#endif // QT_NO_CONCURRENT

#endif
