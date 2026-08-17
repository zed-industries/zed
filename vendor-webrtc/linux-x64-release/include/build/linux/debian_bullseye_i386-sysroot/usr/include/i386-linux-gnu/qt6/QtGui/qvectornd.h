// Copyright (C) 2020 The Qt Company Ltd.
// Copyright (C) 2020 Klarälvdalens Datakonsult AB, a KDAB Group company, info@kdab.com, author Giuseppe D'Angelo <giuseppe.dangelo@kdab.com>
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QVECTORND_H
#define QVECTORND_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qpoint.h>
#include <QtCore/qrect.h>
#include <QtCore/qmath.h>

QT_BEGIN_NAMESPACE

class QVector2D;
class QVector3D;
class QVector4D;
class QMatrix4x4;
class QVariant;

/***************************** QVector2D *****************************/

#ifndef QT_NO_VECTOR2D

class QVector2D
{
public:
    constexpr QVector2D() noexcept;
    explicit QVector2D(Qt::Initialization) noexcept {}
    constexpr QVector2D(float xpos, float ypos) noexcept;
    constexpr explicit QVector2D(QPoint point) noexcept;
    constexpr explicit QVector2D(QPointF point) noexcept;
#ifndef QT_NO_VECTOR3D
    constexpr explicit QVector2D(QVector3D vector) noexcept;
#endif
#ifndef QT_NO_VECTOR4D
    constexpr explicit QVector2D(QVector4D vector) noexcept;
#endif

    constexpr bool isNull() const noexcept;

    constexpr float x() const noexcept;
    constexpr float y() const noexcept;

    constexpr void setX(float x) noexcept;
    constexpr void setY(float y) noexcept;

    constexpr float &operator[](int i);
    constexpr float operator[](int i) const;

    [[nodiscard]] float length() const noexcept;
    [[nodiscard]] constexpr float lengthSquared() const noexcept;

    [[nodiscard]] QVector2D normalized() const noexcept;
    void normalize() noexcept;

    [[nodiscard]] float distanceToPoint(QVector2D point) const noexcept;
    [[nodiscard]] float distanceToLine(QVector2D point, QVector2D direction) const noexcept;

    constexpr QVector2D &operator+=(QVector2D vector) noexcept;
    constexpr QVector2D &operator-=(QVector2D vector) noexcept;
    constexpr QVector2D &operator*=(float factor) noexcept;
    constexpr QVector2D &operator*=(QVector2D vector) noexcept;
    constexpr QVector2D &operator/=(float divisor);
    constexpr QVector2D &operator/=(QVector2D vector);

    [[nodiscard]] static constexpr float dotProduct(QVector2D v1, QVector2D v2) noexcept;

QT_WARNING_PUSH
QT_WARNING_DISABLE_FLOAT_COMPARE
    constexpr friend inline bool operator==(QVector2D v1, QVector2D v2) noexcept
    {
        return v1.v[0] == v2.v[0] && v1.v[1] == v2.v[1];
    }

    constexpr friend inline bool operator!=(QVector2D v1, QVector2D v2) noexcept
    {
        return v1.v[0] != v2.v[0] || v1.v[1] != v2.v[1];
    }
QT_WARNING_POP

    constexpr friend inline QVector2D operator+(QVector2D v1, QVector2D v2) noexcept
    {
        return QVector2D(v1.v[0] + v2.v[0], v1.v[1] + v2.v[1]);
    }

    constexpr friend inline QVector2D operator-(QVector2D v1, QVector2D v2) noexcept
    {
        return QVector2D(v1.v[0] - v2.v[0], v1.v[1] - v2.v[1]);
    }

    constexpr friend inline QVector2D operator*(float factor, QVector2D vector) noexcept
    {
        return QVector2D(vector.v[0] * factor, vector.v[1] * factor);
    }

    constexpr friend inline QVector2D operator*(QVector2D vector, float factor) noexcept
    {
        return QVector2D(vector.v[0] * factor, vector.v[1] * factor);
    }

    constexpr friend inline QVector2D operator*(QVector2D v1, QVector2D v2) noexcept
    {
        return QVector2D(v1.v[0] * v2.v[0], v1.v[1] * v2.v[1]);
    }

    constexpr friend inline QVector2D operator-(QVector2D vector) noexcept
    {
        return QVector2D(-vector.v[0], -vector.v[1]);
    }

    constexpr friend inline QVector2D operator/(QVector2D vector, float divisor)
    {
        Q_ASSERT(divisor < 0 || divisor > 0);
        return QVector2D(vector.v[0] / divisor, vector.v[1] / divisor);
    }

    constexpr friend inline QVector2D operator/(QVector2D vector, QVector2D divisor)
    {
        Q_ASSERT(divisor.v[0] < 0 || divisor.v[0] > 0);
        Q_ASSERT(divisor.v[1] < 0 || divisor.v[1] > 0);
        return QVector2D(vector.v[0] / divisor.v[0], vector.v[1] / divisor.v[1]);
    }

    friend Q_GUI_EXPORT bool qFuzzyCompare(QVector2D v1, QVector2D v2) noexcept;

#ifndef QT_NO_VECTOR3D
    constexpr QVector3D toVector3D() const noexcept;
#endif
#ifndef QT_NO_VECTOR4D
    constexpr QVector4D toVector4D() const noexcept;
#endif

