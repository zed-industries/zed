// Copyright (C) 2022 The Qt Company Ltd.
// Copyright (C) 2016 Kurt Pattyn <pattyn.kurt@gmail.com>.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSSLSERVER_H
#define QSSLSERVER_H

#include <QtNetwork/QTcpServer>

QT_REQUIRE_CONFIG(ssl);

#include <QtNetwork/QSslError>
#include <QtNetwork/QSslConfiguration>
#include <QtNetwork/QSslPreSharedKeyAuthenticator>
#include <QtNetwork/QSslSocket>

#include <QtCore/QList>

QT_BEGIN_NAMESPACE

class QSslSocket;
class QSslServerPrivate;

class Q_NETWORK_EXPORT QSslServer : public QTcpServer
{
    Q_OBJECT
    Q_DISABLE_COPY_MOVE(QSslServer)

public:
    explicit QSslServer(QObject *parent = nullptr);
    ~QSslServer() override;

    void setSslConfiguration(const QSslConfiguration &sslConfiguration);
    QSslConfiguration sslConfiguration() const;

    void setHandshakeTimeout(int timeout);
    int handshakeTimeout() const;

Q_SIGNALS:
    void sslErrors(QSslSocket *socket, const QList<QSslError> &errors);
    void peerVerifyError(QSslSocket *socket, const QSslError &error);
    void errorOccurred(QSslSocket *socket, QAbstractSocket::SocketError error);
    void preSharedKeyAuthenticationRequired(QSslSocket *socket,
                                            QSslPreSharedKeyAuthenticator *authenticator);
    void alertSent(QSslSocket *socket, QSsl::AlertLevel level,
                   QSsl::AlertType type, const QString &description);
    void alertReceived(QSslSocket *socket, QSsl::AlertLevel level,
                       QSsl::AlertType type, const QString &description);
    void handshakeInterruptedOnError(QSslSocket *socket, const QSslError &error);
    void startedEncryptionHandshake(QSslSocket *socket);

protected:
    void incomingConnection(qintptr socket) override;

private:
    Q_DECLARE_PRIVATE(QSslServer)
};

QT_END_NAMESPACE

#endif // QSSLSERVER_H
