// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only
#ifndef QTRANSFORM_H
#define QTRANSFORM_H

#include <QtGui/qtguiglobal.h>
#include <QtGui/qpolygon.h>
#include <QtGui/qregion.h>
#include <QtGui/qwindowdefs.h>
#include <QtCore/qline.h>
#include <QtCore/qpoint.h>
#include <QtCore/qrect.h>

QT_BEGIN_NAMESPACE

class QVariant;
class QPainterPath;

class Q_GUI_EXPORT QTransform
{
public:
    enum TransformationType {
        TxNone      = 0x00,
        TxTranslate = 0x01,
        TxScale     = 0x02,
        TxRotate    = 0x04,
        TxShear     = 0x08,
        TxProject   = 0x10
    };

    inline explicit QTransform(Qt::Initialization) {}
    inline QTransform()
        : m_matrix{ {1, 0, 0}, {0, 1, 0}, {0, 0, 1} }
        , m_type(TxNone)
        , m_dirty(TxNone) {}
    QTransform(qreal h11, qreal h12, qreal h13,
               qreal h21, qreal h22, qreal h23,
               qreal h31, qreal h32, qreal h33)
        : m_matrix{ {h11, h12, h13}, {h21, h22, h23}, {h31, h32, h33} }
        , m_type(TxNone)
        , m_dirty(TxProject) {}
    QTransform(qreal h11, qreal h12, qreal h21,
               qreal h22, qreal dx, qreal dy)
        : m_matrix{ {h11, h12, 0}, {h21, h22, 0}, {dx, dy, 1} }
        , m_type(TxNone)
        , m_dirty(TxShear) {}

    QTransform &operator=(QTransform &&other) noexcept = default;
    QTransform &operator=(const QTransform &) noexcept = default;
    QTransform(QTransform &&other) noexcept = default;
    QTransform(const QTransform &other) noexcept = default;

    bool isAffine() const;
    bool isIdentity() const;
    bool isInvertible() const;
    bool isScaling() const;
    bool isRotating() const;
    bool isTranslating() const;

    TransformationType type() const;

    inline qreal determinant() const;

    qreal m11() const;
    qreal m12() const;
    qreal m13() const;
    qreal m21() const;
    qreal m22() const;
    qreal m23() const;
    qreal m31() const;
    qreal m32() const;
    qreal m33() const;
    qreal dx() const;
    qreal dy() const;

    void setMatrix(qreal m11, qreal m12, qreal m13,
                   qreal m21, qreal m22, qreal m23,
                   qreal m31, qreal m32, qreal m33);

    [[nodiscard]] QTransform inverted(bool *invertible = nullptr) const;
    [[nodiscard]] QTransform adjoint() const;
    [[nodiscard]] QTransform transposed() const;

    QTransform &translate(qreal dx, qreal dy);
    QTransform &scale(qreal sx, qreal sy);
    QTransform &shear(qreal sh, qreal sv);
    QTransform &rotate(qreal a, Qt::Axis axis = Qt::ZAxis);
    QTransform &rotateRadians(qreal a, Qt::Axis axis = Qt::ZAxis);

    static bool squareToQuad(const QPolygonF &square, QTransform &result);
    static bool quadToSquare(const QPolygonF &quad, QTransform &result);
    static bool quadToQuad(const QPolygonF &one,
                           const QPolygonF &two,
                           QTransform &result);

    bool operator==(const QTransform &) const;
    bool operator!=(const QTransform &) const;

    QTransform &operator*=(const QTransform &);
    QTransform operator*(const QTransform &o) const;

    operator QVariant() const;

    void reset();
    QPoint       map(const QPoint &p) const;
    QPointF      map(const QPointF &p) const;
    QLine        map(const QLine &l) const;
    QLineF       map(const QLineF &l) const;
    QPolygonF    map(const QPolygonF &a) const;
    QPolygon     map(const QPolygon &a) const;
    QRegion      map(const QRegion &r) const;
    QPainterPath map(const QPainterPath &p) const;
    QPolygon     mapToPolygon(const QRect &r) const;
    QRect mapRect(const QRect &) const;
    QRectF mapRect(const QRectF &) const;
    void map(int x, int y, int *tx, int *ty) const;
    void map(qreal x, qreal y, qreal *tx, qreal *ty) const;

    QTransform &operator*=(qreal div);
    QTransform &operator/=(qreal div);
    QTransform &operator+=(qreal div);
    QTransform &operator-=(qreal div);

    static QTransform fromTranslate(qreal dx, qreal dy);
    static QTransform fromScale(qreal dx, qreal dy);

private:
    struct Affine {
             qreal (& m_matrix)[3][3];
        };

public:
    auto asAffineMatrix() { return Affine { m_matrix }; }
    friend Q_GUI_EXPORT QDataStream &operator>>(QDataStream &s, Affine &m);
    friend Q_GUI_EXPORT QDataStream &operator<<(QDataStream &s, const Affine &m);

private:
    inline TransformationType inline_type() const;
    qreal m_matrix[3][3];

    mutable uint m_type : 5;
    mutable uint m_dirty : 5;
};
Q_DECLARE_TYPEINFO(QTransform, Q_RELOCATABLE_TYPE);

Q_GUI_EXPORT Q_DECL_CONST_FUNCTION size_t qHash(const QTransform &key, size_t seed = 0) noexcept;

/******* inlines *****/
inline QTransform::TransformationType QTransform::inline_type() const
{
    if (m_dirty == TxNone)
        return static_cast<TransformationType>(m_type);
    return type();
}

inline bool QTransform::isAffine() const
{
    return inline_type() < TxProject;
}
inline bool QTransform::isIdentity() const
{
    return inline_type() == TxNone;
}

inline bool QTransform::isInvertible() const
{
    return !qFuzzyIsNull(determinant());
}

