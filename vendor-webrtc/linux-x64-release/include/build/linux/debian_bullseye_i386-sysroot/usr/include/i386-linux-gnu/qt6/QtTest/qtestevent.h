// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

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

#ifdef QT_WIDGETS_LIB
# define QT_ONLY_WIDGETLIB_USES
#else
# define QT_ONLY_WIDGETLIB_USES Q_DECL_UNUSED_MEMBER
#endif

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
        : _keys(keys), _modifiers(modifiers), _delay(delay) {}
    inline QTestEvent *clone() const override { return new QTestKeyClicksEvent(*this); }

#ifdef QT_WIDGETS_LIB
    inline void simulate(QWidget *w) override
    {
        QTest::keyClicks(w, _keys, _modifiers, _delay);
    }
#endif

private:
    QT_ONLY_WIDGETLIB_USES QString _keys;
    QT_ONLY_WIDGETLIB_USES Qt::KeyboardModifiers _modifiers;
    QT_ONLY_WIDGETLIB_USES int _delay;
};

class QTestMouseEvent: public QTestEvent
{
public:
    inline QTestMouseEvent(QTest::MouseAction action, Qt::MouseButton button,
            Qt::KeyboardModifiers modifiers, QPoint position, int delay)
        : _action(action), _button(button), _modifiers(modifiers), _pos(position), _delay(delay) {}
    inline QTestEvent *clone() const override { return new QTestMouseEvent(*this); }

#ifdef QT_WIDGETS_LIB
    inline void simulate(QWidget *w) override
    {
        QTest::mouseEvent(_action, w, _button, _modifiers, _pos, _delay);
    }
#endif

private:
    QT_ONLY_WIDGETLIB_USES QTest::MouseAction _action;
    QT_ONLY_WIDGETLIB_USES Qt::MouseButton _button;
    QT_ONLY_WIDGETLIB_USES Qt::KeyboardModifiers _modifiers;
    QT_ONLY_WIDGETLIB_USES QPoint _pos;
    QT_ONLY_WIDGETLIB_USES int _delay;
};
#endif //QT_GUI_LIB


class QTestDelayEvent: public QTestEvent
{
public:
    inline QTestDelayEvent(int msecs): _delay(msecs) {}
    inline QTestEvent *clone() const override { return new QTestDelayEvent(*this); }

#ifdef QT_WIDGETS_LIB
    inline void simulate(QWidget * /*w*/) override { QTest::qWait(_delay); }
#endif

private:
    QT_ONLY_WIDGETLIB_USES int _delay;
};

class QTestEventList: public QList<QTestEvent *>
{
public:
    inline QTestEventList() {}
    inline QTestEventList(const QTestEventList &other): QList<QTestEvent *>()
    { for (int i = 0; i < other.size(); ++i) append(other.at(i)->clone()); }
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
        for (int i = 0; i < size(); ++i)
            at(i)->simulate(w);
    }
#endif
};

#undef QT_ONLY_WIDGETLIB_USES

QT_END_NAMESPACE

Q_DECLARE_METATYPE(QTestEventList)

#endif
