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

#ifndef QPOINT_H
#define QPOINT_H

#include <QtCore/qnamespace.h>

#if defined(Q_OS_DARWIN) || defined(Q_QDOC)
struct CGPoint;
#endif

QT_BEGIN_NAMESPACE


class Q_CORE_EXPORT QPoint
{
public:
    Q_DECL_CONSTEXPR QPoint();
    Q_DECL_CONSTEXPR QPoint(int xpos, int ypos);

    Q_DECL_CONSTEXPR inline bool isNull() const;

    Q_DECL_CONSTEXPR inline int x() const;
    Q_DECL_CONSTEXPR inline int y() const;
    Q_DECL_RELAXED_CONSTEXPR inline void setX(int x);
    Q_DECL_RELAXED_CONSTEXPR inline void setY(int y);

    Q_DECL_CONSTEXPR inline int manhattanLength() const;

    Q_DECL_CONSTEXPR QPoint transposed() const noexcept { return {yp, xp}; }

    Q_DECL_RELAXED_CONSTEXPR inline int &rx();
    Q_DECL_RELAXED_CONSTEXPR inline int &ry();

    Q_DECL_RELAXED_CONSTEXPR inline QPoint &operator+=(const QPoint &p);
    Q_DECL_RELAXED_CONSTEXPR inline QPoint &operator-=(const QPoint &p);

    Q_DECL_RELAXED_CONSTEXPR inline QPoint &operator*=(float factor);
    Q_DECL_RELAXED_CONSTEXPR inline QPoint &operator*=(double factor);
    Q_DECL_RELAXED_CONSTEXPR inline QPoint &operator*=(int factor);

    Q_DECL_RELAXED_CONSTEXPR inline QPoint &operator/=(qreal divisor);

    Q_DECL_CONSTEXPR static inline int dotProduct(const QPoint &p1, const QPoint &p2)
    { return p1.xp * p2.xp + p1.yp * p2.yp; }

    friend Q_DECL_CONSTEXPR inline bool operator==(const QPoint &, const QPoint &);
    friend Q_DECL_CONSTEXPR inline bool operator!=(const QPoint &, const QPoint &);
    friend Q_DECL_CONSTEXPR inline const QPoint operator+(const QPoint &, const QPoint &);
    friend Q_DECL_CONSTEXPR inline const QPoint operator-(const QPoint &, const QPoint &);
    friend Q_DECL_CONSTEXPR inline const QPoint operator*(const QPoint &, float);
    friend Q_DECL_CONSTEXPR inline const QPoint operator*(float, const QPoint &);
    friend Q_DECL_CONSTEXPR inline const QPoint operator*(const QPoint &, double);
    friend Q_DECL_CONSTEXPR inline const QPoint operator*(double, const QPoint &);
    friend Q_DECL_CONSTEXPR inline const QPoint operator*(const QPoint &, int);
    friend Q_DECL_CONSTEXPR inline const QPoint operator*(int, const QPoint &);
    friend Q_DECL_CONSTEXPR inline const QPoint operator+(const QPoint &);
    friend Q_DECL_CONSTEXPR inline const QPoint operator-(const QPoint &);
    friend Q_DECL_CONSTEXPR inline const QPoint operator/(const QPoint &, qreal);

#if defined(Q_OS_DARWIN) || defined(Q_QDOC)
    Q_REQUIRED_RESULT CGPoint toCGPoint() const noexcept;
#endif

private:
    friend class QTransform;
    int xp;
    int yp;
};

Q_DECLARE_TYPEINFO(QPoint, Q_MOVABLE_TYPE);

/*****************************************************************************
  QPoint stream functions
 *****************************************************************************/
#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QPoint &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QPoint &);
#endif

/*****************************************************************************
  QPoint inline functions
 *****************************************************************************/

Q_DECL_CONSTEXPR inline QPoint::QPoint() : xp(0), yp(0) {}

Q_DECL_CONSTEXPR inline QPoint::QPoint(int xpos, int ypos) : xp(xpos), yp(ypos) {}

