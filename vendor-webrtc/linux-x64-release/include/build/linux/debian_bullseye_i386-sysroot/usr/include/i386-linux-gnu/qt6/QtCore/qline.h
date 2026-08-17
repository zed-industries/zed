// Copyright (C) 2022 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QLINE_H
#define QLINE_H

#include <QtCore/qpoint.h>

QT_BEGIN_NAMESPACE

class QLineF;

/*******************************************************************************
 * class QLine
 *******************************************************************************/

class Q_CORE_EXPORT QLine
{
public:
    constexpr inline QLine();
    constexpr inline QLine(const QPoint &pt1, const QPoint &pt2);
    constexpr inline QLine(int x1, int y1, int x2, int y2);

    constexpr inline bool isNull() const;

    constexpr inline QPoint p1() const;
    constexpr inline QPoint p2() const;

    constexpr inline int x1() const;
    constexpr inline int y1() const;

    constexpr inline int x2() const;
    constexpr inline int y2() const;

    constexpr inline int dx() const;
    constexpr inline int dy() const;

    inline void translate(const QPoint &p);
    inline void translate(int dx, int dy);

    [[nodiscard]] constexpr inline QLine translated(const QPoint &p) const;
    [[nodiscard]] constexpr inline QLine translated(int dx, int dy) const;

    [[nodiscard]] constexpr inline QPoint center() const;

    inline void setP1(const QPoint &p1);
    inline void setP2(const QPoint &p2);
    inline void setPoints(const QPoint &p1, const QPoint &p2);
    inline void setLine(int x1, int y1, int x2, int y2);

    constexpr inline bool operator==(const QLine &d) const noexcept;
    constexpr inline bool operator!=(const QLine &d) const noexcept { return !(*this == d); }

    [[nodiscard]] constexpr inline QLineF toLineF() const noexcept;

private:
    QPoint pt1, pt2;
};
Q_DECLARE_TYPEINFO(QLine, Q_PRIMITIVE_TYPE);

/*******************************************************************************
 * class QLine inline members
 *******************************************************************************/

constexpr inline QLine::QLine() { }

constexpr inline QLine::QLine(const QPoint &pt1_, const QPoint &pt2_) : pt1(pt1_), pt2(pt2_) { }

constexpr inline QLine::QLine(int x1pos, int y1pos, int x2pos, int y2pos) : pt1(QPoint(x1pos, y1pos)), pt2(QPoint(x2pos, y2pos)) { }

constexpr inline bool QLine::isNull() const
{
    return pt1 == pt2;
}

constexpr inline int QLine::x1() const
{
    return pt1.x();
}

constexpr inline int QLine::y1() const
{
    return pt1.y();
}

constexpr inline int QLine::x2() const
{
    return pt2.x();
}

constexpr inline int QLine::y2() const
{
    return pt2.y();
}

constexpr inline QPoint QLine::p1() const
{
    return pt1;
}

constexpr inline QPoint QLine::p2() const
{
    return pt2;
}

constexpr inline int QLine::dx() const
{
    return pt2.x() - pt1.x();
}

constexpr inline int QLine::dy() const
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

constexpr inline QLine QLine::translated(const QPoint &p) const
{
    return QLine(pt1 + p, pt2 + p);
}

constexpr inline QLine QLine::translated(int adx, int ady) const
{
    return translated(QPoint(adx, ady));
}

constexpr inline QPoint QLine::center() const
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

constexpr inline bool QLine::operator==(const QLine &d) const noexcept
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
class Q_CORE_EXPORT QLineF
{
public:

    enum IntersectionType { NoIntersection, BoundedIntersection, UnboundedIntersection };
    using IntersectType = IntersectionType; // deprecated name

    constexpr inline QLineF();
    constexpr inline QLineF(const QPointF &pt1, const QPointF &pt2);
    constexpr inline QLineF(qreal x1, qreal y1, qreal x2, qreal y2);
    constexpr inline QLineF(const QLine &line) : pt1(line.p1()), pt2(line.p2()) { }

    [[nodiscard]] static QLineF fromPolar(qreal length, qreal angle);

    constexpr bool isNull() const;

    constexpr inline QPointF p1() const;
    constexpr inline QPointF p2() const;

    constexpr inline qreal x1() const;
    constexpr inline qreal y1() const;

