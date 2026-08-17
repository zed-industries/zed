// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QITEMSELECTIONMODEL_H
#define QITEMSELECTIONMODEL_H

#include <QtCore/qglobal.h>

#include <QtCore/qabstractitemmodel.h>
#include <QtCore/qlist.h>
#include <QtCore/qset.h>

QT_REQUIRE_CONFIG(itemmodel);

QT_BEGIN_NAMESPACE

class Q_CORE_EXPORT QItemSelectionRange
{

public:
    QItemSelectionRange() = default;
    QItemSelectionRange(const QModelIndex &topL, const QModelIndex &bottomR) : tl(topL), br(bottomR) {}
    explicit QItemSelectionRange(const QModelIndex &index) : tl(index), br(tl) {}

    void swap(QItemSelectionRange &other) noexcept
    {
        tl.swap(other.tl);
        br.swap(other.br);
    }

    inline int top() const { return tl.row(); }
    inline int left() const { return tl.column(); }
    inline int bottom() const { return br.row(); }
    inline int right() const { return br.column(); }
    inline int width() const { return br.column() - tl.column() + 1; }
    inline int height() const { return br.row() - tl.row() + 1; }

    inline const QPersistentModelIndex &topLeft() const { return tl; }
    inline const QPersistentModelIndex &bottomRight() const { return br; }
    inline QModelIndex parent() const { return tl.parent(); }
    inline const QAbstractItemModel *model() const { return tl.model(); }

    inline bool contains(const QModelIndex &index) const
    {
        return (parent() == index.parent()
                && tl.row() <= index.row() && tl.column() <= index.column()
                && br.row() >= index.row() && br.column() >= index.column());
    }

    inline bool contains(int row, int column, const QModelIndex &parentIndex) const
    {
        return (parent() == parentIndex
                && tl.row() <= row && tl.column() <= column
                && br.row() >= row && br.column() >= column);
    }

    bool intersects(const QItemSelectionRange &other) const;
    QItemSelectionRange intersected(const QItemSelectionRange &other) const;


    inline bool operator==(const QItemSelectionRange &other) const
        { return (tl == other.tl && br == other.br); }
    inline bool operator!=(const QItemSelectionRange &other) const
        { return !operator==(other); }

    inline bool isValid() const
    {
        return (tl.isValid() && br.isValid() && tl.parent() == br.parent()
                && top() <= bottom() && left() <= right());
    }

    bool isEmpty() const;

    QModelIndexList indexes() const;

private:
    QPersistentModelIndex tl, br;
};
Q_DECLARE_TYPEINFO(QItemSelectionRange, Q_RELOCATABLE_TYPE);

class QItemSelection;
class QItemSelectionModelPrivate;

class Q_CORE_EXPORT QItemSelectionModel : public QObject
{
    Q_OBJECT
    Q_PROPERTY(QAbstractItemModel *model READ model WRITE setModel NOTIFY modelChanged
               BINDABLE bindableModel)
    Q_PROPERTY(bool hasSelection READ hasSelection NOTIFY selectionChanged STORED false
               DESIGNABLE false)
    Q_PROPERTY(QModelIndex currentIndex READ currentIndex NOTIFY currentChanged STORED false
               DESIGNABLE false)
    Q_PROPERTY(QItemSelection selection READ selection NOTIFY selectionChanged STORED false
               DESIGNABLE false)
    Q_PROPERTY(QModelIndexList selectedIndexes READ selectedIndexes NOTIFY selectionChanged
               STORED false DESIGNABLE false)

    Q_DECLARE_PRIVATE(QItemSelectionModel)

public:

    enum SelectionFlag {
        NoUpdate       = 0x0000,
        Clear          = 0x0001,
        Select         = 0x0002,
        Deselect       = 0x0004,
        Toggle         = 0x0008,
        Current        = 0x0010,
        Rows           = 0x0020,
        Columns        = 0x0040,
        SelectCurrent  = Select | Current,
        ToggleCurrent  = Toggle | Current,
        ClearAndSelect = Clear | Select
    };

    Q_DECLARE_FLAGS(SelectionFlags, SelectionFlag)
    Q_FLAG(SelectionFlags)

    explicit QItemSelectionModel(QAbstractItemModel *model = nullptr);
    explicit QItemSelectionModel(QAbstractItemModel *model, QObject *parent);
    virtual ~QItemSelectionModel();

    QModelIndex currentIndex() const;

    Q_INVOKABLE bool isSelected(const QModelIndex &index) const;
    Q_INVOKABLE bool isRowSelected(int row, const QModelIndex &parent = QModelIndex()) const;
    Q_INVOKABLE bool isColumnSelected(int column, const QModelIndex &parent = QModelIndex()) const;