Q_DECL_CONSTEXPR inline bool QPoint::isNull() const
{ return xp == 0 && yp == 0; }

Q_DECL_CONSTEXPR inline int QPoint::x() const
{ return xp; }

Q_DECL_CONSTEXPR inline int QPoint::y() const
{ return yp; }

Q_DECL_RELAXED_CONSTEXPR inline void QPoint::setX(int xpos)
{ xp = xpos; }

Q_DECL_RELAXED_CONSTEXPR inline void QPoint::setY(int ypos)
{ yp = ypos; }

inline int Q_DECL_CONSTEXPR QPoint::manhattanLength() const
{ return qAbs(x())+qAbs(y()); }

Q_DECL_RELAXED_CONSTEXPR inline int &QPoint::rx()
{ return xp; }

Q_DECL_RELAXED_CONSTEXPR inline int &QPoint::ry()
{ return yp; }

Q_DECL_RELAXED_CONSTEXPR inline QPoint &QPoint::operator+=(const QPoint &p)
{ xp+=p.xp; yp+=p.yp; return *this; }

Q_DECL_RELAXED_CONSTEXPR inline QPoint &QPoint::operator-=(const QPoint &p)
{ xp-=p.xp; yp-=p.yp; return *this; }

Q_DECL_RELAXED_CONSTEXPR inline QPoint &QPoint::operator*=(float factor)
{ xp = qRound(xp*factor); yp = qRound(yp*factor); return *this; }

Q_DECL_RELAXED_CONSTEXPR inline QPoint &QPoint::operator*=(double factor)
{ xp = qRound(xp*factor); yp = qRound(yp*factor); return *this; }

Q_DECL_RELAXED_CONSTEXPR inline QPoint &QPoint::operator*=(int factor)
{ xp = xp*factor; yp = yp*factor; return *this; }

Q_DECL_CONSTEXPR inline bool operator==(const QPoint &p1, const QPoint &p2)
{ return p1.xp == p2.xp && p1.yp == p2.yp; }

Q_DECL_CONSTEXPR inline bool operator!=(const QPoint &p1, const QPoint &p2)
{ return p1.xp != p2.xp || p1.yp != p2.yp; }

Q_DECL_CONSTEXPR inline const QPoint operator+(const QPoint &p1, const QPoint &p2)
{ return QPoint(p1.xp+p2.xp, p1.yp+p2.yp); }

Q_DECL_CONSTEXPR inline const QPoint operator-(const QPoint &p1, const QPoint &p2)
{ return QPoint(p1.xp-p2.xp, p1.yp-p2.yp); }

Q_DECL_CONSTEXPR inline const QPoint operator*(const QPoint &p, float factor)
{ return QPoint(qRound(p.xp*factor), qRound(p.yp*factor)); }

Q_DECL_CONSTEXPR inline const QPoint operator*(const QPoint &p, double factor)
{ return QPoint(qRound(p.xp*factor), qRound(p.yp*factor)); }

Q_DECL_CONSTEXPR inline const QPoint operator*(const QPoint &p, int factor)
{ return QPoint(p.xp*factor, p.yp*factor); }

Q_DECL_CONSTEXPR inline const QPoint operator*(float factor, const QPoint &p)
{ return QPoint(qRound(p.xp*factor), qRound(p.yp*factor)); }

Q_DECL_CONSTEXPR inline const QPoint operator*(double factor, const QPoint &p)
{ return QPoint(qRound(p.xp*factor), qRound(p.yp*factor)); }

Q_DECL_CONSTEXPR inline const QPoint operator*(int factor, const QPoint &p)
{ return QPoint(p.xp*factor, p.yp*factor); }

Q_DECL_CONSTEXPR inline const QPoint operator+(const QPoint &p)
{ return p; }

Q_DECL_CONSTEXPR inline const QPoint operator-(const QPoint &p)
{ return QPoint(-p.xp, -p.yp); }

Q_DECL_RELAXED_CONSTEXPR inline QPoint &QPoint::operator/=(qreal c)
{
    xp = qRound(xp/c);
    yp = qRound(yp/c);
    return *this;
}

