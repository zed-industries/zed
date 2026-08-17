// Copyright (C) 2021 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QGUIAPPLICATION_PLATFORM_H
#define QGUIAPPLICATION_PLATFORM_H

//
//  W A R N I N G
//  -------------
//
// This file is part of the native interface APIs. Usage of
// this API may make your code source and binary incompatible
// with future versions of Qt.
//

#include <QtGui/qtguiglobal.h>

#include <QtCore/qnativeinterface.h>
#include <QtGui/qguiapplication.h>

#if QT_CONFIG(xcb)
typedef struct _XDisplay Display;
struct xcb_connection_t;
#endif

QT_BEGIN_NAMESPACE

namespace QNativeInterface
{

#if QT_CONFIG(xcb) || defined(Q_CLANG_QDOC)
struct Q_GUI_EXPORT QX11Application
{
    QT_DECLARE_NATIVE_INTERFACE(QX11Application, 1, QGuiApplication)
    virtual Display *display() const = 0;
    virtual xcb_connection_t *connection() const = 0;
};
#endif

} // QNativeInterface

QT_END_NAMESPACE

#endif // QGUIAPPLICATION_PLATFORM_H
