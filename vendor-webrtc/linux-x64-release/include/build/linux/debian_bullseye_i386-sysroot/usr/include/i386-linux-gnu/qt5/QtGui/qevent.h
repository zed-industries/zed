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

#ifndef QEVENT_H
#define QEVENT_H

#include <QtGui/qtguiglobal.h>
#include <QtGui/qwindowdefs.h>
#include <QtGui/qregion.h>
#include <QtCore/qnamespace.h>
#include <QtCore/qstring.h>
#include <QtGui/qkeysequence.h>
#include <QtCore/qcoreevent.h>
#include <QtCore/qvariant.h>
#include <QtCore/qmap.h> // ### Qt 6: Remove
#include <QtCore/qvector.h>
#include <QtCore/qset.h> // ### Qt 6: Remove
#include <QtCore/qurl.h>
#include <QtCore/qfile.h> // ### Qt 6: Replace by <QtCore/qiodevice.h> and forward declare QFile
#include <QtGui/qvector2d.h>
#include <QtGui/qtouchdevice.h> // ### Qt 6: Replace by forward declaration

QT_BEGIN_NAMESPACE


class QAction;
#ifndef QT_NO_GESTURES
class QGesture;
#endif
class QScreen;

class Q_GUI_EXPORT QInputEvent : public QEvent
{
public:
    explicit QInputEvent(Type type, Qt::KeyboardModifiers modifiers = Qt::NoModifier);
    ~QInputEvent();
    inline Qt::KeyboardModifiers modifiers() const { return modState; }
    inline void setModifiers(Qt::KeyboardModifiers amodifiers) { modState = amodifiers; }
    inline ulong timestamp() const { return ts; }
    inline void setTimestamp(ulong atimestamp) { ts = atimestamp; }
protected:
    Qt::KeyboardModifiers modState;
    ulong ts;
};

class Q_GUI_EXPORT QEnterEvent : public QEvent
{
public:
    QEnterEvent(const QPointF &localPos, const QPointF &windowPos, const QPointF &screenPos);
    ~QEnterEvent();

#ifndef QT_NO_INTEGER_EVENT_COORDINATES
    inline QPoint pos() const { return l.toPoint(); }
    inline QPoint globalPos() const { return s.toPoint(); }
    inline int x() const { return qRound(l.x()); }
    inline int y() const { return qRound(l.y()); }
    inline int globalX() const { return qRound(s.x()); }
    inline int globalY() const { return qRound(s.y()); }
#endif
    const QPointF &localPos() const { return l; }
    const QPointF &windowPos() const { return w; }
    const QPointF &screenPos() const { return s; }

protected:
    QPointF l, w, s;
};

class Q_GUI_EXPORT QMouseEvent : public QInputEvent
{
public:
    QMouseEvent(Type type, const QPointF &localPos, Qt::MouseButton button,
                Qt::MouseButtons buttons, Qt::KeyboardModifiers modifiers);
    QMouseEvent(Type type, const QPointF &localPos, const QPointF &screenPos,
                Qt::MouseButton button, Qt::MouseButtons buttons,
                Qt::KeyboardModifiers modifiers);
    QMouseEvent(Type type, const QPointF &localPos, const QPointF &windowPos, const QPointF &screenPos,
                Qt::MouseButton button, Qt::MouseButtons buttons,
                Qt::KeyboardModifiers modifiers);
    QMouseEvent(Type type, const QPointF &localPos, const QPointF &windowPos, const QPointF &screenPos,
                Qt::MouseButton button, Qt::MouseButtons buttons,
                Qt::KeyboardModifiers modifiers, Qt::MouseEventSource source);
    ~QMouseEvent();

#ifndef QT_NO_INTEGER_EVENT_COORDINATES
    inline QPoint pos() const { return l.toPoint(); }
    inline QPoint globalPos() const { return s.toPoint(); }
    inline int x() const { return qRound(l.x()); }
    inline int y() const { return qRound(l.y()); }
    inline int globalX() const { return qRound(s.x()); }
    inline int globalY() const { return qRound(s.y()); }
#endif
    const QPointF &localPos() const { return l; }
    const QPointF &windowPos() const { return w; }
    const QPointF &screenPos() const { return s; }

    inline Qt::MouseButton button() const { return b; }
    inline Qt::MouseButtons buttons() const { return mouseState; }

    inline void setLocalPos(const QPointF &localPosition) { l = localPosition; }

#if QT_DEPRECATED_SINCE(5, 0)
    QT_DEPRECATED inline QPointF posF() const { return l; }
#endif

    Qt::MouseEventSource source() const;
    Qt::MouseEventFlags flags() const;

protected:
    QPointF l, w, s;
    Qt::MouseButton b;
    Qt::MouseButtons mouseState;
    int caps;
    QVector2D velocity;

    friend class QGuiApplicationPrivate;
};

class Q_GUI_EXPORT QHoverEvent : public QInputEvent
{
public:
    QHoverEvent(Type type, const QPointF &pos, const QPointF &oldPos, Qt::KeyboardModifiers modifiers = Qt::NoModifier);
    ~QHoverEvent();

#ifndef QT_NO_INTEGER_EVENT_COORDINATES
    inline QPoint pos() const { return p.toPoint(); }
    inline QPoint oldPos() const { return op.toPoint(); }
#endif

