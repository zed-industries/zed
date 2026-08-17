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

#ifndef QIMAGE_H
#define QIMAGE_H

#include <QtGui/qtguiglobal.h>
#include <QtGui/qcolor.h>
#include <QtGui/qrgb.h>
#include <QtGui/qpaintdevice.h>
#include <QtGui/qpixelformat.h>
#include <QtGui/qtransform.h>
#include <QtCore/qbytearray.h>
#include <QtCore/qrect.h>
#include <QtCore/qstring.h>

#if QT_DEPRECATED_SINCE(5, 0)
#include <QtCore/qstringlist.h>
#endif

#if defined(Q_OS_DARWIN) || defined(Q_QDOC)
Q_FORWARD_DECLARE_MUTABLE_CG_TYPE(CGImage);
#endif

QT_BEGIN_NAMESPACE


class QColorSpace;
class QColorTransform;
class QIODevice;
class QMatrix;
class QStringList;
class QTransform;
class QVariant;
template <class T> class QList;
template <class T> class QVector;

struct QImageData;
class QImageDataMisc; // internal
#if QT_DEPRECATED_SINCE(5, 0)
class QImageTextKeyLang {
public:
    QT_DEPRECATED QImageTextKeyLang(const char* k, const char* l) : key(k), lang(l) { }
    QT_DEPRECATED QImageTextKeyLang() { }

    QByteArray key;
    QByteArray lang;

    bool operator< (const QImageTextKeyLang& other) const
        { return key < other.key || (key==other.key && lang < other.lang); }
    bool operator== (const QImageTextKeyLang& other) const
        { return key==other.key && lang==other.lang; }
    inline bool operator!= (const QImageTextKeyLang &other) const
        { return !operator==(other); }
private:
    friend class QImage;
    QImageTextKeyLang(bool /*dummy*/) {}
};
#endif

typedef void (*QImageCleanupFunction)(void*);

class Q_GUI_EXPORT QImage : public QPaintDevice
{
    Q_GADGET
public:
    enum InvertMode { InvertRgb, InvertRgba };
    enum Format {
        Format_Invalid,
        Format_Mono,
        Format_MonoLSB,
        Format_Indexed8,
        Format_RGB32,
        Format_ARGB32,
        Format_ARGB32_Premultiplied,
        Format_RGB16,
        Format_ARGB8565_Premultiplied,
        Format_RGB666,
        Format_ARGB6666_Premultiplied,
        Format_RGB555,
        Format_ARGB8555_Premultiplied,
        Format_RGB888,
        Format_RGB444,
        Format_ARGB4444_Premultiplied,
        Format_RGBX8888,
        Format_RGBA8888,
        Format_RGBA8888_Premultiplied,
        Format_BGR30,
        Format_A2BGR30_Premultiplied,
        Format_RGB30,
        Format_A2RGB30_Premultiplied,
        Format_Alpha8,
        Format_Grayscale8,
        Format_RGBX64,
        Format_RGBA64,
        Format_RGBA64_Premultiplied,
        Format_Grayscale16,
        Format_BGR888,
#ifndef Q_QDOC
        NImageFormats
#endif
    };
    Q_ENUM(Format)

    QImage() noexcept;
    QImage(const QSize &size, Format format);
    QImage(int width, int height, Format format);
    QImage(uchar *data, int width, int height, Format format, QImageCleanupFunction cleanupFunction = nullptr, void *cleanupInfo = nullptr);
    QImage(const uchar *data, int width, int height, Format format, QImageCleanupFunction cleanupFunction = nullptr, void *cleanupInfo = nullptr);
    QImage(uchar *data, int width, int height, int bytesPerLine, Format format, QImageCleanupFunction cleanupFunction = nullptr, void *cleanupInfo = nullptr);
    QImage(const uchar *data, int width, int height, int bytesPerLine, Format format, QImageCleanupFunction cleanupFunction = nullptr, void *cleanupInfo = nullptr);

#ifndef QT_NO_IMAGEFORMAT_XPM
    explicit QImage(const char * const xpm[]);
#endif
    explicit QImage(const QString &fileName, const char *format = nullptr);

    QImage(const QImage &);
    inline QImage(QImage &&other) noexcept
        : QPaintDevice(), d(nullptr)
    { qSwap(d, other.d); }
    ~QImage();

    QImage &operator=(const QImage &);
    inline QImage &operator=(QImage &&other) noexcept
    { qSwap(d, other.d); return *this; }
    inline void swap(QImage &other) noexcept
    { qSwap(d, other.d); }

