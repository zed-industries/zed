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
#ifndef QSURFACEFORMAT_H
#define QSURFACEFORMAT_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qpair.h>
#include <QtCore/qobjectdefs.h>

QT_BEGIN_NAMESPACE


class QOpenGLContext;
class QSurfaceFormatPrivate;

class Q_GUI_EXPORT QSurfaceFormat
{
    Q_GADGET
public:
    enum FormatOption {
        StereoBuffers            = 0x0001,
        DebugContext             = 0x0002,
        DeprecatedFunctions      = 0x0004,
        ResetNotification        = 0x0008
    };
    Q_ENUM(FormatOption)
    Q_DECLARE_FLAGS(FormatOptions, FormatOption)

    enum SwapBehavior {
        DefaultSwapBehavior,
        SingleBuffer,
        DoubleBuffer,
        TripleBuffer
    };
    Q_ENUM(SwapBehavior)

    enum RenderableType {
        DefaultRenderableType = 0x0,
        OpenGL                = 0x1,
        OpenGLES              = 0x2,
        OpenVG                = 0x4
    };
    Q_ENUM(RenderableType)

    enum OpenGLContextProfile {
        NoProfile,
        CoreProfile,
        CompatibilityProfile
    };
    Q_ENUM(OpenGLContextProfile)

    enum ColorSpace {
        DefaultColorSpace,
        sRGBColorSpace
    };
    Q_ENUM(ColorSpace)

    QSurfaceFormat();
    /*implicit*/ QSurfaceFormat(FormatOptions options);
    QSurfaceFormat(const QSurfaceFormat &other);
    QSurfaceFormat &operator=(const QSurfaceFormat &other);
    ~QSurfaceFormat();

    void setDepthBufferSize(int size);
    int depthBufferSize() const;

    void setStencilBufferSize(int size);
    int stencilBufferSize() const;

    void setRedBufferSize(int size);
    int redBufferSize() const;
    void setGreenBufferSize(int size);
    int greenBufferSize() const;
    void setBlueBufferSize(int size);
    int blueBufferSize() const;
    void setAlphaBufferSize(int size);
    int alphaBufferSize() const;

    void setSamples(int numSamples);
    int samples() const;

    void setSwapBehavior(SwapBehavior behavior);
    SwapBehavior swapBehavior() const;

    bool hasAlpha() const;

    void setProfile(OpenGLContextProfile profile);
    OpenGLContextProfile profile() const;

    void setRenderableType(RenderableType type);
    RenderableType renderableType() const;

    void setMajorVersion(int majorVersion);
    int majorVersion() const;

    void setMinorVersion(int minorVersion);
    int minorVersion() const;

    QPair<int, int> version() const;
    void setVersion(int major, int minor);

    bool stereo() const;
    void setStereo(bool enable);

#if QT_DEPRECATED_SINCE(5, 2)
    QT_DEPRECATED void setOption(QSurfaceFormat::FormatOptions opt);
    QT_DEPRECATED bool testOption(QSurfaceFormat::FormatOptions opt) const;
#endif

    void setOptions(QSurfaceFormat::FormatOptions options);
    void setOption(FormatOption option, bool on = true);
    bool testOption(FormatOption option) const;
    QSurfaceFormat::FormatOptions options() const;

    int swapInterval() const;
    void setSwapInterval(int interval);

    ColorSpace colorSpace() const;
    void setColorSpace(ColorSpace colorSpace);

    static void setDefaultFormat(const QSurfaceFormat &format);
    static QSurfaceFormat defaultFormat();

private:
    QSurfaceFormatPrivate *d;

    void detach();

    friend Q_GUI_EXPORT bool operator==(const QSurfaceFormat&, const QSurfaceFormat&);
    friend Q_GUI_EXPORT bool operator!=(const QSurfaceFormat&, const QSurfaceFormat&);
#ifndef QT_NO_DEBUG_STREAM
    friend Q_GUI_EXPORT QDebug operator<<(QDebug, const QSurfaceFormat &);
#endif
};

Q_GUI_EXPORT bool operator==(const QSurfaceFormat&, const QSurfaceFormat&);
Q_GUI_EXPORT bool operator!=(const QSurfaceFormat&, const QSurfaceFormat&);

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug, const QSurfaceFormat &);
#endif

Q_DECLARE_OPERATORS_FOR_FLAGS(QSurfaceFormat::FormatOptions)

inline bool QSurfaceFormat::stereo() const
{
    return testOption(QSurfaceFormat::StereoBuffers);
}

QT_END_NAMESPACE

#endif //QSURFACEFORMAT_H
