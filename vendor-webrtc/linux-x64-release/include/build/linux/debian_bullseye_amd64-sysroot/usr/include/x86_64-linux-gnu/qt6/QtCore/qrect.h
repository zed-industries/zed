// Copyright (C) 2022 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QRECT_H
#define QRECT_H

#include <QtCore/qhashfunctions.h>
#include <QtCore/qmargins.h>
#include <QtCore/qsize.h>
#include <QtCore/qpoint.h>

#ifdef topLeft
#error qrect.h must be included before any header file that defines topLeft
#endif

#if defined(Q_OS_DARWIN) || defined(Q_QDOC)
struct CGRect;
#endif

QT_BEGIN_NAMESPACE

class QRectF;

class Q_CORE_EXPORT QRect
{
public:
    constexpr QRect() noexcept : x1(0), y1(0), x2(-1), y2(-1) {}
    constexpr QRect(const QPoint &topleft, const QPoint &bottomright) noexcept;
    constexpr QRect(const QPoint &topleft, const QSize &size) noexcept;
    constexpr QRect(int left, int top, int width, int height) noexcept;

    constexpr inline bool isNull() const noexcept;
    constexpr inline bool isEmpty() const noexcept;
    constexpr inline bool isValid() const noexcept;

    constexpr inline int left() const noexcept;
    constexpr inline int top() const noexcept;
    constexpr inline int right() const noexcept;
    constexpr inline int bottom() const noexcept;
    [[nodiscard]] QRect normalized() const noexcept;

    constexpr inline int x() const noexcept;
    constexpr inline int y() const noexcept;
    constexpr inline void setLeft(int pos) noexcept;
    constexpr inline void setTop(int pos) noexcept;
    constexpr inline void setRight(int pos) noexcept;
    constexpr inline void setBottom(int pos) noexcept;
    constexpr inline void setX(int x) noexcept;
    constexpr inline void setY(int y) noexcept;

    constexpr inline void setTopLeft(const QPoint &p) noexcept;
    constexpr inline void setBottomRight(const QPoint &p) noexcept;
    constexpr inline void setTopRight(const QPoint &p) noexcept;
    constexpr inline void setBottomLeft(const QPoint &p) noexcept;

    constexpr inline QPoint topLeft() const noexcept;
    constexpr inline QPoint bottomRight() const noexcept;
    constexpr inline QPoint topRight() const noexcept;
    constexpr inline QPoint bottomLeft() const noexcept;
    constexpr inline QPoint center() const noexcept;

    constexpr inline void moveLeft(int pos) noexcept;
    constexpr inline void moveTop(int pos) noexcept;
    constexpr inline void moveRight(int pos) noexcept;
    constexpr inline void moveBottom(int pos) noexcept;
    constexpr inline void moveTopLeft(const QPoint &p) noexcept;
    constexpr inline void moveBottomRight(const QPoint &p) noexcept;
    constexpr inline void moveTopRight(const QPoint &p) noexcept;
    constexpr inline void moveBottomLeft(const QPoint &p) noexcept;
    constexpr inline void moveCenter(const QPoint &p) noexcept;

    constexpr inline void translate(int dx, int dy) noexcept;
    constexpr inline void translate(const QPoint &p) noexcept;
    [[nodiscard]] constexpr inline QRect translated(int dx, int dy) const noexcept;
    [[nodiscard]] constexpr inline QRect translated(const QPoint &p) const noexcept;
    [[nodiscard]] constexpr inline QRect transposed() const noexcept;

    constexpr inline void moveTo(int x, int t) noexcept;
    constexpr inline void moveTo(const QPoint &p) noexcept;

    constexpr inline void setRect(int x, int y, int w, int h) noexcept;
    constexpr inline void getRect(int *x, int *y, int *w, int *h) const;

    constexpr inline void setCoords(int x1, int y1, int x2, int y2) noexcept;
    constexpr inline void getCoords(int *x1, int *y1, int *x2, int *y2) const;

    constexpr inline void adjust(int x1, int y1, int x2, int y2) noexcept;
    [[nodiscard]] constexpr inline QRect adjusted(int x1, int y1, int x2, int y2) const noexcept;

