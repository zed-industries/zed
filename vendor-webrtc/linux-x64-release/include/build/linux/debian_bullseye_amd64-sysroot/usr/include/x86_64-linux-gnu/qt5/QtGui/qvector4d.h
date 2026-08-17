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

#ifndef QVECTOR4D_H
#define QVECTOR4D_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qpoint.h>
#include <QtCore/qmetatype.h>

QT_BEGIN_NAMESPACE


class QMatrix4x4;
class QVector2D;
class QVector3D;

#ifndef QT_NO_VECTOR4D

class Q_GUI_EXPORT QVector4D
{
public:
    Q_DECL_CONSTEXPR QVector4D();
    explicit QVector4D(Qt::Initialization) {}
    Q_DECL_CONSTEXPR QVector4D(float xpos, float ypos, float zpos, float wpos);
    Q_DECL_CONSTEXPR explicit QVector4D(const QPoint& point);
    Q_DECL_CONSTEXPR explicit QVector4D(const QPointF& point);
#ifndef QT_NO_VECTOR2D
    QVector4D(const QVector2D& vector);
    QVector4D(const QVector2D& vector, float zpos, float wpos);
#endif
#ifndef QT_NO_VECTOR3D
    QVector4D(const QVector3D& vector);
    QVector4D(const QVector3D& vector, float wpos);
#endif

    bool isNull() const;

    Q_DECL_CONSTEXPR float x() const;
    Q_DECL_CONSTEXPR float y() const;
    Q_DECL_CONSTEXPR float z() const;
    Q_DECL_CONSTEXPR float w() const;

    void setX(float x);
    void setY(float y);
    void setZ(float z);
    void setW(float w);

    float &operator[](int i);
    float operator[](int i) const;

    float length() const;
    float lengthSquared() const; //In Qt 6 convert to inline and constexpr

    Q_REQUIRED_RESULT QVector4D normalized() const;
    void normalize();

    QVector4D &operator+=(const QVector4D &vector);
    QVector4D &operator-=(const QVector4D &vector);
    QVector4D &operator*=(float factor);
    QVector4D &operator*=(const QVector4D &vector);
    QVector4D &operator/=(float divisor);
    inline QVector4D &operator/=(const QVector4D &vector);

    static float dotProduct(const QVector4D& v1, const QVector4D& v2); //In Qt 6 convert to inline and constexpr

    Q_DECL_CONSTEXPR friend inline bool operator==(const QVector4D &v1, const QVector4D &v2);
    Q_DECL_CONSTEXPR friend inline bool operator!=(const QVector4D &v1, const QVector4D &v2);
    Q_DECL_CONSTEXPR friend inline const QVector4D operator+(const QVector4D &v1, const QVector4D &v2);
    Q_DECL_CONSTEXPR friend inline const QVector4D operator-(const QVector4D &v1, const QVector4D &v2);
    Q_DECL_CONSTEXPR friend inline const QVector4D operator*(float factor, const QVector4D &vector);
    Q_DECL_CONSTEXPR friend inline const QVector4D operator*(const QVector4D &vector, float factor);
    Q_DECL_CONSTEXPR friend inline const QVector4D operator*(const QVector4D &v1, const QVector4D& v2);
    Q_DECL_CONSTEXPR friend inline const QVector4D operator-(const QVector4D &vector);
    Q_DECL_CONSTEXPR friend inline const QVector4D operator/(const QVector4D &vector, float divisor);
    Q_DECL_CONSTEXPR friend inline const QVector4D operator/(const QVector4D &vector, const QVector4D &divisor);

    Q_DECL_CONSTEXPR friend inline bool qFuzzyCompare(const QVector4D& v1, const QVector4D& v2);

#ifndef QT_NO_VECTOR2D
    QVector2D toVector2D() const;
    QVector2D toVector2DAffine() const;
#endif
#ifndef QT_NO_VECTOR3D
    QVector3D toVector3D() const;
    QVector3D toVector3DAffine() const;
#endif

