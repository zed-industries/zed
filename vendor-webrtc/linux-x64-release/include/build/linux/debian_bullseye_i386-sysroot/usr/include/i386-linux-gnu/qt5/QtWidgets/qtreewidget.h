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

#ifndef QTREEWIDGET_H
#define QTREEWIDGET_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtWidgets/qtreeview.h>
#include <QtWidgets/qtreewidgetitemiterator.h>
#include <QtCore/qvariant.h>
#include <QtCore/qvector.h>

QT_REQUIRE_CONFIG(treewidget);

QT_BEGIN_NAMESPACE

class QTreeWidget;
class QTreeModel;
class QWidgetItemData;
class QTreeWidgetItemPrivate;

class Q_WIDGETS_EXPORT QTreeWidgetItem
{
    friend class QTreeModel;
    friend class QTreeWidget;
    friend class QTreeWidgetPrivate;
    friend class QTreeWidgetItemIterator;
    friend class QTreeWidgetItemPrivate;
public:
    enum ItemType { Type = 0, UserType = 1000 };
    explicit QTreeWidgetItem(int type = Type);
    explicit QTreeWidgetItem(const QStringList &strings, int type = Type);
    explicit QTreeWidgetItem(QTreeWidget *treeview, int type = Type);
    QTreeWidgetItem(QTreeWidget *treeview, const QStringList &strings, int type = Type);
    QTreeWidgetItem(QTreeWidget *treeview, QTreeWidgetItem *after, int type = Type);
    explicit QTreeWidgetItem(QTreeWidgetItem *parent, int type = Type);
    QTreeWidgetItem(QTreeWidgetItem *parent, const QStringList &strings, int type = Type);
    QTreeWidgetItem(QTreeWidgetItem *parent, QTreeWidgetItem *after, int type = Type);
    QTreeWidgetItem(const QTreeWidgetItem &other);
    virtual ~QTreeWidgetItem();

    virtual QTreeWidgetItem *clone() const;

    inline QTreeWidget *treeWidget() const { return view; }

    void setSelected(bool select);
    bool isSelected() const;

    void setHidden(bool hide);
    bool isHidden() const;

    void setExpanded(bool expand);
    bool isExpanded() const;

    void setFirstColumnSpanned(bool span);
    bool isFirstColumnSpanned() const;

    inline void setDisabled(bool disabled);
    inline bool isDisabled() const;

    enum ChildIndicatorPolicy { ShowIndicator, DontShowIndicator, DontShowIndicatorWhenChildless };
    void setChildIndicatorPolicy(QTreeWidgetItem::ChildIndicatorPolicy policy);
    QTreeWidgetItem::ChildIndicatorPolicy childIndicatorPolicy() const;

    Qt::ItemFlags flags() const;
    void setFlags(Qt::ItemFlags flags);

    inline QString text(int column) const
        { return data(column, Qt::DisplayRole).toString(); }
    inline void setText(int column, const QString &text);

    inline QIcon icon(int column) const
        { return qvariant_cast<QIcon>(data(column, Qt::DecorationRole)); }
    inline void setIcon(int column, const QIcon &icon);

    inline QString statusTip(int column) const
        { return data(column, Qt::StatusTipRole).toString(); }
    inline void setStatusTip(int column, const QString &statusTip);

#ifndef QT_NO_TOOLTIP
    inline QString toolTip(int column) const
        { return data(column, Qt::ToolTipRole).toString(); }
    inline void setToolTip(int column, const QString &toolTip);
#endif

#if QT_CONFIG(whatsthis)
    inline QString whatsThis(int column) const
        { return data(column, Qt::WhatsThisRole).toString(); }
    inline void setWhatsThis(int column, const QString &whatsThis);
#endif

    inline QFont font(int column) const
        { return qvariant_cast<QFont>(data(column, Qt::FontRole)); }
    inline void setFont(int column, const QFont &font);

    inline int textAlignment(int column) const
        { return data(column, Qt::TextAlignmentRole).toInt(); }
    inline void setTextAlignment(int column, int alignment)
        { setData(column, Qt::TextAlignmentRole, alignment); }

#if QT_DEPRECATED_SINCE(5, 13)
    QT_DEPRECATED_X ("Use QTreeWidgetItem::background() instead")
    inline QColor backgroundColor(int column) const
        { return qvariant_cast<QColor>(data(column, Qt::BackgroundRole)); }
    QT_DEPRECATED_X ("Use QTreeWidgetItem::setBackground() instead")
    inline void setBackgroundColor(int column, const QColor &color)
        { setData(column, Qt::BackgroundRole, color); }
#endif