    constexpr inline QSize size() const noexcept;
    constexpr inline int width() const noexcept;
    constexpr inline int height() const noexcept;
    constexpr inline void setWidth(int w) noexcept;
    constexpr inline void setHeight(int h) noexcept;
    constexpr inline void setSize(const QSize &s) noexcept;

    QRect operator|(const QRect &r) const noexcept;
    QRect operator&(const QRect &r) const noexcept;
    inline QRect &operator|=(const QRect &r) noexcept;
    inline QRect &operator&=(const QRect &r) noexcept;

    bool contains(const QRect &r, bool proper = false) const noexcept;
    bool contains(const QPoint &p, bool proper = false) const noexcept;
    inline bool contains(int x, int y) const noexcept;
    inline bool contains(int x, int y, bool proper) const noexcept;
    [[nodiscard]] inline QRect united(const QRect &other) const noexcept;
    [[nodiscard]] inline QRect intersected(const QRect &other) const noexcept;
    bool intersects(const QRect &r) const noexcept;

    constexpr inline QRect marginsAdded(const QMargins &margins) const noexcept;
    constexpr inline QRect marginsRemoved(const QMargins &margins) const noexcept;
    constexpr inline QRect &operator+=(const QMargins &margins) noexcept;
    constexpr inline QRect &operator-=(const QMargins &margins) noexcept;

    [[nodiscard]] static constexpr inline QRect span(const QPoint &p1, const QPoint &p2) noexcept;

    friend constexpr inline bool operator==(const QRect &r1, const QRect &r2) noexcept
    { return r1.x1==r2.x1 && r1.x2==r2.x2 && r1.y1==r2.y1 && r1.y2==r2.y2; }
    friend constexpr inline bool operator!=(const QRect &r1, const QRect &r2) noexcept
    { return r1.x1!=r2.x1 || r1.x2!=r2.x2 || r1.y1!=r2.y1 || r1.y2!=r2.y2; }
    friend constexpr inline size_t qHash(const QRect &, size_t) noexcept;

#if defined(Q_OS_DARWIN) || defined(Q_QDOC)
    [[nodiscard]] CGRect toCGRect() const noexcept;
#endif
    [[nodiscard]] constexpr inline QRectF toRectF() const noexcept;

private:
    int x1;
    int y1;
    int x2;
    int y2;
};
Q_DECLARE_TYPEINFO(QRect, Q_RELOCATABLE_TYPE);


/*****************************************************************************
  QRect stream functions
 *****************************************************************************/
#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QRect &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QRect &);
#endif

/*****************************************************************************
  QRect inline member functions
 *****************************************************************************/

constexpr inline QRect::QRect(int aleft, int atop, int awidth, int aheight) noexcept
    : x1(aleft), y1(atop), x2(aleft + awidth - 1), y2(atop + aheight - 1) {}

constexpr inline QRect::QRect(const QPoint &atopLeft, const QPoint &abottomRight) noexcept
    : x1(atopLeft.x()), y1(atopLeft.y()), x2(abottomRight.x()), y2(abottomRight.y()) {}

constexpr inline QRect::QRect(const QPoint &atopLeft, const QSize &asize) noexcept
    : x1(atopLeft.x()), y1(atopLeft.y()), x2(atopLeft.x()+asize.width() - 1), y2(atopLeft.y()+asize.height() - 1) {}

constexpr inline bool QRect::isNull() const noexcept
{ return x2 == x1 - 1 && y2 == y1 - 1; }

constexpr inline bool QRect::isEmpty() const noexcept
{ return x1 > x2 || y1 > y2; }

constexpr inline bool QRect::isValid() const noexcept
{ return x1 <= x2 && y1 <= y2; }

constexpr inline int QRect::left() const noexcept
{ return x1; }

constexpr inline int QRect::top() const noexcept
{ return y1; }

constexpr inline int QRect::right() const noexcept
{ return x2; }

constexpr inline int QRect::bottom() const noexcept
{ return y2; }

constexpr inline int QRect::x() const noexcept
{ return x1; }

constexpr inline int QRect::y() const noexcept
{ return y1; }

constexpr inline void QRect::setLeft(int pos) noexcept
{ x1 = pos; }

constexpr inline void QRect::setTop(int pos) noexcept
{ y1 = pos; }

constexpr inline void QRect::setRight(int pos) noexcept
{ x2 = pos; }