    bool isNull() const;

    int devType() const override;

    bool operator==(const QImage &) const;
    bool operator!=(const QImage &) const;
    operator QVariant() const;
    void detach();
    bool isDetached() const;

    QImage copy(const QRect &rect = QRect()) const;
    inline QImage copy(int x, int y, int w, int h) const
        { return copy(QRect(x, y, w, h)); }

    Format format() const;

#if defined(Q_COMPILER_REF_QUALIFIERS) && !defined(QT_COMPILING_QIMAGE_COMPAT_CPP)
    Q_REQUIRED_RESULT Q_ALWAYS_INLINE QImage convertToFormat(Format f, Qt::ImageConversionFlags flags = Qt::AutoColor) const &
    { return convertToFormat_helper(f, flags); }
    Q_REQUIRED_RESULT Q_ALWAYS_INLINE QImage convertToFormat(Format f, Qt::ImageConversionFlags flags = Qt::AutoColor) &&
    {
        if (convertToFormat_inplace(f, flags))
            return std::move(*this);
        else
            return convertToFormat_helper(f, flags);
    }
#else
    Q_REQUIRED_RESULT QImage convertToFormat(Format f, Qt::ImageConversionFlags flags = Qt::AutoColor) const;
#endif
    Q_REQUIRED_RESULT QImage convertToFormat(Format f, const QVector<QRgb> &colorTable, Qt::ImageConversionFlags flags = Qt::AutoColor) const;
    bool reinterpretAsFormat(Format f);

    void convertTo(Format f, Qt::ImageConversionFlags flags = Qt::AutoColor);

    int width() const;
    int height() const;
    QSize size() const;
    QRect rect() const;

    int depth() const;
    int colorCount() const;
    int bitPlaneCount() const;

    QRgb color(int i) const;
    void setColor(int i, QRgb c);
    void setColorCount(int);

    bool allGray() const;
    bool isGrayscale() const;

    uchar *bits();
    const uchar *bits() const;
    const uchar *constBits() const;

#if QT_DEPRECATED_SINCE(5, 10)
    QT_DEPRECATED_X("Use sizeInBytes") int byteCount() const;
#endif
    qsizetype sizeInBytes() const;

    uchar *scanLine(int);
    const uchar *scanLine(int) const;
    const uchar *constScanLine(int) const;
#if QT_VERSION >= QT_VERSION_CHECK(6,0,0)
    qsizetype bytesPerLine() const;
#else
    int bytesPerLine() const;
#endif

    bool valid(int x, int y) const;
    bool valid(const QPoint &pt) const;

    int pixelIndex(int x, int y) const;
    int pixelIndex(const QPoint &pt) const;

    QRgb pixel(int x, int y) const;
    QRgb pixel(const QPoint &pt) const;

    void setPixel(int x, int y, uint index_or_rgb);
    void setPixel(const QPoint &pt, uint index_or_rgb);

    QColor pixelColor(int x, int y) const;
    QColor pixelColor(const QPoint &pt) const;

    void setPixelColor(int x, int y, const QColor &c);
    void setPixelColor(const QPoint &pt, const QColor &c);

    QVector<QRgb> colorTable() const;
#if QT_VERSION >= QT_VERSION_CHECK(6,0,0)
    void setColorTable(const QVector<QRgb> &colors);
#else
    void setColorTable(const QVector<QRgb> colors);
#endif

    qreal devicePixelRatio() const;
    void setDevicePixelRatio(qreal scaleFactor);

    void fill(uint pixel);
    void fill(const QColor &color);
    void fill(Qt::GlobalColor color);


    bool hasAlphaChannel() const;
    void setAlphaChannel(const QImage &alphaChannel);
#if QT_DEPRECATED_SINCE(5, 15)
    QT_DEPRECATED_X("Use convertToFormat(QImage::Format_Alpha8)")
    QImage alphaChannel() const;
#endif
    QImage createAlphaMask(Qt::ImageConversionFlags flags = Qt::AutoColor) const;
#ifndef QT_NO_IMAGE_HEURISTIC_MASK
    QImage createHeuristicMask(bool clipTight = true) const;
#endif
    QImage createMaskFromColor(QRgb color, Qt::MaskMode mode = Qt::MaskInColor) const;

