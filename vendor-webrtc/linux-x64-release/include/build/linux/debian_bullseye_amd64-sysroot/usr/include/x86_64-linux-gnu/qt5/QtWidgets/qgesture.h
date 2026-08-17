/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtWidgets module of the Qt Toolkit.
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

#ifndef QGESTURE_H
#define QGESTURE_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtCore/qobject.h>
#include <QtCore/qlist.h>
#include <QtCore/qdatetime.h>
#include <QtCore/qpoint.h>
#include <QtCore/qrect.h>
#include <QtCore/qmetatype.h>
#include <QtGui/qevent.h>

#ifndef QT_NO_GESTURES

Q_DECLARE_METATYPE(Qt::GestureState)
Q_DECLARE_METATYPE(Qt::GestureType)

QT_BEGIN_NAMESPACE


class QGesturePrivate;
class Q_WIDGETS_EXPORT QGesture : public QObject
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QGesture)

    Q_PROPERTY(Qt::GestureState state READ state)
    Q_PROPERTY(Qt::GestureType gestureType READ gestureType)
    Q_PROPERTY(QGesture::GestureCancelPolicy gestureCancelPolicy READ gestureCancelPolicy WRITE setGestureCancelPolicy)
    Q_PROPERTY(QPointF hotSpot READ hotSpot WRITE setHotSpot RESET unsetHotSpot)
    Q_PROPERTY(bool hasHotSpot READ hasHotSpot)

public:
    explicit QGesture(QObject *parent = nullptr);
    ~QGesture();

    Qt::GestureType gestureType() const;

    Qt::GestureState state() const;

    QPointF hotSpot() const;
    void setHotSpot(const QPointF &value);
    bool hasHotSpot() const;
    void unsetHotSpot();

    enum GestureCancelPolicy {
        CancelNone = 0,
        CancelAllInContext
    };

    void setGestureCancelPolicy(GestureCancelPolicy policy);
    GestureCancelPolicy gestureCancelPolicy() const;

protected:
    QGesture(QGesturePrivate &dd, QObject *parent);

private:
    friend class QGestureEvent;
    friend class QGestureRecognizer;
    friend class QGestureManager;
    friend class QGraphicsScenePrivate;
};

class QPanGesturePrivate;
class Q_WIDGETS_EXPORT QPanGesture : public QGesture
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QPanGesture)

    Q_PROPERTY(QPointF lastOffset READ lastOffset WRITE setLastOffset)
    Q_PROPERTY(QPointF offset READ offset WRITE setOffset)
    Q_PROPERTY(QPointF delta READ delta STORED false)
    Q_PROPERTY(qreal acceleration READ acceleration WRITE setAcceleration)
    Q_PRIVATE_PROPERTY(QPanGesture::d_func(), qreal horizontalVelocity READ horizontalVelocity WRITE setHorizontalVelocity)
    Q_PRIVATE_PROPERTY(QPanGesture::d_func(), qreal verticalVelocity READ verticalVelocity WRITE setVerticalVelocity)

public:
    explicit QPanGesture(QObject *parent = nullptr);
    ~QPanGesture();

    QPointF lastOffset() const;
    QPointF offset() const;
    QPointF delta() const;
    qreal acceleration() const;

    void setLastOffset(const QPointF &value);
    void setOffset(const QPointF &value);
    void setAcceleration(qreal value);

    friend class QPanGestureRecognizer;
    friend class QWinNativePanGestureRecognizer;
};

class QPinchGesturePrivate;
class Q_WIDGETS_EXPORT QPinchGesture : public QGesture
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QPinchGesture)

public:
    enum ChangeFlag {
        ScaleFactorChanged = 0x1,
        RotationAngleChanged = 0x2,
        CenterPointChanged = 0x4
    };
    Q_FLAG(ChangeFlag)
    Q_DECLARE_FLAGS(ChangeFlags, ChangeFlag)
    Q_FLAG(ChangeFlags)

    Q_PROPERTY(ChangeFlags totalChangeFlags READ totalChangeFlags WRITE setTotalChangeFlags)
    Q_PROPERTY(ChangeFlags changeFlags READ changeFlags WRITE setChangeFlags)

    Q_PROPERTY(qreal totalScaleFactor READ totalScaleFactor WRITE setTotalScaleFactor)
    Q_PROPERTY(qreal lastScaleFactor READ lastScaleFactor WRITE setLastScaleFactor)
    Q_PROPERTY(qreal scaleFactor READ scaleFactor WRITE setScaleFactor)

    Q_PROPERTY(qreal totalRotationAngle READ totalRotationAngle WRITE setTotalRotationAngle)
    Q_PROPERTY(qreal lastRotationAngle READ lastRotationAngle WRITE setLastRotationAngle)
    Q_PROPERTY(qreal rotationAngle READ rotationAngle WRITE setRotationAngle)

    Q_PROPERTY(QPointF startCenterPoint READ startCenterPoint WRITE setStartCenterPoint)
    Q_PROPERTY(QPointF lastCenterPoint READ lastCenterPoint WRITE setLastCenterPoint)
    Q_PROPERTY(QPointF centerPoint READ centerPoint WRITE setCenterPoint)

