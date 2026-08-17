// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTABLEWIDGET_H
#define QTABLEWIDGET_H

#include <QtWidgets/qtableview.h>
#include <QtWidgets/qtwidgetsglobal.h>
#include <QtCore/qlist.h>
#include <QtCore/qvariant.h>

QT_REQUIRE_CONFIG(tablewidget);

QT_BEGIN_NAMESPACE

class QTableWidgetSelectionRange
{
public:
    QTableWidgetSelectionRange() = default;
    QTableWidgetSelectionRange(int top, int left, int bottom, int right)
      : m_top(top), m_left(left), m_bottom(bottom), m_right(right)
    {}

    friend bool operator==(const QTableWidgetSelectionRange &lhs,
                           const QTableWidgetSelectionRange &rhs) noexcept
    { return lhs.m_top == rhs.m_top && lhs.m_left == rhs.m_left
          && lhs.m_bottom == rhs.m_bottom && lhs.m_right == rhs.m_right; };
    friend bool operator!=(const QTableWidgetSelectionRange &lhs,
                           const QTableWidgetSelectionRange &rhs) noexcept
    { return !(lhs == rhs); }

    inline int topRow() const { return m_top; }
    inline int bottomRow() const { return m_bottom; }
    inline int leftColumn() const { return m_left; }
    inline int rightColumn() const { return m_right; }
    inline int rowCount() const { return m_bottom - m_top + 1; }
    inline int columnCount() const { return m_right - m_left + 1; }
private:
    int m_top = -1, m_left = -1, m_bottom = -2, m_right = -2;
};

class QTableWidget;
class QTableModel;
class QWidgetItemData;
class QTableWidgetItemPrivate;

class Q_WIDGETS_EXPORT QTableWidgetItem
{
    friend class QTableWidget;
    friend class QTableModel;
public:
    enum ItemType { Type = 0, UserType = 1000 };
    explicit QTableWidgetItem(int type = Type);
    explicit QTableWidgetItem(const QString &text, int type = Type);
    explicit QTableWidgetItem(const QIcon &icon, const QString &text, int type = Type);
    QTableWidgetItem(const QTableWidgetItem &other);
    virtual ~QTableWidgetItem();

    virtual QTableWidgetItem *clone() const;

    inline QTableWidget *tableWidget() const { return view; }

    inline int row() const;
    inline int column() const;

    void setSelected(bool select);
    bool isSelected() const;

    inline Qt::ItemFlags flags() const { return itemFlags; }
    void setFlags(Qt::ItemFlags flags);

    inline QString text() const
        { return data(Qt::DisplayRole).toString(); }
    inline void setText(const QString &text);

    inline QIcon icon() const
        { return qvariant_cast<QIcon>(data(Qt::DecorationRole)); }
    inline void setIcon(const QIcon &icon);

    inline QString statusTip() const
        { return data(Qt::StatusTipRole).toString(); }
    inline void setStatusTip(const QString &statusTip);

#if QT_CONFIG(tooltip)
    inline QString toolTip() const
        { return data(Qt::ToolTipRole).toString(); }
    inline void setToolTip(const QString &toolTip);
#endif

#if QT_CONFIG(whatsthis)
    inline QString whatsThis() const
        { return data(Qt::WhatsThisRole).toString(); }
    inline void setWhatsThis(const QString &whatsThis);
#endif

    inline QFont font() const
        { return qvariant_cast<QFont>(data(Qt::FontRole)); }
    inline void setFont(const QFont &font);

#if QT_VERSION < QT_VERSION_CHECK(7, 0, 0)
    inline int textAlignment() const
        { return data(Qt::TextAlignmentRole).toInt(); }
#else
    inline Qt::Alignment textAlignment() const
    { return qvariant_cast<Qt::Alignment>(data(Qt::TextAlignmentRole)); }
#endif
#if QT_DEPRECATED_SINCE(6, 4)
    QT_DEPRECATED_VERSION_X_6_4("Use the overload taking Qt::Alignment")
    inline void setTextAlignment(int alignment)
        { setData(Qt::TextAlignmentRole, alignment); }
    inline void setTextAlignment(Qt::AlignmentFlag alignment)
        { setData(Qt::TextAlignmentRole, QVariant::fromValue(Qt::Alignment(alignment))); }
#endif
    inline void setTextAlignment(Qt::Alignment alignment)
        { setData(Qt::TextAlignmentRole, QVariant::fromValue(alignment)); }

    inline QBrush background() const
        { return qvariant_cast<QBrush>(data(Qt::BackgroundRole)); }
    inline void setBackground(const QBrush &brush)
        { setData(Qt::BackgroundRole, brush.style() != Qt::NoBrush ? QVariant(brush) : QVariant()); }

