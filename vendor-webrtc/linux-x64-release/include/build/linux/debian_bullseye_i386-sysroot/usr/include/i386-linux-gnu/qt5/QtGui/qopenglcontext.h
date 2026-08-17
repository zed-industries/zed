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

#ifndef QOPENGLCONTEXT_H
#define QOPENGLCONTEXT_H

#include <QtGui/qtguiglobal.h>

#ifndef QT_NO_OPENGL

#include <QtCore/qnamespace.h>
#include <QtCore/QObject>
#include <QtCore/QScopedPointer>

#include <QtGui/QSurfaceFormat>

#ifdef __GLEW_H__
#if defined(Q_CC_GNU)
#warning qopenglfunctions.h is not compatible with GLEW, GLEW defines will be undefined
#warning To use GLEW with Qt, do not include <qopengl.h> or <QOpenGLFunctions> after glew.h
#endif
#endif

#include <QtGui/qopengl.h>
#include <QtGui/qopenglversionfunctions.h>

#if QT_DEPRECATED_SINCE(5, 6)
#include <QtCore/qhash.h>
#endif
#include <QtCore/qhashfunctions.h>
#include <QtCore/qpair.h>
#include <QtCore/qvariant.h>

QT_BEGIN_NAMESPACE

class QDebug;
class QOpenGLContextPrivate;
class QOpenGLContextGroupPrivate;
class QOpenGLFunctions;
class QOpenGLExtraFunctions;
class QPlatformOpenGLContext;

class QScreen;
class QSurface;

class QOpenGLVersionProfilePrivate;

class Q_GUI_EXPORT QOpenGLVersionProfile
{
public:
    QOpenGLVersionProfile();
    explicit QOpenGLVersionProfile(const QSurfaceFormat &format);
    QOpenGLVersionProfile(const QOpenGLVersionProfile &other);
    ~QOpenGLVersionProfile();

    QOpenGLVersionProfile &operator=(const QOpenGLVersionProfile &rhs);

    QPair<int, int> version() const;
    void setVersion(int majorVersion, int minorVersion);

    QSurfaceFormat::OpenGLContextProfile profile() const;
    void setProfile(QSurfaceFormat::OpenGLContextProfile profile);

    bool hasProfiles() const;
    bool isLegacyVersion() const;
    bool isValid() const;

private:
    QOpenGLVersionProfilePrivate* d;
};

inline uint qHash(const QOpenGLVersionProfile &v, uint seed = 0)
{
    return qHash(static_cast<int>(v.profile() * 1000)
               + v.version().first * 100 + v.version().second * 10, seed);
}

inline bool operator==(const QOpenGLVersionProfile &lhs, const QOpenGLVersionProfile &rhs)
{
    if (lhs.profile() != rhs.profile())
        return false;
    return lhs.version() == rhs.version();
}

inline bool operator!=(const QOpenGLVersionProfile &lhs, const QOpenGLVersionProfile &rhs)
{
    return !operator==(lhs, rhs);
}

class Q_GUI_EXPORT QOpenGLContextGroup : public QObject
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QOpenGLContextGroup)
public:
    ~QOpenGLContextGroup();

    QList<QOpenGLContext *> shares() const;

    static QOpenGLContextGroup *currentContextGroup();

private:
    QOpenGLContextGroup();

    friend class QOpenGLContext;
    friend class QOpenGLContextGroupResourceBase;
    friend class QOpenGLSharedResource;
    friend class QOpenGLMultiGroupSharedResource;
};


class QOpenGLTextureHelper;

class Q_GUI_EXPORT QOpenGLContext : public QObject
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QOpenGLContext)
public:
    explicit QOpenGLContext(QObject *parent = nullptr);
    ~QOpenGLContext();

    void setFormat(const QSurfaceFormat &format);
    void setShareContext(QOpenGLContext *shareContext);
    void setScreen(QScreen *screen);
    void setNativeHandle(const QVariant &handle);

    bool create();
    bool isValid() const;

    QSurfaceFormat format() const;
    QOpenGLContext *shareContext() const;
    QOpenGLContextGroup *shareGroup() const;
    QScreen *screen() const;
    QVariant nativeHandle() const;

    GLuint defaultFramebufferObject() const;

    bool makeCurrent(QSurface *surface);
    void doneCurrent();

    void swapBuffers(QSurface *surface);
    QFunctionPointer getProcAddress(const QByteArray &procName) const;
    QFunctionPointer getProcAddress(const char *procName) const;

    QSurface *surface() const;

    static QOpenGLContext *currentContext();
    static bool areSharing(QOpenGLContext *first, QOpenGLContext *second);

    QPlatformOpenGLContext *handle() const;
    QPlatformOpenGLContext *shareHandle() const;

    QOpenGLFunctions *functions() const;
    QOpenGLExtraFunctions *extraFunctions() const;

    QAbstractOpenGLFunctions *versionFunctions(const QOpenGLVersionProfile &versionProfile = QOpenGLVersionProfile()) const;

    template<class TYPE>
    TYPE *versionFunctions() const
    {
        QOpenGLVersionProfile v = TYPE::versionProfile();
        return static_cast<TYPE*>(versionFunctions(v));
    }

    QSet<QByteArray> extensions() const;
    bool hasExtension(const QByteArray &extension) const;

    static void *openGLModuleHandle();

    enum OpenGLModuleType {
        LibGL,
        LibGLES
    };

    static OpenGLModuleType openGLModuleType();

    bool isOpenGLES() const;

    static bool supportsThreadedOpenGL();
    static QOpenGLContext *globalShareContext();

Q_SIGNALS:
    void aboutToBeDestroyed();

private:
    friend class QGLContext;
    friend class QGLPixelBuffer;
    friend class QOpenGLContextResourceBase;
    friend class QOpenGLPaintDevice;
    friend class QOpenGLGlyphTexture;
    friend class QOpenGLTextureGlyphCache;
    friend class QOpenGLEngineShaderManager;
    friend class QOpenGLFramebufferObject;
    friend class QOpenGLFramebufferObjectPrivate;
    friend class QOpenGL2PaintEngineEx;
    friend class QOpenGL2PaintEngineExPrivate;
    friend class QSGDistanceFieldGlyphCache;
    friend class QWidgetPrivate;
    friend class QAbstractOpenGLFunctionsPrivate;
    friend class QOpenGLTexturePrivate;

    void *qGLContextHandle() const;
    void setQGLContextHandle(void *handle,void (*qGLContextDeleteFunction)(void *));
    void deleteQGLContext();

    QOpenGLVersionFunctionsStorage* functionsBackendStorage() const;
    void insertExternalFunctions(QAbstractOpenGLFunctions *f);
    void removeExternalFunctions(QAbstractOpenGLFunctions *f);

    QOpenGLTextureHelper* textureFunctions() const;
    void setTextureFunctions(QOpenGLTextureHelper* textureFuncs);

    void destroy();

    Q_PRIVATE_SLOT(d_func(), void _q_screenDestroyed(QObject *object))
};

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug debug, const QOpenGLVersionProfile &vp);
Q_GUI_EXPORT QDebug operator<<(QDebug debug, const QOpenGLContext *ctx);
Q_GUI_EXPORT QDebug operator<<(QDebug debug, const QOpenGLContextGroup *cg);
#endif // !QT_NO_DEBUG_STREAM

QT_END_NAMESPACE

#endif // QT_NO_OPENGL

#endif // QOPENGLCONTEXT_H
