// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QBITMAP_H
#define QBITMAP_H

#include <QtGui/qtguiglobal.h>
#include <QtGui/qpixmap.h>

QT_BEGIN_NAMESPACE


class QVariant;

class Q_GUI_EXPORT QBitmap : public QPixmap
{
public:
    QBitmap();
#if QT_DEPRECATED_SINCE(6, 0)
    QT_DEPRECATED_VERSION_X_6_0("Use fromPixmap instead.") explicit QBitmap(const QPixmap &);
#endif
    QBitmap(int w, int h);
    explicit QBitmap(const QSize &);
    explicit QBitmap(const QString &fileName, const char *format = nullptr);
    ~QBitmap() override;

#if QT_DEPRECATED_SINCE(6, 0)
    QT_DEPRECATED_VERSION_X_6_0("Use fromPixmap instead.") QBitmap &operator=(const QPixmap &);
#endif
    inline void swap(QBitmap &other) { QPixmap::swap(other); } // prevent QBitmap<->QPixmap swaps
    operator QVariant() const;

    inline void clear() { fill(Qt::color0); }

    static QBitmap fromImage(const QImage &image, Qt::ImageConversionFlags flags = Qt::AutoColor);
    static QBitmap fromImage(QImage &&image, Qt::ImageConversionFlags flags = Qt::AutoColor);
    static QBitmap fromData(const QSize &size, const uchar *bits,
                            QImage::Format monoFormat = QImage::Format_MonoLSB);
    static QBitmap fromPixmap(const QPixmap &pixmap);

    QBitmap transformed(const QTransform &matrix) const;

    typedef QExplicitlySharedDataPointer<QPlatformPixmap> DataPtr;
};
Q_DECLARE_SHARED(QBitmap)

QT_END_NAMESPACE

#endif // QBITMAP_H
