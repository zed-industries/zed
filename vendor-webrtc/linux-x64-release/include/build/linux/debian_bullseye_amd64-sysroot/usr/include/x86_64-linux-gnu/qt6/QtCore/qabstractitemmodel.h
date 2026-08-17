// Copyright (C) 2016 The Qt Company Ltd.
// Copyright (C) 2020 Klarälvdalens Datakonsult AB, a KDAB Group company, info@kdab.com, author Giuseppe D'Angelo <giuseppe.dangelo@kdab.com>
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QABSTRACTITEMMODEL_H
#define QABSTRACTITEMMODEL_H

#include <QtCore/qhash.h>
#include <QtCore/qlist.h>
#include <QtCore/qobject.h>
#include <QtCore/qvariant.h>

QT_REQUIRE_CONFIG(itemmodel);

QT_BEGIN_NAMESPACE

class QModelRoleData
{
    int m_role;
    QVariant m_data;

public:
    explicit QModelRoleData(int role) noexcept
        : m_role(role)
    {}

    constexpr int role() const noexcept { return m_role; }
    constexpr QVariant &data() noexcept { return m_data; }
    constexpr const QVariant &data() const noexcept { return m_data; }

    template <typename T>
    constexpr void setData(T &&value) noexcept(noexcept(m_data.setValue(std::forward<T>(value))))
    { m_data.setValue(std::forward<T>(value)); }

    void clearData() noexcept { m_data.clear(); }
};

Q_DECLARE_TYPEINFO(QModelRoleData, Q_RELOCATABLE_TYPE);

class QModelRoleDataSpan;

namespace QtPrivate {
template <typename T, typename Enable = void>
struct IsContainerCompatibleWithModelRoleDataSpan : std::false_type {};

template <typename T>
struct IsContainerCompatibleWithModelRoleDataSpan<T, std::enable_if_t<std::conjunction_v<
            // lacking concepts and ranges, we accept any T whose std::data yields a suitable pointer ...
            std::is_convertible<decltype( std::data(std::declval<T &>()) ), QModelRoleData *>,
            // ... and that has a suitable size ...
            std::is_convertible<decltype( std::size(std::declval<T &>()) ), qsizetype>,
            // ... and it's a range as it defines an iterator-like API
            std::is_convertible<
                typename std::iterator_traits<decltype( std::begin(std::declval<T &>()) )>::value_type,
                QModelRoleData
            >,
            std::is_convertible<
                decltype( std::begin(std::declval<T &>()) != std::end(std::declval<T &>()) ),
                bool>,
            // Don't make an accidental copy constructor
            std::negation<std::is_same<std::decay_t<T>, QModelRoleDataSpan>>
        >>> : std::true_type {};
} // namespace QtPrivate

class QModelRoleDataSpan
{
    QModelRoleData *m_modelRoleData = nullptr;
    qsizetype m_len = 0;

    template <typename T>
    using if_compatible_container = std::enable_if_t<QtPrivate::IsContainerCompatibleWithModelRoleDataSpan<T>::value, bool>;

public:
    constexpr QModelRoleDataSpan() noexcept {}

    constexpr QModelRoleDataSpan(QModelRoleData &modelRoleData) noexcept
        : m_modelRoleData(&modelRoleData),
          m_len(1)
    {}

    constexpr QModelRoleDataSpan(QModelRoleData *modelRoleData, qsizetype len)
        : m_modelRoleData(modelRoleData),
          m_len(len)
    {}

    template <typename Container, if_compatible_container<Container> = true>
    constexpr QModelRoleDataSpan(Container &c) noexcept(noexcept(std::data(c)) && noexcept(std::size(c)))
        : m_modelRoleData(std::data(c)),
          m_len(qsizetype(std::size(c)))
    {}

    constexpr qsizetype size() const noexcept { return m_len; }
    constexpr qsizetype length() const noexcept { return m_len; }
    constexpr QModelRoleData *data() const noexcept { return m_modelRoleData; }
    constexpr QModelRoleData *begin() const noexcept { return m_modelRoleData; }
    constexpr QModelRoleData *end() const noexcept { return m_modelRoleData + m_len; }
    constexpr QModelRoleData &operator[](qsizetype index) const { return m_modelRoleData[index]; }

    constexpr QVariant *dataForRole(int role) const
    {
#ifdef __cpp_lib_constexpr_algorithms
        auto result = std::find_if(begin(), end(), [role](const QModelRoleData &roleData) {
            return roleData.role() == role;
        });
#else
        auto result = begin();
        const auto e = end();
        for (; result != e; ++result) {
            if (result->role() == role)
                break;
        }
#endif

        return Q_ASSERT(result != end()), &result->data();
    }
};

Q_DECLARE_TYPEINFO(QModelRoleDataSpan, Q_RELOCATABLE_TYPE);

class QAbstractItemModel;
class QPersistentModelIndex;

