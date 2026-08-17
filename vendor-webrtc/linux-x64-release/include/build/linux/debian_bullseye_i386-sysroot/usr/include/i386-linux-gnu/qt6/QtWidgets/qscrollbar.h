// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSCROLLBAR_H
#define QSCROLLBAR_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtWidgets/qwidget.h>

#include <QtWidgets/qabstractslider.h>

QT_REQUIRE_CONFIG(scrollbar);

QT_BEGIN_NAMESPACE

class QScrollBarPrivate;
class QStyleOptionSlider;

class Q_WIDGETS_EXPORT QScrollBar : public QAbstractSlider
{
    Q_OBJECT
public:
    explicit QScrollBar(QWidget *parent = nullptr);
    explicit QScrollBar(Qt::Orientation, QWidget *parent = nullptr);
    ~QScrollBar();

    QSize sizeHint() const override;
    bool event(QEvent *event) override;

protected:
#if QT_CONFIG(wheelevent)
    void wheelEvent(QWheelEvent *) override;
#endif
    void paintEvent(QPaintEvent *) override;
    void mousePressEvent(QMouseEvent *) override;
    void mouseReleaseEvent(QMouseEvent *) override;
    void mouseMoveEvent(QMouseEvent *) override;
    void hideEvent(QHideEvent*) override;
    void sliderChange(SliderChange change) override;
#ifndef QT_NO_CONTEXTMENU
    void contextMenuEvent(QContextMenuEvent *) override;
#endif
    virtual void initStyleOption(QStyleOptionSlider *option) const;


private:
    friend class QAbstractScrollAreaPrivate;
    friend Q_WIDGETS_EXPORT QStyleOptionSlider qt_qscrollbarStyleOption(QScrollBar *scrollBar);

    Q_DISABLE_COPY(QScrollBar)
    Q_DECLARE_PRIVATE(QScrollBar)
#if QT_CONFIG(itemviews)
    friend class QTableView;
    friend class QTreeViewPrivate;
    friend class QCommonListViewBase;
    friend class QListModeViewBase;
    friend class QAbstractItemView;
#endif
};

QT_END_NAMESPACE

#endif // QSCROLLBAR_H