constexpr inline void QRect::setBottom(int pos) noexcept
{ y2 = pos; }

constexpr inline void QRect::setTopLeft(const QPoint &p) noexcept
{ x1 = p.x(); y1 = p.y(); }

constexpr inline void QRect::setBottomRight(const QPoint &p) noexcept
{ x2 = p.x(); y2 = p.y(); }

constexpr inline void QRect::setTopRight(const QPoint &p) noexcept
{ x2 = p.x(); y1 = p.y(); }

constexpr inline void QRect::setBottomLeft(const QPoint &p) noexcept
{ x1 = p.x(); y2 = p.y(); }

constexpr inline void QRect::setX(int ax) noexcept
{ x1 = ax; }

constexpr inline void QRect::setY(int ay) noexcept
{ y1 = ay; }

constexpr inline QPoint QRect::topLeft() const noexcept
{ return QPoint(x1, y1); }

constexpr inline QPoint QRect::bottomRight() const noexcept
{ return QPoint(x2, y2); }

constexpr inline QPoint QRect::topRight() const noexcept
{ return QPoint(x2, y1); }

constexpr inline QPoint QRect::bottomLeft() const noexcept
{ return QPoint(x1, y2); }

constexpr inline QPoint QRect::center() const noexcept
{ return QPoint(int((qint64(x1)+x2)/2), int((qint64(y1)+y2)/2)); } // cast avoids overflow on addition

constexpr inline int QRect::width() const noexcept
{ return  x2 - x1 + 1; }

constexpr inline int QRect::height() const noexcept
{ return  y2 - y1 + 1; }

constexpr inline QSize QRect::size() const noexcept
{ return QSize(width(), height()); }

constexpr inline void QRect::translate(int dx, int dy) noexcept
{
    x1 += dx;
    y1 += dy;
    x2 += dx;
    y2 += dy;
}

constexpr inline void QRect::translate(const QPoint &p) noexcept
{
    x1 += p.x();
    y1 += p.y();
    x2 += p.x();
    y2 += p.y();
}

constexpr inline QRect QRect::translated(int dx, int dy) const noexcept
{ return QRect(QPoint(x1 + dx, y1 + dy), QPoint(x2 + dx, y2 + dy)); }

constexpr inline QRect QRect::translated(const QPoint &p) const noexcept
{ return QRect(QPoint(x1 + p.x(), y1 + p.y()), QPoint(x2 + p.x(), y2 + p.y())); }

constexpr inline QRect QRect::transposed() const noexcept
{ return QRect(topLeft(), size().transposed()); }

constexpr inline void QRect::moveTo(int ax, int ay) noexcept
{
    x2 += ax - x1;
    y2 += ay - y1;
    x1 = ax;
    y1 = ay;
}

constexpr inline void QRect::moveTo(const QPoint &p) noexcept
{
    x2 += p.x() - x1;
    y2 += p.y() - y1;
    x1 = p.x();
    y1 = p.y();
}

constexpr inline void QRect::moveLeft(int pos) noexcept
{ x2 += (pos - x1); x1 = pos; }

constexpr inline void QRect::moveTop(int pos) noexcept
{ y2 += (pos - y1); y1 = pos; }

constexpr inline void QRect::moveRight(int pos) noexcept
{
    x1 += (pos - x2);
    x2 = pos;
}

constexpr inline void QRect::moveBottom(int pos) noexcept
{
    y1 += (pos - y2);
    y2 = pos;
}

constexpr inline void QRect::moveTopLeft(const QPoint &p) noexcept
{
    moveLeft(p.x());
    moveTop(p.y());
}

constexpr inline void QRect::moveBottomRight(const QPoint &p) noexcept
{
    moveRight(p.x());
    moveBottom(p.y());
}

constexpr inline void QRect::moveTopRight(const QPoint &p) noexcept
{
    moveRight(p.x());
    moveTop(p.y());
}

constexpr inline void QRect::moveBottomLeft(const QPoint &p) noexcept
{
    moveLeft(p.x());
    moveBottom(p.y());
}

constexpr inline void QRect::moveCenter(const QPoint &p) noexcept
{
    int w = x2 - x1;
    int h = y2 - y1;
    x1 = p.x() - w/2;
    y1 = p.y() - h/2;
    x2 = x1 + w;
    y2 = y1 + h;
}

