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

#ifndef QQUATERNION_H
#define QQUATERNION_H

#include <QtGui/qtguiglobal.h>
#include <QtGui/qgenericmatrix.h>
#include <QtGui/qvector3d.h>
#include <QtGui/qvector4d.h>

QT_BEGIN_NAMESPACE


#ifndef QT_NO_QUATERNION

class QMatrix4x4;
class QVariant;

class Q_GUI_EXPORT QQuaternion
{
public:
    QQuaternion();
    explicit QQuaternion(Qt::Initialization) {}
    QQuaternion(float scalar, float xpos, float ypos, float zpos);
#ifndef QT_NO_VECTOR3D
    QQuaternion(float scalar, const QVector3D& vector);
#endif
#ifndef QT_NO_VECTOR4D
    explicit QQuaternion(const QVector4D& vector);
#endif

    bool isNull() const;
    bool isIdentity() const;

#ifndef QT_NO_VECTOR3D
    QVector3D vector() const;
    void setVector(const QVector3D& vector);
#endif
    void setVector(float x, float y, float z);

    float x() const;
    float y() const;
    float z() const;
    float scalar() const;

    void setX(float x);
    void setY(float y);
    void setZ(float z);
    void setScalar(float scalar);

    Q_DECL_CONSTEXPR static inline float dotProduct(const QQuaternion &q1, const QQuaternion &q2);

    float length() const;
    float lengthSquared() const;

    Q_REQUIRED_RESULT QQuaternion normalized() const;
    void normalize();

    inline QQuaternion inverted() const;

    Q_REQUIRED_RESULT QQuaternion conjugated() const;
#if QT_DEPRECATED_SINCE(5, 5)
    Q_REQUIRED_RESULT QT_DEPRECATED QQuaternion conjugate() const;
#endif

    QVector3D rotatedVector(const QVector3D& vector) const;

    QQuaternion &operator+=(const QQuaternion &quaternion);
    QQuaternion &operator-=(const QQuaternion &quaternion);
    QQuaternion &operator*=(float factor);
    QQuaternion &operator*=(const QQuaternion &quaternion);
    QQuaternion &operator/=(float divisor);

    friend inline bool operator==(const QQuaternion &q1, const QQuaternion &q2);
    friend inline bool operator!=(const QQuaternion &q1, const QQuaternion &q2);
    friend inline const QQuaternion operator+(const QQuaternion &q1, const QQuaternion &q2);
    friend inline const QQuaternion operator-(const QQuaternion &q1, const QQuaternion &q2);
    friend inline const QQuaternion operator*(float factor, const QQuaternion &quaternion);
    friend inline const QQuaternion operator*(const QQuaternion &quaternion, float factor);
    friend inline const QQuaternion operator*(const QQuaternion &q1, const QQuaternion& q2);
    friend inline const QQuaternion operator-(const QQuaternion &quaternion);
    friend inline const QQuaternion operator/(const QQuaternion &quaternion, float divisor);

    friend inline bool qFuzzyCompare(const QQuaternion& q1, const QQuaternion& q2);

#ifndef QT_NO_VECTOR4D
    QVector4D toVector4D() const;
#endif

    operator QVariant() const;

#ifndef QT_NO_VECTOR3D
    inline void getAxisAndAngle(QVector3D *axis, float *angle) const;
    static QQuaternion fromAxisAndAngle(const QVector3D& axis, float angle);
#endif
    void getAxisAndAngle(float *x, float *y, float *z, float *angle) const;
    static QQuaternion fromAxisAndAngle
            (float x, float y, float z, float angle);

#ifndef QT_NO_VECTOR3D
    inline QVector3D toEulerAngles() const;
    static inline QQuaternion fromEulerAngles(const QVector3D &eulerAngles);
#endif
    void getEulerAngles(float *pitch, float *yaw, float *roll) const;
    static QQuaternion fromEulerAngles(float pitch, float yaw, float roll);

    QMatrix3x3 toRotationMatrix() const;
    static QQuaternion fromRotationMatrix(const QMatrix3x3 &rot3x3);

#ifndef QT_NO_VECTOR3D
    void getAxes(QVector3D *xAxis, QVector3D *yAxis, QVector3D *zAxis) const;
    static QQuaternion fromAxes(const QVector3D &xAxis, const QVector3D &yAxis, const QVector3D &zAxis);

    static QQuaternion fromDirection(const QVector3D &direction, const QVector3D &up);

    static QQuaternion rotationTo(const QVector3D &from, const QVector3D &to);
#endif

