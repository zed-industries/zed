// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSEMAPHORE_H
#define QSEMAPHORE_H

#include <QtCore/qglobal.h>
#include <QtCore/qmutex.h> // for convertToMilliseconds()

#include <chrono>

QT_REQUIRE_CONFIG(thread);

QT_BEGIN_NAMESPACE

class QSemaphorePrivate;

class Q_CORE_EXPORT QSemaphore
{
public:
    explicit QSemaphore(int n = 0);
    ~QSemaphore();

    void acquire(int n = 1);
    bool tryAcquire(int n = 1);
    bool tryAcquire(int n, int timeout);
    template <typename Rep, typename Period>
    bool tryAcquire(int n, std::chrono::duration<Rep, Period> timeout)
    { return tryAcquire(n, QtPrivate::convertToMilliseconds(timeout)); }

    void release(int n = 1);

    int available() const;

    // std::counting_semaphore compatibility:
    bool try_acquire() noexcept { return tryAcquire(); }
    template <typename Rep, typename Period>
    bool try_acquire_for(const std::chrono::duration<Rep, Period> &timeout)
    { return tryAcquire(1, timeout); }
    template <typename Clock, typename Duration>
    bool try_acquire_until(const std::chrono::time_point<Clock, Duration> &tp)
    {
        return try_acquire_for(tp - Clock::now());
    }
private:
    Q_DISABLE_COPY(QSemaphore)

    union {
        QSemaphorePrivate *d;
        QBasicAtomicInteger<quintptr> u;
        QBasicAtomicInteger<quint32> u32[2];
        QBasicAtomicInteger<quint64> u64;
    };
};

class QSemaphoreReleaser
{
public:
    QSemaphoreReleaser() = default;
    explicit QSemaphoreReleaser(QSemaphore &sem, int n = 1) noexcept
        : m_sem(&sem), m_n(n) {}
    explicit QSemaphoreReleaser(QSemaphore *sem, int n = 1) noexcept
        : m_sem(sem), m_n(n) {}
    QSemaphoreReleaser(QSemaphoreReleaser &&other) noexcept
        : m_sem(other.cancel()), m_n(other.m_n) {}
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_MOVE_AND_SWAP(QSemaphoreReleaser)

    ~QSemaphoreReleaser()
    {
        if (m_sem)
            m_sem->release(m_n);
    }

    void swap(QSemaphoreReleaser &other) noexcept
    {
        qt_ptr_swap(m_sem, other.m_sem);
        std::swap(m_n, other.m_n);
    }

    QSemaphore *semaphore() const noexcept
    { return m_sem; }

    QSemaphore *cancel() noexcept
    {
        return qExchange(m_sem, nullptr);
    }

private:
    QSemaphore *m_sem = nullptr;
    int m_n;
};

QT_END_NAMESPACE

#endif // QSEMAPHORE_H