constexpr inline void QRect::getRect(int *ax, int *ay, int *aw, int *ah) const
{
    *ax = x1;
    *ay = y1;
    *aw = x2 - x1 + 1;
    *ah = y2 - y1 + 1;
}

constexpr inline void QRect::setRect(int ax, int ay, int aw, int ah) noexcept
{
    x1 = ax;
    y1 = ay;
    x2 = (ax + aw - 1);
    y2 = (ay + ah - 1);
}

constexpr inline void QRect::getCoords(int *xp1, int *yp1, int *xp2, int *yp2) const
{
    *xp1 = x1;
    *yp1 = y1;
    *xp2 = x2;
    *yp2 = y2;
}

constexpr inline void QRect::setCoords(int xp1, int yp1, int xp2, int yp2) noexcept
{
    x1 = xp1;
    y1 = yp1;
    x2 = xp2;
    y2 = yp2;
}

constexpr inline QRect QRect::adjusted(int xp1, int yp1, int xp2, int yp2) const noexcept
{ return QRect(QPoint(x1 + xp1, y1 + yp1), QPoint(x2 + xp2, y2 + yp2)); }

constexpr inline void QRect::adjust(int dx1, int dy1, int dx2, int dy2) noexcept
{
    x1 += dx1;
    y1 += dy1;
    x2 += dx2;
    y2 += dy2;
}

constexpr inline void QRect::setWidth(int w) noexcept
{ x2 = (x1 + w - 1); }

constexpr inline void QRect::setHeight(int h) noexcept
{ y2 = (y1 + h - 1); }

constexpr inline void QRect::setSize(const QSize &s) noexcept
{
    x2 = (s.width()  + x1 - 1);
    y2 = (s.height() + y1 - 1);
}

inline bool QRect::contains(int ax, int ay, bool aproper) const noexcept
{
    return contains(QPoint(ax, ay), aproper);
}

inline bool QRect::contains(int ax, int ay) const noexcept
{
    return contains(QPoint(ax, ay), false);
}

inline QRect &QRect::operator|=(const QRect &r) noexcept
{
    *this = *this | r;
    return *this;
}

inline QRect &QRect::operator&=(const QRect &r) noexcept
{
    *this = *this & r;
    return *this;
}

inline QRect QRect::intersected(const QRect &other) const noexcept
{
    return *this & other;
}

inline QRect QRect::united(const QRect &r) const noexcept
{
    return *this | r;
}

constexpr inline size_t qHash(const QRect &r, size_t seed = 0) noexcept
{
    return qHashMulti(seed, r.x1, r.x2, r.y1, r.y2);
}

constexpr inline QRect operator+(const QRect &rectangle, const QMargins &margins) noexcept
{
    return QRect(QPoint(rectangle.left() - margins.left(), rectangle.top() - margins.top()),
                 QPoint(rectangle.right() + margins.right(), rectangle.bottom() + margins.bottom()));
}

constexpr inline QRect operator+(const QMargins &margins, const QRect &rectangle) noexcept
{
    return QRect(QPoint(rectangle.left() - margins.left(), rectangle.top() - margins.top()),
                 QPoint(rectangle.right() + margins.right(), rectangle.bottom() + margins.bottom()));
}

constexpr inline QRect operator-(const QRect &lhs, const QMargins &rhs) noexcept
{
    return QRect(QPoint(lhs.left() + rhs.left(), lhs.top() + rhs.top()),
                 QPoint(lhs.right() - rhs.right(), lhs.bottom() - rhs.bottom()));
}

constexpr inline QRect QRect::marginsAdded(const QMargins &margins) const noexcept
{
    return QRect(QPoint(x1 - margins.left(), y1 - margins.top()),
                 QPoint(x2 + margins.right(), y2 + margins.bottom()));
}

constexpr inline QRect QRect::marginsRemoved(const QMargins &margins) const noexcept
{
    return QRect(QPoint(x1 + margins.left(), y1 + margins.top()),
                 QPoint(x2 - margins.right(), y2 - margins.bottom()));
}

constexpr inline QRect &QRect::operator+=(const QMargins &margins) noexcept
{
    *this = marginsAdded(margins);
    return *this;
}

constexpr inline QRect &QRect::operator-=(const QMargins &margins) noexcept
{
    *this = marginsRemoved(margins);
    return *this;
}

