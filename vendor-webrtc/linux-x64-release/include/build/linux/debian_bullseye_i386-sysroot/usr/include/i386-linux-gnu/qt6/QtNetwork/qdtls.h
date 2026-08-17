// Copyright (C) 2018 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QDTLS_H
#define QDTLS_H

#include <QtNetwork/qtnetworkglobal.h>

#include <QtNetwork/qsslsocket.h>
#include <QtNetwork/qssl.h>

#include <QtCore/qcryptographichash.h>
#include <QtCore/qobject.h>
#include <QtCore/qcontainerfwd.h>

Q_MOC_INCLUDE(<QtNetwork/QSslPreSharedKeyAuthenticator>)

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

    QList<QSslError> peerVerificationErrors() const;
    void ignoreVerificationErrors(const QList<QSslError> &errorsToIgnore);

Q_SIGNALS:

    void pskRequired(QSslPreSharedKeyAuthenticator *authenticator);
    void handshakeTimeout();

private:

    bool startHandshake(QUdpSocket *socket, const QByteArray &dgram);
    bool continueHandshake(QUdpSocket *socket, const QByteArray &dgram);

    Q_DECLARE_PRIVATE(QDtls)
    Q_DISABLE_COPY_MOVE(QDtls)
};

QT_END_NAMESPACE

#endif // QDTLS_H