    constexpr QPoint toPoint() const noexcept;
    constexpr QPointF toPointF() const noexcept;

    Q_GUI_EXPORT operator QVariant() const;

private:
    float v[2];

    friend class QVector3D;
    friend class QVector4D;

    template <std::size_t I,
              typename V,
              std::enable_if_t<(I < 2), bool> = true,
              std::enable_if_t<std::is_same_v<std::decay_t<V>, QVector2D>, bool> = true>
    friend constexpr decltype(auto) get(V &&vec) noexcept
    {
        if constexpr (I == 0)
            return (std::forward<V>(vec).v[0]);
        else if constexpr (I == 1)
            return (std::forward<V>(vec).v[1]);
    }
};

Q_DECLARE_TYPEINFO(QVector2D, Q_PRIMITIVE_TYPE);

#endif // QT_NO_VECTOR2D



/***************************** QVector3D *****************************/

#ifndef QT_NO_VECTOR3D

class QVector3D
{
public:
    constexpr QVector3D() noexcept;
    explicit QVector3D(Qt::Initialization) noexcept {}
    constexpr QVector3D(float xpos, float ypos, float zpos) noexcept : v{xpos, ypos, zpos} {}

    constexpr explicit QVector3D(QPoint point) noexcept;
    constexpr explicit QVector3D(QPointF point) noexcept;
#ifndef QT_NO_VECTOR2D
    constexpr explicit QVector3D(QVector2D vector) noexcept;
    constexpr QVector3D(QVector2D vector, float zpos) noexcept;
#endif
#ifndef QT_NO_VECTOR4D
    constexpr explicit QVector3D(QVector4D vector) noexcept;
#endif

    constexpr bool isNull() const noexcept;

    constexpr float x() const noexcept;
    constexpr float y() const noexcept;
    constexpr float z() const noexcept;

    constexpr void setX(float x) noexcept;
    constexpr void setY(float y) noexcept;
    constexpr void setZ(float z) noexcept;

    constexpr float &operator[](int i);
    constexpr float operator[](int i) const;

    [[nodiscard]] float length() const noexcept;
    [[nodiscard]] constexpr float lengthSquared() const noexcept;

    [[nodiscard]] QVector3D normalized() const noexcept;
    void normalize() noexcept;

    constexpr QVector3D &operator+=(QVector3D vector) noexcept;
    constexpr QVector3D &operator-=(QVector3D vector) noexcept;
    constexpr QVector3D &operator*=(float factor) noexcept;
    constexpr QVector3D &operator*=(QVector3D vector) noexcept;
    constexpr QVector3D &operator/=(float divisor);
    constexpr QVector3D &operator/=(QVector3D vector);

    [[nodiscard]] static constexpr float dotProduct(QVector3D v1, QVector3D v2) noexcept;
    [[nodiscard]] static constexpr QVector3D crossProduct(QVector3D v1, QVector3D v2) noexcept;

    [[nodiscard]] static QVector3D normal(QVector3D v1, QVector3D v2) noexcept;
    [[nodiscard]] static QVector3D normal(QVector3D v1, QVector3D v2, QVector3D v3) noexcept;

    Q_GUI_EXPORT QVector3D project(const QMatrix4x4 &modelView, const QMatrix4x4 &projection, const QRect &viewport) const;
    Q_GUI_EXPORT QVector3D unproject(const QMatrix4x4 &modelView, const QMatrix4x4 &projection, const QRect &viewport) const;

QT_WARNING_PUSH
QT_WARNING_DISABLE_FLOAT_COMPARE
    constexpr friend inline bool operator==(QVector3D v1, QVector3D v2) noexcept
    {
        return v1.v[0] == v2.v[0] && v1.v[1] == v2.v[1] && v1.v[2] == v2.v[2];
    }

    constexpr friend inline bool operator!=(QVector3D v1, QVector3D v2) noexcept
    {
        return v1.v[0] != v2.v[0] || v1.v[1] != v2.v[1] || v1.v[2] != v2.v[2];
    }
QT_WARNING_POP
    float distanceToPoint(QVector3D point) const noexcept;
    constexpr float distanceToPlane(QVector3D plane, QVector3D normal) const noexcept;
    float distanceToPlane(QVector3D plane1, QVector3D plane2, QVector3D plane3) const noexcept;
    float distanceToLine(QVector3D point, QVector3D direction) const noexcept;


    constexpr friend inline QVector3D operator+(QVector3D v1, QVector3D v2) noexcept
    {
        return QVector3D(v1.v[0] + v2.v[0], v1.v[1] + v2.v[1], v1.v[2] + v2.v[2]);
    }

    constexpr friend inline QVector3D operator-(QVector3D v1, QVector3D v2) noexcept
    {
        return QVector3D(v1.v[0] - v2.v[0], v1.v[1] - v2.v[1], v1.v[2] - v2.v[2]);
    }

    constexpr friend inline QVector3D operator*(float factor, QVector3D vector) noexcept
    {
        return QVector3D(vector.v[0] * factor, vector.v[1] * factor, vector.v[2] * factor);
    }

