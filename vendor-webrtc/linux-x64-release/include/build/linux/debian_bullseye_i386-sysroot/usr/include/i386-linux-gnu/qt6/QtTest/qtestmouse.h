// Copyright (C) 2021 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTESTMOUSE_H
#define QTESTMOUSE_H

#if 0
// inform syncqt
#pragma qt_no_master_include
#endif

#include <QtTest/qttestglobal.h>
#include <QtTest/qtestassert.h>
#include <QtTest/qtestsystem.h>
#include <QtTest/qtestspontaneevent.h>
#include <QtCore/qpoint.h>
#include <QtCore/qstring.h>
#include <QtCore/qpointer.h>
#include <QtGui/qevent.h>
#include <QtGui/qwindow.h>

#ifdef QT_WIDGETS_LIB
#include <QtWidgets/qapplication.h>
#include <QtWidgets/qwidget.h>
#endif

#include <QtCore/QDebug>

QT_BEGIN_NAMESPACE

Q_GUI_EXPORT void qt_handleMouseEvent(QWindow *window, const QPointF &local, const QPointF &global,
                                      Qt::MouseButtons state, Qt::MouseButton button,
                                      QEvent::Type type, Qt::KeyboardModifiers mods, int timestamp);

namespace QTestPrivate
{
    extern Q_TESTLIB_EXPORT Qt::MouseButtons qtestMouseButtons;
}

namespace QTest
{
    enum MouseAction { MousePress, MouseRelease, MouseClick, MouseDClick, MouseMove };

    extern Q_TESTLIB_EXPORT int lastMouseTimestamp;

    // This value is used to emulate timestamps to avoid creating double clicks by mistake.
    // Use this constant instead of QStyleHints::mouseDoubleClickInterval property to avoid tests
    // to depend on platform themes.
    static const int mouseDoubleClickInterval = 500;

    /*! \internal
        This function creates a QPA mouse event of type specified by \a action
        and calls QWindowSystemInterface::handleMouseEvent(), simulating the
        windowing system and bypassing the platform plugin. \a delay is the
        amount of time to be added to the simulated clock so that
        QInputEvent::timestamp() will be greater than that of the previous
        event. We expect all event-handling code to rely on the event
        timestamps, not the system clock; therefore tests can be run faster
        than real-time.
    */
    static void mouseEvent(MouseAction action, QWindow *window, Qt::MouseButton button,
                           Qt::KeyboardModifiers stateKey, QPoint pos, int delay=-1)
    {
        QTEST_ASSERT(window);
        extern int Q_TESTLIB_EXPORT defaultMouseDelay();

        // pos is in window local coordinates
        const QSize windowSize = window->geometry().size();
        if (windowSize.width() <= pos.x() || windowSize.height() <= pos.y()) {
            qWarning("Mouse event at %d, %d occurs outside target window (%dx%d).",
                     pos.x(), pos.y(), windowSize.width(), windowSize.height());
        }

        if (delay == -1 || delay < defaultMouseDelay())
            delay = defaultMouseDelay();
        lastMouseTimestamp += qMax(1, delay);

        if (pos.isNull())
            pos = QPoint(window->width() / 2, window->height() / 2);

        QTEST_ASSERT(!stateKey || stateKey & Qt::KeyboardModifierMask);

        stateKey &= Qt::KeyboardModifierMask;

        QPointF global = window->mapToGlobal(pos);
        QPointer<QWindow> w(window);

        using namespace QTestPrivate;
        switch (action)
        {
        case MouseDClick:
            qtestMouseButtons.setFlag(button, true);
            qt_handleMouseEvent(w, pos, global, qtestMouseButtons, button, QEvent::MouseButtonPress,
                                stateKey, lastMouseTimestamp);
            qtestMouseButtons.setFlag(button, false);
            qt_handleMouseEvent(w, pos, global, qtestMouseButtons, button, QEvent::MouseButtonRelease,
                                stateKey, lastMouseTimestamp);
            Q_FALLTHROUGH();
        case MousePress:
        case MouseClick:
            qtestMouseButtons.setFlag(button, true);
            qt_handleMouseEvent(w, pos, global, qtestMouseButtons, button, QEvent::MouseButtonPress,
                                stateKey, lastMouseTimestamp);
            if (action == MousePress)
                break;
            Q_FALLTHROUGH();
        case MouseRelease:
            qtestMouseButtons.setFlag(button, false);
            qt_handleMouseEvent(w, pos, global, qtestMouseButtons, button, QEvent::MouseButtonRelease,
                                stateKey, lastMouseTimestamp);
            lastMouseTimestamp += mouseDoubleClickInterval; // avoid double clicks being generated
            break;
        case MouseMove:
            qt_handleMouseEvent(w, pos, global, qtestMouseButtons, Qt::NoButton, QEvent::MouseMove,
                                stateKey, lastMouseTimestamp);
            break;
        default:
            QTEST_ASSERT(false);
        }
        qApp->processEvents();
    }

