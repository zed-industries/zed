// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSURFACE_H
#define QSURFACE_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qnamespace.h>
#include <QtGui/qsurfaceformat.h>
#include <QtCore/qmetatype.h>
#include <QtCore/qsize.h>

QT_BEGIN_NAMESPACE


class QPlatformSurface;

class QSurfacePrivate;

class Q_GUI_EXPORT QSurface
{
    Q_GADGET
public:
    enum SurfaceClass {
        Window,
        Offscreen
    };
    Q_ENUM(SurfaceClass)

    enum SurfaceType {
        RasterSurface,
        OpenGLSurface,
        RasterGLSurface,
        OpenVGSurface,
        VulkanSurface,
        MetalSurface,
        Direct3DSurface
    };
    Q_ENUM(SurfaceType)

    virtual ~QSurface();

    SurfaceClass surfaceClass() const;

    virtual QSurfaceFormat format() const = 0;
    virtual QPlatformSurface *surfaceHandle() const = 0;

    virtual SurfaceType surfaceType() const = 0;
    bool supportsOpenGL() const;

    virtual QSize size() const = 0;

protected:
    explicit QSurface(SurfaceClass type);

    SurfaceClass m_type;

    QSurfacePrivate *m_reserved;
};

QT_END_NAMESPACE

QT_DECL_METATYPE_EXTERN_TAGGED(QSurface*, QSurface_ptr, Q_GUI_EXPORT)

#endif //QSURFACE_H