    constexpr friend inline QVector3D operator*(QVector3D vector, float factor) noexcept
    {
        return QVector3D(vector.v[0] * factor, vector.v[1] * factor, vector.v[2] * factor);
    }

    constexpr friend inline QVector3D operator*(QVector3D v1, QVector3D v2) noexcept
    {
        return QVector3D(v1.v[0] * v2.v[0], v1.v[1] * v2.v[1], v1.v[2] * v2.v[2]);
    }

    constexpr friend inline QVector3D operator-(QVector3D vector) noexcept
    {
        return QVector3D(-vector.v[0], -vector.v[1], -vector.v[2]);
    }

    constexpr friend inline QVector3D operator/(QVector3D vector, float divisor)
    {
        Q_ASSERT(divisor < 0 || divisor > 0);
        return QVector3D(vector.v[0] / divisor, vector.v[1] / divisor, vector.v[2] / divisor);
    }

    constexpr friend inline QVector3D operator/(QVector3D vector, QVector3D divisor)
    {
        Q_ASSERT(divisor.v[0] > 0 || divisor.v[0] < 0);
        Q_ASSERT(divisor.v[1] > 0 || divisor.v[1] < 0);
        Q_ASSERT(divisor.v[2] > 0 || divisor.v[2] < 0);
        return QVector3D(vector.v[0] / divisor.v[0], vector.v[1] / divisor.v[1],
                         vector.v[2] / divisor.v[2]);
    }

    friend Q_GUI_EXPORT bool qFuzzyCompare(QVector3D v1, QVector3D v2) noexcept;

#ifndef QT_NO_VECTOR2D
    constexpr QVector2D toVector2D() const noexcept;
#endif
#ifndef QT_NO_VECTOR4D
    constexpr QVector4D toVector4D() const noexcept;
#endif

    constexpr QPoint toPoint() const noexcept;
    constexpr QPointF toPointF() const noexcept;

    Q_GUI_EXPORT operator QVariant() const;

private:
    float v[3];

    friend class QVector2D;
    friend class QVector4D;
#ifndef QT_NO_MATRIX4X4
    friend QVector3D operator*(const QVector3D& vector, const QMatrix4x4& matrix);
    friend QVector3D operator*(const QMatrix4x4& matrix, const QVector3D& vector);
#endif

    template <std::size_t I,
              typename V,
              std::enable_if_t<(I < 3), bool> = true,
              std::enable_if_t<std::is_same_v<std::decay_t<V>, QVector3D>, bool> = true>
    friend constexpr decltype(auto) get(V &&vec) noexcept
    {
        if constexpr (I == 0)
            return (std::forward<V>(vec).v[0]);
        else if constexpr (I == 1)
            return (std::forward<V>(vec).v[1]);
        else if constexpr (I == 2)
            return (std::forward<V>(vec).v[2]);
    }
};

Q_DECLARE_TYPEINFO(QVector3D, Q_PRIMITIVE_TYPE);

#endif // QT_NO_VECTOR3D



/***************************** QVector4D *****************************/

#ifndef QT_NO_VECTOR4D

class QVector4D
{
public:
    constexpr QVector4D() noexcept;
    explicit QVector4D(Qt::Initialization) noexcept {}
    constexpr QVector4D(float xpos, float ypos, float zpos, float wpos) noexcept;
    constexpr explicit QVector4D(QPoint point) noexcept;
    constexpr explicit QVector4D(QPointF point) noexcept;
#ifndef QT_NO_VECTOR2D
    constexpr explicit QVector4D(QVector2D vector) noexcept;
    constexpr QVector4D(QVector2D vector, float zpos, float wpos) noexcept;
#endif
#ifndef QT_NO_VECTOR3D
    constexpr explicit QVector4D(QVector3D vector) noexcept;
    constexpr QVector4D(QVector3D vector, float wpos) noexcept;
#endif

    constexpr bool isNull() const noexcept;

    constexpr float x() const noexcept;
    constexpr float y() const noexcept;
    constexpr float z() const noexcept;
    constexpr float w() const noexcept;

    constexpr void setX(float x) noexcept;
    constexpr void setY(float y) noexcept;
    constexpr void setZ(float z) noexcept;
    constexpr void setW(float w) noexcept;

    constexpr float &operator[](int i);
    constexpr float operator[](int i) const;

    [[nodiscard]] float length() const noexcept;
    [[nodiscard]] constexpr float lengthSquared() const noexcept;

    [[nodiscard]] QVector4D normalized() const noexcept;
    void normalize() noexcept;

    constexpr QVector4D &operator+=(QVector4D vector) noexcept;
    constexpr QVector4D &operator-=(QVector4D vector) noexcept;
    constexpr QVector4D &operator*=(float factor) noexcept;
    constexpr QVector4D &operator*=(QVector4D vector) noexcept;
    constexpr QVector4D &operator/=(float divisor);
    constexpr inline QVector4D &operator/=(QVector4D vector);

    [[nodiscard]] static constexpr float dotProduct(QVector4D v1, QVector4D v2) noexcept;

QT_WARNING_PUSH
QT_WARNING_DISABLE_FLOAT_COMPARE
    constexpr friend inline bool operator==(QVector4D v1, QVector4D v2) noexcept
    {
        return v1.v[0] == v2.v[0] && v1.v[1] == v2.v[1] && v1.v[2] == v2.v[2] && v1.v[3] == v2.v[3];
    }