constexpr QRect QRect::span(const QPoint &p1, const QPoint &p2) noexcept
{
    return QRect(QPoint(qMin(p1.x(), p2.x()), qMin(p1.y(), p2.y())),
                 QPoint(qMax(p1.x(), p2.x()), qMax(p1.y(), p2.y())));
}

#ifndef QT_NO_DEBUG_STREAM
Q_CORE_EXPORT QDebug operator<<(QDebug, const QRect &);
#endif


class Q_CORE_EXPORT QRectF
{
public:
    constexpr QRectF() noexcept : xp(0.), yp(0.), w(0.), h(0.) {}
    constexpr QRectF(const QPointF &topleft, const QSizeF &size) noexcept;
    constexpr QRectF(const QPointF &topleft, const QPointF &bottomRight) noexcept;
    constexpr QRectF(qreal left, qreal top, qreal width, qreal height) noexcept;
    constexpr QRectF(const QRect &rect) noexcept;

    constexpr inline bool isNull() const noexcept;
    constexpr inline bool isEmpty() const noexcept;
    constexpr inline bool isValid() const noexcept;
    [[nodiscard]] QRectF normalized() const noexcept;

    constexpr inline qreal left() const noexcept { return xp; }
    constexpr inline qreal top() const noexcept { return yp; }
    constexpr inline qreal right() const noexcept { return xp + w; }
    constexpr inline qreal bottom() const noexcept { return yp + h; }

    constexpr inline qreal x() const noexcept;
    constexpr inline qreal y() const noexcept;
    constexpr inline void setLeft(qreal pos) noexcept;
    constexpr inline void setTop(qreal pos) noexcept;
    constexpr inline void setRight(qreal pos) noexcept;
    constexpr inline void setBottom(qreal pos) noexcept;
    constexpr inline void setX(qreal pos) noexcept { setLeft(pos); }
    constexpr inline void setY(qreal pos) noexcept { setTop(pos); }

    constexpr inline QPointF topLeft() const noexcept { return QPointF(xp, yp); }
    constexpr inline QPointF bottomRight() const noexcept { return QPointF(xp+w, yp+h); }
    constexpr inline QPointF topRight() const noexcept { return QPointF(xp+w, yp); }
    constexpr inline QPointF bottomLeft() const noexcept { return QPointF(xp, yp+h); }
    constexpr inline QPointF center() const noexcept;

    constexpr inline void setTopLeft(const QPointF &p) noexcept;
    constexpr inline void setBottomRight(const QPointF &p) noexcept;
    constexpr inline void setTopRight(const QPointF &p) noexcept;
    constexpr inline void setBottomLeft(const QPointF &p) noexcept;

    constexpr inline void moveLeft(qreal pos) noexcept;
    constexpr inline void moveTop(qreal pos) noexcept;
    constexpr inline void moveRight(qreal pos) noexcept;
    constexpr inline void moveBottom(qreal pos) noexcept;
    constexpr inline void moveTopLeft(const QPointF &p) noexcept;
    constexpr inline void moveBottomRight(const QPointF &p) noexcept;
    constexpr inline void moveTopRight(const QPointF &p) noexcept;
    constexpr inline void moveBottomLeft(const QPointF &p) noexcept;
    constexpr inline void moveCenter(const QPointF &p) noexcept;

    constexpr inline void translate(qreal dx, qreal dy) noexcept;
    constexpr inline void translate(const QPointF &p) noexcept;

    [[nodiscard]] constexpr inline QRectF translated(qreal dx, qreal dy) const noexcept;
    [[nodiscard]] constexpr inline QRectF translated(const QPointF &p) const noexcept;

    [[nodiscard]] constexpr inline QRectF transposed() const noexcept;

    constexpr inline void moveTo(qreal x, qreal y) noexcept;
    constexpr inline void moveTo(const QPointF &p) noexcept;

    constexpr inline void setRect(qreal x, qreal y, qreal w, qreal h) noexcept;
    constexpr inline void getRect(qreal *x, qreal *y, qreal *w, qreal *h) const;

    constexpr inline void setCoords(qreal x1, qreal y1, qreal x2, qreal y2) noexcept;
    constexpr inline void getCoords(qreal *x1, qreal *y1, qreal *x2, qreal *y2) const;

