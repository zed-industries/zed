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

#ifndef QSTATEMACHINE_H
#define QSTATEMACHINE_H

#include <QtCore/qstate.h>

#include <QtCore/qcoreevent.h>
#include <QtCore/qlist.h>
#include <QtCore/qobject.h>
#include <QtCore/qset.h>
#include <QtCore/qvariant.h>

#if __has_include(<chrono>)
#    include <chrono>
#endif

QT_REQUIRE_CONFIG(statemachine);

QT_BEGIN_NAMESPACE

class QStateMachinePrivate;
class QAbstractAnimation;
class Q_CORE_EXPORT QStateMachine : public QState
{
    Q_OBJECT
    Q_PROPERTY(QString errorString READ errorString)
    Q_PROPERTY(QState::RestorePolicy globalRestorePolicy READ globalRestorePolicy WRITE setGlobalRestorePolicy)
    Q_PROPERTY(bool running READ isRunning WRITE setRunning NOTIFY runningChanged)
#if QT_CONFIG(animation)
    Q_PROPERTY(bool animated READ isAnimated WRITE setAnimated)
#endif
public:
    class Q_CORE_EXPORT SignalEvent : public QEvent
    {
    public:
        SignalEvent(QObject *sender, int signalIndex,
                     const QList<QVariant> &arguments);
        ~SignalEvent();

        inline QObject *sender() const { return m_sender; }
        inline int signalIndex() const { return m_signalIndex; }
        inline QList<QVariant> arguments() const { return m_arguments; }

    private:
        QObject *m_sender;
        int m_signalIndex;
        QList<QVariant> m_arguments;

        friend class QSignalTransitionPrivate;
    };

    class Q_CORE_EXPORT WrappedEvent : public QEvent
    {
    public:
        WrappedEvent(QObject *object, QEvent *event);
        ~WrappedEvent();

        inline QObject *object() const { return m_object; }
        inline QEvent *event() const { return m_event; }

    private:
        QObject *m_object;
        QEvent *m_event;
    };

    enum EventPriority {
        NormalPriority,
        HighPriority
    };

    enum Error {
        NoError,
        NoInitialStateError,
        NoDefaultStateInHistoryStateError,
        NoCommonAncestorForTransitionError,
        StateMachineChildModeSetToParallelError
    };

    explicit QStateMachine(QObject *parent = nullptr);
    explicit QStateMachine(QState::ChildMode childMode, QObject *parent = nullptr);
    ~QStateMachine();

    void addState(QAbstractState *state);
    void removeState(QAbstractState *state);

    Error error() const;
    QString errorString() const;
    void clearError();

    bool isRunning() const;

#if QT_CONFIG(animation)
    bool isAnimated() const;
    void setAnimated(bool enabled);

    void addDefaultAnimation(QAbstractAnimation *animation);
    QList<QAbstractAnimation *> defaultAnimations() const;
    void removeDefaultAnimation(QAbstractAnimation *animation);
#endif // animation

    QState::RestorePolicy globalRestorePolicy() const;
    void setGlobalRestorePolicy(QState::RestorePolicy restorePolicy);

    void postEvent(QEvent *event, EventPriority priority = NormalPriority);
    int postDelayedEvent(QEvent *event, int delay);
    bool cancelDelayedEvent(int id);

    QSet<QAbstractState*> configuration() const;

#if QT_CONFIG(qeventtransition)
    bool eventFilter(QObject *watched, QEvent *event) override;
#endif

#if __has_include(<chrono>) || defined(Q_QDOC)
    int postDelayedEvent(QEvent *event, std::chrono::milliseconds delay)
    {
        return postDelayedEvent(event, int(delay.count()));
    }
#endif

public Q_SLOTS:
    void start();
    void stop();
    void setRunning(bool running);

Q_SIGNALS:
    void started(QPrivateSignal);
    void stopped(QPrivateSignal);
    void runningChanged(bool running);


protected:
    void onEntry(QEvent *event) override;
    void onExit(QEvent *event) override;

    virtual void beginSelectTransitions(QEvent *event);
    virtual void endSelectTransitions(QEvent *event);

    virtual void beginMicrostep(QEvent *event);
    virtual void endMicrostep(QEvent *event);

    bool event(QEvent *e) override;

protected:
    QStateMachine(QStateMachinePrivate &dd, QObject *parent);

private:
    Q_DISABLE_COPY(QStateMachine)
    Q_DECLARE_PRIVATE(QStateMachine)
    Q_PRIVATE_SLOT(d_func(), void _q_start())
    Q_PRIVATE_SLOT(d_func(), void _q_process())
#if QT_CONFIG(animation)
    Q_PRIVATE_SLOT(d_func(), void _q_animationFinished())
#endif
    Q_PRIVATE_SLOT(d_func(), void _q_startDelayedEventTimer(int, int))
    Q_PRIVATE_SLOT(d_func(), void _q_killDelayedEventTimer(int, int))
};

QT_END_NAMESPACE

#endif
