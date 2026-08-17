/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtCore module of the Qt Toolkit.
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

#ifndef QABSTRACTTRANSITION_H
#define QABSTRACTTRANSITION_H

#include <QtCore/qobject.h>

#include <QtCore/qlist.h>

QT_REQUIRE_CONFIG(statemachine);

QT_BEGIN_NAMESPACE

class QEvent;
class QAbstractState;
class QState;
class QStateMachine;

#if QT_CONFIG(animation)
class QAbstractAnimation;
#endif

class QAbstractTransitionPrivate;
class Q_CORE_EXPORT QAbstractTransition : public QObject
{
    Q_OBJECT
    Q_PROPERTY(QState* sourceState READ sourceState)
    Q_PROPERTY(QAbstractState* targetState READ targetState WRITE setTargetState NOTIFY targetStateChanged)
    Q_PROPERTY(QList<QAbstractState*> targetStates READ targetStates WRITE setTargetStates NOTIFY targetStatesChanged)
    Q_PROPERTY(TransitionType transitionType READ transitionType WRITE setTransitionType REVISION 1)
public:
    enum TransitionType {
        ExternalTransition,
        InternalTransition
    };
    Q_ENUM(TransitionType)

    QAbstractTransition(QState *sourceState = nullptr);
    virtual ~QAbstractTransition();

    QState *sourceState() const;
    QAbstractState *targetState() const;
    void setTargetState(QAbstractState* target);
    QList<QAbstractState*> targetStates() const;
    void setTargetStates(const QList<QAbstractState*> &targets);

    TransitionType transitionType() const;
    void setTransitionType(TransitionType type);

    QStateMachine *machine() const;

#if QT_CONFIG(animation)
    void addAnimation(QAbstractAnimation *animation);
    void removeAnimation(QAbstractAnimation *animation);
    QList<QAbstractAnimation*> animations() const;
#endif

Q_SIGNALS:
    void triggered(QPrivateSignal);
    void targetStateChanged(QPrivateSignal);
    void targetStatesChanged(QPrivateSignal);

protected:
    virtual bool eventTest(QEvent *event) = 0;

    virtual void onTransition(QEvent *event) = 0;

    bool event(QEvent *e) override;

protected:
    QAbstractTransition(QAbstractTransitionPrivate &dd, QState *parent);

private:
    Q_DISABLE_COPY(QAbstractTransition)
    Q_DECLARE_PRIVATE(QAbstractTransition)
};

QT_END_NAMESPACE

#endif
