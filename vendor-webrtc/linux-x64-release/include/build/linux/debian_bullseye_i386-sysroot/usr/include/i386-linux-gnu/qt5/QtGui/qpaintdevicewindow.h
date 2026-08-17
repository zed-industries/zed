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
    virtual void paintEvent(QPaintEvent *event);

    int metric(PaintDeviceMetric metric) const override;
    void exposeEvent(QExposeEvent *) override;
    bool event(QEvent *event) override;

    QPaintDeviceWindow(QPaintDeviceWindowPrivate &dd, QWindow *parent);

private:
    QPaintEngine *paintEngine() const override;
    Q_DISABLE_COPY(QPaintDeviceWindow)
};

QT_END_NAMESPACE

#endif