    inline QImage scaled(int w, int h, Qt::AspectRatioMode aspectMode = Qt::IgnoreAspectRatio,
                        Qt::TransformationMode mode = Qt::FastTransformation) const
        { return scaled(QSize(w, h), aspectMode, mode); }
    QImage scaled(const QSize &s, Qt::AspectRatioMode aspectMode = Qt::IgnoreAspectRatio,
                 Qt::TransformationMode mode = Qt::FastTransformation) const;
    QImage scaledToWidth(int w, Qt::TransformationMode mode = Qt::FastTransformation) const;
    QImage scaledToHeight(int h, Qt::TransformationMode mode = Qt::FastTransformation) const;
#if QT_DEPRECATED_SINCE(5, 15)
    QT_DEPRECATED_X("Use transformed(const QTransform &matrix, Qt::TransformationMode mode)")
    QImage transformed(const QMatrix &matrix, Qt::TransformationMode mode = Qt::FastTransformation) const;
    QT_DEPRECATED_X("trueMatrix(const QTransform &, int w, int h)")
    static QMatrix trueMatrix(const QMatrix &, int w, int h);
#endif // QT_DEPRECATED_SINCE(5, 15)
    QImage transformed(const QTransform &matrix, Qt::TransformationMode mode = Qt::FastTransformation) const;
    static QTransform trueMatrix(const QTransform &, int w, int h);
#if defined(Q_COMPILER_REF_QUALIFIERS) && !defined(QT_COMPILING_QIMAGE_COMPAT_CPP)
    QImage mirrored(bool horizontally = false, bool vertically = true) const &
        { return mirrored_helper(horizontally, vertically); }
    QImage &&mirrored(bool horizontally = false, bool vertically = true) &&
        { mirrored_inplace(horizontally, vertically); return std::move(*this); }
    QImage rgbSwapped() const &
        { return rgbSwapped_helper(); }
    QImage &&rgbSwapped() &&
        { rgbSwapped_inplace(); return std::move(*this); }
#else
    QImage mirrored(bool horizontally = false, bool vertically = true) const;
    QImage rgbSwapped() const;
#endif
    void invertPixels(InvertMode = InvertRgb);

    QColorSpace colorSpace() const;
    QImage convertedToColorSpace(const QColorSpace &) const;
    void convertToColorSpace(const QColorSpace &);
    void setColorSpace(const QColorSpace &);

    void applyColorTransform(const QColorTransform &transform);

    bool load(QIODevice *device, const char* format);
    bool load(const QString &fileName, const char *format = nullptr);
    bool loadFromData(const uchar *buf, int len, const char *format = nullptr);
    inline bool loadFromData(const QByteArray &data, const char *aformat = nullptr)
        { return loadFromData(reinterpret_cast<const uchar *>(data.constData()), data.size(), aformat); }

    bool save(const QString &fileName, const char *format = nullptr, int quality = -1) const;
    bool save(QIODevice *device, const char *format = nullptr, int quality = -1) const;

    static QImage fromData(const uchar *data, int size, const char *format = nullptr);
    inline static QImage fromData(const QByteArray &data, const char *format = nullptr)
        { return fromData(reinterpret_cast<const uchar *>(data.constData()), data.size(), format); }

#if QT_DEPRECATED_SINCE(5, 0)
    QT_DEPRECATED inline int serialNumber() const { return cacheKey() >> 32; }
#endif
    qint64 cacheKey() const;

    QPaintEngine *paintEngine() const override;

    // Auxiliary data
    int dotsPerMeterX() const;
    int dotsPerMeterY() const;
    void setDotsPerMeterX(int);
    void setDotsPerMeterY(int);
    QPoint offset() const;
    void setOffset(const QPoint&);

    QStringList textKeys() const;
    QString text(const QString &key = QString()) const;
    void setText(const QString &key, const QString &value);

    QPixelFormat pixelFormat() const noexcept;
    static QPixelFormat toPixelFormat(QImage::Format format) noexcept;
    static QImage::Format toImageFormat(QPixelFormat format) noexcept;

