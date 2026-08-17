// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QBACKINGSTORE_H
#define QBACKINGSTORE_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qrect.h>

#include <QtGui/qwindow.h>
#include <QtGui/qregion.h>

QT_BEGIN_NAMESPACE


class QRegion;
class QRect;
class QPoint;
class QImage;
class QBackingStorePrivate;
class QPlatformBackingStore;

class Q_GUI_EXPORT QBackingStore
{
public:
    explicit QBackingStore(QWindow *window);
    ~QBackingStore();

    QWindow *window() const;

    QPaintDevice *paintDevice();

    void flush(const QRegion &region, QWindow *window = nullptr, const QPoint &offset = QPoint());

    void resize(const QSize &size);
    QSize size() const;

    bool scroll(const QRegion &area, int dx, int dy);

    void beginPaint(const QRegion &);
    void endPaint();

    void setStaticContents(const QRegion &region);
    QRegion staticContents() const;
    bool hasStaticContents() const;

    QPlatformBackingStore *handle() const;

private:
    QScopedPointer<QBackingStorePrivate> d_ptr;
};

QT_END_NAMESPACE

#endif // QBACKINGSTORE_H