    inline QBrush foreground() const
        { return qvariant_cast<QBrush>(data(Qt::ForegroundRole)); }
    inline void setForeground(const QBrush &brush)
        { setData(Qt::ForegroundRole, brush.style() != Qt::NoBrush ? QVariant(brush) : QVariant()); }

    inline Qt::CheckState checkState() const
        { return qvariant_cast<Qt::CheckState>(data(Qt::CheckStateRole)); }
    inline void setCheckState(Qt::CheckState state)
        { setData(Qt::CheckStateRole, state); }

    inline QSize sizeHint() const
        { return qvariant_cast<QSize>(data(Qt::SizeHintRole)); }
    inline void setSizeHint(const QSize &size)
        { setData(Qt::SizeHintRole, size.isValid() ? QVariant(size) : QVariant()); }

    virtual QVariant data(int role) const;
    virtual void setData(int role, const QVariant &value);

    virtual bool operator<(const QTableWidgetItem &other) const;

#ifndef QT_NO_DATASTREAM
    virtual void read(QDataStream &in);
    virtual void write(QDataStream &out) const;
#endif
    QTableWidgetItem &operator=(const QTableWidgetItem &other);

    inline int type() const { return rtti; }

private:
    QTableModel *tableModel() const;

private:
    int rtti;
    QList<QWidgetItemData> values;
    QTableWidget *view;
    QTableWidgetItemPrivate *d;
    Qt::ItemFlags itemFlags;
};

inline void QTableWidgetItem::setText(const QString &atext)
{ setData(Qt::DisplayRole, atext); }

inline void QTableWidgetItem::setIcon(const QIcon &aicon)
{ setData(Qt::DecorationRole, aicon); }

inline void QTableWidgetItem::setStatusTip(const QString &astatusTip)
{ setData(Qt::StatusTipRole, astatusTip); }

#if QT_CONFIG(tooltip)
inline void QTableWidgetItem::setToolTip(const QString &atoolTip)
{ setData(Qt::ToolTipRole, atoolTip); }
#endif

#if QT_CONFIG(whatsthis)
inline void QTableWidgetItem::setWhatsThis(const QString &awhatsThis)
{ setData(Qt::WhatsThisRole, awhatsThis); }
#endif

inline void QTableWidgetItem::setFont(const QFont &afont)
{ setData(Qt::FontRole, afont); }

#ifndef QT_NO_DATASTREAM
Q_WIDGETS_EXPORT QDataStream &operator>>(QDataStream &in, QTableWidgetItem &item);
Q_WIDGETS_EXPORT QDataStream &operator<<(QDataStream &out, const QTableWidgetItem &item);
#endif

class QTableWidgetPrivate;

class Q_WIDGETS_EXPORT QTableWidget : public QTableView
{
    Q_OBJECT
    Q_PROPERTY(int rowCount READ rowCount WRITE setRowCount)
    Q_PROPERTY(int columnCount READ columnCount WRITE setColumnCount)

    friend class QTableModel;
public:
    explicit QTableWidget(QWidget *parent = nullptr);
    QTableWidget(int rows, int columns, QWidget *parent = nullptr);
    ~QTableWidget();

    void setRowCount(int rows);
    int rowCount() const;

    void setColumnCount(int columns);
    int columnCount() const;

    int row(const QTableWidgetItem *item) const;
    int column(const QTableWidgetItem *item) const;

    QTableWidgetItem *item(int row, int column) const;
    void setItem(int row, int column, QTableWidgetItem *item);
    QTableWidgetItem *takeItem(int row, int column);
    QList<QTableWidgetItem*> items(const QMimeData *data) const;
    QModelIndex indexFromItem(const QTableWidgetItem *item) const;
    QTableWidgetItem *itemFromIndex(const QModelIndex &index) const;

    QTableWidgetItem *verticalHeaderItem(int row) const;
    void setVerticalHeaderItem(int row, QTableWidgetItem *item);
    QTableWidgetItem *takeVerticalHeaderItem(int row);

    QTableWidgetItem *horizontalHeaderItem(int column) const;
    void setHorizontalHeaderItem(int column, QTableWidgetItem *item);
    QTableWidgetItem *takeHorizontalHeaderItem(int column);
    void setVerticalHeaderLabels(const QStringList &labels);
    void setHorizontalHeaderLabels(const QStringList &labels);

    int currentRow() const;
    int currentColumn() const;
    QTableWidgetItem *currentItem() const;
    void setCurrentItem(QTableWidgetItem *item);
    void setCurrentItem(QTableWidgetItem *item, QItemSelectionModel::SelectionFlags command);
    void setCurrentCell(int row, int column);
    void setCurrentCell(int row, int column, QItemSelectionModel::SelectionFlags command);