Q_DECL_CONSTEXPR inline const QPoint operator/(const QPoint &p, qreal c)
{
    return QPoint(qRound(p.xp/c), qRound(p.yp/c));
}

#ifndef QT_NO_DEBUG_STREAM
Q_CORE_EXPORT QDebug operator<<(QDebug, const QPoint &);
#endif





class Q_CORE_EXPORT QPointF
{
public:
    Q_DECL_CONSTEXPR QPointF();
    Q_DECL_CONSTEXPR QPointF(const QPoint &p);
    Q_DECL_CONSTEXPR QPointF(qreal xpos, qreal ypos);

    Q_DECL_CONSTEXPR inline qreal manhattanLength() const;

    inline bool isNull() const;

    Q_DECL_CONSTEXPR inline qreal x() const;
    Q_DECL_CONSTEXPR inline qreal y() const;
    Q_DECL_RELAXED_CONSTEXPR inline void setX(qreal x);
    Q_DECL_RELAXED_CONSTEXPR inline void setY(qreal y);

    Q_DECL_CONSTEXPR QPointF transposed() const noexcept { return {yp, xp}; }

    Q_DECL_RELAXED_CONSTEXPR inline qreal &rx();
    Q_DECL_RELAXED_CONSTEXPR inline qreal &ry();

    Q_DECL_RELAXED_CONSTEXPR inline QPointF &operator+=(const QPointF &p);
    Q_DECL_RELAXED_CONSTEXPR inline QPointF &operator-=(const QPointF &p);
    Q_DECL_RELAXED_CONSTEXPR inline QPointF &operator*=(qreal c);
    Q_DECL_RELAXED_CONSTEXPR inline QPointF &operator/=(qreal c);

    Q_DECL_CONSTEXPR static inline qreal dotProduct(const QPointF &p1, const QPointF &p2)
    { return p1.xp * p2.xp + p1.yp * p2.yp; }

    friend Q_DECL_CONSTEXPR inline bool operator==(const QPointF &, const QPointF &);
    friend Q_DECL_CONSTEXPR inline bool operator!=(const QPointF &, const QPointF &);
    friend Q_DECL_CONSTEXPR inline const QPointF operator+(const QPointF &, const QPointF &);
    friend Q_DECL_CONSTEXPR inline const QPointF operator-(const QPointF &, const QPointF &);
    friend Q_DECL_CONSTEXPR inline const QPointF operator*(qreal, const QPointF &);
    friend Q_DECL_CONSTEXPR inline const QPointF operator*(const QPointF &, qreal);
    friend Q_DECL_CONSTEXPR inline const QPointF operator+(const QPointF &);
    friend Q_DECL_CONSTEXPR inline const QPointF operator-(const QPointF &);
    friend Q_DECL_CONSTEXPR inline const QPointF operator/(const QPointF &, qreal);

    Q_DECL_CONSTEXPR QPoint toPoint() const;

#if defined(Q_OS_DARWIN) || defined(Q_QDOC)
    Q_REQUIRED_RESULT static QPointF fromCGPoint(CGPoint point) noexcept;
    Q_REQUIRED_RESULT CGPoint toCGPoint() const noexcept;
#endif

private:
    friend class QMatrix;
    friend class QTransform;

    qreal xp;
    qreal yp;
};

Q_DECLARE_TYPEINFO(QPointF, Q_MOVABLE_TYPE);

/*****************************************************************************
  QPointF stream functions
 *****************************************************************************/
#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QPointF &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QPointF &);
#endif

/*****************************************************************************
  QPointF inline functions
 *****************************************************************************/

Q_DECL_CONSTEXPR inline QPointF::QPointF() : xp(0), yp(0) { }

Q_DECL_CONSTEXPR inline QPointF::QPointF(qreal xpos, qreal ypos) : xp(xpos), yp(ypos) { }

Q_DECL_CONSTEXPR inline QPointF::QPointF(const QPoint &p) : xp(p.x()), yp(p.y()) { }

