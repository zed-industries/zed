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

#ifndef QIMAGEREADER_H
#define QIMAGEREADER_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qbytearray.h>
#include <QtCore/qcoreapplication.h>
#include <QtGui/qimage.h>
#include <QtGui/qimageiohandler.h>

QT_BEGIN_NAMESPACE


class QColor;
class QIODevice;
class QRect;
class QSize;
class QStringList;

class QImageReaderPrivate;
class Q_GUI_EXPORT QImageReader
{
    Q_DECLARE_TR_FUNCTIONS(QImageReader)
public:
    enum ImageReaderError {
        UnknownError,
        FileNotFoundError,
        DeviceError,
        UnsupportedFormatError,
        InvalidDataError
    };

    QImageReader();
    explicit QImageReader(QIODevice *device, const QByteArray &format = QByteArray());
    explicit QImageReader(const QString &fileName, const QByteArray &format = QByteArray());
    ~QImageReader();

    void setFormat(const QByteArray &format);
    QByteArray format() const;

    void setAutoDetectImageFormat(bool enabled);
    bool autoDetectImageFormat() const;

    void setDecideFormatFromContent(bool ignored);
    bool decideFormatFromContent() const;

    void setDevice(QIODevice *device);
    QIODevice *device() const;

    void setFileName(const QString &fileName);
    QString fileName() const;

    QSize size() const;

    QImage::Format imageFormat() const;

    QStringList textKeys() const;
    QString text(const QString &key) const;

    void setClipRect(const QRect &rect);
    QRect clipRect() const;

    void setScaledSize(const QSize &size);
    QSize scaledSize() const;

    void setQuality(int quality);
    int quality() const;

    void setScaledClipRect(const QRect &rect);
    QRect scaledClipRect() const;

    void setBackgroundColor(const QColor &color);
    QColor backgroundColor() const;

    bool supportsAnimation() const;

    QImageIOHandler::Transformations transformation() const;

    void setAutoTransform(bool enabled);
    bool autoTransform() const;

#if QT_DEPRECATED_SINCE(5, 15)
    QT_DEPRECATED_VERSION_X_5_15("Use QColorSpace instead")
    void setGamma(float gamma);
    QT_DEPRECATED_VERSION_X_5_15("Use QColorSpace instead")
    float gamma() const;
#endif

    QByteArray subType() const;
    QList<QByteArray> supportedSubTypes() const;

    bool canRead() const;
    QImage read();
    bool read(QImage *image);

    bool jumpToNextImage();
    bool jumpToImage(int imageNumber);
    int loopCount() const;
    int imageCount() const;
    int nextImageDelay() const;
    int currentImageNumber() const;
    QRect currentImageRect() const;

    ImageReaderError error() const;
    QString errorString() const;

    bool supportsOption(QImageIOHandler::ImageOption option) const;

    static QByteArray imageFormat(const QString &fileName);
    static QByteArray imageFormat(QIODevice *device);
    static QList<QByteArray> supportedImageFormats();
    static QList<QByteArray> supportedMimeTypes();
    static QList<QByteArray> imageFormatsForMimeType(const QByteArray &mimeType);

private:
    Q_DISABLE_COPY(QImageReader)
    QImageReaderPrivate *d;
};

QT_END_NAMESPACE

#endif // QIMAGEREADER_H
