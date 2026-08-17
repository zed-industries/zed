/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtGui module of the Qt Toolkit.
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

#ifndef QMATRIX_H
#define QMATRIX_H

#include <QtGui/qtguiglobal.h>
#include <QtGui/qpolygon.h>
#include <QtGui/qregion.h>
#include <QtGui/qwindowdefs.h>
#include <QtCore/qline.h>
#include <QtCore/qpoint.h>
#include <QtCore/qrect.h>

QT_BEGIN_NAMESPACE


class QPainterPath;
class QVariant;

class Q_GUI_EXPORT QMatrix // 2D transform matrix
{
public:
    inline explicit QMatrix(Qt::Initialization) {}
    QMatrix();
    QMatrix(qreal m11, qreal m12, qreal m21, qreal m22,
            qreal dx, qreal dy);

#if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
    // ### Qt 6: remove; the compiler-generated ones are fine!
    QMatrix &operator=(QMatrix &&other) noexcept // = default
    { memcpy(static_cast<void *>(this), static_cast<void *>(&other), sizeof(QMatrix)); return *this; }
    QMatrix &operator=(const QMatrix &) noexcept; // = default
    QMatrix(QMatrix &&other) noexcept // = default
    { memcpy(static_cast<void *>(this), static_cast<void *>(&other), sizeof(QMatrix)); }
    QMatrix(const QMatrix &other) noexcept; // = default
#endif

    void setMatrix(qreal m11, qreal m12, qreal m21, qreal m22,
                   qreal dx, qreal dy);

    qreal m11() const { return _m11; }
    qreal m12() const { return _m12; }
    qreal m21() const { return _m21; }
    qreal m22() const { return _m22; }
    qreal dx() const { return _dx; }
    qreal dy() const { return _dy; }

    void map(int x, int y, int *tx, int *ty) const;
    void map(qreal x, qreal y, qreal *tx, qreal *ty) const;
    QRect mapRect(const QRect &) const;
    QRectF mapRect(const QRectF &) const;

    QPoint map(const QPoint &p) const;
    QPointF map(const QPointF&p) const;
    QLine map(const QLine &l) const;
    QLineF map(const QLineF &l) const;
    QPolygonF map(const QPolygonF &a) const;
    QPolygon map(const QPolygon &a) const;
    QRegion map(const QRegion &r) const;
    QPainterPath map(const QPainterPath &p) const;
    QPolygon mapToPolygon(const QRect &r) const;

    void reset();
    inline bool isIdentity() const;

    QMatrix &translate(qreal dx, qreal dy);
    QMatrix &scale(qreal sx, qreal sy);
    QMatrix &shear(qreal sh, qreal sv);
    QMatrix &rotate(qreal a);

    bool isInvertible() const { return !qFuzzyIsNull(_m11*_m22 - _m12*_m21); }
    qreal determinant() const { return _m11*_m22 - _m12*_m21; }

    Q_REQUIRED_RESULT QMatrix inverted(bool *invertible = nullptr) const;

    bool operator==(const QMatrix &) const;
    bool operator!=(const QMatrix &) const;

    QMatrix &operator*=(const QMatrix &);
    QMatrix operator*(const QMatrix &o) const;

    operator QVariant() const;

private:
    inline QMatrix(bool)
            : _m11(1.)
            , _m12(0.)
            , _m21(0.)
            , _m22(1.)
            , _dx(0.)
            , _dy(0.) {}
    inline QMatrix(qreal am11, qreal am12, qreal am21, qreal am22, qreal adx, qreal ady, bool)
            : _m11(am11)
            , _m12(am12)
            , _m21(am21)
            , _m22(am22)
            , _dx(adx)
            , _dy(ady) {}
    friend class QTransform;
    qreal _m11, _m12;
    qreal _m21, _m22;
    qreal _dx, _dy;
};
Q_DECLARE_TYPEINFO(QMatrix, Q_MOVABLE_TYPE);

Q_GUI_EXPORT Q_DECL_CONST_FUNCTION uint qHash(const QMatrix &key, uint seed = 0) noexcept;

// mathematical semantics
inline QPoint operator*(const QPoint &p, const QMatrix &m)
{ return m.map(p); }
inline QPointF operator*(const QPointF &p, const QMatrix &m)
{ return m.map(p); }
inline QLineF operator*(const QLineF &l, const QMatrix &m)
{ return m.map(l); }
inline QLine operator*(const QLine &l, const QMatrix &m)
{ return m.map(l); }
inline QPolygon operator *(const QPolygon &a, const QMatrix &m)
{ return m.map(a); }
inline QPolygonF operator *(const QPolygonF &a, const QMatrix &m)
{ return m.map(a); }
inline QRegion operator *(const QRegion &r, const QMatrix &m)
{ return m.map(r); }
Q_GUI_EXPORT QPainterPath operator *(const QPainterPath &p, const QMatrix &m);

inline bool QMatrix::isIdentity() const
{
    return qFuzzyIsNull(_m11 - 1) && qFuzzyIsNull(_m22 - 1) && qFuzzyIsNull(_m12)
           && qFuzzyIsNull(_m21) && qFuzzyIsNull(_dx) && qFuzzyIsNull(_dy);
}

inline bool qFuzzyCompare(const QMatrix& m1, const QMatrix& m2)
{
    return qFuzzyCompare(m1.m11(), m2.m11())
        && qFuzzyCompare(m1.m12(), m2.m12())
        && qFuzzyCompare(m1.m21(), m2.m21())
        && qFuzzyCompare(m1.m22(), m2.m22())
        && qFuzzyCompare(m1.dx(), m2.dx())
        && qFuzzyCompare(m1.dy(), m2.dy());
}


/*****************************************************************************
 QMatrix stream functions
 *****************************************************************************/

#ifndef QT_NO_DATASTREAM
Q_GUI_EXPORT QDataStream &operator<<(QDataStream &, const QMatrix &);
Q_GUI_EXPORT QDataStream &operator>>(QDataStream &, QMatrix &);
#endif

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug, const QMatrix &);
#endif

QT_END_NAMESPACE

#endif // QMATRIX_H
