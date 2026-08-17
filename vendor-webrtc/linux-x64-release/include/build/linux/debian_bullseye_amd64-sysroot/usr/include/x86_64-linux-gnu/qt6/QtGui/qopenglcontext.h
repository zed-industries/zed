// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QOPENGLCONTEXT_H
#define QOPENGLCONTEXT_H

#include <QtGui/qtguiglobal.h>

#ifndef QT_NO_OPENGL

#include <QtCore/qnamespace.h>
#include <QtCore/qobject.h>
#include <QtCore/qscopedpointer.h>
#include <QtCore/qset.h>
#include <QtCore/qnativeinterface.h>

#include <QtGui/QSurfaceFormat>

#ifdef __GLEW_H__
#if defined(Q_CC_GNU)
#warning qopenglfunctions.h is not compatible with GLEW, GLEW defines will be undefined
#warning To use GLEW with Qt, do not include <qopengl.h> or <QOpenGLFunctions> after glew.h
#endif
#endif

#include <QtGui/qopengl.h>

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
    friend class QOpenGLContextPrivate;
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

    bool create();
    bool isValid() const;

    QSurfaceFormat format() const;
    QOpenGLContext *shareContext() const;
    QOpenGLContextGroup *shareGroup() const;
    QScreen *screen() const;

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

    QSet<QByteArray> extensions() const;
    bool hasExtension(const QByteArray &extension) const;

    enum OpenGLModuleType {
        LibGL,
        LibGLES
    };

    static OpenGLModuleType openGLModuleType();

    bool isOpenGLES() const;

    static bool supportsThreadedOpenGL();
    static QOpenGLContext *globalShareContext();

    QT_DECLARE_NATIVE_INTERFACE_ACCESSOR(QOpenGLContext)

Q_SIGNALS:
    void aboutToBeDestroyed();

private:
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

    QOpenGLTextureHelper* textureFunctions() const;
    void setTextureFunctions(QOpenGLTextureHelper* textureFuncs, std::function<void()> destroyCallback);

    void destroy();

    Q_PRIVATE_SLOT(d_func(), void _q_screenDestroyed(QObject *object))
};

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug debug, const QOpenGLContext *ctx);
Q_GUI_EXPORT QDebug operator<<(QDebug debug, const QOpenGLContextGroup *cg);
#endif // !QT_NO_DEBUG_STREAM

QT_END_NAMESPACE

#include <QtGui/qopenglcontext_platform.h>

#endif // QT_NO_OPENGL

#endif // QOPENGLCONTEXT_H