class QModelIndex
{
    friend class QAbstractItemModel;
public:
    constexpr inline QModelIndex() noexcept : r(-1), c(-1), i(0), m(nullptr) {}
    // compiler-generated copy/move ctors/assignment operators are fine!
    constexpr inline int row() const noexcept { return r; }
    constexpr inline int column() const noexcept { return c; }
    constexpr inline quintptr internalId() const noexcept { return i; }
    inline void *internalPointer() const noexcept { return reinterpret_cast<void*>(i); }
    inline const void *constInternalPointer() const noexcept { return reinterpret_cast<const void *>(i); }
    inline QModelIndex parent() const;
    inline QModelIndex sibling(int row, int column) const;
    inline QModelIndex siblingAtColumn(int column) const;
    inline QModelIndex siblingAtRow(int row) const;
    inline QVariant data(int role = Qt::DisplayRole) const;
    inline void multiData(QModelRoleDataSpan roleDataSpan) const;
    inline Qt::ItemFlags flags() const;
    constexpr inline const QAbstractItemModel *model() const noexcept { return m; }
    constexpr inline bool isValid() const noexcept { return (r >= 0) && (c >= 0) && (m != nullptr); }
    constexpr inline bool operator==(const QModelIndex &other) const noexcept
        { return (other.r == r) && (other.i == i) && (other.c == c) && (other.m == m); }
    constexpr inline bool operator!=(const QModelIndex &other) const noexcept
        { return !(*this == other); }
    constexpr inline bool operator<(const QModelIndex &other) const noexcept
        {
            return  r <  other.r
                || (r == other.r && (c <  other.c
                                 || (c == other.c && (i <  other.i
                                                  || (i == other.i && std::less<const QAbstractItemModel *>()(m, other.m))))));
        }
private:
    inline QModelIndex(int arow, int acolumn, const void *ptr, const QAbstractItemModel *amodel) noexcept
        : r(arow), c(acolumn), i(reinterpret_cast<quintptr>(ptr)), m(amodel) {}
    constexpr inline QModelIndex(int arow, int acolumn, quintptr id, const QAbstractItemModel *amodel) noexcept
        : r(arow), c(acolumn), i(id), m(amodel) {}
    int r, c;
    quintptr i;
    const QAbstractItemModel *m;
};
Q_DECLARE_TYPEINFO(QModelIndex, Q_RELOCATABLE_TYPE);

#ifndef QT_NO_DEBUG_STREAM
Q_CORE_EXPORT QDebug operator<<(QDebug, const QModelIndex &);
#endif

class QPersistentModelIndexData;

// qHash is a friend, but we can't use default arguments for friends (§8.3.6.4)
size_t qHash(const QPersistentModelIndex &index, size_t seed = 0) noexcept;

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
        : d(qExchange(other.d, nullptr)) {}
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QPersistentModelIndex)
    void swap(QPersistentModelIndex &other) noexcept { qt_ptr_swap(d, other.d); }
    bool operator==(const QModelIndex &other) const;
    bool operator!=(const QModelIndex &other) const;
    QPersistentModelIndex &operator=(const QModelIndex &other);
    operator QModelIndex() const;
    int row() const;
    int column() const;
    void *internalPointer() const;
    const void *constInternalPointer() const;
    quintptr internalId() const;
    QModelIndex parent() const;
    QModelIndex sibling(int row, int column) const;
    QVariant data(int role = Qt::DisplayRole) const;
    void multiData(QModelRoleDataSpan roleDataSpan) const;
    Qt::ItemFlags flags() const;
    const QAbstractItemModel *model() const;
    bool isValid() const;
private:
    QPersistentModelIndexData *d;
    friend size_t qHash(const QPersistentModelIndex &, size_t seed) noexcept;
    friend bool qHashEquals(const QPersistentModelIndex &a, const QPersistentModelIndex &b) noexcept
    { return a.d == b.d; }
#ifndef QT_NO_DEBUG_STREAM
    friend Q_CORE_EXPORT QDebug operator<<(QDebug, const QPersistentModelIndex &);
#endif
};
Q_DECLARE_SHARED(QPersistentModelIndex)

inline size_t qHash(const QPersistentModelIndex &index, size_t seed) noexcept
{ return qHash(index.d, seed); }


