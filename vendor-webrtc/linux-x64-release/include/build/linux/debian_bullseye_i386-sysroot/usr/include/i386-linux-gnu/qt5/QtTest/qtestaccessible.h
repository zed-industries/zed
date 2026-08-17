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

#ifndef QTESTACCESSIBLE_H
#define QTESTACCESSIBLE_H

#if 0
// inform syncqt
#pragma qt_no_master_include
#endif

#include <QtCore/qglobal.h>

#define QVERIFY_EVENT(event) \
    QVERIFY(QTestAccessibility::verifyEvent(event))

#include <QtCore/qlist.h>
#include <QtCore/qdebug.h>
#include <QtGui/qaccessible.h>
#include <QtGui/qguiapplication.h>
#include <QtTest/qttestglobal.h>
#include <QtTest/qtestsystem.h>

#if QT_CONFIG(accessibility)

QT_BEGIN_NAMESPACE


class QObject;

// Use pointers since we subclass QAccessibleEvent
using EventList = QList<QAccessibleEvent*>;

bool operator==(const QAccessibleEvent &l, const QAccessibleEvent &r)
{
    if (l.type() != r.type()) {
//        qDebug() << "QAccessibleEvent with wrong type: " << qAccessibleEventString(l.type()) << " and " << qAccessibleEventString(r.type());
        return false;
    }
    if (l.object() != r.object() ||
            l.child() != r.child()) {
//        qDebug() << "QAccessibleEvent for wrong object: " << l.object() << " and " << r.object() << " child: " << l.child() << " and " << r.child();
        return false;
    }

    if (l.type() == QAccessible::StateChanged) {
        return static_cast<const QAccessibleStateChangeEvent*>(&l)->changedStates()
                == static_cast<const QAccessibleStateChangeEvent*>(&r)->changedStates();
    } else if (l.type() == QAccessible::TextCaretMoved) {
        return static_cast<const QAccessibleTextCursorEvent*>(&l)->cursorPosition()
                == static_cast<const QAccessibleTextCursorEvent*>(&r)->cursorPosition();
    } else if (l.type() == QAccessible::TextSelectionChanged) {
        const QAccessibleTextSelectionEvent *le = static_cast<const QAccessibleTextSelectionEvent*>(&l);
        const QAccessibleTextSelectionEvent *re = static_cast<const QAccessibleTextSelectionEvent*>(&r);
        return  le->cursorPosition() == re->cursorPosition() &&
                le->selectionStart() == re->selectionStart() &&
                le->selectionEnd() == re->selectionEnd();
    } else if (l.type() == QAccessible::TextInserted) {
        const QAccessibleTextInsertEvent *le = static_cast<const QAccessibleTextInsertEvent*>(&l);
        const QAccessibleTextInsertEvent *re = static_cast<const QAccessibleTextInsertEvent*>(&r);
        return  le->cursorPosition() == re->cursorPosition() &&
                le->changePosition() == re->changePosition() &&
                le->textInserted() == re->textInserted();
    } else if (l.type() == QAccessible::TextRemoved) {
        const QAccessibleTextRemoveEvent *le = static_cast<const QAccessibleTextRemoveEvent*>(&l);
        const QAccessibleTextRemoveEvent *re = static_cast<const QAccessibleTextRemoveEvent*>(&r);
        return  le->cursorPosition() == re->cursorPosition() &&
                le->changePosition() == re->changePosition() &&
                le->textRemoved() == re->textRemoved();
    } else if (l.type() == QAccessible::TextUpdated) {
        const QAccessibleTextUpdateEvent *le = static_cast<const QAccessibleTextUpdateEvent*>(&l);
        const QAccessibleTextUpdateEvent *re = static_cast<const QAccessibleTextUpdateEvent*>(&r);
        return  le->cursorPosition() == re->cursorPosition() &&
                le->changePosition() == re->changePosition() &&
                le->textInserted() == re->textInserted() &&
                le->textRemoved() == re->textRemoved();
    } else if (l.type() == QAccessible::ValueChanged) {
        const QAccessibleValueChangeEvent *le = static_cast<const QAccessibleValueChangeEvent*>(&l);
        const QAccessibleValueChangeEvent *re = static_cast<const QAccessibleValueChangeEvent*>(&r);
        return le->value() == re->value();
    }
    return true;
}

class QTestAccessibility
{
public:
    static void initialize()
    {
        if (!instance()) {
            instance() = new QTestAccessibility;
            qAddPostRoutine(cleanup);
        }
    }

    static void cleanup()
    {
        delete instance();
        instance() = nullptr;
    }
    static void clearEvents() { eventList().clear(); }
    static EventList events() { return eventList(); }
    static bool verifyEvent(QAccessibleEvent *ev)
    {
        for (int i = 0; eventList().isEmpty() && i < 5; ++i)
            QTest::qWait(50);
        if (eventList().isEmpty()) {
            qWarning("Timeout waiting for accessibility event.");
            return false;
        }
        const bool res = *eventList().first() == *ev;
        if (!res)
            qWarning("%s", qPrintable(msgAccessibilityEventListMismatch(eventList(), ev)));
        delete eventList().takeFirst();
        return res;
    }
    static bool containsEvent(QAccessibleEvent *event) {
        for (const QAccessibleEvent *ev : qAsConst(eventList())) {
            if (*ev == *event)
                return true;
        }
        return false;
    }

private:
    QTestAccessibility()
    {
        QAccessible::installUpdateHandler(updateHandler);
        QAccessible::installRootObjectHandler(rootObjectHandler);
    }

    ~QTestAccessibility()
    {
        QAccessible::installUpdateHandler(nullptr);
        QAccessible::installRootObjectHandler(nullptr);
    }