    constexpr inline void adjust(qreal x1, qreal y1, qreal x2, qreal y2) noexcept;
    [[nodiscard]] constexpr inline QRectF adjusted(qreal x1, qreal y1, qreal x2, qreal y2) const noexcept;

    constexpr inline QSizeF size() const noexcept;
    constexpr inline qreal width() const noexcept;
    constexpr inline qreal height() const noexcept;
    constexpr inline void setWidth(qreal w) noexcept;
    constexpr inline void setHeight(qreal h) noexcept;
    constexpr inline void setSize(const QSizeF &s) noexcept;

    QRectF operator|(const QRectF &r) const noexcept;
    QRectF operator&(const QRectF &r) const noexcept;
    inline QRectF &operator|=(const QRectF &r) noexcept;
    inline QRectF &operator&=(const QRectF &r) noexcept;

    bool contains(const QRectF &r) const noexcept;
    bool contains(const QPointF &p) const noexcept;
    inline bool contains(qreal x, qreal y) const noexcept;
    [[nodiscard]] inline QRectF united(const QRectF &other) const noexcept;
    [[nodiscard]] inline QRectF intersected(const QRectF &other) const noexcept;
    bool intersects(const QRectF &r) const noexcept;

    constexpr inline QRectF marginsAdded(const QMarginsF &margins) const noexcept;
    constexpr inline QRectF marginsRemoved(const QMarginsF &margins) const noexcept;
    constexpr inline QRectF &operator+=(const QMarginsF &margins) noexcept;
    constexpr inline QRectF &operator-=(const QMarginsF &margins) noexcept;

    friend constexpr inline bool operator==(const QRectF &r1, const QRectF &r2) noexcept
    {
        return r1.topLeft() == r2.topLeft()
            && r1.size() == r2.size();
    }
    friend constexpr inline bool operator!=(const QRectF &r1, const QRectF &r2) noexcept
    {
        return r1.topLeft() != r2.topLeft()
            || r1.size() != r2.size();
    }

    [[nodiscard]] constexpr inline QRect toRect() const noexcept;
    [[nodiscard]] QRect toAlignedRect() const noexcept;

#if defined(Q_OS_DARWIN) || defined(Q_QDOC)
    [[nodiscard]] static QRectF fromCGRect(CGRect rect) noexcept;
    [[nodiscard]] CGRect toCGRect() const noexcept;
#endif

private:
    qreal xp;
    qreal yp;
    qreal w;
    qreal h;
};
Q_DECLARE_TYPEINFO(QRectF, Q_RELOCATABLE_TYPE);


/*****************************************************************************
  QRectF stream functions
 *****************************************************************************/
#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QRectF &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QRectF &);
#endif

/*****************************************************************************
  QRectF inline member functions
 *****************************************************************************/

constexpr inline QRectF::QRectF(qreal aleft, qreal atop, qreal awidth, qreal aheight) noexcept
    : xp(aleft), yp(atop), w(awidth), h(aheight)
{
}

constexpr inline QRectF::QRectF(const QPointF &atopLeft, const QSizeF &asize) noexcept
    : xp(atopLeft.x()), yp(atopLeft.y()), w(asize.width()), h(asize.height())
{
}


constexpr inline QRectF::QRectF(const QPointF &atopLeft, const QPointF &abottomRight) noexcept
    : xp(atopLeft.x()), yp(atopLeft.y()), w(abottomRight.x() - atopLeft.x()), h(abottomRight.y() - atopLeft.y())
{
}

constexpr inline QRectF::QRectF(const QRect &r) noexcept
    : xp(r.x()), yp(r.y()), w(r.width()), h(r.height())
{
}

QT_WARNING_PUSH
QT_WARNING_DISABLE_FLOAT_COMPARE

constexpr inline bool QRectF::isNull() const noexcept
{ return w == 0. && h == 0.; }

constexpr inline bool QRectF::isEmpty() const noexcept
{ return w <= 0. || h <= 0.; }

QT_WARNING_POP

constexpr inline bool QRectF::isValid() const noexcept
{ return w > 0. && h > 0.; }

constexpr inline qreal QRectF::x() const noexcept
{ return xp; }

constexpr inline qreal QRectF::y() const noexcept
{ return yp; }

