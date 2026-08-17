// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QIMAGEIOHANDLER_H
#define QIMAGEIOHANDLER_H

#include <QtGui/qtguiglobal.h>
#include <QtGui/qimage.h>
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

    static bool allocateImage(QSize size, QImage::Format format, QImage *image);

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
