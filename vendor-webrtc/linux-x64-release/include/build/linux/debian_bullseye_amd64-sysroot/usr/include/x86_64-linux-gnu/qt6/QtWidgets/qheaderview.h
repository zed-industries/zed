// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QHEADERVIEW_H
#define QHEADERVIEW_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtWidgets/qabstractitemview.h>

QT_REQUIRE_CONFIG(itemviews);

QT_BEGIN_NAMESPACE

class QHeaderViewPrivate;
class QStyleOptionHeader;

class Q_WIDGETS_EXPORT QHeaderView : public QAbstractItemView
{
    Q_OBJECT
    Q_PROPERTY(bool firstSectionMovable READ isFirstSectionMovable WRITE setFirstSectionMovable)
    Q_PROPERTY(bool showSortIndicator READ isSortIndicatorShown WRITE setSortIndicatorShown)
    Q_PROPERTY(bool highlightSections READ highlightSections WRITE setHighlightSections)
    Q_PROPERTY(bool stretchLastSection READ stretchLastSection WRITE setStretchLastSection)
    Q_PROPERTY(bool cascadingSectionResizes READ cascadingSectionResizes
               WRITE setCascadingSectionResizes)
    Q_PROPERTY(int defaultSectionSize READ defaultSectionSize WRITE setDefaultSectionSize
               RESET resetDefaultSectionSize)
    Q_PROPERTY(int minimumSectionSize READ minimumSectionSize WRITE setMinimumSectionSize)
    Q_PROPERTY(int maximumSectionSize READ maximumSectionSize WRITE setMaximumSectionSize)
    Q_PROPERTY(Qt::Alignment defaultAlignment READ defaultAlignment WRITE setDefaultAlignment)
    Q_PROPERTY(bool sortIndicatorClearable READ isSortIndicatorClearable
               WRITE setSortIndicatorClearable NOTIFY sortIndicatorClearableChanged)

public:

    enum ResizeMode
    {
        Interactive,
        Stretch,
        Fixed,
        ResizeToContents,
        Custom = Fixed
    };
    Q_ENUM(ResizeMode)

    explicit QHeaderView(Qt::Orientation orientation, QWidget *parent = nullptr);
    virtual ~QHeaderView();

    void setModel(QAbstractItemModel *model) override;

    Qt::Orientation orientation() const;
    int offset() const;
    int length() const;
    QSize sizeHint() const override;
    void setVisible(bool v) override;
    int sectionSizeHint(int logicalIndex) const;

    int visualIndexAt(int position) const;
    int logicalIndexAt(int position) const;

    inline int logicalIndexAt(int x, int y) const;
    inline int logicalIndexAt(const QPoint &pos) const;

    int sectionSize(int logicalIndex) const;
    int sectionPosition(int logicalIndex) const;
    int sectionViewportPosition(int logicalIndex) const;

    void moveSection(int from, int to);
    void swapSections(int first, int second);
    void resizeSection(int logicalIndex, int size);
    void resizeSections(QHeaderView::ResizeMode mode);

    bool isSectionHidden(int logicalIndex) const;
    void setSectionHidden(int logicalIndex, bool hide);
    int hiddenSectionCount() const;

    inline void hideSection(int logicalIndex);
    inline void showSection(int logicalIndex);

    int count() const;
    int visualIndex(int logicalIndex) const;
    int logicalIndex(int visualIndex) const;

    void setSectionsMovable(bool movable);
    bool sectionsMovable() const;
    void setFirstSectionMovable(bool movable);
    bool isFirstSectionMovable() const;

    void setSectionsClickable(bool clickable);
    bool sectionsClickable() const;

    void setHighlightSections(bool highlight);
    bool highlightSections() const;

    ResizeMode sectionResizeMode(int logicalIndex) const;
    void setSectionResizeMode(ResizeMode mode);
    void setSectionResizeMode(int logicalIndex, ResizeMode mode);

    void setResizeContentsPrecision(int precision);
    int  resizeContentsPrecision() const;

    int stretchSectionCount() const;

    void setSortIndicatorShown(bool show);
    bool isSortIndicatorShown() const;

    void setSortIndicator(int logicalIndex, Qt::SortOrder order);
    int sortIndicatorSection() const;
    Qt::SortOrder sortIndicatorOrder() const;

    void setSortIndicatorClearable(bool clearable);
    bool isSortIndicatorClearable() const;

    bool stretchLastSection() const;
    void setStretchLastSection(bool stretch);

    bool cascadingSectionResizes() const;
    void setCascadingSectionResizes(bool enable);

    int defaultSectionSize() const;
    void setDefaultSectionSize(int size);
    void resetDefaultSectionSize();

    int minimumSectionSize() const;
    void setMinimumSectionSize(int size);
    int maximumSectionSize() const;
    void setMaximumSectionSize(int size);

    Qt::Alignment defaultAlignment() const;
    void setDefaultAlignment(Qt::Alignment alignment);

    void doItemsLayout() override;
    bool sectionsMoved() const;
    bool sectionsHidden() const;

#ifndef QT_NO_DATASTREAM
    QByteArray saveState() const;
    bool restoreState(const QByteArray &state);
#endif

