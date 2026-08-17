// Copyright (C) 2018 Luca Beldi <v.ronin@yahoo.it>
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTRANSPOSEPROXYMODEL_H
#define QTRANSPOSEPROXYMODEL_H

#include <QtCore/qabstractproxymodel.h>
#include <QtCore/qscopedpointer.h>

QT_REQUIRE_CONFIG(transposeproxymodel);

QT_BEGIN_NAMESPACE

class QTransposeProxyModelPrivate;

class Q_CORE_EXPORT QTransposeProxyModel : public QAbstractProxyModel
{
    Q_OBJECT
    Q_DISABLE_COPY(QTransposeProxyModel)
    Q_DECLARE_PRIVATE(QTransposeProxyModel)
public:
    explicit QTransposeProxyModel(QObject* parent = nullptr);
    ~QTransposeProxyModel();
    void setSourceModel(QAbstractItemModel* newSourceModel) override;
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    int columnCount(const QModelIndex &parent = QModelIndex()) const override;
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;
    bool setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role = Qt::EditRole) override;
    bool setItemData(const QModelIndex &index, const QMap<int, QVariant> &roles) override;
    QSize span(const QModelIndex &index) const override;
    QMap<int, QVariant> itemData(const QModelIndex &index) const override;
    QModelIndex mapFromSource(const QModelIndex &sourceIndex) const override;
    QModelIndex mapToSource(const QModelIndex &proxyIndex) const override;
    QModelIndex parent(const QModelIndex &index) const override;
    QModelIndex index(int row, int column, const QModelIndex &parent = QModelIndex()) const override;
    bool insertRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    bool removeRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    bool moveRows(const QModelIndex &sourceParent, int sourceRow, int count, const QModelIndex &destinationParent, int destinationChild) override;
    bool insertColumns(int column, int count, const QModelIndex &parent = QModelIndex()) override;
    bool removeColumns(int column, int count, const QModelIndex &parent = QModelIndex()) override;
    bool moveColumns(const QModelIndex &sourceParent, int sourceColumn, int count, const QModelIndex &destinationParent, int destinationChild) override;
    void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
protected:
    QTransposeProxyModel(QTransposeProxyModelPrivate &, QObject *parent);
};

QT_END_NAMESPACE

#endif // QTRANSPOSEPROXYMODEL_H