    inline const QPointF &posF() const { return p; }
    inline const QPointF &oldPosF() const { return op; }

protected:
    QPointF p, op;
};

#if QT_CONFIG(wheelevent)
class Q_GUI_EXPORT QWheelEvent : public QInputEvent
{
public:
    enum { DefaultDeltasPerStep = 120 };

#if QT_DEPRECATED_SINCE(5, 15)
    // Actually deprecated since 5.0, in docs
    QT_DEPRECATED_VERSION_X_5_15("Use the last QWheelEvent constructor taking pixelDelta, angleDelta, phase, and inverted")
    QWheelEvent(const QPointF &pos, int delta,
                Qt::MouseButtons buttons, Qt::KeyboardModifiers modifiers,
                Qt::Orientation orient = Qt::Vertical);
    // Actually deprecated since 5.0, in docs
    QT_DEPRECATED_VERSION_X_5_15("Use the last QWheelEvent constructor taking pixelDelta, angleDelta, phase, and inverted")
    QWheelEvent(const QPointF &pos, const QPointF& globalPos, int delta,
                Qt::MouseButtons buttons, Qt::KeyboardModifiers modifiers,
                Qt::Orientation orient = Qt::Vertical);
    QT_DEPRECATED_VERSION_X_5_15("Use the last QWheelEvent constructor taking pixelDelta, angleDelta, phase, and inverted")
    QWheelEvent(const QPointF &pos, const QPointF& globalPos,
                QPoint pixelDelta, QPoint angleDelta, int qt4Delta, Qt::Orientation qt4Orientation,
                Qt::MouseButtons buttons, Qt::KeyboardModifiers modifiers);
    QT_DEPRECATED_VERSION_X_5_15("Use the last QWheelEvent constructor taking pixelDelta, angleDelta, phase, and inverted")
    QWheelEvent(const QPointF &pos, const QPointF& globalPos,
                QPoint pixelDelta, QPoint angleDelta, int qt4Delta, Qt::Orientation qt4Orientation,
                Qt::MouseButtons buttons, Qt::KeyboardModifiers modifiers, Qt::ScrollPhase phase);
    QT_DEPRECATED_VERSION_X_5_15("Use the last QWheelEvent constructor taking pixelDelta, angleDelta, phase, and inverted")
    QWheelEvent(const QPointF &pos, const QPointF &globalPos, QPoint pixelDelta, QPoint angleDelta,
                int qt4Delta, Qt::Orientation qt4Orientation, Qt::MouseButtons buttons,
                Qt::KeyboardModifiers modifiers, Qt::ScrollPhase phase, Qt::MouseEventSource source);
    QT_DEPRECATED_VERSION_X_5_15("Use the last QWheelEvent constructor taking pixelDelta, angleDelta, phase, and inverted")
    QWheelEvent(const QPointF &pos, const QPointF &globalPos, QPoint pixelDelta, QPoint angleDelta,
                int qt4Delta, Qt::Orientation qt4Orientation, Qt::MouseButtons buttons,
                Qt::KeyboardModifiers modifiers, Qt::ScrollPhase phase, Qt::MouseEventSource source, bool inverted);
#endif

    QWheelEvent(QPointF pos, QPointF globalPos, QPoint pixelDelta, QPoint angleDelta,
                Qt::MouseButtons buttons, Qt::KeyboardModifiers modifiers, Qt::ScrollPhase phase,
                bool inverted, Qt::MouseEventSource source = Qt::MouseEventNotSynthesized);
    ~QWheelEvent();


    inline QPoint pixelDelta() const { return pixelD; }
    inline QPoint angleDelta() const { return angleD; }

#if QT_DEPRECATED_SINCE(5, 15)
    // Actually deprecated since 5.0, in docs
    QT_DEPRECATED_VERSION_X_5_15("Use angleDelta()")
    inline int delta() const  { return qt4D; }
    // Actually deprecated since 5.0, in docs
    QT_DEPRECATED_VERSION_X_5_15("Use angleDelta()")
    inline Qt::Orientation orientation() const { return qt4O; }
#ifndef QT_NO_INTEGER_EVENT_COORDINATES
    QT_DEPRECATED_VERSION_X_5_15("Use position()")
    inline QPoint pos() const { return p.toPoint(); }
    QT_DEPRECATED_VERSION_X_5_15("Use globalPosition()")
    inline QPoint globalPos()   const { return g.toPoint(); }
    QT_DEPRECATED_VERSION_X_5_15("Use position()")
    inline int x() const { return int(p.x()); }
    QT_DEPRECATED_VERSION_X_5_15("Use position()")
    inline int y() const { return int(p.y()); }
    QT_DEPRECATED_VERSION_X_5_15("Use globalPosition()")
    inline int globalX() const { return int(g.x()); }
    QT_DEPRECATED_VERSION_X_5_15("Use globalPosition()")
    inline int globalY() const { return int(g.y()); }
#endif
    QT_DEPRECATED_VERSION_X_5_15("Use position()")
    inline const QPointF &posF() const { return p; }
    QT_DEPRECATED_VERSION_X_5_15("Use globalPosition()")
    inline const QPointF &globalPosF()   const { return g; }
#endif // QT_DEPRECATED_SINCE(5, 15)

