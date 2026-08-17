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

#ifndef QPIXMAP_H
#define QPIXMAP_H

#include <QtGui/qtguiglobal.h>
#include <QtGui/qpaintdevice.h>
#include <QtGui/qcolor.h>
#include <QtCore/qnamespace.h>
#include <QtCore/qstring.h> // char*->QString conversion
#include <QtCore/qsharedpointer.h>
#include <QtGui/qimage.h>
#include <QtGui/qtransform.h>

QT_BEGIN_NAMESPACE


class QImageWriter;
class QImageReader;
class QColor;
class QVariant;
class QPlatformPixmap;

class Q_GUI_EXPORT QPixmap : public QPaintDevice
{
public:
    QPixmap();
    explicit QPixmap(QPlatformPixmap *data);
    QPixmap(int w, int h);
    explicit QPixmap(const QSize &);
    QPixmap(const QString& fileName, const char *format = nullptr, Qt::ImageConversionFlags flags = Qt::AutoColor);
#ifndef QT_NO_IMAGEFORMAT_XPM
    explicit QPixmap(const char * const xpm[]);
#endif
    QPixmap(const QPixmap &);
    ~QPixmap();

    QPixmap &operator=(const QPixmap &);
    inline QPixmap &operator=(QPixmap &&other) noexcept
    { qSwap(data, other.data); return *this; }
    inline void swap(QPixmap &other) noexcept
    { qSwap(data, other.data); }

    operator QVariant() const;

    bool isNull() const;
    int devType() const override;

    int width() const;
    int height() const;
    QSize size() const;
    QRect rect() const;
    int depth() const;

    static int defaultDepth();

    void fill(const QColor &fillColor = Qt::white);
#if QT_DEPRECATED_SINCE(5, 13)
    QT_DEPRECATED_X("Use QPainter or fill(QColor)")
    void fill(const QPaintDevice *device, const QPoint &ofs);
    QT_DEPRECATED_X("Use QPainter or fill(QColor)")
    void fill(const QPaintDevice *device, int xofs, int yofs);
#endif

    QBitmap mask() const;
    void setMask(const QBitmap &);

    qreal devicePixelRatio() const;
    void setDevicePixelRatio(qreal scaleFactor);

    bool hasAlpha() const;
    bool hasAlphaChannel() const;

#ifndef QT_NO_IMAGE_HEURISTIC_MASK
    QBitmap createHeuristicMask(bool clipTight = true) const;
#endif
    QBitmap createMaskFromColor(const QColor &maskColor, Qt::MaskMode mode = Qt::MaskInColor) const;

#if QT_DEPRECATED_SINCE(5, 13)
    QT_DEPRECATED_X("Use QScreen::grabWindow() instead")
    static QPixmap grabWindow(WId, int x = 0, int y = 0, int w = -1, int h = -1);
    QT_DEPRECATED_X("Use QWidget::grab() instead")
    static QPixmap grabWidget(QObject *widget, const QRect &rect);
    QT_DEPRECATED_X("Use QWidget::grab() instead")
    static QPixmap grabWidget(QObject *widget, int x = 0, int y = 0, int w = -1, int h = -1);
#endif

    inline QPixmap scaled(int w, int h, Qt::AspectRatioMode aspectMode = Qt::IgnoreAspectRatio,
                          Qt::TransformationMode mode = Qt::FastTransformation) const
        { return scaled(QSize(w, h), aspectMode, mode); }
    QPixmap scaled(const QSize &s, Qt::AspectRatioMode aspectMode = Qt::IgnoreAspectRatio,
                   Qt::TransformationMode mode = Qt::FastTransformation) const;
    QPixmap scaledToWidth(int w, Qt::TransformationMode mode = Qt::FastTransformation) const;
    QPixmap scaledToHeight(int h, Qt::TransformationMode mode = Qt::FastTransformation) const;
#if QT_DEPRECATED_SINCE(5, 15)
    QT_DEPRECATED_X("Use transformed(const QTransform &, Qt::TransformationMode mode)")
    QPixmap transformed(const QMatrix &, Qt::TransformationMode mode = Qt::FastTransformation) const;
    QT_DEPRECATED_X("Use trueMatrix(const QTransform &m, int w, int h)")
    static QMatrix trueMatrix(const QMatrix &m, int w, int h);
#endif // QT_DEPRECATED_SINCE(5, 15)
    QPixmap transformed(const QTransform &, Qt::TransformationMode mode = Qt::FastTransformation) const;
    static QTransform trueMatrix(const QTransform &m, int w, int h);

