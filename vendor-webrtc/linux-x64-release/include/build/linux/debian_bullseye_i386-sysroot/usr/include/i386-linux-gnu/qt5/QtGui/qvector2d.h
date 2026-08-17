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

#ifndef QVECTOR2D_H
#define QVECTOR2D_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qpoint.h>
#include <QtCore/qmetatype.h>

QT_BEGIN_NAMESPACE


class QVector3D;
class QVector4D;
class QVariant;

#ifndef QT_NO_VECTOR2D

class Q_GUI_EXPORT QVector2D
{
public:
    Q_DECL_CONSTEXPR QVector2D();
    explicit QVector2D(Qt::Initialization) {}
    Q_DECL_CONSTEXPR QVector2D(float xpos, float ypos);
    Q_DECL_CONSTEXPR explicit QVector2D(const QPoint& point);
    Q_DECL_CONSTEXPR explicit QVector2D(const QPointF& point);
#ifndef QT_NO_VECTOR3D
    explicit QVector2D(const QVector3D& vector);
#endif
#ifndef QT_NO_VECTOR4D
    explicit QVector2D(const QVector4D& vector);
#endif

    bool isNull() const;

    Q_DECL_CONSTEXPR float x() const;
    Q_DECL_CONSTEXPR float y() const;

    void setX(float x);
    void setY(float y);

    float &operator[](int i);
    float operator[](int i) const;

    float length() const;
    float lengthSquared() const; //In Qt 6 convert to inline and constexpr

    Q_REQUIRED_RESULT QVector2D normalized() const;
    void normalize();

    float distanceToPoint(const QVector2D &point) const;
    float distanceToLine(const QVector2D& point, const QVector2D& direction) const;

    QVector2D &operator+=(const QVector2D &vector);
    QVector2D &operator-=(const QVector2D &vector);
    QVector2D &operator*=(float factor);
    QVector2D &operator*=(const QVector2D &vector);
    QVector2D &operator/=(float divisor);
    inline QVector2D &operator/=(const QVector2D &vector);

    static float dotProduct(const QVector2D& v1, const QVector2D& v2); //In Qt 6 convert to inline and constexpr

    Q_DECL_CONSTEXPR friend inline bool operator==(const QVector2D &v1, const QVector2D &v2);
    Q_DECL_CONSTEXPR friend inline bool operator!=(const QVector2D &v1, const QVector2D &v2);
    Q_DECL_CONSTEXPR friend inline const QVector2D operator+(const QVector2D &v1, const QVector2D &v2);
    Q_DECL_CONSTEXPR friend inline const QVector2D operator-(const QVector2D &v1, const QVector2D &v2);
    Q_DECL_CONSTEXPR friend inline const QVector2D operator*(float factor, const QVector2D &vector);
    Q_DECL_CONSTEXPR friend inline const QVector2D operator*(const QVector2D &vector, float factor);
    Q_DECL_CONSTEXPR friend inline const QVector2D operator*(const QVector2D &v1, const QVector2D &v2);
    Q_DECL_CONSTEXPR friend inline const QVector2D operator-(const QVector2D &vector);
    Q_DECL_CONSTEXPR friend inline const QVector2D operator/(const QVector2D &vector, float divisor);
    Q_DECL_CONSTEXPR friend inline const QVector2D operator/(const QVector2D &vector, const QVector2D &divisor);

    Q_DECL_CONSTEXPR friend inline bool qFuzzyCompare(const QVector2D& v1, const QVector2D& v2);

#ifndef QT_NO_VECTOR3D
    QVector3D toVector3D() const;
#endif
#ifndef QT_NO_VECTOR4D
    QVector4D toVector4D() const;
#endif

    Q_DECL_CONSTEXPR QPoint toPoint() const;
    Q_DECL_CONSTEXPR QPointF toPointF() const;

    operator QVariant() const;

private:
    float v[2];

    friend class QVector3D;
    friend class QVector4D;
};

Q_DECLARE_TYPEINFO(QVector2D, Q_PRIMITIVE_TYPE);

Q_DECL_CONSTEXPR inline QVector2D::QVector2D() : v{0.0f, 0.0f} {}

Q_DECL_CONSTEXPR inline QVector2D::QVector2D(float xpos, float ypos) : v{xpos, ypos} {}

Q_DECL_CONSTEXPR inline QVector2D::QVector2D(const QPoint& point) : v{float(point.x()), float(point.y())} {}

Q_DECL_CONSTEXPR inline QVector2D::QVector2D(const QPointF& point) : v{float(point.x()), float(point.y())} {}

inline bool QVector2D::isNull() const
{
    return qIsNull(v[0]) && qIsNull(v[1]);
}

Q_DECL_CONSTEXPR inline float QVector2D::x() const { return v[0]; }
Q_DECL_CONSTEXPR inline float QVector2D::y() const { return v[1]; }