    inline QPointF position() const { return p; }
    inline QPointF globalPosition() const { return g; }

    inline Qt::MouseButtons buttons() const { return mouseState; }

    inline Qt::ScrollPhase phase() const { return Qt::ScrollPhase(ph); }
    inline bool inverted() const { return invertedScrolling; }

    Qt::MouseEventSource source() const { return Qt::MouseEventSource(src); }

protected:
    QPointF p;
    QPointF g;
    QPoint pixelD;
    QPoint angleD;
    int qt4D = 0;
    Qt::Orientation qt4O = Qt::Vertical;
    Qt::MouseButtons mouseState = Qt::NoButton;
    uint _unused_ : 2; // Kept for binary compatibility
    uint src: 2;
    bool invertedScrolling : 1;
    uint ph : 3;
    int reserved : 24;

    friend class QApplication;
};
#endif

#if QT_CONFIG(tabletevent)
class Q_GUI_EXPORT QTabletEvent : public QInputEvent
{
    Q_GADGET
public:
    enum TabletDevice { NoDevice, Puck, Stylus, Airbrush, FourDMouse,
                        XFreeEraser /*internal*/, RotationStylus };
    Q_ENUM(TabletDevice)
    enum PointerType { UnknownPointer, Pen, Cursor, Eraser };
    Q_ENUM(PointerType)

#if QT_DEPRECATED_SINCE(5, 15)
    // Actually deprecated since 5.4, in docs
    QT_DEPRECATED_VERSION_X_5_15("Use the other QTabletEvent constructor")
    QTabletEvent(Type t, const QPointF &pos, const QPointF &globalPos,
                 int device, int pointerType, qreal pressure, int xTilt, int yTilt,
                 qreal tangentialPressure, qreal rotation, int z,
                 Qt::KeyboardModifiers keyState, qint64 uniqueID); // ### remove in Qt 6
#endif
    QTabletEvent(Type t, const QPointF &pos, const QPointF &globalPos,
                 int device, int pointerType, qreal pressure, int xTilt, int yTilt,
                 qreal tangentialPressure, qreal rotation, int z,
                 Qt::KeyboardModifiers keyState, qint64 uniqueID,
                 Qt::MouseButton button, Qt::MouseButtons buttons);
    ~QTabletEvent();

    inline QPoint pos() const { return mPos.toPoint(); }
    inline QPoint globalPos() const { return mGPos.toPoint(); }
#if QT_DEPRECATED_SINCE(5,0)
    QT_DEPRECATED inline const QPointF &hiResGlobalPos() const { return mPos; }
#endif

    inline const QPointF &posF() const { return mPos; }
    inline const QPointF &globalPosF() const { return mGPos; }

    inline int x() const { return qRound(mPos.x()); }
    inline int y() const { return qRound(mPos.y()); }
    inline int globalX() const { return qRound(mGPos.x()); }
    inline int globalY() const { return qRound(mGPos.y()); }
#if QT_DEPRECATED_SINCE(5, 15)
    QT_DEPRECATED_VERSION_X_5_15("use globalPosF().x()")
    inline qreal hiResGlobalX() const { return mGPos.x(); }
    QT_DEPRECATED_VERSION_X_5_15("use globalPosF().y()")
    inline qreal hiResGlobalY() const { return mGPos.y(); }
    QT_DEPRECATED_VERSION_X_5_15("Use deviceType()")
    inline TabletDevice device() const { return TabletDevice(mDev); }
#endif
    inline TabletDevice deviceType() const { return TabletDevice(mDev); }
    inline PointerType pointerType() const { return PointerType(mPointerType); }
    inline qint64 uniqueId() const { return mUnique; }
    inline qreal pressure() const { return mPress; }
    inline int z() const { return mZ; }
    inline qreal tangentialPressure() const { return mTangential; }
    inline qreal rotation() const { return mRot; }
    inline int xTilt() const { return mXT; }
    inline int yTilt() const { return mYT; }
    Qt::MouseButton button() const;
    Qt::MouseButtons buttons() const;

protected:
    QPointF mPos, mGPos;
    int mDev, mPointerType, mXT, mYT, mZ;
    qreal mPress, mTangential, mRot;
    qint64 mUnique;

    // QTabletEventPrivate for extra storage.
    // ### Qt 6: QPointingEvent will have Buttons, QTabletEvent will inherit
    void *mExtra;
};
#endif // QT_CONFIG(tabletevent)

