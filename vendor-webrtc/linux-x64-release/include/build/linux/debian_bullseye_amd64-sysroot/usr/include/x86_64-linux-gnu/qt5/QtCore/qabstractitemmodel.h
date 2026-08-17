/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
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

#ifndef QABSTRACTITEMMODEL_H
#define QABSTRACTITEMMODEL_H

#include <QtCore/qvariant.h>
#include <QtCore/qobject.h>
#include <QtCore/qhash.h>
#include <QtCore/qvector.h>

QT_REQUIRE_CONFIG(itemmodel);

QT_BEGIN_NAMESPACE


class QAbstractItemModel;
class QPersistentModelIndex;

class Q_CORE_EXPORT QModelIndex
{
    friend class QAbstractItemModel;
public:
    Q_DECL_CONSTEXPR inline QModelIndex() noexcept : r(-1), c(-1), i(0), m(nullptr) {}
    // compiler-generated copy/move ctors/assignment operators are fine!
    Q_DECL_CONSTEXPR inline int row() const noexcept { return r; }
    Q_DECL_CONSTEXPR inline int column() const noexcept { return c; }
    Q_DECL_CONSTEXPR inline quintptr internalId() const noexcept { return i; }
    inline void *internalPointer() const noexcept { return reinterpret_cast<void*>(i); }
    inline QModelIndex parent() const;
    inline QModelIndex sibling(int row, int column) const;
    inline QModelIndex siblingAtColumn(int column) const;
    inline QModelIndex siblingAtRow(int row) const;
#if QT_DEPRECATED_SINCE(5, 8)
    QT_DEPRECATED_X("Use QAbstractItemModel::index") inline QModelIndex child(int row, int column) const;
#endif
    inline QVariant data(int role = Qt::DisplayRole) const;
    inline Qt::ItemFlags flags() const;
    Q_DECL_CONSTEXPR inline const QAbstractItemModel *model() const noexcept { return m; }
    Q_DECL_CONSTEXPR inline bool isValid() const noexcept { return (r >= 0) && (c >= 0) && (m != nullptr); }
    Q_DECL_CONSTEXPR inline bool operator==(const QModelIndex &other) const noexcept
        { return (other.r == r) && (other.i == i) && (other.c == c) && (other.m == m); }
    Q_DECL_CONSTEXPR inline bool operator!=(const QModelIndex &other) const noexcept
        { return !(*this == other); }
    Q_DECL_CONSTEXPR inline bool operator<(const QModelIndex &other) const noexcept
        {
            return  r <  other.r
                || (r == other.r && (c <  other.c
                                 || (c == other.c && (i <  other.i
                                                  || (i == other.i && std::less<const QAbstractItemModel *>()(m, other.m))))));
        }
private:
    inline QModelIndex(int arow, int acolumn, void *ptr, const QAbstractItemModel *amodel) noexcept
        : r(arow), c(acolumn), i(reinterpret_cast<quintptr>(ptr)), m(amodel) {}
    Q_DECL_CONSTEXPR inline QModelIndex(int arow, int acolumn, quintptr id, const QAbstractItemModel *amodel) noexcept
        : r(arow), c(acolumn), i(id), m(amodel) {}
    int r, c;
    quintptr i;
    const QAbstractItemModel *m;
};
Q_DECLARE_TYPEINFO(QModelIndex, Q_MOVABLE_TYPE);

#ifndef QT_NO_DEBUG_STREAM
Q_CORE_EXPORT QDebug operator<<(QDebug, const QModelIndex &);
#endif

class QPersistentModelIndexData;

// qHash is a friend, but we can't use default arguments for friends (§8.3.6.4)
uint qHash(const QPersistentModelIndex &index, uint seed = 0) noexcept;