    Q_DECL_CONSTEXPR QPoint toPoint() const;
    Q_DECL_CONSTEXPR QPointF toPointF() const;

    operator QVariant() const;

private:
    float v[4];

    friend class QVector2D;
    friend class QVector3D;
#ifndef QT_NO_MATRIX4X4
    friend QVector4D operator*(const QVector4D& vector, const QMatrix4x4& matrix);
    friend QVector4D operator*(const QMatrix4x4& matrix, const QVector4D& vector);
#endif
};

Q_DECLARE_TYPEINFO(QVector4D, Q_PRIMITIVE_TYPE);

Q_DECL_CONSTEXPR inline QVector4D::QVector4D() : v{0.0f, 0.0f, 0.0f, 0.0f} {}

Q_DECL_CONSTEXPR inline QVector4D::QVector4D(float xpos, float ypos, float zpos, float wpos) : v{xpos, ypos, zpos, wpos} {}

Q_DECL_CONSTEXPR inline QVector4D::QVector4D(const QPoint& point) : v{float(point.x()), float(point.y()), 0.0f, 0.0f} {}

Q_DECL_CONSTEXPR inline QVector4D::QVector4D(const QPointF& point) : v{float(point.x()), float(point.y()), 0.0f, 0.0f} {}

inline bool QVector4D::isNull() const
{
    return qIsNull(v[0]) && qIsNull(v[1]) && qIsNull(v[2]) && qIsNull(v[3]);
}

Q_DECL_CONSTEXPR inline float QVector4D::x() const { return v[0]; }
Q_DECL_CONSTEXPR inline float QVector4D::y() const { return v[1]; }
Q_DECL_CONSTEXPR inline float QVector4D::z() const { return v[2]; }
Q_DECL_CONSTEXPR inline float QVector4D::w() const { return v[3]; }

inline void QVector4D::setX(float aX) { v[0] = aX; }
inline void QVector4D::setY(float aY) { v[1] = aY; }
inline void QVector4D::setZ(float aZ) { v[2] = aZ; }
inline void QVector4D::setW(float aW) { v[3] = aW; }

inline float &QVector4D::operator[](int i)
{
    Q_ASSERT(uint(i) < 4u);
    return v[i];
}

inline float QVector4D::operator[](int i) const
{
    Q_ASSERT(uint(i) < 4u);
    return v[i];
}

inline QVector4D &QVector4D::operator+=(const QVector4D &vector)
{
    v[0] += vector.v[0];
    v[1] += vector.v[1];
    v[2] += vector.v[2];
    v[3] += vector.v[3];
    return *this;
}

inline QVector4D &QVector4D::operator-=(const QVector4D &vector)
{
    v[0] -= vector.v[0];
    v[1] -= vector.v[1];
    v[2] -= vector.v[2];
    v[3] -= vector.v[3];
    return *this;
}

inline QVector4D &QVector4D::operator*=(float factor)
{
    v[0] *= factor;
    v[1] *= factor;
    v[2] *= factor;
    v[3] *= factor;
    return *this;
}

inline QVector4D &QVector4D::operator*=(const QVector4D &vector)
{
    v[0] *= vector.v[0];
    v[1] *= vector.v[1];
    v[2] *= vector.v[2];
    v[3] *= vector.v[3];
    return *this;
}

inline QVector4D &QVector4D::operator/=(float divisor)
{
    v[0] /= divisor;
    v[1] /= divisor;
    v[2] /= divisor;
    v[3] /= divisor;
    return *this;
}

inline QVector4D &QVector4D::operator/=(const QVector4D &vector)
{
    v[0] /= vector.v[0];
    v[1] /= vector.v[1];
    v[2] /= vector.v[2];
    v[3] /= vector.v[3];
    return *this;
}