    constexpr friend inline bool operator!=(QVector4D v1, QVector4D v2) noexcept
    {
        return v1.v[0] != v2.v[0] || v1.v[1] != v2.v[1] || v1.v[2] != v2.v[2] || v1.v[3] != v2.v[3];
    }
QT_WARNING_POP
    constexpr friend inline QVector4D operator+(QVector4D v1, QVector4D v2) noexcept
    {
        return QVector4D(v1.v[0] + v2.v[0], v1.v[1] + v2.v[1], v1.v[2] + v2.v[2], v1.v[3] + v2.v[3]);
    }

    constexpr friend inline QVector4D operator-(QVector4D v1, QVector4D v2) noexcept
    {
        return QVector4D(v1.v[0] - v2.v[0], v1.v[1] - v2.v[1], v1.v[2] - v2.v[2], v1.v[3] - v2.v[3]);
    }

    constexpr friend inline QVector4D operator*(float factor, QVector4D vector) noexcept
    {
        return QVector4D(vector.v[0] * factor, vector.v[1] * factor, vector.v[2] * factor, vector.v[3] * factor);
    }

    constexpr friend inline QVector4D operator*(QVector4D vector, float factor) noexcept
    {
        return QVector4D(vector.v[0] * factor, vector.v[1] * factor, vector.v[2] * factor, vector.v[3] * factor);
    }

    constexpr friend inline QVector4D operator*(QVector4D v1, QVector4D v2) noexcept
    {
        return QVector4D(v1.v[0] * v2.v[0], v1.v[1] * v2.v[1], v1.v[2] * v2.v[2], v1.v[3] * v2.v[3]);
    }

    constexpr friend inline QVector4D operator-(QVector4D vector) noexcept
    {
        return QVector4D(-vector.v[0], -vector.v[1], -vector.v[2], -vector.v[3]);
    }

    constexpr friend inline QVector4D operator/(QVector4D vector, float divisor)
    {
        Q_ASSERT(divisor < 0 || divisor > 0);
        return QVector4D(vector.v[0] / divisor, vector.v[1] / divisor, vector.v[2] / divisor, vector.v[3] / divisor);
    }

    constexpr friend inline QVector4D operator/(QVector4D vector, QVector4D divisor)
    {
        Q_ASSERT(divisor.v[0] > 0 || divisor.v[0] < 0);
        Q_ASSERT(divisor.v[1] > 0 || divisor.v[1] < 0);
        Q_ASSERT(divisor.v[2] > 0 || divisor.v[2] < 0);
        Q_ASSERT(divisor.v[3] > 0 || divisor.v[3] < 0);
        return QVector4D(vector.v[0] / divisor.v[0], vector.v[1] / divisor.v[1],
                         vector.v[2] / divisor.v[2], vector.v[3] / divisor.v[3]);
    }

    friend Q_GUI_EXPORT bool qFuzzyCompare(QVector4D v1, QVector4D v2) noexcept;

#ifndef QT_NO_VECTOR2D
    constexpr QVector2D toVector2D() const noexcept;
    constexpr QVector2D toVector2DAffine() const noexcept;
#endif
#ifndef QT_NO_VECTOR3D
    constexpr QVector3D toVector3D() const noexcept;
    constexpr QVector3D toVector3DAffine() const noexcept;
#endif

    constexpr QPoint toPoint() const noexcept;
    constexpr QPointF toPointF() const noexcept;

    Q_GUI_EXPORT operator QVariant() const;

private:
    float v[4];

    friend class QVector2D;
    friend class QVector3D;
    friend class QMatrix4x4;
#ifndef QT_NO_MATRIX4X4
    friend QVector4D operator*(const QVector4D& vector, const QMatrix4x4& matrix);
    friend QVector4D operator*(const QMatrix4x4& matrix, const QVector4D& vector);
#endif

    template <std::size_t I,
              typename V,
              std::enable_if_t<(I < 4), bool> = true,
              std::enable_if_t<std::is_same_v<std::decay_t<V>, QVector4D>, bool> = true>
    friend constexpr decltype(auto) get(V &&vec) noexcept
    {
        if constexpr (I == 0)
            return (std::forward<V>(vec).v[0]);
        else if constexpr (I == 1)
            return (std::forward<V>(vec).v[1]);
        else if constexpr (I == 2)
            return (std::forward<V>(vec).v[2]);
        else if constexpr (I == 3)
            return (std::forward<V>(vec).v[3]);
    }
};

Q_DECLARE_TYPEINFO(QVector4D, Q_PRIMITIVE_TYPE);

#endif // QT_NO_VECTOR4D



/***************************** QVector2D *****************************/

#ifndef QT_NO_VECTOR2D

constexpr inline QVector2D::QVector2D() noexcept : v{0.0f, 0.0f} {}

constexpr inline QVector2D::QVector2D(float xpos, float ypos) noexcept : v{xpos, ypos} {}

constexpr inline QVector2D::QVector2D(QPoint point) noexcept : v{float(point.x()), float(point.y())} {}

