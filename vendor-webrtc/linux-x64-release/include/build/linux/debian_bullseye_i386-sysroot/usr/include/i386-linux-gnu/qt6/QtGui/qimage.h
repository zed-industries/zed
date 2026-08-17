// Copyright (C) 2021 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QIMAGE_H
#define QIMAGE_H

#include <QtGui/qtguiglobal.h>
#include <QtGui/qcolor.h>
#include <QtGui/qrgb.h>
#include <QtGui/qpaintdevice.h>
#include <QtGui/qpixelformat.h>
#include <QtGui/qtransform.h>
#include <QtCore/qbytearray.h>
#include <QtCore/qbytearrayview.h>
#include <QtCore/qrect.h>
#include <QtCore/qstring.h>
#include <QtCore/qcontainerfwd.h>

#if defined(Q_OS_DARWIN) || defined(Q_QDOC)
Q_FORWARD_DECLARE_MUTABLE_CG_TYPE(CGImage);
#endif

QT_BEGIN_NAMESPACE


class QColorSpace;
class QColorTransform;
class QIODevice;
class QTransform;
class QVariant;

struct QImageData;

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
        Format_RGBX16FPx4,
        Format_RGBA16FPx4,
        Format_RGBA16FPx4_Premultiplied,
        Format_RGBX32FPx4,
        Format_RGBA32FPx4,
        Format_RGBA32FPx4_Premultiplied,
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
    QImage(uchar *data, int width, int height, qsizetype bytesPerLine, Format format, QImageCleanupFunction cleanupFunction = nullptr, void *cleanupInfo = nullptr);
    QImage(const uchar *data, int width, int height, qsizetype bytesPerLine, Format format, QImageCleanupFunction cleanupFunction = nullptr, void *cleanupInfo = nullptr);

#ifndef QT_NO_IMAGEFORMAT_XPM
    explicit QImage(const char * const xpm[]);
#endif
    explicit QImage(const QString &fileName, const char *format = nullptr);

    QImage(const QImage &);
    QImage(QImage &&other) noexcept
        : QPaintDevice(), d(qExchange(other.d, nullptr))
    {}
    ~QImage();

    QImage &operator=(const QImage &);
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_MOVE_AND_SWAP(QImage)
    void swap(QImage &other) noexcept
    { qt_ptr_swap(d, other.d); }

    bool isNull() const;

    int devType() const override;

    bool operator==(const QImage &) const;
    bool operator!=(const QImage &) const;
    operator QVariant() const;
    void detach();
    bool isDetached() const;

    [[nodiscard]] QImage copy(const QRect &rect = QRect()) const;
    [[nodiscard]] QImage copy(int x, int y, int w, int h) const
    { return copy(QRect(x, y, w, h)); }

    Format format() const;

    [[nodiscard]] QImage convertToFormat(Format f, Qt::ImageConversionFlags flags = Qt::AutoColor) const &
    { return convertToFormat_helper(f, flags); }
    [[nodiscard]] QImage convertToFormat(Format f, Qt::ImageConversionFlags flags = Qt::AutoColor) &&
    {
        if (convertToFormat_inplace(f, flags))
            return std::move(*this);
        else
            return convertToFormat_helper(f, flags);
    }
    [[nodiscard]] QImage convertToFormat(Format f, const QList<QRgb> &colorTable,
                                         Qt::ImageConversionFlags flags = Qt::AutoColor) const;

    bool reinterpretAsFormat(Format f);
    [[nodiscard]] QImage convertedTo(Format f, Qt::ImageConversionFlags flags = Qt::AutoColor) const &
    { return convertToFormat(f, flags); }
    [[nodiscard]] QImage convertedTo(Format f, Qt::ImageConversionFlags flags = Qt::AutoColor) &&
    { return convertToFormat(f, flags); }
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

    qsizetype sizeInBytes() const;

    uchar *scanLine(int);
    const uchar *scanLine(int) const;
    const uchar *constScanLine(int) const;
    qsizetype bytesPerLine() const;

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

    QList<QRgb> colorTable() const;
    void setColorTable(const QList<QRgb> &colors);

    qreal devicePixelRatio() const;
    void setDevicePixelRatio(qreal scaleFactor);
    QSizeF deviceIndependentSize() const;

    void fill(uint pixel);
    void fill(const QColor &color);
    void fill(Qt::GlobalColor color);


    bool hasAlphaChannel() const;
    void setAlphaChannel(const QImage &alphaChannel);
    [[nodiscard]] QImage createAlphaMask(Qt::ImageConversionFlags flags = Qt::AutoColor) const;
