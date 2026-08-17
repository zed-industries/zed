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

#ifndef QGRAPHICSSCENE_H
#define QGRAPHICSSCENE_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtCore/qobject.h>
#include <QtCore/qpoint.h>
#include <QtCore/qrect.h>
#include <QtGui/qbrush.h>
#include <QtGui/qfont.h>
#include <QtGui/qtransform.h>
#include <QtGui/qpen.h>

QT_REQUIRE_CONFIG(graphicsview);

QT_BEGIN_NAMESPACE

template<typename T> class QList;
class QFocusEvent;
class QFont;
class QFontMetrics;
class QGraphicsEllipseItem;
class QGraphicsItem;
class QGraphicsItemGroup;
class QGraphicsLineItem;
class QGraphicsPathItem;
class QGraphicsPixmapItem;
class QGraphicsPolygonItem;
class QGraphicsProxyWidget;
class QGraphicsRectItem;
class QGraphicsSceneContextMenuEvent;
class QGraphicsSceneDragDropEvent;
class QGraphicsSceneEvent;
class QGraphicsSceneHelpEvent;
class QGraphicsSceneHoverEvent;
class QGraphicsSceneMouseEvent;
class QGraphicsSceneWheelEvent;
class QGraphicsSimpleTextItem;
class QGraphicsTextItem;
class QGraphicsView;
class QGraphicsWidget;
class QGraphicsSceneIndex;
class QHelpEvent;
class QInputMethodEvent;
class QKeyEvent;
class QLineF;
class QPainterPath;
class QPixmap;
class QPointF;
class QPolygonF;
class QRectF;
class QSizeF;
class QStyle;
class QStyleOptionGraphicsItem;

class QGraphicsScenePrivate;
class Q_WIDGETS_EXPORT QGraphicsScene : public QObject
{
    Q_OBJECT
    Q_PROPERTY(QBrush backgroundBrush READ backgroundBrush WRITE setBackgroundBrush)
    Q_PROPERTY(QBrush foregroundBrush READ foregroundBrush WRITE setForegroundBrush)
    Q_PROPERTY(ItemIndexMethod itemIndexMethod READ itemIndexMethod WRITE setItemIndexMethod)
    Q_PROPERTY(QRectF sceneRect READ sceneRect WRITE setSceneRect)
    Q_PROPERTY(int bspTreeDepth READ bspTreeDepth WRITE setBspTreeDepth)
    Q_PROPERTY(QPalette palette READ palette WRITE setPalette)
    Q_PROPERTY(QFont font READ font WRITE setFont)
#if QT_DEPRECATED_SINCE(5, 13)
    Q_PROPERTY(bool sortCacheEnabled READ isSortCacheEnabled WRITE setSortCacheEnabled)
#endif
    Q_PROPERTY(bool stickyFocus READ stickyFocus WRITE setStickyFocus)
    Q_PROPERTY(qreal minimumRenderSize READ minimumRenderSize WRITE setMinimumRenderSize)
    Q_PROPERTY(bool focusOnTouch READ focusOnTouch WRITE setFocusOnTouch)

public:
    enum ItemIndexMethod {
        BspTreeIndex,
        NoIndex = -1
    };
    Q_ENUM(ItemIndexMethod)
    enum SceneLayer {
        ItemLayer = 0x1,
        BackgroundLayer = 0x2,
        ForegroundLayer = 0x4,
        AllLayers = 0xffff
    };
    Q_DECLARE_FLAGS(SceneLayers, SceneLayer)

    QGraphicsScene(QObject *parent = nullptr);
    QGraphicsScene(const QRectF &sceneRect, QObject *parent = nullptr);
    QGraphicsScene(qreal x, qreal y, qreal width, qreal height, QObject *parent = nullptr);
    virtual ~QGraphicsScene();