#ifndef QT_NO_GESTURES
class Q_GUI_EXPORT QNativeGestureEvent : public QInputEvent
{
public:
#if QT_DEPRECATED_SINCE(5, 10)
    QT_DEPRECATED QNativeGestureEvent(Qt::NativeGestureType type, const QPointF &localPos, const QPointF &windowPos,
                        const QPointF &screenPos, qreal value, ulong sequenceId, quint64 intArgument);
#endif
    QNativeGestureEvent(Qt::NativeGestureType type, const QTouchDevice *dev, const QPointF &localPos, const QPointF &windowPos,
                        const QPointF &screenPos, qreal value, ulong sequenceId, quint64 intArgument);
    ~QNativeGestureEvent();
    Qt::NativeGestureType gestureType() const { return mGestureType; }
    qreal value() const { return mRealValue; }

#ifndef QT_NO_INTEGER_EVENT_COORDINATES
    inline const QPoint pos() const { return mLocalPos.toPoint(); }
    inline const QPoint globalPos() const { return mScreenPos.toPoint(); }
#endif
    const QPointF &localPos() const { return mLocalPos; }
    const QPointF &windowPos() const { return mWindowPos; }
    const QPointF &screenPos() const { return mScreenPos; }

    const QTouchDevice *device() const;

protected:
    Qt::NativeGestureType mGestureType;
    QPointF mLocalPos;
    QPointF mWindowPos;
    QPointF mScreenPos;
    qreal mRealValue;
    ulong mSequenceId;
    quint64 mIntValue;
};
#endif // QT_NO_GESTURES

class Q_GUI_EXPORT QKeyEvent : public QInputEvent
{
public:
    QKeyEvent(Type type, int key, Qt::KeyboardModifiers modifiers, const QString& text = QString(),
              bool autorep = false, ushort count = 1);
    QKeyEvent(Type type, int key, Qt::KeyboardModifiers modifiers,
              quint32 nativeScanCode, quint32 nativeVirtualKey, quint32 nativeModifiers,
              const QString &text = QString(), bool autorep = false, ushort count = 1);
    ~QKeyEvent();

    int key() const { return k; }
#ifndef QT_NO_SHORTCUT
    bool matches(QKeySequence::StandardKey key) const;
#endif
    Qt::KeyboardModifiers modifiers() const;
    inline QString text() const { return txt; }
    inline bool isAutoRepeat() const { return autor; }
    inline int count() const { return int(c); }

    inline quint32 nativeScanCode() const { return nScanCode; }
    inline quint32 nativeVirtualKey() const { return nVirtualKey; }
    inline quint32 nativeModifiers() const { return nModifiers; }

    // Functions for the extended key event information
#if QT_DEPRECATED_SINCE(5, 0)
    static inline QKeyEvent *createExtendedKeyEvent(Type type, int key, Qt::KeyboardModifiers modifiers,
                                             quint32 nativeScanCode, quint32 nativeVirtualKey,
                                             quint32 nativeModifiers,
                                             const QString& text = QString(), bool autorep = false,
                                             ushort count = 1)
    {
        return new QKeyEvent(type, key, modifiers,
                             nativeScanCode, nativeVirtualKey, nativeModifiers,
                             text, autorep, count);
    }

    inline bool hasExtendedInfo() const { return true; }
#endif

protected:
    QString txt;
    int k;
    quint32 nScanCode;
    quint32 nVirtualKey;
    quint32 nModifiers;
    ushort c;
    ushort autor:1;
    // ushort reserved:15;
};


class Q_GUI_EXPORT QFocusEvent : public QEvent
{
public:
    explicit QFocusEvent(Type type, Qt::FocusReason reason=Qt::OtherFocusReason);
    ~QFocusEvent();

    inline bool gotFocus() const { return type() == FocusIn; }
    inline bool lostFocus() const { return type() == FocusOut; }

    Qt::FocusReason reason() const;

private:
    Qt::FocusReason m_reason;
};


class Q_GUI_EXPORT QPaintEvent : public QEvent
{
public:
    explicit QPaintEvent(const QRegion& paintRegion);
    explicit QPaintEvent(const QRect &paintRect);
    ~QPaintEvent();

    inline const QRect &rect() const { return m_rect; }
    inline const QRegion &region() const { return m_region; }

protected:
    QRect m_rect;
    QRegion m_region;
    bool m_erased;
};

class Q_GUI_EXPORT QMoveEvent : public QEvent
{
public:
    QMoveEvent(const QPoint &pos, const QPoint &oldPos);
    ~QMoveEvent();

    inline const QPoint &pos() const { return p; }
    inline const QPoint &oldPos() const { return oldp;}
protected:
    QPoint p, oldp;
    friend class QApplication;
};

class Q_GUI_EXPORT QExposeEvent : public QEvent
{
public:
    explicit QExposeEvent(const QRegion &rgn);
    ~QExposeEvent();

    inline const QRegion &region() const { return rgn; }

protected:
    QRegion rgn;
};

class Q_GUI_EXPORT QPlatformSurfaceEvent : public QEvent
{
public:
    enum SurfaceEventType {
        SurfaceCreated,
        SurfaceAboutToBeDestroyed
    };

    explicit QPlatformSurfaceEvent(SurfaceEventType surfaceEventType);
    ~QPlatformSurfaceEvent();

    inline SurfaceEventType surfaceEventType() const { return m_surfaceEventType; }

protected:
    SurfaceEventType m_surfaceEventType;
};

class Q_GUI_EXPORT QResizeEvent : public QEvent
{
public:
    QResizeEvent(const QSize &size, const QSize &oldSize);
    ~QResizeEvent();

