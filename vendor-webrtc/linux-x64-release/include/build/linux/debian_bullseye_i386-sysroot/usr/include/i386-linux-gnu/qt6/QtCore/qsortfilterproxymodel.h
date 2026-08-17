// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSORTFILTERPROXYMODEL_H
#define QSORTFILTERPROXYMODEL_H

#include <QtCore/qabstractproxymodel.h>

#include <QtCore/qregularexpression.h>

QT_REQUIRE_CONFIG(sortfilterproxymodel);

QT_BEGIN_NAMESPACE


class QSortFilterProxyModelPrivate;
class QSortFilterProxyModelLessThan;
class QSortFilterProxyModelGreaterThan;

class Q_CORE_EXPORT QSortFilterProxyModel : public QAbstractProxyModel
{
    friend class QSortFilterProxyModelLessThan;
    friend class QSortFilterProxyModelGreaterThan;

    Q_OBJECT
    Q_PROPERTY(QRegularExpression filterRegularExpression READ filterRegularExpression
               WRITE setFilterRegularExpression BINDABLE bindableFilterRegularExpression)
    Q_PROPERTY(int filterKeyColumn READ filterKeyColumn WRITE setFilterKeyColumn
               BINDABLE bindableFilterKeyColumn)
    Q_PROPERTY(bool dynamicSortFilter READ dynamicSortFilter WRITE setDynamicSortFilter
               BINDABLE bindableDynamicSortFilter)
    Q_PROPERTY(Qt::CaseSensitivity filterCaseSensitivity READ filterCaseSensitivity
               WRITE setFilterCaseSensitivity NOTIFY filterCaseSensitivityChanged
               BINDABLE bindableFilterCaseSensitivity)
    Q_PROPERTY(Qt::CaseSensitivity sortCaseSensitivity READ sortCaseSensitivity
               WRITE setSortCaseSensitivity NOTIFY sortCaseSensitivityChanged
               BINDABLE bindableSortCaseSensitivity)
    Q_PROPERTY(bool isSortLocaleAware READ isSortLocaleAware WRITE setSortLocaleAware
               NOTIFY sortLocaleAwareChanged BINDABLE bindableIsSortLocaleAware)
    Q_PROPERTY(int sortRole READ sortRole WRITE setSortRole NOTIFY sortRoleChanged
               BINDABLE bindableSortRole)
    Q_PROPERTY(int filterRole READ filterRole WRITE setFilterRole NOTIFY filterRoleChanged
               BINDABLE bindableFilterRole)
    Q_PROPERTY(bool recursiveFilteringEnabled READ isRecursiveFilteringEnabled
               WRITE setRecursiveFilteringEnabled NOTIFY recursiveFilteringEnabledChanged
               BINDABLE bindableRecursiveFilteringEnabled)
    Q_PROPERTY(bool autoAcceptChildRows READ autoAcceptChildRows WRITE setAutoAcceptChildRows
               NOTIFY autoAcceptChildRowsChanged BINDABLE bindableAutoAcceptChildRows)

public:
    explicit QSortFilterProxyModel(QObject *parent = nullptr);
    ~QSortFilterProxyModel();

    void setSourceModel(QAbstractItemModel *sourceModel) override;

    QModelIndex mapToSource(const QModelIndex &proxyIndex) const override;
    QModelIndex mapFromSource(const QModelIndex &sourceIndex) const override;

    QItemSelection mapSelectionToSource(const QItemSelection &proxySelection) const override;
    QItemSelection mapSelectionFromSource(const QItemSelection &sourceSelection) const override;

    QRegularExpression filterRegularExpression() const;
    QBindable<QRegularExpression> bindableFilterRegularExpression();

    int filterKeyColumn() const;
    void setFilterKeyColumn(int column);
    QBindable<int> bindableFilterKeyColumn();

    Qt::CaseSensitivity filterCaseSensitivity() const;
    void setFilterCaseSensitivity(Qt::CaseSensitivity cs);
    QBindable<Qt::CaseSensitivity> bindableFilterCaseSensitivity();

    Qt::CaseSensitivity sortCaseSensitivity() const;
    void setSortCaseSensitivity(Qt::CaseSensitivity cs);
    QBindable<Qt::CaseSensitivity> bindableSortCaseSensitivity();

    bool isSortLocaleAware() const;
    void setSortLocaleAware(bool on);
    QBindable<bool> bindableIsSortLocaleAware();

    int sortColumn() const;
    Qt::SortOrder sortOrder() const;

    bool dynamicSortFilter() const;
    void setDynamicSortFilter(bool enable);
    QBindable<bool> bindableDynamicSortFilter();

    int sortRole() const;
    void setSortRole(int role);
    QBindable<int> bindableSortRole();

    int filterRole() const;
    void setFilterRole(int role);
    QBindable<int> bindableFilterRole();

    bool isRecursiveFilteringEnabled() const;
    void setRecursiveFilteringEnabled(bool recursive);
    QBindable<bool> bindableRecursiveFilteringEnabled();

    bool autoAcceptChildRows() const;
    void setAutoAcceptChildRows(bool accept);
    QBindable<bool> bindableAutoAcceptChildRows();

public Q_SLOTS:
    void setFilterRegularExpression(const QString &pattern);
    void setFilterRegularExpression(const QRegularExpression &regularExpression);
    void setFilterWildcard(const QString &pattern);
    void setFilterFixedString(const QString &pattern);
    void invalidate();

protected:
    virtual bool filterAcceptsRow(int source_row, const QModelIndex &source_parent) const;
    virtual bool filterAcceptsColumn(int source_column, const QModelIndex &source_parent) const;
    virtual bool lessThan(const QModelIndex &source_left, const QModelIndex &source_right) const;

