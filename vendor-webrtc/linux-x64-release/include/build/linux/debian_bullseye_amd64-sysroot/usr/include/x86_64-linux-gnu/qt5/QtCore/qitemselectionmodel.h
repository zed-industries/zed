/****************************************************************************
**
** Copyright (C) 2020 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtCore module of the Qt Toolkit.
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

#ifndef QITEMSELECTIONMODEL_H
#define QITEMSELECTIONMODEL_H

#include <QtCore/qglobal.h>

#include <QtCore/qset.h>
#include <QtCore/qvector.h>
#include <QtCore/qlist.h>
#include <QtCore/qabstractitemmodel.h>

QT_REQUIRE_CONFIG(itemmodel);

QT_BEGIN_NAMESPACE

class Q_CORE_EXPORT QItemSelectionRange
{

public:
    inline QItemSelectionRange() : tl(), br() {}
#if QT_VERSION < QT_VERSION_CHECK(6,0,0)
    // ### Qt 6: remove them all, the compiler-generated ones are fine
    inline QItemSelectionRange(const QItemSelectionRange &other)
        : tl(other.tl), br(other.br) {}
    QItemSelectionRange(QItemSelectionRange &&other) noexcept
        : tl(std::move(other.tl)), br(std::move(other.br)) {}
    QItemSelectionRange &operator=(QItemSelectionRange &&other) noexcept
    { tl = std::move(other.tl); br = std::move(other.br); return *this; }
    QItemSelectionRange &operator=(const QItemSelectionRange &other)
    { tl = other.tl; br = other.br; return *this; }
#endif // Qt < 6
    QItemSelectionRange(const QModelIndex &topL, const QModelIndex &bottomR) : tl(topL), br(bottomR) {}
    explicit QItemSelectionRange(const QModelIndex &index) : tl(index), br(tl) {}

    void swap(QItemSelectionRange &other) noexcept
    {
        qSwap(tl, other.tl);
        qSwap(br, other.br);
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
#if QT_DEPRECATED_SINCE(5, 0)
    inline QItemSelectionRange intersect(const QItemSelectionRange &other) const
        { return intersected(other); }
#endif
    QItemSelectionRange intersected(const QItemSelectionRange &other) const;


    inline bool operator==(const QItemSelectionRange &other) const
        { return (tl == other.tl && br == other.br); }
    inline bool operator!=(const QItemSelectionRange &other) const
        { return !operator==(other); }
#if QT_DEPRECATED_SINCE(5, 15)
    QT_DEPRECATED bool operator<(const QItemSelectionRange &other) const;
#endif

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
Q_DECLARE_TYPEINFO(QItemSelectionRange, Q_MOVABLE_TYPE);

class QItemSelection;
class QItemSelectionModelPrivate;

class Q_CORE_EXPORT QItemSelectionModel : public QObject
{
    Q_OBJECT
    Q_PROPERTY(QAbstractItemModel *model READ model WRITE setModel NOTIFY modelChanged)
    Q_PROPERTY(bool hasSelection READ hasSelection NOTIFY selectionChanged STORED false DESIGNABLE false)
    Q_PROPERTY(QModelIndex currentIndex READ currentIndex NOTIFY currentChanged STORED false DESIGNABLE false)
    Q_PROPERTY(QItemSelection selection READ selection NOTIFY selectionChanged STORED false DESIGNABLE false)
    Q_PROPERTY(QModelIndexList selectedIndexes READ selectedIndexes NOTIFY selectionChanged STORED false DESIGNABLE false)

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

    // ### Qt 6: Merge these two as "QAbstractItemModel *model() const"
    const QAbstractItemModel *model() const;
    QAbstractItemModel *model();

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
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QItemSelectionModel::SelectionFlags)

// dummy implentation of qHash() necessary for instantiating QList<QItemSelectionRange>::toSet() with MSVC
inline uint qHash(const QItemSelectionRange &) { return 0; }

#ifdef Q_CC_MSVC

/*
   ### Qt 6:
   ### This needs to be removed for next releases of Qt. It is a workaround for vc++ because
   ### Qt exports QItemSelection that inherits QList<QItemSelectionRange>.
*/

# ifndef Q_TEMPLATE_EXTERN
#  if defined(QT_BUILD_CORE_LIB)
#   define Q_TEMPLATE_EXTERN
#  else
#   define Q_TEMPLATE_EXTERN extern
#  endif
# endif
Q_TEMPLATE_EXTERN template class Q_CORE_EXPORT QList<QItemSelectionRange>;
#endif // Q_CC_MSVC

class Q_CORE_EXPORT QItemSelection : public QList<QItemSelectionRange>
{
public:
    QItemSelection() noexcept : QList<QItemSelectionRange>() {}
    QItemSelection(const QModelIndex &topLeft, const QModelIndex &bottomRight);

    // reusing QList::swap() here is OK!

    void select(const QModelIndex &topLeft, const QModelIndex &bottomRight);
    bool contains(const QModelIndex &index) const;
    QModelIndexList indexes() const;
    void merge(const QItemSelection &other, QItemSelectionModel::SelectionFlags command);
    static void split(const QItemSelectionRange &range,
                      const QItemSelectionRange &other,
                      QItemSelection *result);
};
Q_DECLARE_SHARED_NOT_MOVABLE_UNTIL_QT6(QItemSelection)

#ifndef QT_NO_DEBUG_STREAM
Q_CORE_EXPORT QDebug operator<<(QDebug, const QItemSelectionRange &);
#endif

QT_END_NAMESPACE

Q_DECLARE_METATYPE(QItemSelectionRange)
Q_DECLARE_METATYPE(QItemSelection)

#endif // QITEMSELECTIONMODEL_H