    inline const QSize &size() const { return s; }
    inline const QSize &oldSize()const { return olds;}
protected:
    QSize s, olds;
    friend class QApplication;
};


class Q_GUI_EXPORT QCloseEvent : public QEvent
{
public:
    QCloseEvent();
    ~QCloseEvent();
};


class Q_GUI_EXPORT QIconDragEvent : public QEvent
{
public:
    QIconDragEvent();
    ~QIconDragEvent();
};


class Q_GUI_EXPORT QShowEvent : public QEvent
{
public:
    QShowEvent();
    ~QShowEvent();
};


class Q_GUI_EXPORT QHideEvent : public QEvent
{
public:
    QHideEvent();
    ~QHideEvent();
};

#ifndef QT_NO_CONTEXTMENU
class Q_GUI_EXPORT QContextMenuEvent : public QInputEvent
{
public:
    enum Reason { Mouse, Keyboard, Other };

    QContextMenuEvent(Reason reason, const QPoint &pos, const QPoint &globalPos,
                      Qt::KeyboardModifiers modifiers);
    QContextMenuEvent(Reason reason, const QPoint &pos, const QPoint &globalPos);
    QContextMenuEvent(Reason reason, const QPoint &pos);
    ~QContextMenuEvent();

    inline int x() const { return p.x(); }
    inline int y() const { return p.y(); }
    inline int globalX() const { return gp.x(); }
    inline int globalY() const { return gp.y(); }

    inline const QPoint& pos() const { return p; }
    inline const QPoint& globalPos() const { return gp; }

    inline Reason reason() const { return Reason(reas); }

protected:
    QPoint p;
    QPoint gp;
    uint reas : 8;
};
#endif // QT_NO_CONTEXTMENU

#ifndef QT_NO_INPUTMETHOD
class Q_GUI_EXPORT QInputMethodEvent : public QEvent
{
public:
    enum AttributeType {
       TextFormat,
       Cursor,
       Language,
       Ruby,
       Selection
    };
    class Attribute {
    public:
        Attribute(AttributeType typ, int s, int l, QVariant val) : type(typ), start(s), length(l), value(std::move(val)) {}
        Attribute(AttributeType typ, int s, int l) : type(typ), start(s), length(l), value() {}

        AttributeType type;
        int start;
        int length;
        QVariant value;
    };
    QInputMethodEvent();
    QInputMethodEvent(const QString &preeditText, const QList<Attribute> &attributes);
    ~QInputMethodEvent();

    void setCommitString(const QString &commitString, int replaceFrom = 0, int replaceLength = 0);
    inline const QList<Attribute> &attributes() const { return attrs; }
    inline const QString &preeditString() const { return preedit; }

    inline const QString &commitString() const { return commit; }
    inline int replacementStart() const { return replace_from; }
    inline int replacementLength() const { return replace_length; }

    QInputMethodEvent(const QInputMethodEvent &other);

private:
    QString preedit;
    QList<Attribute> attrs;
    QString commit;
    int replace_from;
    int replace_length;
};
Q_DECLARE_TYPEINFO(QInputMethodEvent::Attribute, Q_MOVABLE_TYPE);

class Q_GUI_EXPORT QInputMethodQueryEvent : public QEvent
{
public:
    explicit QInputMethodQueryEvent(Qt::InputMethodQueries queries);
    ~QInputMethodQueryEvent();

    Qt::InputMethodQueries queries() const { return m_queries; }

    void setValue(Qt::InputMethodQuery query, const QVariant &value);
    QVariant value(Qt::InputMethodQuery query) const;
private:
    Qt::InputMethodQueries m_queries;
    struct QueryPair {
        Qt::InputMethodQuery query;
        QVariant value;
    };
    friend QTypeInfo<QueryPair>;
    QVector<QueryPair> m_values;
};
Q_DECLARE_TYPEINFO(QInputMethodQueryEvent::QueryPair, Q_MOVABLE_TYPE);

#endif // QT_NO_INPUTMETHOD

#if QT_CONFIG(draganddrop)

class QMimeData;

class Q_GUI_EXPORT QDropEvent : public QEvent
{
public:
    QDropEvent(const QPointF& pos, Qt::DropActions actions, const QMimeData *data,
               Qt::MouseButtons buttons, Qt::KeyboardModifiers modifiers, Type type = Drop);
    ~QDropEvent();

    inline QPoint pos() const { return p.toPoint(); }
    inline const QPointF &posF() const { return p; }
    inline Qt::MouseButtons mouseButtons() const { return mouseState; }
    inline Qt::KeyboardModifiers keyboardModifiers() const { return modState; }

    inline Qt::DropActions possibleActions() const { return act; }
    inline Qt::DropAction proposedAction() const { return default_action; }
    inline void acceptProposedAction() { drop_action = default_action; accept(); }

    inline Qt::DropAction dropAction() const { return drop_action; }
    void setDropAction(Qt::DropAction action);