    void sortItems(int column, Qt::SortOrder order = Qt::AscendingOrder);
    void setSortingEnabled(bool enable);
    bool isSortingEnabled() const;

    void editItem(QTableWidgetItem *item);
    void openPersistentEditor(QTableWidgetItem *item);
    void closePersistentEditor(QTableWidgetItem *item);
    using QAbstractItemView::isPersistentEditorOpen;
    bool isPersistentEditorOpen(QTableWidgetItem *item) const;

    QWidget *cellWidget(int row, int column) const;
    void setCellWidget(int row, int column, QWidget *widget);
    inline void removeCellWidget(int row, int column);

    void setRangeSelected(const QTableWidgetSelectionRange &range, bool select);

    QList<QTableWidgetSelectionRange> selectedRanges() const;
    QList<QTableWidgetItem*> selectedItems() const;
    QList<QTableWidgetItem*> findItems(const QString &text, Qt::MatchFlags flags) const;

    int visualRow(int logicalRow) const;
    int visualColumn(int logicalColumn) const;

    QTableWidgetItem *itemAt(const QPoint &p) const;
    inline QTableWidgetItem *itemAt(int x, int y) const;
    QRect visualItemRect(const QTableWidgetItem *item) const;

    const QTableWidgetItem *itemPrototype() const;
    void setItemPrototype(const QTableWidgetItem *item);

public Q_SLOTS:
    void scrollToItem(const QTableWidgetItem *item, QAbstractItemView::ScrollHint hint = EnsureVisible);
    void insertRow(int row);
    void insertColumn(int column);
    void removeRow(int row);
    void removeColumn(int column);
    void clear();
    void clearContents();

Q_SIGNALS:
    void itemPressed(QTableWidgetItem *item);
    void itemClicked(QTableWidgetItem *item);
    void itemDoubleClicked(QTableWidgetItem *item);

    void itemActivated(QTableWidgetItem *item);
    void itemEntered(QTableWidgetItem *item);
    void itemChanged(QTableWidgetItem *item);

    void currentItemChanged(QTableWidgetItem *current, QTableWidgetItem *previous);
    void itemSelectionChanged();

    void cellPressed(int row, int column);
    void cellClicked(int row, int column);
    void cellDoubleClicked(int row, int column);

    void cellActivated(int row, int column);
    void cellEntered(int row, int column);
    void cellChanged(int row, int column);

    void currentCellChanged(int currentRow, int currentColumn, int previousRow, int previousColumn);

protected:
    bool event(QEvent *e) override;
    virtual QStringList mimeTypes() const;
    virtual QMimeData *mimeData(const QList<QTableWidgetItem *> &items) const;
    virtual bool dropMimeData(int row, int column, const QMimeData *data, Qt::DropAction action);
    virtual Qt::DropActions supportedDropActions() const;

protected:
#if QT_CONFIG(draganddrop)
    void dropEvent(QDropEvent *event) override;
#endif
private:
    void setModel(QAbstractItemModel *model) override;

    Q_DECLARE_PRIVATE(QTableWidget)
    Q_DISABLE_COPY(QTableWidget)

    Q_PRIVATE_SLOT(d_func(), void _q_emitItemPressed(const QModelIndex &index))
    Q_PRIVATE_SLOT(d_func(), void _q_emitItemClicked(const QModelIndex &index))
    Q_PRIVATE_SLOT(d_func(), void _q_emitItemDoubleClicked(const QModelIndex &index))
    Q_PRIVATE_SLOT(d_func(), void _q_emitItemActivated(const QModelIndex &index))
    Q_PRIVATE_SLOT(d_func(), void _q_emitItemEntered(const QModelIndex &index))
    Q_PRIVATE_SLOT(d_func(), void _q_emitItemChanged(const QModelIndex &index))
    Q_PRIVATE_SLOT(d_func(), void _q_emitCurrentItemChanged(const QModelIndex &previous, const QModelIndex &current))
    Q_PRIVATE_SLOT(d_func(), void _q_sort())
    Q_PRIVATE_SLOT(d_func(), void _q_dataChanged(const QModelIndex &topLeft, const QModelIndex &bottomRight))
};

inline void QTableWidget::removeCellWidget(int arow, int acolumn)
{ setCellWidget(arow, acolumn, nullptr); }

inline QTableWidgetItem *QTableWidget::itemAt(int ax, int ay) const
{ return itemAt(QPoint(ax, ay)); }

inline int QTableWidgetItem::row() const
{ return (view ? view->row(this) : -1); }

inline int QTableWidgetItem::column() const
{ return (view ? view->column(this) : -1); }

QT_END_NAMESPACE

#endif // QTABLEWIDGET_H
