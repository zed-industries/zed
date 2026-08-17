// Copyright (C) 2016 The Qt Company Ltd.
// Copyright (C) 2017 Klarälvdalens Datakonsult AB, a KDAB Group company, info@kdab.com, author Giuseppe D'Angelo <giuseppe.dangelo@kdab.com>
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTHREAD_H
#define QTHREAD_H

#include <QtCore/qobject.h>
#include <QtCore/qdeadlinetimer.h>

// For QThread::create
#if QT_CONFIG(cxx11_future)
#  include <future> // for std::async
#  include <functional> // for std::invoke; no guard needed as it's a C++98 header
#endif
// internal compiler error with mingw 8.1
#if defined(Q_CC_MSVC) && defined(Q_PROCESSOR_X86)
#include <intrin.h>
#endif

QT_BEGIN_NAMESPACE


class QThreadData;
class QThreadPrivate;
class QAbstractEventDispatcher;

class Q_CORE_EXPORT QThread : public QObject
{
    Q_OBJECT
public:
    static Qt::HANDLE currentThreadId() noexcept Q_DECL_PURE_FUNCTION;
    static QThread *currentThread();
    static int idealThreadCount() noexcept;
    static void yieldCurrentThread();

    explicit QThread(QObject *parent = nullptr);
    ~QThread();

    enum Priority {
        IdlePriority,

        LowestPriority,
        LowPriority,
        NormalPriority,
        HighPriority,
        HighestPriority,

        TimeCriticalPriority,

        InheritPriority
    };

    void setPriority(Priority priority);
    Priority priority() const;

    bool isFinished() const;
    bool isRunning() const;

    void requestInterruption();
    bool isInterruptionRequested() const;

    void setStackSize(uint stackSize);
    uint stackSize() const;

    QAbstractEventDispatcher *eventDispatcher() const;
    void setEventDispatcher(QAbstractEventDispatcher *eventDispatcher);

    bool event(QEvent *event) override;
    int loopLevel() const;

#if QT_CONFIG(cxx11_future) || defined(Q_CLANG_QDOC)
    template <typename Function, typename... Args>
    [[nodiscard]] static QThread *create(Function &&f, Args &&... args);
#endif

public Q_SLOTS:
    void start(Priority = InheritPriority);
    void terminate();
    void exit(int retcode = 0);
    void quit();

public:
    bool wait(QDeadlineTimer deadline = QDeadlineTimer(QDeadlineTimer::Forever));
    bool wait(unsigned long time)
    {
        if (time == (std::numeric_limits<unsigned long>::max)())
            return wait(QDeadlineTimer(QDeadlineTimer::Forever));
        return wait(QDeadlineTimer(time));
    }

    static void sleep(unsigned long);
    static void msleep(unsigned long);
    static void usleep(unsigned long);

Q_SIGNALS:
    void started(QPrivateSignal);
    void finished(QPrivateSignal);

protected:
    virtual void run();
    int exec();

    static void setTerminationEnabled(bool enabled = true);

protected:
    QThread(QThreadPrivate &dd, QObject *parent = nullptr);

private:
    Q_DECLARE_PRIVATE(QThread)

#if QT_CONFIG(cxx11_future)
    [[nodiscard]] static QThread *createThreadImpl(std::future<void> &&future);
#endif
    static Qt::HANDLE currentThreadIdImpl() noexcept Q_DECL_PURE_FUNCTION;

    friend class QCoreApplication;
    friend class QThreadData;
};

#if QT_CONFIG(cxx11_future)
template <typename Function, typename... Args>
QThread *QThread::create(Function &&f, Args &&... args)
{
    using DecayedFunction = typename std::decay<Function>::type;
    auto threadFunction =
        [f = static_cast<DecayedFunction>(std::forward<Function>(f))](auto &&... largs) mutable -> void
        {
            (void)std::invoke(std::move(f), std::forward<decltype(largs)>(largs)...);
        };

    return createThreadImpl(std::async(std::launch::deferred,
                                       std::move(threadFunction),
                                       std::forward<Args>(args)...));
}
#endif // QT_CONFIG(cxx11_future)

/*
    On architectures and platforms we know, interpret the thread control
    block (TCB) as a unique identifier for a thread within a process. Otherwise,
    fall back to a slower but safe implementation.

    As per the documentation of currentThreadId, we return an opaque handle
    as a thread identifier, and application code is not supposed to use that
    value for anything. In Qt we use the handle to check if threads are identical,
    for which the TCB is sufficient.

    So we use the fastest possible way, rather than spend time on returning
    some pseudo-interoperable value.
*/
inline Qt::HANDLE QThread::currentThreadId() noexcept
{
    // define is undefed if we have to fall back to currentThreadIdImpl
#define QT_HAS_FAST_CURRENT_THREAD_ID
    Qt::HANDLE tid; // typedef to void*
    static_assert(sizeof(tid) == sizeof(void*));
    // See https://akkadia.org/drepper/tls.pdf for x86 ABI
#if defined(Q_PROCESSOR_X86_32) && ((defined(Q_OS_LINUX) && defined(__GLIBC__)) || defined(Q_OS_FREEBSD)) // x86 32-bit always uses GS
    __asm__("movl %%gs:%c1, %0" : "=r" (tid) : "i" (2 * sizeof(void*)) : );
#elif defined(Q_PROCESSOR_X86_64) && defined(Q_OS_DARWIN64)
    // 64bit macOS uses GS, see https://github.com/apple/darwin-xnu/blob/master/libsyscall/os/tsd.h
    __asm__("movq %%gs:0, %0" : "=r" (tid) : : );
#elif defined(Q_PROCESSOR_X86_64) && ((defined(Q_OS_LINUX) && defined(__GLIBC__)) || defined(Q_OS_FREEBSD))
    // x86_64 Linux, BSD uses FS
    __asm__("movq %%fs:%c1, %0" : "=r" (tid) : "i" (2 * sizeof(void*)) : );
#elif defined(Q_PROCESSOR_X86_64) && defined(Q_OS_WIN)
    // See https://en.wikipedia.org/wiki/Win32_Thread_Information_Block
    // First get the pointer to the TIB
    quint8 *tib;
# if defined(Q_CC_MINGW) // internal compiler error when using the intrinsics
    __asm__("movq %%gs:0x30, %0" : "=r" (tib) : :);
# else
    tib = reinterpret_cast<quint8 *>(__readgsqword(0x30));
# endif
    // Then read the thread ID
    tid = *reinterpret_cast<Qt::HANDLE *>(tib + 0x48);
#elif defined(Q_PROCESSOR_X86_32) && defined(Q_OS_WIN)
    // First get the pointer to the TIB
    quint8 *tib;
# if defined(Q_CC_MINGW) // internal compiler error when using the intrinsics
    __asm__("movl %%fs:0x18, %0" : "=r" (tib) : :);
# else
    tib = reinterpret_cast<quint8 *>(__readfsdword(0x18));
# endif
    // Then read the thread ID
    tid = *reinterpret_cast<Qt::HANDLE *>(tib + 0x24);
#else
#undef QT_HAS_FAST_CURRENT_THREAD_ID
    tid = currentThreadIdImpl();
#endif
    return tid;
}

QT_END_NAMESPACE

#endif // QTHREAD_H