constexpr inline void QRectF::setLeft(qreal pos) noexcept
{ qreal diff = pos - xp; xp += diff; w -= diff; }

constexpr inline void QRectF::setRight(qreal pos) noexcept
{ w = pos - xp; }

constexpr inline void QRectF::setTop(qreal pos) noexcept
{ qreal diff = pos - yp; yp += diff; h -= diff; }

constexpr inline void QRectF::setBottom(qreal pos) noexcept
{ h = pos - yp; }

constexpr inline void QRectF::setTopLeft(const QPointF &p) noexcept
{ setLeft(p.x()); setTop(p.y()); }

constexpr inline void QRectF::setTopRight(const QPointF &p) noexcept
{ setRight(p.x()); setTop(p.y()); }

constexpr inline void QRectF::setBottomLeft(const QPointF &p) noexcept
{ setLeft(p.x()); setBottom(p.y()); }

constexpr inline void QRectF::setBottomRight(const QPointF &p) noexcept
{ setRight(p.x()); setBottom(p.y()); }

constexpr inline QPointF QRectF::center() const noexcept
{ return QPointF(xp + w/2, yp + h/2); }

constexpr inline void QRectF::moveLeft(qreal pos) noexcept
{ xp = pos; }

constexpr inline void QRectF::moveTop(qreal pos) noexcept
{ yp = pos; }

constexpr inline void QRectF::moveRight(qreal pos) noexcept
{ xp = pos - w; }

constexpr inline void QRectF::moveBottom(qreal pos) noexcept
{ yp = pos - h; }

constexpr inline void QRectF::moveTopLeft(const QPointF &p) noexcept
{ moveLeft(p.x()); moveTop(p.y()); }

constexpr inline void QRectF::moveTopRight(const QPointF &p) noexcept
{ moveRight(p.x()); moveTop(p.y()); }

constexpr inline void QRectF::moveBottomLeft(const QPointF &p) noexcept
{ moveLeft(p.x()); moveBottom(p.y()); }

constexpr inline void QRectF::moveBottomRight(const QPointF &p) noexcept
{ moveRight(p.x()); moveBottom(p.y()); }

constexpr inline void QRectF::moveCenter(const QPointF &p) noexcept
{ xp = p.x() - w/2; yp = p.y() - h/2; }

constexpr inline qreal QRectF::width() const noexcept
{ return w; }

constexpr inline qreal QRectF::height() const noexcept
{ return h; }

constexpr inline QSizeF QRectF::size() const noexcept
{ return QSizeF(w, h); }

constexpr inline void QRectF::translate(qreal dx, qreal dy) noexcept
{
    xp += dx;
    yp += dy;
}

constexpr inline void QRectF::translate(const QPointF &p) noexcept
{
    xp += p.x();
    yp += p.y();
}

constexpr inline void QRectF::moveTo(qreal ax, qreal ay) noexcept
{
    xp = ax;
    yp = ay;
}

constexpr inline void QRectF::moveTo(const QPointF &p) noexcept
{
    xp = p.x();
    yp = p.y();
}

constexpr inline QRectF QRectF::translated(qreal dx, qreal dy) const noexcept
{
    return QRectF(xp + dx, yp + dy, w, h);
}

constexpr inline QRectF QRectF::translated(const QPointF &p) const noexcept
{ return QRectF(xp + p.x(), yp + p.y(), w, h); }

constexpr inline QRectF QRectF::transposed() const noexcept
{ return QRectF(topLeft(), size().transposed()); }

constexpr inline void QRectF::getRect(qreal *ax, qreal *ay, qreal *aaw, qreal *aah) const
{
    *ax = this->xp;
    *ay = this->yp;
    *aaw = this->w;
    *aah = this->h;
}

constexpr inline void QRectF::setRect(qreal ax, qreal ay, qreal aaw, qreal aah) noexcept
{
    this->xp = ax;
    this->yp = ay;
    this->w = aaw;
    this->h = aah;
}

constexpr inline void QRectF::getCoords(qreal *xp1, qreal *yp1, qreal *xp2, qreal *yp2) const
{
    *xp1 = xp;
    *yp1 = yp;
    *xp2 = xp + w;
    *yp2 = yp + h;
}

