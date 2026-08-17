// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only


#ifndef QSSLERROR_H
#define QSSLERROR_H

#include <QtNetwork/qtnetworkglobal.h>
#include <QtCore/qvariant.h>
#include <QtNetwork/qsslcertificate.h>

#include <memory>

QT_BEGIN_NAMESPACE


#ifndef QT_NO_SSL

class QSslErrorPrivate;
class Q_NETWORK_EXPORT QSslError
{
    Q_GADGET
public:
    enum SslError {
        NoError,
        UnableToGetIssuerCertificate,
        UnableToDecryptCertificateSignature,
        UnableToDecodeIssuerPublicKey,
        CertificateSignatureFailed,
        CertificateNotYetValid,
        CertificateExpired,
        InvalidNotBeforeField,
        InvalidNotAfterField,
        SelfSignedCertificate,
        SelfSignedCertificateInChain,
        UnableToGetLocalIssuerCertificate,
        UnableToVerifyFirstCertificate,
        CertificateRevoked,
        InvalidCaCertificate,
        PathLengthExceeded,
        InvalidPurpose,
        CertificateUntrusted,
        CertificateRejected,
        SubjectIssuerMismatch, // hostname mismatch?
        AuthorityIssuerSerialNumberMismatch,
        NoPeerCertificate,
        HostNameMismatch,
        NoSslSupport,
        CertificateBlacklisted,
        CertificateStatusUnknown,
        OcspNoResponseFound,
        OcspMalformedRequest,
        OcspMalformedResponse,
        OcspInternalError,
        OcspTryLater,
        OcspSigRequred,
        OcspUnauthorized,
        OcspResponseCannotBeTrusted,
        OcspResponseCertIdUnknown,
        OcspResponseExpired,
        OcspStatusUnknown,
        UnspecifiedError = -1
    };
    Q_ENUM(SslError)

    // RVCT compiler in debug build does not like about default values in const-
    // So as an workaround we define all constructor overloads here explicitly
    QSslError();
    explicit QSslError(SslError error);
    QSslError(SslError error, const QSslCertificate &certificate);

    QSslError(const QSslError &other);

    void swap(QSslError &other) noexcept
    { d.swap(other.d); }

    ~QSslError();
    QSslError &operator=(QSslError &&other) noexcept { swap(other); return *this; }
    QSslError &operator=(const QSslError &other);
    bool operator==(const QSslError &other) const;
    inline bool operator!=(const QSslError &other) const
    { return !(*this == other); }

    SslError error() const;
    QString errorString() const;
    QSslCertificate certificate() const;

private:
    // ### Qt 7: make QSslError implicitly shared
    std::unique_ptr<QSslErrorPrivate> d;
};
Q_DECLARE_SHARED(QSslError)

Q_NETWORK_EXPORT size_t qHash(const QSslError &key, size_t seed = 0) noexcept;

#ifndef QT_NO_DEBUG_STREAM
class QDebug;
Q_NETWORK_EXPORT QDebug operator<<(QDebug debug, const QSslError &error);
Q_NETWORK_EXPORT QDebug operator<<(QDebug debug, const QSslError::SslError &error);
#endif
#else
class Q_NETWORK_EXPORT QSslError {}; // dummy class so that moc has a complete type
#endif // QT_NO_SSL

QT_END_NAMESPACE

#ifndef QT_NO_SSL
QT_DECL_METATYPE_EXTERN_TAGGED(QList<QSslError>, QList_QSslError, Q_NETWORK_EXPORT)
#endif

#endif
