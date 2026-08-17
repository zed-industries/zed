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

#ifndef QTOUCHDEVICE_H
#define QTOUCHDEVICE_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qobject.h>

QT_BEGIN_NAMESPACE

class QDebug;
class QTouchDevicePrivate;

class Q_GUI_EXPORT QTouchDevice
{
    Q_GADGET
public:
    enum DeviceType {
        TouchScreen,
        TouchPad
    };
    Q_ENUM(DeviceType)

    enum CapabilityFlag {
        Position = 0x0001,
        Area = 0x0002,
        Pressure = 0x0004,
        Velocity = 0x0008,
        RawPositions = 0x0010,
        NormalizedPosition = 0x0020,
        MouseEmulation = 0x0040
    };
    Q_FLAG(CapabilityFlag)
    Q_DECLARE_FLAGS(Capabilities, CapabilityFlag)

    QTouchDevice();
    ~QTouchDevice();

    static QList<const QTouchDevice *> devices();

    QString name() const;
    DeviceType type() const;
    Capabilities capabilities() const;
    int maximumTouchPoints() const;

    void setName(const QString &name);
    void setType(DeviceType devType);
    void setCapabilities(Capabilities caps);
    void setMaximumTouchPoints(int max);

private:
    QTouchDevicePrivate *d;
    friend class QTouchDevicePrivate;
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QTouchDevice::Capabilities)

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug, const QTouchDevice *);
#endif

QT_END_NAMESPACE

#endif // QTOUCHDEVICE_H
