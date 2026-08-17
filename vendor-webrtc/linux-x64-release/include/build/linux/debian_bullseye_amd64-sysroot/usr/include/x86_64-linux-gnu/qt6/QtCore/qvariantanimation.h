// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QVARIANTANIMATION_H
#define QVARIANTANIMATION_H

#include <QtCore/qabstractanimation.h>
#include <QtCore/qeasingcurve.h>
#include <QtCore/qlist.h>
#include <QtCore/qpair.h>
#include <QtCore/qvariant.h>

QT_REQUIRE_CONFIG(animation);

QT_BEGIN_NAMESPACE

class QVariantAnimationPrivate;
class Q_CORE_EXPORT QVariantAnimation : public QAbstractAnimation
{
    Q_OBJECT
    Q_PROPERTY(QVariant startValue READ startValue WRITE setStartValue)
    Q_PROPERTY(QVariant endValue READ endValue WRITE setEndValue)
    Q_PROPERTY(QVariant currentValue READ currentValue NOTIFY valueChanged)
    Q_PROPERTY(int duration READ duration WRITE setDuration BINDABLE bindableDuration)
    Q_PROPERTY(QEasingCurve easingCurve READ easingCurve WRITE setEasingCurve
               BINDABLE bindableEasingCurve)

public:
    typedef QPair<qreal, QVariant> KeyValue;
    typedef QList<KeyValue> KeyValues;

    QVariantAnimation(QObject *parent = nullptr);
    ~QVariantAnimation();

    QVariant startValue() const;
    void setStartValue(const QVariant &value);

    QVariant endValue() const;
    void setEndValue(const QVariant &value);

    QVariant keyValueAt(qreal step) const;
    void setKeyValueAt(qreal step, const QVariant &value);

    KeyValues keyValues() const;
    void setKeyValues(const KeyValues &values);

    QVariant currentValue() const;

    int duration() const override;
    void setDuration(int msecs);
    QBindable<int> bindableDuration();

    QEasingCurve easingCurve() const;
    void setEasingCurve(const QEasingCurve &easing);
    QBindable<QEasingCurve> bindableEasingCurve();

    typedef QVariant (*Interpolator)(const void *from, const void *to, qreal progress);

Q_SIGNALS:
    void valueChanged(const QVariant &value);

protected:
    QVariantAnimation(QVariantAnimationPrivate &dd, QObject *parent = nullptr);
    bool event(QEvent *event) override;

    void updateCurrentTime(int) override;
    void updateState(QAbstractAnimation::State newState, QAbstractAnimation::State oldState) override;

    virtual void updateCurrentValue(const QVariant &value);
    virtual QVariant interpolated(const QVariant &from, const QVariant &to, qreal progress) const;

private:
    template <typename T> friend void qRegisterAnimationInterpolator(QVariant (*func)(const T &, const T &, qreal));
    static void registerInterpolator(Interpolator func, int interpolationType);

    Q_DISABLE_COPY(QVariantAnimation)
    Q_DECLARE_PRIVATE(QVariantAnimation)
};

template <typename T>
void qRegisterAnimationInterpolator(QVariant (*func)(const T &from, const T &to, qreal progress)) {
    QVariantAnimation::registerInterpolator(reinterpret_cast<QVariantAnimation::Interpolator>(reinterpret_cast<void(*)()>(func)), qMetaTypeId<T>());
}

QT_END_NAMESPACE

#endif //QVARIANTANIMATION_H
