// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QOFFSCREENSURFACE_PLATFORM_H
#define QOFFSCREENSURFACE_PLATFORM_H

//
//  W A R N I N G
//  -------------
//
// This file is part of the native interface APIs. Usage of
// this API may make your code source and binary incompatible
// with future versions of Qt.
//

#include <QtGui/qtguiglobal.h>
#include <QtGui/qoffscreensurface.h>
#include <QtCore/qnativeinterface.h>

#if defined(Q_OS_ANDROID)
struct ANativeWindow;
#endif

QT_BEGIN_NAMESPACE

namespace QNativeInterface {

#if defined(Q_OS_ANDROID) || defined(Q_CLANG_QDOC)
struct Q_GUI_EXPORT QAndroidOffscreenSurface
{
    QT_DECLARE_NATIVE_INTERFACE(QAndroidOffscreenSurface, 1, QOffscreenSurface)
    static QOffscreenSurface *fromNative(ANativeWindow *nativeSurface);
    virtual ANativeWindow *nativeSurface() const = 0;
};
#endif

} // QNativeInterface

QT_END_NAMESPACE

#endif // QOFFSCREENSURFACE_PLATFORM_H
