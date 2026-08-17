// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSIZEGRIP_H
#define QSIZEGRIP_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtWidgets/qwidget.h>

QT_REQUIRE_CONFIG(sizegrip);

QT_BEGIN_NAMESPACE

class QSizeGripPrivate;
class Q_WIDGETS_EXPORT QSizeGrip : public QWidget
{
    Q_OBJECT
public:
    explicit QSizeGrip(QWidget *parent);
    ~QSizeGrip();

    QSize sizeHint() const override;
    void setVisible(bool) override;

protected:
    void paintEvent(QPaintEvent *) override;
    void mousePressEvent(QMouseEvent *) override;
    void mouseMoveEvent(QMouseEvent *) override;
    void mouseReleaseEvent(QMouseEvent *mouseEvent) override;
    void moveEvent(QMoveEvent *moveEvent) override;
    void showEvent(QShowEvent *showEvent) override;
    void hideEvent(QHideEvent *hideEvent) override;
    bool eventFilter(QObject *, QEvent *) override;
    bool event(QEvent *) override;

public:

private:
    Q_DECLARE_PRIVATE(QSizeGrip)
    Q_DISABLE_COPY(QSizeGrip)
    Q_PRIVATE_SLOT(d_func(), void _q_showIfNotHidden())
};

QT_END_NAMESPACE

#endif // QSIZEGRIP_H
