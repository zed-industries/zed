/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtTest module of the Qt Toolkit.
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

#ifndef QTESTTOUCH_H
#define QTESTTOUCH_H

#if 0
// inform syncqt
#pragma qt_no_master_include
#endif

#include <QtTest/qttestglobal.h>
#include <QtTest/qtestassert.h>
#include <QtTest/qtestsystem.h>
#include <QtTest/qtestspontaneevent.h>
#include <QtCore/qmap.h>
#include <QtGui/qevent.h>
#include <QtGui/qwindow.h>
#ifdef QT_WIDGETS_LIB
#include <QtWidgets/qwidget.h>
#endif

QT_BEGIN_NAMESPACE

Q_GUI_EXPORT  void qt_handleTouchEvent(QWindow *w, QTouchDevice *device,
                                const QList<QTouchEvent::TouchPoint> &points,
                                Qt::KeyboardModifiers mods = Qt::NoModifier);


namespace QTest
{
    Q_GUI_EXPORT QTouchDevice * createTouchDevice(QTouchDevice::DeviceType devType = QTouchDevice::TouchScreen);

    class QTouchEventSequence
    {
    public:
        ~QTouchEventSequence()
        {
            if (commitWhenDestroyed)
                commit();
        }
        QTouchEventSequence& press(int touchId, const QPoint &pt, QWindow *window = nullptr)
        {
            QTouchEvent::TouchPoint &p = point(touchId);
            p.setScreenPos(mapToScreen(window, pt));
            p.setState(Qt::TouchPointPressed);
            return *this;
        }
        QTouchEventSequence& move(int touchId, const QPoint &pt, QWindow *window = nullptr)
        {
            QTouchEvent::TouchPoint &p = point(touchId);
            p.setScreenPos(mapToScreen(window, pt));
            p.setState(Qt::TouchPointMoved);
            return *this;
        }
        QTouchEventSequence& release(int touchId, const QPoint &pt, QWindow *window = nullptr)
        {
            QTouchEvent::TouchPoint &p = point(touchId);
            p.setScreenPos(mapToScreen(window, pt));
            p.setState(Qt::TouchPointReleased);
            return *this;
        }
        QTouchEventSequence& stationary(int touchId)
        {
            QTouchEvent::TouchPoint &p = pointOrPreviousPoint(touchId);
            p.setState(Qt::TouchPointStationary);
            return *this;
        }

#ifdef QT_WIDGETS_LIB
        QTouchEventSequence& press(int touchId, const QPoint &pt, QWidget *widget = nullptr)
        {
            QTouchEvent::TouchPoint &p = point(touchId);
            p.setScreenPos(mapToScreen(widget, pt));
            p.setState(Qt::TouchPointPressed);
            return *this;
        }
        QTouchEventSequence& move(int touchId, const QPoint &pt, QWidget *widget = nullptr)
        {
            QTouchEvent::TouchPoint &p = point(touchId);
            p.setScreenPos(mapToScreen(widget, pt));
            p.setState(Qt::TouchPointMoved);
            return *this;
        }
        QTouchEventSequence& release(int touchId, const QPoint &pt, QWidget *widget = nullptr)
        {
            QTouchEvent::TouchPoint &p = point(touchId);
            p.setScreenPos(mapToScreen(widget, pt));
            p.setState(Qt::TouchPointReleased);
            return *this;
        }
#endif

        void commit(bool processEvents = true)
        {
            if (!points.isEmpty()) {
                qSleep(1);
                if (targetWindow)
                {
                    qt_handleTouchEvent(targetWindow, device, points.values());
                }
#ifdef QT_WIDGETS_LIB
                else if (targetWidget)
                {
                    qt_handleTouchEvent(targetWidget->windowHandle(), device, points.values());
                }
#endif
            }
            if (processEvents)
                QCoreApplication::processEvents();
            previousPoints = points;
            points.clear();
        }

private:
#ifdef QT_WIDGETS_LIB
        QTouchEventSequence(QWidget *widget, QTouchDevice *aDevice, bool autoCommit)
            : targetWidget(widget), targetWindow(nullptr), device(aDevice), commitWhenDestroyed(autoCommit)
        {
        }
#endif
        QTouchEventSequence(QWindow *window, QTouchDevice *aDevice, bool autoCommit)
            :
#ifdef QT_WIDGETS_LIB
              targetWidget(nullptr),
#endif
              targetWindow(window), device(aDevice), commitWhenDestroyed(autoCommit)
        {
        }

        QTouchEvent::TouchPoint &point(int touchId)
        {
            if (!points.contains(touchId))
                points[touchId] = QTouchEvent::TouchPoint(touchId);
            return points[touchId];
        }

        QTouchEvent::TouchPoint &pointOrPreviousPoint(int touchId)
        {
            if (!points.contains(touchId)) {
                if (previousPoints.contains(touchId))
                    points[touchId] = previousPoints.value(touchId);
                else
                    points[touchId] = QTouchEvent::TouchPoint(touchId);
            }
            return points[touchId];
        }

#ifdef QT_WIDGETS_LIB
        QPoint mapToScreen(QWidget *widget, const QPoint &pt)
        {
            if (widget)
                return widget->mapToGlobal(pt);
            return targetWidget ? targetWidget->mapToGlobal(pt) : pt;
        }
#endif
        QPoint mapToScreen(QWindow *window, const QPoint &pt)
        {
            if(window)
                return window->mapToGlobal(pt);
            return targetWindow ? targetWindow->mapToGlobal(pt) : pt;
        }

        QMap<int, QTouchEvent::TouchPoint> previousPoints;
        QMap<int, QTouchEvent::TouchPoint> points;
#ifdef QT_WIDGETS_LIB
        QWidget *targetWidget;
#endif
        QWindow *targetWindow;
        QTouchDevice *device;
        bool commitWhenDestroyed;
#if defined(QT_WIDGETS_LIB) || defined(Q_CLANG_QDOC)
        friend QTouchEventSequence touchEvent(QWidget *widget, QTouchDevice *device, bool autoCommit);
#endif
        friend QTouchEventSequence touchEvent(QWindow *window, QTouchDevice *device, bool autoCommit);
    };

#if defined(QT_WIDGETS_LIB) || defined(Q_CLANG_QDOC)
    inline
    QTouchEventSequence touchEvent(QWidget *widget,
                                   QTouchDevice *device,
                                   bool autoCommit = true)
    {
        return QTouchEventSequence(widget, device, autoCommit);
    }
#endif
    inline
    QTouchEventSequence touchEvent(QWindow *window,
                                   QTouchDevice *device,
                                   bool autoCommit = true)
    {
        return QTouchEventSequence(window, device, autoCommit);
    }

}

QT_END_NAMESPACE

#endif // QTESTTOUCH_H