    inline void mousePress(QWindow *window, Qt::MouseButton button,
                           Qt::KeyboardModifiers stateKey = Qt::KeyboardModifiers(),
                           QPoint pos = QPoint(), int delay=-1)
    { mouseEvent(MousePress, window, button, stateKey, pos, delay); }
    inline void mouseRelease(QWindow *window, Qt::MouseButton button,
                             Qt::KeyboardModifiers stateKey = Qt::KeyboardModifiers(),
                             QPoint pos = QPoint(), int delay=-1)
    { mouseEvent(MouseRelease, window, button, stateKey, pos, delay); }
    inline void mouseClick(QWindow *window, Qt::MouseButton button,
                           Qt::KeyboardModifiers stateKey = Qt::KeyboardModifiers(),
                           QPoint pos = QPoint(), int delay=-1)
    { mouseEvent(MouseClick, window, button, stateKey, pos, delay); }
    inline void mouseDClick(QWindow *window, Qt::MouseButton button,
                            Qt::KeyboardModifiers stateKey = Qt::KeyboardModifiers(),
                            QPoint pos = QPoint(), int delay=-1)
    { mouseEvent(MouseDClick, window, button, stateKey, pos, delay); }
    inline void mouseMove(QWindow *window, QPoint pos = QPoint(), int delay=-1)
    { mouseEvent(MouseMove, window, Qt::NoButton, Qt::KeyboardModifiers(), pos, delay); }

#ifdef QT_WIDGETS_LIB
    static void mouseEvent(MouseAction action, QWidget *widget, Qt::MouseButton button,
                           Qt::KeyboardModifiers stateKey, QPoint pos, int delay=-1)
    {
        QTEST_ASSERT(widget);

        if (pos.isNull())
            pos = widget->rect().center();

#ifdef QTEST_QPA_MOUSE_HANDLING
        QWindow *w = widget->window()->windowHandle();
        QTEST_ASSERT(w);
        mouseEvent(action, w, button, stateKey, w->mapFromGlobal(widget->mapToGlobal(pos)), delay);
#else
        extern int Q_TESTLIB_EXPORT defaultMouseDelay();

        if (delay == -1 || delay < defaultMouseDelay())
            delay = defaultMouseDelay();
        lastMouseTimestamp += qMax(1, delay);

        if (action == MouseClick) {
            mouseEvent(MousePress, widget, button, stateKey, pos);
            mouseEvent(MouseRelease, widget, button, stateKey, pos);
            return;
        }

        QTEST_ASSERT(!stateKey || stateKey & Qt::KeyboardModifierMask);

        stateKey &= Qt::KeyboardModifierMask;

        QEvent::Type meType;
        using namespace QTestPrivate;
        switch (action)
        {
            case MousePress:
                qtestMouseButtons.setFlag(button, true);
                meType = QEvent::MouseButtonPress;
                break;
            case MouseRelease:
                qtestMouseButtons.setFlag(button, false);
                meType = QEvent::MouseButtonRelease;
                break;
            case MouseDClick:
                qtestMouseButtons.setFlag(button, true);
                meType = QEvent::MouseButtonDblClick;
                break;
            case MouseMove:
                // ### Qt 7: compatibility with < Qt 6.3, we should not rely on QCursor::setPos
                // for generating mouse move events, and code that depends on QCursor::pos should
                // be tested using QCursor::setPos explicitly.
                if (qtestMouseButtons == Qt::NoButton) {
                    QCursor::setPos(widget->mapToGlobal(pos));
                    qApp->processEvents();
                    return;
                }
                meType = QEvent::MouseMove;
                break;
            default:
                QTEST_ASSERT(false);
        }
        QMouseEvent me(meType, pos, widget->mapToGlobal(pos), button, qtestMouseButtons, stateKey, QPointingDevice::primaryPointingDevice());
        me.setTimestamp(lastMouseTimestamp);
        if (action == MouseRelease) // avoid double clicks being generated
            lastMouseTimestamp += mouseDoubleClickInterval;

        QSpontaneKeyEvent::setSpontaneous(&me);
        if (!qApp->notify(widget, &me)) {
            static const char *const mouseActionNames[] =
                { "MousePress", "MouseRelease", "MouseClick", "MouseDClick", "MouseMove" };
            qWarning("Mouse event \"%s\" not accepted by receiving widget",
                     mouseActionNames[static_cast<int>(action)]);
        }
#endif
    }

    inline void mousePress(QWidget *widget, Qt::MouseButton button,
                           Qt::KeyboardModifiers stateKey = Qt::KeyboardModifiers(),
                           QPoint pos = QPoint(), int delay=-1)
    { mouseEvent(MousePress, widget, button, stateKey, pos, delay); }
    inline void mouseRelease(QWidget *widget, Qt::MouseButton button,
                             Qt::KeyboardModifiers stateKey = Qt::KeyboardModifiers(),
                             QPoint pos = QPoint(), int delay=-1)
    { mouseEvent(MouseRelease, widget, button, stateKey, pos, delay); }
    inline void mouseClick(QWidget *widget, Qt::MouseButton button,
                           Qt::KeyboardModifiers stateKey = Qt::KeyboardModifiers(),
                           QPoint pos = QPoint(), int delay=-1)
    { mouseEvent(MouseClick, widget, button, stateKey, pos, delay); }
    inline void mouseDClick(QWidget *widget, Qt::MouseButton button,
                            Qt::KeyboardModifiers stateKey = Qt::KeyboardModifiers(),
                            QPoint pos = QPoint(), int delay=-1)
    { mouseEvent(MouseDClick, widget, button, stateKey, pos, delay); }
    inline void mouseMove(QWidget *widget, QPoint pos = QPoint(), int delay=-1)
    { mouseEvent(MouseMove, widget, Qt::NoButton, Qt::KeyboardModifiers(), pos, delay); }
#endif // QT_WIDGETS_LIB
}

QT_END_NAMESPACE

#endif // QTESTMOUSE_H
