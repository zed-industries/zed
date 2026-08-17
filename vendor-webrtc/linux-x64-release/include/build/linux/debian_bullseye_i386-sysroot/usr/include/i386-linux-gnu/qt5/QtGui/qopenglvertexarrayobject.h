/****************************************************************************
**
** Copyright (C) 2014 Klarälvdalens Datakonsult AB, a KDAB Group company, info@kdab.com, author Sean Harmer <sean.harmer@kdab.com>
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtGui module of the Qt Toolkit.
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

#ifndef QOPENGLVERTEXARRAYOBJECT_H
#define QOPENGLVERTEXARRAYOBJECT_H

#include <QtGui/qtguiglobal.h>

#ifndef QT_NO_OPENGL

#include <QtCore/QObject>
#include <QtGui/qopengl.h>

QT_BEGIN_NAMESPACE

class QOpenGLVertexArrayObjectPrivate;

class Q_GUI_EXPORT QOpenGLVertexArrayObject : public QObject
{
    Q_OBJECT

public:
    explicit QOpenGLVertexArrayObject(QObject* parent = nullptr);
    ~QOpenGLVertexArrayObject();

    bool create();
    void destroy();
    bool isCreated() const;
    GLuint objectId() const;
    void bind();
    void release();

    class Q_GUI_EXPORT Binder
    {
    public:
        inline Binder(QOpenGLVertexArrayObject *v)
            : vao(v)
        {
            Q_ASSERT(v);
            if (vao->isCreated() || vao->create())
                vao->bind();
        }

        inline ~Binder()
        {
            release();
        }

        inline void release()
        {
            vao->release();
        }

        inline void rebind()
        {
            vao->bind();
        }

    private:
        Q_DISABLE_COPY(Binder)
        QOpenGLVertexArrayObject *vao;
    };

private:
    Q_DISABLE_COPY(QOpenGLVertexArrayObject)
    Q_DECLARE_PRIVATE(QOpenGLVertexArrayObject)
    Q_PRIVATE_SLOT(d_func(), void _q_contextAboutToBeDestroyed())
    QOpenGLVertexArrayObject(QOpenGLVertexArrayObjectPrivate &dd);
};

QT_END_NAMESPACE

#endif

#endif // QOPENGLVERTEXARRAYOBJECT_H