    QObject* source() const;
    inline const QMimeData *mimeData() const { return mdata; }

protected:
    friend class QApplication;
    QPointF p;
    Qt::MouseButtons mouseState;
    Qt::KeyboardModifiers modState;
    Qt::DropActions act;
    Qt::DropAction drop_action;
    Qt::DropAction default_action;
    const QMimeData *mdata;
};


class Q_GUI_EXPORT QDragMoveEvent : public QDropEvent
{
public:
    QDragMoveEvent(const QPoint &pos, Qt::DropActions actions, const QMimeData *data,
                   Qt::MouseButtons buttons, Qt::KeyboardModifiers modifiers, Type type = DragMove);
    ~QDragMoveEvent();

    inline QRect answerRect() const { return rect; }

    inline void accept() { QDropEvent::accept(); }
    inline void ignore() { QDropEvent::ignore(); }

    inline void accept(const QRect & r) { accept(); rect = r; }
    inline void ignore(const QRect & r) { ignore(); rect = r; }

protected:
    QRect rect;
};


class Q_GUI_EXPORT QDragEnterEvent : public QDragMoveEvent
{
public:
    QDragEnterEvent(const QPoint &pos, Qt::DropActions actions, const QMimeData *data,
                    Qt::MouseButtons buttons, Qt::KeyboardModifiers modifiers);
    ~QDragEnterEvent();
};


class Q_GUI_EXPORT QDragLeaveEvent : public QEvent
{
public:
    QDragLeaveEvent();
    ~QDragLeaveEvent();
};
#endif // QT_CONFIG(draganddrop)


class Q_GUI_EXPORT QHelpEvent : public QEvent
{
public:
    QHelpEvent(Type type, const QPoint &pos, const QPoint &globalPos);
    ~QHelpEvent();

    inline int x() const { return p.x(); }
    inline int y() const { return p.y(); }
    inline int globalX() const { return gp.x(); }
    inline int globalY() const { return gp.y(); }

    inline const QPoint& pos()  const { return p; }
    inline const QPoint& globalPos() const { return gp; }

private:
    QPoint p;
    QPoint gp;
};

#ifndef QT_NO_STATUSTIP
class Q_GUI_EXPORT QStatusTipEvent : public QEvent
{
public:
    explicit QStatusTipEvent(const QString &tip);
    ~QStatusTipEvent();

    inline QString tip() const { return s; }
private:
    QString s;
};
#endif

#if QT_CONFIG(whatsthis)
class Q_GUI_EXPORT QWhatsThisClickedEvent : public QEvent
{
public:
    explicit QWhatsThisClickedEvent(const QString &href);
    ~QWhatsThisClickedEvent();

    inline QString href() const { return s; }
private:
    QString s;
};
#endif

#ifndef QT_NO_ACTION
class Q_GUI_EXPORT QActionEvent : public QEvent
{
    QAction *act, *bef;
public:
    QActionEvent(int type, QAction *action, QAction *before = nullptr);
    ~QActionEvent();

    inline QAction *action() const { return act; }
    inline QAction *before() const { return bef; }
};
#endif

class Q_GUI_EXPORT QFileOpenEvent : public QEvent
{
public:
    explicit QFileOpenEvent(const QString &file);
    explicit QFileOpenEvent(const QUrl &url);
    ~QFileOpenEvent();

    inline QString file() const { return f; }
    QUrl url() const { return m_url; }
    bool openFile(QFile &file, QIODevice::OpenMode flags) const;
private:
    QString f;
    QUrl m_url;
};

#ifndef QT_NO_TOOLBAR
class Q_GUI_EXPORT QToolBarChangeEvent : public QEvent
{
public:
    explicit QToolBarChangeEvent(bool t);
    ~QToolBarChangeEvent();

    inline bool toggle() const { return tog; }
private:
    uint tog : 1;
};
#endif

#ifndef QT_NO_SHORTCUT
class Q_GUI_EXPORT QShortcutEvent : public QEvent
{
public:
    QShortcutEvent(const QKeySequence &key, int id, bool ambiguous = false);
    ~QShortcutEvent();

    inline const QKeySequence &key() const { return sequence; }
    inline int shortcutId() const { return sid; }
    inline bool isAmbiguous() const { return ambig; }
protected:
    QKeySequence sequence;
    bool ambig;
    int  sid;
};
#endif

class Q_GUI_EXPORT QWindowStateChangeEvent: public QEvent
{
public:
    explicit QWindowStateChangeEvent(Qt::WindowStates aOldState, bool isOverride = false);
    ~QWindowStateChangeEvent();

    inline Qt::WindowStates oldState() const { return ostate; }
    bool isOverride() const;

private:
    Qt::WindowStates ostate;
    bool m_override;
};

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug, const QEvent *);
#endif

#ifndef QT_NO_SHORTCUT
inline bool operator==(QKeyEvent *e, QKeySequence::StandardKey key){return (e ? e->matches(key) : false);}
inline bool operator==(QKeySequence::StandardKey key, QKeyEvent *e){return (e ? e->matches(key) : false);}
#endif // QT_NO_SHORTCUT

