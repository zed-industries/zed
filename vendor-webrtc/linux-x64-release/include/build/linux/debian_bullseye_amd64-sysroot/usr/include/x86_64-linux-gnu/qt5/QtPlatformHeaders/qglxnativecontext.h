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

#ifndef QGLXNATIVECONTEXT_H
#define QGLXNATIVECONTEXT_H

#include <QtCore/QMetaType>
#include <X11/Xlib.h>
#include <GL/glx.h>

QT_BEGIN_NAMESPACE

#if defined(Q_CLANG_QDOC)
typedef int GLXContext;
typedef void Display;
typedef int Window;
typedef int VisualID;
#endif

struct QGLXNativeContext
{
    QGLXNativeContext()
        : m_context(nullptr),
          m_display(nullptr),
          m_window(0),
          m_visualId(0)
    { }

    QGLXNativeContext(GLXContext ctx, Display *dpy = nullptr, Window wnd = 0, VisualID vid = 0)
        : m_context(ctx),
          m_display(dpy),
          m_window(wnd),
          m_visualId(vid)
    { }

    GLXContext context() const { return m_context; }
    Display *display() const { return m_display; }
    Window window() const { return m_window; }
    VisualID visualId() const { return m_visualId; }

private:
    GLXContext m_context;
    Display *m_display;
    Window m_window;
    VisualID m_visualId;
};

QT_END_NAMESPACE

Q_DECLARE_METATYPE(QGLXNativeContext)

#endif // QGLXNATIVECONTEXT_H
