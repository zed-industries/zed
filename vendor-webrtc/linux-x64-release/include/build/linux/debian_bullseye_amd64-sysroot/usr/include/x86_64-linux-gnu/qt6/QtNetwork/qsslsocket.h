// Copyright (C) 2021 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only


#ifndef QSSLSOCKET_H
#define QSSLSOCKET_H

#include <QtNetwork/qtnetworkglobal.h>
#include <QtCore/qlist.h>
#ifndef QT_NO_SSL
#   include <QtNetwork/qtcpsocket.h>
#   include <QtNetwork/qsslerror.h>
#endif

QT_BEGIN_NAMESPACE


#ifndef QT_NO_SSL

class QDir;
class QSslCipher;
class QSslCertificate;
class QSslConfiguration;
class QSslPreSharedKeyAuthenticator;
class QOcspResponse;

class QSslSocketPrivate;
class Q_NETWORK_EXPORT QSslSocket : public QTcpSocket
{
    Q_OBJECT
    Q_MOC_INCLUDE(<QtNetwork/qsslpresharedkeyauthenticator.h>)
public:
    enum SslMode {
        UnencryptedMode,
        SslClientMode,
        SslServerMode
    };

    enum PeerVerifyMode {
        VerifyNone,
        QueryPeer,
        VerifyPeer,
        AutoVerifyPeer
    };

    explicit QSslSocket(QObject *parent = nullptr);
    ~QSslSocket();
    void resume() override; // to continue after proxy authentication required, SSL errors etc.

    // Autostarting the SSL client handshake.
    void connectToHostEncrypted(const QString &hostName, quint16 port, OpenMode mode = ReadWrite, NetworkLayerProtocol protocol = AnyIPProtocol);
    void connectToHostEncrypted(const QString &hostName, quint16 port, const QString &sslPeerName, OpenMode mode = ReadWrite, NetworkLayerProtocol protocol = AnyIPProtocol);
    bool setSocketDescriptor(qintptr socketDescriptor, SocketState state = ConnectedState,
                             OpenMode openMode = ReadWrite) override;

    using QAbstractSocket::connectToHost;
    void connectToHost(const QString &hostName, quint16 port, OpenMode openMode = ReadWrite, NetworkLayerProtocol protocol = AnyIPProtocol) override;
    void disconnectFromHost() override;

    virtual void setSocketOption(QAbstractSocket::SocketOption option, const QVariant &value) override;
    virtual QVariant socketOption(QAbstractSocket::SocketOption option) override;

    SslMode mode() const;
    bool isEncrypted() const;

    QSsl::SslProtocol protocol() const;
    void setProtocol(QSsl::SslProtocol protocol);

    QSslSocket::PeerVerifyMode peerVerifyMode() const;
    void setPeerVerifyMode(QSslSocket::PeerVerifyMode mode);

    int peerVerifyDepth() const;
    void setPeerVerifyDepth(int depth);

    QString peerVerifyName() const;
    void setPeerVerifyName(const QString &hostName);

    // From QIODevice
    qint64 bytesAvailable() const override;
    qint64 bytesToWrite() const override;
    bool canReadLine() const override;
    void close() override;
    bool atEnd() const override;

    // From QAbstractSocket:
    void setReadBufferSize(qint64 size) override;

    // Similar to QIODevice's:
    qint64 encryptedBytesAvailable() const;
    qint64 encryptedBytesToWrite() const;

    // SSL configuration
    QSslConfiguration sslConfiguration() const;
    void setSslConfiguration(const QSslConfiguration &config);

    // Certificate & cipher accessors.
    void setLocalCertificateChain(const QList<QSslCertificate> &localChain);
    QList<QSslCertificate> localCertificateChain() const;

    void setLocalCertificate(const QSslCertificate &certificate);
    void setLocalCertificate(const QString &fileName, QSsl::EncodingFormat format = QSsl::Pem);
    QSslCertificate localCertificate() const;
    QSslCertificate peerCertificate() const;
    QList<QSslCertificate> peerCertificateChain() const;
    QSslCipher sessionCipher() const;
    QSsl::SslProtocol sessionProtocol() const;
    QList<QOcspResponse> ocspResponses() const;