    void invalidateFilter();
    void invalidateRowsFilter();
    void invalidateColumnsFilter();

public:
    using QObject::parent;

    QModelIndex index(int row, int column, const QModelIndex &parent = QModelIndex()) const override;
    QModelIndex parent(const QModelIndex &child) const override;
    QModelIndex sibling(int row, int column, const QModelIndex &idx) const override;

    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    int columnCount(const QModelIndex &parent = QModelIndex()) const override;
    bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;

    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    bool setData(const QModelIndex &index, const QVariant &value, int role = Qt::EditRole) override;

    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;
    bool setHeaderData(int section, Qt::Orientation orientation,
            const QVariant &value, int role = Qt::EditRole) override;

    QMimeData *mimeData(const QModelIndexList &indexes) const override;
    bool dropMimeData(const QMimeData *data, Qt::DropAction action,
                      int row, int column, const QModelIndex &parent) override;

    bool insertRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    bool insertColumns(int column, int count, const QModelIndex &parent = QModelIndex()) override;
    bool removeRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    bool removeColumns(int column, int count, const QModelIndex &parent = QModelIndex()) override;

    void fetchMore(const QModelIndex &parent) override;
    bool canFetchMore(const QModelIndex &parent) const override;
    Qt::ItemFlags flags(const QModelIndex &index) const override;

    QModelIndex buddy(const QModelIndex &index) const override;
    QModelIndexList match(const QModelIndex &start, int role,
                          const QVariant &value, int hits = 1,
                          Qt::MatchFlags flags =
                          Qt::MatchFlags(Qt::MatchStartsWith|Qt::MatchWrap)) const override;
    QSize span(const QModelIndex &index) const override;
    void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;

    QStringList mimeTypes() const override;
    Qt::DropActions supportedDropActions() const override;

Q_SIGNALS:
    void dynamicSortFilterChanged(bool dynamicSortFilter);
    void filterCaseSensitivityChanged(Qt::CaseSensitivity filterCaseSensitivity);
    void sortCaseSensitivityChanged(Qt::CaseSensitivity sortCaseSensitivity);
    void sortLocaleAwareChanged(bool sortLocaleAware);
    void sortRoleChanged(int sortRole);
    void filterRoleChanged(int filterRole);
    void recursiveFilteringEnabledChanged(bool recursiveFilteringEnabled);
    void autoAcceptChildRowsChanged(bool autoAcceptChildRows);

private:
    Q_DECLARE_PRIVATE(QSortFilterProxyModel)
    Q_DISABLE_COPY(QSortFilterProxyModel)

    Q_PRIVATE_SLOT(d_func(),
                   void _q_sourceDataChanged(const QModelIndex &source_top_left, const QModelIndex &source_bottom_right,
                                             const QList<int> &roles))
    Q_PRIVATE_SLOT(d_func(), void _q_sourceHeaderDataChanged(Qt::Orientation orientation, int start, int end))
    Q_PRIVATE_SLOT(d_func(), void _q_sourceAboutToBeReset())
    Q_PRIVATE_SLOT(d_func(), void _q_sourceReset())
    Q_PRIVATE_SLOT(d_func(), void _q_sourceLayoutAboutToBeChanged(const QList<QPersistentModelIndex> &sourceParents, QAbstractItemModel::LayoutChangeHint hint))
    Q_PRIVATE_SLOT(d_func(), void _q_sourceLayoutChanged(const QList<QPersistentModelIndex> &sourceParents, QAbstractItemModel::LayoutChangeHint hint))
    Q_PRIVATE_SLOT(d_func(), void _q_sourceRowsAboutToBeInserted(const QModelIndex &source_parent, int start, int end))
    Q_PRIVATE_SLOT(d_func(), void _q_sourceRowsInserted(const QModelIndex &source_parent, int start, int end))
    Q_PRIVATE_SLOT(d_func(), void _q_sourceRowsAboutToBeRemoved(const QModelIndex &source_parent, int start, int end))
    Q_PRIVATE_SLOT(d_func(), void _q_sourceRowsRemoved(const QModelIndex &source_parent, int start, int end))
    Q_PRIVATE_SLOT(d_func(), void _q_sourceRowsAboutToBeMoved(QModelIndex,int,int,QModelIndex,int))
    Q_PRIVATE_SLOT(d_func(), void _q_sourceRowsMoved(QModelIndex,int,int,QModelIndex,int))
    Q_PRIVATE_SLOT(d_func(), void _q_sourceColumnsAboutToBeInserted(const QModelIndex &source_parent, int start, int end))
    Q_PRIVATE_SLOT(d_func(), void _q_sourceColumnsInserted(const QModelIndex &source_parent, int start, int end))
    Q_PRIVATE_SLOT(d_func(), void _q_sourceColumnsAboutToBeRemoved(const QModelIndex &source_parent, int start, int end))
    Q_PRIVATE_SLOT(d_func(), void _q_sourceColumnsRemoved(const QModelIndex &source_parent, int start, int end))
    Q_PRIVATE_SLOT(d_func(), void _q_sourceColumnsAboutToBeMoved(QModelIndex,int,int,QModelIndex,int))
    Q_PRIVATE_SLOT(d_func(), void _q_sourceColumnsMoved(QModelIndex,int,int,QModelIndex,int))
    Q_PRIVATE_SLOT(d_func(), void _q_clearMapping())
};

QT_END_NAMESPACE

#endif // QSORTFILTERPROXYMODEL_H
