// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QINPUTDEVICE_H
#define QINPUTDEVICE_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qobject.h>
#include <QtGui/qscreen.h>

QT_BEGIN_NAMESPACE

class QDebug;
class QInputDevicePrivate;

class Q_GUI_EXPORT QInputDevice : public QObject
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QInputDevice)
    Q_PROPERTY(QString name READ name CONSTANT)
    Q_PROPERTY(DeviceType type READ type CONSTANT)
    Q_PROPERTY(Capabilities capabilities READ capabilities CONSTANT)
    Q_PROPERTY(qint64 systemId READ systemId CONSTANT)
    Q_PROPERTY(QString seatName READ seatName CONSTANT)
    Q_PROPERTY(QRect availableVirtualGeometry READ availableVirtualGeometry
               NOTIFY availableVirtualGeometryChanged)

public:
    enum class DeviceType {
        Unknown = 0x0000,
        Mouse = 0x0001,
        TouchScreen = 0x0002,
        TouchPad = 0x0004,
        Puck = 0x0008,
        Stylus = 0x0010,
        Airbrush = 0x0020,
        Keyboard = 0x1000,
        AllDevices = 0x7FFFFFFF
    };
    Q_DECLARE_FLAGS(DeviceTypes, DeviceType)
    Q_FLAG(DeviceTypes)

    enum class Capability {
        None = 0,
        Position = 0x0001,
        Area = 0x0002,
        Pressure = 0x0004,
        Velocity = 0x0008,
        NormalizedPosition = 0x0020,
        MouseEmulation = 0x0040,
        PixelScroll = 0x0080,
        Scroll      = 0x0100,
        Hover       = 0x0200,
        Rotation    = 0x0400,
        XTilt       = 0x0800,
        YTilt       = 0x1000,
        TangentialPressure = 0x2000,
        ZPosition   = 0x4000,
        All = 0x7FFFFFFF
    };
    Q_DECLARE_FLAGS(Capabilities, Capability)
    Q_FLAG(Capabilities)

    QInputDevice(QObject *parent = nullptr);
    ~QInputDevice();
    QInputDevice(const QString &name, qint64 systemId, DeviceType type,
                 const QString &seatName = QString(), QObject *parent = nullptr);

    QString name() const;
    DeviceType type() const;
    Capabilities capabilities() const;
    bool hasCapability(Capability cap) const;
    qint64 systemId() const;
    QString seatName() const;
    QRect availableVirtualGeometry() const;

    static QStringList seatNames();
    static QList<const QInputDevice *> devices();
    static const QInputDevice *primaryKeyboard(const QString& seatName = QString());

    bool operator==(const QInputDevice &other) const;

Q_SIGNALS:
    void availableVirtualGeometryChanged(QRect area);

protected:
    QInputDevice(QInputDevicePrivate &d, QObject *parent);

    Q_DISABLE_COPY_MOVE(QInputDevice)
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QInputDevice::DeviceTypes)
Q_DECLARE_OPERATORS_FOR_FLAGS(QInputDevice::Capabilities)

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug, const QInputDevice *);
#endif

QT_END_NAMESPACE

#endif // QINPUTDEVICE_H
