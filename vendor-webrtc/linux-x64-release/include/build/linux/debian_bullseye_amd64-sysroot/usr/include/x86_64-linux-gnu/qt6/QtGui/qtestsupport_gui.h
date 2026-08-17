// Copyright (C) 2018 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTESTSUPPORT_GUI_H
#define QTESTSUPPORT_GUI_H

#include <QtGui/qtguiglobal.h>
#include <QtGui/qevent.h>
#include <QtCore/qmap.h>

QT_BEGIN_NAMESPACE

class QWindow;

Q_GUI_EXPORT void qt_handleTouchEvent(QWindow *w, const QPointingDevice *device,
                                const QList<QEventPoint> &points,
                                Qt::KeyboardModifiers mods = Qt::NoModifier);

Q_GUI_EXPORT bool qt_handleTouchEventv2(QWindow *w, const QPointingDevice *device,
                                const QList<QEventPoint> &points,
                                Qt::KeyboardModifiers mods = Qt::NoModifier);

namespace QTest {

[[nodiscard]] Q_GUI_EXPORT bool qWaitForWindowActive(QWindow *window, int timeout = 5000);
[[nodiscard]] Q_GUI_EXPORT bool qWaitForWindowExposed(QWindow *window, int timeout = 5000);

Q_GUI_EXPORT QPointingDevice * createTouchDevice(QInputDevice::DeviceType devType = QInputDevice::DeviceType::TouchScreen,
                                                 QInputDevice::Capabilities caps = QInputDevice::Capability::Position);

class Q_GUI_EXPORT QTouchEventSequence
{
public:
    virtual ~QTouchEventSequence();
    QTouchEventSequence& press(int touchId, const QPoint &pt, QWindow *window = nullptr);
    QTouchEventSequence& move(int touchId, const QPoint &pt, QWindow *window = nullptr);
    QTouchEventSequence& release(int touchId, const QPoint &pt, QWindow *window = nullptr);
    virtual QTouchEventSequence& stationary(int touchId);

    virtual bool commit(bool processEvents = true);

protected:
    QTouchEventSequence(QWindow *window, QPointingDevice *aDevice, bool autoCommit);

    QPoint mapToScreen(QWindow *window, const QPoint &pt);

    QEventPoint &point(int touchId);

    QEventPoint &pointOrPreviousPoint(int touchId);

    QMap<int, QEventPoint> previousPoints;
    QMap<int, QEventPoint> points;
    QWindow *targetWindow;
    QPointingDevice *device;
    bool commitWhenDestroyed;
    friend QTouchEventSequence touchEvent(QWindow *window, QPointingDevice *device, bool autoCommit);
};

} // namespace QTest

QT_END_NAMESPACE

#endif