    static QQuaternion slerp
        (const QQuaternion& q1, const QQuaternion& q2, float t);
    static QQuaternion nlerp
        (const QQuaternion& q1, const QQuaternion& q2, float t);

private:
    float wp, xp, yp, zp;
};

Q_DECLARE_TYPEINFO(QQuaternion, Q_MOVABLE_TYPE);

inline QQuaternion::QQuaternion() : wp(1.0f), xp(0.0f), yp(0.0f), zp(0.0f) {}

inline QQuaternion::QQuaternion(float aScalar, float xpos, float ypos, float zpos) : wp(aScalar), xp(xpos), yp(ypos), zp(zpos) {}

QT_WARNING_PUSH
QT_WARNING_DISABLE_CLANG("-Wfloat-equal")
QT_WARNING_DISABLE_GCC("-Wfloat-equal")
QT_WARNING_DISABLE_INTEL(1572)
inline bool QQuaternion::isNull() const
{
    return wp == 0.0f && xp == 0.0f && yp == 0.0f && zp == 0.0f;
}

inline bool QQuaternion::isIdentity() const
{
    return wp == 1.0f && xp == 0.0f && yp == 0.0f && zp == 0.0f;
}

inline bool operator==(const QQuaternion &q1, const QQuaternion &q2)
{
    return q1.wp == q2.wp && q1.xp == q2.xp && q1.yp == q2.yp && q1.zp == q2.zp;
}
QT_WARNING_POP

inline float QQuaternion::x() const { return xp; }
inline float QQuaternion::y() const { return yp; }
inline float QQuaternion::z() const { return zp; }
inline float QQuaternion::scalar() const { return wp; }

inline void QQuaternion::setX(float aX) { xp = aX; }
inline void QQuaternion::setY(float aY) { yp = aY; }
inline void QQuaternion::setZ(float aZ) { zp = aZ; }
inline void QQuaternion::setScalar(float aScalar) { wp = aScalar; }

Q_DECL_CONSTEXPR inline float QQuaternion::dotProduct(const QQuaternion &q1, const QQuaternion &q2)
{
    return q1.wp * q2.wp + q1.xp * q2.xp + q1.yp * q2.yp + q1.zp * q2.zp;
}

inline QQuaternion QQuaternion::inverted() const
{
    // Need some extra precision if the length is very small.
    double len = double(wp) * double(wp) +
                 double(xp) * double(xp) +
                 double(yp) * double(yp) +
                 double(zp) * double(zp);
    if (!qFuzzyIsNull(len))
        return QQuaternion(float(double(wp) / len), float(double(-xp) / len),
                           float(double(-yp) / len), float(double(-zp) / len));
    return QQuaternion(0.0f, 0.0f, 0.0f, 0.0f);
}

inline QQuaternion QQuaternion::conjugated() const
{
    return QQuaternion(wp, -xp, -yp, -zp);
}

#if QT_DEPRECATED_SINCE(5, 5)
inline QQuaternion QQuaternion::conjugate() const
{
    return conjugated();
}
#endif

inline QQuaternion &QQuaternion::operator+=(const QQuaternion &quaternion)
{
    wp += quaternion.wp;
    xp += quaternion.xp;
    yp += quaternion.yp;
    zp += quaternion.zp;
    return *this;
}

inline QQuaternion &QQuaternion::operator-=(const QQuaternion &quaternion)
{
    wp -= quaternion.wp;
    xp -= quaternion.xp;
    yp -= quaternion.yp;
    zp -= quaternion.zp;
    return *this;
}

inline QQuaternion &QQuaternion::operator*=(float factor)
{
    wp *= factor;
    xp *= factor;
    yp *= factor;
    zp *= factor;
    return *this;
}

inline const QQuaternion operator*(const QQuaternion &q1, const QQuaternion& q2)
{
    float yy = (q1.wp - q1.yp) * (q2.wp + q2.zp);
    float zz = (q1.wp + q1.yp) * (q2.wp - q2.zp);
    float ww = (q1.zp + q1.xp) * (q2.xp + q2.yp);
    float xx = ww + yy + zz;
    float qq = 0.5f * (xx + (q1.zp - q1.xp) * (q2.xp - q2.yp));

    float w = qq - ww + (q1.zp - q1.yp) * (q2.yp - q2.zp);
    float x = qq - xx + (q1.xp + q1.wp) * (q2.xp + q2.wp);
    float y = qq - yy + (q1.wp - q1.xp) * (q2.yp + q2.zp);
    float z = qq - zz + (q1.zp + q1.yp) * (q2.wp - q2.xp);

    return QQuaternion(w, x, y, z);
}