    QRectF sceneRect() const;
    inline qreal width() const { return sceneRect().width(); }
    inline qreal height() const { return sceneRect().height(); }
    void setSceneRect(const QRectF &rect);
    inline void setSceneRect(qreal x, qreal y, qreal w, qreal h)
    { setSceneRect(QRectF(x, y, w, h)); }

    void render(QPainter *painter,
                const QRectF &target = QRectF(), const QRectF &source = QRectF(),
                Qt::AspectRatioMode aspectRatioMode = Qt::KeepAspectRatio);

    ItemIndexMethod itemIndexMethod() const;
    void setItemIndexMethod(ItemIndexMethod method);

#if QT_DEPRECATED_SINCE(5, 13)
    QT_DEPRECATED bool isSortCacheEnabled() const;
    QT_DEPRECATED void setSortCacheEnabled(bool enabled);
#endif

    int bspTreeDepth() const;
    void setBspTreeDepth(int depth);

    QRectF itemsBoundingRect() const;

    QList<QGraphicsItem *> items(Qt::SortOrder order = Qt::DescendingOrder) const;

    QList<QGraphicsItem *> items(const QPointF &pos, Qt::ItemSelectionMode mode = Qt::IntersectsItemShape, Qt::SortOrder order = Qt::DescendingOrder, const QTransform &deviceTransform = QTransform()) const;
    QList<QGraphicsItem *> items(const QRectF &rect, Qt::ItemSelectionMode mode = Qt::IntersectsItemShape, Qt::SortOrder order = Qt::DescendingOrder, const QTransform &deviceTransform = QTransform()) const;
    QList<QGraphicsItem *> items(const QPolygonF &polygon, Qt::ItemSelectionMode mode = Qt::IntersectsItemShape, Qt::SortOrder order = Qt::DescendingOrder, const QTransform &deviceTransform = QTransform()) const;
    QList<QGraphicsItem *> items(const QPainterPath &path, Qt::ItemSelectionMode mode = Qt::IntersectsItemShape, Qt::SortOrder order = Qt::DescendingOrder, const QTransform &deviceTransform = QTransform()) const;

    QList<QGraphicsItem *> collidingItems(const QGraphicsItem *item, Qt::ItemSelectionMode mode = Qt::IntersectsItemShape) const;
#if QT_DEPRECATED_SINCE(5, 0)
    QT_DEPRECATED inline QGraphicsItem *itemAt(const QPointF &position) const {
        QList<QGraphicsItem *> itemsAtPoint = items(position);
        return itemsAtPoint.isEmpty() ? nullptr : itemsAtPoint.first();
    }
#endif
    QGraphicsItem *itemAt(const QPointF &pos, const QTransform &deviceTransform) const;
#if QT_DEPRECATED_SINCE(5, 0)
    QT_DEPRECATED inline QList<QGraphicsItem *> items(qreal x, qreal y, qreal w, qreal h, Qt::ItemSelectionMode mode = Qt::IntersectsItemShape) const
    { return items(QRectF(x, y, w, h), mode); }
#endif
    inline QList<QGraphicsItem *> items(qreal x, qreal y, qreal w, qreal h, Qt::ItemSelectionMode mode, Qt::SortOrder order,
                                        const QTransform &deviceTransform = QTransform()) const
    { return items(QRectF(x, y, w, h), mode, order, deviceTransform); }
#if QT_DEPRECATED_SINCE(5, 0)
    QT_DEPRECATED inline QGraphicsItem *itemAt(qreal x, qreal y) const {
        QList<QGraphicsItem *> itemsAtPoint = items(QPointF(x, y));
        return itemsAtPoint.isEmpty() ? nullptr : itemsAtPoint.first();
    }
#endif
    inline QGraphicsItem *itemAt(qreal x, qreal y, const QTransform &deviceTransform) const
    { return itemAt(QPointF(x, y), deviceTransform); }

