/****************************************************************************
**
** Copyright (C) 2016 Intel Corporation.
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
    { qSwap(d, other.d); }

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

Q_DECLARE_METATYPE(QNetworkDatagram)

#endif // QT_NO_UDPSOCKET

#endif // QNETWORKDATAGRAM_H