constexpr inline QVector2D::QVector2D(QPointF point) noexcept : v{float(point.x()), float(point.y())} {}

#ifndef QT_NO_VECTOR3D
constexpr inline QVector2D::QVector2D(QVector3D vector) noexcept : v{vector[0], vector[1]} {}
#endif
#ifndef QT_NO_VECTOR4D
constexpr inline QVector2D::QVector2D(QVector4D vector) noexcept : v{vector[0], vector[1]} {}
#endif

constexpr inline bool QVector2D::isNull() const noexcept
{
    return qIsNull(v[0]) && qIsNull(v[1]);
}

constexpr inline float QVector2D::x() const noexcept { return v[0]; }
constexpr inline float QVector2D::y() const noexcept { return v[1]; }

constexpr inline void QVector2D::setX(float aX) noexcept { v[0] = aX; }
constexpr inline void QVector2D::setY(float aY) noexcept { v[1] = aY; }

constexpr inline float &QVector2D::operator[](int i)
{
    Q_ASSERT(uint(i) < 2u);
    return v[i];
}

constexpr inline float QVector2D::operator[](int i) const
{
    Q_ASSERT(uint(i) < 2u);
    return v[i];
}

inline float QVector2D::length() const noexcept
{
    return qHypot(v[0], v[1]);
}

constexpr inline float QVector2D::lengthSquared() const noexcept
{
    return v[0] * v[0] + v[1] * v[1];
}

inline QVector2D QVector2D::normalized() const noexcept
{
    const float len = length();
    return qFuzzyIsNull(len - 1.0f) ? *this : qFuzzyIsNull(len) ? QVector2D()
        : QVector2D(v[0] / len, v[1] / len);
}

inline void QVector2D::normalize() noexcept
{
    const float len = length();
    if (qFuzzyIsNull(len - 1.0f) || qFuzzyIsNull(len))
        return;

    v[0] /= len;
    v[1] /= len;
}

inline float QVector2D::distanceToPoint(QVector2D point) const noexcept
{
    return (*this - point).length();
}

inline float QVector2D::distanceToLine(QVector2D point, QVector2D direction) const noexcept
{
    if (direction.isNull())
        return (*this - point).length();
    QVector2D p = point + dotProduct(*this - point, direction) * direction;
    return (*this - p).length();
}

constexpr inline QVector2D &QVector2D::operator+=(QVector2D vector) noexcept
{
    v[0] += vector.v[0];
    v[1] += vector.v[1];
    return *this;
}

constexpr inline QVector2D &QVector2D::operator-=(QVector2D vector) noexcept
{
    v[0] -= vector.v[0];
    v[1] -= vector.v[1];
    return *this;
}

constexpr inline QVector2D &QVector2D::operator*=(float factor) noexcept
{
    v[0] *= factor;
    v[1] *= factor;
    return *this;
}

constexpr inline QVector2D &QVector2D::operator*=(QVector2D vector) noexcept
{
    v[0] *= vector.v[0];
    v[1] *= vector.v[1];
    return *this;
}

constexpr inline QVector2D &QVector2D::operator/=(float divisor)
{
    Q_ASSERT(divisor < 0 || divisor > 0);
    v[0] /= divisor;
    v[1] /= divisor;
    return *this;
}

constexpr inline QVector2D &QVector2D::operator/=(QVector2D vector)
{
    Q_ASSERT(vector.v[0] > 0 || vector.v[0] < 0);
    Q_ASSERT(vector.v[1] > 0 || vector.v[1] < 0);
    v[0] /= vector.v[0];
    v[1] /= vector.v[1];
    return *this;
}

constexpr inline float QVector2D::dotProduct(QVector2D v1, QVector2D v2) noexcept
{
    return v1.v[0] * v2.v[0] + v1.v[1] * v2.v[1];
}

#ifndef QT_NO_VECTOR3D
constexpr inline QVector3D QVector2D::toVector3D() const noexcept
{
    return QVector3D(v[0], v[1], 0.0f);
}
#endif
#ifndef QT_NO_VECTOR4D
constexpr inline QVector4D QVector2D::toVector4D() const noexcept
{
    return QVector4D(v[0], v[1], 0.0f, 0.0f);
}
#endif


constexpr inline QPoint QVector2D::toPoint() const noexcept
{
    return QPoint(qRound(v[0]), qRound(v[1]));
}

constexpr inline QPointF QVector2D::toPointF() const noexcept
{
    return QPointF(qreal(v[0]), qreal(v[1]));
}

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug dbg, QVector2D vector);
#endif

#ifndef QT_NO_DATASTREAM
Q_GUI_EXPORT QDataStream &operator<<(QDataStream &, QVector2D );
Q_GUI_EXPORT QDataStream &operator>>(QDataStream &, QVector2D &);
#endif

#endif // QT_NO_VECTOR2D



/***************************** QVector3D *****************************/

#ifndef QT_NO_VECTOR3D

constexpr inline QVector3D::QVector3D() noexcept : v{0.0f, 0.0f, 0.0f} {}

constexpr inline QVector3D::QVector3D(QPoint point) noexcept : v{float(point.x()), float(point.y()), 0.0f} {}

constexpr inline QVector3D::QVector3D(QPointF point) noexcept : v{float(point.x()), float(point.y()), 0.0f} {}