    // Platform specific conversion functions
#if defined(Q_OS_DARWIN) || defined(Q_QDOC)
    CGImageRef toCGImage() const Q_DECL_CF_RETURNS_RETAINED;
#endif

#if QT_DEPRECATED_SINCE(5, 0)
    QT_DEPRECATED inline QString text(const char *key, const char *lang = nullptr) const;
    QT_DEPRECATED inline QList<QImageTextKeyLang> textList() const;
    QT_DEPRECATED inline QStringList textLanguages() const;
    QT_DEPRECATED inline QString text(const QImageTextKeyLang&) const;
    QT_DEPRECATED inline void setText(const char* key, const char* lang, const QString&);
#endif

#if QT_DEPRECATED_SINCE(5, 0)
    QT_DEPRECATED inline int numColors() const;
    QT_DEPRECATED inline void setNumColors(int);
    QT_DEPRECATED inline int numBytes() const;
#endif

protected:
    virtual int metric(PaintDeviceMetric metric) const override;
    QImage mirrored_helper(bool horizontal, bool vertical) const;
    QImage rgbSwapped_helper() const;
    void mirrored_inplace(bool horizontal, bool vertical);
    void rgbSwapped_inplace();
    QImage convertToFormat_helper(Format format, Qt::ImageConversionFlags flags) const;
    bool convertToFormat_inplace(Format format, Qt::ImageConversionFlags flags);
    QImage smoothScaled(int w, int h) const;

private:
    friend class QWSOnScreenSurface;
    QImageData *d;

    friend class QRasterPlatformPixmap;
    friend class QBlittablePlatformPixmap;
    friend class QPixmapCacheEntry;
    friend struct QImageData;

public:
    typedef QImageData * DataPtr;
    inline DataPtr &data_ptr() { return d; }
};

Q_DECLARE_SHARED(QImage)

// Inline functions...

inline bool QImage::valid(const QPoint &pt) const { return valid(pt.x(), pt.y()); }
inline int QImage::pixelIndex(const QPoint &pt) const { return pixelIndex(pt.x(), pt.y());}
inline QRgb QImage::pixel(const QPoint &pt) const { return pixel(pt.x(), pt.y()); }
inline void QImage::setPixel(const QPoint &pt, uint index_or_rgb) { setPixel(pt.x(), pt.y(), index_or_rgb); }
inline QColor QImage::pixelColor(const QPoint &pt) const { return pixelColor(pt.x(), pt.y()); }
inline void QImage::setPixelColor(const QPoint &pt, const QColor &c) { setPixelColor(pt.x(), pt.y(), c); }

#if QT_DEPRECATED_SINCE(5, 0)

QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED

inline QString QImage::text(const char* key, const char* lang) const
{
    if (!d)
        return QString();
    QString k = QString::fromLatin1(key);
    if (lang && *lang)
        k += QLatin1Char('/') + QString::fromLatin1(lang);
    return text(k);
}

inline QList<QImageTextKeyLang> QImage::textList() const
{
    QList<QImageTextKeyLang> imageTextKeys;
    if (!d)
        return imageTextKeys;
    QStringList keys = textKeys();
    for (int i = 0; i < keys.size(); ++i) {
        int index = keys.at(i).indexOf(QLatin1Char('/'));
        if (index > 0) {
            QImageTextKeyLang tkl(true);
            tkl.key = keys.at(i).left(index).toLatin1();
            tkl.lang = keys.at(i).mid(index+1).toLatin1();
            imageTextKeys += tkl;
        }
    }

    return imageTextKeys;
}

inline QStringList QImage::textLanguages() const
{
    if (!d)
        return QStringList();
    QStringList keys = textKeys();
    QStringList languages;
    for (int i = 0; i < keys.size(); ++i) {
        int index = keys.at(i).indexOf(QLatin1Char('/'));
        if (index > 0)
            languages += keys.at(i).mid(index+1);
    }

    return languages;
}

inline QString QImage::text(const QImageTextKeyLang&kl) const
{
    if (!d)
        return QString();
    QString k = QString::fromLatin1(kl.key.constData());
    if (!kl.lang.isEmpty())
        k += QLatin1Char('/') + QString::fromLatin1(kl.lang.constData());
    return text(k);
}

inline void QImage::setText(const char* key, const char* lang, const QString &s)
{
    if (!d)
        return;
    detach();

    // In case detach() ran out of memory
    if (!d)
        return;

    QString k = QString::fromLatin1(key);
    if (lang && *lang)
        k += QLatin1Char('/') + QString::fromLatin1(lang);
    setText(k, s);
}

QT_WARNING_POP

inline int QImage::numColors() const
{
    return colorCount();
}

inline void QImage::setNumColors(int n)
{
    setColorCount(n);
}

inline int QImage::numBytes() const
{
    return int(sizeInBytes());
}
#endif

// QImage stream functions

#if !defined(QT_NO_DATASTREAM)
Q_GUI_EXPORT QDataStream &operator<<(QDataStream &, const QImage &);
Q_GUI_EXPORT QDataStream &operator>>(QDataStream &, QImage &);
#endif

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug, const QImage &);
#endif


QT_END_NAMESPACE

#endif // QIMAGE_H
