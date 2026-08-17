// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QGESTURERECOGNIZER_H
#define QGESTURERECOGNIZER_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtCore/qnamespace.h>

#ifndef QT_NO_GESTURES

QT_BEGIN_NAMESPACE


class QObject;
class QEvent;
class QGesture;
class Q_WIDGETS_EXPORT QGestureRecognizer
{
public:
    enum ResultFlag
    {
        Ignore           = 0x0001,

        MayBeGesture     = 0x0002,
        TriggerGesture   = 0x0004,
        FinishGesture    = 0x0008,
        CancelGesture    = 0x0010,

        ResultState_Mask = 0x00ff,

        ConsumeEventHint        = 0x0100,
        // StoreEventHint          = 0x0200,
        // ReplayStoredEventsHint  = 0x0400,
        // DiscardStoredEventsHint = 0x0800,

        ResultHint_Mask = 0xff00
    };
    Q_DECLARE_FLAGS(Result, ResultFlag)

    QGestureRecognizer();
    virtual ~QGestureRecognizer();

    virtual QGesture *create(QObject *target);
    virtual Result recognize(QGesture *state, QObject *watched,
                             QEvent *event) = 0;
    virtual void reset(QGesture *state);

    static Qt::GestureType registerRecognizer(QGestureRecognizer *recognizer);
    static void unregisterRecognizer(Qt::GestureType type);
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QGestureRecognizer::Result)

QT_END_NAMESPACE

#endif // QT_NO_GESTURES

#endif // QGESTURERECOGNIZER_H