class Q_CORE_EXPORT QPersistentModelIndex
{
public:
    QPersistentModelIndex();
    QPersistentModelIndex(const QModelIndex &index);
    QPersistentModelIndex(const QPersistentModelIndex &other);
    ~QPersistentModelIndex();
    bool operator<(const QPersistentModelIndex &other) const;
    bool operator==(const QPersistentModelIndex &other) const;
    inline bool operator!=(const QPersistentModelIndex &other) const
    { return !operator==(other); }
    QPersistentModelIndex &operator=(const QPersistentModelIndex &other);
    inline QPersistentModelIndex(QPersistentModelIndex &&other) noexcept
        : d(other.d) { other.d = nullptr; }
    inline QPersistentModelIndex &operator=(QPersistentModelIndex &&other) noexcept
    { qSwap(d, other.d); return *this; }
    inline void swap(QPersistentModelIndex &other) noexcept { qSwap(d, other.d); }
    bool operator==(const QModelIndex &other) const;
    bool operator!=(const QModelIndex &other) const;
    QPersistentModelIndex &operator=(const QModelIndex &other);
    operator const QModelIndex&() const;
    int row() const;
    int column() const;
    void *internalPointer() const;
    quintptr internalId() const;
    QModelIndex parent() const;
    QModelIndex sibling(int row, int column) const;
#if QT_DEPRECATED_SINCE(5, 8)
    QT_DEPRECATED_X("Use QAbstractItemModel::index") QModelIndex child(int row, int column) const;
#endif
    QVariant data(int role = Qt::DisplayRole) const;
    Qt::ItemFlags flags() const;
    const QAbstractItemModel *model() const;
    bool isValid() const;
private:
    QPersistentModelIndexData *d;
    friend uint qHash(const QPersistentModelIndex &, uint seed) noexcept;
#ifndef QT_NO_DEBUG_STREAM
    friend Q_CORE_EXPORT QDebug operator<<(QDebug, const QPersistentModelIndex &);
#endif
};
Q_DECLARE_SHARED(QPersistentModelIndex)

inline uint qHash(const QPersistentModelIndex &index, uint seed) noexcept
{ return qHash(index.d, seed); }


#ifndef QT_NO_DEBUG_STREAM
Q_CORE_EXPORT QDebug operator<<(QDebug, const QPersistentModelIndex &);
#endif

template<typename T> class QList;
typedef QList<QModelIndex> QModelIndexList;

class QMimeData;
class QAbstractItemModelPrivate;
class QTransposeProxyModelPrivate;
template <class Key, class T> class QMap;


class Q_CORE_EXPORT QAbstractItemModel : public QObject
{
    Q_OBJECT

    friend class QPersistentModelIndexData;
    friend class QAbstractItemViewPrivate;
    friend class QIdentityProxyModel;
    friend class QTransposeProxyModelPrivate;
public:

    explicit QAbstractItemModel(QObject *parent = nullptr);
    virtual ~QAbstractItemModel();

    Q_INVOKABLE bool hasIndex(int row, int column, const QModelIndex &parent = QModelIndex()) const;
    Q_INVOKABLE virtual QModelIndex index(int row, int column,
                              const QModelIndex &parent = QModelIndex()) const = 0;
    Q_INVOKABLE virtual QModelIndex parent(const QModelIndex &child) const = 0;

    Q_INVOKABLE virtual QModelIndex sibling(int row, int column, const QModelIndex &idx) const;
    Q_INVOKABLE virtual int rowCount(const QModelIndex &parent = QModelIndex()) const = 0;
    Q_INVOKABLE virtual int columnCount(const QModelIndex &parent = QModelIndex()) const = 0;
    Q_INVOKABLE virtual bool hasChildren(const QModelIndex &parent = QModelIndex()) const;

    Q_INVOKABLE virtual QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const = 0;
    Q_INVOKABLE virtual bool setData(const QModelIndex &index, const QVariant &value, int role = Qt::EditRole);

    Q_INVOKABLE virtual QVariant headerData(int section, Qt::Orientation orientation,
                                int role = Qt::DisplayRole) const;
    virtual bool setHeaderData(int section, Qt::Orientation orientation, const QVariant &value,
                               int role = Qt::EditRole);

    virtual QMap<int, QVariant> itemData(const QModelIndex &index) const;
    virtual bool setItemData(const QModelIndex &index, const QMap<int, QVariant> &roles);
#if QT_VERSION >= QT_VERSION_CHECK(6, 0, 0)
    virtual bool clearItemData(const QModelIndex &index);
#endif

    virtual QStringList mimeTypes() const;
    virtual QMimeData *mimeData(const QModelIndexList &indexes) const;
    virtual bool canDropMimeData(const QMimeData *data, Qt::DropAction action,
                                 int row, int column, const QModelIndex &parent) const;
    virtual bool dropMimeData(const QMimeData *data, Qt::DropAction action,
                              int row, int column, const QModelIndex &parent);
    virtual Qt::DropActions supportedDropActions() const;