QT_WARNING_PUSH
QT_WARNING_DISABLE_CLANG("-Wfloat-equal")
QT_WARNING_DISABLE_GCC("-Wfloat-equal")
QT_WARNING_DISABLE_INTEL(1572)
Q_DECL_CONSTEXPR inline bool operator==(const QVector4D &v1, const QVector4D &v2)
{
    return v1.v[0] == v2.v[0] && v1.v[1] == v2.v[1] && v1.v[2] == v2.v[2] && v1.v[3] == v2.v[3];
}

Q_DECL_CONSTEXPR inline bool operator!=(const QVector4D &v1, const QVector4D &v2)
{
    return v1.v[0] != v2.v[0] || v1.v[1] != v2.v[1] || v1.v[2] != v2.v[2] || v1.v[3] != v2.v[3];
}
QT_WARNING_POP

Q_DECL_CONSTEXPR inline const QVector4D operator+(const QVector4D &v1, const QVector4D &v2)
{
    return QVector4D(v1.v[0] + v2.v[0], v1.v[1] + v2.v[1], v1.v[2] + v2.v[2], v1.v[3] + v2.v[3]);
}

Q_DECL_CONSTEXPR inline const QVector4D operator-(const QVector4D &v1, const QVector4D &v2)
{
    return QVector4D(v1.v[0] - v2.v[0], v1.v[1] - v2.v[1], v1.v[2] - v2.v[2], v1.v[3] - v2.v[3]);
}

Q_DECL_CONSTEXPR inline const QVector4D operator*(float factor, const QVector4D &vector)
{
    return QVector4D(vector.v[0] * factor, vector.v[1] * factor, vector.v[2] * factor, vector.v[3] * factor);
}

Q_DECL_CONSTEXPR inline const QVector4D operator*(const QVector4D &vector, float factor)
{
    return QVector4D(vector.v[0] * factor, vector.v[1] * factor, vector.v[2] * factor, vector.v[3] * factor);
}

Q_DECL_CONSTEXPR inline const QVector4D operator*(const QVector4D &v1, const QVector4D& v2)
{
    return QVector4D(v1.v[0] * v2.v[0], v1.v[1] * v2.v[1], v1.v[2] * v2.v[2], v1.v[3] * v2.v[3]);
}

Q_DECL_CONSTEXPR inline const QVector4D operator-(const QVector4D &vector)
{
    return QVector4D(-vector.v[0], -vector.v[1], -vector.v[2], -vector.v[3]);
}

Q_DECL_CONSTEXPR inline const QVector4D operator/(const QVector4D &vector, float divisor)
{
    return QVector4D(vector.v[0] / divisor, vector.v[1] / divisor, vector.v[2] / divisor, vector.v[3] / divisor);
}

Q_DECL_CONSTEXPR inline const QVector4D operator/(const QVector4D &vector, const QVector4D &divisor)
{
    return QVector4D(vector.v[0] / divisor.v[0], vector.v[1] / divisor.v[1], vector.v[2] / divisor.v[2], vector.v[3] / divisor.v[3]);
}

Q_DECL_CONSTEXPR inline bool qFuzzyCompare(const QVector4D& v1, const QVector4D& v2)
{
    return qFuzzyCompare(v1.v[0], v2.v[0]) &&
           qFuzzyCompare(v1.v[1], v2.v[1]) &&
           qFuzzyCompare(v1.v[2], v2.v[2]) &&
           qFuzzyCompare(v1.v[3], v2.v[3]);
}

Q_DECL_CONSTEXPR inline QPoint QVector4D::toPoint() const
{
    return QPoint(qRound(v[0]), qRound(v[1]));
}

Q_DECL_CONSTEXPR inline QPointF QVector4D::toPointF() const
{
    return QPointF(qreal(v[0]), qreal(v[1]));
}

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug dbg, const QVector4D &vector);
#endif

#ifndef QT_NO_DATASTREAM
Q_GUI_EXPORT QDataStream &operator<<(QDataStream &, const QVector4D &);
Q_GUI_EXPORT QDataStream &operator>>(QDataStream &, QVector4D &);
#endif

#endif

QT_END_NAMESPACE

#endif
