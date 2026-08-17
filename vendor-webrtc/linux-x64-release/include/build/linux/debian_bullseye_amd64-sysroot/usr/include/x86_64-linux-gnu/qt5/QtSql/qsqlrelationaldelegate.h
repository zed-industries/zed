/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtSql module of the Qt Toolkit.
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

#ifndef QSQLRELATIONALDELEGATE_H
#define QSQLRELATIONALDELEGATE_H

#include <QtSql/qtsqlglobal.h>

QT_REQUIRE_CONFIG(sqlmodel);

#ifdef QT_WIDGETS_LIB

#include <QtWidgets/qitemdelegate.h>
#if QT_CONFIG(listview)
#include <QtWidgets/qlistview.h>
#endif
#if QT_CONFIG(combobox)
#include <QtWidgets/qcombobox.h>
#endif
#include <QtSql/qsqldriver.h>
#include <QtSql/qsqlrelationaltablemodel.h>
#include <QtCore/qmetaobject.h>
QT_BEGIN_NAMESPACE

// ### Qt6: QStyledItemDelegate
class QSqlRelationalDelegate: public QItemDelegate
{
    static int fieldIndex(const QSqlTableModel *const model,
                          const QSqlDriver *const driver,
                          const QString &fieldName)
    {
        const QString stripped = driver->isIdentifierEscaped(fieldName, QSqlDriver::FieldName)
                ? driver->stripDelimiters(fieldName, QSqlDriver::FieldName)
                : fieldName;
        return model->fieldIndex(stripped);
    }

public:

explicit QSqlRelationalDelegate(QObject *aParent = nullptr)
    : QItemDelegate(aParent)
{}

~QSqlRelationalDelegate()
{}

QWidget *createEditor(QWidget *aParent,
                      const QStyleOptionViewItem &option,
                      const QModelIndex &index) const override
{
    const QSqlRelationalTableModel *sqlModel = qobject_cast<const QSqlRelationalTableModel *>(index.model());
    QSqlTableModel *childModel = sqlModel ? sqlModel->relationModel(index.column()) : nullptr;
    if (!childModel)
        return QItemDelegate::createEditor(aParent, option, index);
    const QSqlDriver *const driver = childModel->database().driver();

    QComboBox *combo = new QComboBox(aParent);
    combo->setModel(childModel);
    combo->setModelColumn(fieldIndex(childModel, driver,
                                     sqlModel->relation(index.column()).displayColumn()));
    combo->installEventFilter(const_cast<QSqlRelationalDelegate *>(this));

    return combo;
}

    void setEditorData(QWidget *editor, const QModelIndex &index) const override
    {
        if (!index.isValid())
            return;

        if (qobject_cast<QComboBox *>(editor)) {
            // Taken from QItemDelegate::setEditorData() as we need
            // to present the DisplayRole and not the EditRole which
            // is the id reference to the related model
            QVariant v = index.data(Qt::DisplayRole);
            const QByteArray n = editor->metaObject()->userProperty().name();
            if (!n.isEmpty()) {
                if (!v.isValid())
                    v = QVariant(editor->property(n.data()).userType(), nullptr);
                editor->setProperty(n.data(), v);
                return;
            }
        }
        QItemDelegate::setEditorData(editor, index);
    }

void setModelData(QWidget *editor, QAbstractItemModel *model, const QModelIndex &index) const override
{
    if (!index.isValid())
        return;

    QSqlRelationalTableModel *sqlModel = qobject_cast<QSqlRelationalTableModel *>(model);
    QSqlTableModel *childModel = sqlModel ? sqlModel->relationModel(index.column()) : nullptr;
    QComboBox *combo = qobject_cast<QComboBox *>(editor);
    if (!sqlModel || !childModel || !combo) {
        QItemDelegate::setModelData(editor, model, index);
        return;
    }
    const QSqlDriver *const driver = childModel->database().driver();

    int currentItem = combo->currentIndex();
    int childColIndex = fieldIndex(childModel, driver,
                                   sqlModel->relation(index.column()).displayColumn());
    int childEditIndex = fieldIndex(childModel, driver,
                                    sqlModel->relation(index.column()).indexColumn());
    sqlModel->setData(index,
            childModel->data(childModel->index(currentItem, childColIndex), Qt::DisplayRole),
            Qt::DisplayRole);
    sqlModel->setData(index,
            childModel->data(childModel->index(currentItem, childEditIndex), Qt::EditRole),
            Qt::EditRole);
}

};

QT_END_NAMESPACE

#endif // QT_WIDGETS_LIB

#endif // QSQLRELATIONALDELEGATE_H
