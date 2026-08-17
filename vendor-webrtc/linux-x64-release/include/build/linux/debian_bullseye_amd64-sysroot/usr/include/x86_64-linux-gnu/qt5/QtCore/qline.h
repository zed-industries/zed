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

#ifndef QLINE_H
#define QLINE_H

#include <QtCore/qpoint.h>

QT_BEGIN_NAMESPACE


/*******************************************************************************
 * class QLine
 *******************************************************************************/

class Q_CORE_EXPORT QLine
{
public:
    Q_DECL_CONSTEXPR inline QLine();
    Q_DECL_CONSTEXPR inline QLine(const QPoint &pt1, const QPoint &pt2);
    Q_DECL_CONSTEXPR inline QLine(int x1, int y1, int x2, int y2);

    Q_DECL_CONSTEXPR inline bool isNull() const;

    Q_DECL_CONSTEXPR inline QPoint p1() const;
    Q_DECL_CONSTEXPR inline QPoint p2() const;

    Q_DECL_CONSTEXPR inline int x1() const;
    Q_DECL_CONSTEXPR inline int y1() const;

    Q_DECL_CONSTEXPR inline int x2() const;
    Q_DECL_CONSTEXPR inline int y2() const;

    Q_DECL_CONSTEXPR inline int dx() const;
    Q_DECL_CONSTEXPR inline int dy() const;

    inline void translate(const QPoint &p);
    inline void translate(int dx, int dy);

    Q_REQUIRED_RESULT Q_DECL_CONSTEXPR inline QLine translated(const QPoint &p) const;
    Q_REQUIRED_RESULT Q_DECL_CONSTEXPR inline QLine translated(int dx, int dy) const;

    Q_REQUIRED_RESULT Q_DECL_CONSTEXPR inline QPoint center() const;

    inline void setP1(const QPoint &p1);
    inline void setP2(const QPoint &p2);
    inline void setPoints(const QPoint &p1, const QPoint &p2);
    inline void setLine(int x1, int y1, int x2, int y2);

    Q_DECL_CONSTEXPR inline bool operator==(const QLine &d) const;
    Q_DECL_CONSTEXPR inline bool operator!=(const QLine &d) const { return !(*this == d); }

private:
    QPoint pt1, pt2;
};
Q_DECLARE_TYPEINFO(QLine, Q_MOVABLE_TYPE);

/*******************************************************************************
 * class QLine inline members
 *******************************************************************************/

Q_DECL_CONSTEXPR inline QLine::QLine() { }

Q_DECL_CONSTEXPR inline QLine::QLine(const QPoint &pt1_, const QPoint &pt2_) : pt1(pt1_), pt2(pt2_) { }

Q_DECL_CONSTEXPR inline QLine::QLine(int x1pos, int y1pos, int x2pos, int y2pos) : pt1(QPoint(x1pos, y1pos)), pt2(QPoint(x2pos, y2pos)) { }

Q_DECL_CONSTEXPR inline bool QLine::isNull() const
{
    return pt1 == pt2;
}

Q_DECL_CONSTEXPR inline int QLine::x1() const
{
    return pt1.x();
}

Q_DECL_CONSTEXPR inline int QLine::y1() const
{
    return pt1.y();
}

Q_DECL_CONSTEXPR inline int QLine::x2() const
{
    return pt2.x();
}

Q_DECL_CONSTEXPR inline int QLine::y2() const
{
    return pt2.y();
}

Q_DECL_CONSTEXPR inline QPoint QLine::p1() const
{
    return pt1;
}

Q_DECL_CONSTEXPR inline QPoint QLine::p2() const
{
    return pt2;
}

Q_DECL_CONSTEXPR inline int QLine::dx() const
{
    return pt2.x() - pt1.x();
}

Q_DECL_CONSTEXPR inline int QLine::dy() const
{
    return pt2.y() - pt1.y();
}

inline void QLine::translate(const QPoint &point)
{
    pt1 += point;
    pt2 += point;
}

inline void QLine::translate(int adx, int ady)
{
    this->translate(QPoint(adx, ady));
}

Q_DECL_CONSTEXPR inline QLine QLine::translated(const QPoint &p) const
{
    return QLine(pt1 + p, pt2 + p);
}

Q_DECL_CONSTEXPR inline QLine QLine::translated(int adx, int ady) const
{
    return translated(QPoint(adx, ady));
}