#ifndef QT_NO_VECTOR2D
constexpr inline QVector3D::QVector3D(QVector2D vector) noexcept : v{vector[0], vector[1], 0.0f} {}
constexpr inline QVector3D::QVector3D(QVector2D vector, float zpos) noexcept : v{vector[0], vector[1], zpos} {}
#endif

#ifndef QT_NO_VECTOR4D
constexpr inline QVector3D::QVector3D(QVector4D vector) noexcept : v{vector[0], vector[1], vector[2]} {}
#endif

constexpr inline bool QVector3D::isNull() const noexcept
{
    return qIsNull(v[0]) && qIsNull(v[1]) && qIsNull(v[2]);
}

constexpr inline float QVector3D::x() const noexcept { return v[0]; }
constexpr inline float QVector3D::y() const noexcept { return v[1]; }
constexpr inline float QVector3D::z() const noexcept { return v[2]; }

constexpr inline void QVector3D::setX(float aX) noexcept { v[0] = aX; }
constexpr inline void QVector3D::setY(float aY) noexcept { v[1] = aY; }
constexpr inline void QVector3D::setZ(float aZ) noexcept { v[2] = aZ; }

constexpr inline float &QVector3D::operator[](int i)
{
    Q_ASSERT(uint(i) < 3u);
    return v[i];
}

constexpr inline float QVector3D::operator[](int i) const
{
    Q_ASSERT(uint(i) < 3u);
    return v[i];
}

inline float QVector3D::length() const noexcept
{
    return qHypot(v[0], v[1], v[2]);
}

inline QVector3D QVector3D::normalized() const noexcept
{
    const float len = length();
    return qFuzzyIsNull(len - 1.0f) ? *this : qFuzzyIsNull(len) ? QVector3D()
        : QVector3D(v[0] / len, v[1] / len, v[2] / len);
}

inline void QVector3D::normalize() noexcept
{
    const float len = length();
    if (qFuzzyIsNull(len - 1.0f) || qFuzzyIsNull(len))
        return;

    v[0] /= len;
    v[1] /= len;
    v[2] /= len;
}

constexpr inline float QVector3D::lengthSquared() const noexcept
{
    return v[0] * v[0] + v[1] * v[1] + v[2] * v[2];
}

constexpr inline QVector3D &QVector3D::operator+=(QVector3D vector) noexcept
{
    v[0] += vector.v[0];
    v[1] += vector.v[1];
    v[2] += vector.v[2];
    return *this;
}

constexpr inline QVector3D &QVector3D::operator-=(QVector3D vector) noexcept
{
    v[0] -= vector.v[0];
    v[1] -= vector.v[1];
    v[2] -= vector.v[2];
    return *this;
}

constexpr inline QVector3D &QVector3D::operator*=(float factor) noexcept
{
    v[0] *= factor;
    v[1] *= factor;
    v[2] *= factor;
    return *this;
}

constexpr inline QVector3D &QVector3D::operator*=(QVector3D vector) noexcept
{
    v[0] *= vector.v[0];
    v[1] *= vector.v[1];
    v[2] *= vector.v[2];
    return *this;
}

constexpr inline QVector3D &QVector3D::operator/=(float divisor)
{
    Q_ASSERT(divisor < 0 || divisor > 0);
    v[0] /= divisor;
    v[1] /= divisor;
    v[2] /= divisor;
    return *this;
}

constexpr inline QVector3D &QVector3D::operator/=(QVector3D vector)
{
    Q_ASSERT(vector.v[0] > 0 || vector.v[0] < 0);
    Q_ASSERT(vector.v[1] > 0 || vector.v[1] < 0);
    Q_ASSERT(vector.v[2] > 0 || vector.v[2] < 0);
    v[0] /= vector.v[0];
    v[1] /= vector.v[1];
    v[2] /= vector.v[2];
    return *this;
}

constexpr inline float QVector3D::dotProduct(QVector3D v1, QVector3D v2) noexcept
{
    return v1.v[0] * v2.v[0] + v1.v[1] * v2.v[1] + v1.v[2] * v2.v[2];
}

constexpr inline QVector3D QVector3D::crossProduct(QVector3D v1, QVector3D v2) noexcept
{
    return QVector3D(v1.v[1] * v2.v[2] - v1.v[2] * v2.v[1],
                     v1.v[2] * v2.v[0] - v1.v[0] * v2.v[2],
                     v1.v[0] * v2.v[1] - v1.v[1] * v2.v[0]);
}

inline QVector3D QVector3D::normal(QVector3D v1, QVector3D v2) noexcept
{
    return crossProduct(v1, v2).normalized();
}

inline QVector3D QVector3D::normal(QVector3D v1, QVector3D v2, QVector3D v3) noexcept
{
    return crossProduct((v2 - v1), (v3 - v1)).normalized();
}

inline float QVector3D::distanceToPoint(QVector3D point) const noexcept
{
    return (*this - point).length();
}

constexpr inline float QVector3D::distanceToPlane(QVector3D plane, QVector3D normal) const noexcept
{
    return dotProduct(*this - plane, normal);
}

