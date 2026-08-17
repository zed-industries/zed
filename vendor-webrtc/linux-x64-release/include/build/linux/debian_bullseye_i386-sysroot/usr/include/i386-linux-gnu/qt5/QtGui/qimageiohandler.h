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

#ifndef QIMAGEIOHANDLER_H
#define QIMAGEIOHANDLER_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qiodevice.h>
#include <QtCore/qplugin.h>
#include <QtCore/qfactoryinterface.h>
#include <QtCore/qscopedpointer.h>

QT_BEGIN_NAMESPACE


class QImage;
class QRect;
class QSize;
class QVariant;

class QImageIOHandlerPrivate;
class Q_GUI_EXPORT QImageIOHandler
{
    Q_DECLARE_PRIVATE(QImageIOHandler)
public:
    QImageIOHandler();
    virtual ~QImageIOHandler();

    void setDevice(QIODevice *device);
    QIODevice *device() const;

    void setFormat(const QByteArray &format);
    void setFormat(const QByteArray &format) const;
    QByteArray format() const;

    QT_DEPRECATED_X("Use QImageIOHandler::format() instead")
    virtual QByteArray name() const;

    virtual bool canRead() const = 0;
    virtual bool read(QImage *image) = 0;
    virtual bool write(const QImage &image);

    enum ImageOption {
        Size,
        ClipRect,
        Description,
        ScaledClipRect,
        ScaledSize,
        CompressionRatio,
        Gamma,
        Quality,
        Name,
        SubType,
        IncrementalReading,
        Endianness,
        Animation,
        BackgroundColor,
        ImageFormat,
        SupportedSubTypes,
        OptimizedWrite,
        ProgressiveScanWrite,
        ImageTransformation
#if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
        , TransformedByDefault
#endif
    };

    enum Transformation {
        TransformationNone = 0,
        TransformationMirror = 1,
        TransformationFlip = 2,
        TransformationRotate180 = TransformationMirror | TransformationFlip,
        TransformationRotate90 = 4,
        TransformationMirrorAndRotate90 = TransformationMirror | TransformationRotate90,
        TransformationFlipAndRotate90 = TransformationFlip | TransformationRotate90,
        TransformationRotate270 = TransformationRotate180 | TransformationRotate90
    };
    Q_DECLARE_FLAGS(Transformations, Transformation)

    virtual QVariant option(ImageOption option) const;
    virtual void setOption(ImageOption option, const QVariant &value);
    virtual bool supportsOption(ImageOption option) const;

    // incremental loading
    virtual bool jumpToNextImage();
    virtual bool jumpToImage(int imageNumber);
    virtual int loopCount() const;
    virtual int imageCount() const;
    virtual int nextImageDelay() const;
    virtual int currentImageNumber() const;
    virtual QRect currentImageRect() const;

protected:
    QImageIOHandler(QImageIOHandlerPrivate &dd);
    QScopedPointer<QImageIOHandlerPrivate> d_ptr;
private:
    Q_DISABLE_COPY(QImageIOHandler)
};

#ifndef QT_NO_IMAGEFORMATPLUGIN

#define QImageIOHandlerFactoryInterface_iid "org.qt-project.Qt.QImageIOHandlerFactoryInterface"

class Q_GUI_EXPORT QImageIOPlugin : public QObject
{
    Q_OBJECT
public:
    explicit QImageIOPlugin(QObject *parent = nullptr);
    ~QImageIOPlugin();

    enum Capability {
        CanRead = 0x1,
        CanWrite = 0x2,
        CanReadIncremental = 0x4
    };
    Q_DECLARE_FLAGS(Capabilities, Capability)

    virtual Capabilities capabilities(QIODevice *device, const QByteArray &format) const = 0;
    virtual QImageIOHandler *create(QIODevice *device, const QByteArray &format = QByteArray()) const = 0;
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QImageIOPlugin::Capabilities)

#endif // QT_NO_IMAGEFORMATPLUGIN

QT_END_NAMESPACE

#endif // QIMAGEIOHANDLER_H