Q_DECL_CONSTEXPR inline QPoint QLine::center() const
{
    return QPoint(int((qint64(pt1.x()) + pt2.x()) / 2), int((qint64(pt1.y()) + pt2.y()) / 2));
}

inline void QLine::setP1(const QPoint &aP1)
{
    pt1 = aP1;
}

inline void QLine::setP2(const QPoint &aP2)
{
    pt2 = aP2;
}

inline void QLine::setPoints(const QPoint &aP1, const QPoint &aP2)
{
    pt1 = aP1;
    pt2 = aP2;
}

inline void QLine::setLine(int aX1, int aY1, int aX2, int aY2)
{
    pt1 = QPoint(aX1, aY1);
    pt2 = QPoint(aX2, aY2);
}

Q_DECL_CONSTEXPR inline bool QLine::operator==(const QLine &d) const
{
    return pt1 == d.pt1 && pt2 == d.pt2;
}

#ifndef QT_NO_DEBUG_STREAM
Q_CORE_EXPORT QDebug operator<<(QDebug d, const QLine &p);
#endif

#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QLine &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QLine &);
#endif

/*******************************************************************************
 * class QLineF
 *******************************************************************************/
class Q_CORE_EXPORT QLineF {
public:

    enum IntersectType { NoIntersection, BoundedIntersection, UnboundedIntersection };
    using IntersectionType = IntersectType;

    Q_DECL_CONSTEXPR inline QLineF();
    Q_DECL_CONSTEXPR inline QLineF(const QPointF &pt1, const QPointF &pt2);
    Q_DECL_CONSTEXPR inline QLineF(qreal x1, qreal y1, qreal x2, qreal y2);
    Q_DECL_CONSTEXPR inline QLineF(const QLine &line) : pt1(line.p1()), pt2(line.p2()) { }

    Q_REQUIRED_RESULT static QLineF fromPolar(qreal length, qreal angle);

    Q_DECL_CONSTEXPR bool isNull() const;

    Q_DECL_CONSTEXPR inline QPointF p1() const;
    Q_DECL_CONSTEXPR inline QPointF p2() const;

    Q_DECL_CONSTEXPR inline qreal x1() const;
    Q_DECL_CONSTEXPR inline qreal y1() const;

    Q_DECL_CONSTEXPR inline qreal x2() const;
    Q_DECL_CONSTEXPR inline qreal y2() const;

    Q_DECL_CONSTEXPR inline qreal dx() const;
    Q_DECL_CONSTEXPR inline qreal dy() const;

    qreal length() const;
    void setLength(qreal len);

    qreal angle() const;
    void setAngle(qreal angle);

    qreal angleTo(const QLineF &l) const;

    Q_REQUIRED_RESULT QLineF unitVector() const;
    Q_REQUIRED_RESULT Q_DECL_CONSTEXPR inline QLineF normalVector() const;

    IntersectionType intersects(const QLineF &l, QPointF *intersectionPoint) const;

#if QT_DEPRECATED_SINCE(5, 14)
    QT_DEPRECATED_VERSION_X(5, 14, "Use intersects() instead")
    IntersectType intersect(const QLineF &l, QPointF *intersectionPoint) const;
    QT_DEPRECATED_X("Use qMin(l1.angleTo(l2), l2.angleTo(l1)) instead")
    qreal angle(const QLineF &l) const;
#endif

    Q_DECL_CONSTEXPR inline QPointF pointAt(qreal t) const;
    inline void translate(const QPointF &p);
    inline void translate(qreal dx, qreal dy);

    Q_REQUIRED_RESULT Q_DECL_CONSTEXPR inline QLineF translated(const QPointF &p) const;
    Q_REQUIRED_RESULT Q_DECL_CONSTEXPR inline QLineF translated(qreal dx, qreal dy) const;

    Q_REQUIRED_RESULT Q_DECL_CONSTEXPR inline QPointF center() const;

    inline void setP1(const QPointF &p1);
    inline void setP2(const QPointF &p2);
    inline void setPoints(const QPointF &p1, const QPointF &p2);
    inline void setLine(qreal x1, qreal y1, qreal x2, qreal y2);

    Q_DECL_CONSTEXPR inline bool operator==(const QLineF &d) const;
    Q_DECL_CONSTEXPR inline bool operator!=(const QLineF &d) const { return !(*this == d); }

    Q_DECL_CONSTEXPR QLine toLine() const;

private:
    QPointF pt1, pt2;
};
Q_DECLARE_TYPEINFO(QLineF, Q_MOVABLE_TYPE);

