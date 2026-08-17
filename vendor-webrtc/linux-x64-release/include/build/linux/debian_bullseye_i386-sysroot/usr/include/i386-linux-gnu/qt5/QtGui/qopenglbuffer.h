/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
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

#ifndef QOPENGLBUFFER_H
#define QOPENGLBUFFER_H

#include <QtGui/qtguiglobal.h>

#ifndef QT_NO_OPENGL

#include <QtCore/qscopedpointer.h>
#include <QtGui/qopengl.h>

QT_BEGIN_NAMESPACE


class QOpenGLBufferPrivate;

class Q_GUI_EXPORT QOpenGLBuffer
{
public:
    enum Type
    {
        VertexBuffer        = 0x8892, // GL_ARRAY_BUFFER
        IndexBuffer         = 0x8893, // GL_ELEMENT_ARRAY_BUFFER
        PixelPackBuffer     = 0x88EB, // GL_PIXEL_PACK_BUFFER
        PixelUnpackBuffer   = 0x88EC  // GL_PIXEL_UNPACK_BUFFER
    };

    QOpenGLBuffer();
    explicit QOpenGLBuffer(QOpenGLBuffer::Type type);
    QOpenGLBuffer(const QOpenGLBuffer &other);
    ~QOpenGLBuffer();

    QOpenGLBuffer &operator=(const QOpenGLBuffer &other);

    enum UsagePattern
    {
        StreamDraw          = 0x88E0, // GL_STREAM_DRAW
        StreamRead          = 0x88E1, // GL_STREAM_READ
        StreamCopy          = 0x88E2, // GL_STREAM_COPY
        StaticDraw          = 0x88E4, // GL_STATIC_DRAW
        StaticRead          = 0x88E5, // GL_STATIC_READ
        StaticCopy          = 0x88E6, // GL_STATIC_COPY
        DynamicDraw         = 0x88E8, // GL_DYNAMIC_DRAW
        DynamicRead         = 0x88E9, // GL_DYNAMIC_READ
        DynamicCopy         = 0x88EA  // GL_DYNAMIC_COPY
    };

    enum Access
    {
        ReadOnly            = 0x88B8, // GL_READ_ONLY
        WriteOnly           = 0x88B9, // GL_WRITE_ONLY
        ReadWrite           = 0x88BA  // GL_READ_WRITE
    };

    enum RangeAccessFlag
    {
        RangeRead             = 0x0001, // GL_MAP_READ_BIT
        RangeWrite            = 0x0002, // GL_MAP_WRITE_BIT
        RangeInvalidate       = 0x0004, // GL_MAP_INVALIDATE_RANGE_BIT
        RangeInvalidateBuffer = 0x0008, // GL_MAP_INVALIDATE_BUFFER_BIT
        RangeFlushExplicit    = 0x0010, // GL_MAP_FLUSH_EXPLICIT_BIT
        RangeUnsynchronized   = 0x0020  // GL_MAP_UNSYNCHRONIZED_BIT
    };
    Q_DECLARE_FLAGS(RangeAccessFlags, RangeAccessFlag)

    QOpenGLBuffer::Type type() const;

    QOpenGLBuffer::UsagePattern usagePattern() const;
    void setUsagePattern(QOpenGLBuffer::UsagePattern value);

    bool create();
    bool isCreated() const;

    void destroy();

    bool bind();
    void release();

    static void release(QOpenGLBuffer::Type type);

    GLuint bufferId() const;

    int size() const;

    bool read(int offset, void *data, int count);
    void write(int offset, const void *data, int count);

    void allocate(const void *data, int count);
    inline void allocate(int count) { allocate(nullptr, count); }

    void *map(QOpenGLBuffer::Access access);
    void *mapRange(int offset, int count, QOpenGLBuffer::RangeAccessFlags access);
    bool unmap();

private:
    QOpenGLBufferPrivate *d_ptr;

    Q_DECLARE_PRIVATE(QOpenGLBuffer)
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QOpenGLBuffer::RangeAccessFlags)

QT_END_NAMESPACE

#endif // QT_NO_OPENGL

#endif