    inline QBrush background(int column) const
        { return qvariant_cast<QBrush>(data(column, Qt::BackgroundRole)); }
    inline void setBackground(int column, const QBrush &brush)
        { setData(column, Qt::BackgroundRole, brush.style() != Qt::NoBrush ? QVariant(brush) : QVariant()); }

#if QT_DEPRECATED_SINCE(5, 13)
    QT_DEPRECATED_X ("Use QTreeWidgetItem::foreground() instead")
    inline QColor textColor(int column) const
        { return qvariant_cast<QColor>(data(column, Qt::ForegroundRole)); }
    QT_DEPRECATED_X ("Use QTreeWidgetItem::setForeground() instead")
    inline void setTextColor(int column, const QColor &color)
        { setData(column, Qt::ForegroundRole, color); }
#endif

    inline QBrush foreground(int column) const
        { return qvariant_cast<QBrush>(data(column, Qt::ForegroundRole)); }
    inline void setForeground(int column, const QBrush &brush)
        { setData(column, Qt::ForegroundRole, brush.style() != Qt::NoBrush ? QVariant(brush) : QVariant()); }

    inline Qt::CheckState checkState(int column) const
        { return static_cast<Qt::CheckState>(data(column, Qt::CheckStateRole).toInt()); }
    inline void setCheckState(int column, Qt::CheckState state)
        { setData(column, Qt::CheckStateRole, state); }

    inline QSize sizeHint(int column) const
        { return qvariant_cast<QSize>(data(column, Qt::SizeHintRole)); }
    inline void setSizeHint(int column, const QSize &size)
        { setData(column, Qt::SizeHintRole, size.isValid() ? QVariant(size) : QVariant()); }

    virtual QVariant data(int column, int role) const;
    virtual void setData(int column, int role, const QVariant &value);

    virtual bool operator<(const QTreeWidgetItem &other) const;

#ifndef QT_NO_DATASTREAM
    virtual void read(QDataStream &in);
    virtual void write(QDataStream &out) const;
#endif
    QTreeWidgetItem &operator=(const QTreeWidgetItem &other);

    inline QTreeWidgetItem *parent() const { return par; }
    inline QTreeWidgetItem *child(int index) const {
        if (index < 0 || index >= children.size())
            return nullptr;
        executePendingSort();
        return children.at(index);
    }
    inline int childCount() const { return children.count(); }
    inline int columnCount() const { return values.count(); }
    inline int indexOfChild(QTreeWidgetItem *child) const;

    void addChild(QTreeWidgetItem *child);
    void insertChild(int index, QTreeWidgetItem *child);
    void removeChild(QTreeWidgetItem *child);
    QTreeWidgetItem *takeChild(int index);

    void addChildren(const QList<QTreeWidgetItem*> &children);
    void insertChildren(int index, const QList<QTreeWidgetItem*> &children);
    QList<QTreeWidgetItem*> takeChildren();

    inline int type() const { return rtti; }
    inline void sortChildren(int column, Qt::SortOrder order)
        { sortChildren(column, order, false); }

protected:
    void emitDataChanged();

private:
    void sortChildren(int column, Qt::SortOrder order, bool climb);
    QVariant childrenCheckState(int column) const;
    void itemChanged();
    void executePendingSort() const;
    QTreeModel *treeModel(QTreeWidget *v = nullptr) const;

    int rtti;
    // One item has a vector of column entries. Each column has a vector of (role, value) pairs.
    QVector< QVector<QWidgetItemData> > values;
    QTreeWidget *view;
    QTreeWidgetItemPrivate *d;
    QTreeWidgetItem *par;
    QList<QTreeWidgetItem*> children;
    Qt::ItemFlags itemFlags;
};

inline void QTreeWidgetItem::setText(int column, const QString &atext)
{ setData(column, Qt::DisplayRole, atext); }

inline void QTreeWidgetItem::setIcon(int column, const QIcon &aicon)
{ setData(column, Qt::DecorationRole, aicon); }

#if QT_CONFIG(statustip)
inline void QTreeWidgetItem::setStatusTip(int column, const QString &astatusTip)
{ setData(column, Qt::StatusTipRole, astatusTip); }
#endif

#ifndef QT_NO_TOOLTIP
inline void QTreeWidgetItem::setToolTip(int column, const QString &atoolTip)
{ setData(column, Qt::ToolTipRole, atoolTip); }
#endif

#if QT_CONFIG(whatsthis)
inline void QTreeWidgetItem::setWhatsThis(int column, const QString &awhatsThis)
{ setData(column, Qt::WhatsThisRole, awhatsThis); }
#endif

inline void QTreeWidgetItem::setFont(int column, const QFont &afont)
{ setData(column, Qt::FontRole, afont); }

inline int QTreeWidgetItem::indexOfChild(QTreeWidgetItem *achild) const
{ executePendingSort(); return children.indexOf(achild); }