    virtual Qt::DropActions supportedDragActions() const;
#if QT_DEPRECATED_SINCE(5, 0)
    QT_DEPRECATED void setSupportedDragActions(Qt::DropActions actions)
    { doSetSupportedDragActions(actions); }
#endif

    virtual bool insertRows(int row, int count, const QModelIndex &parent = QModelIndex());
    virtual bool insertColumns(int column, int count, const QModelIndex &parent = QModelIndex());
    virtual bool removeRows(int row, int count, const QModelIndex &parent = QModelIndex());
    virtual bool removeColumns(int column, int count, const QModelIndex &parent = QModelIndex());
    virtual bool moveRows(const QModelIndex &sourceParent, int sourceRow, int count,
                          const QModelIndex &destinationParent, int destinationChild);
    virtual bool moveColumns(const QModelIndex &sourceParent, int sourceColumn, int count,
                             const QModelIndex &destinationParent, int destinationChild);

    inline bool insertRow(int row, const QModelIndex &parent = QModelIndex());
    inline bool insertColumn(int column, const QModelIndex &parent = QModelIndex());
    inline bool removeRow(int row, const QModelIndex &parent = QModelIndex());
    inline bool removeColumn(int column, const QModelIndex &parent = QModelIndex());
    inline bool moveRow(const QModelIndex &sourceParent, int sourceRow,
                        const QModelIndex &destinationParent, int destinationChild);
    inline bool moveColumn(const QModelIndex &sourceParent, int sourceColumn,
                           const QModelIndex &destinationParent, int destinationChild);

    Q_INVOKABLE virtual void fetchMore(const QModelIndex &parent);
    Q_INVOKABLE virtual bool canFetchMore(const QModelIndex &parent) const;
    Q_INVOKABLE virtual Qt::ItemFlags flags(const QModelIndex &index) const;
    virtual void sort(int column, Qt::SortOrder order = Qt::AscendingOrder);
    virtual QModelIndex buddy(const QModelIndex &index) const;
    Q_INVOKABLE virtual QModelIndexList match(const QModelIndex &start, int role,
                                              const QVariant &value, int hits = 1,
                                              Qt::MatchFlags flags =
                                              Qt::MatchFlags(Qt::MatchStartsWith|Qt::MatchWrap)) const;
    virtual QSize span(const QModelIndex &index) const;

    virtual QHash<int,QByteArray> roleNames() const;

    using QObject::parent;

    enum LayoutChangeHint
    {
        NoLayoutChangeHint,
        VerticalSortHint,
        HorizontalSortHint
    };
    Q_ENUM(LayoutChangeHint)

    enum class CheckIndexOption {
        NoOption         = 0x0000,
        IndexIsValid     = 0x0001,
        DoNotUseParent   = 0x0002,
        ParentIsInvalid  = 0x0004,
    };
    Q_ENUM(CheckIndexOption)
    Q_DECLARE_FLAGS(CheckIndexOptions, CheckIndexOption)

    Q_REQUIRED_RESULT bool checkIndex(const QModelIndex &index, CheckIndexOptions options = CheckIndexOption::NoOption) const;

Q_SIGNALS:
    void dataChanged(const QModelIndex &topLeft, const QModelIndex &bottomRight, const QVector<int> &roles = QVector<int>());
    void headerDataChanged(Qt::Orientation orientation, int first, int last);
    void layoutChanged(const QList<QPersistentModelIndex> &parents = QList<QPersistentModelIndex>(), QAbstractItemModel::LayoutChangeHint hint = QAbstractItemModel::NoLayoutChangeHint);
    void layoutAboutToBeChanged(const QList<QPersistentModelIndex> &parents = QList<QPersistentModelIndex>(), QAbstractItemModel::LayoutChangeHint hint = QAbstractItemModel::NoLayoutChangeHint);

    void rowsAboutToBeInserted(const QModelIndex &parent, int first, int last, QPrivateSignal);
    void rowsInserted(const QModelIndex &parent, int first, int last, QPrivateSignal);

