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

#ifndef QFORMLAYOUT_H
#define QFORMLAYOUT_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtWidgets/QLayout>

QT_REQUIRE_CONFIG(formlayout);

QT_BEGIN_NAMESPACE


class QFormLayoutPrivate;

class Q_WIDGETS_EXPORT QFormLayout : public QLayout
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QFormLayout)
    Q_PROPERTY(FieldGrowthPolicy fieldGrowthPolicy READ fieldGrowthPolicy WRITE setFieldGrowthPolicy RESET resetFieldGrowthPolicy)
    Q_PROPERTY(RowWrapPolicy rowWrapPolicy READ rowWrapPolicy WRITE setRowWrapPolicy RESET resetRowWrapPolicy)
    Q_PROPERTY(Qt::Alignment labelAlignment READ labelAlignment WRITE setLabelAlignment RESET resetLabelAlignment)
    Q_PROPERTY(Qt::Alignment formAlignment READ formAlignment WRITE setFormAlignment RESET resetFormAlignment)
    Q_PROPERTY(int horizontalSpacing READ horizontalSpacing WRITE setHorizontalSpacing)
    Q_PROPERTY(int verticalSpacing READ verticalSpacing WRITE setVerticalSpacing)

public:
    enum FieldGrowthPolicy {
        FieldsStayAtSizeHint,
        ExpandingFieldsGrow,
        AllNonFixedFieldsGrow
    };
    Q_ENUM(FieldGrowthPolicy)

    enum RowWrapPolicy {
        DontWrapRows,
        WrapLongRows,
        WrapAllRows
    };
    Q_ENUM(RowWrapPolicy)

    enum ItemRole {
        LabelRole = 0,
        FieldRole = 1,
        SpanningRole = 2
    };
    Q_ENUM(ItemRole)

    struct TakeRowResult {
        QLayoutItem *labelItem;
        QLayoutItem *fieldItem;
    };

    explicit QFormLayout(QWidget *parent = nullptr);
    ~QFormLayout();

    void setFieldGrowthPolicy(FieldGrowthPolicy policy);
    FieldGrowthPolicy fieldGrowthPolicy() const;
    void setRowWrapPolicy(RowWrapPolicy policy);
    RowWrapPolicy rowWrapPolicy() const;
    void setLabelAlignment(Qt::Alignment alignment);
    Qt::Alignment labelAlignment() const;
    void setFormAlignment(Qt::Alignment alignment);
    Qt::Alignment formAlignment() const;

    void setHorizontalSpacing(int spacing);
    int horizontalSpacing() const;
    void setVerticalSpacing(int spacing);
    int verticalSpacing() const;

    int spacing() const;
    void setSpacing(int);

    void addRow(QWidget *label, QWidget *field);
    void addRow(QWidget *label, QLayout *field);
    void addRow(const QString &labelText, QWidget *field);
    void addRow(const QString &labelText, QLayout *field);
    void addRow(QWidget *widget);
    void addRow(QLayout *layout);

    void insertRow(int row, QWidget *label, QWidget *field);
    void insertRow(int row, QWidget *label, QLayout *field);
    void insertRow(int row, const QString &labelText, QWidget *field);
    void insertRow(int row, const QString &labelText, QLayout *field);
    void insertRow(int row, QWidget *widget);
    void insertRow(int row, QLayout *layout);

    void removeRow(int row);
    void removeRow(QWidget *widget);
    void removeRow(QLayout *layout);

    TakeRowResult takeRow(int row);
    TakeRowResult takeRow(QWidget *widget);
    TakeRowResult takeRow(QLayout *layout);

    void setItem(int row, ItemRole role, QLayoutItem *item);
    void setWidget(int row, ItemRole role, QWidget *widget);
    void setLayout(int row, ItemRole role, QLayout *layout);

    QLayoutItem *itemAt(int row, ItemRole role) const;
    void getItemPosition(int index, int *rowPtr, ItemRole *rolePtr) const;
    void getWidgetPosition(QWidget *widget, int *rowPtr, ItemRole *rolePtr) const;
    void getLayoutPosition(QLayout *layout, int *rowPtr, ItemRole *rolePtr) const;
    QWidget *labelForField(QWidget *field) const;
    QWidget *labelForField(QLayout *field) const;

    // reimplemented from QLayout
    void addItem(QLayoutItem *item) override;
    QLayoutItem *itemAt(int index) const override;
    QLayoutItem *takeAt(int index) override;

    void setGeometry(const QRect &rect) override;
    QSize minimumSize() const override;
    QSize sizeHint() const override;
    void invalidate() override;

    bool hasHeightForWidth() const override;
    int heightForWidth(int width) const override;
    Qt::Orientations expandingDirections() const override;
    int count() const override;

    int rowCount() const;

#if 0
    void dump() const;
#endif

private:
    void resetFieldGrowthPolicy();
    void resetRowWrapPolicy();
    void resetLabelAlignment();
    void resetFormAlignment();
};

Q_DECLARE_TYPEINFO(QFormLayout::TakeRowResult, Q_PRIMITIVE_TYPE);

QT_END_NAMESPACE

#endif