    QList<QGraphicsItem *> selectedItems() const;
    QPainterPath selectionArea() const;
    void setSelectionArea(const QPainterPath &path, const QTransform &deviceTransform);
    void setSelectionArea(const QPainterPath &path, Qt::ItemSelectionMode mode = Qt::IntersectsItemShape, const QTransform &deviceTransform = QTransform());
    void setSelectionArea(const QPainterPath &path, Qt::ItemSelectionOperation selectionOperation, Qt::ItemSelectionMode mode = Qt::IntersectsItemShape, const QTransform &deviceTransform = QTransform());
    // ### Qt6 merge the last 2 functions and add a default: Qt::ItemSelectionOperation selectionOperation = Qt::ReplaceSelection

    QGraphicsItemGroup *createItemGroup(const QList<QGraphicsItem *> &items);
    void destroyItemGroup(QGraphicsItemGroup *group);

    void addItem(QGraphicsItem *item);
    QGraphicsEllipseItem *addEllipse(const QRectF &rect, const QPen &pen = QPen(), const QBrush &brush = QBrush());
    QGraphicsLineItem *addLine(const QLineF &line, const QPen &pen = QPen());
    QGraphicsPathItem *addPath(const QPainterPath &path, const QPen &pen = QPen(), const QBrush &brush = QBrush());
    QGraphicsPixmapItem *addPixmap(const QPixmap &pixmap);
    QGraphicsPolygonItem *addPolygon(const QPolygonF &polygon, const QPen &pen = QPen(), const QBrush &brush = QBrush());
    QGraphicsRectItem *addRect(const QRectF &rect, const QPen &pen = QPen(), const QBrush &brush = QBrush());
    QGraphicsTextItem *addText(const QString &text, const QFont &font = QFont());
    QGraphicsSimpleTextItem *addSimpleText(const QString &text, const QFont &font = QFont());
    QGraphicsProxyWidget *addWidget(QWidget *widget, Qt::WindowFlags wFlags = Qt::WindowFlags());
    inline QGraphicsEllipseItem *addEllipse(qreal x, qreal y, qreal w, qreal h, const QPen &pen = QPen(), const QBrush &brush = QBrush())
    { return addEllipse(QRectF(x, y, w, h), pen, brush); }
    inline QGraphicsLineItem *addLine(qreal x1, qreal y1, qreal x2, qreal y2, const QPen &pen = QPen())
    { return addLine(QLineF(x1, y1, x2, y2), pen); }
    inline QGraphicsRectItem *addRect(qreal x, qreal y, qreal w, qreal h, const QPen &pen = QPen(), const QBrush &brush = QBrush())
    { return addRect(QRectF(x, y, w, h), pen, brush); }
    void removeItem(QGraphicsItem *item);

    QGraphicsItem *focusItem() const;
    void setFocusItem(QGraphicsItem *item, Qt::FocusReason focusReason = Qt::OtherFocusReason);
    bool hasFocus() const;
    void setFocus(Qt::FocusReason focusReason = Qt::OtherFocusReason);
    void clearFocus();

    void setStickyFocus(bool enabled);
    bool stickyFocus() const;

    QGraphicsItem *mouseGrabberItem() const;

    QBrush backgroundBrush() const;
    void setBackgroundBrush(const QBrush &brush);

    QBrush foregroundBrush() const;
    void setForegroundBrush(const QBrush &brush);

    virtual QVariant inputMethodQuery(Qt::InputMethodQuery query) const;

    QList <QGraphicsView *> views() const;

    inline void update(qreal x, qreal y, qreal w, qreal h)
    { update(QRectF(x, y, w, h)); }
    inline void invalidate(qreal x, qreal y, qreal w, qreal h, SceneLayers layers = AllLayers)
    { invalidate(QRectF(x, y, w, h), layers); }

    QStyle *style() const;
    void setStyle(QStyle *style);

    QFont font() const;
    void setFont(const QFont &font);

    QPalette palette() const;
    void setPalette(const QPalette &palette);

