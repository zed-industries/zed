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

#ifndef QTESTEVENT_H
#define QTESTEVENT_H

#if 0
// inform syncqt
#pragma qt_no_master_include
#endif

#include <QtTest/qttestglobal.h>
#ifdef QT_GUI_LIB
#include <QtTest/qtestkeyboard.h>
#include <QtTest/qtestmouse.h>
#endif
#include <QtTest/qtestsystem.h>

#include <QtCore/qlist.h>

#include <stdlib.h>

QT_BEGIN_NAMESPACE


class QTestEvent
{
public:
#ifdef QT_WIDGETS_LIB
    virtual void simulate(QWidget *w) = 0;
#endif
    virtual QTestEvent *clone() const = 0;

    virtual ~QTestEvent() {}
};

#ifdef QT_GUI_LIB
class QTestKeyEvent: public QTestEvent
{
public:
    inline QTestKeyEvent(QTest::KeyAction action, Qt::Key key, Qt::KeyboardModifiers modifiers, int delay)
        : _action(action), _delay(delay), _modifiers(modifiers), _ascii(0), _key(key) {}
    inline QTestKeyEvent(QTest::KeyAction action, char ascii, Qt::KeyboardModifiers modifiers, int delay)
        : _action(action), _delay(delay), _modifiers(modifiers),
          _ascii(ascii), _key(Qt::Key_unknown) {}
    inline QTestEvent *clone() const override { return new QTestKeyEvent(*this); }

#ifdef QT_WIDGETS_LIB
    inline void simulate(QWidget *w) override
    {
        if (_ascii == 0)
            QTest::keyEvent(_action, w, _key, _modifiers, _delay);
        else
            QTest::keyEvent(_action, w, _ascii, _modifiers, _delay);
    }
#endif

protected:
    QTest::KeyAction _action;
    int _delay;
    Qt::KeyboardModifiers _modifiers;
    char _ascii;
    Qt::Key _key;
};

class QTestKeyClicksEvent: public QTestEvent
{
public:
    inline QTestKeyClicksEvent(const QString &keys, Qt::KeyboardModifiers modifiers, int delay)
        : _keys(keys), _modifiers(modifiers), _delay(delay)
        {
            Q_UNUSED(_delay) // Silence -Werror,-Wunused-private-field
        }
    inline QTestEvent *clone() const override { return new QTestKeyClicksEvent(*this); }

#ifdef QT_WIDGETS_LIB
    inline void simulate(QWidget *w) override
    {
        QTest::keyClicks(w, _keys, _modifiers, _delay);
    }
#endif

private:
    QString _keys;
    Qt::KeyboardModifiers _modifiers;
    int _delay;
};

class QTestMouseEvent: public QTestEvent
{
public:
    inline QTestMouseEvent(QTest::MouseAction action, Qt::MouseButton button,
            Qt::KeyboardModifiers modifiers, QPoint position, int delay)
        : _action(action), _button(button), _modifiers(modifiers), _pos(position), _delay(delay)
        {
            Q_UNUSED(_action)
            Q_UNUSED(_button)
            Q_UNUSED(_delay)
        }
    inline QTestEvent *clone() const override { return new QTestMouseEvent(*this); }

#ifdef QT_WIDGETS_LIB
    inline void simulate(QWidget *w) override
    {
        QTest::mouseEvent(_action, w, _button, _modifiers, _pos, _delay);
    }
#endif

private:
    QTest::MouseAction _action;
    Qt::MouseButton _button;
    Qt::KeyboardModifiers _modifiers;
    QPoint _pos;
    int _delay;
};
#endif //QT_GUI_LIB


class QTestDelayEvent: public QTestEvent
{
public:
    inline QTestDelayEvent(int msecs): _delay(msecs)
    {
        Q_UNUSED(_delay)
    }
    inline QTestEvent *clone() const override { return new QTestDelayEvent(*this); }

#ifdef QT_WIDGETS_LIB
    inline void simulate(QWidget * /*w*/) override { QTest::qWait(_delay); }
#endif

private:
    int _delay;
};

