// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSCREEN_H
#define QSCREEN_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/QList>
#include <QtCore/QObject>
#include <QtCore/QRect>
#include <QtCore/QSize>
#include <QtCore/QSizeF>

#include <QtGui/QTransform>

#include <QtCore/qnamespace.h>
#include <QtCore/qnativeinterface.h>

QT_BEGIN_NAMESPACE


class QPlatformScreen;
class QScreenPrivate;
class QWindow;
class QRect;
class QPixmap;
#ifndef QT_NO_DEBUG_STREAM
class QDebug;
#endif

class Q_GUI_EXPORT QScreen : public QObject
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QScreen)

    Q_PROPERTY(QString name READ name CONSTANT)
    Q_PROPERTY(QString manufacturer READ manufacturer CONSTANT)
    Q_PROPERTY(QString model READ model CONSTANT)
    Q_PROPERTY(QString serialNumber READ serialNumber CONSTANT)
    Q_PROPERTY(int depth READ depth CONSTANT)
    Q_PROPERTY(QSize size READ size NOTIFY geometryChanged)
    Q_PROPERTY(QSize availableSize READ availableSize NOTIFY availableGeometryChanged)
    Q_PROPERTY(QSize virtualSize READ virtualSize NOTIFY virtualGeometryChanged)
    Q_PROPERTY(QSize availableVirtualSize READ availableVirtualSize NOTIFY virtualGeometryChanged)
    Q_PROPERTY(QRect geometry READ geometry NOTIFY geometryChanged)
    Q_PROPERTY(QRect availableGeometry READ availableGeometry NOTIFY availableGeometryChanged)
    Q_PROPERTY(QRect virtualGeometry READ virtualGeometry NOTIFY virtualGeometryChanged)
    Q_PROPERTY(QRect availableVirtualGeometry READ availableVirtualGeometry
               NOTIFY virtualGeometryChanged)
    Q_PROPERTY(QSizeF physicalSize READ physicalSize NOTIFY physicalSizeChanged)
    Q_PROPERTY(qreal physicalDotsPerInchX READ physicalDotsPerInchX
               NOTIFY physicalDotsPerInchChanged)
    Q_PROPERTY(qreal physicalDotsPerInchY READ physicalDotsPerInchY
               NOTIFY physicalDotsPerInchChanged)
    Q_PROPERTY(qreal physicalDotsPerInch READ physicalDotsPerInch NOTIFY physicalDotsPerInchChanged)
    Q_PROPERTY(qreal logicalDotsPerInchX READ logicalDotsPerInchX NOTIFY logicalDotsPerInchChanged)
    Q_PROPERTY(qreal logicalDotsPerInchY READ logicalDotsPerInchY NOTIFY logicalDotsPerInchChanged)
    Q_PROPERTY(qreal logicalDotsPerInch READ logicalDotsPerInch NOTIFY logicalDotsPerInchChanged)
    Q_PROPERTY(qreal devicePixelRatio READ devicePixelRatio NOTIFY physicalDotsPerInchChanged)
    Q_PROPERTY(Qt::ScreenOrientation primaryOrientation READ primaryOrientation
               NOTIFY primaryOrientationChanged)
    Q_PROPERTY(Qt::ScreenOrientation orientation READ orientation NOTIFY orientationChanged)
    Q_PROPERTY(Qt::ScreenOrientation nativeOrientation READ nativeOrientation)
    Q_PROPERTY(qreal refreshRate READ refreshRate NOTIFY refreshRateChanged)

public:
    ~QScreen();
    QPlatformScreen *handle() const;

    QString name() const;

    QString manufacturer() const;
    QString model() const;
    QString serialNumber() const;

    int depth() const;

    QSize size() const;
    QRect geometry() const;

    QSizeF physicalSize() const;

    qreal physicalDotsPerInchX() const;
    qreal physicalDotsPerInchY() const;
    qreal physicalDotsPerInch() const;

    qreal logicalDotsPerInchX() const;
    qreal logicalDotsPerInchY() const;
    qreal logicalDotsPerInch() const;

    qreal devicePixelRatio() const;

    QSize availableSize() const;
    QRect availableGeometry() const;

    QList<QScreen *> virtualSiblings() const;
    QScreen *virtualSiblingAt(QPoint point);

    QSize virtualSize() const;
    QRect virtualGeometry() const;

    QSize availableVirtualSize() const;
    QRect availableVirtualGeometry() const;

    Qt::ScreenOrientation primaryOrientation() const;
    Qt::ScreenOrientation orientation() const;
    Qt::ScreenOrientation nativeOrientation() const;

    int angleBetween(Qt::ScreenOrientation a, Qt::ScreenOrientation b) const;
    QTransform transformBetween(Qt::ScreenOrientation a, Qt::ScreenOrientation b, const QRect &target) const;
    QRect mapBetween(Qt::ScreenOrientation a, Qt::ScreenOrientation b, const QRect &rect) const;

    bool isPortrait(Qt::ScreenOrientation orientation) const;
    bool isLandscape(Qt::ScreenOrientation orientation) const;

    QPixmap grabWindow(WId window = 0, int x = 0, int y = 0, int w = -1, int h = -1);

    qreal refreshRate() const;

    QT_DECLARE_NATIVE_INTERFACE_ACCESSOR(QScreen)

Q_SIGNALS:
    void geometryChanged(const QRect &geometry);
    void availableGeometryChanged(const QRect &geometry);
    void physicalSizeChanged(const QSizeF &size);
    void physicalDotsPerInchChanged(qreal dpi);
    void logicalDotsPerInchChanged(qreal dpi);
    void virtualGeometryChanged(const QRect &rect);
    void primaryOrientationChanged(Qt::ScreenOrientation orientation);
    void orientationChanged(Qt::ScreenOrientation orientation);
    void refreshRateChanged(qreal refreshRate);

private:
    explicit QScreen(QPlatformScreen *screen);

    Q_DISABLE_COPY(QScreen)
    friend class QGuiApplicationPrivate;
    friend class QPlatformIntegration;
    friend class QPlatformScreen;
    friend class QHighDpiScaling;
    friend class QWindowSystemInterface;
};

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug, const QScreen *);
#endif

QT_END_NAMESPACE

#endif // QSCREEN_H

