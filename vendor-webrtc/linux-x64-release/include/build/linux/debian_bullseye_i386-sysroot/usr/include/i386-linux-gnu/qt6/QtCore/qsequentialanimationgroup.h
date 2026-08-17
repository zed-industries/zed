// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSEQUENTIALANIMATIONGROUP_H
#define QSEQUENTIALANIMATIONGROUP_H

#include <QtCore/qanimationgroup.h>

QT_REQUIRE_CONFIG(animation);

QT_BEGIN_NAMESPACE

class QPauseAnimation;
class QSequentialAnimationGroupPrivate;

class Q_CORE_EXPORT QSequentialAnimationGroup : public QAnimationGroup
{
    Q_OBJECT
    Q_PROPERTY(QAbstractAnimation *currentAnimation READ currentAnimation
               NOTIFY currentAnimationChanged BINDABLE bindableCurrentAnimation)

public:
    QSequentialAnimationGroup(QObject *parent = nullptr);
    ~QSequentialAnimationGroup();

    QPauseAnimation *addPause(int msecs);
    QPauseAnimation *insertPause(int index, int msecs);

    QAbstractAnimation *currentAnimation() const;
    QBindable<QAbstractAnimation *> bindableCurrentAnimation() const;
    int duration() const override;

Q_SIGNALS:
    void currentAnimationChanged(QAbstractAnimation *current);

protected:
    QSequentialAnimationGroup(QSequentialAnimationGroupPrivate &dd, QObject *parent);
    bool event(QEvent *event) override;

    void updateCurrentTime(int) override;
    void updateState(QAbstractAnimation::State newState, QAbstractAnimation::State oldState) override;
    void updateDirection(QAbstractAnimation::Direction direction) override;

private:
    Q_DISABLE_COPY(QSequentialAnimationGroup)
    Q_DECLARE_PRIVATE(QSequentialAnimationGroup)
    Q_PRIVATE_SLOT(d_func(), void _q_uncontrolledAnimationFinished())
};

QT_END_NAMESPACE

#endif //QSEQUENTIALANIMATIONGROUP_H