#ifndef QT_NO_IMAGE_HEURISTIC_MASK
    [[nodiscard]] QImage createHeuristicMask(bool clipTight = true) const;
#endif
    [[nodiscard]] QImage createMaskFromColor(QRgb color, Qt::MaskMode mode = Qt::MaskInColor) const;

    [[nodiscard]] QImage scaled(int w, int h, Qt::AspectRatioMode aspectMode = Qt::IgnoreAspectRatio,
                                Qt::TransformationMode mode = Qt::FastTransformation) const
    { return scaled(QSize(w, h), aspectMode, mode); }
    [[nodiscard]] QImage scaled(const QSize &s, Qt::AspectRatioMode aspectMode = Qt::IgnoreAspectRatio,
                                Qt::TransformationMode mode = Qt::FastTransformation) const;
    [[nodiscard]] QImage scaledToWidth(int w, Qt::TransformationMode mode = Qt::FastTransformation) const;
    [[nodiscard]] QImage scaledToHeight(int h, Qt::TransformationMode mode = Qt::FastTransformation) const;
    [[nodiscard]] QImage transformed(const QTransform &matrix, Qt::TransformationMode mode = Qt::FastTransformation) const;
    static QTransform trueMatrix(const QTransform &, int w, int h);

    [[nodiscard]] QImage mirrored(bool horizontally = false, bool vertically = true) const &
    { return mirrored_helper(horizontally, vertically); }
    [[nodiscard]] QImage mirrored(bool horizontally = false, bool vertically = true) &&
    { mirrored_inplace(horizontally, vertically); return std::move(*this); }
    [[nodiscard]] QImage rgbSwapped() const &
    { return rgbSwapped_helper(); }
    [[nodiscard]] QImage rgbSwapped() &&
    { rgbSwapped_inplace(); return std::move(*this); }
    void mirror(bool horizontally = false, bool vertically = true)
    { mirrored_inplace(horizontally, vertically); }
    void rgbSwap()
    { rgbSwapped_inplace(); }
    void invertPixels(InvertMode = InvertRgb);

    QColorSpace colorSpace() const;
    [[nodiscard]] QImage convertedToColorSpace(const QColorSpace &) const;
    void convertToColorSpace(const QColorSpace &);
    void setColorSpace(const QColorSpace &);

    QImage colorTransformed(const QColorTransform &transform) const &;
    QImage colorTransformed(const QColorTransform &transform) &&;
    void applyColorTransform(const QColorTransform &transform);

    bool load(QIODevice *device, const char *format);
    bool load(const QString &fileName, const char *format = nullptr);
    bool loadFromData(QByteArrayView data, const char *format = nullptr);
    bool loadFromData(const uchar *buf, int len, const char *format = nullptr); // ### Qt 7: qsizetype
    bool loadFromData(const QByteArray &data, const char *format = nullptr) // ### Qt 7: drop
    { return loadFromData(QByteArrayView(data), format); }

    bool save(const QString &fileName, const char *format = nullptr, int quality = -1) const;
    bool save(QIODevice *device, const char *format = nullptr, int quality = -1) const;

    static QImage fromData(QByteArrayView data, const char *format = nullptr);
    static QImage fromData(const uchar *data, int size, const char *format = nullptr); // ### Qt 7: qsizetype
    static QImage fromData(const QByteArray &data, const char *format = nullptr)  // ### Qt 7: drop
    { return fromData(QByteArrayView(data), format); }

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
#if defined(Q_OS_WIN) || defined(Q_QDOC)
    HBITMAP toHBITMAP() const;
    HICON toHICON(const QImage &mask = {}) const;
    static QImage fromHBITMAP(HBITMAP hbitmap);
    static QImage fromHICON(HICON icon);
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

    void detachMetadata(bool invalidateCache = false);

private:
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