inline bool QTransform::isScaling() const
{
    return type() >= TxScale;
}
inline bool QTransform::isRotating() const
{
    return inline_type() >= TxRotate;
}

inline bool QTransform::isTranslating() const
{
    return inline_type() >= TxTranslate;
}

inline qreal QTransform::determinant() const
{
    return m_matrix[0][0] * (m_matrix[2][2] * m_matrix[1][1] - m_matrix[2][1] * m_matrix[1][2]) -
           m_matrix[1][0] * (m_matrix[2][2] * m_matrix[0][1] - m_matrix[2][1] * m_matrix[0][2]) +
           m_matrix[2][0] * (m_matrix[1][2] * m_matrix[0][1] - m_matrix[1][1] * m_matrix[0][2]);
}
inline qreal QTransform::m11() const
{
    return m_matrix[0][0];
}
inline qreal QTransform::m12() const
{
    return m_matrix[0][1];
}
inline qreal QTransform::m13() const
{
    return m_matrix[0][2];
}
inline qreal QTransform::m21() const
{
    return m_matrix[1][0];
}
inline qreal QTransform::m22() const
{
    return m_matrix[1][1];
}
inline qreal QTransform::m23() const
{
    return m_matrix[1][2];
}
inline qreal QTransform::m31() const
{
    return m_matrix[2][0];
}
inline qreal QTransform::m32() const
{
    return m_matrix[2][1];
}
inline qreal QTransform::m33() const
{
    return m_matrix[2][2];
}
inline qreal QTransform::dx() const
{
    return m_matrix[2][0];
}
inline qreal QTransform::dy() const
{
    return m_matrix[2][1];
}

QT_WARNING_PUSH
QT_WARNING_DISABLE_FLOAT_COMPARE

inline QTransform &QTransform::operator*=(qreal num)
{
    if (num == 1.)
        return *this;
    m_matrix[0][0] *= num;
    m_matrix[0][1] *= num;
    m_matrix[0][2] *= num;
    m_matrix[1][0] *= num;
    m_matrix[1][1] *= num;
    m_matrix[1][2] *= num;
    m_matrix[2][0] *= num;
    m_matrix[2][1] *= num;
    m_matrix[2][2] *= num;
    if (m_dirty < TxScale)
        m_dirty = TxScale;
    return *this;
}
inline QTransform &QTransform::operator/=(qreal div)
{
    if (div == 0)
        return *this;
    div = 1/div;
    return operator*=(div);
}
inline QTransform &QTransform::operator+=(qreal num)
{
    if (num == 0)
        return *this;
    m_matrix[0][0] += num;
    m_matrix[0][1] += num;
    m_matrix[0][2] += num;
    m_matrix[1][0] += num;
    m_matrix[1][1] += num;
    m_matrix[1][2] += num;
    m_matrix[2][0] += num;
    m_matrix[2][1] += num;
    m_matrix[2][2] += num;
    m_dirty     = TxProject;
    return *this;
}
inline QTransform &QTransform::operator-=(qreal num)
{
    if (num == 0)
        return *this;
    m_matrix[0][0] -= num;
    m_matrix[0][1] -= num;
    m_matrix[0][2] -= num;
    m_matrix[1][0] -= num;
    m_matrix[1][1] -= num;
    m_matrix[1][2] -= num;
    m_matrix[2][0] -= num;
    m_matrix[2][1] -= num;
    m_matrix[2][2] -= num;
    m_dirty     = TxProject;
    return *this;
}

QT_WARNING_POP

inline bool qFuzzyCompare(const QTransform& t1, const QTransform& t2)
{
    return qFuzzyCompare(t1.m11(), t2.m11())
        && qFuzzyCompare(t1.m12(), t2.m12())
        && qFuzzyCompare(t1.m13(), t2.m13())
        && qFuzzyCompare(t1.m21(), t2.m21())
        && qFuzzyCompare(t1.m22(), t2.m22())
        && qFuzzyCompare(t1.m23(), t2.m23())
        && qFuzzyCompare(t1.m31(), t2.m31())
        && qFuzzyCompare(t1.m32(), t2.m32())
        && qFuzzyCompare(t1.m33(), t2.m33());
}


/****** stream functions *******************/
#ifndef QT_NO_DATASTREAM
Q_GUI_EXPORT QDataStream &operator<<(QDataStream &, const QTransform &);
Q_GUI_EXPORT QDataStream &operator>>(QDataStream &, QTransform &);
#endif

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug, const QTransform &);
#endif
/****** end stream functions *******************/

// mathematical semantics
inline QPoint operator*(const QPoint &p, const QTransform &m)
{ return m.map(p); }
inline QPointF operator*(const QPointF &p, const QTransform &m)
{ return m.map(p); }
inline QLineF operator*(const QLineF &l, const QTransform &m)
{ return m.map(l); }
inline QLine operator*(const QLine &l, const QTransform &m)
{ return m.map(l); }
inline QPolygon operator *(const QPolygon &a, const QTransform &m)
{ return m.map(a); }
inline QPolygonF operator *(const QPolygonF &a, const QTransform &m)
{ return m.map(a); }
inline QRegion operator *(const QRegion &r, const QTransform &m)
{ return m.map(r); }

inline QTransform operator *(const QTransform &a, qreal n)
{ QTransform t(a); t *= n; return t; }
inline QTransform operator /(const QTransform &a, qreal n)
{ QTransform t(a); t /= n; return t; }
inline QTransform operator +(const QTransform &a, qreal n)
{ QTransform t(a); t += n; return t; }
inline QTransform operator -(const QTransform &a, qreal n)
{ QTransform t(a); t -= n; return t; }

QT_END_NAMESPACE

#endif // QTRANSFORM_H
