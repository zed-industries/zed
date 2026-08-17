// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSLIDER_H
#define QSLIDER_H

#include <QtWidgets/qtwidgetsglobal.h>

#include <QtWidgets/qabstractslider.h>

QT_REQUIRE_CONFIG(slider);

QT_BEGIN_NAMESPACE

class QSliderPrivate;
class QStyleOptionSlider;
class Q_WIDGETS_EXPORT QSlider : public QAbstractSlider
{
    Q_OBJECT

    Q_PROPERTY(TickPosition tickPosition READ tickPosition WRITE setTickPosition)
    Q_PROPERTY(int tickInterval READ tickInterval WRITE setTickInterval)

public:
    enum TickPosition {
        NoTicks = 0,
        TicksAbove = 1,
        TicksLeft = TicksAbove,
        TicksBelow = 2,
        TicksRight = TicksBelow,
        TicksBothSides = 3
    };
    Q_ENUM(TickPosition)

    explicit QSlider(QWidget *parent = nullptr);
    explicit QSlider(Qt::Orientation orientation, QWidget *parent = nullptr);

    ~QSlider();

    QSize sizeHint() const override;
    QSize minimumSizeHint() const override;

    void setTickPosition(TickPosition position);
    TickPosition tickPosition() const;

    void setTickInterval(int ti);
    int tickInterval() const;

    bool event(QEvent *event) override;

protected:
    void paintEvent(QPaintEvent *ev) override;
    void mousePressEvent(QMouseEvent *ev) override;
    void mouseReleaseEvent(QMouseEvent *ev) override;
    void mouseMoveEvent(QMouseEvent *ev) override;
    virtual void initStyleOption(QStyleOptionSlider *option) const;


private:
    friend Q_WIDGETS_EXPORT QStyleOptionSlider qt_qsliderStyleOption(QSlider *slider);

    Q_DISABLE_COPY(QSlider)
    Q_DECLARE_PRIVATE(QSlider)
};

QT_END_NAMESPACE

#endif // QSLIDER_H
