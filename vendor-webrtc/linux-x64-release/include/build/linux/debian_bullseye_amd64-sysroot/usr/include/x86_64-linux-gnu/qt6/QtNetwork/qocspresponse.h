// Copyright (C) 2011 Richard J. Moore <rich@kde.org>
// Copyright (C) 2019 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QOCSPRESPONSE_H
#define QOCSPRESPONSE_H

#include <QtNetwork/qtnetworkglobal.h>

#include <QtCore/qshareddata.h>
#include <QtCore/qmetatype.h>
#include <QtCore/qobject.h>

#ifndef Q_CLANG_QDOC
QT_REQUIRE_CONFIG(ssl);
#endif

QT_BEGIN_NAMESPACE

enum class QOcspCertificateStatus
{
    Good,
    Revoked,
    Unknown
};

enum class QOcspRevocationReason
{
    None = -1,
    Unspecified,
    KeyCompromise,
    CACompromise,
    AffiliationChanged,
    Superseded,
    CessationOfOperation,
    CertificateHold,
    RemoveFromCRL
};

namespace QTlsPrivate {
class TlsCryptographOpenSSL;
}

class QOcspResponse;
Q_NETWORK_EXPORT size_t qHash(const QOcspResponse &response, size_t seed = 0) noexcept;

class QOcspResponsePrivate;
class Q_NETWORK_EXPORT QOcspResponse
{
public:

    QOcspResponse();
    QOcspResponse(const QOcspResponse &other);
    QOcspResponse(QOcspResponse && other)  noexcept;
    ~QOcspResponse();

    QOcspResponse &operator = (const QOcspResponse &other);
    QOcspResponse &operator = (QOcspResponse &&other) noexcept;

    QOcspCertificateStatus certificateStatus() const;
    QOcspRevocationReason revocationReason() const;

    class QSslCertificate responder() const;
    QSslCertificate subject() const;

    void swap(QOcspResponse &other) noexcept { d.swap(other.d); }

private:
    bool isEqual(const QOcspResponse &other) const;

    friend class QTlsPrivate::TlsCryptographOpenSSL;
    friend bool operator==(const QOcspResponse &lhs, const QOcspResponse &rhs)
    { return lhs.isEqual(rhs); }
    friend bool operator!=(const QOcspResponse &lhs, const QOcspResponse &rhs)
    { return !lhs.isEqual(rhs); }

    friend Q_NETWORK_EXPORT size_t qHash(const QOcspResponse &response, size_t seed) noexcept;

    QSharedDataPointer<QOcspResponsePrivate> d;
};

Q_DECLARE_SHARED(QOcspResponse)

QT_END_NAMESPACE

QT_DECL_METATYPE_EXTERN(QOcspResponse, Q_NETWORK_EXPORT)

#endif // QOCSPRESPONSE_H
