// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QGRAPHICSLAYOUTITEM_H
#define QGRAPHICSLAYOUTITEM_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtCore/qscopedpointer.h>
#include <QtWidgets/qsizepolicy.h>
#include <QtGui/qevent.h>

QT_REQUIRE_CONFIG(graphicsview);

QT_BEGIN_NAMESPACE

class QGraphicsLayoutItemPrivate;
class QGraphicsItem;
class Q_WIDGETS_EXPORT QGraphicsLayoutItem
{
public:
    QGraphicsLayoutItem(QGraphicsLayoutItem *parent = nullptr, bool isLayout = false);
    virtual ~QGraphicsLayoutItem();

    void setSizePolicy(const QSizePolicy &policy);
    void setSizePolicy(QSizePolicy::Policy hPolicy, QSizePolicy::Policy vPolicy, QSizePolicy::ControlType controlType = QSizePolicy::DefaultType);
    QSizePolicy sizePolicy() const;

    void setMinimumSize(const QSizeF &size);
    inline void setMinimumSize(qreal w, qreal h);
    QSizeF minimumSize() const;
    void setMinimumWidth(qreal width);
    inline qreal minimumWidth() const;
    void setMinimumHeight(qreal height);
    inline qreal minimumHeight() const;

    void setPreferredSize(const QSizeF &size);
    inline void setPreferredSize(qreal w, qreal h);
    QSizeF preferredSize() const;
    void setPreferredWidth(qreal width);
    inline qreal preferredWidth() const;
    void setPreferredHeight(qreal height);
    inline qreal preferredHeight() const;

    void setMaximumSize(const QSizeF &size);
    inline void setMaximumSize(qreal w, qreal h);
    QSizeF maximumSize() const;
    void setMaximumWidth(qreal width);
    inline qreal maximumWidth() const;
    void setMaximumHeight(qreal height);
    inline qreal maximumHeight() const;

    virtual void setGeometry(const QRectF &rect);
    QRectF geometry() const;
    virtual void getContentsMargins(qreal *left, qreal *top, qreal *right, qreal *bottom) const;
    QRectF contentsRect() const;

    QSizeF effectiveSizeHint(Qt::SizeHint which, const QSizeF &constraint = QSizeF()) const;

    virtual void updateGeometry();

    virtual bool isEmpty() const;
    QGraphicsLayoutItem *parentLayoutItem() const;
    void setParentLayoutItem(QGraphicsLayoutItem *parent);

    bool isLayout() const;
    QGraphicsItem *graphicsItem() const;
    bool ownedByLayout() const;

protected:
    void setGraphicsItem(QGraphicsItem *item);
    void setOwnedByLayout(bool ownedByLayout);
    QGraphicsLayoutItem(QGraphicsLayoutItemPrivate &dd);

    virtual QSizeF sizeHint(Qt::SizeHint which, const QSizeF &constraint = QSizeF()) const = 0;
    QScopedPointer<QGraphicsLayoutItemPrivate> d_ptr;

private:
    QSizeF *effectiveSizeHints(const QSizeF &constraint) const;
    Q_DECLARE_PRIVATE(QGraphicsLayoutItem)

    friend class QGraphicsLayout;
};

#ifndef Q_CLANG_QDOC
Q_DECLARE_INTERFACE(QGraphicsLayoutItem, "org.qt-project.Qt.QGraphicsLayoutItem")
#endif

inline void QGraphicsLayoutItem::setMinimumSize(qreal aw, qreal ah)
{ setMinimumSize(QSizeF(aw, ah)); }
inline void QGraphicsLayoutItem::setPreferredSize(qreal aw, qreal ah)
{ setPreferredSize(QSizeF(aw, ah)); }
inline void QGraphicsLayoutItem::setMaximumSize(qreal aw, qreal ah)
{ setMaximumSize(QSizeF(aw, ah)); }

inline qreal QGraphicsLayoutItem::minimumWidth() const
{ return effectiveSizeHint(Qt::MinimumSize).width(); }
inline qreal QGraphicsLayoutItem::minimumHeight() const
{ return effectiveSizeHint(Qt::MinimumSize).height(); }

inline qreal QGraphicsLayoutItem::preferredWidth() const
{ return effectiveSizeHint(Qt::PreferredSize).width(); }
inline qreal QGraphicsLayoutItem::preferredHeight() const
{ return effectiveSizeHint(Qt::PreferredSize).height(); }

inline qreal QGraphicsLayoutItem::maximumWidth() const
{ return effectiveSizeHint(Qt::MaximumSize).width(); }
inline qreal QGraphicsLayoutItem::maximumHeight() const
{ return effectiveSizeHint(Qt::MaximumSize).height(); }

QT_END_NAMESPACE

#endif