inline float QVector3D::distanceToPlane(QVector3D plane1, QVector3D plane2, QVector3D plane3) const noexcept
{
    QVector3D n = normal(plane2 - plane1, plane3 - plane1);
    return dotProduct(*this - plane1, n);
}

inline float QVector3D::distanceToLine(QVector3D point, QVector3D direction) const noexcept
{
    if (direction.isNull())
        return (*this - point).length();
    QVector3D p = point + dotProduct(*this - point, direction) * direction;
    return (*this - p).length();
}

#ifndef QT_NO_VECTOR2D
constexpr inline QVector2D QVector3D::toVector2D() const noexcept
{
    return QVector2D(v[0], v[1]);
}
#endif
#ifndef QT_NO_VECTOR4D
constexpr inline QVector4D QVector3D::toVector4D() const noexcept
{
    return QVector4D(v[0], v[1], v[2], 0.0f);
}
#endif

constexpr inline QPoint QVector3D::toPoint() const noexcept
{
    return QPoint(qRound(v[0]), qRound(v[1]));
}

constexpr inline QPointF QVector3D::toPointF() const noexcept
{
    return QPointF(qreal(v[0]), qreal(v[1]));
}

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug dbg, QVector3D vector);
#endif

#ifndef QT_NO_DATASTREAM
Q_GUI_EXPORT QDataStream &operator<<(QDataStream &, QVector3D );
Q_GUI_EXPORT QDataStream &operator>>(QDataStream &, QVector3D &);
#endif

#endif // QT_NO_VECTOR3D



/***************************** QVector4D *****************************/

#ifndef QT_NO_VECTOR4D

constexpr inline QVector4D::QVector4D() noexcept : v{0.0f, 0.0f, 0.0f, 0.0f} {}

constexpr inline QVector4D::QVector4D(float xpos, float ypos, float zpos, float wpos) noexcept : v{xpos, ypos, zpos, wpos} {}

constexpr inline QVector4D::QVector4D(QPoint point) noexcept : v{float(point.x()), float(point.y()), 0.0f, 0.0f} {}

constexpr inline QVector4D::QVector4D(QPointF point) noexcept : v{float(point.x()), float(point.y()), 0.0f, 0.0f} {}

#ifndef QT_NO_VECTOR2D
constexpr QVector4D::QVector4D(QVector2D vector) noexcept : v{vector[0], vector[1], 0.0f, 0.0f} {}
constexpr QVector4D::QVector4D(QVector2D vector, float zpos, float wpos) noexcept : v{vector[0], vector[1], zpos, wpos} {}
#endif
#ifndef QT_NO_VECTOR3D
constexpr QVector4D::QVector4D(QVector3D vector) noexcept : v{vector[0], vector[1], vector[2], 0.0f} {}
constexpr QVector4D::QVector4D(QVector3D vector, float wpos) noexcept : v{vector[0], vector[1], vector[2], wpos} {}
#endif

constexpr inline bool QVector4D::isNull() const noexcept
{
    return qIsNull(v[0]) && qIsNull(v[1]) && qIsNull(v[2]) && qIsNull(v[3]);
}

constexpr inline float QVector4D::x() const noexcept { return v[0]; }
constexpr inline float QVector4D::y() const noexcept { return v[1]; }
constexpr inline float QVector4D::z() const noexcept { return v[2]; }
constexpr inline float QVector4D::w() const noexcept { return v[3]; }

constexpr inline void QVector4D::setX(float aX) noexcept { v[0] = aX; }
constexpr inline void QVector4D::setY(float aY) noexcept { v[1] = aY; }
constexpr inline void QVector4D::setZ(float aZ) noexcept { v[2] = aZ; }
constexpr inline void QVector4D::setW(float aW) noexcept { v[3] = aW; }

constexpr inline float &QVector4D::operator[](int i)
{
    Q_ASSERT(uint(i) < 4u);
    return v[i];
}

constexpr inline float QVector4D::operator[](int i) const
{
    Q_ASSERT(uint(i) < 4u);
    return v[i];
}

inline float QVector4D::length() const noexcept
{
    return qHypot(v[0], v[1], v[2], v[3]);
}

constexpr inline float QVector4D::lengthSquared() const noexcept
{
    return v[0] * v[0] + v[1] * v[1] + v[2] * v[2] + v[3] * v[3];
}

inline QVector4D QVector4D::normalized() const noexcept
{
    const float len = length();
    return qFuzzyIsNull(len - 1.0f) ? *this : qFuzzyIsNull(len) ? QVector4D()
        : QVector4D(v[0] / len, v[1] / len, v[2] / len, v[3] / len);
}

inline void QVector4D::normalize() noexcept
{
    const float len = length();
    if (qFuzzyIsNull(len - 1.0f) || qFuzzyIsNull(len))
        return;

    v[0] /= len;
    v[1] /= len;
    v[2] /= len;
    v[3] /= len;
}

constexpr inline QVector4D &QVector4D::operator+=(QVector4D vector) noexcept
{
    v[0] += vector.v[0];
    v[1] += vector.v[1];
    v[2] += vector.v[2];
    v[3] += vector.v[3];
    return *this;
}

constexpr inline QVector4D &QVector4D::operator-=(QVector4D vector) noexcept
{
    v[0] -= vector.v[0];
    v[1] -= vector.v[1];
    v[2] -= vector.v[2];
    v[3] -= vector.v[3];
    return *this;
}