inline QQuaternion &QQuaternion::operator*=(const QQuaternion &quaternion)
{
    *this = *this * quaternion;
    return *this;
}

inline QQuaternion &QQuaternion::operator/=(float divisor)
{
    wp /= divisor;
    xp /= divisor;
    yp /= divisor;
    zp /= divisor;
    return *this;
}

inline bool operator!=(const QQuaternion &q1, const QQuaternion &q2)
{
    return !operator==(q1, q2);
}

inline const QQuaternion operator+(const QQuaternion &q1, const QQuaternion &q2)
{
    return QQuaternion(q1.wp + q2.wp, q1.xp + q2.xp, q1.yp + q2.yp, q1.zp + q2.zp);
}

inline const QQuaternion operator-(const QQuaternion &q1, const QQuaternion &q2)
{
    return QQuaternion(q1.wp - q2.wp, q1.xp - q2.xp, q1.yp - q2.yp, q1.zp - q2.zp);
}

inline const QQuaternion operator*(float factor, const QQuaternion &quaternion)
{
    return QQuaternion(quaternion.wp * factor, quaternion.xp * factor, quaternion.yp * factor, quaternion.zp * factor);
}

inline const QQuaternion operator*(const QQuaternion &quaternion, float factor)
{
    return QQuaternion(quaternion.wp * factor, quaternion.xp * factor, quaternion.yp * factor, quaternion.zp * factor);
}

inline const QQuaternion operator-(const QQuaternion &quaternion)
{
    return QQuaternion(-quaternion.wp, -quaternion.xp, -quaternion.yp, -quaternion.zp);
}

inline const QQuaternion operator/(const QQuaternion &quaternion, float divisor)
{
    return QQuaternion(quaternion.wp / divisor, quaternion.xp / divisor, quaternion.yp / divisor, quaternion.zp / divisor);
}

inline bool qFuzzyCompare(const QQuaternion& q1, const QQuaternion& q2)
{
    return qFuzzyCompare(q1.wp, q2.wp) &&
           qFuzzyCompare(q1.xp, q2.xp) &&
           qFuzzyCompare(q1.yp, q2.yp) &&
           qFuzzyCompare(q1.zp, q2.zp);
}

#ifndef QT_NO_VECTOR3D

inline QQuaternion::QQuaternion(float aScalar, const QVector3D& aVector)
    : wp(aScalar), xp(aVector.x()), yp(aVector.y()), zp(aVector.z()) {}

inline void QQuaternion::setVector(const QVector3D& aVector)
{
    xp = aVector.x();
    yp = aVector.y();
    zp = aVector.z();
}

inline QVector3D QQuaternion::vector() const
{
    return QVector3D(xp, yp, zp);
}

inline QVector3D operator*(const QQuaternion &quaternion, const QVector3D &vec)
{
    return quaternion.rotatedVector(vec);
}

inline void QQuaternion::getAxisAndAngle(QVector3D *axis, float *angle) const
{
    float aX, aY, aZ;
    getAxisAndAngle(&aX, &aY, &aZ, angle);
    *axis = QVector3D(aX, aY, aZ);
}

inline QVector3D QQuaternion::toEulerAngles() const
{
    float pitch, yaw, roll;
    getEulerAngles(&pitch, &yaw, &roll);
    return QVector3D(pitch, yaw, roll);
}

inline QQuaternion QQuaternion::fromEulerAngles(const QVector3D &eulerAngles)
{
    return QQuaternion::fromEulerAngles(eulerAngles.x(), eulerAngles.y(), eulerAngles.z());
}

#endif

inline void QQuaternion::setVector(float aX, float aY, float aZ)
{
    xp = aX;
    yp = aY;
    zp = aZ;
}

#ifndef QT_NO_VECTOR4D

inline QQuaternion::QQuaternion(const QVector4D& aVector)
    : wp(aVector.w()), xp(aVector.x()), yp(aVector.y()), zp(aVector.z()) {}

inline QVector4D QQuaternion::toVector4D() const
{
    return QVector4D(xp, yp, zp, wp);
}

#endif

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug dbg, const QQuaternion &q);
#endif

#ifndef QT_NO_DATASTREAM
Q_GUI_EXPORT QDataStream &operator<<(QDataStream &, const QQuaternion &);
Q_GUI_EXPORT QDataStream &operator>>(QDataStream &, QQuaternion &);
#endif

#endif

QT_END_NAMESPACE

#endif
