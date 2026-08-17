/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtWidgets module of the Qt Toolkit.
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
