// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTCPSOCKET_H
#define QTCPSOCKET_H

#include <QtNetwork/qtnetworkglobal.h>
#include <QtNetwork/qabstractsocket.h>
#include <QtNetwork/qhostaddress.h>
#include <QtCore/qvariant.h>

QT_BEGIN_NAMESPACE


class QTcpSocketPrivate;

class Q_NETWORK_EXPORT QTcpSocket : public QAbstractSocket
{
    Q_OBJECT
public:
    explicit QTcpSocket(QObject *parent = nullptr);
    virtual ~QTcpSocket();

#if QT_VERSION < QT_VERSION_CHECK(7,0,0) && !defined(Q_CLANG_QDOC)
    // ### Qt7: move into QAbstractSocket
    using QAbstractSocket::bind;
    bool bind(QHostAddress::SpecialAddress addr, quint16 port = 0, BindMode mode = DefaultForPlatform)
    { return bind(QHostAddress(addr), port, mode); }
#endif

protected:
    QTcpSocket(QTcpSocketPrivate &dd, QObject *parent = nullptr);
    QTcpSocket(QAbstractSocket::SocketType socketType, QTcpSocketPrivate &dd,
               QObject *parent = nullptr);

private:
    Q_DISABLE_COPY_MOVE(QTcpSocket)
    Q_DECLARE_PRIVATE(QTcpSocket)
};

QT_END_NAMESPACE

#endif // QTCPSOCKET_H
