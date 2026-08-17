// Copyright (C) 2014 Governikus GmbH & Co. KG.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSSLELLIPTICCURVE_H
#define QSSLELLIPTICCURVE_H

#include <QtNetwork/qtnetworkglobal.h>
#include <QtCore/QString>
#include <QtCore/QMetaType>
#include <QtCore/qhashfunctions.h>

QT_BEGIN_NAMESPACE

class QSslEllipticCurve;
// qHash is a friend, but we can't use default arguments for friends (§8.3.6.4)
constexpr size_t qHash(QSslEllipticCurve curve, size_t seed = 0) noexcept;

class QSslEllipticCurve {
public:
    constexpr QSslEllipticCurve() noexcept
        : id(0)
    {
    }

    Q_NETWORK_EXPORT static QSslEllipticCurve fromShortName(const QString &name);
    Q_NETWORK_EXPORT static QSslEllipticCurve fromLongName(const QString &name);

    [[nodiscard]] Q_NETWORK_EXPORT QString shortName() const;
    [[nodiscard]] Q_NETWORK_EXPORT QString longName() const;

    constexpr bool isValid() const noexcept
    {
        return id != 0;
    }

    Q_NETWORK_EXPORT bool isTlsNamedCurve() const noexcept;

private:
    int id;

    friend constexpr bool operator==(QSslEllipticCurve lhs, QSslEllipticCurve rhs) noexcept
    { return lhs.id == rhs.id; }
    friend constexpr bool operator!=(QSslEllipticCurve lhs, QSslEllipticCurve rhs) noexcept
    { return !(lhs == rhs); }
    friend constexpr size_t qHash(QSslEllipticCurve curve, size_t seed) noexcept;

    friend class QSslContext;
    friend class QSslSocketPrivate;
};

Q_DECLARE_TYPEINFO(QSslEllipticCurve, Q_PRIMITIVE_TYPE);

constexpr inline size_t qHash(QSslEllipticCurve curve, size_t seed) noexcept
{ return qHash(curve.id, seed); }

#ifndef QT_NO_DEBUG_STREAM
class QDebug;
Q_NETWORK_EXPORT QDebug operator<<(QDebug debug, QSslEllipticCurve curve);
#endif

QT_END_NAMESPACE

QT_DECL_METATYPE_EXTERN(QSslEllipticCurve, Q_NETWORK_EXPORT)

#endif // QSSLELLIPTICCURVE_H
