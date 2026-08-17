// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTCPSERVER_H
#define QTCPSERVER_H

#include <QtNetwork/qtnetworkglobal.h>
#include <QtCore/qobject.h>
#include <QtNetwork/qabstractsocket.h>
#include <QtNetwork/qhostaddress.h>

QT_BEGIN_NAMESPACE


class QTcpServerPrivate;
#ifndef QT_NO_NETWORKPROXY
class QNetworkProxy;
#endif
class QTcpSocket;

class Q_NETWORK_EXPORT QTcpServer : public QObject
{
    Q_OBJECT
public:
    explicit QTcpServer(QObject *parent = nullptr);
    virtual ~QTcpServer();

    bool listen(const QHostAddress &address = QHostAddress::Any, quint16 port = 0);
    void close();

    bool isListening() const;

    void setMaxPendingConnections(int numConnections);
    int maxPendingConnections() const;

    void setListenBacklogSize(int size);
    int listenBacklogSize() const;

    quint16 serverPort() const;
    QHostAddress serverAddress() const;

    qintptr socketDescriptor() const;
    bool setSocketDescriptor(qintptr socketDescriptor);

    bool waitForNewConnection(int msec = 0, bool *timedOut = nullptr);
    virtual bool hasPendingConnections() const;
    virtual QTcpSocket *nextPendingConnection();

    QAbstractSocket::SocketError serverError() const;
    QString errorString() const;

    void pauseAccepting();
    void resumeAccepting();

#ifndef QT_NO_NETWORKPROXY
    void setProxy(const QNetworkProxy &networkProxy);
    QNetworkProxy proxy() const;
#endif

protected:
    virtual void incomingConnection(qintptr handle);
    void addPendingConnection(QTcpSocket* socket);

    QTcpServer(QAbstractSocket::SocketType socketType, QTcpServerPrivate &dd,
               QObject *parent = nullptr);

Q_SIGNALS:
    void newConnection();
    void pendingConnectionAvailable(QPrivateSignal);
    void acceptError(QAbstractSocket::SocketError socketError);

private:
    Q_DISABLE_COPY(QTcpServer)
    Q_DECLARE_PRIVATE(QTcpServer)
};

QT_END_NAMESPACE

#endif // QTCPSERVER_H