    Q_INVOKABLE bool rowIntersectsSelection(int row, const QModelIndex &parent = QModelIndex()) const;
    Q_INVOKABLE bool columnIntersectsSelection(int column, const QModelIndex &parent = QModelIndex()) const;

    bool hasSelection() const;

    QModelIndexList selectedIndexes() const;
    Q_INVOKABLE QModelIndexList selectedRows(int column = 0) const;
    Q_INVOKABLE QModelIndexList selectedColumns(int row = 0) const;
    const QItemSelection selection() const;

    const QAbstractItemModel *model() const;
    QAbstractItemModel *model();
    QBindable<QAbstractItemModel *> bindableModel();

    void setModel(QAbstractItemModel *model);

public Q_SLOTS:
    virtual void setCurrentIndex(const QModelIndex &index, QItemSelectionModel::SelectionFlags command);
    virtual void select(const QModelIndex &index, QItemSelectionModel::SelectionFlags command);
    virtual void select(const QItemSelection &selection, QItemSelectionModel::SelectionFlags command);
    virtual void clear();
    virtual void reset();

    void clearSelection();
    virtual void clearCurrentIndex();

Q_SIGNALS:
    void selectionChanged(const QItemSelection &selected, const QItemSelection &deselected);
    void currentChanged(const QModelIndex &current, const QModelIndex &previous);
    void currentRowChanged(const QModelIndex &current, const QModelIndex &previous);
    void currentColumnChanged(const QModelIndex &current, const QModelIndex &previous);
    void modelChanged(QAbstractItemModel *model);

protected:
    QItemSelectionModel(QItemSelectionModelPrivate &dd, QAbstractItemModel *model);
    void emitSelectionChanged(const QItemSelection &newSelection, const QItemSelection &oldSelection);

private:
    Q_DISABLE_COPY(QItemSelectionModel)
    Q_PRIVATE_SLOT(d_func(), void _q_columnsAboutToBeRemoved(const QModelIndex&, int, int))
    Q_PRIVATE_SLOT(d_func(), void _q_rowsAboutToBeRemoved(const QModelIndex&, int, int))
    Q_PRIVATE_SLOT(d_func(), void _q_columnsAboutToBeInserted(const QModelIndex&, int, int))
    Q_PRIVATE_SLOT(d_func(), void _q_rowsAboutToBeInserted(const QModelIndex&, int, int))
    Q_PRIVATE_SLOT(d_func(), void _q_layoutAboutToBeChanged(const QList<QPersistentModelIndex> &parents = QList<QPersistentModelIndex>(), QAbstractItemModel::LayoutChangeHint hint = QAbstractItemModel::NoHint))
    Q_PRIVATE_SLOT(d_func(), void _q_layoutChanged(const QList<QPersistentModelIndex> &parents = QList<QPersistentModelIndex>(), QAbstractItemModel::LayoutChangeHint hint = QAbstractItemModel::NoHint))
    Q_PRIVATE_SLOT(d_func(), void _q_modelDestroyed())
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QItemSelectionModel::SelectionFlags)

// We export each out-of-line method individually to prevent MSVC from
// exporting the whole QList class.
class QItemSelection : public QList<QItemSelectionRange>
{
public:
    using QList<QItemSelectionRange>::QList;
    Q_CORE_EXPORT QItemSelection(const QModelIndex &topLeft, const QModelIndex &bottomRight);

    // reusing QList::swap() here is OK!

    Q_CORE_EXPORT void select(const QModelIndex &topLeft, const QModelIndex &bottomRight);
    Q_CORE_EXPORT bool contains(const QModelIndex &index) const;
    Q_CORE_EXPORT QModelIndexList indexes() const;
    Q_CORE_EXPORT void merge(const QItemSelection &other, QItemSelectionModel::SelectionFlags command);
    Q_CORE_EXPORT static void split(const QItemSelectionRange &range,
                      const QItemSelectionRange &other,
                      QItemSelection *result);
};
Q_DECLARE_SHARED(QItemSelection)

#ifndef QT_NO_DEBUG_STREAM
Q_CORE_EXPORT QDebug operator<<(QDebug, const QItemSelectionRange &);
#endif

QT_END_NAMESPACE

QT_DECL_METATYPE_EXTERN(QItemSelectionRange, Q_CORE_EXPORT)
QT_DECL_METATYPE_EXTERN(QItemSelection, Q_CORE_EXPORT)

#endif // QITEMSELECTIONMODEL_H