    void rowsAboutToBeRemoved(const QModelIndex &parent, int first, int last, QPrivateSignal);
    void rowsRemoved(const QModelIndex &parent, int first, int last, QPrivateSignal);

    void columnsAboutToBeInserted(const QModelIndex &parent, int first, int last, QPrivateSignal);
    void columnsInserted(const QModelIndex &parent, int first, int last, QPrivateSignal);

    void columnsAboutToBeRemoved(const QModelIndex &parent, int first, int last, QPrivateSignal);
    void columnsRemoved(const QModelIndex &parent, int first, int last, QPrivateSignal);

    void modelAboutToBeReset(QPrivateSignal);
    void modelReset(QPrivateSignal);

    void rowsAboutToBeMoved( const QModelIndex &sourceParent, int sourceStart, int sourceEnd, const QModelIndex &destinationParent, int destinationRow, QPrivateSignal);
    void rowsMoved( const QModelIndex &parent, int start, int end, const QModelIndex &destination, int row, QPrivateSignal);

    void columnsAboutToBeMoved( const QModelIndex &sourceParent, int sourceStart, int sourceEnd, const QModelIndex &destinationParent, int destinationColumn, QPrivateSignal);
    void columnsMoved( const QModelIndex &parent, int start, int end, const QModelIndex &destination, int column, QPrivateSignal);

public Q_SLOTS:
    virtual bool submit();
    virtual void revert();

protected Q_SLOTS:
#if QT_VERSION >= QT_VERSION_CHECK(6, 0, 0)
    virtual
#endif
    void resetInternalData();

protected:
    QAbstractItemModel(QAbstractItemModelPrivate &dd, QObject *parent = nullptr);

    inline QModelIndex createIndex(int row, int column, void *data = nullptr) const;
    inline QModelIndex createIndex(int row, int column, quintptr id) const;

    void encodeData(const QModelIndexList &indexes, QDataStream &stream) const;
    bool decodeData(int row, int column, const QModelIndex &parent, QDataStream &stream);

    void beginInsertRows(const QModelIndex &parent, int first, int last);
    void endInsertRows();

    void beginRemoveRows(const QModelIndex &parent, int first, int last);
    void endRemoveRows();

    bool beginMoveRows(const QModelIndex &sourceParent, int sourceFirst, int sourceLast, const QModelIndex &destinationParent, int destinationRow);
    void endMoveRows();

    void beginInsertColumns(const QModelIndex &parent, int first, int last);
    void endInsertColumns();

    void beginRemoveColumns(const QModelIndex &parent, int first, int last);
    void endRemoveColumns();

    bool beginMoveColumns(const QModelIndex &sourceParent, int sourceFirst, int sourceLast, const QModelIndex &destinationParent, int destinationColumn);
    void endMoveColumns();


#if QT_DEPRECATED_SINCE(5,0)
    QT_DEPRECATED void reset()
    {
        beginResetModel();
        endResetModel();
    }
#endif

    void beginResetModel();
    void endResetModel();

    void changePersistentIndex(const QModelIndex &from, const QModelIndex &to);
    void changePersistentIndexList(const QModelIndexList &from, const QModelIndexList &to);
    QModelIndexList persistentIndexList() const;

#if QT_DEPRECATED_SINCE(5,0)
    QT_DEPRECATED void setRoleNames(const QHash<int,QByteArray> &theRoleNames)
    {
        doSetRoleNames(theRoleNames);
    }
#endif

private:
    void doSetRoleNames(const QHash<int,QByteArray> &roleNames);
    void doSetSupportedDragActions(Qt::DropActions actions);

    Q_DECLARE_PRIVATE(QAbstractItemModel)
    Q_DISABLE_COPY(QAbstractItemModel)
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QAbstractItemModel::CheckIndexOptions)