constexpr inline void QRectF::setCoords(qreal xp1, qreal yp1, qreal xp2, qreal yp2) noexcept
{
    xp = xp1;
    yp = yp1;
    w = xp2 - xp1;
    h = yp2 - yp1;
}

constexpr inline void QRectF::adjust(qreal xp1, qreal yp1, qreal xp2, qreal yp2) noexcept
{
    xp += xp1;
    yp += yp1;
    w += xp2 - xp1;
    h += yp2 - yp1;
}

constexpr inline QRectF QRectF::adjusted(qreal xp1, qreal yp1, qreal xp2, qreal yp2) const noexcept
{
    return QRectF(xp + xp1, yp + yp1, w + xp2 - xp1, h + yp2 - yp1);
}

constexpr inline void QRectF::setWidth(qreal aw) noexcept
{ this->w = aw; }

constexpr inline void QRectF::setHeight(qreal ah) noexcept
{ this->h = ah; }

constexpr inline void QRectF::setSize(const QSizeF &s) noexcept
{
    w = s.width();
    h = s.height();
}

inline bool QRectF::contains(qreal ax, qreal ay) const noexcept
{
    return contains(QPointF(ax, ay));
}

inline QRectF &QRectF::operator|=(const QRectF &r) noexcept
{
    *this = *this | r;
    return *this;
}

inline QRectF &QRectF::operator&=(const QRectF &r) noexcept
{
    *this = *this & r;
    return *this;
}

inline QRectF QRectF::intersected(const QRectF &r) const noexcept
{
    return *this & r;
}

inline QRectF QRectF::united(const QRectF &r) const noexcept
{
    return *this | r;
}

constexpr QRectF QRect::toRectF() const noexcept { return *this; }

constexpr inline QRect QRectF::toRect() const noexcept
{
    // This rounding is designed to minimize the maximum possible difference
    // in topLeft(), bottomRight(), and size() after rounding.
    // All dimensions are at most off by 0.75, and topLeft by at most 0.5.
    const int nxp = qRound(xp);
    const int nyp = qRound(yp);
    const int nw = qRound(w + (xp - nxp) / 2);
    const int nh = qRound(h + (yp - nyp) / 2);
    return QRect(nxp, nyp, nw, nh);
}

constexpr inline QRectF operator+(const QRectF &lhs, const QMarginsF &rhs) noexcept
{
    return QRectF(QPointF(lhs.left() - rhs.left(), lhs.top() - rhs.top()),
                  QSizeF(lhs.width() + rhs.left() + rhs.right(), lhs.height() + rhs.top() + rhs.bottom()));
}

constexpr inline QRectF operator+(const QMarginsF &lhs, const QRectF &rhs) noexcept
{
    return QRectF(QPointF(rhs.left() - lhs.left(), rhs.top() - lhs.top()),
                  QSizeF(rhs.width() + lhs.left() + lhs.right(), rhs.height() + lhs.top() + lhs.bottom()));
}

constexpr inline QRectF operator-(const QRectF &lhs, const QMarginsF &rhs) noexcept
{
    return QRectF(QPointF(lhs.left() + rhs.left(), lhs.top() + rhs.top()),
                  QSizeF(lhs.width() - rhs.left() - rhs.right(), lhs.height() - rhs.top() - rhs.bottom()));
}

constexpr inline QRectF QRectF::marginsAdded(const QMarginsF &margins) const noexcept
{
    return QRectF(QPointF(xp - margins.left(), yp - margins.top()),
                  QSizeF(w + margins.left() + margins.right(), h + margins.top() + margins.bottom()));
}

constexpr inline QRectF QRectF::marginsRemoved(const QMarginsF &margins) const noexcept
{
    return QRectF(QPointF(xp + margins.left(), yp + margins.top()),
                  QSizeF(w - margins.left() - margins.right(), h - margins.top() - margins.bottom()));
}

constexpr inline QRectF &QRectF::operator+=(const QMarginsF &margins) noexcept
{
    *this = marginsAdded(margins);
    return *this;
}

constexpr inline QRectF &QRectF::operator-=(const QMarginsF &margins) noexcept
{
    *this = marginsRemoved(margins);
    return *this;
}

#ifndef QT_NO_DEBUG_STREAM
Q_CORE_EXPORT QDebug operator<<(QDebug, const QRectF &);
#endif

QT_END_NAMESPACE

#endif // QRECT_H
