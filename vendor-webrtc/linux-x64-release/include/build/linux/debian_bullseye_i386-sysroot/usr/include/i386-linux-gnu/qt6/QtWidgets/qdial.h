// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only


#ifndef QDIAL_H
#define QDIAL_H

#include <QtWidgets/qtwidgetsglobal.h>

#include <QtWidgets/qabstractslider.h>

QT_REQUIRE_CONFIG(dial);

QT_BEGIN_NAMESPACE

class QDialPrivate;
class QStyleOptionSlider;

class Q_WIDGETS_EXPORT QDial: public QAbstractSlider
{
    Q_OBJECT

    Q_PROPERTY(bool wrapping READ wrapping WRITE setWrapping)
    Q_PROPERTY(int notchSize READ notchSize)
    Q_PROPERTY(qreal notchTarget READ notchTarget WRITE setNotchTarget)
    Q_PROPERTY(bool notchesVisible READ notchesVisible WRITE setNotchesVisible)
public:
    explicit QDial(QWidget *parent = nullptr);

    ~QDial();

    bool wrapping() const;

    int notchSize() const;

    void setNotchTarget(double target);
    qreal notchTarget() const;
    bool notchesVisible() const;

    QSize sizeHint() const override;
    QSize minimumSizeHint() const override;

public Q_SLOTS:
    void setNotchesVisible(bool visible);
    void setWrapping(bool on);

protected:
    bool event(QEvent *e) override;
    void resizeEvent(QResizeEvent *re) override;
    void paintEvent(QPaintEvent *pe) override;

    void mousePressEvent(QMouseEvent *me) override;
    void mouseReleaseEvent(QMouseEvent *me) override;
    void mouseMoveEvent(QMouseEvent *me) override;

    void sliderChange(SliderChange change) override;
    virtual void initStyleOption(QStyleOptionSlider *option) const;


private:
    Q_DECLARE_PRIVATE(QDial)
    Q_DISABLE_COPY(QDial)
};

QT_END_NAMESPACE

#endif // QDIAL_H