class Q_GUI_EXPORT QPointingDeviceUniqueId
{
    Q_GADGET
    Q_PROPERTY(qint64 numericId READ numericId CONSTANT)
public:
    Q_ALWAYS_INLINE
    Q_DECL_CONSTEXPR QPointingDeviceUniqueId() noexcept : m_numericId(-1) {}
    // compiler-generated copy/move ctor/assignment operators are ok!
    // compiler-generated dtor is ok!

    static QPointingDeviceUniqueId fromNumericId(qint64 id);

    Q_ALWAYS_INLINE Q_DECL_CONSTEXPR bool isValid() const noexcept { return m_numericId != -1; }
    qint64 numericId() const noexcept;

private:
    // TODO: for TUIO 2, or any other type of complex token ID, an internal
    // array (or hash) can be added to hold additional properties.
    // In this case, m_numericId will then turn into an index into that array (or hash).
    qint64 m_numericId;
};
Q_DECLARE_TYPEINFO(QPointingDeviceUniqueId, Q_MOVABLE_TYPE);

#if 0
#pragma qt_sync_suspend_processing
#endif
template <> class QList<QPointingDeviceUniqueId> {}; // to prevent instantiation: use QVector instead
#if 0
#pragma qt_sync_resume_processing
#endif

Q_GUI_EXPORT bool operator==(QPointingDeviceUniqueId lhs, QPointingDeviceUniqueId rhs) noexcept;
inline bool operator!=(QPointingDeviceUniqueId lhs, QPointingDeviceUniqueId rhs) noexcept
{ return !operator==(lhs, rhs); }
Q_GUI_EXPORT uint qHash(QPointingDeviceUniqueId key, uint seed = 0) noexcept;



class QTouchEventTouchPointPrivate;
class Q_GUI_EXPORT QTouchEvent : public QInputEvent
{
public:
    class Q_GUI_EXPORT TouchPoint
    {
    public:
        enum InfoFlag {
            Pen  = 0x0001,
            Token = 0x0002
        };
#ifndef Q_MOC_RUN
        // otherwise moc gives
        // Error: Meta object features not supported for nested classes
        Q_DECLARE_FLAGS(InfoFlags, InfoFlag)
#endif

        explicit TouchPoint(int id = -1);
        TouchPoint(const TouchPoint &other);
        TouchPoint(TouchPoint &&other) noexcept
            : d(nullptr)
        { qSwap(d, other.d); }
        TouchPoint &operator=(TouchPoint &&other) noexcept
        { qSwap(d, other.d); return *this; }
        ~TouchPoint();

        TouchPoint &operator=(const TouchPoint &other)
        { if ( d != other.d ) { TouchPoint copy(other); swap(copy); } return *this; }

        void swap(TouchPoint &other) noexcept
        { qSwap(d, other.d); }

        int id() const;
        QPointingDeviceUniqueId uniqueId() const;

        Qt::TouchPointState state() const;

        QPointF pos() const;
        QPointF startPos() const;
        QPointF lastPos() const;

        QPointF scenePos() const;
        QPointF startScenePos() const;
        QPointF lastScenePos() const;

        QPointF screenPos() const;
        QPointF startScreenPos() const;
        QPointF lastScreenPos() const;

        QPointF normalizedPos() const;
        QPointF startNormalizedPos() const;
        QPointF lastNormalizedPos() const;

#if QT_DEPRECATED_SINCE(5, 15)
        // All these are actually deprecated since 5.9, in docs
        QT_DEPRECATED_VERSION_X_5_15("Use pos() and ellipseDiameters()")
        QRectF rect() const;
        QT_DEPRECATED_VERSION_X_5_15("Use scenePos() and ellipseDiameters()")
        QRectF sceneRect() const;
        QT_DEPRECATED_VERSION_X_5_15("Use screenPos() and ellipseDiameters()")
        QRectF screenRect() const;

        // internal
        QT_DEPRECATED_VERSION_X_5_15("Use setPos() and setEllipseDiameters()")
        void setRect(const QRectF &rect); // deprecated
        QT_DEPRECATED_VERSION_X_5_15("Use setScenePos() and setEllipseDiameters()")
        void setSceneRect(const QRectF &sceneRect); // deprecated
        QT_DEPRECATED_VERSION_X_5_15("Use setScreenPos() and setEllipseDiameters()")
        void setScreenRect(const QRectF &screenRect); // deprecated
#endif
        qreal pressure() const;
        qreal rotation() const;
        QSizeF ellipseDiameters() const;

        QVector2D velocity() const;
        InfoFlags flags() const;
        QVector<QPointF> rawScreenPositions() const;

        // internal
        void setId(int id);
        void setUniqueId(qint64 uid);
        void setState(Qt::TouchPointStates state);
        void setPos(const QPointF &pos);
        void setScenePos(const QPointF &scenePos);
        void setScreenPos(const QPointF &screenPos);
        void setNormalizedPos(const QPointF &normalizedPos);
        void setStartPos(const QPointF &startPos);
        void setStartScenePos(const QPointF &startScenePos);
        void setStartScreenPos(const QPointF &startScreenPos);
        void setStartNormalizedPos(const QPointF &startNormalizedPos);
        void setLastPos(const QPointF &lastPos);
        void setLastScenePos(const QPointF &lastScenePos);
        void setLastScreenPos(const QPointF &lastScreenPos);
        void setLastNormalizedPos(const QPointF &lastNormalizedPos);
        void setPressure(qreal pressure);
        void setRotation(qreal angle);
        void setEllipseDiameters(const QSizeF &dia);
        void setVelocity(const QVector2D &v);
        void setFlags(InfoFlags flags);
        void setRawScreenPositions(const QVector<QPointF> &positions);