#ifndef QT_NO_DATASTREAM
Q_WIDGETS_EXPORT QDataStream &operator<<(QDataStream &out, const QTreeWidgetItem &item);
Q_WIDGETS_EXPORT QDataStream &operator>>(QDataStream &in, QTreeWidgetItem &item);
#endif

class QTreeWidgetPrivate;

class Q_WIDGETS_EXPORT QTreeWidget : public QTreeView
{
    Q_OBJECT
    Q_PROPERTY(int columnCount READ columnCount WRITE setColumnCount)
    Q_PROPERTY(int topLevelItemCount READ topLevelItemCount)

    friend class QTreeModel;
    friend class QTreeWidgetItem;
public:
    explicit QTreeWidget(QWidget *parent = nullptr);
    ~QTreeWidget();

    int columnCount() const;
    void setColumnCount(int columns);

    QTreeWidgetItem *invisibleRootItem() const;
    QTreeWidgetItem *topLevelItem(int index) const;
    int topLevelItemCount() const;
    void insertTopLevelItem(int index, QTreeWidgetItem *item);
    void addTopLevelItem(QTreeWidgetItem *item);
    QTreeWidgetItem *takeTopLevelItem(int index);
    int indexOfTopLevelItem(QTreeWidgetItem *item) const;

    void insertTopLevelItems(int index, const QList<QTreeWidgetItem*> &items);
    void addTopLevelItems(const QList<QTreeWidgetItem*> &items);

    QTreeWidgetItem *headerItem() const;
    void setHeaderItem(QTreeWidgetItem *item);
    void setHeaderLabels(const QStringList &labels);
    inline void setHeaderLabel(const QString &label);

    QTreeWidgetItem *currentItem() const;
    int currentColumn() const;
    void setCurrentItem(QTreeWidgetItem *item);
    void setCurrentItem(QTreeWidgetItem *item, int column);
    void setCurrentItem(QTreeWidgetItem *item, int column, QItemSelectionModel::SelectionFlags command);

    QTreeWidgetItem *itemAt(const QPoint &p) const;
    inline QTreeWidgetItem *itemAt(int x, int y) const;
    QRect visualItemRect(const QTreeWidgetItem *item) const;

    int sortColumn() const;
    void sortItems(int column, Qt::SortOrder order);

    void editItem(QTreeWidgetItem *item, int column = 0);
    void openPersistentEditor(QTreeWidgetItem *item, int column = 0);
    void closePersistentEditor(QTreeWidgetItem *item, int column = 0);
    using QAbstractItemView::isPersistentEditorOpen;
    bool isPersistentEditorOpen(QTreeWidgetItem *item, int column = 0) const;

    QWidget *itemWidget(QTreeWidgetItem *item, int column) const;
    void setItemWidget(QTreeWidgetItem *item, int column, QWidget *widget);
    inline void removeItemWidget(QTreeWidgetItem *item, int column);

#if QT_DEPRECATED_SINCE(5, 13)
    QT_DEPRECATED_X ("Use QTreeWidgetItem::isSelected() instead")
    bool isItemSelected(const QTreeWidgetItem *item) const;
    QT_DEPRECATED_X ("Use QTreeWidgetItem::setSelected() instead")
    void setItemSelected(const QTreeWidgetItem *item, bool select);
#endif
    QList<QTreeWidgetItem*> selectedItems() const;
    QList<QTreeWidgetItem*> findItems(const QString &text, Qt::MatchFlags flags,
                                      int column = 0) const;

#if QT_DEPRECATED_SINCE(5, 13)
    QT_DEPRECATED_X ("Use QTreeWidgetItem::isHidden() instead")
    bool isItemHidden(const QTreeWidgetItem *item) const;
    QT_DEPRECATED_X ("Use QTreeWidgetItem::setHidden() instead")
    void setItemHidden(const QTreeWidgetItem *item, bool hide);

    QT_DEPRECATED_X ("Use QTreeWidgetItem::isExpanded() instead")
    bool isItemExpanded(const QTreeWidgetItem *item) const;
    QT_DEPRECATED_X ("Use QTreeWidgetItem::setExpanded() instead")
    void setItemExpanded(const QTreeWidgetItem *item, bool expand);

    QT_DEPRECATED_X ("Use QTreeWidgetItem::isFirstColumnSpanned() instead")
    bool isFirstItemColumnSpanned(const QTreeWidgetItem *item) const;
    QT_DEPRECATED_X ("Use QTreeWidgetItem::setFirstColumnSpanned() instead")
    void setFirstItemColumnSpanned(const QTreeWidgetItem *item, bool span);
#endif

    QTreeWidgetItem *itemAbove(const QTreeWidgetItem *item) const;
    QTreeWidgetItem *itemBelow(const QTreeWidgetItem *item) const;