/*******************************************************************************
 * class QLineF inline members
 *******************************************************************************/

Q_DECL_CONSTEXPR inline QLineF::QLineF()
{
}

Q_DECL_CONSTEXPR inline QLineF::QLineF(const QPointF &apt1, const QPointF &apt2)
    : pt1(apt1), pt2(apt2)
{
}

Q_DECL_CONSTEXPR inline QLineF::QLineF(qreal x1pos, qreal y1pos, qreal x2pos, qreal y2pos)
    : pt1(x1pos, y1pos), pt2(x2pos, y2pos)
{
}

Q_DECL_CONSTEXPR inline qreal QLineF::x1() const
{
    return pt1.x();
}

Q_DECL_CONSTEXPR inline qreal QLineF::y1() const
{
    return pt1.y();
}

Q_DECL_CONSTEXPR inline qreal QLineF::x2() const
{
    return pt2.x();
}

Q_DECL_CONSTEXPR inline qreal QLineF::y2() const
{
    return pt2.y();
}

Q_DECL_CONSTEXPR inline bool QLineF::isNull() const
{
    return qFuzzyCompare(pt1.x(), pt2.x()) && qFuzzyCompare(pt1.y(), pt2.y());
}

Q_DECL_CONSTEXPR inline QPointF QLineF::p1() const
{
    return pt1;
}

Q_DECL_CONSTEXPR inline QPointF QLineF::p2() const
{
    return pt2;
}

Q_DECL_CONSTEXPR inline qreal QLineF::dx() const
{
    return pt2.x() - pt1.x();
}

Q_DECL_CONSTEXPR inline qreal QLineF::dy() const
{
    return pt2.y() - pt1.y();
}

Q_DECL_CONSTEXPR inline QLineF QLineF::normalVector() const
{
    return QLineF(p1(), p1() + QPointF(dy(), -dx()));
}

inline void QLineF::translate(const QPointF &point)
{
    pt1 += point;
    pt2 += point;
}

inline void QLineF::translate(qreal adx, qreal ady)
{
    this->translate(QPointF(adx, ady));
}

Q_DECL_CONSTEXPR inline QLineF QLineF::translated(const QPointF &p) const
{
    return QLineF(pt1 + p, pt2 + p);
}

Q_DECL_CONSTEXPR inline QLineF QLineF::translated(qreal adx, qreal ady) const
{
    return translated(QPointF(adx, ady));
}

Q_DECL_CONSTEXPR inline QPointF QLineF::center() const
{
    return QPointF(0.5 * pt1.x() + 0.5 * pt2.x(), 0.5 * pt1.y() + 0.5 * pt2.y());
}

inline void QLineF::setLength(qreal len)
{
    if (isNull())
        return;
    Q_ASSERT(length() > 0);
    const QLineF v = unitVector();
    len /= v.length(); // In case it's not quite exactly 1.
    pt2 = QPointF(pt1.x() + len * v.dx(), pt1.y() + len * v.dy());
}

Q_DECL_CONSTEXPR inline QPointF QLineF::pointAt(qreal t) const
{
    return QPointF(pt1.x() + (pt2.x() - pt1.x()) * t, pt1.y() + (pt2.y() - pt1.y()) * t);
}

Q_DECL_CONSTEXPR inline QLine QLineF::toLine() const
{
    return QLine(pt1.toPoint(), pt2.toPoint());
}


inline void QLineF::setP1(const QPointF &aP1)
{
    pt1 = aP1;
}

inline void QLineF::setP2(const QPointF &aP2)
{
    pt2 = aP2;
}

inline void QLineF::setPoints(const QPointF &aP1, const QPointF &aP2)
{
    pt1 = aP1;
    pt2 = aP2;
}

inline void QLineF::setLine(qreal aX1, qreal aY1, qreal aX2, qreal aY2)
{
    pt1 = QPointF(aX1, aY1);
    pt2 = QPointF(aX2, aY2);
}


Q_DECL_CONSTEXPR inline bool QLineF::operator==(const QLineF &d) const
{
    return pt1 == d.pt1 && pt2 == d.pt2;
}



#ifndef QT_NO_DEBUG_STREAM
Q_CORE_EXPORT QDebug operator<<(QDebug d, const QLineF &p);
#endif

#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QLineF &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QLineF &);
#endif

QT_END_NAMESPACE

#endif // QLINE_H
