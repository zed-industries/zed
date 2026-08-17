/****************************************************************************
**
** Copyright (C) 2018 The Qt Company Ltd.
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

#ifndef QDTLS_H
#define QDTLS_H

#include <QtNetwork/qtnetworkglobal.h>

#include <QtNetwork/qsslsocket.h>
#include <QtNetwork/qssl.h>

#include <QtCore/qcryptographichash.h>
#include <QtCore/qobject.h>

#ifndef Q_CLANG_QDOC
QT_REQUIRE_CONFIG(dtls);
#endif

QT_BEGIN_NAMESPACE

enum class QDtlsError : unsigned char
{
    NoError,
    InvalidInputParameters,
    InvalidOperation,
    UnderlyingSocketError,
    RemoteClosedConnectionError,
    PeerVerificationError,
    TlsInitializationError,
    TlsFatalError,
    TlsNonFatalError
};

class QHostAddress;
class QUdpSocket;
class QByteArray;
class QString;

class QDtlsClientVerifierPrivate;
class Q_NETWORK_EXPORT QDtlsClientVerifier : public QObject
{
    Q_OBJECT

public:

    explicit QDtlsClientVerifier(QObject *parent = nullptr);
    ~QDtlsClientVerifier();

    struct Q_NETWORK_EXPORT GeneratorParameters
    {
        GeneratorParameters();
        GeneratorParameters(QCryptographicHash::Algorithm a, const QByteArray &s);
        QCryptographicHash::Algorithm hash = QCryptographicHash::Sha1;
        QByteArray secret;
    };

    bool setCookieGeneratorParameters(const GeneratorParameters &params);
    GeneratorParameters cookieGeneratorParameters() const;

    bool verifyClient(QUdpSocket *socket, const QByteArray &dgram,
                      const QHostAddress &address, quint16 port);
    QByteArray verifiedHello() const;

    QDtlsError dtlsError() const;
    QString dtlsErrorString() const;

private:

    Q_DECLARE_PRIVATE(QDtlsClientVerifier)
    Q_DISABLE_COPY(QDtlsClientVerifier)
};

class QSslPreSharedKeyAuthenticator;
template<class> class QVector;
class QSslConfiguration;
class QSslCipher;
class QSslError;

class QDtlsPrivate;
class Q_NETWORK_EXPORT QDtls : public QObject
{
    Q_OBJECT

public:

    enum HandshakeState
    {
        HandshakeNotStarted,
        HandshakeInProgress,
        PeerVerificationFailed,
        HandshakeComplete
    };

    explicit QDtls(QSslSocket::SslMode mode, QObject *parent = nullptr);
    ~QDtls();

    bool setPeer(const QHostAddress &address, quint16 port,
                 const QString &verificationName = {});
    bool setPeerVerificationName(const QString &name);
    QHostAddress peerAddress() const;
    quint16 peerPort() const;
    QString peerVerificationName() const;
    QSslSocket::SslMode sslMode() const;

    void setMtuHint(quint16 mtuHint);
    quint16 mtuHint() const;

    using GeneratorParameters = QDtlsClientVerifier::GeneratorParameters;
    bool setCookieGeneratorParameters(const GeneratorParameters &params);
    GeneratorParameters cookieGeneratorParameters() const;

    bool setDtlsConfiguration(const QSslConfiguration &configuration);
    QSslConfiguration dtlsConfiguration() const;

    HandshakeState handshakeState() const;

    bool doHandshake(QUdpSocket *socket, const QByteArray &dgram = {});
    bool handleTimeout(QUdpSocket *socket);
    bool resumeHandshake(QUdpSocket *socket);
    bool abortHandshake(QUdpSocket *socket);
    bool shutdown(QUdpSocket *socket);

    bool isConnectionEncrypted() const;
    QSslCipher sessionCipher() const;
    QSsl::SslProtocol sessionProtocol() const;

    qint64 writeDatagramEncrypted(QUdpSocket *socket, const QByteArray &dgram);
    QByteArray decryptDatagram(QUdpSocket *socket, const QByteArray &dgram);

    QDtlsError dtlsError() const;
    QString dtlsErrorString() const;

    QVector<QSslError> peerVerificationErrors() const;
    void ignoreVerificationErrors(const QVector<QSslError> &errorsToIgnore);

Q_SIGNALS:

    void pskRequired(QSslPreSharedKeyAuthenticator *authenticator);
    void handshakeTimeout();

private:

    bool startHandshake(QUdpSocket *socket, const QByteArray &dgram);
    bool continueHandshake(QUdpSocket *socket, const QByteArray &dgram);

    Q_DECLARE_PRIVATE(QDtls)
    Q_DISABLE_COPY(QDtls)
};

QT_END_NAMESPACE

#endif // QDTLS_H
