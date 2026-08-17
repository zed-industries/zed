/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
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

#ifndef QFUNCTIONS_WINRT_H
#define QFUNCTIONS_WINRT_H

#include <QtCore/qglobal.h>

#ifdef Q_OS_WIN

#include <QtCore/QCoreApplication>
#include <QtCore/QThread>
#include <QtCore/QAbstractEventDispatcher>
#include <QtCore/QElapsedTimer>
#include <QtCore/qt_windows.h>

// Convenience macros for handling HRESULT values
#define RETURN_IF_FAILED(msg, ret) \
    if (FAILED(hr)) { \
        qErrnoWarning(hr, msg); \
        ret; \
    }

#define RETURN_IF_FAILED_WITH_ARGS(msg, ret, ...) \
    if (FAILED(hr)) { \
        qErrnoWarning(hr, msg, __VA_ARGS__); \
        ret; \
    }

#define RETURN_HR_IF_FAILED(msg) RETURN_IF_FAILED(msg, return hr)
#define RETURN_OK_IF_FAILED(msg) RETURN_IF_FAILED(msg, return S_OK)
#define RETURN_FALSE_IF_FAILED(msg) RETURN_IF_FAILED(msg, return false)
#define RETURN_VOID_IF_FAILED(msg) RETURN_IF_FAILED(msg, return)
#define RETURN_HR_IF_FAILED_WITH_ARGS(msg, ...) RETURN_IF_FAILED_WITH_ARGS(msg, return hr, __VA_ARGS__)
#define RETURN_OK_IF_FAILED_WITH_ARGS(msg, ...) RETURN_IF_FAILED_WITH_ARGS(msg, return S_OK, __VA_ARGS__)
#define RETURN_FALSE_IF_FAILED_WITH_ARGS(msg, ...) RETURN_IF_FAILED_WITH_ARGS(msg, return false, __VA_ARGS__)
#define RETURN_VOID_IF_FAILED_WITH_ARGS(msg, ...) RETURN_IF_FAILED_WITH_ARGS(msg, return, __VA_ARGS__)

#define Q_ASSERT_SUCCEEDED(hr) \
    Q_ASSERT_X(SUCCEEDED(hr), Q_FUNC_INFO, qPrintable(qt_error_string(hr)));

#ifdef Q_OS_WINRT

QT_BEGIN_NAMESPACE

#ifdef QT_BUILD_CORE_LIB
#endif

// Environment ------------------------------------------------------
errno_t qt_fake_getenv_s(size_t*, char*, size_t, const char*);
errno_t qt_fake__putenv_s(const char*, const char*);
void qt_winrt_tzset();
void qt_winrt__tzset();

QT_END_NAMESPACE

// As Windows Runtime lacks some standard functions used in Qt, these got
// reimplemented. Other projects do this as well. Inline functions are used
// that there is a central place to disable functions for newer versions if
// they get available. There are no defines used anymore, because this
// will break member functions of classes which are called like these
// functions.
// The other declarations available in this file are being used per
// define inside qplatformdefs.h of the corresponding WinRT mkspec.

