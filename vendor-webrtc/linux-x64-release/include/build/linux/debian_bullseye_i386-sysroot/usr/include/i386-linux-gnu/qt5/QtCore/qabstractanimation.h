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

#ifndef QABSTRACTANIMATION_H
#define QABSTRACTANIMATION_H

#include <QtCore/qobject.h>

QT_REQUIRE_CONFIG(animation);

QT_BEGIN_NAMESPACE

class QAnimationGroup;
class QSequentialAnimationGroup;
class QAnimationDriver;

class QAbstractAnimationPrivate;
class Q_CORE_EXPORT QAbstractAnimation : public QObject
{
    Q_OBJECT

    Q_PROPERTY(State state READ state NOTIFY stateChanged)
    Q_PROPERTY(int loopCount READ loopCount WRITE setLoopCount)
    Q_PROPERTY(int currentTime READ currentTime WRITE setCurrentTime)
    Q_PROPERTY(int currentLoop READ currentLoop NOTIFY currentLoopChanged)
    Q_PROPERTY(Direction direction READ direction WRITE setDirection NOTIFY directionChanged)
    Q_PROPERTY(int duration READ duration)

public:
    enum Direction {
        Forward,
        Backward
    };
    Q_ENUM(Direction)

    enum State {
        Stopped,
        Paused,
        Running
    };
    Q_ENUM(State)

    enum DeletionPolicy {
        KeepWhenStopped = 0,
        DeleteWhenStopped
    };

    QAbstractAnimation(QObject *parent = nullptr);
    virtual ~QAbstractAnimation();

    State state() const;

    QAnimationGroup *group() const;

    Direction direction() const;
    void setDirection(Direction direction);

    int currentTime() const;
    int currentLoopTime() const;

    int loopCount() const;
    void setLoopCount(int loopCount);
    int currentLoop() const;

    virtual int duration() const = 0;
    int totalDuration() const;

Q_SIGNALS:
    void finished();
    void stateChanged(QAbstractAnimation::State newState, QAbstractAnimation::State oldState);
    void currentLoopChanged(int currentLoop);
    void directionChanged(QAbstractAnimation::Direction);

public Q_SLOTS:
    void start(QAbstractAnimation::DeletionPolicy policy = KeepWhenStopped);
    void pause();
    void resume();
    void setPaused(bool);
    void stop();
    void setCurrentTime(int msecs);

protected:
    QAbstractAnimation(QAbstractAnimationPrivate &dd, QObject *parent = nullptr);
    bool event(QEvent *event) override;

    virtual void updateCurrentTime(int currentTime) = 0;
    virtual void updateState(QAbstractAnimation::State newState, QAbstractAnimation::State oldState);
    virtual void updateDirection(QAbstractAnimation::Direction direction);

private:
    Q_DISABLE_COPY(QAbstractAnimation)
    Q_DECLARE_PRIVATE(QAbstractAnimation)
};

class QAnimationDriverPrivate;
class Q_CORE_EXPORT QAnimationDriver : public QObject
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QAnimationDriver)

public:
    QAnimationDriver(QObject *parent = nullptr);
    ~QAnimationDriver();

    virtual void advance();

    void install();
    void uninstall();

    bool isRunning() const;

    virtual qint64 elapsed() const;

#if QT_DEPRECATED_SINCE(5, 13)
    QT_DEPRECATED void setStartTime(qint64 startTime);
    QT_DEPRECATED qint64 startTime() const;
#endif

Q_SIGNALS:
    void started();
    void stopped();

protected:
    // ### Qt6: Remove timestep argument
    void advanceAnimation(qint64 timeStep = -1);
    virtual void start();
    virtual void stop();

    QAnimationDriver(QAnimationDriverPrivate &dd, QObject *parent = nullptr);

private:
    friend class QUnifiedTimer;

};

QT_END_NAMESPACE

#endif // QABSTRACTANIMATION_H
