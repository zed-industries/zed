// Copyright (C) 2016 Intel Corporation.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QNETWORKDATAGRAM_H
#define QNETWORKDATAGRAM_H

#include <QtCore/qbytearray.h>
#include <QtNetwork/qhostaddress.h>

#ifndef QT_NO_UDPSOCKET

QT_BEGIN_NAMESPACE

class QNetworkDatagramPrivate;
class QUdpSocketPrivate;

class Q_NETWORK_EXPORT QNetworkDatagram
{
public:
    QNetworkDatagram();
    QNetworkDatagram(const QByteArray &data, const QHostAddress &destinationAddress = QHostAddress(),
                     quint16 port = 0); // implicit
    QNetworkDatagram(const QNetworkDatagram &other);
    QNetworkDatagram &operator=(const QNetworkDatagram &other);
    ~QNetworkDatagram()
    { if (d) destroy(d); }

    QNetworkDatagram(QNetworkDatagram &&other) noexcept
        : d(other.d)
    { other.d = nullptr; }
    QNetworkDatagram &operator=(QNetworkDatagram &&other) noexcept
    { swap(other); return *this; }

    void swap(QNetworkDatagram &other) noexcept
    { qt_ptr_swap(d, other.d); }

    void clear();
    bool isValid() const;
    bool isNull() const
    { return !isValid(); }

    uint interfaceIndex() const;
    void setInterfaceIndex(uint index);

    QHostAddress senderAddress() const;
    QHostAddress destinationAddress() const;
    int senderPort() const;
    int destinationPort() const;
    void setSender(const QHostAddress &address, quint16 port = 0);
    void setDestination(const QHostAddress &address, quint16 port);

    int hopLimit() const;
    void setHopLimit(int count);

    QByteArray data() const;
    void setData(const QByteArray &data);

#if defined(Q_COMPILER_REF_QUALIFIERS) || defined(Q_CLANG_QDOC)
    QNetworkDatagram makeReply(const QByteArray &payload) const &
    { return makeReply_helper(payload); }
    QNetworkDatagram makeReply(const QByteArray &payload) &&
    { makeReply_helper_inplace(payload); return *this; }
#else
    QNetworkDatagram makeReply(const QByteArray &paylaod) const
    { return makeReply_helper(paylaod); }
#endif

private:
    QNetworkDatagramPrivate *d;
    friend class QUdpSocket;
    friend class QSctpSocket;

    explicit QNetworkDatagram(QNetworkDatagramPrivate &dd);
    QNetworkDatagram makeReply_helper(const QByteArray &data) const;
    void makeReply_helper_inplace(const QByteArray &data);
    static void destroy(QNetworkDatagramPrivate *d);
};

Q_DECLARE_SHARED(QNetworkDatagram)

QT_END_NAMESPACE

QT_DECL_METATYPE_EXTERN(QNetworkDatagram, Q_NETWORK_EXPORT)

#endif // QT_NO_UDPSOCKET

#endif // QNETWORKDATAGRAM_H