#define generate_inline_return_func0(funcname, returntype) \
        inline returntype funcname() \
        { \
            return QT_PREPEND_NAMESPACE(qt_winrt_##funcname)(); \
        }
#define generate_inline_return_func1(funcname, returntype, param1) \
        inline returntype funcname(param1 p1) \
        { \
            return QT_PREPEND_NAMESPACE(qt_winrt_##funcname)(p1); \
        }
#define generate_inline_return_func2(funcname, returntype, prependnamespace, param1, param2) \
        inline returntype funcname(param1 p1, param2 p2) \
        { \
            return QT_PREPEND_NAMESPACE(prependnamespace##funcname)(p1,  p2); \
        }
#define generate_inline_return_func3(funcname, returntype, param1, param2, param3) \
        inline returntype funcname(param1 p1, param2 p2, param3 p3) \
        { \
            return QT_PREPEND_NAMESPACE(qt_winrt_##funcname)(p1,  p2, p3); \
        }
#define generate_inline_return_func4(funcname, returntype, prependnamespace, param1, param2, param3, param4) \
        inline returntype funcname(param1 p1, param2 p2, param3 p3, param4 p4) \
        { \
            return QT_PREPEND_NAMESPACE(prependnamespace##funcname)(p1,  p2, p3, p4); \
        }
#define generate_inline_return_func5(funcname, returntype, param1, param2, param3, param4, param5) \
        inline returntype funcname(param1 p1, param2 p2, param3 p3, param4 p4, param5 p5) \
        { \
            return QT_PREPEND_NAMESPACE(qt_winrt_##funcname)(p1,  p2, p3, p4, p5); \
        }
#define generate_inline_return_func6(funcname, returntype, param1, param2, param3, param4, param5, param6) \
        inline returntype funcname(param1 p1, param2 p2, param3 p3, param4 p4, param5 p5, param6 p6) \
        { \
            return QT_PREPEND_NAMESPACE(qt_winrt_##funcname)(p1,  p2, p3, p4, p5, p6); \
        }
#define generate_inline_return_func7(funcname, returntype, param1, param2, param3, param4, param5, param6, param7) \
        inline returntype funcname(param1 p1, param2 p2, param3 p3, param4 p4, param5 p5, param6 p6, param7 p7) \
        { \
            return QT_PREPEND_NAMESPACE(qt_winrt_##funcname)(p1,  p2, p3, p4, p5, p6, p7); \
        }

typedef unsigned (__stdcall *StartAdressExFunc)(void *);
typedef void(*StartAdressFunc)(void *);
typedef int ( __cdecl *CompareFunc ) (const void *, const void *) ;

generate_inline_return_func4(getenv_s, errno_t, qt_fake_, size_t *, char *, size_t, const char *)
generate_inline_return_func2(_putenv_s, errno_t, qt_fake_, const char *, const char *)
generate_inline_return_func0(tzset, void)
generate_inline_return_func0(_tzset, void)

namespace Microsoft {
    namespace WRL {
        template <typename T> class ComPtr;
    }
}

QT_BEGIN_NAMESPACE

namespace QWinRTFunctions {

// Synchronization methods
enum AwaitStyle
{
    YieldThread = 0,
    ProcessThreadEvents = 1,
    ProcessMainThreadEvents = 2
};

template <typename T>
static inline HRESULT _await_impl(const Microsoft::WRL::ComPtr<T> &asyncOp, AwaitStyle awaitStyle, uint timeout)
{
    Microsoft::WRL::ComPtr<IAsyncInfo> asyncInfo;
    HRESULT hr = asyncOp.As(&asyncInfo);
    if (FAILED(hr))
        return hr;

    AsyncStatus status;
    QElapsedTimer t;
    if (timeout)
        t.start();
    switch (awaitStyle) {
    case ProcessMainThreadEvents:
        while (SUCCEEDED(hr = asyncInfo->get_Status(&status)) && status == AsyncStatus::Started) {
            QCoreApplication::processEvents();
            if (timeout && t.hasExpired(timeout))
                return ERROR_TIMEOUT;
        }
        break;
    case ProcessThreadEvents:
        if (QAbstractEventDispatcher *dispatcher = QThread::currentThread()->eventDispatcher()) {
            while (SUCCEEDED(hr = asyncInfo->get_Status(&status)) && status == AsyncStatus::Started) {
                dispatcher->processEvents(QEventLoop::AllEvents);
                if (timeout && t.hasExpired(timeout))
                    return ERROR_TIMEOUT;
            }
            break;
        }
        // fall through
    default:
    case YieldThread:
        while (SUCCEEDED(hr = asyncInfo->get_Status(&status)) && status == AsyncStatus::Started) {
            QThread::yieldCurrentThread();
            if (timeout && t.hasExpired(timeout))
                return ERROR_TIMEOUT;
        }
        break;
    }

    if (FAILED(hr) || status != AsyncStatus::Completed) {
        HRESULT ec;
        hr = asyncInfo->get_ErrorCode(&ec);
        if (FAILED(hr))
            return hr;
        hr = asyncInfo->Close();
        if (FAILED(hr))
            return hr;
        return ec;
    }

    return hr;
}

template <typename T>
static inline HRESULT await(const Microsoft::WRL::ComPtr<T> &asyncOp, AwaitStyle awaitStyle = YieldThread, uint timeout = 0)
{
    HRESULT hr = _await_impl(asyncOp, awaitStyle, timeout);
    if (FAILED(hr))
        return hr;

    return asyncOp->GetResults();
}

template <typename T, typename U>
static inline HRESULT await(const Microsoft::WRL::ComPtr<T> &asyncOp, U *results, AwaitStyle awaitStyle = YieldThread, uint timeout = 0)
{
    HRESULT hr = _await_impl(asyncOp, awaitStyle, timeout);
    if (FAILED(hr))
        return hr;

    return asyncOp->GetResults(results);
}

} // QWinRTFunctions

QT_END_NAMESPACE

#endif // Q_OS_WINRT

#endif // Q_OS_WIN

#endif // QFUNCTIONS_WINRT_H