    bool isActive() const;
    QGraphicsItem *activePanel() const;
    void setActivePanel(QGraphicsItem *item);
    QGraphicsWidget *activeWindow() const;
    void setActiveWindow(QGraphicsWidget *widget);

    bool sendEvent(QGraphicsItem *item, QEvent *event);

    qreal minimumRenderSize() const;
    void setMinimumRenderSize(qreal minSize);

    bool focusOnTouch() const;
    void setFocusOnTouch(bool enabled);

public Q_SLOTS:
    void update(const QRectF &rect = QRectF());
    void invalidate(const QRectF &rect = QRectF(), SceneLayers layers = AllLayers);
    void advance();
    void clearSelection();
    void clear();

protected:
    bool event(QEvent *event) override;
    bool eventFilter(QObject *watched, QEvent *event) override;
    virtual void contextMenuEvent(QGraphicsSceneContextMenuEvent *event);
    virtual void dragEnterEvent(QGraphicsSceneDragDropEvent *event);
    virtual void dragMoveEvent(QGraphicsSceneDragDropEvent *event);
    virtual void dragLeaveEvent(QGraphicsSceneDragDropEvent *event);
    virtual void dropEvent(QGraphicsSceneDragDropEvent *event);
    virtual void focusInEvent(QFocusEvent *event);
    virtual void focusOutEvent(QFocusEvent *event);
    virtual void helpEvent(QGraphicsSceneHelpEvent *event);
    virtual void keyPressEvent(QKeyEvent *event);
    virtual void keyReleaseEvent(QKeyEvent *event);
    virtual void mousePressEvent(QGraphicsSceneMouseEvent *event);
    virtual void mouseMoveEvent(QGraphicsSceneMouseEvent *event);
    virtual void mouseReleaseEvent(QGraphicsSceneMouseEvent *event);
    virtual void mouseDoubleClickEvent(QGraphicsSceneMouseEvent *event);
    virtual void wheelEvent(QGraphicsSceneWheelEvent *event);
    virtual void inputMethodEvent(QInputMethodEvent *event);

    virtual void drawBackground(QPainter *painter, const QRectF &rect);
    virtual void drawForeground(QPainter *painter, const QRectF &rect);
    virtual void drawItems(QPainter *painter, int numItems,
                           QGraphicsItem *items[],
                           const QStyleOptionGraphicsItem options[],
                           QWidget *widget = nullptr);

protected Q_SLOTS:
    QT6_VIRTUAL bool focusNextPrevChild(bool next);

Q_SIGNALS:
    void changed(const QList<QRectF> &region);
    void sceneRectChanged(const QRectF &rect);
    void selectionChanged();
    void focusItemChanged(QGraphicsItem *newFocus, QGraphicsItem *oldFocus, Qt::FocusReason reason);

private:
    Q_DECLARE_PRIVATE(QGraphicsScene)
    Q_DISABLE_COPY(QGraphicsScene)
    Q_PRIVATE_SLOT(d_func(), void _q_emitUpdated())
    Q_PRIVATE_SLOT(d_func(), void _q_polishItems())
    Q_PRIVATE_SLOT(d_func(), void _q_processDirtyItems())
    Q_PRIVATE_SLOT(d_func(), void _q_updateScenePosDescendants())
    friend class QGraphicsItem;
    friend class QGraphicsItemPrivate;
    friend class QGraphicsObject;
    friend class QGraphicsView;
    friend class QGraphicsViewPrivate;
    friend class QGraphicsWidget;
    friend class QGraphicsWidgetPrivate;
    friend class QGraphicsEffect;
    friend class QGraphicsSceneIndex;
    friend class QGraphicsSceneIndexPrivate;
    friend class QGraphicsSceneBspTreeIndex;
    friend class QGraphicsSceneBspTreeIndexPrivate;
    friend class QGraphicsItemEffectSourcePrivate;
#ifndef QT_NO_GESTURES
    friend class QGesture;
#endif
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QGraphicsScene::SceneLayers)

QT_END_NAMESPACE

#endif