    void reset() override;

public Q_SLOTS:
    void setOffset(int offset);
    void setOffsetToSectionPosition(int visualIndex);
    void setOffsetToLastSection();
    void headerDataChanged(Qt::Orientation orientation, int logicalFirst, int logicalLast);

Q_SIGNALS:
    void sectionMoved(int logicalIndex, int oldVisualIndex, int newVisualIndex);
    void sectionResized(int logicalIndex, int oldSize, int newSize);
    void sectionPressed(int logicalIndex);
    void sectionClicked(int logicalIndex);
    void sectionEntered(int logicalIndex);
    void sectionDoubleClicked(int logicalIndex);
    void sectionCountChanged(int oldCount, int newCount);
    void sectionHandleDoubleClicked(int logicalIndex);
    void geometriesChanged();
    void sortIndicatorChanged(int logicalIndex, Qt::SortOrder order);
    void sortIndicatorClearableChanged(bool clearable);

protected Q_SLOTS:
    void updateSection(int logicalIndex);
    void resizeSections();
    void sectionsInserted(const QModelIndex &parent, int logicalFirst, int logicalLast);
    void sectionsAboutToBeRemoved(const QModelIndex &parent, int logicalFirst, int logicalLast);

protected:
    QHeaderView(QHeaderViewPrivate &dd, Qt::Orientation orientation, QWidget *parent = nullptr);
    void initialize();

    void initializeSections();
    void initializeSections(int start, int end);
    void currentChanged(const QModelIndex &current, const QModelIndex &old) override;

    bool event(QEvent *e) override;
    void paintEvent(QPaintEvent *e) override;
    void mousePressEvent(QMouseEvent *e) override;
    void mouseMoveEvent(QMouseEvent *e) override;
    void mouseReleaseEvent(QMouseEvent *e) override;
    void mouseDoubleClickEvent(QMouseEvent *e) override;
    bool viewportEvent(QEvent *e) override;

    virtual void paintSection(QPainter *painter, const QRect &rect, int logicalIndex) const;
    virtual QSize sectionSizeFromContents(int logicalIndex) const;

    int horizontalOffset() const override;
    int verticalOffset() const override;
    void updateGeometries() override;
    void scrollContentsBy(int dx, int dy) override;

    void dataChanged(const QModelIndex &topLeft, const QModelIndex &bottomRight,
                     const QList<int> &roles = QList<int>()) override;
    void rowsInserted(const QModelIndex &parent, int start, int end) override;

    QRect visualRect(const QModelIndex &index) const override;
    void scrollTo(const QModelIndex &index, ScrollHint hint) override;

    QModelIndex indexAt(const QPoint &p) const override;
    bool isIndexHidden(const QModelIndex &index) const override;

    QModelIndex moveCursor(CursorAction, Qt::KeyboardModifiers) override;
    void setSelection(const QRect& rect, QItemSelectionModel::SelectionFlags flags) override;
    QRegion visualRegionForSelection(const QItemSelection &selection) const override;
    virtual void initStyleOptionForIndex(QStyleOptionHeader *option, int logicalIndex) const;
    virtual void initStyleOption(QStyleOptionHeader *option) const;

    friend class QTableView;
    friend class QTreeView;

private:
    void initStyleOption(QStyleOptionFrame *option) const override;

    // ### Qt6: make them protected slots in QHeaderViewPrivate
    Q_PRIVATE_SLOT(d_func(), void _q_sectionsRemoved(const QModelIndex &parent, int logicalFirst, int logicalLast))
    Q_PRIVATE_SLOT(d_func(), void _q_sectionsAboutToBeMoved(const QModelIndex &sourceParent, int logicalStart, int logicalEnd, const QModelIndex &destinationParent, int logicalDestination))
    Q_PRIVATE_SLOT(d_func(), void _q_sectionsMoved(const QModelIndex &sourceParent, int logicalStart, int logicalEnd, const QModelIndex &destinationParent, int logicalDestination))
    Q_PRIVATE_SLOT(d_func(), void _q_sectionsAboutToBeChanged(const QList<QPersistentModelIndex> &parents = QList<QPersistentModelIndex>(),
                                                              QAbstractItemModel::LayoutChangeHint hint = QAbstractItemModel::NoLayoutChangeHint))
    Q_PRIVATE_SLOT(d_func(), void _q_sectionsChanged(const QList<QPersistentModelIndex> &parents = QList<QPersistentModelIndex>(),
                                                     QAbstractItemModel::LayoutChangeHint hint = QAbstractItemModel::NoLayoutChangeHint))
    Q_DECLARE_PRIVATE(QHeaderView)
    Q_DISABLE_COPY(QHeaderView)
};

inline int QHeaderView::logicalIndexAt(int ax, int ay) const
{ return orientation() == Qt::Horizontal ? logicalIndexAt(ax) : logicalIndexAt(ay); }
inline int QHeaderView::logicalIndexAt(const QPoint &apos) const
{ return logicalIndexAt(apos.x(), apos.y()); }
inline void QHeaderView::hideSection(int alogicalIndex)
{ setSectionHidden(alogicalIndex, true); }
inline void QHeaderView::showSection(int alogicalIndex)
{ setSectionHidden(alogicalIndex, false); }

QT_END_NAMESPACE

#endif // QHEADERVIEW_H
