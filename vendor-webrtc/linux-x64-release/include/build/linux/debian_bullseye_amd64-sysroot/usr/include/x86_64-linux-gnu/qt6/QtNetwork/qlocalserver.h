// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QLOCALSERVER_H
#define QLOCALSERVER_H

#include <QtNetwork/qtnetworkglobal.h>
#include <QtNetwork/qabstractsocket.h>

#include <QtCore/qproperty.h>

QT_REQUIRE_CONFIG(localserver);

QT_BEGIN_NAMESPACE

class QLocalSocket;
class QLocalServerPrivate;

class Q_NETWORK_EXPORT QLocalServer : public QObject
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QLocalServer)
    Q_PROPERTY(SocketOptions socketOptions READ socketOptions WRITE setSocketOptions
               BINDABLE bindableSocketOptions)

Q_SIGNALS:
    void newConnection();

public:
    enum SocketOption {
        NoOptions = 0x0,
        UserAccessOption = 0x01,
        GroupAccessOption = 0x2,
        OtherAccessOption = 0x4,
        WorldAccessOption = 0x7,
        AbstractNamespaceOption = 0x8
    };
    Q_ENUM(SocketOption)
    Q_DECLARE_FLAGS(SocketOptions, SocketOption)
    Q_FLAG(SocketOptions)

    explicit QLocalServer(QObject *parent = nullptr);
    ~QLocalServer();

    void close();
    QString errorString() const;
    virtual bool hasPendingConnections() const;
    bool isListening() const;
    bool listen(const QString &name);
    bool listen(qintptr socketDescriptor);
    int maxPendingConnections() const;
    virtual QLocalSocket *nextPendingConnection();
    QString serverName() const;
    QString fullServerName() const;
    static bool removeServer(const QString &name);
    QAbstractSocket::SocketError serverError() const;
    void setMaxPendingConnections(int numConnections);
    bool waitForNewConnection(int msec = 0, bool *timedOut = nullptr);

    void setListenBacklogSize(int size);
    int listenBacklogSize() const;

    void setSocketOptions(SocketOptions options);
    SocketOptions socketOptions() const;
    QBindable<SocketOptions> bindableSocketOptions();

    qintptr socketDescriptor() const;

protected:
    virtual void incomingConnection(quintptr socketDescriptor);

private:
    Q_DISABLE_COPY(QLocalServer)
    Q_PRIVATE_SLOT(d_func(), void _q_onNewConnection())
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QLocalServer::SocketOptions)

QT_END_NAMESPACE

#endif // QLOCALSERVER_H