    constexpr inline qreal x2() const;
    constexpr inline qreal y2() const;

    constexpr inline qreal dx() const;
    constexpr inline qreal dy() const;

    qreal length() const;
    void setLength(qreal len);

    qreal angle() const;
    void setAngle(qreal angle);

    qreal angleTo(const QLineF &l) const;

    [[nodiscard]] QLineF unitVector() const;
    [[nodiscard]] constexpr inline QLineF normalVector() const;

    IntersectionType intersects(const QLineF &l, QPointF *intersectionPoint = nullptr) const;

    constexpr inline QPointF pointAt(qreal t) const;
    inline void translate(const QPointF &p);
    inline void translate(qreal dx, qreal dy);

    [[nodiscard]] constexpr inline QLineF translated(const QPointF &p) const;
    [[nodiscard]] constexpr inline QLineF translated(qreal dx, qreal dy) const;

    [[nodiscard]] constexpr inline QPointF center() const;

    inline void setP1(const QPointF &p1);
    inline void setP2(const QPointF &p2);
    inline void setPoints(const QPointF &p1, const QPointF &p2);
    inline void setLine(qreal x1, qreal y1, qreal x2, qreal y2);

    constexpr inline bool operator==(const QLineF &d) const;
    constexpr inline bool operator!=(const QLineF &d) const { return !(*this == d); }

    constexpr QLine toLine() const;

private:
    QPointF pt1, pt2;
};
Q_DECLARE_TYPEINFO(QLineF, Q_PRIMITIVE_TYPE);

/*******************************************************************************
 * class QLineF inline members
 *******************************************************************************/

constexpr inline QLineF::QLineF()
{
}

constexpr inline QLineF::QLineF(const QPointF &apt1, const QPointF &apt2)
    : pt1(apt1), pt2(apt2)
{
}

constexpr inline QLineF::QLineF(qreal x1pos, qreal y1pos, qreal x2pos, qreal y2pos)
    : pt1(x1pos, y1pos), pt2(x2pos, y2pos)
{
}

constexpr inline qreal QLineF::x1() const
{
    return pt1.x();
}

constexpr inline qreal QLineF::y1() const
{
    return pt1.y();
}

constexpr inline qreal QLineF::x2() const
{
    return pt2.x();
}

constexpr inline qreal QLineF::y2() const
{
    return pt2.y();
}

constexpr inline bool QLineF::isNull() const
{
    return qFuzzyCompare(pt1.x(), pt2.x()) && qFuzzyCompare(pt1.y(), pt2.y());
}

constexpr inline QPointF QLineF::p1() const
{
    return pt1;
}

constexpr inline QPointF QLineF::p2() const
{
    return pt2;
}

constexpr inline qreal QLineF::dx() const
{
    return pt2.x() - pt1.x();
}

constexpr inline qreal QLineF::dy() const
{
    return pt2.y() - pt1.y();
}

constexpr inline QLineF QLineF::normalVector() const
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

constexpr inline QLineF QLineF::translated(const QPointF &p) const
{
    return QLineF(pt1 + p, pt2 + p);
}

constexpr inline QLineF QLineF::translated(qreal adx, qreal ady) const
{
    return translated(QPointF(adx, ady));
}

constexpr inline QPointF QLineF::center() const
{
    return QPointF(0.5 * pt1.x() + 0.5 * pt2.x(), 0.5 * pt1.y() + 0.5 * pt2.y());
}

inline void QLineF::setLength(qreal len)
{
    Q_ASSERT(qIsFinite(len));
    const qreal oldLength = length();
    Q_ASSERT(qIsFinite(oldLength));
    // Scale len by dx() / length() and dy() / length(), two O(1) quantities,
    // rather than scaling dx() and dy() by len / length(), which might overflow.
    if (oldLength > 0)
        pt2 = QPointF(pt1.x() + len * (dx() / oldLength), pt1.y() + len * (dy() / oldLength));
}

constexpr inline QPointF QLineF::pointAt(qreal t) const
{
    return QPointF(pt1.x() + (pt2.x() - pt1.x()) * t, pt1.y() + (pt2.y() - pt1.y()) * t);
}

constexpr inline QLineF QLine::toLineF() const noexcept { return *this; }

constexpr inline QLine QLineF::toLine() const
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


constexpr inline bool QLineF::operator==(const QLineF &d) const
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
