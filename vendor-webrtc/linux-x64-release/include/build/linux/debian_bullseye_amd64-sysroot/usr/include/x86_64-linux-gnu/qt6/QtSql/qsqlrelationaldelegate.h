// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSQLRELATIONALDELEGATE_H
#define QSQLRELATIONALDELEGATE_H

#include <QtSql/qtsqlglobal.h>

QT_REQUIRE_CONFIG(sqlmodel);

#ifdef QT_WIDGETS_LIB

#include <QtWidgets/qstyleditemdelegate.h>
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

class QSqlRelationalDelegate : public QStyledItemDelegate
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
        : QStyledItemDelegate(aParent)
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
            return QStyledItemDelegate::createEditor(aParent, option, index);
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
                    v = QVariant(editor->property(n.data()).metaType());
                editor->setProperty(n.data(), v);
                return;
            }
        }
        QStyledItemDelegate::setEditorData(editor, index);
    }

void setModelData(QWidget *editor, QAbstractItemModel *model, const QModelIndex &index) const override
{
    if (!index.isValid())
        return;

    QSqlRelationalTableModel *sqlModel = qobject_cast<QSqlRelationalTableModel *>(model);
    QSqlTableModel *childModel = sqlModel ? sqlModel->relationModel(index.column()) : nullptr;
    QComboBox *combo = qobject_cast<QComboBox *>(editor);
    if (!sqlModel || !childModel || !combo) {
        QStyledItemDelegate::setModelData(editor, model, index);
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
