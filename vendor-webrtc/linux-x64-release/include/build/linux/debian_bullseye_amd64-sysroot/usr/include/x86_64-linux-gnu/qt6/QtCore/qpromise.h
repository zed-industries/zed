// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QPROMISE_H
#define QPROMISE_H

#include <QtCore/qglobal.h>
#include <QtCore/qfutureinterface.h>

#include <utility>

QT_REQUIRE_CONFIG(future);

QT_BEGIN_NAMESPACE

namespace QtPrivate {

template<class T, class U>
using EnableIfSameOrConvertible = std::enable_if_t<std::is_convertible_v<T, U>>;

} // namespace QtPrivate

template<typename T>
class QPromise
{
    static_assert (std::is_move_constructible_v<T>
                   || std::is_same_v<T, void>,
                   "A move-constructible type or type void is required");
public:
    QPromise() = default;
    Q_DISABLE_COPY(QPromise)
    QPromise(QPromise<T> &&other) = default;
    QPromise(const QFutureInterface<T> &other) : d(other) {}
    QPromise(QFutureInterface<T> &&other) noexcept : d(std::move(other)) {}
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_MOVE_AND_SWAP(QPromise)
    ~QPromise()
    {
        // If computation is not finished at this point, cancel
        // potential waits
        if (d.d && !(d.loadState() & QFutureInterfaceBase::State::Finished)) {
            d.cancelAndFinish(); // cancel and finalize the state
            d.runContinuation();
        }
        d.cleanContinuation();
    }

    // Core QPromise APIs
    QFuture<T> future() const { return d.future(); }
    template<typename U, typename = QtPrivate::EnableIfSameOrConvertible<U, T>>
    bool addResult(U &&result, int index = -1)
    {
        return d.reportResult(std::forward<U>(result), index);
    }
#ifndef QT_NO_EXCEPTIONS
    void setException(const QException &e) { d.reportException(e); }
#if QT_VERSION < QT_VERSION_CHECK(7, 0, 0)
    void setException(std::exception_ptr e) { d.reportException(e); }
#else
    void setException(const std::exception_ptr &e) { d.reportException(e); }
#endif
#endif
    void start() { d.reportStarted(); }
    void finish() { d.reportFinished(); }

    void suspendIfRequested() { d.suspendIfRequested(); }

    bool isCanceled() const { return d.isCanceled(); }

    // Progress methods
    void setProgressRange(int minimum, int maximum) { d.setProgressRange(minimum, maximum); }
    void setProgressValue(int progressValue) { d.setProgressValue(progressValue); }
    void setProgressValueAndText(int progressValue, const QString &progressText)
    {
        d.setProgressValueAndText(progressValue, progressText);
    }

    void swap(QPromise<T> &other) noexcept
    {
        d.swap(other.d);
    }

#if defined(Q_CLANG_QDOC)  // documentation-only simplified signatures
    bool addResult(const T &result, int index = -1) { }
    bool addResult(T &&result, int index = -1) { }
#endif
private:
    mutable QFutureInterface<T> d;
};

template<typename T>
inline void swap(QPromise<T> &a, QPromise<T> &b) noexcept
{
    a.swap(b);
}

QT_END_NAMESPACE

#endif  // QPROMISE_H
