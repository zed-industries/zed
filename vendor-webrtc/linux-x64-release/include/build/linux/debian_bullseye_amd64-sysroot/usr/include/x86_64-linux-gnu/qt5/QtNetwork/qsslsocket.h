/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
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


#ifndef QSSLSOCKET_H
#define QSSLSOCKET_H

#include <QtNetwork/qtnetworkglobal.h>
#include <QtCore/qlist.h>
#include <QtCore/qregexp.h>
#include <QtCore/qvector.h>
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
class QSslEllipticCurve;
class QSslPreSharedKeyAuthenticator;
class QOcspResponse;

class QSslSocketPrivate;
class Q_NETWORK_EXPORT QSslSocket : public QTcpSocket
{
    Q_OBJECT
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
    bool flush(); // ### Qt6: remove me (implementation moved to private flush())
    void abort();

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
    QVector<QOcspResponse> ocspResponses() const;

    // Private keys, for server sockets.
    void setPrivateKey(const QSslKey &key);
    void setPrivateKey(const QString &fileName, QSsl::KeyAlgorithm algorithm = QSsl::Rsa,
                       QSsl::EncodingFormat format = QSsl::Pem,
                       const QByteArray &passPhrase = QByteArray());
    QSslKey privateKey() const;

    // Cipher settings.
#if QT_DEPRECATED_SINCE(5, 5)
    QT_DEPRECATED_X("Use QSslConfiguration::ciphers()") QList<QSslCipher> ciphers() const;
    QT_DEPRECATED_X("Use QSslConfiguration::setCiphers()") void setCiphers(const QList<QSslCipher> &ciphers);
    QT_DEPRECATED void setCiphers(const QString &ciphers);
    QT_DEPRECATED static void setDefaultCiphers(const QList<QSslCipher> &ciphers);
    QT_DEPRECATED static QList<QSslCipher> defaultCiphers();
    QT_DEPRECATED_X("Use QSslConfiguration::supportedCiphers()") static QList<QSslCipher> supportedCiphers();
#endif // QT_DEPRECATED_SINCE(5, 5)

    // CA settings.
#if QT_DEPRECATED_SINCE(5, 15)
    QT_DEPRECATED_X("Use QSslConfiguration::addCaCertificates()") bool addCaCertificates(const QString &path, QSsl::EncodingFormat format = QSsl::Pem,
                           QRegExp::PatternSyntax syntax = QRegExp::FixedString);
    QT_DEPRECATED_X("Use QSslConfiguration::addCaCertificate()") void addCaCertificate(const QSslCertificate &certificate);
    QT_DEPRECATED_X("Use QSslConfiguration::addCaCertificates()") void addCaCertificates(const QList<QSslCertificate> &certificates);
#endif // QT_DEPRECATED_SINCE(5, 15)
#if QT_DEPRECATED_SINCE(5, 5)
    QT_DEPRECATED_X("Use QSslConfiguration::setCaCertificates()") void setCaCertificates(const QList<QSslCertificate> &certificates);
    QT_DEPRECATED_X("Use QSslConfiguration::caCertificates()") QList<QSslCertificate> caCertificates() const;
#endif // QT_DEPRECATED_SINCE(5, 5)
#if QT_DEPRECATED_SINCE(5, 15)
    QT_DEPRECATED static bool addDefaultCaCertificates(const QString &path, QSsl::EncodingFormat format = QSsl::Pem,
                                         QRegExp::PatternSyntax syntax = QRegExp::FixedString);
    QT_DEPRECATED static void addDefaultCaCertificate(const QSslCertificate &certificate);
    QT_DEPRECATED static void addDefaultCaCertificates(const QList<QSslCertificate> &certificates);
#endif // QT_DEPRECATED_SINCE(5, 15)
#if QT_DEPRECATED_SINCE(5, 5)
    QT_DEPRECATED static void setDefaultCaCertificates(const QList<QSslCertificate> &certificates);
    QT_DEPRECATED static QList<QSslCertificate> defaultCaCertificates();
    QT_DEPRECATED_X("Use QSslConfiguration::systemCaCertificates()") static QList<QSslCertificate> systemCaCertificates();
#endif // QT_DEPRECATED_SINCE(5, 5)

    bool waitForConnected(int msecs = 30000) override;
    bool waitForEncrypted(int msecs = 30000);
    bool waitForReadyRead(int msecs = 30000) override;
    bool waitForBytesWritten(int msecs = 30000) override;
    bool waitForDisconnected(int msecs = 30000) override;

#if QT_DEPRECATED_SINCE(5, 15)
    QT_DEPRECATED_X("Use sslHandshakeErrors()") QList<QSslError> sslErrors() const;
#endif // QT_DEPRECATED_SINCE(5, 15)
    QList<QSslError> sslHandshakeErrors() const;

    static bool supportsSsl();
    static long sslLibraryVersionNumber();
    static QString sslLibraryVersionString();
    static long sslLibraryBuildVersionNumber();
    static QString sslLibraryBuildVersionString();

    void ignoreSslErrors(const QList<QSslError> &errors);

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

protected:
    qint64 readData(char *data, qint64 maxlen) override;
    qint64 writeData(const char *data, qint64 len) override;

private:
    Q_DECLARE_PRIVATE(QSslSocket)
    Q_DISABLE_COPY(QSslSocket)
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
#if defined(Q_OS_WIN) && !defined(Q_OS_WINRT) && !QT_CONFIG(schannel)
    Q_PRIVATE_SLOT(d_func(), void _q_caRootLoaded(QSslCertificate,QSslCertificate))
#endif
    friend class QSslSocketBackendPrivate;
};

#endif // QT_NO_SSL

QT_END_NAMESPACE

#endif