public:
    explicit QPinchGesture(QObject *parent = nullptr);
    ~QPinchGesture();

    ChangeFlags totalChangeFlags() const;
    void setTotalChangeFlags(ChangeFlags value);

    ChangeFlags changeFlags() const;
    void setChangeFlags(ChangeFlags value);

    QPointF startCenterPoint() const;
    QPointF lastCenterPoint() const;
    QPointF centerPoint() const;
    void setStartCenterPoint(const QPointF &value);
    void setLastCenterPoint(const QPointF &value);
    void setCenterPoint(const QPointF &value);

    qreal totalScaleFactor() const;
    qreal lastScaleFactor() const;
    qreal scaleFactor() const;
    void setTotalScaleFactor(qreal value);
    void setLastScaleFactor(qreal value);
    void setScaleFactor(qreal value);

    qreal totalRotationAngle() const;
    qreal lastRotationAngle() const;
    qreal rotationAngle() const;
    void setTotalRotationAngle(qreal value);
    void setLastRotationAngle(qreal value);
    void setRotationAngle(qreal value);

    friend class QPinchGestureRecognizer;
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QPinchGesture::ChangeFlags)

QT_END_NAMESPACE

Q_DECLARE_METATYPE(QPinchGesture::ChangeFlags)

QT_BEGIN_NAMESPACE

class QSwipeGesturePrivate;
class Q_WIDGETS_EXPORT QSwipeGesture : public QGesture
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QSwipeGesture)

    Q_PROPERTY(SwipeDirection horizontalDirection READ horizontalDirection STORED false)
    Q_PROPERTY(SwipeDirection verticalDirection READ verticalDirection STORED false)
    Q_PROPERTY(qreal swipeAngle READ swipeAngle WRITE setSwipeAngle)
    Q_PRIVATE_PROPERTY(QSwipeGesture::d_func(), qreal velocity READ velocity WRITE setVelocity)

public:
    enum SwipeDirection { NoDirection, Left, Right, Up, Down };
    Q_ENUM(SwipeDirection)

    explicit QSwipeGesture(QObject *parent = nullptr);
    ~QSwipeGesture();

    SwipeDirection horizontalDirection() const;
    SwipeDirection verticalDirection() const;

    qreal swipeAngle() const;
    void setSwipeAngle(qreal value);

    friend class QSwipeGestureRecognizer;
};

class QTapGesturePrivate;
class Q_WIDGETS_EXPORT QTapGesture : public QGesture
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QTapGesture)

    Q_PROPERTY(QPointF position READ position WRITE setPosition)

public:
    explicit QTapGesture(QObject *parent = nullptr);
    ~QTapGesture();

    QPointF position() const;
    void setPosition(const QPointF &pos);

    friend class QTapGestureRecognizer;
};

class QTapAndHoldGesturePrivate;
class Q_WIDGETS_EXPORT QTapAndHoldGesture : public QGesture
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QTapAndHoldGesture)

    Q_PROPERTY(QPointF position READ position WRITE setPosition)

public:
    explicit QTapAndHoldGesture(QObject *parent = nullptr);
    ~QTapAndHoldGesture();

    QPointF position() const;
    void setPosition(const QPointF &pos);

    static void setTimeout(int msecs);
    static int timeout();

    friend class QTapAndHoldGestureRecognizer;
};

class QGesture;
class QGestureEventPrivate;
class Q_WIDGETS_EXPORT QGestureEvent : public QEvent
{
public:
    explicit QGestureEvent(const QList<QGesture *> &gestures);
    ~QGestureEvent();

    QList<QGesture *> gestures() const;
    QGesture *gesture(Qt::GestureType type) const;

    QList<QGesture *> activeGestures() const;
    QList<QGesture *> canceledGestures() const;

    using QEvent::setAccepted;
    using QEvent::isAccepted;
    using QEvent::accept;
    using QEvent::ignore;

    void setAccepted(QGesture *, bool);
    void accept(QGesture *);
    void ignore(QGesture *);
    bool isAccepted(QGesture *) const;

    void setAccepted(Qt::GestureType, bool);
    void accept(Qt::GestureType);
    void ignore(Qt::GestureType);
    bool isAccepted(Qt::GestureType) const;

    void setWidget(QWidget *widget);
    QWidget *widget() const;

#if QT_CONFIG(graphicsview)
    QPointF mapToGraphicsScene(const QPointF &gesturePoint) const;
#endif

private:
    QList<QGesture *> m_gestures;
    QWidget *m_widget;
    QMap<Qt::GestureType, bool> m_accepted;
    QMap<Qt::GestureType, QWidget *> m_targetWidgets;

    friend class QApplication;
    friend class QGestureManager;
};

#  ifndef QT_NO_DEBUG_STREAM
Q_WIDGETS_EXPORT QDebug operator<<(QDebug, const QGesture *);
Q_WIDGETS_EXPORT QDebug operator<<(QDebug, const QGestureEvent *);
#  endif

QT_END_NAMESPACE

Q_DECLARE_METATYPE(QGesture::GestureCancelPolicy)
#endif // QT_NO_GESTURES

#endif // QGESTURE_H