inline void QVector2D::setX(float aX) { v[0] = aX; }
inline void QVector2D::setY(float aY) { v[1] = aY; }

inline float &QVector2D::operator[](int i)
{
    Q_ASSERT(uint(i) < 2u);
    return v[i];
}

inline float QVector2D::operator[](int i) const
{
    Q_ASSERT(uint(i) < 2u);
    return v[i];
}

inline QVector2D &QVector2D::operator+=(const QVector2D &vector)
{
    v[0] += vector.v[0];
    v[1] += vector.v[1];
    return *this;
}

inline QVector2D &QVector2D::operator-=(const QVector2D &vector)
{
    v[0] -= vector.v[0];
    v[1] -= vector.v[1];
    return *this;
}

inline QVector2D &QVector2D::operator*=(float factor)
{
    v[0] *= factor;
    v[1] *= factor;
    return *this;
}

inline QVector2D &QVector2D::operator*=(const QVector2D &vector)
{
    v[0] *= vector.v[0];
    v[1] *= vector.v[1];
    return *this;
}

inline QVector2D &QVector2D::operator/=(float divisor)
{
    v[0] /= divisor;
    v[1] /= divisor;
    return *this;
}

inline QVector2D &QVector2D::operator/=(const QVector2D &vector)
{
    v[0] /= vector.v[0];
    v[1] /= vector.v[1];
    return *this;
}

QT_WARNING_PUSH
QT_WARNING_DISABLE_CLANG("-Wfloat-equal")
QT_WARNING_DISABLE_GCC("-Wfloat-equal")
QT_WARNING_DISABLE_INTEL(1572)
Q_DECL_CONSTEXPR inline bool operator==(const QVector2D &v1, const QVector2D &v2)
{
    return v1.v[0] == v2.v[0] && v1.v[1] == v2.v[1];
}

Q_DECL_CONSTEXPR inline bool operator!=(const QVector2D &v1, const QVector2D &v2)
{
    return v1.v[0] != v2.v[0] || v1.v[1] != v2.v[1];
}
QT_WARNING_POP

Q_DECL_CONSTEXPR inline const QVector2D operator+(const QVector2D &v1, const QVector2D &v2)
{
    return QVector2D(v1.v[0] + v2.v[0], v1.v[1] + v2.v[1]);
}

Q_DECL_CONSTEXPR inline const QVector2D operator-(const QVector2D &v1, const QVector2D &v2)
{
    return QVector2D(v1.v[0] - v2.v[0], v1.v[1] - v2.v[1]);
}

Q_DECL_CONSTEXPR inline const QVector2D operator*(float factor, const QVector2D &vector)
{
    return QVector2D(vector.v[0] * factor, vector.v[1] * factor);
}

Q_DECL_CONSTEXPR inline const QVector2D operator*(const QVector2D &vector, float factor)
{
    return QVector2D(vector.v[0] * factor, vector.v[1] * factor);
}

Q_DECL_CONSTEXPR inline const QVector2D operator*(const QVector2D &v1, const QVector2D &v2)
{
    return QVector2D(v1.v[0] * v2.v[0], v1.v[1] * v2.v[1]);
}

Q_DECL_CONSTEXPR inline const QVector2D operator-(const QVector2D &vector)
{
    return QVector2D(-vector.v[0], -vector.v[1]);
}

Q_DECL_CONSTEXPR inline const QVector2D operator/(const QVector2D &vector, float divisor)
{
    return QVector2D(vector.v[0] / divisor, vector.v[1] / divisor);
}

Q_DECL_CONSTEXPR inline const QVector2D operator/(const QVector2D &vector, const QVector2D &divisor)
{
    return QVector2D(vector.v[0] / divisor.v[0], vector.v[1] / divisor.v[1]);
}

Q_DECL_CONSTEXPR inline bool qFuzzyCompare(const QVector2D& v1, const QVector2D& v2)
{
    return qFuzzyCompare(v1.v[0], v2.v[0]) && qFuzzyCompare(v1.v[1], v2.v[1]);
}

Q_DECL_CONSTEXPR inline QPoint QVector2D::toPoint() const
{
    return QPoint(qRound(v[0]), qRound(v[1]));
}

Q_DECL_CONSTEXPR inline QPointF QVector2D::toPointF() const
{
    return QPointF(qreal(v[0]), qreal(v[1]));
}

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug dbg, const QVector2D &vector);
#endif

#ifndef QT_NO_DATASTREAM
Q_GUI_EXPORT QDataStream &operator<<(QDataStream &, const QVector2D &);
Q_GUI_EXPORT QDataStream &operator>>(QDataStream &, QVector2D &);
#endif

#endif

QT_END_NAMESPACE

#endif