Q_DECL_CONSTEXPR inline qreal QPointF::manhattanLength() const
{
    return qAbs(x())+qAbs(y());
}

inline bool QPointF::isNull() const
{
    return qIsNull(xp) && qIsNull(yp);
}

Q_DECL_CONSTEXPR inline qreal QPointF::x() const
{
    return xp;
}

Q_DECL_CONSTEXPR inline qreal QPointF::y() const
{
    return yp;
}

Q_DECL_RELAXED_CONSTEXPR inline void QPointF::setX(qreal xpos)
{
    xp = xpos;
}

Q_DECL_RELAXED_CONSTEXPR inline void QPointF::setY(qreal ypos)
{
    yp = ypos;
}

Q_DECL_RELAXED_CONSTEXPR inline qreal &QPointF::rx()
{
    return xp;
}

Q_DECL_RELAXED_CONSTEXPR inline qreal &QPointF::ry()
{
    return yp;
}

Q_DECL_RELAXED_CONSTEXPR inline QPointF &QPointF::operator+=(const QPointF &p)
{
    xp+=p.xp;
    yp+=p.yp;
    return *this;
}

Q_DECL_RELAXED_CONSTEXPR inline QPointF &QPointF::operator-=(const QPointF &p)
{
    xp-=p.xp; yp-=p.yp; return *this;
}

Q_DECL_RELAXED_CONSTEXPR inline QPointF &QPointF::operator*=(qreal c)
{
    xp*=c; yp*=c; return *this;
}

QT_WARNING_PUSH
QT_WARNING_DISABLE_CLANG("-Wfloat-equal")
QT_WARNING_DISABLE_GCC("-Wfloat-equal")
QT_WARNING_DISABLE_INTEL(1572)

Q_DECL_CONSTEXPR inline bool operator==(const QPointF &p1, const QPointF &p2)
{
    return ((!p1.xp || !p2.xp) ? qFuzzyIsNull(p1.xp - p2.xp) : qFuzzyCompare(p1.xp, p2.xp))
        && ((!p1.yp || !p2.yp) ? qFuzzyIsNull(p1.yp - p2.yp) : qFuzzyCompare(p1.yp, p2.yp));
}

Q_DECL_CONSTEXPR inline bool operator!=(const QPointF &p1, const QPointF &p2)
{
    return !(p1 == p2);
}

QT_WARNING_POP

Q_DECL_CONSTEXPR inline const QPointF operator+(const QPointF &p1, const QPointF &p2)
{
    return QPointF(p1.xp+p2.xp, p1.yp+p2.yp);
}

Q_DECL_CONSTEXPR inline const QPointF operator-(const QPointF &p1, const QPointF &p2)
{
    return QPointF(p1.xp-p2.xp, p1.yp-p2.yp);
}

Q_DECL_CONSTEXPR inline const QPointF operator*(const QPointF &p, qreal c)
{
    return QPointF(p.xp*c, p.yp*c);
}

Q_DECL_CONSTEXPR inline const QPointF operator*(qreal c, const QPointF &p)
{
    return QPointF(p.xp*c, p.yp*c);
}

Q_DECL_CONSTEXPR inline const QPointF operator+(const QPointF &p)
{
    return p;
}

Q_DECL_CONSTEXPR inline const QPointF operator-(const QPointF &p)
{
    return QPointF(-p.xp, -p.yp);
}

Q_DECL_RELAXED_CONSTEXPR inline QPointF &QPointF::operator/=(qreal divisor)
{
    xp/=divisor;
    yp/=divisor;
    return *this;
}

Q_DECL_CONSTEXPR inline const QPointF operator/(const QPointF &p, qreal divisor)
{
    return QPointF(p.xp/divisor, p.yp/divisor);
}

Q_DECL_CONSTEXPR inline QPoint QPointF::toPoint() const
{
    return QPoint(qRound(xp), qRound(yp));
}

#ifndef QT_NO_DEBUG_STREAM
Q_CORE_EXPORT QDebug operator<<(QDebug d, const QPointF &p);
#endif

QT_END_NAMESPACE

#endif // QPOINT_H
