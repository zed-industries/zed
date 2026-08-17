/****************************************************************************
**
** Copyright (C) 2016 Intel Corporation.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtCore module of the Qt Toolkit.
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

#include <QtCore/qglobal.h>

#ifndef QGLOBALSTATIC_H
#define QGLOBALSTATIC_H

#include <QtCore/qatomic.h>

QT_BEGIN_NAMESPACE

namespace QtGlobalStatic {
enum GuardValues {
    Destroyed = -2,
    Initialized = -1,
    Uninitialized = 0,
    Initializing = 1
};
}

#if !QT_CONFIG(thread) || defined(Q_COMPILER_THREADSAFE_STATICS)
// some compilers support thread-safe statics
// The IA-64 C++ ABI requires this, so we know that all GCC versions since 3.4
// support it. C++11 also requires this behavior.
// Clang and Intel CC masquerade as GCC when compiling on Linux.
//
// Apple's libc++abi however uses a global lock for initializing local statics,
// which will block other threads also trying to initialize a local static
// until the constructor returns ...
// We better avoid these kind of problems by using our own locked implementation.

#if defined(Q_OS_UNIX) && defined(Q_CC_INTEL)
// Work around Intel issue ID 6000058488:
// local statics inside an inline function inside an anonymous namespace are global
// symbols (this affects the IA-64 C++ ABI, so OS X and Linux only)
#  define Q_GLOBAL_STATIC_INTERNAL_DECORATION Q_DECL_HIDDEN
#else
#  define Q_GLOBAL_STATIC_INTERNAL_DECORATION Q_DECL_HIDDEN inline
#endif

#define Q_GLOBAL_STATIC_INTERNAL(ARGS)                          \
    Q_GLOBAL_STATIC_INTERNAL_DECORATION Type *innerFunction()   \
    {                                                           \
        struct HolderBase {                                     \
            ~HolderBase() noexcept                        \
            { if (guard.loadRelaxed() == QtGlobalStatic::Initialized)  \
                  guard.storeRelaxed(QtGlobalStatic::Destroyed); }     \
        };                                                      \
        static struct Holder : public HolderBase {              \
            Type value;                                         \
            Holder()                                            \
                noexcept(noexcept(Type ARGS))       \
                : value ARGS                                    \
            { guard.storeRelaxed(QtGlobalStatic::Initialized); }       \
        } holder;                                               \
        return &holder.value;                                   \
    }
#else
// We don't know if this compiler supports thread-safe global statics
// so use our own locked implementation

QT_END_NAMESPACE
#include <QtCore/qmutex.h>
#include <mutex>
QT_BEGIN_NAMESPACE

#define Q_GLOBAL_STATIC_INTERNAL(ARGS)                                  \
    Q_DECL_HIDDEN inline Type *innerFunction()                          \
    {                                                                   \
        static Type *d;                                                 \
        static QBasicMutex mutex;                                       \
        int x = guard.loadAcquire();                                    \
        if (Q_UNLIKELY(x >= QtGlobalStatic::Uninitialized)) {           \
            const std::lock_guard<QBasicMutex> locker(mutex);           \
            if (guard.loadRelaxed() == QtGlobalStatic::Uninitialized) {        \
                d = new Type ARGS;                                      \
                static struct Cleanup {                                 \
                    ~Cleanup() {                                        \
                        delete d;                                       \
                        guard.storeRelaxed(QtGlobalStatic::Destroyed);         \
                    }                                                   \
                } cleanup;                                              \
                guard.storeRelease(QtGlobalStatic::Initialized);        \
            }                                                           \
        }                                                               \
        return d;                                                       \
    }
#endif

// this class must be POD, unless the compiler supports thread-safe statics
template <typename T, T *(&innerFunction)(), QBasicAtomicInt &guard>
struct QGlobalStatic
{
    typedef T Type;

    bool isDestroyed() const { return guard.loadRelaxed() <= QtGlobalStatic::Destroyed; }
    bool exists() const { return guard.loadRelaxed() == QtGlobalStatic::Initialized; }
    operator Type *() { if (isDestroyed()) return nullptr; return innerFunction(); }
    Type *operator()() { if (isDestroyed()) return nullptr; return innerFunction(); }
    Type *operator->()
    {
      Q_ASSERT_X(!isDestroyed(), "Q_GLOBAL_STATIC", "The global static was used after being destroyed");
      return innerFunction();
    }
    Type &operator*()
    {
      Q_ASSERT_X(!isDestroyed(), "Q_GLOBAL_STATIC", "The global static was used after being destroyed");
      return *innerFunction();
    }
};

#define Q_GLOBAL_STATIC_WITH_ARGS(TYPE, NAME, ARGS)                         \
    namespace { namespace Q_QGS_ ## NAME {                                  \
        typedef TYPE Type;                                                  \
        QBasicAtomicInt guard = Q_BASIC_ATOMIC_INITIALIZER(QtGlobalStatic::Uninitialized); \
        Q_GLOBAL_STATIC_INTERNAL(ARGS)                                      \
    } }                                                                     \
    static QGlobalStatic<TYPE,                                              \
                         Q_QGS_ ## NAME::innerFunction,                     \
                         Q_QGS_ ## NAME::guard> NAME;

#define Q_GLOBAL_STATIC(TYPE, NAME)                                         \
    Q_GLOBAL_STATIC_WITH_ARGS(TYPE, NAME, ())

QT_END_NAMESPACE
#endif // QGLOBALSTATIC_H