    QImage toImage() const;
    static QPixmap fromImage(const QImage &image, Qt::ImageConversionFlags flags = Qt::AutoColor);
    static QPixmap fromImageReader(QImageReader *imageReader, Qt::ImageConversionFlags flags = Qt::AutoColor);
    static QPixmap fromImage(QImage &&image, Qt::ImageConversionFlags flags = Qt::AutoColor)
    {
        return fromImageInPlace(image, flags);
    }

    bool load(const QString& fileName, const char *format = nullptr, Qt::ImageConversionFlags flags = Qt::AutoColor);
    bool loadFromData(const uchar *buf, uint len, const char* format = nullptr, Qt::ImageConversionFlags flags = Qt::AutoColor);
    inline bool loadFromData(const QByteArray &data, const char* format = nullptr, Qt::ImageConversionFlags flags = Qt::AutoColor);
    bool save(const QString& fileName, const char* format = nullptr, int quality = -1) const;
    bool save(QIODevice* device, const char* format = nullptr, int quality = -1) const;

    bool convertFromImage(const QImage &img, Qt::ImageConversionFlags flags = Qt::AutoColor);

    inline QPixmap copy(int x, int y, int width, int height) const;
    QPixmap copy(const QRect &rect = QRect()) const;

    inline void scroll(int dx, int dy, int x, int y, int width, int height, QRegion *exposed = nullptr);
    void scroll(int dx, int dy, const QRect &rect, QRegion *exposed = nullptr);

#if QT_DEPRECATED_SINCE(5, 0)
    QT_DEPRECATED inline int serialNumber() const { return cacheKey() >> 32; }
#endif
    qint64 cacheKey() const;

    bool isDetached() const;
    void detach();

    bool isQBitmap() const;

    QPaintEngine *paintEngine() const override;

    inline bool operator!() const { return isNull(); }

#if QT_DEPRECATED_SINCE(5, 0)
    QT_DEPRECATED inline QPixmap alphaChannel() const;
    QT_DEPRECATED inline void setAlphaChannel(const QPixmap &);
#endif

protected:
    int metric(PaintDeviceMetric) const override;
    static QPixmap fromImageInPlace(QImage &image, Qt::ImageConversionFlags flags = Qt::AutoColor);

private:
    QExplicitlySharedDataPointer<QPlatformPixmap> data;

    bool doImageIO(QImageWriter *io, int quality) const;

    QPixmap(const QSize &s, int type);
    void doInit(int, int, int);
    Q_DUMMY_COMPARISON_OPERATOR(QPixmap)
    friend class QPlatformPixmap;
    friend class QBitmap;
    friend class QPaintDevice;
    friend class QPainter;
    friend class QOpenGLWidget;
    friend class QWidgetPrivate;
    friend class QRasterBuffer;
#if !defined(QT_NO_DATASTREAM)
    friend Q_GUI_EXPORT QDataStream &operator>>(QDataStream &, QPixmap &);
#endif

public:
    QPlatformPixmap* handle() const;

public:
    typedef QExplicitlySharedDataPointer<QPlatformPixmap> DataPtr;
    inline DataPtr &data_ptr() { return data; }
};

Q_DECLARE_SHARED(QPixmap)

inline QPixmap QPixmap::copy(int ax, int ay, int awidth, int aheight) const
{
    return copy(QRect(ax, ay, awidth, aheight));
}

inline void QPixmap::scroll(int dx, int dy, int ax, int ay, int awidth, int aheight, QRegion *exposed)
{
    scroll(dx, dy, QRect(ax, ay, awidth, aheight), exposed);
}

inline bool QPixmap::loadFromData(const QByteArray &buf, const char *format,
                                  Qt::ImageConversionFlags flags)
{
    return loadFromData(reinterpret_cast<const uchar *>(buf.constData()), buf.size(), format, flags);
}

#if QT_DEPRECATED_SINCE(5, 0)
inline QPixmap QPixmap::alphaChannel() const
{
    QT_WARNING_PUSH
    QT_WARNING_DISABLE_DEPRECATED
    return QPixmap::fromImage(toImage().alphaChannel());
    QT_WARNING_POP
}

inline void QPixmap::setAlphaChannel(const QPixmap &p)
{
    QImage image = toImage();
    image.setAlphaChannel(p.toImage());
    *this = QPixmap::fromImage(image);

}
#endif

/*****************************************************************************
 QPixmap stream functions
*****************************************************************************/

#if !defined(QT_NO_DATASTREAM)
Q_GUI_EXPORT QDataStream &operator<<(QDataStream &, const QPixmap &);
Q_GUI_EXPORT QDataStream &operator>>(QDataStream &, QPixmap &);
#endif

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug, const QPixmap &);
#endif

QT_END_NAMESPACE

#endif // QPIXMAP_H
