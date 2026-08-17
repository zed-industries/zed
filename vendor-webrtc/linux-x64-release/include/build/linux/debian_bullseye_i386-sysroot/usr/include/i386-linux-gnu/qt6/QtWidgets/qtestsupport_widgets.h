// Copyright (C) 2018 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTESTSUPPORT_WIDGETS_H
#define QTESTSUPPORT_WIDGETS_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtGui/qtestsupport_gui.h>

QT_BEGIN_NAMESPACE

class QPointingDevice;
class QWidget;

namespace QTest {

[[nodiscard]] Q_WIDGETS_EXPORT bool qWaitForWindowActive(QWidget *widget, int timeout = 5000);
[[nodiscard]] Q_WIDGETS_EXPORT bool qWaitForWindowExposed(QWidget *widget, int timeout = 5000);

class Q_WIDGETS_EXPORT QTouchEventWidgetSequence : public QTouchEventSequence
{
public:
    ~QTouchEventWidgetSequence() override;
    QTouchEventWidgetSequence& press(int touchId, const QPoint &pt, QWidget *widget = nullptr);
    QTouchEventWidgetSequence& move(int touchId, const QPoint &pt, QWidget *widget = nullptr);
    QTouchEventWidgetSequence& release(int touchId, const QPoint &pt, QWidget *widget = nullptr);
    QTouchEventWidgetSequence& stationary(int touchId) override;

    bool commit(bool processEvents = true) override;

private:
    QTouchEventWidgetSequence(QWidget *widget, QPointingDevice *aDevice, bool autoCommit);

    QPoint mapToScreen(QWidget *widget, const QPoint &pt);

    QWidget *targetWidget = nullptr;

    friend QTouchEventWidgetSequence touchEvent(QWidget *widget, QPointingDevice *device, bool autoCommit);
};

} // namespace QTest

QT_END_NAMESPACE

#endif
