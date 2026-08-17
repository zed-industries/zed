// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QPAINTDEVICEWINDOW_H
#define QPAINTDEVICEWINDOW_H

#include <QtGui/qtguiglobal.h>
#include <QtGui/QWindow>
#include <QtGui/QPaintDevice>

QT_BEGIN_NAMESPACE

class QPaintDeviceWindowPrivate;
class QPaintEvent;

class Q_GUI_EXPORT QPaintDeviceWindow : public QWindow, public QPaintDevice
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QPaintDeviceWindow)

public:
    void update(const QRect &rect);
    void update(const QRegion &region);

    using QWindow::width;
    using QWindow::height;
    using QWindow::devicePixelRatio;

public Q_SLOTS:
    void update();

protected:
    void exposeEvent(QExposeEvent *) override;
    void paintEvent(QPaintEvent *event) override;

    int metric(PaintDeviceMetric metric) const override;
    bool event(QEvent *event) override;

    QPaintDeviceWindow(QPaintDeviceWindowPrivate &dd, QWindow *parent);

private:
    QPaintEngine *paintEngine() const override;
    Q_DISABLE_COPY(QPaintDeviceWindow)
};

QT_END_NAMESPACE

#endif
