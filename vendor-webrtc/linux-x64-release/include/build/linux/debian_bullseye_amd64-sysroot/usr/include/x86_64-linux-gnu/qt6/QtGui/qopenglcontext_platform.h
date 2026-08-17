// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QOPENGLCONTEXT_PLATFORM_H
#define QOPENGLCONTEXT_PLATFORM_H

//
//  W A R N I N G
//  -------------
//
// This file is part of the native interface APIs. Usage of
// this API may make your code source and binary incompatible
// with future versions of Qt.
//

#ifndef QT_NO_OPENGL

#include <QtGui/qtguiglobal.h>
#include <QtGui/qopenglcontext.h>
#include <QtGui/qwindowdefs.h>

#include <QtCore/qnativeinterface.h>

#if defined(Q_OS_MACOS)
Q_FORWARD_DECLARE_OBJC_CLASS(NSOpenGLContext);
#endif

#if QT_CONFIG(xcb_glx_plugin)
struct __GLXcontextRec; typedef struct __GLXcontextRec *GLXContext;
#endif
#if QT_CONFIG(egl)
typedef void *EGLContext;
typedef void *EGLDisplay;
typedef void *EGLConfig;
#endif

#if !defined(Q_OS_MACOS) && defined(Q_CLANG_QDOC)
typedef void *NSOpenGLContext;
#endif

QT_BEGIN_NAMESPACE

namespace QNativeInterface {

#if defined(Q_OS_MACOS) || defined(Q_CLANG_QDOC)
struct Q_GUI_EXPORT QCocoaGLContext
{
    QT_DECLARE_NATIVE_INTERFACE(QCocoaGLContext, 1, QOpenGLContext)
    static QOpenGLContext *fromNative(QT_IGNORE_DEPRECATIONS(NSOpenGLContext) *context, QOpenGLContext *shareContext = nullptr);
    virtual QT_IGNORE_DEPRECATIONS(NSOpenGLContext) *nativeContext() const = 0;
};
#endif

#if defined(Q_OS_WIN) || defined(Q_CLANG_QDOC)
struct Q_GUI_EXPORT QWGLContext
{
    QT_DECLARE_NATIVE_INTERFACE(QWGLContext, 1, QOpenGLContext)
    static HMODULE openGLModuleHandle();
    static QOpenGLContext *fromNative(HGLRC context, HWND window, QOpenGLContext *shareContext = nullptr);
    virtual HGLRC nativeContext() const = 0;
};
#endif

#if QT_CONFIG(xcb_glx_plugin) || defined(Q_CLANG_QDOC)
struct Q_GUI_EXPORT QGLXContext
{
    QT_DECLARE_NATIVE_INTERFACE(QGLXContext, 1, QOpenGLContext)
    static QOpenGLContext *fromNative(GLXContext configBasedContext, QOpenGLContext *shareContext = nullptr);
    static QOpenGLContext *fromNative(GLXContext visualBasedContext, void *visualInfo, QOpenGLContext *shareContext = nullptr);
    virtual GLXContext nativeContext() const = 0;
};
#endif

#if QT_CONFIG(egl) || defined(Q_CLANG_QDOC)
struct Q_GUI_EXPORT QEGLContext
{
    QT_DECLARE_NATIVE_INTERFACE(QEGLContext, 1, QOpenGLContext)
    static QOpenGLContext *fromNative(EGLContext context, EGLDisplay display, QOpenGLContext *shareContext = nullptr);
    virtual EGLContext nativeContext() const = 0;
    virtual EGLConfig config() const = 0;
    virtual EGLDisplay display() const = 0;
};
#endif

} // QNativeInterface

QT_END_NAMESPACE

#endif // QT_NO_OPENGL

#endif // QOPENGLCONTEXT_PLATFORM_H