constexpr inline QVector4D &QVector4D::operator*=(float factor) noexcept
{
    v[0] *= factor;
    v[1] *= factor;
    v[2] *= factor;
    v[3] *= factor;
    return *this;
}

constexpr inline QVector4D &QVector4D::operator*=(QVector4D vector) noexcept
{
    v[0] *= vector.v[0];
    v[1] *= vector.v[1];
    v[2] *= vector.v[2];
    v[3] *= vector.v[3];
    return *this;
}

constexpr inline QVector4D &QVector4D::operator/=(float divisor)
{
    Q_ASSERT(divisor < 0 || divisor > 0);
    v[0] /= divisor;
    v[1] /= divisor;
    v[2] /= divisor;
    v[3] /= divisor;
    return *this;
}

constexpr inline QVector4D &QVector4D::operator/=(QVector4D vector)
{
    Q_ASSERT(vector.v[0] > 0 || vector.v[0] < 0);
    Q_ASSERT(vector.v[1] > 0 || vector.v[1] < 0);
    Q_ASSERT(vector.v[2] > 0 || vector.v[2] < 0);
    Q_ASSERT(vector.v[3] > 0 || vector.v[3] < 0);
    v[0] /= vector.v[0];
    v[1] /= vector.v[1];
    v[2] /= vector.v[2];
    v[3] /= vector.v[3];
    return *this;
}

constexpr float QVector4D::dotProduct(QVector4D v1, QVector4D v2) noexcept
{
    return v1.v[0] * v2.v[0] + v1.v[1] * v2.v[1] + v1.v[2] * v2.v[2] + v1.v[3] * v2.v[3];
}

#ifndef QT_NO_VECTOR2D

constexpr inline QVector2D QVector4D::toVector2D() const noexcept
{
    return QVector2D(v[0], v[1]);
}

constexpr inline QVector2D QVector4D::toVector2DAffine() const noexcept
{
    if (qIsNull(v[3]))
        return QVector2D();
    return QVector2D(v[0] / v[3], v[1] / v[3]);
}

#endif // QT_NO_VECTOR2D

#ifndef QT_NO_VECTOR3D

constexpr inline QVector3D QVector4D::toVector3D() const noexcept
{
    return QVector3D(v[0], v[1], v[2]);
}

constexpr QVector3D QVector4D::toVector3DAffine() const noexcept
{
    if (qIsNull(v[3]))
        return QVector3D();
    return QVector3D(v[0] / v[3], v[1] / v[3], v[2] / v[3]);
}

#endif // QT_NO_VECTOR3D

constexpr inline QPoint QVector4D::toPoint() const noexcept
{
    return QPoint(qRound(v[0]), qRound(v[1]));
}

constexpr inline QPointF QVector4D::toPointF() const noexcept
{
    return QPointF(qreal(v[0]), qreal(v[1]));
}

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug dbg, QVector4D vector);
#endif

#ifndef QT_NO_DATASTREAM
Q_GUI_EXPORT QDataStream &operator<<(QDataStream &, QVector4D );
Q_GUI_EXPORT QDataStream &operator>>(QDataStream &, QVector4D &);
#endif

#endif // QT_NO_VECTOR4D


QT_END_NAMESPACE

/***************************** Tuple protocol *****************************/

namespace std {
#ifndef QT_NO_VECTOR2D
    template <>
    class tuple_size<QT_PREPEND_NAMESPACE(QVector2D)> : public integral_constant<size_t, 2> {};
    template <>
    class tuple_element<0, QT_PREPEND_NAMESPACE(QVector2D)> { public: using type = float; };
    template <>
    class tuple_element<1, QT_PREPEND_NAMESPACE(QVector2D)> { public: using type = float; };
#endif // QT_NO_VECTOR2D

#ifndef QT_NO_VECTOR3D
    template <>
    class tuple_size<QT_PREPEND_NAMESPACE(QVector3D)> : public integral_constant<size_t, 3> {};
    template <>
    class tuple_element<0, QT_PREPEND_NAMESPACE(QVector3D)> { public: using type = float; };
    template <>
    class tuple_element<1, QT_PREPEND_NAMESPACE(QVector3D)> { public: using type = float; };
    template <>
    class tuple_element<2, QT_PREPEND_NAMESPACE(QVector3D)> { public: using type = float; };
#endif // QT_NO_VECTOR3D

#ifndef QT_NO_VECTOR4D
    template <>
    class tuple_size<QT_PREPEND_NAMESPACE(QVector4D)> : public integral_constant<size_t, 4> {};
    template <>
    class tuple_element<0, QT_PREPEND_NAMESPACE(QVector4D)> { public: using type = float; };
    template <>
    class tuple_element<1, QT_PREPEND_NAMESPACE(QVector4D)> { public: using type = float; };
    template <>
    class tuple_element<2, QT_PREPEND_NAMESPACE(QVector4D)> { public: using type = float; };
    template <>
    class tuple_element<3, QT_PREPEND_NAMESPACE(QVector4D)> { public: using type = float; };
#endif // QT_NO_VECTOR4D
}

#endif // QVECTORND_H
