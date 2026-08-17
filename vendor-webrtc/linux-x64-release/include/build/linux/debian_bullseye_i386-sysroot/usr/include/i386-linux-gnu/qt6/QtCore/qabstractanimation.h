// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QABSTRACTANIMATION_H
#define QABSTRACTANIMATION_H

#include <QtCore/qobject.h>

QT_REQUIRE_CONFIG(animation);

QT_BEGIN_NAMESPACE

class QAnimationGroup;
class QSequentialAnimationGroup;
class QAnimationDriver;
class QUnifiedTimer;

class QAbstractAnimationPrivate;
class Q_CORE_EXPORT QAbstractAnimation : public QObject
{
    Q_OBJECT

    Q_PROPERTY(State state READ state NOTIFY stateChanged BINDABLE bindableState)
    Q_PROPERTY(int loopCount READ loopCount WRITE setLoopCount BINDABLE bindableLoopCount)
    Q_PROPERTY(int currentTime READ currentTime WRITE setCurrentTime BINDABLE bindableCurrentTime)
    Q_PROPERTY(int currentLoop READ currentLoop NOTIFY currentLoopChanged
               BINDABLE bindableCurrentLoop)
    Q_PROPERTY(Direction direction READ direction WRITE setDirection NOTIFY directionChanged
               BINDABLE bindableDirection)
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
    QBindable<QAbstractAnimation::State> bindableState() const;

    QAnimationGroup *group() const;

    Direction direction() const;
    void setDirection(Direction direction);
    QBindable<Direction> bindableDirection();

    int currentTime() const;
    QBindable<int> bindableCurrentTime();

    int currentLoopTime() const;

    int loopCount() const;
    void setLoopCount(int loopCount);
    QBindable<int> bindableLoopCount();

    int currentLoop() const;
    QBindable<int> bindableCurrentLoop() const;

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

Q_SIGNALS:
    void started();
    void stopped();

protected:
    void advanceAnimation();
    virtual void start();
    virtual void stop();

    QAnimationDriver(QAnimationDriverPrivate &dd, QObject *parent = nullptr);

private:
    friend class QUnifiedTimer;

};

QT_END_NAMESPACE

#endif // QABSTRACTANIMATION_H
