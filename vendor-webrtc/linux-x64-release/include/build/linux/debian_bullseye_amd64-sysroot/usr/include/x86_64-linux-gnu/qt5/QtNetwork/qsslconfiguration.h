/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Copyright (C) 2014 BlackBerry Limited. All rights reserved.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtNetwork module of the Qt Toolkit.
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

template<typename T> class QList;
class QSslCertificate;
class QSslCipher;
class QSslKey;
class QSslEllipticCurve;
class QSslDiffieHellmanParameters;

namespace dtlsopenssl
{
class DtlsState;
}

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
    { qSwap(d, other.d); }

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
    QVector<QSslEllipticCurve> ellipticCurves() const;
    void setEllipticCurves(const QVector<QSslEllipticCurve> &curves);
    static QVector<QSslEllipticCurve> supportedEllipticCurves();

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

    void setOcspStaplingEnabled(bool enable);
    bool ocspStaplingEnabled() const;

    enum NextProtocolNegotiationStatus {
        NextProtocolNegotiationNone,
        NextProtocolNegotiationNegotiated,
        NextProtocolNegotiationUnsupported
    };

#if QT_VERSION >= QT_VERSION_CHECK(6,0,0)
    void setAllowedNextProtocols(const QList<QByteArray> &protocols);
#else
    void setAllowedNextProtocols(QList<QByteArray> protocols);
#endif
    QList<QByteArray> allowedNextProtocols() const;

    QByteArray nextNegotiatedProtocol() const;
    NextProtocolNegotiationStatus nextProtocolNegotiationStatus() const;

    static const char ALPNProtocolHTTP2[];
    static const char NextProtocolSpdy3_0[];
    static const char NextProtocolHttp1_1[];

private:
    friend class QSslSocket;
    friend class QSslConfigurationPrivate;
    friend class QSslSocketBackendPrivate;
    friend class QSslContext;
    friend class QDtlsBasePrivate;
    friend class dtlsopenssl::DtlsState;
    QSslConfiguration(QSslConfigurationPrivate *dd);
    QSharedDataPointer<QSslConfigurationPrivate> d;
};

Q_DECLARE_SHARED(QSslConfiguration)

QT_END_NAMESPACE

Q_DECLARE_METATYPE(QSslConfiguration)

#endif  // QT_NO_SSL

#endif
