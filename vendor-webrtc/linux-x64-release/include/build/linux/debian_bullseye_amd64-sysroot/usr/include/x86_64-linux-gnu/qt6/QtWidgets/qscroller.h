// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSCROLLER_H
#define QSCROLLER_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtCore/QObject>
#include <QtCore/QPointF>
#include <QtWidgets/QScrollerProperties>

QT_REQUIRE_CONFIG(scroller);

QT_BEGIN_NAMESPACE


class QWidget;
class QScrollerPrivate;
class QScrollerProperties;
#ifndef QT_NO_GESTURES
class QFlickGestureRecognizer;
class QMouseFlickGestureRecognizer;
#endif

class Q_WIDGETS_EXPORT QScroller : public QObject
{
    Q_OBJECT
    Q_PROPERTY(State state READ state NOTIFY stateChanged)
    Q_PROPERTY(QScrollerProperties scrollerProperties READ scrollerProperties
               WRITE setScrollerProperties NOTIFY scrollerPropertiesChanged)

public:
    enum State
    {
        Inactive,
        Pressed,
        Dragging,
        Scrolling
    };
    Q_ENUM(State)

    enum ScrollerGestureType
    {
        TouchGesture,
        LeftMouseButtonGesture,
        RightMouseButtonGesture,
        MiddleMouseButtonGesture
    };

    enum Input
    {
        InputPress = 1,
        InputMove,
        InputRelease
    };

    static bool hasScroller(QObject *target);

    static QScroller *scroller(QObject *target);
    static const QScroller *scroller(const QObject *target);

#ifndef QT_NO_GESTURES
    static Qt::GestureType grabGesture(QObject *target, ScrollerGestureType gestureType = TouchGesture);
    static Qt::GestureType grabbedGesture(QObject *target);
    static void ungrabGesture(QObject *target);
#endif

    static QList<QScroller *> activeScrollers();

    QObject *target() const;

    State state() const;

    bool handleInput(Input input, const QPointF &position, qint64 timestamp = 0);

    void stop();
    QPointF velocity() const;
    QPointF finalPosition() const;
    QPointF pixelPerMeter() const;

    QScrollerProperties scrollerProperties() const;

    void setSnapPositionsX( const QList<qreal> &positions );
    void setSnapPositionsX( qreal first, qreal interval );
    void setSnapPositionsY( const QList<qreal> &positions );
    void setSnapPositionsY( qreal first, qreal interval );

public Q_SLOTS:
    void setScrollerProperties(const QScrollerProperties &prop);
    void scrollTo(const QPointF &pos);
    void scrollTo(const QPointF &pos, int scrollTime);
    void ensureVisible(const QRectF &rect, qreal xmargin, qreal ymargin);
    void ensureVisible(const QRectF &rect, qreal xmargin, qreal ymargin, int scrollTime);
    void resendPrepareEvent();

Q_SIGNALS:
    void stateChanged(QScroller::State newstate);
    void scrollerPropertiesChanged(const QScrollerProperties &);

private:
    QScrollerPrivate *d_ptr;

    QScroller(QObject *target);
    virtual ~QScroller();

    Q_DISABLE_COPY(QScroller)
    Q_DECLARE_PRIVATE(QScroller)

#ifndef QT_NO_GESTURES
    friend class QFlickGestureRecognizer;
#endif
};

QT_END_NAMESPACE

#endif // QSCROLLER_H