class QTestEventList: public QList<QTestEvent *>
{
public:
    inline QTestEventList() {}
    inline QTestEventList(const QTestEventList &other): QList<QTestEvent *>()
    { for (int i = 0; i < other.count(); ++i) append(other.at(i)->clone()); }
    inline ~QTestEventList()
    { clear(); }
    inline void clear()
    { qDeleteAll(*this); QList<QTestEvent *>::clear(); }

#ifdef QT_GUI_LIB
    inline void addKeyClick(Qt::Key qtKey, Qt::KeyboardModifiers modifiers = Qt::NoModifier, int msecs = -1)
    { addKeyEvent(QTest::Click, qtKey, modifiers, msecs); }
    inline void addKeyPress(Qt::Key qtKey, Qt::KeyboardModifiers modifiers = Qt::NoModifier, int msecs = -1)
    { addKeyEvent(QTest::Press, qtKey, modifiers, msecs); }
    inline void addKeyRelease(Qt::Key qtKey, Qt::KeyboardModifiers modifiers = Qt::NoModifier, int msecs = -1)
    { addKeyEvent(QTest::Release, qtKey, modifiers, msecs); }
    inline void addKeyEvent(QTest::KeyAction action, Qt::Key qtKey,
                            Qt::KeyboardModifiers modifiers = Qt::NoModifier, int msecs = -1)
    { append(new QTestKeyEvent(action, qtKey, modifiers, msecs)); }

    inline void addKeyClick(char ascii, Qt::KeyboardModifiers modifiers = Qt::NoModifier, int msecs = -1)
    { addKeyEvent(QTest::Click, ascii, modifiers, msecs); }
    inline void addKeyPress(char ascii, Qt::KeyboardModifiers modifiers = Qt::NoModifier, int msecs = -1)
    { addKeyEvent(QTest::Press, ascii, modifiers, msecs); }
    inline void addKeyRelease(char ascii, Qt::KeyboardModifiers modifiers = Qt::NoModifier, int msecs = -1)
    { addKeyEvent(QTest::Release, ascii, modifiers, msecs); }
    inline void addKeyClicks(const QString &keys, Qt::KeyboardModifiers modifiers = Qt::NoModifier, int msecs = -1)
    { append(new QTestKeyClicksEvent(keys, modifiers, msecs)); }
    inline void addKeyEvent(QTest::KeyAction action, char ascii, Qt::KeyboardModifiers modifiers = Qt::NoModifier, int msecs = -1)
    { append(new QTestKeyEvent(action, ascii, modifiers, msecs)); }

    inline void addMousePress(Qt::MouseButton button, Qt::KeyboardModifiers stateKey = Qt::KeyboardModifiers(),
                              QPoint pos = QPoint(), int delay=-1)
    { append(new QTestMouseEvent(QTest::MousePress, button, stateKey, pos, delay)); }
    inline void addMouseRelease(Qt::MouseButton button, Qt::KeyboardModifiers stateKey = Qt::KeyboardModifiers(),
                                QPoint pos = QPoint(), int delay=-1)
    { append(new QTestMouseEvent(QTest::MouseRelease, button, stateKey, pos, delay)); }
    inline void addMouseClick(Qt::MouseButton button, Qt::KeyboardModifiers stateKey = Qt::KeyboardModifiers(),
                              QPoint pos = QPoint(), int delay=-1)
    { append(new QTestMouseEvent(QTest::MouseClick, button, stateKey, pos, delay)); }
    inline void addMouseDClick(Qt::MouseButton button, Qt::KeyboardModifiers stateKey = Qt::KeyboardModifiers(),
                            QPoint pos = QPoint(), int delay=-1)
    { append(new QTestMouseEvent(QTest::MouseDClick, button, stateKey, pos, delay)); }
    inline void addMouseMove(QPoint pos = QPoint(), int delay=-1)
    { append(new QTestMouseEvent(QTest::MouseMove, Qt::NoButton, Qt::KeyboardModifiers(), pos, delay)); }
#endif //QT_GUI_LIB

    inline void addDelay(int msecs)
    { append(new QTestDelayEvent(msecs)); }

#ifdef QT_WIDGETS_LIB
    inline void simulate(QWidget *w)
    {
        for (int i = 0; i < count(); ++i)
            at(i)->simulate(w);
    }
#endif
};

QT_END_NAMESPACE

Q_DECLARE_METATYPE(QTestEventList)

#endif