    static void rootObjectHandler(QObject *object)
    {
        //    qDebug("rootObjectHandler called %p", object);
        if (object) {
            QGuiApplication* app = qobject_cast<QGuiApplication*>(object);
            if ( !app )
                qWarning("root Object is not a QGuiApplication!");
        } else {
            qWarning("root Object called with 0 pointer");
        }
    }

    static void updateHandler(QAccessibleEvent *event)
    {
        eventList().append(copyEvent(event));
    }
    static QAccessibleEvent *copyEvent(QAccessibleEvent *event)
    {
        QAccessibleEvent *ev;
        if (event->type() == QAccessible::StateChanged) {
            if (event->object())
                ev = new QAccessibleStateChangeEvent(event->object(),
                        static_cast<QAccessibleStateChangeEvent*>(event)->changedStates());
            else
                ev = new QAccessibleStateChangeEvent(event->accessibleInterface(),
                        static_cast<QAccessibleStateChangeEvent*>(event)->changedStates());
        } else if (event->type() == QAccessible::TextCaretMoved) {
            if (event->object())
                ev = new QAccessibleTextCursorEvent(event->object(), static_cast<QAccessibleTextCursorEvent*>(event)->cursorPosition());
            else
                ev = new QAccessibleTextCursorEvent(event->accessibleInterface(), static_cast<QAccessibleTextCursorEvent*>(event)->cursorPosition());
        } else if (event->type() == QAccessible::TextSelectionChanged) {
            const QAccessibleTextSelectionEvent *original = static_cast<QAccessibleTextSelectionEvent*>(event);
            QAccessibleTextSelectionEvent *sel;
            if (event->object())
                sel = new QAccessibleTextSelectionEvent(event->object(), original->selectionStart(), original->selectionEnd());
            else
                sel = new QAccessibleTextSelectionEvent(event->accessibleInterface(), original->selectionStart(), original->selectionEnd());
            sel->setCursorPosition(original->cursorPosition());
            ev = sel;
        } else if (event->type() == QAccessible::TextInserted) {
            const QAccessibleTextInsertEvent *original = static_cast<QAccessibleTextInsertEvent*>(event);
            QAccessibleTextInsertEvent *ins;
            if (original->object())
                ins = new QAccessibleTextInsertEvent(event->object(), original->changePosition(), original->textInserted());
            else
                ins = new QAccessibleTextInsertEvent(event->accessibleInterface(), original->changePosition(), original->textInserted());
            ins->setCursorPosition(original->cursorPosition());
            ev = ins;
        } else if (event->type() == QAccessible::TextRemoved) {
            const QAccessibleTextRemoveEvent *original = static_cast<QAccessibleTextRemoveEvent*>(event);
            QAccessibleTextRemoveEvent *rem;
            if (event->object())
                rem = new QAccessibleTextRemoveEvent(event->object(), original->changePosition(), original->textRemoved());
            else
                rem = new QAccessibleTextRemoveEvent(event->accessibleInterface(), original->changePosition(), original->textRemoved());
            rem->setCursorPosition(original->cursorPosition());
            ev = rem;
        } else if (event->type() == QAccessible::TextUpdated) {
            const QAccessibleTextUpdateEvent *original = static_cast<QAccessibleTextUpdateEvent*>(event);
            QAccessibleTextUpdateEvent *upd;
            if (event->object())
                upd = new QAccessibleTextUpdateEvent(event->object(), original->changePosition(), original->textRemoved(), original->textInserted());
            else
                upd = new QAccessibleTextUpdateEvent(event->accessibleInterface(), original->changePosition(), original->textRemoved(), original->textInserted());
            upd->setCursorPosition(original->cursorPosition());
            ev = upd;
        } else if (event->type() == QAccessible::ValueChanged) {
            if (event->object())
                ev = new QAccessibleValueChangeEvent(event->object(), static_cast<QAccessibleValueChangeEvent*>(event)->value());
            else
                ev = new QAccessibleValueChangeEvent(event->accessibleInterface(), static_cast<QAccessibleValueChangeEvent*>(event)->value());
        } else if (event->type() == QAccessible::TableModelChanged) {
            QAccessibleTableModelChangeEvent *oldEvent = static_cast<QAccessibleTableModelChangeEvent*>(event);
            QAccessibleTableModelChangeEvent *newEvent;
            if (event->object())
                newEvent = new QAccessibleTableModelChangeEvent(event->object(), oldEvent->modelChangeType());
            else
                newEvent = new QAccessibleTableModelChangeEvent(event->accessibleInterface(), oldEvent->modelChangeType());
            newEvent->setFirstRow(oldEvent->firstRow());
            newEvent->setFirstColumn(oldEvent->firstColumn());
            newEvent->setLastRow(oldEvent->lastRow());
            newEvent->setLastColumn(oldEvent->lastColumn());
            ev = newEvent;
        } else {
            if (event->object())
                ev = new QAccessibleEvent(event->object(), event->type());
            else
                ev = new QAccessibleEvent(event->accessibleInterface(), event->type());
        }
        ev->setChild(event->child());
        return ev;
    }

    static EventList &eventList()
    {
        static EventList list;
        return list;
    }

    static QTestAccessibility *&instance()
    {
        static QTestAccessibility *ta = nullptr;
        return ta;
    }

private:
    static QString msgAccessibilityEventListMismatch(const EventList &haystack,
                                                     const QAccessibleEvent *needle)
    {
        QString rc;
        QDebug str = QDebug(&rc).nospace();
        str << "Event " << *needle
            <<  " not found at head of event list of size " << haystack.size() << " :";
        for (const QAccessibleEvent *e : haystack)
            str << ' ' << *e;
        return rc;
    }

};

QT_END_NAMESPACE

#endif // QT_CONFIG(accessibility)
#endif // QTESTACCESSIBLE_H
