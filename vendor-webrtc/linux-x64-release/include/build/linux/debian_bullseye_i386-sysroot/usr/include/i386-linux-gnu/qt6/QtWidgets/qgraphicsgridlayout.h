// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QGRAPHICSGRIDLAYOUT_H
#define QGRAPHICSGRIDLAYOUT_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtWidgets/qgraphicsitem.h>
#include <QtWidgets/qgraphicslayout.h>

QT_REQUIRE_CONFIG(graphicsview);

QT_BEGIN_NAMESPACE

class QGraphicsGridLayoutPrivate;

class Q_WIDGETS_EXPORT QGraphicsGridLayout : public QGraphicsLayout
{
public:
    QGraphicsGridLayout(QGraphicsLayoutItem *parent = nullptr);
    virtual ~QGraphicsGridLayout();

    void addItem(QGraphicsLayoutItem *item, int row, int column, int rowSpan, int columnSpan,
                 Qt::Alignment alignment = Qt::Alignment());
    inline void addItem(QGraphicsLayoutItem *item, int row, int column, Qt::Alignment alignment = Qt::Alignment());

    void setHorizontalSpacing(qreal spacing);
    qreal horizontalSpacing() const;
    void setVerticalSpacing(qreal spacing);
    qreal verticalSpacing() const;
    void setSpacing(qreal spacing);

    void setRowSpacing(int row, qreal spacing);
    qreal rowSpacing(int row) const;
    void setColumnSpacing(int column, qreal spacing);
    qreal columnSpacing(int column) const;

    void setRowStretchFactor(int row, int stretch);
    int rowStretchFactor(int row) const;
    void setColumnStretchFactor(int column, int stretch);
    int columnStretchFactor(int column) const;

    void setRowMinimumHeight(int row, qreal height);
    qreal rowMinimumHeight(int row) const;
    void setRowPreferredHeight(int row, qreal height);
    qreal rowPreferredHeight(int row) const;
    void setRowMaximumHeight(int row, qreal height);
    qreal rowMaximumHeight(int row) const;
    void setRowFixedHeight(int row, qreal height);

    void setColumnMinimumWidth(int column, qreal width);
    qreal columnMinimumWidth(int column) const;
    void setColumnPreferredWidth(int column, qreal width);
    qreal columnPreferredWidth(int column) const;
    void setColumnMaximumWidth(int column, qreal width);
    qreal columnMaximumWidth(int column) const;
    void setColumnFixedWidth(int column, qreal width);

    void setRowAlignment(int row, Qt::Alignment alignment);
    Qt::Alignment rowAlignment(int row) const;
    void setColumnAlignment(int column, Qt::Alignment alignment);
    Qt::Alignment columnAlignment(int column) const;

    void setAlignment(QGraphicsLayoutItem *item, Qt::Alignment alignment);
    Qt::Alignment alignment(QGraphicsLayoutItem *item) const;

    int rowCount() const;
    int columnCount() const;

    QGraphicsLayoutItem *itemAt(int row, int column) const;

    // inherited from QGraphicsLayout
    int count() const override;
    QGraphicsLayoutItem *itemAt(int index) const override;
    void removeAt(int index) override;
    void removeItem(QGraphicsLayoutItem *item);

    void invalidate() override;

    // inherited from QGraphicsLayoutItem
    void setGeometry(const QRectF &rect) override;
    QSizeF sizeHint(Qt::SizeHint which, const QSizeF &constraint = QSizeF()) const override;

    // ####
    //QRect cellRect(int row, int column, int rowSpan = 1, int columnSpan = 1) const;
    //QSizePolicy::ControlTypes controlTypes(LayoutSide side) const;

private:
    Q_DISABLE_COPY(QGraphicsGridLayout)
    Q_DECLARE_PRIVATE(QGraphicsGridLayout)
};

inline void QGraphicsGridLayout::addItem(QGraphicsLayoutItem *aitem, int arow, int acolumn, Qt::Alignment aalignment)
{ addItem(aitem, arow, acolumn, 1, 1, aalignment); }

QT_END_NAMESPACE

#endif
