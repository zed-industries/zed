// Copyright (C) 2016 The Qt Company Ltd.
// Copyright (C) 2014 BlackBerry Limited. All rights reserved.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

/****************************************************************************
**
** In addition, as a special exception, the copyright holders listed above give
** permission to link the code of its release of Qt with the OpenSSL project's
** "OpenSSL" library (or modified versions of the "OpenSSL" library that use the
** same license as the original version), and distribute the linked executables.
**
** You must comply with the GNU General Public License version 2 in all
** respects for all of the code used other than the "OpenSSL" code.  If you
** modify this file, you may extend this exception to your version of the file,
** but you are not obligated to do so.  If you do not wish to do so, delete
** this exception statement from your version of this file.
**
****************************************************************************/

#ifndef QSSLCONFIGURATION_H
#define QSSLCONFIGURATION_H

#include <QtNetwork/qtnetworkglobal.h>
#include <QtCore/qmap.h>
#include <QtCore/qshareddata.h>
#include <QtNetwork/qsslsocket.h>
#include <QtNetwork/qssl.h>

#ifndef QT_NO_SSL

QT_BEGIN_NAMESPACE

class QSslCertificate;
class QSslCipher;
class QSslKey;
class QSslEllipticCurve;
class QSslDiffieHellmanParameters;

class QSslConfigurationPrivate;
class Q_NETWORK_EXPORT QSslConfiguration
{
public:
    QSslConfiguration();
    QSslConfiguration(const QSslConfiguration &other);
    ~QSslConfiguration();
    QSslConfiguration &operator=(QSslConfiguration &&other) noexcept { swap(other); return *this; }
    QSslConfiguration &operator=(const QSslConfiguration &other);

    void swap(QSslConfiguration &other) noexcept
    { d.swap(other.d); }

    bool operator==(const QSslConfiguration &other) const;
    inline bool operator!=(const QSslConfiguration &other) const
    { return !(*this == other); }

    bool isNull() const;

    QSsl::SslProtocol protocol() const;
    void setProtocol(QSsl::SslProtocol protocol);

    // Verification
    QSslSocket::PeerVerifyMode peerVerifyMode() const;
    void setPeerVerifyMode(QSslSocket::PeerVerifyMode mode);

    int peerVerifyDepth() const;
    void setPeerVerifyDepth(int depth);

    // Certificate & cipher configuration
    QList<QSslCertificate> localCertificateChain() const;
    void setLocalCertificateChain(const QList<QSslCertificate> &localChain);

    QSslCertificate localCertificate() const;
    void setLocalCertificate(const QSslCertificate &certificate);

    QSslCertificate peerCertificate() const;
    QList<QSslCertificate> peerCertificateChain() const;
    QSslCipher sessionCipher() const;
    QSsl::SslProtocol sessionProtocol() const;

    // Private keys, for server sockets
    QSslKey privateKey() const;
    void setPrivateKey(const QSslKey &key);

    // Cipher settings
    QList<QSslCipher> ciphers() const;
    void setCiphers(const QList<QSslCipher> &ciphers);
    void setCiphers(const QString &ciphers);
    static QList<QSslCipher> supportedCiphers();

    // Certificate Authority (CA) settings
    QList<QSslCertificate> caCertificates() const;
    void setCaCertificates(const QList<QSslCertificate> &certificates);
    bool addCaCertificates(
            const QString &path, QSsl::EncodingFormat format = QSsl::Pem,
            QSslCertificate::PatternSyntax syntax = QSslCertificate::PatternSyntax::FixedString);
    void addCaCertificate(const QSslCertificate &certificate);
    void addCaCertificates(const QList<QSslCertificate> &certificates);

    static QList<QSslCertificate> systemCaCertificates();

    void setSslOption(QSsl::SslOption option, bool on);
    bool testSslOption(QSsl::SslOption option) const;

    QByteArray sessionTicket() const;
    void setSessionTicket(const QByteArray &sessionTicket);
    int sessionTicketLifeTimeHint() const;

    QSslKey ephemeralServerKey() const;

    // EC settings
    QList<QSslEllipticCurve> ellipticCurves() const;
    void setEllipticCurves(const QList<QSslEllipticCurve> &curves);
    static QList<QSslEllipticCurve> supportedEllipticCurves();

    QByteArray preSharedKeyIdentityHint() const;
    void setPreSharedKeyIdentityHint(const QByteArray &hint);

    QSslDiffieHellmanParameters diffieHellmanParameters() const;
    void setDiffieHellmanParameters(const QSslDiffieHellmanParameters &dhparams);

    QMap<QByteArray, QVariant> backendConfiguration() const;
    void setBackendConfigurationOption(const QByteArray &name, const QVariant &value);
    void setBackendConfiguration(const QMap<QByteArray, QVariant> &backendConfiguration = QMap<QByteArray, QVariant>());

    static QSslConfiguration defaultConfiguration();
    static void setDefaultConfiguration(const QSslConfiguration &configuration);

#if QT_CONFIG(dtls) || defined(Q_CLANG_QDOC)
    bool dtlsCookieVerificationEnabled() const;
    void setDtlsCookieVerificationEnabled(bool enable);

    static QSslConfiguration defaultDtlsConfiguration();
    static void setDefaultDtlsConfiguration(const QSslConfiguration &configuration);
#endif // dtls

    bool handshakeMustInterruptOnError() const;
    void setHandshakeMustInterruptOnError(bool interrupt);

    bool missingCertificateIsFatal() const;
    void setMissingCertificateIsFatal(bool cannotRecover);

    void setOcspStaplingEnabled(bool enable);
    bool ocspStaplingEnabled() const;

    enum NextProtocolNegotiationStatus {
        NextProtocolNegotiationNone,
        NextProtocolNegotiationNegotiated,
        NextProtocolNegotiationUnsupported
    };

    void setAllowedNextProtocols(const QList<QByteArray> &protocols);
    QList<QByteArray> allowedNextProtocols() const;

    QByteArray nextNegotiatedProtocol() const;
    NextProtocolNegotiationStatus nextProtocolNegotiationStatus() const;

    static const char ALPNProtocolHTTP2[];
    static const char NextProtocolHttp1_1[];

private:
    friend class QSslSocket;
    friend class QSslConfigurationPrivate;
    friend class QSslContext;
    friend class QTlsBackend;
    QSslConfiguration(QSslConfigurationPrivate *dd);
    QSharedDataPointer<QSslConfigurationPrivate> d;
};

Q_DECLARE_SHARED(QSslConfiguration)

QT_END_NAMESPACE

QT_DECL_METATYPE_EXTERN(QSslConfiguration, Q_NETWORK_EXPORT)

#endif  // QT_NO_SSL

#endif