    void setSelectionModel(QItemSelectionModel *selectionModel) override;

public Q_SLOTS:
    void scrollToItem(const QTreeWidgetItem *item,
                      QAbstractItemView::ScrollHint hint = EnsureVisible);
    void expandItem(const QTreeWidgetItem *item);
    void collapseItem(const QTreeWidgetItem *item);
    void clear();

Q_SIGNALS:
    void itemPressed(QTreeWidgetItem *item, int column);
    void itemClicked(QTreeWidgetItem *item, int column);
    void itemDoubleClicked(QTreeWidgetItem *item, int column);
    void itemActivated(QTreeWidgetItem *item, int column);
    void itemEntered(QTreeWidgetItem *item, int column);
    // ### Qt 6: add changed roles
    void itemChanged(QTreeWidgetItem *item, int column);
    void itemExpanded(QTreeWidgetItem *item);
    void itemCollapsed(QTreeWidgetItem *item);
    void currentItemChanged(QTreeWidgetItem *current, QTreeWidgetItem *previous);
    void itemSelectionChanged();

protected:
    bool event(QEvent *e) override;
    virtual QStringList mimeTypes() const;
#if QT_VERSION >= QT_VERSION_CHECK(6,0,0)
    virtual QMimeData *mimeData(const QList<QTreeWidgetItem *> &items) const;
#else
    virtual QMimeData *mimeData(const QList<QTreeWidgetItem*> items) const;
#endif
    virtual bool dropMimeData(QTreeWidgetItem *parent, int index,
                              const QMimeData *data, Qt::DropAction action);
    virtual Qt::DropActions supportedDropActions() const;

#if QT_VERSION >= QT_VERSION_CHECK(6, 0, 0)
public:
#else
protected:
#endif
    QList<QTreeWidgetItem*> items(const QMimeData *data) const;

    QModelIndex indexFromItem(const QTreeWidgetItem *item, int column = 0) const;
#if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
    QModelIndex indexFromItem(QTreeWidgetItem *item, int column = 0) const; // ### Qt 6: remove
#endif
    QTreeWidgetItem *itemFromIndex(const QModelIndex &index) const;

protected:
#if QT_CONFIG(draganddrop)
    void dropEvent(QDropEvent *event) override;
#endif
private:
    void setModel(QAbstractItemModel *model) override;

    Q_DECLARE_PRIVATE(QTreeWidget)
    Q_DISABLE_COPY(QTreeWidget)

    Q_PRIVATE_SLOT(d_func(), void _q_emitItemPressed(const QModelIndex &index))
    Q_PRIVATE_SLOT(d_func(), void _q_emitItemClicked(const QModelIndex &index))
    Q_PRIVATE_SLOT(d_func(), void _q_emitItemDoubleClicked(const QModelIndex &index))
    Q_PRIVATE_SLOT(d_func(), void _q_emitItemActivated(const QModelIndex &index))
    Q_PRIVATE_SLOT(d_func(), void _q_emitItemEntered(const QModelIndex &index))
    Q_PRIVATE_SLOT(d_func(), void _q_emitItemChanged(const QModelIndex &index))
    Q_PRIVATE_SLOT(d_func(), void _q_emitItemExpanded(const QModelIndex &index))
    Q_PRIVATE_SLOT(d_func(), void _q_emitItemCollapsed(const QModelIndex &index))
    Q_PRIVATE_SLOT(d_func(), void _q_emitCurrentItemChanged(const QModelIndex &previous, const QModelIndex &current))
    Q_PRIVATE_SLOT(d_func(), void _q_sort())
    Q_PRIVATE_SLOT(d_func(), void _q_dataChanged(const QModelIndex &topLeft, const QModelIndex &bottomRight))
    Q_PRIVATE_SLOT(d_func(), void _q_selectionChanged(const QItemSelection &selected, const QItemSelection &deselected))
};

inline void QTreeWidget::removeItemWidget(QTreeWidgetItem *item, int column)
{ setItemWidget(item, column, nullptr); }

inline QTreeWidgetItem *QTreeWidget::itemAt(int ax, int ay) const
{ return itemAt(QPoint(ax, ay)); }

inline void QTreeWidget::setHeaderLabel(const QString &alabel)
{ setHeaderLabels(QStringList(alabel)); }

inline void QTreeWidgetItem::setDisabled(bool disabled)
{ setFlags(disabled ? (flags() & ~Qt::ItemIsEnabled) : flags() | Qt::ItemIsEnabled); }

inline bool QTreeWidgetItem::isDisabled() const
{ return !(flags() & Qt::ItemIsEnabled); }

QT_END_NAMESPACE

#endif // QTREEWIDGET_H
