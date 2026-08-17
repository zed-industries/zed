// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QPROPERTYANIMATION_H
#define QPROPERTYANIMATION_H

#include <QtCore/qvariantanimation.h>

QT_REQUIRE_CONFIG(animation);

QT_BEGIN_NAMESPACE

class QPropertyAnimationPrivate;
class Q_CORE_EXPORT QPropertyAnimation : public QVariantAnimation
{
    Q_OBJECT
    Q_PROPERTY(QByteArray propertyName READ propertyName WRITE setPropertyName
               BINDABLE bindablePropertyName)
    Q_PROPERTY(QObject* targetObject READ targetObject WRITE setTargetObject
               BINDABLE bindableTargetObject)

public:
    QPropertyAnimation(QObject *parent = nullptr);
    QPropertyAnimation(QObject *target, const QByteArray &propertyName, QObject *parent = nullptr);
    ~QPropertyAnimation();

    QObject *targetObject() const;
    void setTargetObject(QObject *target);
    QBindable<QObject *> bindableTargetObject();

    QByteArray propertyName() const;
    void setPropertyName(const QByteArray &propertyName);
    QBindable<QByteArray> bindablePropertyName();

protected:
    bool event(QEvent *event) override;
    void updateCurrentValue(const QVariant &value) override;
    void updateState(QAbstractAnimation::State newState, QAbstractAnimation::State oldState) override;

private:
    Q_DISABLE_COPY(QPropertyAnimation)
    Q_DECLARE_PRIVATE(QPropertyAnimation)
};

QT_END_NAMESPACE

#endif // QPROPERTYANIMATION_H