    private:
        QTouchEventTouchPointPrivate *d;
        friend class QGuiApplication;
        friend class QGuiApplicationPrivate;
        friend class QApplication;
        friend class QApplicationPrivate;
        friend class QQuickPointerTouchEvent;
        friend class QQuickMultiPointTouchArea;
    };

#if QT_DEPRECATED_SINCE(5, 0)
    enum DeviceType {
        TouchScreen,
        TouchPad
    };
#endif

    explicit QTouchEvent(QEvent::Type eventType,
                         QTouchDevice *device = nullptr,
                         Qt::KeyboardModifiers modifiers = Qt::NoModifier,
                         Qt::TouchPointStates touchPointStates = Qt::TouchPointStates(),
                         const QList<QTouchEvent::TouchPoint> &touchPoints = QList<QTouchEvent::TouchPoint>());
    ~QTouchEvent();

    inline QWindow *window() const { return _window; }
    inline QObject *target() const { return _target; }
#if QT_DEPRECATED_SINCE(5, 0)
    QT_DEPRECATED inline QTouchEvent::DeviceType deviceType() const { return static_cast<DeviceType>(int(_device->type())); }
#endif
    inline Qt::TouchPointStates touchPointStates() const { return _touchPointStates; }
    inline const QList<QTouchEvent::TouchPoint> &touchPoints() const { return _touchPoints; }
    inline QTouchDevice *device() const { return _device; }

    // internal
    inline void setWindow(QWindow *awindow) { _window = awindow; }
    inline void setTarget(QObject *atarget) { _target = atarget; }
    inline void setTouchPointStates(Qt::TouchPointStates aTouchPointStates) { _touchPointStates = aTouchPointStates; }
    inline void setTouchPoints(const QList<QTouchEvent::TouchPoint> &atouchPoints) { _touchPoints = atouchPoints; }
    inline void setDevice(QTouchDevice *adevice) { _device = adevice; }

protected:
    QWindow *_window;
    QObject *_target;
    QTouchDevice *_device;
    Qt::TouchPointStates _touchPointStates;
    QList<QTouchEvent::TouchPoint> _touchPoints;

    friend class QGuiApplication;
    friend class QGuiApplicationPrivate;
    friend class QApplication;
    friend class QApplicationPrivate;
#ifndef QT_NO_GRAPHICSVIEW
    friend class QGraphicsScenePrivate; // direct access to _touchPoints
#endif
};
Q_DECLARE_TYPEINFO(QTouchEvent::TouchPoint, Q_MOVABLE_TYPE);
Q_DECLARE_OPERATORS_FOR_FLAGS(QTouchEvent::TouchPoint::InfoFlags)

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug, const QTouchEvent::TouchPoint &);
#endif

class Q_GUI_EXPORT QScrollPrepareEvent : public QEvent
{
public:
    explicit QScrollPrepareEvent(const QPointF &startPos);
    ~QScrollPrepareEvent();

    QPointF startPos() const;

    QSizeF viewportSize() const;
    QRectF contentPosRange() const;
    QPointF contentPos() const;

    void setViewportSize(const QSizeF &size);
    void setContentPosRange(const QRectF &rect);
    void setContentPos(const QPointF &pos);

private:
    QObject* m_target; // Qt 6 remove.
    QPointF m_startPos;
    QSizeF m_viewportSize;
    QRectF m_contentPosRange;
    QPointF m_contentPos;
};


class Q_GUI_EXPORT QScrollEvent : public QEvent
{
public:
    enum ScrollState
    {
        ScrollStarted,
        ScrollUpdated,
        ScrollFinished
    };

    QScrollEvent(const QPointF &contentPos, const QPointF &overshoot, ScrollState scrollState);
    ~QScrollEvent();

    QPointF contentPos() const;
    QPointF overshootDistance() const;
    ScrollState scrollState() const;

private:
    QPointF m_contentPos;
    QPointF m_overshoot;
    QScrollEvent::ScrollState m_state;
};

class Q_GUI_EXPORT QScreenOrientationChangeEvent : public QEvent
{
public:
    QScreenOrientationChangeEvent(QScreen *screen, Qt::ScreenOrientation orientation);
    ~QScreenOrientationChangeEvent();

    QScreen *screen() const;
    Qt::ScreenOrientation orientation() const;

private:
    QScreen *m_screen;
    Qt::ScreenOrientation m_orientation;
};

class Q_GUI_EXPORT QApplicationStateChangeEvent : public QEvent
{
public:
    explicit QApplicationStateChangeEvent(Qt::ApplicationState state);
    Qt::ApplicationState applicationState() const;

private:
    Qt::ApplicationState m_applicationState;
};

QT_END_NAMESPACE

#endif // QEVENT_H
