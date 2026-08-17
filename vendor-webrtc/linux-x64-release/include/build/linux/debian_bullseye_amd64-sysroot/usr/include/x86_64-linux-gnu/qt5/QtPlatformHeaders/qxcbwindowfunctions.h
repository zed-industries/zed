/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the plugins of the Qt Toolkit.
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

#ifndef QXCBWINDOWFUNCTIONS_H
#define QXCBWINDOWFUNCTIONS_H

#include <QtPlatformHeaders/QPlatformHeaderHelper>

QT_BEGIN_NAMESPACE

class QWindow;

class QXcbWindowFunctions {
public:
    enum WmWindowType {
        Normal       = 0x000001,
        Desktop      = 0x000002,
        Dock         = 0x000004,
        Toolbar      = 0x000008,
        Menu         = 0x000010,
        Utility      = 0x000020,
        Splash       = 0x000040,
        Dialog       = 0x000080,
        DropDownMenu = 0x000100,
        PopupMenu    = 0x000200,
        Tooltip      = 0x000400,
        Notification = 0x000800,
        Combo        = 0x001000,
        Dnd          = 0x002000,
        KdeOverride  = 0x004000
    };

    Q_DECLARE_FLAGS(WmWindowTypes, WmWindowType)

    typedef void (*SetWmWindowType)(QWindow *window, QXcbWindowFunctions::WmWindowTypes windowType);
    static const QByteArray setWmWindowTypeIdentifier() { return QByteArrayLiteral("XcbSetWmWindowType"); }
    static void setWmWindowType(QWindow *window, WmWindowType type)
    {
        return QPlatformHeaderHelper::callPlatformFunction<void, SetWmWindowType, QWindow *, WmWindowType>(setWmWindowTypeIdentifier(), window, type);
    }

    typedef void (*SetWmWindowRole)(QWindow *window, const QByteArray &role);
    static const QByteArray setWmWindowRoleIdentifier() { return QByteArrayLiteral("XcbSetWmWindowRole"); }

    static void setWmWindowRole(QWindow *window, const QByteArray &role)
    {
        return QPlatformHeaderHelper::callPlatformFunction<void, SetWmWindowRole, QWindow *, const QByteArray &>(setWmWindowRoleIdentifier(), window, role);
    }

    typedef void (*SetWmWindowIconText)(QWindow *window, const QString &text);
    static const QByteArray setWmWindowIconTextIdentifier() { return QByteArrayLiteral("XcbSetWmWindowIconText"); }
    static void setWmWindowIconText(QWindow *window, const QString &text)
    {
        return QPlatformHeaderHelper::callPlatformFunction<void, SetWmWindowIconText, QWindow *, const QString &>(setWmWindowIconTextIdentifier(), window, text);
    }

    typedef uint (*VisualId)(QWindow *window);
    static const QByteArray visualIdIdentifier() { return QByteArrayLiteral("XcbVisualId"); }

    static uint visualId(QWindow *window)
    {
        QXcbWindowFunctions::VisualId func = reinterpret_cast<VisualId>(QGuiApplication::platformFunction(visualIdIdentifier()));
        if (func)
            return func(window);
        return UINT_MAX;
    }
};


QT_END_NAMESPACE

#endif // QXCBWINDOWFUNCTIONS_H
