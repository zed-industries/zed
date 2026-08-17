// Copyright (C) 2021 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QCOREAPPLICATION_PLATFORM_H
#define QCOREAPPLICATION_PLATFORM_H

//
//  W A R N I N G
//  -------------
//
// This file is part of the native interface APIs. Usage of
// this API may make your code source and binary incompatible
// with future versions of Qt.
//

#include <QtCore/qglobal.h>
#include <QtCore/qnativeinterface.h>
#include <QtCore/qcoreapplication.h>

#if defined(Q_OS_ANDROID) || defined(Q_CLANG_QDOC)
#include <QtCore/qjnitypes.h>
#if QT_CONFIG(future) && !defined(QT_NO_QOBJECT)
#include <QtCore/qfuture.h>
#include <QtCore/qvariant.h>
#endif
#endif // #if defined(Q_OS_ANDROID) || defined(Q_CLANG_QDOC)

#if defined(Q_OS_ANDROID)
class _jobject;
typedef _jobject* jobject;
#endif

QT_BEGIN_NAMESPACE

#if defined(Q_OS_ANDROID)
Q_DECLARE_JNI_TYPE(Context, "Landroid/content/Context;")
#endif

namespace QNativeInterface
{
#if defined(Q_OS_ANDROID) || defined(Q_CLANG_QDOC)
struct Q_CORE_EXPORT QAndroidApplication
{
    QT_DECLARE_NATIVE_INTERFACE(QAndroidApplication, 1, QCoreApplication)
#ifdef Q_CLANG_QDOC
    static jobject context();
#else
    static QtJniTypes::Context context();
#endif
    static bool isActivityContext();
    static int sdkVersion();
    static void hideSplashScreen(int duration = 0);

#if QT_CONFIG(future) && !defined(QT_NO_QOBJECT)
    static QFuture<QVariant> runOnAndroidMainThread(const std::function<QVariant()> &runnable,
                                            const QDeadlineTimer timeout = QDeadlineTimer::Forever);

    template <class T>
    std::enable_if_t<std::is_invocable_v<T> && std::is_same_v<std::invoke_result_t<T>, void>,
    QFuture<void>> static runOnAndroidMainThread(const T &runnable,
                                            const QDeadlineTimer timeout = QDeadlineTimer::Forever)
    {
        std::function<QVariant()> func = [runnable](){ runnable(); return QVariant(); };
        return static_cast<QFuture<void>>(runOnAndroidMainThread(func, timeout));
    }
#endif
};
#endif
}

QT_END_NAMESPACE

#endif // QCOREAPPLICATION_PLATFORM_H
