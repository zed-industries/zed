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

#ifndef QOPENGLTEXTUREBLITTER_H
#define QOPENGLTEXTUREBLITTER_H

#include <QtGui/qtguiglobal.h>

#ifndef QT_NO_OPENGL

#include <QtGui/qopengl.h>
#include <QtGui/QMatrix3x3>
#include <QtGui/QMatrix4x4>

QT_BEGIN_NAMESPACE

class QOpenGLTextureBlitterPrivate;

class Q_GUI_EXPORT QOpenGLTextureBlitter
{
public:
    QOpenGLTextureBlitter();
    ~QOpenGLTextureBlitter();

    enum Origin {
        OriginBottomLeft,
        OriginTopLeft
    };

    bool create();
    bool isCreated() const;
    void destroy();

    bool supportsExternalOESTarget() const;

    void bind(GLenum target = GL_TEXTURE_2D);
    void release();

    void setRedBlueSwizzle(bool swizzle);
    void setOpacity(float opacity);

    void blit(GLuint texture, const QMatrix4x4 &targetTransform, Origin sourceOrigin);
    void blit(GLuint texture, const QMatrix4x4 &targetTransform, const QMatrix3x3 &sourceTransform);

    static QMatrix4x4 targetTransform(const QRectF &target, const QRect &viewport);
    static QMatrix3x3 sourceTransform(const QRectF &subTexture, const QSize &textureSize, Origin origin);

private:
    Q_DISABLE_COPY(QOpenGLTextureBlitter)
    Q_DECLARE_PRIVATE(QOpenGLTextureBlitter)
    QScopedPointer<QOpenGLTextureBlitterPrivate> d_ptr;
};

QT_END_NAMESPACE

#endif

#endif //QOPENGLTEXTUREBLITTER_H
