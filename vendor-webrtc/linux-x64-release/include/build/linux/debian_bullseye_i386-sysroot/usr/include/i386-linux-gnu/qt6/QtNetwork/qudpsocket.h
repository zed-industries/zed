// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QUDPSOCKET_H
#define QUDPSOCKET_H

#include <QtNetwork/qtnetworkglobal.h>
#include <QtNetwork/qabstractsocket.h>
#include <QtNetwork/qhostaddress.h>

QT_BEGIN_NAMESPACE


#ifndef QT_NO_UDPSOCKET

class QNetworkDatagram;
class QNetworkInterface;
class QUdpSocketPrivate;

class Q_NETWORK_EXPORT QUdpSocket : public QAbstractSocket
{
    Q_OBJECT
public:
    explicit QUdpSocket(QObject *parent = nullptr);
    virtual ~QUdpSocket();

#if QT_VERSION < QT_VERSION_CHECK(7,0,0) && !defined(Q_CLANG_QDOC)
    // ### Qt7: move into QAbstractSocket
    using QAbstractSocket::bind;
    bool bind(QHostAddress::SpecialAddress addr, quint16 port = 0, BindMode mode = DefaultForPlatform)
    { return bind(QHostAddress(addr), port, mode); }
#endif

#ifndef QT_NO_NETWORKINTERFACE
    bool joinMulticastGroup(const QHostAddress &groupAddress);
    bool joinMulticastGroup(const QHostAddress &groupAddress,
                            const QNetworkInterface &iface);
    bool leaveMulticastGroup(const QHostAddress &groupAddress);
    bool leaveMulticastGroup(const QHostAddress &groupAddress,
                             const QNetworkInterface &iface);

    QNetworkInterface multicastInterface() const;
    void setMulticastInterface(const QNetworkInterface &iface);
#endif

    bool hasPendingDatagrams() const;
    qint64 pendingDatagramSize() const;
    QNetworkDatagram receiveDatagram(qint64 maxSize = -1);
    qint64 readDatagram(char *data, qint64 maxlen, QHostAddress *host = nullptr, quint16 *port = nullptr);

    qint64 writeDatagram(const QNetworkDatagram &datagram);
    qint64 writeDatagram(const char *data, qint64 len, const QHostAddress &host, quint16 port);
    inline qint64 writeDatagram(const QByteArray &datagram, const QHostAddress &host, quint16 port)
        { return writeDatagram(datagram.constData(), datagram.size(), host, port); }

private:
    Q_DISABLE_COPY_MOVE(QUdpSocket)
    Q_DECLARE_PRIVATE(QUdpSocket)
};

#endif // QT_NO_UDPSOCKET

QT_END_NAMESPACE

#endif // QUDPSOCKET_H