    // Private keys, for server sockets.
    void setPrivateKey(const QSslKey &key);
    void setPrivateKey(const QString &fileName, QSsl::KeyAlgorithm algorithm = QSsl::Rsa,
                       QSsl::EncodingFormat format = QSsl::Pem,
                       const QByteArray &passPhrase = QByteArray());
    QSslKey privateKey() const;

    bool waitForConnected(int msecs = 30000) override;
    bool waitForEncrypted(int msecs = 30000);
    bool waitForReadyRead(int msecs = 30000) override;
    bool waitForBytesWritten(int msecs = 30000) override;
    bool waitForDisconnected(int msecs = 30000) override;

    QList<QSslError> sslHandshakeErrors() const;

    static bool supportsSsl();
    static long sslLibraryVersionNumber();
    static QString sslLibraryVersionString();
    static long sslLibraryBuildVersionNumber();
    static QString sslLibraryBuildVersionString();

    static QList<QString> availableBackends();
    static QString activeBackend();
    static bool setActiveBackend(const QString &backendName);
    static QList<QSsl::SslProtocol> supportedProtocols(const QString &backendName = {});
    static bool isProtocolSupported(QSsl::SslProtocol protocol, const QString &backendName = {});
    static QList<QSsl::ImplementedClass> implementedClasses(const QString &backendName = {});
    static bool isClassImplemented(QSsl::ImplementedClass cl, const QString &backendName = {});
    static QList<QSsl::SupportedFeature> supportedFeatures(const QString &backendName = {});
    static bool isFeatureSupported(QSsl::SupportedFeature feat, const QString &backendName = {});

    void ignoreSslErrors(const QList<QSslError> &errors);
    void continueInterruptedHandshake();

public Q_SLOTS:
    void startClientEncryption();
    void startServerEncryption();
    void ignoreSslErrors();

Q_SIGNALS:
    void encrypted();
    void peerVerifyError(const QSslError &error);
    void sslErrors(const QList<QSslError> &errors);
    void modeChanged(QSslSocket::SslMode newMode);
    void encryptedBytesWritten(qint64 totalBytes);
    void preSharedKeyAuthenticationRequired(QSslPreSharedKeyAuthenticator *authenticator);
    void newSessionTicketReceived();
    void alertSent(QSsl::AlertLevel level, QSsl::AlertType type, const QString &description);
    void alertReceived(QSsl::AlertLevel level, QSsl::AlertType type, const QString &description);
    void handshakeInterruptedOnError(const QSslError &error);

protected:
    qint64 readData(char *data, qint64 maxlen) override;
    qint64 skipData(qint64 maxSize) override;
    qint64 writeData(const char *data, qint64 len) override;

private:
    Q_DECLARE_PRIVATE(QSslSocket)
    Q_DISABLE_COPY_MOVE(QSslSocket)

    Q_PRIVATE_SLOT(d_func(), void _q_connectedSlot())
    Q_PRIVATE_SLOT(d_func(), void _q_hostFoundSlot())
    Q_PRIVATE_SLOT(d_func(), void _q_disconnectedSlot())
    Q_PRIVATE_SLOT(d_func(), void _q_stateChangedSlot(QAbstractSocket::SocketState))
    Q_PRIVATE_SLOT(d_func(), void _q_errorSlot(QAbstractSocket::SocketError))
    Q_PRIVATE_SLOT(d_func(), void _q_readyReadSlot())
    Q_PRIVATE_SLOT(d_func(), void _q_channelReadyReadSlot(int))
    Q_PRIVATE_SLOT(d_func(), void _q_bytesWrittenSlot(qint64))
    Q_PRIVATE_SLOT(d_func(), void _q_channelBytesWrittenSlot(int, qint64))
    Q_PRIVATE_SLOT(d_func(), void _q_readChannelFinishedSlot())
    Q_PRIVATE_SLOT(d_func(), void _q_flushWriteBuffer())
    Q_PRIVATE_SLOT(d_func(), void _q_flushReadBuffer())
    Q_PRIVATE_SLOT(d_func(), void _q_resumeImplementation())
};

#endif // QT_NO_SSL

QT_END_NAMESPACE

#endif
