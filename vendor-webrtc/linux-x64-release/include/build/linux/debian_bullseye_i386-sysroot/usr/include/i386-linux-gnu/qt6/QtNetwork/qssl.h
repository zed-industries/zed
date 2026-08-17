// Copyright (C) 2021 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only


#ifndef QSSL_H
#define QSSL_H

#include <QtNetwork/qtnetworkglobal.h>
#include <QtCore/QFlags>

QT_BEGIN_NAMESPACE


namespace QSsl {
    enum KeyType {
        PrivateKey,
        PublicKey
    };

    enum EncodingFormat {
        Pem,
        Der
    };

    enum KeyAlgorithm {
        Opaque,
        Rsa,
        Dsa,
        Ec,
        Dh,
    };

    enum AlternativeNameEntryType {
        EmailEntry,
        DnsEntry,
        IpAddressEntry
    };

    enum SslProtocol {
        TlsV1_0 QT_DEPRECATED_VERSION_X_6_3("Use TlsV1_2OrLater instead."),
        TlsV1_1 QT_DEPRECATED_VERSION_X_6_3("Use TlsV1_2OrLater instead."),
        TlsV1_2,
        AnyProtocol,
        SecureProtocols,

        TlsV1_0OrLater QT_DEPRECATED_VERSION_X_6_3("Use TlsV1_2OrLater instead."),
        TlsV1_1OrLater QT_DEPRECATED_VERSION_X_6_3("Use TlsV1_2OrLater instead."),
        TlsV1_2OrLater,

        DtlsV1_0 QT_DEPRECATED_VERSION_X_6_3("Use DtlsV1_2OrLater instead."),
        DtlsV1_0OrLater QT_DEPRECATED_VERSION_X_6_3("Use DtlsV1_2OrLater instead."),
        DtlsV1_2,
        DtlsV1_2OrLater,

        TlsV1_3,
        TlsV1_3OrLater,

        UnknownProtocol = -1
    };

    enum SslOption {
        SslOptionDisableEmptyFragments = 0x01,
        SslOptionDisableSessionTickets = 0x02,
        SslOptionDisableCompression = 0x04,
        SslOptionDisableServerNameIndication = 0x08,
        SslOptionDisableLegacyRenegotiation = 0x10,
        SslOptionDisableSessionSharing = 0x20,
        SslOptionDisableSessionPersistence = 0x40,
        SslOptionDisableServerCipherPreference = 0x80
    };
    Q_DECLARE_FLAGS(SslOptions, SslOption)

    enum class AlertLevel {
        Warning,
        Fatal,
        Unknown
    };

    enum class AlertType {
        CloseNotify,
        UnexpectedMessage = 10,
        BadRecordMac = 20,
        RecordOverflow = 22,
        DecompressionFailure = 30, // reserved
        HandshakeFailure = 40,
        NoCertificate = 41, // reserved
        BadCertificate = 42,
        UnsupportedCertificate = 43,
        CertificateRevoked = 44,
        CertificateExpired = 45,
        CertificateUnknown = 46,
        IllegalParameter = 47,
        UnknownCa = 48,
        AccessDenied = 49,
        DecodeError = 50,
        DecryptError = 51,
        ExportRestriction = 60, // reserved
        ProtocolVersion = 70,
        InsufficientSecurity = 71,
        InternalError = 80,
        InappropriateFallback = 86,
        UserCancelled = 90,
        NoRenegotiation = 100,
        MissingExtension = 109,
        UnsupportedExtension = 110,
        CertificateUnobtainable = 111, // reserved
        UnrecognizedName = 112,
        BadCertificateStatusResponse = 113,
        BadCertificateHashValue = 114, // reserved
        UnknownPskIdentity = 115,
        CertificateRequired = 116,
        NoApplicationProtocol = 120,
        UnknownAlertMessage = 255
    };

    enum class ImplementedClass
    {
        Key,
        Certificate,
        Socket,
        DiffieHellman,
        EllipticCurve,
        Dtls,
        DtlsCookie
    };

    enum class SupportedFeature
    {
        CertificateVerification,
        ClientSideAlpn,
        ServerSideAlpn,
        Ocsp,
        Psk,
        SessionTicket,
        Alerts
    };
}

Q_DECLARE_OPERATORS_FOR_FLAGS(QSsl::SslOptions)

QT_END_NAMESPACE

#endif // QSSL_H