#ifndef QT_NO_DEBUG_STREAM
Q_CORE_EXPORT QDebug operator<<(QDebug, const QPersistentModelIndex &);
#endif

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
    friend class QAbstractProxyModel;
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
    virtual bool clearItemData(const QModelIndex &index);

    virtual QStringList mimeTypes() const;
    virtual QMimeData *mimeData(const QModelIndexList &indexes) const;
    virtual bool canDropMimeData(const QMimeData *data, Qt::DropAction action,
                                 int row, int column, const QModelIndex &parent) const;
    virtual bool dropMimeData(const QMimeData *data, Qt::DropAction action,
                              int row, int column, const QModelIndex &parent);
    virtual Qt::DropActions supportedDropActions() const;
    virtual Qt::DropActions supportedDragActions() const;

    Q_INVOKABLE Q_REVISION(6, 4) virtual bool insertRows(int row, int count, const QModelIndex &parent = QModelIndex());
    Q_INVOKABLE Q_REVISION(6, 4) virtual bool insertColumns(int column, int count, const QModelIndex &parent = QModelIndex());
    Q_INVOKABLE Q_REVISION(6, 4) virtual bool removeRows(int row, int count, const QModelIndex &parent = QModelIndex());
    Q_INVOKABLE Q_REVISION(6, 4) virtual bool removeColumns(int column, int count, const QModelIndex &parent = QModelIndex());
    Q_INVOKABLE Q_REVISION(6, 4) virtual bool moveRows(const QModelIndex &sourceParent, int sourceRow, int count,
                          const QModelIndex &destinationParent, int destinationChild);
    Q_INVOKABLE Q_REVISION(6, 4) virtual bool moveColumns(const QModelIndex &sourceParent, int sourceColumn, int count,
                             const QModelIndex &destinationParent, int destinationChild);

    Q_INVOKABLE Q_REVISION(6, 4) inline bool insertRow(int row, const QModelIndex &parent = QModelIndex());
    Q_INVOKABLE Q_REVISION(6, 4) inline bool insertColumn(int column, const QModelIndex &parent = QModelIndex());
    Q_INVOKABLE Q_REVISION(6, 4) inline bool removeRow(int row, const QModelIndex &parent = QModelIndex());
    Q_INVOKABLE Q_REVISION(6, 4) inline bool removeColumn(int column, const QModelIndex &parent = QModelIndex());
    Q_INVOKABLE Q_REVISION(6, 4) inline bool moveRow(const QModelIndex &sourceParent, int sourceRow,
                        const QModelIndex &destinationParent, int destinationChild);
    Q_INVOKABLE Q_REVISION(6, 4) inline bool moveColumn(const QModelIndex &sourceParent, int sourceColumn,
                           const QModelIndex &destinationParent, int destinationChild);

    Q_INVOKABLE virtual void fetchMore(const QModelIndex &parent);
    Q_INVOKABLE virtual bool canFetchMore(const QModelIndex &parent) const;
    Q_INVOKABLE virtual Qt::ItemFlags flags(const QModelIndex &index) const;
    Q_INVOKABLE Q_REVISION(6, 4) virtual void sort(int column, Qt::SortOrder order = Qt::AscendingOrder);
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

    [[nodiscard]] bool checkIndex(const QModelIndex &index, CheckIndexOptions options = CheckIndexOption::NoOption) const;

    virtual void multiData(const QModelIndex &index, QModelRoleDataSpan roleDataSpan) const;

Q_SIGNALS:
    void dataChanged(const QModelIndex &topLeft, const QModelIndex &bottomRight,
                     const QList<int> &roles = QList<int>());
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
    void rowsMoved( const QModelIndex &sourceParent, int sourceStart, int sourceEnd, const QModelIndex &destinationParent, int destinationRow, QPrivateSignal);

    void columnsAboutToBeMoved( const QModelIndex &sourceParent, int sourceStart, int sourceEnd, const QModelIndex &destinationParent, int destinationColumn, QPrivateSignal);
    void columnsMoved( const QModelIndex &sourceParent, int sourceStart, int sourceEnd, const QModelIndex &destinationParent, int destinationColumn, QPrivateSignal);

public Q_SLOTS:
    virtual bool submit();
    virtual void revert();

protected Q_SLOTS:
    virtual void resetInternalData();

protected:
    QAbstractItemModel(QAbstractItemModelPrivate &dd, QObject *parent = nullptr);

    inline QModelIndex createIndex(int row, int column, const void *data = nullptr) const;
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

    void beginResetModel();
    void endResetModel();

    void changePersistentIndex(const QModelIndex &from, const QModelIndex &to);
    void changePersistentIndexList(const QModelIndexList &from, const QModelIndexList &to);
    QModelIndexList persistentIndexList() const;

private:
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
inline QModelIndex QAbstractItemModel::createIndex(int arow, int acolumn, const void *adata) const
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

inline QVariant QModelIndex::data(int arole) const
{ return m ? m->data(*this, arole) : QVariant(); }

inline void QModelIndex::multiData(QModelRoleDataSpan roleDataSpan) const
{ if (m) m->multiData(*this, roleDataSpan); }

inline Qt::ItemFlags QModelIndex::flags() const
{ return m ? m->flags(*this) : Qt::ItemFlags(); }

inline size_t qHash(const QModelIndex &index, size_t seed = 0) noexcept
{ return size_t((size_t(index.row()) << 4) + size_t(index.column()) + index.internalId()) ^ seed; }

QT_END_NAMESPACE

QT_DECL_METATYPE_EXTERN(QModelIndexList, Q_CORE_EXPORT)

#endif // QABSTRACTITEMMODEL_H