inline bool QAbstractItemModel::insertRow(int arow, const QModelIndex &aparent)
{ return insertRows(arow, 1, aparent); }
inline bool QAbstractItemModel::insertColumn(int acolumn, const QModelIndex &aparent)
{ return insertColumns(acolumn, 1, aparent); }
inline bool QAbstractItemModel::removeRow(int arow, const QModelIndex &aparent)
{ return removeRows(arow, 1, aparent); }
inline bool QAbstractItemModel::removeColumn(int acolumn, const QModelIndex &aparent)
{ return removeColumns(acolumn, 1, aparent); }
inline bool QAbstractItemModel::moveRow(const QModelIndex &sourceParent, int sourceRow,
                                        const QModelIndex &destinationParent, int destinationChild)
{ return moveRows(sourceParent, sourceRow, 1, destinationParent, destinationChild); }
inline bool QAbstractItemModel::moveColumn(const QModelIndex &sourceParent, int sourceColumn,
                                           const QModelIndex &destinationParent, int destinationChild)
{ return moveColumns(sourceParent, sourceColumn, 1, destinationParent, destinationChild); }
inline QModelIndex QAbstractItemModel::createIndex(int arow, int acolumn, void *adata) const
{ return QModelIndex(arow, acolumn, adata, this); }
inline QModelIndex QAbstractItemModel::createIndex(int arow, int acolumn, quintptr aid) const
{ return QModelIndex(arow, acolumn, aid, this); }

class Q_CORE_EXPORT QAbstractTableModel : public QAbstractItemModel
{
    Q_OBJECT

public:
    explicit QAbstractTableModel(QObject *parent = nullptr);
    ~QAbstractTableModel();

    QModelIndex index(int row, int column, const QModelIndex &parent = QModelIndex()) const override;
    QModelIndex sibling(int row, int column, const QModelIndex &idx) const override;
    bool dropMimeData(const QMimeData *data, Qt::DropAction action,
                      int row, int column, const QModelIndex &parent) override;

    Qt::ItemFlags flags(const QModelIndex &index) const override;

    using QObject::parent;

protected:
    QAbstractTableModel(QAbstractItemModelPrivate &dd, QObject *parent);

private:
    Q_DISABLE_COPY(QAbstractTableModel)
    QModelIndex parent(const QModelIndex &child) const override;
    bool hasChildren(const QModelIndex &parent) const override;
};

class Q_CORE_EXPORT QAbstractListModel : public QAbstractItemModel
{
    Q_OBJECT

public:
    explicit QAbstractListModel(QObject *parent = nullptr);
    ~QAbstractListModel();

    QModelIndex index(int row, int column = 0, const QModelIndex &parent = QModelIndex()) const override;
    QModelIndex sibling(int row, int column, const QModelIndex &idx) const override;
    bool dropMimeData(const QMimeData *data, Qt::DropAction action,
                      int row, int column, const QModelIndex &parent) override;

    Qt::ItemFlags flags(const QModelIndex &index) const override;

    using QObject::parent;

protected:
    QAbstractListModel(QAbstractItemModelPrivate &dd, QObject *parent);

private:
    Q_DISABLE_COPY(QAbstractListModel)
    QModelIndex parent(const QModelIndex &child) const override;
    int columnCount(const QModelIndex &parent) const override;
    bool hasChildren(const QModelIndex &parent) const override;
};

// inline implementations

inline QModelIndex QModelIndex::parent() const
{ return m ? m->parent(*this) : QModelIndex(); }

inline QModelIndex QModelIndex::sibling(int arow, int acolumn) const
{ return m ? (r == arow && c == acolumn) ? *this : m->sibling(arow, acolumn, *this) : QModelIndex(); }

inline QModelIndex QModelIndex::siblingAtColumn(int acolumn) const
{ return m ? (c == acolumn) ? *this : m->sibling(r, acolumn, *this) : QModelIndex(); }

inline QModelIndex QModelIndex::siblingAtRow(int arow) const
{ return m ? (r == arow) ? *this : m->sibling(arow, c, *this) : QModelIndex(); }

#if QT_DEPRECATED_SINCE(5, 8)
inline QModelIndex QModelIndex::child(int arow, int acolumn) const
{ return m ? m->index(arow, acolumn, *this) : QModelIndex(); }
#endif

inline QVariant QModelIndex::data(int arole) const
{ return m ? m->data(*this, arole) : QVariant(); }

inline Qt::ItemFlags QModelIndex::flags() const
{ return m ? m->flags(*this) : Qt::ItemFlags(); }

inline uint qHash(const QModelIndex &index) noexcept
{ return uint((uint(index.row()) << 4) + index.column() + index.internalId()); }

QT_END_NAMESPACE

Q_DECLARE_METATYPE(QModelIndexList)

#endif // QABSTRACTITEMMODEL_H
